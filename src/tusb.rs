use crate::pac::interrupt;

/** Define the C functions we will be calling from the tinyUSB library
*/
extern {
  fn tusb_init() -> i8;
  fn dcd_int_handler(rhport: u8);
  fn tud_task();
  fn tud_cdc_n_available(itf: u8) -> u32;
  fn tud_cdc_n_read(itf: u8, buf: *mut u8, bufsize: u32) -> u32;
  fn tud_cdc_n_write(itf: u8, buf: *const u8, bufsize: u32) -> u32;
  fn tud_cdc_n_write_flush(itf: u8) -> u32;
}

/** Define rust functions to wrap up the tinyUSB ones. These converrt types 
as needed, and hides the unsafe code from the main application. */
pub fn init() {
  unsafe {
    tusb_init();
  }
}

pub fn poll() {
  unsafe {
    tud_task();
  }
}

pub mod cdc {
  use super::*;
  pub fn available() -> usize {
    let bytes_available: u32;
    unsafe {
      bytes_available = tud_cdc_n_available(0);
    }
    bytes_available as usize
  }

  pub fn read(buf: &mut [u8]) -> usize {
    unsafe {
      tud_cdc_n_read(0, buf.as_mut_ptr(), buf.len() as u32) as usize
    }
  }

  pub fn write(buf: &[u8]) -> usize {
    unsafe {
      tud_cdc_n_write(0, buf.as_ptr(), buf.len() as u32) as usize
    }
  }

  pub fn write_flush() -> usize {
    unsafe { 
      tud_cdc_n_write_flush(0) as usize
    }
  }

}

/** Create an interrupt handler and simply pass along to the tinyUSB handler */
#[interrupt]
fn UDP() {
  unsafe {
    dcd_int_handler(0);
  }
}