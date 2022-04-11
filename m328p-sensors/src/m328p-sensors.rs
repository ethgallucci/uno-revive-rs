// Linter Config
#![warn(unused_crate_dependencies, unused_imports, clippy::cast_precision_loss)]
#![forbid(clippy::logic_bug)]
#![deny(unused_allocation)]
#![cfg(target_arch = "avr")]

// Embedded mode (only lib-core)
#![cfg_attr(not(test), no_std)]
#![no_main]

// Extern Dependencies
extern crate arduino_hal;
extern crate avr_hal_generic;
extern crate embedded_hal;
extern crate ufmt;

// Macros we defined in another crate
extern crate reviver_macros;
use reviver_macros::*;

// Panic Info
use core::panic::PanicInfo;

// Imports
use arduino_hal as hal;
use hal::{adc, prelude::*, hal::wdt};

// Panic Implementation
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    core::panic!()
}

// ----------------------------------------------------------------
// ENTRY
// ----------------------------------------------------------------
#[arduino_hal::entry]
fn root() -> ! {
    // Access peripherals and indiv. pins
    let periph = hal::Peripherals::take().unwrap();
    let pins = hal::pins!(periph);

    // Establish serial console
    let mut serial = hal::default_serial!(periph, pins, 57600);

    // Instantiate ADC channels
    let mut adc = hal::Adc::new(periph.ADC, Default::default());

    // ADC Channel readouts
    vgt!(adc, serial);

    // Sensors
    let a0 = pins.a0.into_analog_input(&mut adc);
    let a1 = pins.a1.into_analog_input(&mut adc);

    // Setup our watchdog with an 8-second timeout
    let mut watchdog = wdt::Wdt::new(periph.WDT, &periph.CPU.mcusr);
    watchdog.start(wdt::Timeout::Ms8000).unwrap();

    loop {
        // Analog read in our soil sensors
        let values = two_pins_to_arr_analog!(&mut adc, a0, a1);

        // Writing values to serial console
        two_pins_analog_to_ser_out!(&mut serial, values);
        
        // Wait 3 seconds
        hal::delay_ms(3000);
        // Feed the watchdog
        watchdog.feed();
    }
}