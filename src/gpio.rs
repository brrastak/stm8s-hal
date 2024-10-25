//! General Purpose Input / Output

use core::convert::Infallible;
use core::marker::PhantomData;
use embedded_hal::digital::{
    ErrorType,
    InputPin,
    OutputPin,
    PinState,
    StatefulOutputPin
};

/// Default pin mode
pub type DefaultMode = Floating;

/// Extension trait to split a GPIO peripheral in independent pins and registers
pub trait GpioExt {
    /// The parts to split the GPIO into
    type Parts;

    /// Splits the GPIO block into independent pins and registers
    fn split(self) -> Self::Parts;
}

trait GpioRegExt {
    fn is_low(&self, pos: u8) -> bool;
    fn is_set_low(&self, pos: u8) -> bool;
    fn set_high(&self, pos: u8);
    fn set_low(&self, pos: u8);
}

/// Input mode (type state)
pub struct Input<MODE> {
    _mode: PhantomData<MODE>,
}

/// Floating input (type state)
pub struct Floating;

/// Pulled down input (type state)
pub struct PullDown;

/// Pulled up input (type state)
pub struct PullUp;

/// Open drain input or output (type state)
pub struct OpenDrain;

/// Analog mode (type state)
pub struct Analog;

/// Output mode (type state)
pub struct Output<MODE> {
    _mode: PhantomData<MODE>,
}

/// Push pull output (type state)
pub struct PushPull;

/// Fully erased pin
pub struct Pin<MODE> {
    i: u8,
    gpio: *const dyn GpioRegExt,
    _mode: PhantomData<MODE>,
}

macro_rules! gpio_trait {
    ($portx:ident) => {
        impl GpioRegExt for crate::pac::$portx::RegisterBlock {
            fn is_low(&self, pos: u8) -> bool {
                // NOTE(unsafe) atomic read with no side effects
                self.idr().read().bits() & (1 << pos) == 0
            }

            fn is_set_low(&self, pos: u8) -> bool {
                // NOTE(unsafe) atomic read with no side effects
                self.odr().read().bits() & (1 << pos) == 0
            }

            fn set_high(&self, pos: u8) {
                unsafe { self.odr().modify(|r, w| w.bits(r.bits() | 1 << pos)) }
            }

            fn set_low(&self, pos: u8) {
                unsafe { self.odr().modify(|r, w| w.bits(r.bits() & !(1 << pos))) }
            }
        }
    };
}

gpio_trait!(porta);
gpio_trait!(portb);
gpio_trait!(portc);
gpio_trait!(portd);

impl<MODE> ErrorType for Pin<Input<MODE>> {
    type Error = Infallible;
}

impl<MODE> ErrorType for Pin<Output<MODE>> {
    type Error = Infallible;
}

impl<MODE> StatefulOutputPin for Pin<Output<MODE>> {

    #[inline(always)]
    fn is_set_high(&mut self) -> Result<bool, Self::Error> {
        self.is_set_low().map(|v| !v)
    }

    #[inline(always)]
    fn is_set_low(&mut self) -> Result<bool, Self::Error> {
        Ok(unsafe { (*self.gpio).is_set_low(self.i) })
    }
}

impl<MODE> OutputPin for Pin<Output<MODE>> {

    #[inline(always)]
    fn set_high(&mut self) -> Result<(), Self::Error> {
        unsafe { (*self.gpio).set_high(self.i) };
        Ok(())
    }

    #[inline(always)]
    fn set_low(&mut self) -> Result<(), Self::Error> {
        unsafe { (*self.gpio).set_low(self.i) }
        Ok(())
    }
}

impl InputPin for Pin<Output<OpenDrain>> {

    #[inline(always)]
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        self.is_low().map(|v| !v)
    }

    #[inline(always)]
    fn is_low(&mut self) -> Result<bool, Self::Error> {
        Ok(unsafe { (*self.gpio).is_low(self.i) })
    }
}

impl<MODE> InputPin for Pin<Input<MODE>> {

    #[inline(always)]
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        self.is_low().map(|v| !v)
    }

    #[inline(always)]
    fn is_low(&mut self) -> Result<bool, Self::Error> {
        Ok(unsafe { (*self.gpio).is_low(self.i) })
    }
}

/// GPIO Pin output speed selection
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Speed {
    /// Output speed up to 2MHz
    Low,
    /// Output speed up to 10MHz   
    High,   
}

macro_rules! gpio {
    ($PORTX:ident, $portx:ident, $PXx:ident, $Pxn:expr, [
        $($PXi:ident: ($pxi:ident, $i:expr),)+
    ]) => {
        /// GPIO
        pub mod $portx {
            use core::convert::Infallible;
            use core::marker::PhantomData;
            use crate::pac::$PORTX;
            use embedded_hal::digital::{
                InputPin,
                OutputPin,
                StatefulOutputPin
            };
            use super::*;

            /// GPIO parts
            pub struct Parts {
                $(
                    pub $pxi: $PXi<DefaultMode>,
                )+
            }

            impl GpioExt for $PORTX {
                type Parts = Parts;

                fn split(self) -> Parts {
                    Parts {
                        $(
                            $pxi: $PXi { _mode: PhantomData },
                        )+
                    }
                }
            }

            /// Partially erased pin
            pub struct $PXx<MODE> {
                i: u8,
                _mode: PhantomData<MODE>,
            }

            impl<MODE> ErrorType for $PXx<Output<MODE>> {
                type Error = Infallible;
            }

            impl<MODE> OutputPin for $PXx<Output<MODE>> {
                fn set_high(&mut self) -> Result<(), Self::Error> {
                    unsafe { (*$PORTX::ptr()).odr().modify(|r, w| w.bits(r.bits() | 1 << self.i)) };
                    Ok(())
                }

                fn set_low(&mut self) -> Result<(), Self::Error> {
                    unsafe { (*$PORTX::ptr()).odr().modify(|r, w| w.bits(r.bits() & !(1 << self.i))) };
                    Ok(())
                }
            }

            impl<MODE> StatefulOutputPin for $PXx<Output<MODE>> {
                fn is_set_high(&mut self) -> Result<bool, Self::Error> {
                    let is_set_high = !self.is_set_low()?;
                    Ok(is_set_high)
                }

                fn is_set_low(&mut self) -> Result<bool, Self::Error> {
                    // NOTE(unsafe) atomic read with no side effects
                    let is_set_low = unsafe { (*$PORTX::ptr()).odr().read().bits() & (1 << self.i) == 0 };
                    Ok(is_set_low)
                }
            }

            impl<MODE> ErrorType for $PXx<Input<MODE>> {
                type Error = Infallible;
            }

            impl<MODE> InputPin for $PXx<Output<MODE>> {
                fn is_high(&mut self) -> Result<bool, Self::Error> {
                    let is_high = !self.is_low()?;
                    Ok(is_high)
                }

                fn is_low(&mut self) -> Result<bool, Self::Error>  {
                    // NOTE(unsafe) atomic read with no side effects
                    let is_low = unsafe { (*$PORTX::ptr()).idr().read().bits() & (1 << self.i) == 0 };
                    Ok(is_low)
                }
            }

            impl<MODE> InputPin for $PXx<Input<MODE>> {
                fn is_high(&mut self) -> Result<bool, Self::Error> {
                    let is_high = !self.is_low()?;
                    Ok(is_high)
                }

                fn is_low(&mut self) -> Result<bool, Self::Error> {
                    // NOTE(unsafe) atomic read with no side effects
                    let is_low = unsafe { (*$PORTX::ptr()).idr().read().bits() & (1 << self.i) == 0 };
                    Ok(is_low)
                }
            }

            $(
                pub struct $PXi<MODE> {
                    _mode: PhantomData<MODE>,
                }

                #[allow(clippy::from_over_into)]
                impl Into<$PXi<Input<PullUp>>> for $PXi<DefaultMode> {
                    fn into(self) -> $PXi<Input<PullUp>> {
                        self.into_pull_up_input()
                    }
                }

                #[allow(clippy::from_over_into)]
                impl Into<$PXi<Input<Floating>>> for $PXi<DefaultMode> {
                    fn into(self) -> $PXi<Input<Floating>> {
                        self.into_floating_input()
                    }
                }

                #[allow(clippy::from_over_into)]
                impl Into<$PXi<Output<OpenDrain>>> for $PXi<DefaultMode> {
                    fn into(self) -> $PXi<Output<OpenDrain>> {
                        self.into_open_drain_output()
                    }
                }

                #[allow(clippy::from_over_into)]
                impl Into<$PXi<Output<PushPull>>> for $PXi<DefaultMode> {
                    fn into(self) -> $PXi<Output<PushPull>> {
                        self.into_push_pull_output()
                    }
                }

                impl<MODE> $PXi<MODE> {
                    /// Configures the pin to operate as a floating input pin
                    pub fn into_floating_input(self) -> $PXi<Input<Floating>> {
                        unsafe {
                            let gpio = &(*$PORTX::ptr());
                            gpio.ddr().modify(|r, w| {
                                w.bits(r.bits() & !(1 << $i))
                            });
                            gpio.cr1().modify(|r, w| {
                                w.bits(r.bits() & !(1 << $i))
                            });
                            gpio.cr2().modify(|r, w| {
                                w.bits(r.bits() & !(1 << $i))
                            });
                        };
                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin to operate as a pulled up input pin
                    pub fn into_pull_up_input(self) -> $PXi<Input<PullUp>> {
                        unsafe {
                            let gpio = &(*$PORTX::ptr());
                            gpio.ddr().modify(|r, w| {
                                w.bits(r.bits() & !(1 << $i))
                            });
                            gpio.cr1().modify(|r, w| {
                                w.bits(r.bits() | (1 << $i))
                            });
                            gpio.cr2().modify(|r, w| {
                                w.bits(r.bits() & !(1 << $i))
                            });
                        };
                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin to operate as an open drain output
                    /// pin with `initial_state` specifying whether the pin
                    /// should initially be high or low
                    pub fn into_open_drain_output_in_state(mut self, initial_state: PinState) -> $PXi<Output<OpenDrain>> {
                        self.internal_set_state(initial_state);
                        self.into_open_drain_output()
                    }

                    /// Configures the pin to operate as an open drain output pin
                    pub fn into_open_drain_output(self) -> $PXi<Output<OpenDrain>> {
                        unsafe {
                            let gpio = &(*$PORTX::ptr());
                            gpio.ddr().modify(|r, w| {
                                w.bits(r.bits() | (1 << $i))
                            });
                            gpio.cr1().modify(|r, w| {
                                w.bits(r.bits() & !(1 << $i))
                            });
                            gpio.cr2().modify(|r, w| {
                                w.bits(r.bits() | (1 << $i))
                            });
                        };
                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin to operate as a push pull output pin
                    /// with `initial_state` specifying whether the pin should
                    /// initially be high or low
                    pub fn into_push_pull_output_in_state(mut self, initial_state: PinState) -> $PXi<Output<PushPull>> {
                        self.internal_set_state(initial_state);
                        self.into_push_pull_output()
                    }

                    /// Configures the pin to operate as a push pull output pin
                    pub fn into_push_pull_output(self) -> $PXi<Output<PushPull>> {
                        unsafe {
                            let gpio = &(*$PORTX::ptr());
                            gpio.ddr().modify(|r, w| {
                                w.bits(r.bits() | (1 << $i))
                            });
                            gpio.cr1().modify(|r, w| {
                                w.bits(r.bits() | (1 << $i))
                            });
                            gpio.cr2().modify(|r, w| {
                                w.bits(r.bits() | (1 << $i))
                            });
                        };
                        $PXi { _mode: PhantomData }
                    }

                    /// Set pin speed
                    pub fn set_speed(self, speed: Speed) -> Self {
                        unsafe {
                            let gpio = &(*$PORTX::ptr());
                            match speed {
                                Speed::Low => gpio.cr2().modify(|r, w| {
                                    w.bits(r.bits() & !(1 << $i))
                                }),
                                Speed::High => gpio.cr2().modify(|r, w| {
                                    w.bits(r.bits() | (1 << $i))
                                })
                            };
                        };
                        self
                    }

                    fn internal_set_state(&mut self, state: PinState) {
                        match state {
                            PinState::High => {
                                unsafe { (*$PORTX::ptr()).odr().modify(|r, w| w.bits(r.bits() | 1 << $i)) };
                            }
                            PinState::Low => {
                                unsafe { (*$PORTX::ptr()).odr().modify(|r, w| w.bits(r.bits() & !(1 << $i))) };
                            }
                        }
                    }
                }

                impl<MODE> $PXi<Output<MODE>> {
                    /// Erases the pin number from the type
                    ///
                    /// This is useful when you want to collect the pins into an array where you
                    /// need all the elements to have the same type
                    pub fn downgrade(self) -> $PXx<Output<MODE>> {
                        $PXx { i: $i, _mode: self._mode }
                    }
                }

                impl<MODE> ErrorType for $PXi<Output<MODE>> {
                    type Error = Infallible;
                }

                impl<MODE> OutputPin for $PXi<Output<MODE>> {
                    fn set_high(&mut self) -> Result<(), Self::Error> {
                        self.internal_set_state(PinState::High);
                        Ok(())
                    }

                    fn set_low(&mut self) -> Result<(), Self::Error>{
                        self.internal_set_state(PinState::Low);
                        Ok(())
                    }
                }

                impl<MODE> StatefulOutputPin for $PXi<Output<MODE>> {
                    fn is_set_high(&mut self) -> Result<bool, Self::Error> {
                        let is_set_high = !self.is_set_low()?;
                        Ok(is_set_high)
                    }

                    fn is_set_low(&mut self) -> Result<bool, Self::Error> {
                        // NOTE(unsafe) atomic read with no side effects
                        let is_set_low = unsafe { (*$PORTX::ptr()).odr().read().bits() & (1 << $i) == 0 };
                        Ok(is_set_low)
                    }
                }

                impl<MODE> InputPin for $PXi<Output<MODE>> {
                    fn is_high(&mut self) -> Result<bool, Self::Error> {
                        let is_high = !self.is_low()?;
                        Ok(is_high)
                    }

                    fn is_low(&mut self) -> Result<bool, Self::Error>  {
                        // NOTE(unsafe) atomic read with no side effects
                        let is_low = unsafe { (*$PORTX::ptr()).idr().read().bits() & (1 << $i) == 0 };
                        Ok(is_low)
                    }
                }

                impl<MODE> $PXi<Input<MODE>> {
                    /// Erases the pin number from the type
                    ///
                    /// This is useful when you want to collect the pins into an array where you
                    /// need all the elements to have the same type
                    pub fn downgrade(self) -> $PXx<Input<MODE>> {
                        $PXx { i: $i, _mode: self._mode }
                    }
                }

                impl<MODE> ErrorType for $PXi<Input<MODE>> {
                    type Error = Infallible;
                }

                impl<MODE> InputPin for $PXi<Input<MODE>> {
                    fn is_high(&mut self) -> Result<bool, Self::Error> {
                        let is_high = !self.is_low()?;
                        Ok(is_high)
                    }

                    fn is_low(&mut self) -> Result<bool, Self::Error> {
                        // NOTE(unsafe) atomic read with no side effects
                        let is_low = unsafe { (*$PORTX::ptr()).idr().read().bits() & (1 << $i) == 0 };
                        Ok(is_low)
                    }
                }
            )+

            impl<TYPE> $PXx<TYPE> {
                pub fn get_id (&self) -> u8 {
                    self.i
                }
            }

            impl<MODE> $PXx<Output<MODE>> {
                /// Erases the gpio number from the type
                ///
                /// This is useful when you want to collect the pins into an array where you
                /// need all the elements to have the same type
                pub fn downgrade(self) -> Pin<Output<MODE>> {
                    Pin {
                        i: self.get_id(),
                        gpio: $PORTX::ptr() as *const dyn GpioRegExt,
                        _mode: self._mode,
                    }
                }
            }

            impl<MODE> $PXx<Input<MODE>> {
                /// Erases the gpio number from the type
                ///
                /// This is useful when you want to collect the pins into an array where you
                /// need all the elements to have the same type
                pub fn downgrade(self) -> Pin<Input<MODE>> {
                    Pin {
                        i: self.get_id(),
                        gpio: $PORTX::ptr() as *const dyn GpioRegExt,
                        _mode: self._mode,
                    }
                }
            }
        }

        pub use $portx::{ $($PXi,)+ };
    }
}

gpio!(PORTA, gpioa, PA, 0, [
    PA0: (pa0, 0),
    PA1: (pa1, 1),
    PA2: (pa2, 2),
    PA3: (pa3, 3),
    PA4: (pa4, 4),
    PA5: (pa5, 5),
    PA6: (pa6, 6),
    PA7: (pa7, 7),
]);

gpio!(PORTB, gpiob, PB, 1, [
    PB0: (pb0, 0),
    PB1: (pb1, 1),
    PB2: (pb2, 2),
    PB3: (pb3, 3),
    PB4: (pb4, 4),
    PB5: (pb5, 5),
    PB6: (pb6, 6),
    PB7: (pb7, 7),
]);

gpio!(PORTC, gpioc, PC, 2, [
    PC0: (pc0, 0),
    PC1: (pc1, 1),
    PC2: (pc2, 2),
    PC3: (pc3, 3),
    PC4: (pc4, 4),
    PC5: (pc5, 5),
    PC6: (pc6, 6),
    PC7: (pc7, 7),
]);

gpio!(PORTD, gpiod, PD, 3, [
    PD0: (pd0, 0),
    PD1: (pd1, 1),
    PD2: (pd2, 2),
    PD3: (pd3, 3),
    PD4: (pd4, 4),
    PD5: (pd5, 5),
    PD6: (pd6, 6),
    PD7: (pd7, 7),
]);
