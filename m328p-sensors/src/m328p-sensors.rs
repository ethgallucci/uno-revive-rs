// Linter Config
#![allow(dead_code, unused_variables)]
#![warn(unused_crate_dependencies, unused_imports, clippy::cast_precision_loss)]
#![deny(unused_allocation)]
#![forbid(clippy::many_single_char_names)]
#![cfg(target_arch = "avr")]

// Embedded mode (only lib-core)
#![no_std]
#![no_main]

// Extern Dependencies
extern crate arduino_hal;
extern crate avr_hal_generic;
extern crate embedded_hal;
extern crate ufmt;

// Internal Modules
mod spi_feedback;
mod usart;

// Panic Info
use core::panic::PanicInfo;

// Imports
use arduino_hal::{adc, prelude::*, spi, hal::wdt};
use embedded_hal::{spi::FullDuplex, serial::Read};

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
    let periph = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(periph);

    // Establish serial console
    let mut serial = arduino_hal::default_serial!(periph, pins, 57600);

    // Instantiate ADC channels
    let mut adc = arduino_hal::Adc::new(periph.ADC, Default::default());

    // Grab ADC channel readouts
    let (vbg, gnd, tmp) = (
        adc.read_blocking(&adc::channel::Vbg),
        adc.read_blocking(&adc::channel::Gnd),
        adc.read_blocking(&adc::channel::Temperature),
    );
    ufmt::uwriteln!(&mut serial, "Vbandgap: {}", vbg).void_unwrap();
    ufmt::uwriteln!(&mut serial, "Ground: {}", gnd).void_unwrap();
    ufmt::uwriteln!(&mut serial, "Temperature: {}", tmp).void_unwrap();
    
    // Acess our soil sensors located on pins a0, a1
    let a0 = pins.a0.into_analog_input(&mut adc);
    let a1 = pins.a1.into_analog_input(&mut adc);

    // Setup our watchdog with an 8-second timeout
    let mut watchdog = wdt::Wdt::new(periph.WDT, &periph.CPU.mcusr);
    watchdog.start(wdt::Timeout::Ms8000).unwrap();

    loop {
        // Analog read in our soil sensors
        let values = [
            a0.analog_read(&mut adc),
            a1.analog_read(&mut adc),
        ];

        // Writing values to serial console
        for(i, v) in values.iter().enumerate() {
            ufmt::uwrite!(&mut serial, "A{}: {} ", i, v).void_unwrap();
        }
        ufmt::uwriteln!(&mut serial, "").void_unwrap();
        
        // Wait 3 seconds
        arduino_hal::delay_ms(3000);
        // Feed the watchdog
        watchdog.feed();
    }
}