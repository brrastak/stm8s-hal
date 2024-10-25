#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    db0rh: DB0RH,
    db0rl: DB0RL,
    db1rh: DB1RH,
    db1rl: DB1RL,
    db2rh: DB2RH,
    db2rl: DB2RL,
    db3rh: DB3RH,
    db3rl: DB3RL,
    db4rh: DB4RH,
    db4rl: DB4RL,
    db5rh: DB5RH,
    db5rl: DB5RL,
    db6rh: DB6RH,
    db6rl: DB6RL,
    db7rh: DB7RH,
    db7rl: DB7RL,
    db8rh: DB8RH,
    db8rl: DB8RL,
    db9rh: DB9RH,
    db9rl: DB9RL,
    _reserved20: [u8; 0x0c],
    csr: CSR,
    cr1: CR1,
    cr2: CR2,
    cr3: CR3,
    drh: DRH,
    drl: DRL,
    tdrh: TDRH,
    tdrl: TDRL,
    htrh: HTRH,
    htrl: HTRL,
    ltrh: LTRH,
    ltrl: LTRL,
    awsrh: AWSRH,
    awsrl: AWSRL,
    awcrh: AWCRH,
    awcrl: AWCRL,
}
impl RegisterBlock {
    #[doc = "0x00 - ADC data buffer registers"]
    #[inline(always)]
    pub const fn db0rh(&self) -> &DB0RH {
        &self.db0rh
    }
    #[doc = "0x01 - ADC data buffer registers"]
    #[inline(always)]
    pub const fn db0rl(&self) -> &DB0RL {
        &self.db0rl
    }
    #[doc = "0x02 - ADC data buffer registers"]
    #[inline(always)]
    pub const fn db1rh(&self) -> &DB1RH {
        &self.db1rh
    }
    #[doc = "0x03 - ADC data buffer registers"]
    #[inline(always)]
    pub const fn db1rl(&self) -> &DB1RL {
        &self.db1rl
    }
    #[doc = "0x04 - ADC data buffer registers"]
    #[inline(always)]
    pub const fn db2rh(&self) -> &DB2RH {
        &self.db2rh
    }
    #[doc = "0x05 - ADC data buffer registers"]
    #[inline(always)]
    pub const fn db2rl(&self) -> &DB2RL {
        &self.db2rl
    }
    #[doc = "0x06 - ADC data buffer registers"]
    #[inline(always)]
    pub const fn db3rh(&self) -> &DB3RH {
        &self.db3rh
    }
    #[doc = "0x07 - ADC data buffer registers"]
    #[inline(always)]
    pub const fn db3rl(&self) -> &DB3RL {
        &self.db3rl
    }
    #[doc = "0x08 - ADC data buffer registers"]
    #[inline(always)]
    pub const fn db4rh(&self) -> &DB4RH {
        &self.db4rh
    }
    #[doc = "0x09 - ADC data buffer registers"]
    #[inline(always)]
    pub const fn db4rl(&self) -> &DB4RL {
        &self.db4rl
    }
    #[doc = "0x0a - ADC data buffer registers"]
    #[inline(always)]
    pub const fn db5rh(&self) -> &DB5RH {
        &self.db5rh
    }
    #[doc = "0x0b - ADC data buffer registers"]
    #[inline(always)]
    pub const fn db5rl(&self) -> &DB5RL {
        &self.db5rl
    }
    #[doc = "0x0c - ADC data buffer registers"]
    #[inline(always)]
    pub const fn db6rh(&self) -> &DB6RH {
        &self.db6rh
    }
    #[doc = "0x0d - ADC data buffer registers"]
    #[inline(always)]
    pub const fn db6rl(&self) -> &DB6RL {
        &self.db6rl
    }
    #[doc = "0x0e - ADC data buffer registers"]
    #[inline(always)]
    pub const fn db7rh(&self) -> &DB7RH {
        &self.db7rh
    }
    #[doc = "0x0f - ADC data buffer registers"]
    #[inline(always)]
    pub const fn db7rl(&self) -> &DB7RL {
        &self.db7rl
    }
    #[doc = "0x10 - ADC data buffer registers"]
    #[inline(always)]
    pub const fn db8rh(&self) -> &DB8RH {
        &self.db8rh
    }
    #[doc = "0x11 - ADC data buffer registers"]
    #[inline(always)]
    pub const fn db8rl(&self) -> &DB8RL {
        &self.db8rl
    }
    #[doc = "0x12 - ADC data buffer registers"]
    #[inline(always)]
    pub const fn db9rh(&self) -> &DB9RH {
        &self.db9rh
    }
    #[doc = "0x13 - ADC data buffer registers"]
    #[inline(always)]
    pub const fn db9rl(&self) -> &DB9RL {
        &self.db9rl
    }
    #[doc = "0x20 - ADC control/status register"]
    #[inline(always)]
    pub const fn csr(&self) -> &CSR {
        &self.csr
    }
    #[doc = "0x21 - ADC configuration register 1"]
    #[inline(always)]
    pub const fn cr1(&self) -> &CR1 {
        &self.cr1
    }
    #[doc = "0x22 - ADC configuration register 2"]
    #[inline(always)]
    pub const fn cr2(&self) -> &CR2 {
        &self.cr2
    }
    #[doc = "0x23 - ADC configuration register 3"]
    #[inline(always)]
    pub const fn cr3(&self) -> &CR3 {
        &self.cr3
    }
    #[doc = "0x24 - ADC data register high"]
    #[inline(always)]
    pub const fn drh(&self) -> &DRH {
        &self.drh
    }
    #[doc = "0x25 - ADC data register low"]
    #[inline(always)]
    pub const fn drl(&self) -> &DRL {
        &self.drl
    }
    #[doc = "0x26 - ADC Schmitt trigger disable register high"]
    #[inline(always)]
    pub const fn tdrh(&self) -> &TDRH {
        &self.tdrh
    }
    #[doc = "0x27 - ADC Schmitt trigger disable register low"]
    #[inline(always)]
    pub const fn tdrl(&self) -> &TDRL {
        &self.tdrl
    }
    #[doc = "0x28 - ADC high threshold register high"]
    #[inline(always)]
    pub const fn htrh(&self) -> &HTRH {
        &self.htrh
    }
    #[doc = "0x29 - ADC high threshold register low"]
    #[inline(always)]
    pub const fn htrl(&self) -> &HTRL {
        &self.htrl
    }
    #[doc = "0x2a - ADC low threshold register high"]
    #[inline(always)]
    pub const fn ltrh(&self) -> &LTRH {
        &self.ltrh
    }
    #[doc = "0x2b - ADC low threshold register low"]
    #[inline(always)]
    pub const fn ltrl(&self) -> &LTRL {
        &self.ltrl
    }
    #[doc = "0x2c - ADC analog watchdog status register high"]
    #[inline(always)]
    pub const fn awsrh(&self) -> &AWSRH {
        &self.awsrh
    }
    #[doc = "0x2d - ADC analog watchdog status register low"]
    #[inline(always)]
    pub const fn awsrl(&self) -> &AWSRL {
        &self.awsrl
    }
    #[doc = "0x2e - ADC analog watchdog control register high"]
    #[inline(always)]
    pub const fn awcrh(&self) -> &AWCRH {
        &self.awcrh
    }
    #[doc = "0x2f - ADC analog watchdog control register low"]
    #[inline(always)]
    pub const fn awcrl(&self) -> &AWCRL {
        &self.awcrl
    }
}
#[doc = "DB0RH (rw) register accessor: ADC data buffer registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`db0rh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`db0rh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@db0rh`]
module"]
pub type DB0RH = crate::Reg<db0rh::DB0RH_SPEC>;
#[doc = "ADC data buffer registers"]
pub mod db0rh;
#[doc = "DB0RL (rw) register accessor: ADC data buffer registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`db0rl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`db0rl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@db0rl`]
module"]
pub type DB0RL = crate::Reg<db0rl::DB0RL_SPEC>;
#[doc = "ADC data buffer registers"]
pub mod db0rl;
#[doc = "DB1RH (rw) register accessor: ADC data buffer registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`db1rh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`db1rh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@db1rh`]
module"]
pub type DB1RH = crate::Reg<db1rh::DB1RH_SPEC>;
#[doc = "ADC data buffer registers"]
pub mod db1rh;
#[doc = "DB1RL (rw) register accessor: ADC data buffer registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`db1rl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`db1rl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@db1rl`]
module"]
pub type DB1RL = crate::Reg<db1rl::DB1RL_SPEC>;
#[doc = "ADC data buffer registers"]
pub mod db1rl;
#[doc = "DB2RH (rw) register accessor: ADC data buffer registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`db2rh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`db2rh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@db2rh`]
module"]
pub type DB2RH = crate::Reg<db2rh::DB2RH_SPEC>;
#[doc = "ADC data buffer registers"]
pub mod db2rh;
#[doc = "DB2RL (rw) register accessor: ADC data buffer registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`db2rl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`db2rl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@db2rl`]
module"]
pub type DB2RL = crate::Reg<db2rl::DB2RL_SPEC>;
#[doc = "ADC data buffer registers"]
pub mod db2rl;
#[doc = "DB3RH (rw) register accessor: ADC data buffer registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`db3rh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`db3rh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@db3rh`]
module"]
pub type DB3RH = crate::Reg<db3rh::DB3RH_SPEC>;
#[doc = "ADC data buffer registers"]
pub mod db3rh;
#[doc = "DB3RL (rw) register accessor: ADC data buffer registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`db3rl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`db3rl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@db3rl`]
module"]
pub type DB3RL = crate::Reg<db3rl::DB3RL_SPEC>;
#[doc = "ADC data buffer registers"]
pub mod db3rl;
#[doc = "DB4RH (rw) register accessor: ADC data buffer registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`db4rh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`db4rh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@db4rh`]
module"]
pub type DB4RH = crate::Reg<db4rh::DB4RH_SPEC>;
#[doc = "ADC data buffer registers"]
pub mod db4rh;
#[doc = "DB4RL (rw) register accessor: ADC data buffer registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`db4rl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`db4rl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@db4rl`]
module"]
pub type DB4RL = crate::Reg<db4rl::DB4RL_SPEC>;
#[doc = "ADC data buffer registers"]
pub mod db4rl;
#[doc = "DB5RH (rw) register accessor: ADC data buffer registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`db5rh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`db5rh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@db5rh`]
module"]
pub type DB5RH = crate::Reg<db5rh::DB5RH_SPEC>;
#[doc = "ADC data buffer registers"]
pub mod db5rh;
#[doc = "DB5RL (rw) register accessor: ADC data buffer registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`db5rl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`db5rl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@db5rl`]
module"]
pub type DB5RL = crate::Reg<db5rl::DB5RL_SPEC>;
#[doc = "ADC data buffer registers"]
pub mod db5rl;
#[doc = "DB6RH (rw) register accessor: ADC data buffer registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`db6rh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`db6rh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@db6rh`]
module"]
pub type DB6RH = crate::Reg<db6rh::DB6RH_SPEC>;
#[doc = "ADC data buffer registers"]
pub mod db6rh;
#[doc = "DB6RL (rw) register accessor: ADC data buffer registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`db6rl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`db6rl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@db6rl`]
module"]
pub type DB6RL = crate::Reg<db6rl::DB6RL_SPEC>;
#[doc = "ADC data buffer registers"]
pub mod db6rl;
#[doc = "DB7RH (rw) register accessor: ADC data buffer registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`db7rh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`db7rh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@db7rh`]
module"]
pub type DB7RH = crate::Reg<db7rh::DB7RH_SPEC>;
#[doc = "ADC data buffer registers"]
pub mod db7rh;
#[doc = "DB7RL (rw) register accessor: ADC data buffer registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`db7rl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`db7rl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@db7rl`]
module"]
pub type DB7RL = crate::Reg<db7rl::DB7RL_SPEC>;
#[doc = "ADC data buffer registers"]
pub mod db7rl;
#[doc = "DB8RH (rw) register accessor: ADC data buffer registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`db8rh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`db8rh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@db8rh`]
module"]
pub type DB8RH = crate::Reg<db8rh::DB8RH_SPEC>;
#[doc = "ADC data buffer registers"]
pub mod db8rh;
#[doc = "DB8RL (rw) register accessor: ADC data buffer registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`db8rl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`db8rl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@db8rl`]
module"]
pub type DB8RL = crate::Reg<db8rl::DB8RL_SPEC>;
#[doc = "ADC data buffer registers"]
pub mod db8rl;
#[doc = "DB9RH (rw) register accessor: ADC data buffer registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`db9rh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`db9rh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@db9rh`]
module"]
pub type DB9RH = crate::Reg<db9rh::DB9RH_SPEC>;
#[doc = "ADC data buffer registers"]
pub mod db9rh;
#[doc = "DB9RL (rw) register accessor: ADC data buffer registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`db9rl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`db9rl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@db9rl`]
module"]
pub type DB9RL = crate::Reg<db9rl::DB9RL_SPEC>;
#[doc = "ADC data buffer registers"]
pub mod db9rl;
#[doc = "CSR (rw) register accessor: ADC control/status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr`]
module"]
pub type CSR = crate::Reg<csr::CSR_SPEC>;
#[doc = "ADC control/status register"]
pub mod csr;
#[doc = "CR1 (rw) register accessor: ADC configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr1`]
module"]
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
#[doc = "ADC configuration register 1"]
pub mod cr1;
#[doc = "CR2 (rw) register accessor: ADC configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr2`]
module"]
pub type CR2 = crate::Reg<cr2::CR2_SPEC>;
#[doc = "ADC configuration register 2"]
pub mod cr2;
#[doc = "CR3 (rw) register accessor: ADC configuration register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr3`]
module"]
pub type CR3 = crate::Reg<cr3::CR3_SPEC>;
#[doc = "ADC configuration register 3"]
pub mod cr3;
#[doc = "DRH (rw) register accessor: ADC data register high\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`drh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`drh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@drh`]
module"]
pub type DRH = crate::Reg<drh::DRH_SPEC>;
#[doc = "ADC data register high"]
pub mod drh;
#[doc = "DRL (rw) register accessor: ADC data register low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`drl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`drl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@drl`]
module"]
pub type DRL = crate::Reg<drl::DRL_SPEC>;
#[doc = "ADC data register low"]
pub mod drl;
#[doc = "TDRH (rw) register accessor: ADC Schmitt trigger disable register high\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tdrh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tdrh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tdrh`]
module"]
pub type TDRH = crate::Reg<tdrh::TDRH_SPEC>;
#[doc = "ADC Schmitt trigger disable register high"]
pub mod tdrh;
#[doc = "TDRL (rw) register accessor: ADC Schmitt trigger disable register low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tdrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tdrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tdrl`]
module"]
pub type TDRL = crate::Reg<tdrl::TDRL_SPEC>;
#[doc = "ADC Schmitt trigger disable register low"]
pub mod tdrl;
#[doc = "HTRH (rw) register accessor: ADC high threshold register high\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`htrh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`htrh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@htrh`]
module"]
pub type HTRH = crate::Reg<htrh::HTRH_SPEC>;
#[doc = "ADC high threshold register high"]
pub mod htrh;
#[doc = "HTRL (rw) register accessor: ADC high threshold register low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`htrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`htrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@htrl`]
module"]
pub type HTRL = crate::Reg<htrl::HTRL_SPEC>;
#[doc = "ADC high threshold register low"]
pub mod htrl;
#[doc = "LTRH (rw) register accessor: ADC low threshold register high\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ltrh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ltrh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ltrh`]
module"]
pub type LTRH = crate::Reg<ltrh::LTRH_SPEC>;
#[doc = "ADC low threshold register high"]
pub mod ltrh;
#[doc = "LTRL (rw) register accessor: ADC low threshold register low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ltrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ltrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ltrl`]
module"]
pub type LTRL = crate::Reg<ltrl::LTRL_SPEC>;
#[doc = "ADC low threshold register low"]
pub mod ltrl;
#[doc = "AWSRH (rw) register accessor: ADC analog watchdog status register high\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`awsrh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`awsrh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@awsrh`]
module"]
pub type AWSRH = crate::Reg<awsrh::AWSRH_SPEC>;
#[doc = "ADC analog watchdog status register high"]
pub mod awsrh;
#[doc = "AWSRL (rw) register accessor: ADC analog watchdog status register low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`awsrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`awsrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@awsrl`]
module"]
pub type AWSRL = crate::Reg<awsrl::AWSRL_SPEC>;
#[doc = "ADC analog watchdog status register low"]
pub mod awsrl;
#[doc = "AWCRH (rw) register accessor: ADC analog watchdog control register high\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`awcrh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`awcrh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@awcrh`]
module"]
pub type AWCRH = crate::Reg<awcrh::AWCRH_SPEC>;
#[doc = "ADC analog watchdog control register high"]
pub mod awcrh;
#[doc = "AWCRL (rw) register accessor: ADC analog watchdog control register low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`awcrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`awcrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@awcrl`]
module"]
pub type AWCRL = crate::Reg<awcrl::AWCRL_SPEC>;
#[doc = "ADC analog watchdog control register low"]
pub mod awcrl;
