#![no_std]


pub use stm8s_pac as pac;

pub mod gpio;
pub mod clk;
pub mod prelude;
pub mod timer;


/// Incapsulating unsafe steal inside
/// It's not possible to properly implement harware-specific critical section
/// due to compiling into wasm
pub fn take_peripherals() -> pac::Peripherals {
    unsafe {
        pac::Peripherals::steal()
    }
}
