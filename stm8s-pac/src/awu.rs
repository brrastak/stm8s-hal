#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    csr1: CSR1,
    apr: APR,
    tbr: TBR,
}
impl RegisterBlock {
    #[doc = "0x00 - AWU control/status register 1"]
    #[inline(always)]
    pub const fn csr1(&self) -> &CSR1 {
        &self.csr1
    }
    #[doc = "0x01 - AWU asynchronous prescaler buffer register"]
    #[inline(always)]
    pub const fn apr(&self) -> &APR {
        &self.apr
    }
    #[doc = "0x02 - AWU timebase selection register"]
    #[inline(always)]
    pub const fn tbr(&self) -> &TBR {
        &self.tbr
    }
}
#[doc = "CSR1 (rw) register accessor: AWU control/status register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr1`]
module"]
pub type CSR1 = crate::Reg<csr1::CSR1_SPEC>;
#[doc = "AWU control/status register 1"]
pub mod csr1;
#[doc = "APR (rw) register accessor: AWU asynchronous prescaler buffer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apr`]
module"]
pub type APR = crate::Reg<apr::APR_SPEC>;
#[doc = "AWU asynchronous prescaler buffer register"]
pub mod apr;
#[doc = "TBR (rw) register accessor: AWU timebase selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tbr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tbr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbr`]
module"]
pub type TBR = crate::Reg<tbr::TBR_SPEC>;
#[doc = "AWU timebase selection register"]
pub mod tbr;
