#![allow(dead_code, unused_variables, unused_mut)]
#![no_std] // Embedded mode (only lib-core)
#![no_main]

use core::panic::PanicInfo;

extern crate arduino_hal;
use arduino_hal::hal::port::PC0;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[arduino_hal::entry]
fn main() -> ! {
    // Access board peripherals
    let periph = arduino_hal::Peripherals::take().unwrap();
    // Access individual pins
    let pins = arduino_hal::pins!(periph);

    // Access d13 port
    let mut led = pins.d13.into_output();

    // Monitor Log
    loop {
        led.toggle();
        arduino_hal::delay_ms(1000);
    }
}

// Proto fn capture sunlight sensor value
fn capture_sunlight_sensor() -> PC0 {}
