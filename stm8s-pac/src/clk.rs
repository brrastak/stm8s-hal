#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    ickr: ICKR,
    eckr: ECKR,
    _reserved2: [u8; 0x01],
    cmsr: CMSR,
    swr: SWR,
    swcr: SWCR,
    ckdivr: CKDIVR,
    pckenr1: PCKENR1,
    cssr: CSSR,
    ccor: CCOR,
    pckenr2: PCKENR2,
    canccr: CANCCR,
    hsitrimr: HSITRIMR,
    swimccr: SWIMCCR,
}
impl RegisterBlock {
    #[doc = "0x00 - Internal clock control register"]
    #[inline(always)]
    pub const fn ickr(&self) -> &ICKR {
        &self.ickr
    }
    #[doc = "0x01 - External clock control register"]
    #[inline(always)]
    pub const fn eckr(&self) -> &ECKR {
        &self.eckr
    }
    #[doc = "0x03 - Clock master status register"]
    #[inline(always)]
    pub const fn cmsr(&self) -> &CMSR {
        &self.cmsr
    }
    #[doc = "0x04 - Clock master switch register"]
    #[inline(always)]
    pub const fn swr(&self) -> &SWR {
        &self.swr
    }
    #[doc = "0x05 - Clock switch control register"]
    #[inline(always)]
    pub const fn swcr(&self) -> &SWCR {
        &self.swcr
    }
    #[doc = "0x06 - Clock divider register"]
    #[inline(always)]
    pub const fn ckdivr(&self) -> &CKDIVR {
        &self.ckdivr
    }
    #[doc = "0x07 - Peripheral clock gating register 1"]
    #[inline(always)]
    pub const fn pckenr1(&self) -> &PCKENR1 {
        &self.pckenr1
    }
    #[doc = "0x08 - Clock security system register"]
    #[inline(always)]
    pub const fn cssr(&self) -> &CSSR {
        &self.cssr
    }
    #[doc = "0x09 - Configurable clock control register"]
    #[inline(always)]
    pub const fn ccor(&self) -> &CCOR {
        &self.ccor
    }
    #[doc = "0x0a - Peripheral clock gating register 2"]
    #[inline(always)]
    pub const fn pckenr2(&self) -> &PCKENR2 {
        &self.pckenr2
    }
    #[doc = "0x0b - CAN clock control register"]
    #[inline(always)]
    pub const fn canccr(&self) -> &CANCCR {
        &self.canccr
    }
    #[doc = "0x0c - HSI clock calibration trimming register"]
    #[inline(always)]
    pub const fn hsitrimr(&self) -> &HSITRIMR {
        &self.hsitrimr
    }
    #[doc = "0x0d - SWIM clock control register"]
    #[inline(always)]
    pub const fn swimccr(&self) -> &SWIMCCR {
        &self.swimccr
    }
}
#[doc = "ICKR (rw) register accessor: Internal clock control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ickr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ickr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ickr`]
module"]
pub type ICKR = crate::Reg<ickr::ICKR_SPEC>;
#[doc = "Internal clock control register"]
pub mod ickr;
#[doc = "ECKR (rw) register accessor: External clock control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eckr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eckr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eckr`]
module"]
pub type ECKR = crate::Reg<eckr::ECKR_SPEC>;
#[doc = "External clock control register"]
pub mod eckr;
#[doc = "CMSR (rw) register accessor: Clock master status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmsr`]
module"]
pub type CMSR = crate::Reg<cmsr::CMSR_SPEC>;
#[doc = "Clock master status register"]
pub mod cmsr;
#[doc = "SWR (rw) register accessor: Clock master switch register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swr`]
module"]
pub type SWR = crate::Reg<swr::SWR_SPEC>;
#[doc = "Clock master switch register"]
pub mod swr;
#[doc = "SWCR (rw) register accessor: Clock switch control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swcr`]
module"]
pub type SWCR = crate::Reg<swcr::SWCR_SPEC>;
#[doc = "Clock switch control register"]
pub mod swcr;
#[doc = "CKDIVR (rw) register accessor: Clock divider register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ckdivr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ckdivr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ckdivr`]
module"]
pub type CKDIVR = crate::Reg<ckdivr::CKDIVR_SPEC>;
#[doc = "Clock divider register"]
pub mod ckdivr;
#[doc = "PCKENR1 (rw) register accessor: Peripheral clock gating register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pckenr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pckenr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pckenr1`]
module"]
pub type PCKENR1 = crate::Reg<pckenr1::PCKENR1_SPEC>;
#[doc = "Peripheral clock gating register 1"]
pub mod pckenr1;
#[doc = "CSSR (rw) register accessor: Clock security system register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cssr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cssr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cssr`]
module"]
pub type CSSR = crate::Reg<cssr::CSSR_SPEC>;
#[doc = "Clock security system register"]
pub mod cssr;
#[doc = "CCOR (rw) register accessor: Configurable clock control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccor::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccor::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccor`]
module"]
pub type CCOR = crate::Reg<ccor::CCOR_SPEC>;
#[doc = "Configurable clock control register"]
pub mod ccor;
#[doc = "PCKENR2 (rw) register accessor: Peripheral clock gating register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pckenr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pckenr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pckenr2`]
module"]
pub type PCKENR2 = crate::Reg<pckenr2::PCKENR2_SPEC>;
#[doc = "Peripheral clock gating register 2"]
pub mod pckenr2;
#[doc = "CANCCR (rw) register accessor: CAN clock control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`canccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`canccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@canccr`]
module"]
pub type CANCCR = crate::Reg<canccr::CANCCR_SPEC>;
#[doc = "CAN clock control register"]
pub mod canccr;
#[doc = "HSITRIMR (rw) register accessor: HSI clock calibration trimming register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hsitrimr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hsitrimr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsitrimr`]
module"]
pub type HSITRIMR = crate::Reg<hsitrimr::HSITRIMR_SPEC>;
#[doc = "HSI clock calibration trimming register"]
pub mod hsitrimr;
#[doc = "SWIMCCR (rw) register accessor: SWIM clock control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swimccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swimccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swimccr`]
module"]
pub type SWIMCCR = crate::Reg<swimccr::SWIMCCR_SPEC>;
#[doc = "SWIM clock control register"]
pub mod swimccr;
