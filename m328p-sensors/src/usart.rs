use super::*;

// Proto fn writing/reading from serial console
pub fn proto_usart() -> ! {
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