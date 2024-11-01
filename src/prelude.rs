
pub use embedded_hal as hal;
pub use hal::{
    delay::DelayNs,
    digital::*,
};

pub use crate::clk::ClkExt as _;
pub use crate::gpio::GpioExt as _;
pub use crate::timer::TimerExt as _;
