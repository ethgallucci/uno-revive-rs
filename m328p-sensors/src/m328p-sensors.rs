#![allow(dead_code, unused_variables, unused_mut)]

#![no_std] // Embedded mode (only lib-core)
#![no_main]

use core::panic::PanicInfo;

extern crate embedded_hal;
extern crate nb;
extern crate ufmt;
extern crate arduino_hal;

use nb::block;
use arduino_hal::prelude::*;
use arduino_hal::hal::port::{PB1, PB2, PB3};
use arduino_hal::port::{mode::Output, Pin};
use arduino_hal::spi;
use embedded_hal::{spi::FullDuplex, serial::Read};


#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[arduino_hal::entry]
fn root() -> ! {
    loop {
        // Access peripherals and indiv. pins
        let periph = arduino_hal::Peripherals::take().unwrap();
        let pins = arduino_hal::pins!(periph);

        // Get sensor values here

        // Tidydata

        // Export data to cloud db

        // 30 minute delay in between logs
        arduino_hal::delay_ms(65535)
    }
}

// Proto fn for establishing SPI comms
fn proto_spi_feedback() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    // Setup SPI for text output
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    // Create SPI interface
    let (mut spi, _) = arduino_hal::Spi::new(
        dp.SPI,
        pins.d13.into_output(),
        pins.d11.into_output(),
        pins.d12.into_pull_up_input(),
        pins.d10.into_output(),
        spi::Settings::default(),
    );

    loop {
        // Send a byte
        block!(spi.send(0b00001111)).void_unwrap();
        // Assumes MISO connected to MOSI, read data should be same
        let dat = nb::block!(spi.read()).void_unwrap();
        arduino_hal::delay_ms(1000);
    }
}

// Proto fn for sending monitor data accross channels (to the board that will write this data to SD)
fn recursive_monitor() -> ! {
    let periph = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(periph);

    // Access ports we'll want
    let soil_moisture_value = pins.d11.into_output();
    let temperature_value = pins.d10.into_output();
    let irrigation_controller_status = pins.d9.into_output();

    some_fn_that_outputs_to_sd(
        soil_moisture_value,
        temperature_value,
        irrigation_controller_status,
    );

    recursive_monitor()
}

// Proto fn for writing monitor data to OLED shields on board
#[inline] fn some_fn_that_outputs_to_sd(
    smv: Pin<Output, PB3>,
    tmp: Pin<Output, PB2>,
    irr: Pin<Output, PB1>,
) -> () {}

// Proto fn writing/reading from serial console
fn proto_usart() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    ufmt::uwriteln!(&mut serial, "Hello from m328p garden!\r").void_unwrap();

    loop {
        // read a byte
        let b = nb::block!(serial.read()).void_unwrap();

        // respond
        ufmt::uwriteln!(&mut serial, "Got {}!\r", b).void_unwrap();
    }
}

