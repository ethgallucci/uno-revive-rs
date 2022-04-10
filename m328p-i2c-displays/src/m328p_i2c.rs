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

use arduino_hal::prelude::*;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    core::panic!()
}

#[arduino_hal::entry]
fn root() -> ! {
    let periph = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(periph);
    let mut serial = arduino_hal::default_serial!(periph, pins, 57600);

    let mut i2c = arduino_hal::I2c::new(
        periph.TWI,
        pins.a4.into_pull_up_input(),
        pins.a5.into_pull_up_input(),
        50000,
    );

    ufmt::uwriteln!(&mut serial, "Write direction test:\r").void_unwrap();
    i2c.i2cdetect(&mut serial, arduino_hal::i2c::Direction::Write)
        .void_unwrap();
    ufmt::uwriteln!(&mut serial, "\r\nRead direction test:\r").void_unwrap();
    i2c.i2cdetect(&mut serial, arduino_hal::i2c::Direction::Read)
        .void_unwrap();

    loop {}
}
