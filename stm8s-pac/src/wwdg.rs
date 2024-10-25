#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    cr: CR,
    wr: WR,
}
impl RegisterBlock {
    #[doc = "0x00 - WWDG control register"]
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    #[doc = "0x01 - WWDR window register"]
    #[inline(always)]
    pub const fn wr(&self) -> &WR {
        &self.wr
    }
}
#[doc = "CR (rw) register accessor: WWDG control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "WWDG control register"]
pub mod cr;
#[doc = "WR (rw) register accessor: WWDR window register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wr`]
module"]
pub type WR = crate::Reg<wr::WR_SPEC>;
#[doc = "WWDR window register"]
pub mod wr;
