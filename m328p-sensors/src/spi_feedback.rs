use super::*;

// Proto fn for establishing SPI comms
pub fn proto_spi_feedback() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    // Setup SPI for text output
    let serial = arduino_hal::default_serial!(dp, pins, 57600);

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