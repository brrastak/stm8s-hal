#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    kr: KR,
    pr: PR,
    rlr: RLR,
}
impl RegisterBlock {
    #[doc = "0x00 - IWDG key register"]
    #[inline(always)]
    pub const fn kr(&self) -> &KR {
        &self.kr
    }
    #[doc = "0x01 - IWDG prescaler register"]
    #[inline(always)]
    pub const fn pr(&self) -> &PR {
        &self.pr
    }
    #[doc = "0x02 - IWDG reload register"]
    #[inline(always)]
    pub const fn rlr(&self) -> &RLR {
        &self.rlr
    }
}
#[doc = "KR (rw) register accessor: IWDG key register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`kr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`kr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@kr`]
module"]
pub type KR = crate::Reg<kr::KR_SPEC>;
#[doc = "IWDG key register"]
pub mod kr;
#[doc = "PR (rw) register accessor: IWDG prescaler register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr`]
module"]
pub type PR = crate::Reg<pr::PR_SPEC>;
#[doc = "IWDG prescaler register"]
pub mod pr;
#[doc = "RLR (rw) register accessor: IWDG reload register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rlr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rlr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rlr`]
module"]
pub type RLR = crate::Reg<rlr::RLR_SPEC>;
#[doc = "IWDG reload register"]
pub mod rlr;
