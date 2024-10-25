#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    cr1: CR1,
    cr2: CR2,
    freqr: FREQR,
    oarl: OARL,
    oarh: OARH,
    _reserved5: [u8; 0x01],
    dr: DR,
    sr1: SR1,
    sr2: SR2,
    sr3: SR3,
    itr: ITR,
    ccrl: CCRL,
    ccrh: CCRH,
    triser: TRISER,
    pecr: PECR,
}
impl RegisterBlock {
    #[doc = "0x00 - I2C control register 1"]
    #[inline(always)]
    pub const fn cr1(&self) -> &CR1 {
        &self.cr1
    }
    #[doc = "0x01 - I2C control register 2"]
    #[inline(always)]
    pub const fn cr2(&self) -> &CR2 {
        &self.cr2
    }
    #[doc = "0x02 - I2C frequency register"]
    #[inline(always)]
    pub const fn freqr(&self) -> &FREQR {
        &self.freqr
    }
    #[doc = "0x03 - I2C Own address register low"]
    #[inline(always)]
    pub const fn oarl(&self) -> &OARL {
        &self.oarl
    }
    #[doc = "0x04 - I2C Own address register high"]
    #[inline(always)]
    pub const fn oarh(&self) -> &OARH {
        &self.oarh
    }
    #[doc = "0x06 - I2C data register"]
    #[inline(always)]
    pub const fn dr(&self) -> &DR {
        &self.dr
    }
    #[doc = "0x07 - I2C status register 1"]
    #[inline(always)]
    pub const fn sr1(&self) -> &SR1 {
        &self.sr1
    }
    #[doc = "0x08 - I2C status register 2"]
    #[inline(always)]
    pub const fn sr2(&self) -> &SR2 {
        &self.sr2
    }
    #[doc = "0x09 - I2C status register 3"]
    #[inline(always)]
    pub const fn sr3(&self) -> &SR3 {
        &self.sr3
    }
    #[doc = "0x0a - I2C interrupt control register"]
    #[inline(always)]
    pub const fn itr(&self) -> &ITR {
        &self.itr
    }
    #[doc = "0x0b - I2C Clock control register low"]
    #[inline(always)]
    pub const fn ccrl(&self) -> &CCRL {
        &self.ccrl
    }
    #[doc = "0x0c - I2C Clock control register high"]
    #[inline(always)]
    pub const fn ccrh(&self) -> &CCRH {
        &self.ccrh
    }
    #[doc = "0x0d - I2C TRISE register"]
    #[inline(always)]
    pub const fn triser(&self) -> &TRISER {
        &self.triser
    }
    #[doc = "0x0e - I2C packet error checking register"]
    #[inline(always)]
    pub const fn pecr(&self) -> &PECR {
        &self.pecr
    }
}
#[doc = "CR1 (rw) register accessor: I2C control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr1`]
module"]
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
#[doc = "I2C control register 1"]
pub mod cr1;
#[doc = "CR2 (rw) register accessor: I2C control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr2`]
module"]
pub type CR2 = crate::Reg<cr2::CR2_SPEC>;
#[doc = "I2C control register 2"]
pub mod cr2;
#[doc = "FREQR (rw) register accessor: I2C frequency register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`freqr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`freqr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@freqr`]
module"]
pub type FREQR = crate::Reg<freqr::FREQR_SPEC>;
#[doc = "I2C frequency register"]
pub mod freqr;
#[doc = "OARL (rw) register accessor: I2C Own address register low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`oarl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`oarl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oarl`]
module"]
pub type OARL = crate::Reg<oarl::OARL_SPEC>;
#[doc = "I2C Own address register low"]
pub mod oarl;
#[doc = "OARH (rw) register accessor: I2C Own address register high\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`oarh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`oarh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oarh`]
module"]
pub type OARH = crate::Reg<oarh::OARH_SPEC>;
#[doc = "I2C Own address register high"]
pub mod oarh;
#[doc = "DR (rw) register accessor: I2C data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr`]
module"]
pub type DR = crate::Reg<dr::DR_SPEC>;
#[doc = "I2C data register"]
pub mod dr;
#[doc = "SR1 (rw) register accessor: I2C status register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr1`]
module"]
pub type SR1 = crate::Reg<sr1::SR1_SPEC>;
#[doc = "I2C status register 1"]
pub mod sr1;
#[doc = "SR2 (rw) register accessor: I2C status register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr2`]
module"]
pub type SR2 = crate::Reg<sr2::SR2_SPEC>;
#[doc = "I2C status register 2"]
pub mod sr2;
#[doc = "SR3 (rw) register accessor: I2C status register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr3`]
module"]
pub type SR3 = crate::Reg<sr3::SR3_SPEC>;
#[doc = "I2C status register 3"]
pub mod sr3;
#[doc = "ITR (rw) register accessor: I2C interrupt control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`itr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@itr`]
module"]
pub type ITR = crate::Reg<itr::ITR_SPEC>;
#[doc = "I2C interrupt control register"]
pub mod itr;
#[doc = "CCRL (rw) register accessor: I2C Clock control register low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccrl`]
module"]
pub type CCRL = crate::Reg<ccrl::CCRL_SPEC>;
#[doc = "I2C Clock control register low"]
pub mod ccrl;
#[doc = "CCRH (rw) register accessor: I2C Clock control register high\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccrh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccrh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccrh`]
module"]
pub type CCRH = crate::Reg<ccrh::CCRH_SPEC>;
#[doc = "I2C Clock control register high"]
pub mod ccrh;
#[doc = "TRISER (rw) register accessor: I2C TRISE register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`triser::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`triser::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@triser`]
module"]
pub type TRISER = crate::Reg<triser::TRISER_SPEC>;
#[doc = "I2C TRISE register"]
pub mod triser;
#[doc = "PECR (rw) register accessor: I2C packet error checking register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pecr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pecr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pecr`]
module"]
pub type PECR = crate::Reg<pecr::PECR_SPEC>;
#[doc = "I2C packet error checking register"]
pub mod pecr;
