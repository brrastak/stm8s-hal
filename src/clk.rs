use crate::pac::CLK;

pub use fugit::{
    ExtU32, HertzU32 as Hertz, HoursDurationU32 as Hour, MicrosDurationU32 as MicroSecond,
    MinutesDurationU32 as Minute, RateExtU32, SecsDurationU32 as Second,
};

mod config;

pub use config::*;

/// HSI frequency
pub const HSI_FREQ: u32 = 16_000_000;

/// Clock frequencies
#[derive(Clone, Copy)]
pub struct Clocks {
    /// Master frequency
    pub master_clk: Hertz,
    /// CPU frequency
    pub cpu_clk: Hertz,
}

impl Default for Clocks {
    fn default() -> Clocks {
        Clocks {
            master_clk: 16.MHz(),
            cpu_clk: 16.MHz(),
        }
    }
}

/// Constrained RCC peripheral
pub struct Clk {
    /// Clock configuration
    pub clocks: Clocks,
    pub(crate) rb: CLK,
}

impl core::ops::Deref for Clk {
    type Target = CLK;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.rb
    }
}

impl Clk {
    /// Apply clock configuration
    pub fn freeze(self, clk_cfg: Config) -> Self {

        let master_clk = match clk_cfg.master_src {
            MasterClockSrc::HSI(psc) => {
                let (freq, psc_bits) = match psc {
                    Prescaler::Div2 => (HSI_FREQ / 2, 0b01),
                    Prescaler::Div4 => (HSI_FREQ / 4, 0b10),
                    Prescaler::Div8 => (HSI_FREQ / 8, 0b11),
                    _ => (HSI_FREQ, 0b00),
                };
                self.ckdivr().modify(|_, w| unsafe { w.hsidiv().bits(psc_bits)});
                freq.Hz()
            }
            MasterClockSrc::HSE(freq) => {
                self.enable_hse();
                freq
            }
            MasterClockSrc::LSI => {
                self.enable_lsi();
                128_000.Hz()
            }
        };

        let master_freq = master_clk.raw();
        let (cpu_clk, psc_bits) = match clk_cfg.cpu_psc {
            Prescaler::Div2 => (master_freq / 2, 0b001),
            Prescaler::Div4 => (master_freq / 4, 0b010),
            Prescaler::Div8 => (master_freq / 8, 0b011),
            Prescaler::Div16 => (master_freq / 16, 0b100),
            Prescaler::Div64 => (master_freq / 64, 0b101),
            Prescaler::Div128 => (master_freq / 128, 0b111),
            _ => (master_freq, 0b000),
        };
        let cpu_clk = cpu_clk.Hz();
        self.ckdivr().modify(|_, w| unsafe { w.cpudiv().bits(psc_bits) });

        Clk {
            rb: self.rb,
            clocks: Clocks {
                master_clk,
                cpu_clk
            },
        }
    }

    pub(crate) fn _enable_hsi(&self) {
        self.swcr().modify(|_, w| w.swien().clear_bit().swen().set_bit());
        self.swr().write(|w| unsafe { w.bits(0xE1) });
        while self.swcr().read().swif().bit_is_clear() {}
    }

    pub(crate) fn enable_hse(&self) {
        self.eckr().modify(|_, w| w.hseen().set_bit());
        self.swcr().modify(|_, w| w.swien().clear_bit().swen().set_bit());
        self.swr().write(|w| unsafe { w.bits(0xB4) });
        while self.swcr().read().swif().bit_is_clear() {}
    }

    /// The corresponding option bytes configuration should be set
    pub(crate) fn enable_lsi(&self) {
        self.ickr().modify(|_, w| w.lsien().set_bit());
        self.swcr().modify(|_, w| w.swien().clear_bit().swen().set_bit());
        self.swr().write(|w| unsafe { w.bits(0xD2) });
        while self.swcr().read().swif().bit_is_clear() {}
    }

}

/// Extension trait that constrains the CLK peripheral
pub trait ClkExt {
    /// Constrains the CLK peripheral so it plays nicely with the other abstractions
    fn constrain(self) -> Clk;
    /// Constrains the CLK peripheral and apply clock configuration
    fn freeze(self, clk_cfg: Config) -> Clk;
}

impl ClkExt for CLK {
    fn constrain(self) -> Clk {
        Clk {
            rb: self,
            clocks: Clocks::default(),
        }
    }

    fn freeze(self, clk_cfg: Config) -> Clk {
        self.constrain().freeze(clk_cfg)
    }
}
