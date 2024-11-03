
pub use embedded_hal as hal;
pub use embedded_io as io;
pub use hal::{
    delay::DelayNs,
    digital::*,
};
pub use io::{
    ErrorType,
    Write,
};

pub use crate::clk::ClkExt as _;
pub use crate::gpio::GpioExt as _;
pub use crate::timer::TimerExt as _;
pub use crate::bitbang_uart::*;
