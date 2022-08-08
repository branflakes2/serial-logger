#![no_std]
use embedded_hal::serial::Write;

pub struct SerialLogger<'a, E> {
    uart: &'a mut dyn Write<u8, Error = E>,
}

impl<'a, E> SerialLogger<'a, E> {
    pub fn new(uart: &'a mut impl Write<u8, Error = E>) -> Self {
        return SerialLogger { uart };
    }

    pub fn l(self, m: &'static str) -> Result<(), nb::Error<E>> {
        for i in 0..m.len() {
            self.uart.write(m.as_bytes()[i])?;
        }
        return Ok(());
    }
}
