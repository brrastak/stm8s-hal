#[doc = r"Enumeration of all the interrupts."]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Interrupt {
    #[doc = "1 - Interrupt_IRQ1 interrupt"]
    INTERRUPT_IRQ1 = 1,
    #[doc = "2 - Interrupt_IRQ2 interrupt"]
    INTERRUPT_IRQ2 = 2,
    #[doc = "10 - Interrupt_IRQ10 interrupt"]
    INTERRUPT_IRQ10 = 10,
    #[doc = "11 - Interrupt_IRQ11 interrupt"]
    INTERRUPT_IRQ11 = 11,
    #[doc = "12 - Interrupt_IRQ12 interrupt"]
    INTERRUPT_IRQ12 = 12,
    #[doc = "13 - Interrupt_IRQ13 interrupt"]
    INTERRUPT_IRQ13 = 13,
    #[doc = "14 - Interrupt_IRQ14 interrupt"]
    INTERRUPT_IRQ14 = 14,
    #[doc = "17 - Interrupt_IRQ17 interrupt"]
    INTERRUPT_IRQ17 = 17,
    #[doc = "18 - Interrupt_IRQ18 interrupt"]
    INTERRUPT_IRQ18 = 18,
    #[doc = "19 - Interrupt_IRQ19 interrupt"]
    INTERRUPT_IRQ19 = 19,
    #[doc = "22 - Interrupt_IRQ22 interrupt"]
    INTERRUPT_IRQ22 = 22,
    #[doc = "23 - Interrupt_IRQ23 interrupt"]
    INTERRUPT_IRQ23 = 23,
    #[doc = "24 - Interrupt_IRQ24 interrupt"]
    INTERRUPT_IRQ24 = 24,
}
#[doc = r" TryFromInterruptError"]
#[derive(Debug, Copy, Clone)]
pub struct TryFromInterruptError(());
impl Interrupt {
    #[doc = r" Attempt to convert a given value into an `Interrupt`"]
    #[inline]
    pub fn try_from(value: u8) -> Result<Self, TryFromInterruptError> {
        match value {
            1 => Ok(Interrupt::INTERRUPT_IRQ1),
            2 => Ok(Interrupt::INTERRUPT_IRQ2),
            10 => Ok(Interrupt::INTERRUPT_IRQ10),
            11 => Ok(Interrupt::INTERRUPT_IRQ11),
            12 => Ok(Interrupt::INTERRUPT_IRQ12),
            13 => Ok(Interrupt::INTERRUPT_IRQ13),
            14 => Ok(Interrupt::INTERRUPT_IRQ14),
            17 => Ok(Interrupt::INTERRUPT_IRQ17),
            18 => Ok(Interrupt::INTERRUPT_IRQ18),
            19 => Ok(Interrupt::INTERRUPT_IRQ19),
            22 => Ok(Interrupt::INTERRUPT_IRQ22),
            23 => Ok(Interrupt::INTERRUPT_IRQ23),
            24 => Ok(Interrupt::INTERRUPT_IRQ24),
            _ => Err(TryFromInterruptError(())),
        }
    }
}
