// Linter Config
#![allow(dead_code, unused_variables)]
#![warn(unused_crate_dependencies, unused_imports, clippy::cast_precision_loss)]
#![deny(unused_allocation)]
#![forbid(clippy::many_single_char_names)]
#![cfg(target_arch = "avr")]

// Embedded mode (only lib-core)
#![no_std]
#![no_main]
use arduino_hal::prelude::*;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    core::panic!()
}

/*!
 * Forked from https://github.com/Rahix/avr-hal/blob/main/examples/arduino-uno/src/bin/uno-i2cdetect.rs
 * 
 * Detect all devices connected on the I2C/TWI bus.  Useful if you can't figure out the address of
 * an I2C device.
 *
 * This example will check all possible addresses on the I2C bus for whether a device responds to
 * them.  It will output a table of the results.  This check is done twice, once for reading and
 * once for writing, as some devices only respond to one of the two operations.
 *
 * ATTENTION: Randomly reading from and writing to devices can lead to unexpected results.  Some
 * devices do not cope well with this.  Use with care!
 *
 * Connections
 * -----------
 *  - `A4`: I2C SDA signal
 *  - `A5`: I2C SCL signal
 */

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
