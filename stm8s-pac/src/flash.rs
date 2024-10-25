#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    cr1: CR1,
    cr2: CR2,
    ncr2: NCR2,
    fpr: FPR,
    nfpr: NFPR,
    iapsr: IAPSR,
    _reserved6: [u8; 0x02],
    pukr: PUKR,
    _reserved7: [u8; 0x01],
    dukr: DUKR,
}
impl RegisterBlock {
    #[doc = "0x00 - Flash control register 1"]
    #[inline(always)]
    pub const fn cr1(&self) -> &CR1 {
        &self.cr1
    }
    #[doc = "0x01 - Flash control register 2"]
    #[inline(always)]
    pub const fn cr2(&self) -> &CR2 {
        &self.cr2
    }
    #[doc = "0x02 - Flash complementary control register 2"]
    #[inline(always)]
    pub const fn ncr2(&self) -> &NCR2 {
        &self.ncr2
    }
    #[doc = "0x03 - Flash protection register"]
    #[inline(always)]
    pub const fn fpr(&self) -> &FPR {
        &self.fpr
    }
    #[doc = "0x04 - Flash complementary protection register"]
    #[inline(always)]
    pub const fn nfpr(&self) -> &NFPR {
        &self.nfpr
    }
    #[doc = "0x05 - Flash in-application programming status register"]
    #[inline(always)]
    pub const fn iapsr(&self) -> &IAPSR {
        &self.iapsr
    }
    #[doc = "0x08 - Flash program memory unprotection register"]
    #[inline(always)]
    pub const fn pukr(&self) -> &PUKR {
        &self.pukr
    }
    #[doc = "0x0a - Data EEPROM unprotection register"]
    #[inline(always)]
    pub const fn dukr(&self) -> &DUKR {
        &self.dukr
    }
}
#[doc = "CR1 (rw) register accessor: Flash control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr1`]
module"]
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
#[doc = "Flash control register 1"]
pub mod cr1;
#[doc = "CR2 (rw) register accessor: Flash control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr2`]
module"]
pub type CR2 = crate::Reg<cr2::CR2_SPEC>;
#[doc = "Flash control register 2"]
pub mod cr2;
#[doc = "NCR2 (rw) register accessor: Flash complementary control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ncr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ncr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ncr2`]
module"]
pub type NCR2 = crate::Reg<ncr2::NCR2_SPEC>;
#[doc = "Flash complementary control register 2"]
pub mod ncr2;
#[doc = "FPR (rw) register accessor: Flash protection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fpr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fpr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fpr`]
module"]
pub type FPR = crate::Reg<fpr::FPR_SPEC>;
#[doc = "Flash protection register"]
pub mod fpr;
#[doc = "NFPR (rw) register accessor: Flash complementary protection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nfpr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nfpr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nfpr`]
module"]
pub type NFPR = crate::Reg<nfpr::NFPR_SPEC>;
#[doc = "Flash complementary protection register"]
pub mod nfpr;
#[doc = "IAPSR (rw) register accessor: Flash in-application programming status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iapsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iapsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iapsr`]
module"]
pub type IAPSR = crate::Reg<iapsr::IAPSR_SPEC>;
#[doc = "Flash in-application programming status register"]
pub mod iapsr;
#[doc = "PUKR (rw) register accessor: Flash program memory unprotection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pukr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pukr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pukr`]
module"]
pub type PUKR = crate::Reg<pukr::PUKR_SPEC>;
#[doc = "Flash program memory unprotection register"]
pub mod pukr;
#[doc = "DUKR (rw) register accessor: Data EEPROM unprotection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dukr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dukr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dukr`]
module"]
pub type DUKR = crate::Reg<dukr::DUKR_SPEC>;
#[doc = "Data EEPROM unprotection register"]
pub mod dukr;
