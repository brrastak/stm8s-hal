#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    opt0: OPT0,
    opt1: OPT1,
    nopt1: NOPT1,
    opt2: OPT2,
    nopt2: NOPT2,
    opt3: OPT3,
    nopt3: NOPT3,
    opt4: OPT4,
    nopt4: NOPT4,
    opt5: OPT5,
    nopt5: NOPT5,
}
impl RegisterBlock {
    #[doc = "0x00 - Read-out protection (ROP)"]
    #[inline(always)]
    pub const fn opt0(&self) -> &OPT0 {
        &self.opt0
    }
    #[doc = "0x01 - User boot code (UBC)"]
    #[inline(always)]
    pub const fn opt1(&self) -> &OPT1 {
        &self.opt1
    }
    #[doc = "0x02 - User boot code (UBC) (complementary byte)"]
    #[inline(always)]
    pub const fn nopt1(&self) -> &NOPT1 {
        &self.nopt1
    }
    #[doc = "0x03 - Alternate function remapping (AFR)"]
    #[inline(always)]
    pub const fn opt2(&self) -> &OPT2 {
        &self.opt2
    }
    #[doc = "0x04 - Alternate function remapping (AFR) (complementary byte)"]
    #[inline(always)]
    pub const fn nopt2(&self) -> &NOPT2 {
        &self.nopt2
    }
    #[doc = "0x05 - Misc. option"]
    #[inline(always)]
    pub const fn opt3(&self) -> &OPT3 {
        &self.opt3
    }
    #[doc = "0x06 - Misc. option (complementary byte)"]
    #[inline(always)]
    pub const fn nopt3(&self) -> &NOPT3 {
        &self.nopt3
    }
    #[doc = "0x07 - Clock option"]
    #[inline(always)]
    pub const fn opt4(&self) -> &OPT4 {
        &self.opt4
    }
    #[doc = "0x08 - Clock option (complementary byte)"]
    #[inline(always)]
    pub const fn nopt4(&self) -> &NOPT4 {
        &self.nopt4
    }
    #[doc = "0x09 - HSE clock startup"]
    #[inline(always)]
    pub const fn opt5(&self) -> &OPT5 {
        &self.opt5
    }
    #[doc = "0x0a - HSE clock startup (complementary byte)"]
    #[inline(always)]
    pub const fn nopt5(&self) -> &NOPT5 {
        &self.nopt5
    }
}
#[doc = "OPT0 (rw) register accessor: Read-out protection (ROP)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`opt0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`opt0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opt0`]
module"]
pub type OPT0 = crate::Reg<opt0::OPT0_SPEC>;
#[doc = "Read-out protection (ROP)"]
pub mod opt0;
#[doc = "OPT1 (rw) register accessor: User boot code (UBC)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`opt1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`opt1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opt1`]
module"]
pub type OPT1 = crate::Reg<opt1::OPT1_SPEC>;
#[doc = "User boot code (UBC)"]
pub mod opt1;
#[doc = "NOPT1 (rw) register accessor: User boot code (UBC) (complementary byte)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nopt1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nopt1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nopt1`]
module"]
pub type NOPT1 = crate::Reg<nopt1::NOPT1_SPEC>;
#[doc = "User boot code (UBC) (complementary byte)"]
pub mod nopt1;
#[doc = "OPT2 (rw) register accessor: Alternate function remapping (AFR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`opt2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`opt2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opt2`]
module"]
pub type OPT2 = crate::Reg<opt2::OPT2_SPEC>;
#[doc = "Alternate function remapping (AFR)"]
pub mod opt2;
#[doc = "NOPT2 (rw) register accessor: Alternate function remapping (AFR) (complementary byte)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nopt2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nopt2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nopt2`]
module"]
pub type NOPT2 = crate::Reg<nopt2::NOPT2_SPEC>;
#[doc = "Alternate function remapping (AFR) (complementary byte)"]
pub mod nopt2;
#[doc = "OPT3 (rw) register accessor: Misc. option\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`opt3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`opt3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opt3`]
module"]
pub type OPT3 = crate::Reg<opt3::OPT3_SPEC>;
#[doc = "Misc. option"]
pub mod opt3;
#[doc = "NOPT3 (rw) register accessor: Misc. option (complementary byte)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nopt3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nopt3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nopt3`]
module"]
pub type NOPT3 = crate::Reg<nopt3::NOPT3_SPEC>;
#[doc = "Misc. option (complementary byte)"]
pub mod nopt3;
#[doc = "OPT4 (rw) register accessor: Clock option\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`opt4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`opt4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opt4`]
module"]
pub type OPT4 = crate::Reg<opt4::OPT4_SPEC>;
#[doc = "Clock option"]
pub mod opt4;
#[doc = "NOPT4 (rw) register accessor: Clock option (complementary byte)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nopt4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nopt4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nopt4`]
module"]
pub type NOPT4 = crate::Reg<nopt4::NOPT4_SPEC>;
#[doc = "Clock option (complementary byte)"]
pub mod nopt4;
#[doc = "OPT5 (rw) register accessor: HSE clock startup\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`opt5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`opt5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opt5`]
module"]
pub type OPT5 = crate::Reg<opt5::OPT5_SPEC>;
#[doc = "HSE clock startup"]
pub mod opt5;
#[doc = "NOPT5 (rw) register accessor: HSE clock startup (complementary byte)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nopt5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nopt5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nopt5`]
module"]
pub type NOPT5 = crate::Reg<nopt5::NOPT5_SPEC>;
#[doc = "HSE clock startup (complementary byte)"]
pub mod nopt5;
