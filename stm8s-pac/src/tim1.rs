#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    cr1: CR1,
    cr2: CR2,
    smcr: SMCR,
    etr: ETR,
    ier: IER,
    sr1: SR1,
    sr2: SR2,
    egr: EGR,
    ccmr1: CCMR1,
    ccmr2: CCMR2,
    ccmr3: CCMR3,
    ccmr4: CCMR4,
    ccer1: CCER1,
    ccer2: CCER2,
    cntrh: CNTRH,
    cntrl: CNTRL,
    pscrh: PSCRH,
    pscrl: PSCRL,
    arrh: ARRH,
    arrl: ARRL,
    rcr: RCR,
    ccr1h: CCR1H,
    ccr1l: CCR1L,
    ccr2h: CCR2H,
    ccr2l: CCR2L,
    ccr3h: CCR3H,
    ccr3l: CCR3L,
    ccr4h: CCR4H,
    ccr4l: CCR4L,
    bkr: BKR,
    dtr: DTR,
    oisr: OISR,
}
impl RegisterBlock {
    #[doc = "0x00 - TIM1 control register 1"]
    #[inline(always)]
    pub const fn cr1(&self) -> &CR1 {
        &self.cr1
    }
    #[doc = "0x01 - TIM1 control register 2"]
    #[inline(always)]
    pub const fn cr2(&self) -> &CR2 {
        &self.cr2
    }
    #[doc = "0x02 - TIM1 slave mode control register"]
    #[inline(always)]
    pub const fn smcr(&self) -> &SMCR {
        &self.smcr
    }
    #[doc = "0x03 - TIM1 external trigger register"]
    #[inline(always)]
    pub const fn etr(&self) -> &ETR {
        &self.etr
    }
    #[doc = "0x04 - TIM1 interrupt enable register"]
    #[inline(always)]
    pub const fn ier(&self) -> &IER {
        &self.ier
    }
    #[doc = "0x05 - TIM1 status register 1"]
    #[inline(always)]
    pub const fn sr1(&self) -> &SR1 {
        &self.sr1
    }
    #[doc = "0x06 - TIM1 status register 2"]
    #[inline(always)]
    pub const fn sr2(&self) -> &SR2 {
        &self.sr2
    }
    #[doc = "0x07 - TIM1 event generation register"]
    #[inline(always)]
    pub const fn egr(&self) -> &EGR {
        &self.egr
    }
    #[doc = "0x08 - TIM1 capture/compare mode register 1"]
    #[inline(always)]
    pub const fn ccmr1(&self) -> &CCMR1 {
        &self.ccmr1
    }
    #[doc = "0x09 - TIM1 capture/compare mode register 2"]
    #[inline(always)]
    pub const fn ccmr2(&self) -> &CCMR2 {
        &self.ccmr2
    }
    #[doc = "0x0a - TIM1 capture/compare mode register 3"]
    #[inline(always)]
    pub const fn ccmr3(&self) -> &CCMR3 {
        &self.ccmr3
    }
    #[doc = "0x0b - TIM1 capture/compare mode register 4"]
    #[inline(always)]
    pub const fn ccmr4(&self) -> &CCMR4 {
        &self.ccmr4
    }
    #[doc = "0x0c - TIM1 capture/compare enable register 1"]
    #[inline(always)]
    pub const fn ccer1(&self) -> &CCER1 {
        &self.ccer1
    }
    #[doc = "0x0d - TIM1 capture/compare enable register 2"]
    #[inline(always)]
    pub const fn ccer2(&self) -> &CCER2 {
        &self.ccer2
    }
    #[doc = "0x0e - TIM1 counter high"]
    #[inline(always)]
    pub const fn cntrh(&self) -> &CNTRH {
        &self.cntrh
    }
    #[doc = "0x0f - TIM1 counter low"]
    #[inline(always)]
    pub const fn cntrl(&self) -> &CNTRL {
        &self.cntrl
    }
    #[doc = "0x10 - TIM1 prescaler register high"]
    #[inline(always)]
    pub const fn pscrh(&self) -> &PSCRH {
        &self.pscrh
    }
    #[doc = "0x11 - TIM1 prescaler register low"]
    #[inline(always)]
    pub const fn pscrl(&self) -> &PSCRL {
        &self.pscrl
    }
    #[doc = "0x12 - TIM1 auto-reload register high"]
    #[inline(always)]
    pub const fn arrh(&self) -> &ARRH {
        &self.arrh
    }
    #[doc = "0x13 - TIM1 auto-reload register low"]
    #[inline(always)]
    pub const fn arrl(&self) -> &ARRL {
        &self.arrl
    }
    #[doc = "0x14 - TIM1 repetition counter register"]
    #[inline(always)]
    pub const fn rcr(&self) -> &RCR {
        &self.rcr
    }
    #[doc = "0x15 - TIM1 capture/compare register 1 high"]
    #[inline(always)]
    pub const fn ccr1h(&self) -> &CCR1H {
        &self.ccr1h
    }
    #[doc = "0x16 - TIM1 capture/compare register 1 low"]
    #[inline(always)]
    pub const fn ccr1l(&self) -> &CCR1L {
        &self.ccr1l
    }
    #[doc = "0x17 - TIM1 capture/compare register 2 high"]
    #[inline(always)]
    pub const fn ccr2h(&self) -> &CCR2H {
        &self.ccr2h
    }
    #[doc = "0x18 - TIM1 capture/compare register 2 low"]
    #[inline(always)]
    pub const fn ccr2l(&self) -> &CCR2L {
        &self.ccr2l
    }
    #[doc = "0x19 - TIM1 capture/compare register 3 high"]
    #[inline(always)]
    pub const fn ccr3h(&self) -> &CCR3H {
        &self.ccr3h
    }
    #[doc = "0x1a - TIM1 capture/compare register 3 low"]
    #[inline(always)]
    pub const fn ccr3l(&self) -> &CCR3L {
        &self.ccr3l
    }
    #[doc = "0x1b - TIM1 capture/compare register 4 high"]
    #[inline(always)]
    pub const fn ccr4h(&self) -> &CCR4H {
        &self.ccr4h
    }
    #[doc = "0x1c - TIM1 capture/compare register 4 low"]
    #[inline(always)]
    pub const fn ccr4l(&self) -> &CCR4L {
        &self.ccr4l
    }
    #[doc = "0x1d - TIM1 break register"]
    #[inline(always)]
    pub const fn bkr(&self) -> &BKR {
        &self.bkr
    }
    #[doc = "0x1e - TIM1 dead-time register"]
    #[inline(always)]
    pub const fn dtr(&self) -> &DTR {
        &self.dtr
    }
    #[doc = "0x1f - TIM1 output idle state register"]
    #[inline(always)]
    pub const fn oisr(&self) -> &OISR {
        &self.oisr
    }
}
#[doc = "CR1 (rw) register accessor: TIM1 control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr1`]
module"]
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
#[doc = "TIM1 control register 1"]
pub mod cr1;
#[doc = "CR2 (rw) register accessor: TIM1 control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr2`]
module"]
pub type CR2 = crate::Reg<cr2::CR2_SPEC>;
#[doc = "TIM1 control register 2"]
pub mod cr2;
#[doc = "SMCR (rw) register accessor: TIM1 slave mode control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smcr`]
module"]
pub type SMCR = crate::Reg<smcr::SMCR_SPEC>;
#[doc = "TIM1 slave mode control register"]
pub mod smcr;
#[doc = "ETR (rw) register accessor: TIM1 external trigger register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`etr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etr`]
module"]
pub type ETR = crate::Reg<etr::ETR_SPEC>;
#[doc = "TIM1 external trigger register"]
pub mod etr;
#[doc = "IER (rw) register accessor: TIM1 interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier`]
module"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "TIM1 interrupt enable register"]
pub mod ier;
#[doc = "SR1 (rw) register accessor: TIM1 status register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr1`]
module"]
pub type SR1 = crate::Reg<sr1::SR1_SPEC>;
#[doc = "TIM1 status register 1"]
pub mod sr1;
#[doc = "SR2 (rw) register accessor: TIM1 status register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr2`]
module"]
pub type SR2 = crate::Reg<sr2::SR2_SPEC>;
#[doc = "TIM1 status register 2"]
pub mod sr2;
#[doc = "EGR (rw) register accessor: TIM1 event generation register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`egr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`egr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@egr`]
module"]
pub type EGR = crate::Reg<egr::EGR_SPEC>;
#[doc = "TIM1 event generation register"]
pub mod egr;
#[doc = "CCMR1 (rw) register accessor: TIM1 capture/compare mode register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccmr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccmr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccmr1`]
module"]
pub type CCMR1 = crate::Reg<ccmr1::CCMR1_SPEC>;
#[doc = "TIM1 capture/compare mode register 1"]
pub mod ccmr1;
#[doc = "CCMR2 (rw) register accessor: TIM1 capture/compare mode register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccmr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccmr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccmr2`]
module"]
pub type CCMR2 = crate::Reg<ccmr2::CCMR2_SPEC>;
#[doc = "TIM1 capture/compare mode register 2"]
pub mod ccmr2;
#[doc = "CCMR3 (rw) register accessor: TIM1 capture/compare mode register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccmr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccmr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccmr3`]
module"]
pub type CCMR3 = crate::Reg<ccmr3::CCMR3_SPEC>;
#[doc = "TIM1 capture/compare mode register 3"]
pub mod ccmr3;
#[doc = "CCMR4 (rw) register accessor: TIM1 capture/compare mode register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccmr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccmr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccmr4`]
module"]
pub type CCMR4 = crate::Reg<ccmr4::CCMR4_SPEC>;
#[doc = "TIM1 capture/compare mode register 4"]
pub mod ccmr4;
#[doc = "CCER1 (rw) register accessor: TIM1 capture/compare enable register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccer1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccer1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccer1`]
module"]
pub type CCER1 = crate::Reg<ccer1::CCER1_SPEC>;
#[doc = "TIM1 capture/compare enable register 1"]
pub mod ccer1;
#[doc = "CCER2 (rw) register accessor: TIM1 capture/compare enable register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccer2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccer2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccer2`]
module"]
pub type CCER2 = crate::Reg<ccer2::CCER2_SPEC>;
#[doc = "TIM1 capture/compare enable register 2"]
pub mod ccer2;
#[doc = "CNTRH (rw) register accessor: TIM1 counter high\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cntrh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cntrh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cntrh`]
module"]
pub type CNTRH = crate::Reg<cntrh::CNTRH_SPEC>;
#[doc = "TIM1 counter high"]
pub mod cntrh;
#[doc = "CNTRL (rw) register accessor: TIM1 counter low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cntrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cntrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cntrl`]
module"]
pub type CNTRL = crate::Reg<cntrl::CNTRL_SPEC>;
#[doc = "TIM1 counter low"]
pub mod cntrl;
#[doc = "PSCRH (rw) register accessor: TIM1 prescaler register high\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pscrh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pscrh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pscrh`]
module"]
pub type PSCRH = crate::Reg<pscrh::PSCRH_SPEC>;
#[doc = "TIM1 prescaler register high"]
pub mod pscrh;
#[doc = "PSCRL (rw) register accessor: TIM1 prescaler register low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pscrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pscrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pscrl`]
module"]
pub type PSCRL = crate::Reg<pscrl::PSCRL_SPEC>;
#[doc = "TIM1 prescaler register low"]
pub mod pscrl;
#[doc = "ARRH (rw) register accessor: TIM1 auto-reload register high\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arrh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arrh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arrh`]
module"]
pub type ARRH = crate::Reg<arrh::ARRH_SPEC>;
#[doc = "TIM1 auto-reload register high"]
pub mod arrh;
#[doc = "ARRL (rw) register accessor: TIM1 auto-reload register low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arrl`]
module"]
pub type ARRL = crate::Reg<arrl::ARRL_SPEC>;
#[doc = "TIM1 auto-reload register low"]
pub mod arrl;
#[doc = "RCR (rw) register accessor: TIM1 repetition counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcr`]
module"]
pub type RCR = crate::Reg<rcr::RCR_SPEC>;
#[doc = "TIM1 repetition counter register"]
pub mod rcr;
#[doc = "CCR1H (rw) register accessor: TIM1 capture/compare register 1 high\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr1h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr1h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr1h`]
module"]
pub type CCR1H = crate::Reg<ccr1h::CCR1H_SPEC>;
#[doc = "TIM1 capture/compare register 1 high"]
pub mod ccr1h;
#[doc = "CCR1L (rw) register accessor: TIM1 capture/compare register 1 low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr1l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr1l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr1l`]
module"]
pub type CCR1L = crate::Reg<ccr1l::CCR1L_SPEC>;
#[doc = "TIM1 capture/compare register 1 low"]
pub mod ccr1l;
#[doc = "CCR2H (rw) register accessor: TIM1 capture/compare register 2 high\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr2h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr2h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr2h`]
module"]
pub type CCR2H = crate::Reg<ccr2h::CCR2H_SPEC>;
#[doc = "TIM1 capture/compare register 2 high"]
pub mod ccr2h;
#[doc = "CCR2L (rw) register accessor: TIM1 capture/compare register 2 low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr2l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr2l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr2l`]
module"]
pub type CCR2L = crate::Reg<ccr2l::CCR2L_SPEC>;
#[doc = "TIM1 capture/compare register 2 low"]
pub mod ccr2l;
#[doc = "CCR3H (rw) register accessor: TIM1 capture/compare register 3 high\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr3h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr3h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr3h`]
module"]
pub type CCR3H = crate::Reg<ccr3h::CCR3H_SPEC>;
#[doc = "TIM1 capture/compare register 3 high"]
pub mod ccr3h;
#[doc = "CCR3L (rw) register accessor: TIM1 capture/compare register 3 low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr3l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr3l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr3l`]
module"]
pub type CCR3L = crate::Reg<ccr3l::CCR3L_SPEC>;
#[doc = "TIM1 capture/compare register 3 low"]
pub mod ccr3l;
#[doc = "CCR4H (rw) register accessor: TIM1 capture/compare register 4 high\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr4h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr4h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr4h`]
module"]
pub type CCR4H = crate::Reg<ccr4h::CCR4H_SPEC>;
#[doc = "TIM1 capture/compare register 4 high"]
pub mod ccr4h;
#[doc = "CCR4L (rw) register accessor: TIM1 capture/compare register 4 low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr4l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr4l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr4l`]
module"]
pub type CCR4L = crate::Reg<ccr4l::CCR4L_SPEC>;
#[doc = "TIM1 capture/compare register 4 low"]
pub mod ccr4l;
#[doc = "BKR (rw) register accessor: TIM1 break register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bkr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bkr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkr`]
module"]
pub type BKR = crate::Reg<bkr::BKR_SPEC>;
#[doc = "TIM1 break register"]
pub mod bkr;
#[doc = "DTR (rw) register accessor: TIM1 dead-time register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dtr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtr`]
module"]
pub type DTR = crate::Reg<dtr::DTR_SPEC>;
#[doc = "TIM1 dead-time register"]
pub mod dtr;
#[doc = "OISR (rw) register accessor: TIM1 output idle state register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`oisr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`oisr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oisr`]
module"]
pub type OISR = crate::Reg<oisr::OISR_SPEC>;
#[doc = "TIM1 output idle state register"]
pub mod oisr;
