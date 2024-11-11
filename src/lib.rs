#![no_std]


pub use stm8s_pac as pac;

pub mod bitbang_uart;
pub mod adc;
pub mod gpio;
pub mod clk;
pub mod prelude;
pub mod timer;
pub mod uart;


/// Incapsulating unsafe steal inside
/// It's not possible to properly implement harware-specific critical section
/// due to compiling into wasm
pub fn take_peripherals() -> pac::Peripherals {
    unsafe {
        pac::Peripherals::steal()
    }
}
