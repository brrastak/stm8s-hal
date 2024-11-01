//! Timers
//! 

use crate::clk::*;
use crate::pac::*;
use crate::prelude::DelayNs;


pub trait TimerExt<TIM> {
    
    /// Hardware timer for delay.
    /// Set to the frequency of 1MHz
    type Timer;

    /// Timer as delay provider.
    /// Master clock frequency should have one of the following values: 1, 2, 4, 8, 16 MHz
    fn new(self, clk: &Clk) -> Self::Timer;
}

pub struct Timer<TIM> {
    tim: TIM,
}

impl<TIM> TimerExt<TIM> for TIM where 
Timer<TIM>: TimerBase {

    type Timer = Timer<Self>;

    fn new(self, clk: &Clk) -> Timer<Self> {
        let new= Timer { tim: self };
        let psc_value = match clk.master_clk() / 1.MHz() as Hertz {
            1 => Prescaler::NotDivided,
            2 => Prescaler::Div2,
            4 => Prescaler::Div4,
            8 => Prescaler::Div8,
            16 => Prescaler::Div16,
            _ => panic!(),
        };
        new.set_psc(psc_value);
        new
    }
}

impl<TIM> DelayNs for Timer<TIM> where 
Timer<TIM>: TimerBase {
    
    fn delay_ns(&mut self, ns: u32) {
        let mut us = if ns > 1000 {
            ns / 1000
        }
        else {
            1
        };

        let max_value = Self::max_base_value();

        while us > max_value as u32 {
            self.internal_delay_us(max_value);
            us -= max_value as u32;
        }
        if us > 0 {
            self.internal_delay_us(us as u16);
        }
    }
}

enum Prescaler {
    NotDivided,
    Div2,
    Div4,
    Div8,
    Div16,
}

trait TimerBase {

    /// Maximum value that can be stored in counter and reload registers
    fn max_base_value() -> u16;
    
    /// Delay for N timer cycles = N us
    fn internal_delay_us(&self, us: u16) {
        // Start counting us timer cycles
        self.set_reload(us);
        self.reset();
        self.update();
        self.clear_update_flag();
        self.start();
        self.clear_update_flag();

        // Wait till counting is finished
        while !self.was_update_event() {}
        self.stop();
        self.clear_update_flag();
    }

    /// Set timer prescaler value
    fn set_psc(&self, psc: Prescaler);

    /// Set reload register value
    fn set_reload(&self, value: u16);

    /// Reset counter value
    fn reset(&self);

    /// Enable counting
    fn start(&self);

    /// Disable counting
    fn stop(&self);

    /// Generate update event
    fn update(&self);

    /// Return True after update event happend (counter == reload register)
    fn was_update_event(&self) -> bool;

    /// Should be called after update event happened
    fn clear_update_flag(&self);

}

macro_rules! timers {
    ($($TIM:ident: (
        $base_type:ident,
        ($pscrl:ident $(,$pscrh:ident)*),
        ($arrl:ident $(,$arrh:ident)*),
        ($cntrl:ident $(,$cntrh:ident)*),
        $sr:ident),
    )+) => {
        $(
            impl TimerBase for Timer<$TIM> {
    
                #[inline(always)]
                fn max_base_value() -> u16 {
                    $base_type::MAX as u16
                }

                #[inline(always)]
                fn set_psc(&self, psc: Prescaler) {
                    #[allow(unused)]
                    let value = match psc {
                        Prescaler::NotDivided => 0,
                        Prescaler::Div2 => 1,
                        Prescaler::Div4 => 2,
                        Prescaler::Div8 => 3,
                        Prescaler::Div16 => 4,
                    };
                    $(
                        let value = match psc {
                            Prescaler::NotDivided => 0,
                            Prescaler::Div2 => 1,
                            Prescaler::Div4 => 3,
                            Prescaler::Div8 => 7,
                            Prescaler::Div16 => 15,
                        };
                        self.tim.$pscrh().reset();
                    )*
                    unsafe {
                        self.tim.$pscrl().write(|w| w.bits(value));
                    }
                }

                #[inline(always)]
                fn set_reload(&self, value: u16) {
                    unsafe {
                        $(
                            self.tim.$arrh().write(|w| w.bits((value >> 8) as u8));
                        )*
                        self.tim.$arrl().write(|w| w.bits(value as u8));
                    }
                }

                #[inline(always)]
                fn reset(&self) {
                    $(
                        self.tim.$cntrh().reset();
                    )*
                    self.tim.$cntrl().reset();
                }

                #[inline(always)]
                fn start(&self) {
                    self.tim.cr1().modify(|_, w| w.cen().set_bit());
                }

                #[inline(always)]
                fn stop(&self) {
                    self.tim.cr1().modify(|_, w| w.cen().clear_bit());
                }

                #[inline(always)]
                fn update(&self) {
                    self.tim.egr().modify(|_, w| w.ug().set_bit());
                }

                #[inline(always)]
                fn was_update_event(&self) -> bool {
                    self.tim.$sr().read().uif().bit_is_set()
                }

                #[inline(always)]
                fn clear_update_flag(&self) {
                    self.tim.$sr().modify(|_, w| w.uif().clear_bit());
                }
            }

        )+
    }
}

timers! {
    TIM1: (u16, (pscrl, pscrh), (arrl, arrh), (cntrl, cntrh), sr1),
    TIM2: (u16, (pscr), (arrl, arrh), (cntrl, cntrh), sr1),
    TIM4: (u8, (pscr), (arr), (cntr), sr),
}
