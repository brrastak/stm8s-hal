//! Independent watchdog
//!

use crate::pac::*;


impl Iwdg {

    /// Create new instance and start watchdog timer.
    /// Inner timer set to 1kHz and so timeout is in ms
    pub fn new(iwdg: IWDG, timeout_ms: u8) -> Self {
        let iwdg = Iwdg { iwdg };

        iwdg.write_key(Key::Enable);
        iwdg.write_key(Key::Access);
        unsafe {
            // Clock frequency is 64kHz
            // Set prescaler to 64 to get 1kHz
            iwdg.iwdg.pr().write(|w| w.bits(0b100));
            // Set number of ms for timeout
            iwdg.iwdg.rlr().write(|w| w.bits(timeout_ms));
        }
        iwdg.write_key(Key::Refresh);

        iwdg
    }

    /// Refresh watchdog timer
    pub fn refresh(&self) {
        self.write_key(Key::Refresh);
    }

    #[inline(always)]
    fn write_key(&self, key: Key) {
        let value = match key {
            Key::Enable => 0xCC,
            Key::Access => 0x55,
            Key::Refresh => 0xAA,
        };
        unsafe {
            self.iwdg.kr().write(|w| w.bits(value));
        }
    }
}

/// Watchdog
pub struct Iwdg
{
    iwdg: IWDG,
}

enum Key {
    Enable,
    Access,
    Refresh
}
