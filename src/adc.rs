//! ADC
//!

use embedded_hal::digital::InputPin;

use crate::clk::*;
use crate::gpio::*;
use crate::pac::*;


pub trait AdcExt<ADC> {
    
    /// Hardware ADC
    type Adc;

    /// Create a new instance
    fn adc(self, clk: &Clk) -> Self::Adc;
}

/// Internal bandgap reference voltage channel
#[inline(always)]
pub fn reference_channel() -> Channel {
    Channel::C7
}

/// Internal bandgap reference voltage value in mV
#[inline(always)]
pub fn reference_value() -> u16 {
    1220
}

pub struct Adc<ADC> {
    adc: ADC,
}

impl AdcExt<ADC1> for ADC1 {

    type Adc = Adc<Self>;

    fn adc(self, clk: &Clk) -> Adc<Self> {
        let new = Adc { adc: self };

        // Set ADC frequency: shall be in a range [1..4] MHz
        const DIV2: u8 = 0b000;
        const DIV3: u8 = 0b001;
        const DIV4: u8 = 0b010;
        const DIV6: u8 = 0b011;
        const DIV8: u8 = 0b100;
        let divider = match clk.master_clk().to_MHz() {
            2..=4 => DIV2,
            5..=6 => DIV3,
            7..=8 => DIV4,
            9..=12 => DIV6,
            13..=16 => DIV8,
            _ => DIV2,
        };
        unsafe {
            new.adc.cr1().modify(|_, w| w.spsel().bits(divider));
        }

        // Set right alignment of result
        new.adc.cr2().modify(|_, w| w.align().set_bit());
        // Enable ADC1
        new.adc.cr1().modify(|_, w| w.adon().set_bit());

        new
    }
}

/// Number of channel to read data
pub enum Channel {
    C0 = 0,
    C1,
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
}

impl Adc<ADC1> {
    
    /// Read the value from dedicated channel == analog input
    pub fn read(&self, channel: Channel) -> u16 {

        // Set channel number to be converted
        unsafe {
            self.adc.csr().modify(|_, w| w.ch().bits(channel as u8));
        }
        // Start conversion
        self.adc.cr1().modify(|_, w| w.adon().set_bit());
        // Wait for conversion completion
        while self.adc.csr().read().eoc().bit_is_clear() {}

        let low = self.adc.drl().read().bits();
        let high = self.adc.drh().read().bits();
    
        // Clear EOC flag
        self.adc.csr().modify(|_, w| w.eoc().clear_bit());

        ((high as u16) << 8) + low as u16
    }

    /// Read the value from the channel associated with particular pin
    #[inline(always)]
    pub fn read_pin(&self, pin: &impl AdcInput) -> u16 {
        self.read(pin.channel())
    }

}

pub trait AdcInput: InputPin {
    fn channel(&self) -> Channel;
}

macro_rules! adc_inputs {
    ($($PIN:ident: $channel:ident,
    )+) => {
        $(
            impl<MODE> AdcInput for $PIN<Input<MODE>> {
            #[inline(always)]
            fn channel(&self) -> Channel {
                Channel::$channel
            }
        }
        )+
    };
}

adc_inputs!(
    PC4: C2,
    PD2: C3,
    PD3: C4,
    PD5: C5,
    PD6: C6,
);
