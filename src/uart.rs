//! Write-only UART
//! 

use core::convert::Infallible;
use embedded_hal::digital::OutputPin;
pub use embedded_io::{
    ErrorType,
    Write,
};

use crate::clk::*;
use crate::gpio::*;
use crate::pac;

mod baudrate;
pub use baudrate::BaudRate;

impl<TX> Uart<TX>
where
    TX: TxPin,
{
    /// Create instance
    pub fn new(uart: pac::UART1, tx: TX, baudrate: BaudRate, clk: &Clk) -> Self {
        let baudrate = match baudrate {
            BaudRate::Baud9600 => 9600,
            BaudRate::Baud19200 => 19200,
            BaudRate::Baud38400 => 38400,
            BaudRate::Baud57600 => 57600,
            BaudRate::Baud115200 => 115200,
        };

        let baud_divider = clk.master_clk().to_Hz() / baudrate;
        let bits_12_15 = (baud_divider >> 12) as u8;
        let bits_4_11 = (baud_divider >> 4) as u8;
        let bits_0_3 = (baud_divider & 0b1111) as u8;

        let brr1_value = bits_4_11;
        let brr2_value = (bits_12_15 << 4) | bits_0_3;

        // Set baudrate
        unsafe {
            uart.brr2().write(|w| w.uart_div().bits(brr2_value));
            uart.brr1().write(|w| w.uart_div().bits(brr1_value));
        }

        // Enable transmitter
        uart.cr2().modify(|_, w| w.ten().set_bit());

        Uart { uart, _tx: tx }
    }

}

/// UART transmit pin.
/// Possible options are PD5 or PA3 (enabled by the corresponding option byte)
pub trait TxPin: OutputPin {}

impl<MODE> TxPin for PD5<Output<MODE>> {}
impl<MODE> TxPin for PA3<Output<MODE>> {}

impl<TX> ErrorType for Uart<TX>
where
    TX: TxPin,
{
    type Error = Infallible;
}

/// UART device
pub struct Uart<TX>
where
    TX: TxPin,
{
    uart: pac::UART1,
    _tx: TX,
}

impl<TX> Write for Uart<TX>
where
    TX: TxPin,
{
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        let mut counter: usize = 0;

        for byte in buf {
            unsafe {
                // Send byte
                self.uart.dr().write(|w| w.dr().bits(*byte));
                // Wait for transmitter register is empty
                while self.uart.sr().read().txe().bit_is_clear() {}
            }
            counter += 1;
        }
        // Wait for transmission ending
        while self.uart.sr().read().tc().bit_is_clear() {}

        Ok(counter)
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

}
