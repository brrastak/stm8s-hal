#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    cr1: CR1,
    _reserved1: [u8; 0x02],
    ier: IER,
    sr1: SR1,
    sr2: SR2,
    egr: EGR,
    ccmr1: CCMR1,
    ccmr2: CCMR2,
    ccmr3: CCMR3,
    ccer1: CCER1,
    ccer2: CCER2,
    cntrh: CNTRH,
    cntrl: CNTRL,
    pscr: PSCR,
    arrh: ARRH,
    arrl: ARRL,
    ccr1h: CCR1H,
    ccr1l: CCR1L,
    ccr2h: CCR2H,
    ccr2l: CCR2L,
    ccr3h: CCR3H,
    ccr3l: CCR3L,
}
impl RegisterBlock {
    #[doc = "0x00 - TIM2 control register 1"]
    #[inline(always)]
    pub const fn cr1(&self) -> &CR1 {
        &self.cr1
    }
    #[doc = "0x03 - TIM2 Interrupt enable register"]
    #[inline(always)]
    pub const fn ier(&self) -> &IER {
        &self.ier
    }
    #[doc = "0x04 - TIM2 status register 1"]
    #[inline(always)]
    pub const fn sr1(&self) -> &SR1 {
        &self.sr1
    }
    #[doc = "0x05 - TIM2 status register 2"]
    #[inline(always)]
    pub const fn sr2(&self) -> &SR2 {
        &self.sr2
    }
    #[doc = "0x06 - TIM2 event generation register"]
    #[inline(always)]
    pub const fn egr(&self) -> &EGR {
        &self.egr
    }
    #[doc = "0x07 - TIM2 capture/compare mode register 1"]
    #[inline(always)]
    pub const fn ccmr1(&self) -> &CCMR1 {
        &self.ccmr1
    }
    #[doc = "0x08 - TIM2 capture/compare mode register 2"]
    #[inline(always)]
    pub const fn ccmr2(&self) -> &CCMR2 {
        &self.ccmr2
    }
    #[doc = "0x09 - TIM2 capture/compare mode register 3"]
    #[inline(always)]
    pub const fn ccmr3(&self) -> &CCMR3 {
        &self.ccmr3
    }
    #[doc = "0x0a - TIM2 capture/compare enable register 1"]
    #[inline(always)]
    pub const fn ccer1(&self) -> &CCER1 {
        &self.ccer1
    }
    #[doc = "0x0b - TIM2 capture/compare enable register 2"]
    #[inline(always)]
    pub const fn ccer2(&self) -> &CCER2 {
        &self.ccer2
    }
    #[doc = "0x0c - TIM2 counter high"]
    #[inline(always)]
    pub const fn cntrh(&self) -> &CNTRH {
        &self.cntrh
    }
    #[doc = "0x0d - TIM2 counter low"]
    #[inline(always)]
    pub const fn cntrl(&self) -> &CNTRL {
        &self.cntrl
    }
    #[doc = "0x0e - TIM2 prescaler register"]
    #[inline(always)]
    pub const fn pscr(&self) -> &PSCR {
        &self.pscr
    }
    #[doc = "0x0f - TIM2 auto-reload register high"]
    #[inline(always)]
    pub const fn arrh(&self) -> &ARRH {
        &self.arrh
    }
    #[doc = "0x10 - TIM2 auto-reload register low"]
    #[inline(always)]
    pub const fn arrl(&self) -> &ARRL {
        &self.arrl
    }
    #[doc = "0x11 - TIM2 capture/compare register 1 high"]
    #[inline(always)]
    pub const fn ccr1h(&self) -> &CCR1H {
        &self.ccr1h
    }
    #[doc = "0x12 - TIM2 capture/compare register 1 low"]
    #[inline(always)]
    pub const fn ccr1l(&self) -> &CCR1L {
        &self.ccr1l
    }
    #[doc = "0x13 - TIM2 capture/compare reg"]
    #[inline(always)]
    pub const fn ccr2h(&self) -> &CCR2H {
        &self.ccr2h
    }
    #[doc = "0x14 - TIM2 capture/compare register 2 low"]
    #[inline(always)]
    pub const fn ccr2l(&self) -> &CCR2L {
        &self.ccr2l
    }
    #[doc = "0x15 - TIM2 capture/compare register 3 high"]
    #[inline(always)]
    pub const fn ccr3h(&self) -> &CCR3H {
        &self.ccr3h
    }
    #[doc = "0x16 - TIM2 capture/compare register 3 low"]
    #[inline(always)]
    pub const fn ccr3l(&self) -> &CCR3L {
        &self.ccr3l
    }
}
#[doc = "CR1 (rw) register accessor: TIM2 control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr1`]
module"]
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
#[doc = "TIM2 control register 1"]
pub mod cr1;
#[doc = "IER (rw) register accessor: TIM2 Interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier`]
module"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "TIM2 Interrupt enable register"]
pub mod ier;
#[doc = "SR1 (rw) register accessor: TIM2 status register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr1`]
module"]
pub type SR1 = crate::Reg<sr1::SR1_SPEC>;
#[doc = "TIM2 status register 1"]
pub mod sr1;
#[doc = "SR2 (rw) register accessor: TIM2 status register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr2`]
module"]
pub type SR2 = crate::Reg<sr2::SR2_SPEC>;
#[doc = "TIM2 status register 2"]
pub mod sr2;
#[doc = "EGR (rw) register accessor: TIM2 event generation register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`egr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`egr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@egr`]
module"]
pub type EGR = crate::Reg<egr::EGR_SPEC>;
#[doc = "TIM2 event generation register"]
pub mod egr;
#[doc = "CCMR1 (rw) register accessor: TIM2 capture/compare mode register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccmr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccmr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccmr1`]
module"]
pub type CCMR1 = crate::Reg<ccmr1::CCMR1_SPEC>;
#[doc = "TIM2 capture/compare mode register 1"]
pub mod ccmr1;
#[doc = "CCMR2 (rw) register accessor: TIM2 capture/compare mode register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccmr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccmr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccmr2`]
module"]
pub type CCMR2 = crate::Reg<ccmr2::CCMR2_SPEC>;
#[doc = "TIM2 capture/compare mode register 2"]
pub mod ccmr2;
#[doc = "CCMR3 (rw) register accessor: TIM2 capture/compare mode register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccmr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccmr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccmr3`]
module"]
pub type CCMR3 = crate::Reg<ccmr3::CCMR3_SPEC>;
#[doc = "TIM2 capture/compare mode register 3"]
pub mod ccmr3;
#[doc = "CCER1 (rw) register accessor: TIM2 capture/compare enable register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccer1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccer1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccer1`]
module"]
pub type CCER1 = crate::Reg<ccer1::CCER1_SPEC>;
#[doc = "TIM2 capture/compare enable register 1"]
pub mod ccer1;
#[doc = "CCER2 (rw) register accessor: TIM2 capture/compare enable register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccer2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccer2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccer2`]
module"]
pub type CCER2 = crate::Reg<ccer2::CCER2_SPEC>;
#[doc = "TIM2 capture/compare enable register 2"]
pub mod ccer2;
#[doc = "CNTRH (rw) register accessor: TIM2 counter high\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cntrh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cntrh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cntrh`]
module"]
pub type CNTRH = crate::Reg<cntrh::CNTRH_SPEC>;
#[doc = "TIM2 counter high"]
pub mod cntrh;
#[doc = "CNTRL (rw) register accessor: TIM2 counter low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cntrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cntrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cntrl`]
module"]
pub type CNTRL = crate::Reg<cntrl::CNTRL_SPEC>;
#[doc = "TIM2 counter low"]
pub mod cntrl;
#[doc = "PSCR (rw) register accessor: TIM2 prescaler register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pscr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pscr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pscr`]
module"]
pub type PSCR = crate::Reg<pscr::PSCR_SPEC>;
#[doc = "TIM2 prescaler register"]
pub mod pscr;
#[doc = "ARRH (rw) register accessor: TIM2 auto-reload register high\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arrh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arrh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arrh`]
module"]
pub type ARRH = crate::Reg<arrh::ARRH_SPEC>;
#[doc = "TIM2 auto-reload register high"]
pub mod arrh;
#[doc = "ARRL (rw) register accessor: TIM2 auto-reload register low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arrl`]
module"]
pub type ARRL = crate::Reg<arrl::ARRL_SPEC>;
#[doc = "TIM2 auto-reload register low"]
pub mod arrl;
#[doc = "CCR1H (rw) register accessor: TIM2 capture/compare register 1 high\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr1h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr1h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr1h`]
module"]
pub type CCR1H = crate::Reg<ccr1h::CCR1H_SPEC>;
#[doc = "TIM2 capture/compare register 1 high"]
pub mod ccr1h;
#[doc = "CCR1L (rw) register accessor: TIM2 capture/compare register 1 low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr1l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr1l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr1l`]
module"]
pub type CCR1L = crate::Reg<ccr1l::CCR1L_SPEC>;
#[doc = "TIM2 capture/compare register 1 low"]
pub mod ccr1l;
#[doc = "CCR2H (rw) register accessor: TIM2 capture/compare reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr2h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr2h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr2h`]
module"]
pub type CCR2H = crate::Reg<ccr2h::CCR2H_SPEC>;
#[doc = "TIM2 capture/compare reg"]
pub mod ccr2h;
#[doc = "CCR2L (rw) register accessor: TIM2 capture/compare register 2 low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr2l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr2l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr2l`]
module"]
pub type CCR2L = crate::Reg<ccr2l::CCR2L_SPEC>;
#[doc = "TIM2 capture/compare register 2 low"]
pub mod ccr2l;
#[doc = "CCR3H (rw) register accessor: TIM2 capture/compare register 3 high\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr3h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr3h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr3h`]
module"]
pub type CCR3H = crate::Reg<ccr3h::CCR3H_SPEC>;
#[doc = "TIM2 capture/compare register 3 high"]
pub mod ccr3h;
#[doc = "CCR3L (rw) register accessor: TIM2 capture/compare register 3 low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr3l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr3l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr3l`]
module"]
pub type CCR3L = crate::Reg<ccr3l::CCR3L_SPEC>;
#[doc = "TIM2 capture/compare register 3 low"]
pub mod ccr3l;
