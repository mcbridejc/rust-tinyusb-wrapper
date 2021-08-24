//! This build script copies the `memory.x` file from the crate root into
//! a directory where the linker can always find it at build time.
//! For many projects this is optional, as the linker always searches the
//! project root directory -- wherever `Cargo.toml` is. However, if you
//! are using a workspace or have a more complicated build setup, this
//! build script becomes required. Additionally, by requesting that
//! Cargo re-run the build script whenever `memory.x` is changed,
//! updating `memory.x` ensures a rebuild of the application with the
//! new memory settings.

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use cc;

fn main() {
    // Put `memory.x` in our output directory and ensure it's
    // on the linker search path.
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    File::create(out.join("memory.x"))
        .unwrap()
        .write_all(include_bytes!("memory.x"))
        .unwrap();
    println!("cargo:rustc-link-search={}", out.display());

    // By default, Cargo will re-run a build script whenever
    // any file in the project changes. By specifying `memory.x`
    // here, we ensure the build script is only re-run when
    // `memory.x` is changed.
    println!("cargo:rerun-if-changed=memory.x");

    // Build tinyUSB static library
    let tusb_root = PathBuf::from("ext/tinyusb");
    let _cc = cc::Build::new()
        .files([
            tusb_root.join("src/tusb.c"),
            tusb_root.join("src/common/tusb_fifo.c"),
            tusb_root.join("src/device/usbd.c"),
            tusb_root.join("src/device/usbd_control.c"),
            tusb_root.join("src/class/msc/msc_device.c"),
            tusb_root.join("src/class/cdc/cdc_device.c"),
            tusb_root.join("src/class/hid/hid_device.c"),
            tusb_root.join("src/class/midi/midi_device.c"),
            tusb_root.join("src/class/vendor/vendor_device.c"),
            tusb_root.join("src/portable/microchip/samg/dcd_samg.c"),
            PathBuf::from("src/usb_descriptors.c")
        ].iter())
        .define("__SAMG55J19__", None)
        .include(tusb_root.join("src"))
        .include(tusb_root.join("hw/mcu/microchip/samg55/samg55/include/"))
        .include(tusb_root.join("hw/mcu/microchip/samg55/CMSIS/Core/Include/"))
        .include("src")
        .compile("tinyusb");
}
