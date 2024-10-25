#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    cr1: CR1,
    cr2: CR2,
    _reserved2: [u8; 0x11],
    rst_sr: RST_SR,
    _reserved3: [u8; 0x2ebc],
    spr1: SPR1,
    spr2: SPR2,
    spr3: SPR3,
    spr4: SPR4,
    spr5: SPR5,
    spr6: SPR6,
    spr7: SPR7,
    spr8: SPR8,
}
impl RegisterBlock {
    #[doc = "0x00 - External interrupt control register 1"]
    #[inline(always)]
    pub const fn cr1(&self) -> &CR1 {
        &self.cr1
    }
    #[doc = "0x01 - External interrupt control register 2"]
    #[inline(always)]
    pub const fn cr2(&self) -> &CR2 {
        &self.cr2
    }
    #[doc = "0x13 - Reset status register"]
    #[inline(always)]
    pub const fn rst_sr(&self) -> &RST_SR {
        &self.rst_sr
    }
    #[doc = "0x2ed0 - Interrupt software priority register 1"]
    #[inline(always)]
    pub const fn spr1(&self) -> &SPR1 {
        &self.spr1
    }
    #[doc = "0x2ed1 - Interrupt software priority register 2"]
    #[inline(always)]
    pub const fn spr2(&self) -> &SPR2 {
        &self.spr2
    }
    #[doc = "0x2ed2 - Interrupt software priority register 3"]
    #[inline(always)]
    pub const fn spr3(&self) -> &SPR3 {
        &self.spr3
    }
    #[doc = "0x2ed3 - Interrupt software priority register 4"]
    #[inline(always)]
    pub const fn spr4(&self) -> &SPR4 {
        &self.spr4
    }
    #[doc = "0x2ed4 - Interrupt software priority register 5"]
    #[inline(always)]
    pub const fn spr5(&self) -> &SPR5 {
        &self.spr5
    }
    #[doc = "0x2ed5 - Interrupt software priority register 6"]
    #[inline(always)]
    pub const fn spr6(&self) -> &SPR6 {
        &self.spr6
    }
    #[doc = "0x2ed6 - Interrupt software priority register 7"]
    #[inline(always)]
    pub const fn spr7(&self) -> &SPR7 {
        &self.spr7
    }
    #[doc = "0x2ed7 - Interrupt software priority register 8"]
    #[inline(always)]
    pub const fn spr8(&self) -> &SPR8 {
        &self.spr8
    }
}
#[doc = "CR1 (rw) register accessor: External interrupt control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr1`]
module"]
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
#[doc = "External interrupt control register 1"]
pub mod cr1;
#[doc = "CR2 (rw) register accessor: External interrupt control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr2`]
module"]
pub type CR2 = crate::Reg<cr2::CR2_SPEC>;
#[doc = "External interrupt control register 2"]
pub mod cr2;
#[doc = "RST_SR (rw) register accessor: Reset status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rst_sr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rst_sr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rst_sr`]
module"]
pub type RST_SR = crate::Reg<rst_sr::RST_SR_SPEC>;
#[doc = "Reset status register"]
pub mod rst_sr;
#[doc = "SPR1 (rw) register accessor: Interrupt software priority register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spr1`]
module"]
pub type SPR1 = crate::Reg<spr1::SPR1_SPEC>;
#[doc = "Interrupt software priority register 1"]
pub mod spr1;
#[doc = "SPR2 (rw) register accessor: Interrupt software priority register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spr2`]
module"]
pub type SPR2 = crate::Reg<spr2::SPR2_SPEC>;
#[doc = "Interrupt software priority register 2"]
pub mod spr2;
#[doc = "SPR3 (rw) register accessor: Interrupt software priority register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spr3`]
module"]
pub type SPR3 = crate::Reg<spr3::SPR3_SPEC>;
#[doc = "Interrupt software priority register 3"]
pub mod spr3;
#[doc = "SPR4 (rw) register accessor: Interrupt software priority register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spr4`]
module"]
pub type SPR4 = crate::Reg<spr4::SPR4_SPEC>;
#[doc = "Interrupt software priority register 4"]
pub mod spr4;
#[doc = "SPR5 (rw) register accessor: Interrupt software priority register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spr5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spr5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spr5`]
module"]
pub type SPR5 = crate::Reg<spr5::SPR5_SPEC>;
#[doc = "Interrupt software priority register 5"]
pub mod spr5;
#[doc = "SPR6 (rw) register accessor: Interrupt software priority register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spr6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spr6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spr6`]
module"]
pub type SPR6 = crate::Reg<spr6::SPR6_SPEC>;
#[doc = "Interrupt software priority register 6"]
pub mod spr6;
#[doc = "SPR7 (rw) register accessor: Interrupt software priority register 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spr7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spr7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spr7`]
module"]
pub type SPR7 = crate::Reg<spr7::SPR7_SPEC>;
#[doc = "Interrupt software priority register 7"]
pub mod spr7;
#[doc = "SPR8 (rw) register accessor: Interrupt software priority register 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spr8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spr8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spr8`]
module"]
pub type SPR8 = crate::Reg<spr8::SPR8_SPEC>;
#[doc = "Interrupt software priority register 8"]
pub mod spr8;
