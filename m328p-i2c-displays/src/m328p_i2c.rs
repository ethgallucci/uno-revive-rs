// Linter Config
#![allow(dead_code, unused_variables)]
#![warn(unused_crate_dependencies, unused_imports, clippy::cast_precision_loss)]
#![deny(unused_allocation)]
#![forbid(clippy::many_single_char_names)]
#![cfg(target_arch = "avr")]

// Embedded mode (only lib-core)
#![no_std]
#![no_main]
extern crate arduino_hal;
extern crate ufmt;

// Macros
extern crate reviver_macros;
use reviver_macros::*;

use arduino_hal as hal;
use hal::{adc, prelude::*, hal::wdt};
use hal::prelude::*;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    core::panic!()
}

#[arduino_hal::entry]
fn root() -> ! {
    let periph = hal::Peripherals::take().unwrap();
    let pins = hal::pins!(periph);
    let mut serial = hal::default_serial!(periph, pins, 57600);

    // VGT Readouts
    let mut adc = hal::Adc::new(periph.ADC, Default::default());
    vgt!(adc, serial);

    let mut i2c = hal::I2c::new(
        periph.TWI,
        pins.a4.into_pull_up_input(),
        pins.a5.into_pull_up_input(),
        50000,
    );

    ufmt::uwriteln!(&mut serial, "Write direction test:\r").void_unwrap();
    i2c.i2cdetect(&mut serial, hal::i2c::Direction::Write)
        .void_unwrap();
    ufmt::uwriteln!(&mut serial, "\r\nRead direction test:\r").void_unwrap();
    i2c.i2cdetect(&mut serial, hal::i2c::Direction::Read)
        .void_unwrap();

    loop {}
}
