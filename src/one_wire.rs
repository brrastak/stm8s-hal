//! Simplified library for the 1-Wire protocol.
//! Based on https://github.com/daniel-larsen/one-wire-bus.git
//! Delay interval durations tuned to correspond my project and slow micro


use embedded_hal::{digital::{InputPin, OutputPin}, delay::DelayNs};


pub type OneWireResult<T, E> = Result<T, OneWireError<E>>;


#[derive(Debug, Copy, Clone)]
pub enum OneWireError<E> {
    /// The Bus was expected to be pulled high by a ~5K ohm pull-up resistor, but it wasn't
    BusNotHigh,

    PinError(E),

    /// An unexpected response was received from a command. This generally happens when a new sensor is added
    /// or removed from the bus during a command, such as a device search.
    UnexpectedResponse,

    FamilyCodeMismatch,
    CrcMismatch,
    Timeout,
}

/// All the delays should be shorter to achive desired value.
/// Added value calculated for a case CPU frequency = 16MHz
const fn tune_delay(delay: u32) -> u32 {

    let added_delay = 128 / 16;

    if delay > added_delay {
        delay - added_delay
    }
    else {
        1
    }
}

impl<Pin, E> OneWire<Pin>
where
    Pin: InputPin<Error = E>,
    Pin: OutputPin<Error = E>,
{
    pub fn new(pin: Pin) -> OneWireResult<OneWire<Pin>, E> {
        let mut one_wire = OneWire { pin };
        // Pin should be high during idle.
        one_wire.release_bus()?;
        Ok(one_wire)
    }

    /// Disconnects the bus, letting another device (or the pull-up resistor) set the bus value
    pub fn release_bus(&mut self) -> OneWireResult<(), E> {
        self.pin
            .set_high()
            .map_err(|err| OneWireError::PinError(err))
    }

    /// Drives the bus low
    pub fn set_bus_low(&mut self) -> OneWireResult<(), E> {
        self.pin
            .set_low()
            .map_err(|err| OneWireError::PinError(err))
    }

    pub fn is_bus_high(&mut self) -> OneWireResult<bool, E> {
        self.pin
            .is_high()
            .map_err(|err| OneWireError::PinError(err))
    }

    pub fn is_bus_low(&mut self) -> OneWireResult<bool, E> {
        self.pin.is_low().map_err(|err| OneWireError::PinError(err))
    }

    fn wait_for_high(&mut self, delay: &mut impl DelayNs) -> OneWireResult<(), E> {
        // wait up to 250 µs for the bus to become high (from the pull-up resistor)
        for _ in 0..125 {
            if self.is_bus_high()? {
                return Ok(());
            }
            delay.delay_us(2);
        }
        Err(OneWireError::BusNotHigh)
    }

    /// Sends a reset pulse, then returns true if a device is present
    pub fn reset(&mut self, delay: &mut impl DelayNs) -> OneWireResult<bool, E> {
        self.wait_for_high(delay)?;

        self.set_bus_low()?;
        delay.delay_us(480); // Maxim recommended wait time

        self.release_bus()?;
        delay.delay_us(70); // Maxim recommended wait time

        let device_present = self.is_bus_low()?;

        delay.delay_us(410); // Maxim recommended wait time
        Ok(device_present)
    }

    pub fn read_bit(&mut self, delay: &mut impl DelayNs) -> OneWireResult<bool, E> {
        self.set_bus_low()?;
        delay.delay_us(tune_delay(6)); // Maxim recommended wait time

        self.release_bus()?;
        delay.delay_us(tune_delay(9)); // Maxim recommended wait time

        let bit_value = self.is_bus_high()?;
        delay.delay_us(tune_delay(55)); // Maxim recommended wait time
        Ok(bit_value)
    }

    pub fn read_byte(&mut self, delay: &mut impl DelayNs) -> OneWireResult<u8, E> {
        let mut output: u8 = 0;
        for _ in 0..8 {
            output >>= 1;
            if self.read_bit(delay)? {
                output |= 0x80;
            }
        }
        Ok(output)
    }
    pub fn read_bytes(
        &mut self,
        output: &mut [u8],
        delay: &mut impl DelayNs,
    ) -> OneWireResult<(), E> {
        for i in 0..output.len() {
            output[i] = self.read_byte(delay)?;
        }
        Ok(())
    }

    pub fn write_1_bit(&mut self, delay: &mut impl DelayNs) -> OneWireResult<(), E> {
        self.set_bus_low()?;
        delay.delay_us(tune_delay(6)); // Maxim recommended wait time

        self.release_bus()?;
        delay.delay_us(tune_delay(64)); // Maxim recommended wait time
        Ok(())
    }

    pub fn write_0_bit(&mut self, delay: &mut impl DelayNs) -> OneWireResult<(), E> {
        self.set_bus_low()?;
        delay.delay_us(tune_delay(60)); // Maxim recommended wait time

        self.release_bus()?;
        delay.delay_us(tune_delay(10)); // Maxim recommended wait time
        Ok(())
    }

    pub fn write_bit(
        &mut self,
        value: bool,
        delay: &mut impl DelayNs,
    ) -> OneWireResult<(), E> {
        if value {
            self.write_1_bit(delay)
        } else {
            self.write_0_bit(delay)
        }
    }

    pub fn write_byte(
        &mut self,
        mut value: u8,
        delay: &mut impl DelayNs,
    ) -> OneWireResult<(), E> {
        for _ in 0..8 {
            self.write_bit(value & 0x01 == 0x01, delay)?;
            value >>= 1;
        }
        Ok(())
    }

    pub fn write_bytes(
        &mut self,
        bytes: &[u8],
        delay: &mut impl DelayNs,
    ) -> OneWireResult<(), E> {
        for i in 0..bytes.len() {
            self.write_byte(bytes[i], delay)?;
        }
        Ok(())
    }

    /// Address all devices on the bus simultaneously.
    /// This should only be called after a reset, and should be immediately followed by another command
    pub fn skip_address(&mut self, delay: &mut impl DelayNs) -> OneWireResult<(), E> {
        self.write_byte(Command::SkipRom as u8, delay)?;
        Ok(())
    }

    /// Sends a reset, followed with either a SKIP_ROM and then the supplied command
    /// This should be followed by any reading/writing, if needed by the command used
    pub fn send_command(
        &mut self,
        command: u8,
        delay: &mut impl DelayNs,
    ) -> OneWireResult<(), E> {
        self.reset(delay)?;
        self.skip_address(delay)?;
        self.write_byte(command, delay)?;
        Ok(())
    }
}

enum Command {
    SkipRom = 0xcc,
}

pub struct OneWire<Pin> {
    pin: Pin,
}

