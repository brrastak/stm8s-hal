#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    bk1re: BK1RE,
    bk1rh: BK1RH,
    bk1rl: BK1RL,
    bk2re: BK2RE,
    bk2rh: BK2RH,
    bk2rl: BK2RL,
    cr1: CR1,
    cr2: CR2,
    csr1: CSR1,
    csr2: CSR2,
    enfctr: ENFCTR,
}
impl RegisterBlock {
    #[doc = "0x00 - DM breakpoint 1 register extended byte"]
    #[inline(always)]
    pub const fn bk1re(&self) -> &BK1RE {
        &self.bk1re
    }
    #[doc = "0x01 - DM breakpoint 1 register high byte"]
    #[inline(always)]
    pub const fn bk1rh(&self) -> &BK1RH {
        &self.bk1rh
    }
    #[doc = "0x02 - DM breakpoint 1 register low byte"]
    #[inline(always)]
    pub const fn bk1rl(&self) -> &BK1RL {
        &self.bk1rl
    }
    #[doc = "0x03 - DM breakpoint 2 register extended byte"]
    #[inline(always)]
    pub const fn bk2re(&self) -> &BK2RE {
        &self.bk2re
    }
    #[doc = "0x04 - DM breakpoint 2 register high byte"]
    #[inline(always)]
    pub const fn bk2rh(&self) -> &BK2RH {
        &self.bk2rh
    }
    #[doc = "0x05 - DM breakpoint 2 register low byte"]
    #[inline(always)]
    pub const fn bk2rl(&self) -> &BK2RL {
        &self.bk2rl
    }
    #[doc = "0x06 - DM debug module control register 1"]
    #[inline(always)]
    pub const fn cr1(&self) -> &CR1 {
        &self.cr1
    }
    #[doc = "0x07 - DM debug module control register 2"]
    #[inline(always)]
    pub const fn cr2(&self) -> &CR2 {
        &self.cr2
    }
    #[doc = "0x08 - DM debug module control/status register 1"]
    #[inline(always)]
    pub const fn csr1(&self) -> &CSR1 {
        &self.csr1
    }
    #[doc = "0x09 - DM debug module control/status register 2"]
    #[inline(always)]
    pub const fn csr2(&self) -> &CSR2 {
        &self.csr2
    }
    #[doc = "0x0a - DM enable function register"]
    #[inline(always)]
    pub const fn enfctr(&self) -> &ENFCTR {
        &self.enfctr
    }
}
#[doc = "BK1RE (rw) register accessor: DM breakpoint 1 register extended byte\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bk1re::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bk1re::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bk1re`]
module"]
pub type BK1RE = crate::Reg<bk1re::BK1RE_SPEC>;
#[doc = "DM breakpoint 1 register extended byte"]
pub mod bk1re;
#[doc = "BK1RH (rw) register accessor: DM breakpoint 1 register high byte\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bk1rh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bk1rh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bk1rh`]
module"]
pub type BK1RH = crate::Reg<bk1rh::BK1RH_SPEC>;
#[doc = "DM breakpoint 1 register high byte"]
pub mod bk1rh;
#[doc = "BK1RL (rw) register accessor: DM breakpoint 1 register low byte\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bk1rl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bk1rl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bk1rl`]
module"]
pub type BK1RL = crate::Reg<bk1rl::BK1RL_SPEC>;
#[doc = "DM breakpoint 1 register low byte"]
pub mod bk1rl;
#[doc = "BK2RE (rw) register accessor: DM breakpoint 2 register extended byte\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bk2re::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bk2re::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bk2re`]
module"]
pub type BK2RE = crate::Reg<bk2re::BK2RE_SPEC>;
#[doc = "DM breakpoint 2 register extended byte"]
pub mod bk2re;
#[doc = "BK2RH (rw) register accessor: DM breakpoint 2 register high byte\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bk2rh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bk2rh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bk2rh`]
module"]
pub type BK2RH = crate::Reg<bk2rh::BK2RH_SPEC>;
#[doc = "DM breakpoint 2 register high byte"]
pub mod bk2rh;
#[doc = "BK2RL (rw) register accessor: DM breakpoint 2 register low byte\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bk2rl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bk2rl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bk2rl`]
module"]
pub type BK2RL = crate::Reg<bk2rl::BK2RL_SPEC>;
#[doc = "DM breakpoint 2 register low byte"]
pub mod bk2rl;
#[doc = "CR1 (rw) register accessor: DM debug module control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr1`]
module"]
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
#[doc = "DM debug module control register 1"]
pub mod cr1;
#[doc = "CR2 (rw) register accessor: DM debug module control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr2`]
module"]
pub type CR2 = crate::Reg<cr2::CR2_SPEC>;
#[doc = "DM debug module control register 2"]
pub mod cr2;
#[doc = "CSR1 (rw) register accessor: DM debug module control/status register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr1`]
module"]
pub type CSR1 = crate::Reg<csr1::CSR1_SPEC>;
#[doc = "DM debug module control/status register 1"]
pub mod csr1;
#[doc = "CSR2 (rw) register accessor: DM debug module control/status register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr2`]
module"]
pub type CSR2 = crate::Reg<csr2::CSR2_SPEC>;
#[doc = "DM debug module control/status register 2"]
pub mod csr2;
#[doc = "ENFCTR (rw) register accessor: DM enable function register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enfctr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enfctr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enfctr`]
module"]
pub type ENFCTR = crate::Reg<enfctr::ENFCTR_SPEC>;
#[doc = "DM enable function register"]
pub mod enfctr;
