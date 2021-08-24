#![no_std]
#![no_main]

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
// use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m::{iprintln, Peripherals};
use cortex_m_rt::entry;

use atsamg_hal::target_device as pac;

mod tusb;

fn enable_pll_a(mul: u16, pmc: &mut pac::PMC) {
    if mul < 8 || mul > 7500 {
      panic!("Out of range multiplier for PLLA")
    }
    unsafe {
      pmc.ckgr_pllar.write(|w| 
        w.mula().bits(mul - 1)
        .pllaen().bits(1)
        .pllacount().bits(50) 
      );
      // Wait for PLL lock
      while pmc.pmc_sr.read().locka().bit_is_clear() {}
    }
  }
  
  fn enable_pll_b(mul: u16, pmc: &mut pac::PMC) {
    // TODO: Can I statically check this? 
    if mul < 8 || mul > 7500 {
      panic!("Out of range multiplier for PLLB")
    }
    unsafe {
      pmc.ckgr_pllbr.write(|w| 
        w.mulb().bits(mul - 1)
        .pllben().bits(1)
        .pllbcount().bits(50) 
      );
      // Wait for PLL lock
      while pmc.pmc_sr.read().locka().bit_is_clear() {}
    }
  }
  

fn setup_clocks(efc: &mut pac::EFC, supc: &mut pac::SUPC, pmc: &mut pac::PMC) {
    // Select external 32kHz crystal
    unsafe {
        supc.cr.write_with_zero(|w| 
            w.key().passwd()
            .xtalsel().crystal_sel()
        );
    }

    // Set flash wait states for 120MHz
    efc.fmr.modify(|_r, w| unsafe { w.fws().bits(5) });

    // Use PLL A for main 120MHz clock
    enable_pll_a(3662, pmc);
    pmc.pmc_mckr.modify(|_r, w| 
      w.pres().clk_1()
      .plladiv2().clear_bit()
    );
    while pmc.pmc_sr.read().mckrdy().bit_is_clear() {}
    pmc.pmc_mckr.modify(|_r, w| 
      w.css().plla_clk()
    );
    while pmc.pmc_sr.read().mckrdy().bit_is_clear() {}

    // Use PLL B for 48MHz USB clock
    enable_pll_b(1465, pmc);
    unsafe {
      pmc.pmc_usb.write(|w|
        w.usbdiv().bits(0) // divide by 1
        .usbs().set_bit() // use PLLB as source
      );
    }
}

#[entry]
fn main() -> ! {
    let mut core = Peripherals::take().unwrap();
    let device = pac::Peripherals::take().unwrap();

    let mut pmc = device.PMC;
    let mut efc = device.EFC;
    let mut supc = device.SUPC;

    let stim = &mut core.ITM.stim[0];

    // disable watchdog
    device.WDT.mr.write(|w| w.wddis().set_bit());

    iprintln!(stim, "Running!");

    setup_clocks(&mut efc, &mut supc, &mut pmc);
    
    // Enable porta clock
    unsafe {pmc.pmc_pcer0.write_with_zero(|w| w.pid11().set_bit());}
    // Enable DM/DP pins
    device.MATRIX.ccfg_sysio.modify(|_r, w| w.sysio10().clear_bit().sysio11().clear_bit());
    // Enable UDP clocks
    unsafe {pmc.pmc_pcer1.write_with_zero(|w| w.pid48().set_bit());}
    unsafe {pmc.pmc_scer.write_with_zero(|w| w.udp().set_bit());}
    // Put USB in device mode and tranceiver active
    device.MATRIX.ccfg_usbmr.write(|w| w.usbmode().set_bit().usbhtssc().clear_bit());
    
    // Call tinyUSB library init routine
    tusb::init();

    iprintln!(stim, "Init complete, polling USB");
    loop {
        tusb::poll();
        cdc_task();
    }
}

fn cdc_task() {
    if tusb::cdc::available() > 0 {
        let mut buf = [0u8; 64];

        let bytes_read = tusb::cdc::read(&mut buf);
        if bytes_read > 0 {
            tusb::cdc::write(&buf[..bytes_read]);
            tusb::cdc::write_flush();
        }
    }
}
