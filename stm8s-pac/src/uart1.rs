#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    sr: SR,
    dr: DR,
    brr1: BRR1,
    brr2: BRR2,
    cr1: CR1,
    cr2: CR2,
    cr3: CR3,
    cr4: CR4,
    cr5: CR5,
    gtr: GTR,
    pscr: PSCR,
}
impl RegisterBlock {
    #[doc = "0x00 - UART1 status register"]
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    #[doc = "0x01 - UART1 data register"]
    #[inline(always)]
    pub const fn dr(&self) -> &DR {
        &self.dr
    }
    #[doc = "0x02 - UART1 baud rate register 1"]
    #[inline(always)]
    pub const fn brr1(&self) -> &BRR1 {
        &self.brr1
    }
    #[doc = "0x03 - UART1 baud rate register 2"]
    #[inline(always)]
    pub const fn brr2(&self) -> &BRR2 {
        &self.brr2
    }
    #[doc = "0x04 - UART1 control register 1"]
    #[inline(always)]
    pub const fn cr1(&self) -> &CR1 {
        &self.cr1
    }
    #[doc = "0x05 - UART1 control register 2"]
    #[inline(always)]
    pub const fn cr2(&self) -> &CR2 {
        &self.cr2
    }
    #[doc = "0x06 - UART1 control register 3"]
    #[inline(always)]
    pub const fn cr3(&self) -> &CR3 {
        &self.cr3
    }
    #[doc = "0x07 - UART1 control register 4"]
    #[inline(always)]
    pub const fn cr4(&self) -> &CR4 {
        &self.cr4
    }
    #[doc = "0x08 - UART1 control register 5"]
    #[inline(always)]
    pub const fn cr5(&self) -> &CR5 {
        &self.cr5
    }
    #[doc = "0x09 - UART1 guard time register"]
    #[inline(always)]
    pub const fn gtr(&self) -> &GTR {
        &self.gtr
    }
    #[doc = "0x0a - UART1 prescaler register"]
    #[inline(always)]
    pub const fn pscr(&self) -> &PSCR {
        &self.pscr
    }
}
#[doc = "SR (rw) register accessor: UART1 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`]
module"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "UART1 status register"]
pub mod sr;
#[doc = "DR (rw) register accessor: UART1 data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr`]
module"]
pub type DR = crate::Reg<dr::DR_SPEC>;
#[doc = "UART1 data register"]
pub mod dr;
#[doc = "BRR1 (rw) register accessor: UART1 baud rate register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`brr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`brr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@brr1`]
module"]
pub type BRR1 = crate::Reg<brr1::BRR1_SPEC>;
#[doc = "UART1 baud rate register 1"]
pub mod brr1;
#[doc = "BRR2 (rw) register accessor: UART1 baud rate register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`brr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`brr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@brr2`]
module"]
pub type BRR2 = crate::Reg<brr2::BRR2_SPEC>;
#[doc = "UART1 baud rate register 2"]
pub mod brr2;
#[doc = "CR1 (rw) register accessor: UART1 control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr1`]
module"]
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
#[doc = "UART1 control register 1"]
pub mod cr1;
#[doc = "CR2 (rw) register accessor: UART1 control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr2`]
module"]
pub type CR2 = crate::Reg<cr2::CR2_SPEC>;
#[doc = "UART1 control register 2"]
pub mod cr2;
#[doc = "CR3 (rw) register accessor: UART1 control register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr3`]
module"]
pub type CR3 = crate::Reg<cr3::CR3_SPEC>;
#[doc = "UART1 control register 3"]
pub mod cr3;
#[doc = "CR4 (rw) register accessor: UART1 control register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr4`]
module"]
pub type CR4 = crate::Reg<cr4::CR4_SPEC>;
#[doc = "UART1 control register 4"]
pub mod cr4;
#[doc = "CR5 (rw) register accessor: UART1 control register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr5`]
module"]
pub type CR5 = crate::Reg<cr5::CR5_SPEC>;
#[doc = "UART1 control register 5"]
pub mod cr5;
#[doc = "GTR (rw) register accessor: UART1 guard time register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtr`]
module"]
pub type GTR = crate::Reg<gtr::GTR_SPEC>;
#[doc = "UART1 guard time register"]
pub mod gtr;
#[doc = "PSCR (rw) register accessor: UART1 prescaler register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pscr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pscr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pscr`]
module"]
pub type PSCR = crate::Reg<pscr::PSCR_SPEC>;
#[doc = "UART1 prescaler register"]
pub mod pscr;
