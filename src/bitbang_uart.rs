//! Write-only bit-bang UART
//! 
//! OutputSwitch is used instead of OutputPin in order to inverse bit levels if needed
//!

use core::convert::Infallible;
use embedded_hal::delay::DelayNs;
pub use embedded_io::{
    ErrorType,
    Write,
};
pub use switch_hal::*;

use crate::clk::*;
use crate::uart::BaudRate;


impl<TX, Timer, E> BitbangUart<TX, Timer>
where
    TX: OutputSwitch<Error = E>,
    Timer: DelayNs,
    Infallible: From<E>,
{
    /// Create instance
    pub fn new(tx: TX, timer: Timer, baudrate: BaudRate, clk: &Clk) -> Self {
        let desired_delay_us = match baudrate {
            BaudRate::Baud9600 => 104,
            BaudRate::Baud19200 => 52,
            BaudRate::Baud38400 => 26,
            BaudRate::Baud57600 => 17,
            BaudRate::Baud115200 => 9,
        };
        // Consider time to switch pin, calculations etc.
        // .to_MHz doesn't work for some reason
        let added_delay_us = 128 / (clk.cpu_clk().to_Hz() / 1_000_000);
        assert!(desired_delay_us >= added_delay_us);
        let delay_us = desired_delay_us - added_delay_us;
        BitbangUart { tx, timer, delay_us }
    }

    #[inline(always)]
    fn write_byte(&mut self, byte: &u8) -> Result<(), <Self as embedded_io::ErrorType>::Error> {
        let mut byte = byte.clone();

        // start bit
        self.tx.off()?; 
        self.timer.delay_us(self.delay_us);

        // 8 bits
        for _bit in 0..8 {
            if byte & 1 == 1 {
                self.tx.on()?;
            } else {
                self.tx.off()?;
            }
            byte >>= 1;
            self.timer.delay_us(self.delay_us);
        }

        // stop bit
        self.tx.on()?;
        self.timer.delay_us(self.delay_us);
        Ok(())
    }
}

impl<TX, Timer> ErrorType for BitbangUart<TX, Timer>
where
    TX: OutputSwitch,
    Timer: DelayNs,
{
    type Error = Infallible;
}

/// Bit banging UART device
pub struct BitbangUart<TX, Timer>
where
    TX: OutputSwitch,
    Timer: DelayNs,
{
    tx: TX,
    timer: Timer,
    delay_us: u32,
}

impl<TX, Timer, E> Write for BitbangUart<TX, Timer>
where
    TX: OutputSwitch<Error = E>,
    Timer: DelayNs,
    Infallible: From<E>
{
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        let mut counter: usize = 0;

        for byte in buf {
            self.write_byte(byte)?;
            counter += 1;
        }

        Ok(counter)
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

}
