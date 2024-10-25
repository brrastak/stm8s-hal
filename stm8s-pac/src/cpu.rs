#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    a: A,
    pce: PCE,
    pch: PCH,
    pcl: PCL,
    xh: XH,
    xl: XL,
    yh: YH,
    yl: YL,
    sph: SPH,
    spl: SPL,
    ccr: CCR,
    _reserved11: [u8; 0x55],
    cfg_gcr: CFG_GCR,
}
impl RegisterBlock {
    #[doc = "0x00 - Accumulator"]
    #[inline(always)]
    pub const fn a(&self) -> &A {
        &self.a
    }
    #[doc = "0x01 - Program counter extended"]
    #[inline(always)]
    pub const fn pce(&self) -> &PCE {
        &self.pce
    }
    #[doc = "0x02 - Program counter high"]
    #[inline(always)]
    pub const fn pch(&self) -> &PCH {
        &self.pch
    }
    #[doc = "0x03 - Program counter low"]
    #[inline(always)]
    pub const fn pcl(&self) -> &PCL {
        &self.pcl
    }
    #[doc = "0x04 - X index register high"]
    #[inline(always)]
    pub const fn xh(&self) -> &XH {
        &self.xh
    }
    #[doc = "0x05 - X index register low"]
    #[inline(always)]
    pub const fn xl(&self) -> &XL {
        &self.xl
    }
    #[doc = "0x06 - Y index register high"]
    #[inline(always)]
    pub const fn yh(&self) -> &YH {
        &self.yh
    }
    #[doc = "0x07 - Y index register low"]
    #[inline(always)]
    pub const fn yl(&self) -> &YL {
        &self.yl
    }
    #[doc = "0x08 - Stack pointer high"]
    #[inline(always)]
    pub const fn sph(&self) -> &SPH {
        &self.sph
    }
    #[doc = "0x09 - Stack pointer low"]
    #[inline(always)]
    pub const fn spl(&self) -> &SPL {
        &self.spl
    }
    #[doc = "0x0a - Condition code register"]
    #[inline(always)]
    pub const fn ccr(&self) -> &CCR {
        &self.ccr
    }
    #[doc = "0x60 - Global configuration register"]
    #[inline(always)]
    pub const fn cfg_gcr(&self) -> &CFG_GCR {
        &self.cfg_gcr
    }
}
#[doc = "A (rw) register accessor: Accumulator\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`a::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`a::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@a`]
module"]
pub type A = crate::Reg<a::A_SPEC>;
#[doc = "Accumulator"]
pub mod a;
#[doc = "PCE (rw) register accessor: Program counter extended\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pce::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pce::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pce`]
module"]
pub type PCE = crate::Reg<pce::PCE_SPEC>;
#[doc = "Program counter extended"]
pub mod pce;
#[doc = "PCH (rw) register accessor: Program counter high\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pch::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pch::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pch`]
module"]
pub type PCH = crate::Reg<pch::PCH_SPEC>;
#[doc = "Program counter high"]
pub mod pch;
#[doc = "PCL (rw) register accessor: Program counter low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcl`]
module"]
pub type PCL = crate::Reg<pcl::PCL_SPEC>;
#[doc = "Program counter low"]
pub mod pcl;
#[doc = "XH (rw) register accessor: X index register high\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`xh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`xh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xh`]
module"]
pub type XH = crate::Reg<xh::XH_SPEC>;
#[doc = "X index register high"]
pub mod xh;
#[doc = "XL (rw) register accessor: X index register low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`xl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`xl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xl`]
module"]
pub type XL = crate::Reg<xl::XL_SPEC>;
#[doc = "X index register low"]
pub mod xl;
#[doc = "YH (rw) register accessor: Y index register high\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`yh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`yh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@yh`]
module"]
pub type YH = crate::Reg<yh::YH_SPEC>;
#[doc = "Y index register high"]
pub mod yh;
#[doc = "YL (rw) register accessor: Y index register low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`yl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`yl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@yl`]
module"]
pub type YL = crate::Reg<yl::YL_SPEC>;
#[doc = "Y index register low"]
pub mod yl;
#[doc = "SPH (rw) register accessor: Stack pointer high\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sph::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sph::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sph`]
module"]
pub type SPH = crate::Reg<sph::SPH_SPEC>;
#[doc = "Stack pointer high"]
pub mod sph;
#[doc = "SPL (rw) register accessor: Stack pointer low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spl`]
module"]
pub type SPL = crate::Reg<spl::SPL_SPEC>;
#[doc = "Stack pointer low"]
pub mod spl;
#[doc = "CCR (rw) register accessor: Condition code register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr`]
module"]
pub type CCR = crate::Reg<ccr::CCR_SPEC>;
#[doc = "Condition code register"]
pub mod ccr;
#[doc = "CFG_GCR (rw) register accessor: Global configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_gcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_gcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_gcr`]
module"]
pub type CFG_GCR = crate::Reg<cfg_gcr::CFG_GCR_SPEC>;
#[doc = "Global configuration register"]
pub mod cfg_gcr;
