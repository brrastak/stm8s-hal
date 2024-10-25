use crate::clk::Hertz;

/// Prescaler
#[derive(Clone, Copy)]
pub enum Prescaler {
    NotDivided,
    Div2,
    Div4,
    Div8,
    Div16,
    Div32,
    Div64,
    Div128,
}

/// Master clock mux source
pub enum MasterClockSrc {
    LSI,
    HSI(Prescaler),
    HSE(Hertz),
}

/// Configurable clock output source
pub enum CCOSrc {
    HSI,
    HSIDIV,
    HSE,
    LSI,
    Master,
    CPU(Prescaler),
}

/// Clocks configutation
pub struct Config {
    pub(crate) master_src: MasterClockSrc,
    pub(crate) cpu_psc: Prescaler,
}

impl Config {
    pub fn new(src: MasterClockSrc) -> Self {
        Config::default().clock_src(src)
    }

    pub fn hsi(psc: Prescaler) -> Self {
        Config::default().clock_src(MasterClockSrc::HSI(psc))
    }

    pub fn lsi() -> Self {
        Config::default().clock_src(MasterClockSrc::LSI)
    }

    pub fn clock_src(mut self, src: MasterClockSrc) -> Self {
        self.master_src = src;
        self
    }

    pub fn cpu_psc(mut self, psc: Prescaler) -> Self {
        self.cpu_psc = psc;
        self
    }
}

impl Default for Config {
    fn default() -> Config {
        Config {
            master_src: MasterClockSrc::HSI(Prescaler::NotDivided),
            cpu_psc: Prescaler::NotDivided,
        }
    }
}
