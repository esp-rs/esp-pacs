#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    blk0_rdata0: BLK0_RDATA0,
    blk0_rdata1: BLK0_RDATA1,
    blk0_rdata2: BLK0_RDATA2,
    blk0_rdata3: BLK0_RDATA3,
    blk0_rdata4: BLK0_RDATA4,
    blk0_rdata5: BLK0_RDATA5,
    blk0_rdata6: BLK0_RDATA6,
    blk0_wdata0: BLK0_WDATA0,
    blk0_wdata1: BLK0_WDATA1,
    blk0_wdata2: BLK0_WDATA2,
    blk0_wdata3: BLK0_WDATA3,
    blk0_wdata4: BLK0_WDATA4,
    blk0_wdata5: BLK0_WDATA5,
    blk0_wdata6: BLK0_WDATA6,
    blk1_rdata0: BLK1_RDATA0,
    blk1_rdata1: BLK1_RDATA1,
    blk1_rdata2: BLK1_RDATA2,
    blk1_rdata3: BLK1_RDATA3,
    blk1_rdata4: BLK1_RDATA4,
    blk1_rdata5: BLK1_RDATA5,
    blk1_rdata6: BLK1_RDATA6,
    blk1_rdata7: BLK1_RDATA7,
    blk2_rdata0: BLK2_RDATA0,
    blk2_rdata1: BLK2_RDATA1,
    blk2_rdata2: BLK2_RDATA2,
    blk2_rdata3: BLK2_RDATA3,
    blk2_rdata4: BLK2_RDATA4,
    blk2_rdata5: BLK2_RDATA5,
    blk2_rdata6: BLK2_RDATA6,
    blk2_rdata7: BLK2_RDATA7,
    blk3_rdata0: BLK3_RDATA0,
    blk3_rdata1: BLK3_RDATA1,
    blk3_rdata2: BLK3_RDATA2,
    blk3_rdata3: BLK3_RDATA3,
    blk3_rdata4: BLK3_RDATA4,
    blk3_rdata5: BLK3_RDATA5,
    blk3_rdata6: BLK3_RDATA6,
    blk3_rdata7: BLK3_RDATA7,
    blk1_wdata0: BLK1_WDATA0,
    blk1_wdata1: BLK1_WDATA1,
    blk1_wdata2: BLK1_WDATA2,
    blk1_wdata3: BLK1_WDATA3,
    blk1_wdata4: BLK1_WDATA4,
    blk1_wdata5: BLK1_WDATA5,
    blk1_wdata6: BLK1_WDATA6,
    blk1_wdata7: BLK1_WDATA7,
    blk2_wdata0: BLK2_WDATA0,
    blk2_wdata1: BLK2_WDATA1,
    blk2_wdata2: BLK2_WDATA2,
    blk2_wdata3: BLK2_WDATA3,
    blk2_wdata4: BLK2_WDATA4,
    blk2_wdata5: BLK2_WDATA5,
    blk2_wdata6: BLK2_WDATA6,
    blk2_wdata7: BLK2_WDATA7,
    blk3_wdata0: BLK3_WDATA0,
    blk3_wdata1: BLK3_WDATA1,
    blk3_wdata2: BLK3_WDATA2,
    blk3_wdata3: BLK3_WDATA3,
    blk3_wdata4: BLK3_WDATA4,
    blk3_wdata5: BLK3_WDATA5,
    blk3_wdata6: BLK3_WDATA6,
    blk3_wdata7: BLK3_WDATA7,
    clk: CLK,
    conf: CONF,
    status: STATUS,
    cmd: CMD,
    int_raw: INT_RAW,
    int_st: INT_ST,
    int_ena: INT_ENA,
    int_clr: INT_CLR,
    dac_conf: DAC_CONF,
    dec_status: DEC_STATUS,
    _reserved72: [u8; 0xdc],
    date: DATE,
}
impl RegisterBlock {
    #[doc = "0x00 - "]
    #[inline(always)]
    pub const fn blk0_rdata0(&self) -> &BLK0_RDATA0 {
        &self.blk0_rdata0
    }
    #[doc = "0x04 - "]
    #[inline(always)]
    pub const fn blk0_rdata1(&self) -> &BLK0_RDATA1 {
        &self.blk0_rdata1
    }
    #[doc = "0x08 - "]
    #[inline(always)]
    pub const fn blk0_rdata2(&self) -> &BLK0_RDATA2 {
        &self.blk0_rdata2
    }
    #[doc = "0x0c - "]
    #[inline(always)]
    pub const fn blk0_rdata3(&self) -> &BLK0_RDATA3 {
        &self.blk0_rdata3
    }
    #[doc = "0x10 - "]
    #[inline(always)]
    pub const fn blk0_rdata4(&self) -> &BLK0_RDATA4 {
        &self.blk0_rdata4
    }
    #[doc = "0x14 - "]
    #[inline(always)]
    pub const fn blk0_rdata5(&self) -> &BLK0_RDATA5 {
        &self.blk0_rdata5
    }
    #[doc = "0x18 - "]
    #[inline(always)]
    pub const fn blk0_rdata6(&self) -> &BLK0_RDATA6 {
        &self.blk0_rdata6
    }
    #[doc = "0x1c - "]
    #[inline(always)]
    pub const fn blk0_wdata0(&self) -> &BLK0_WDATA0 {
        &self.blk0_wdata0
    }
    #[doc = "0x20 - "]
    #[inline(always)]
    pub const fn blk0_wdata1(&self) -> &BLK0_WDATA1 {
        &self.blk0_wdata1
    }
    #[doc = "0x24 - "]
    #[inline(always)]
    pub const fn blk0_wdata2(&self) -> &BLK0_WDATA2 {
        &self.blk0_wdata2
    }
    #[doc = "0x28 - "]
    #[inline(always)]
    pub const fn blk0_wdata3(&self) -> &BLK0_WDATA3 {
        &self.blk0_wdata3
    }
    #[doc = "0x2c - "]
    #[inline(always)]
    pub const fn blk0_wdata4(&self) -> &BLK0_WDATA4 {
        &self.blk0_wdata4
    }
    #[doc = "0x30 - "]
    #[inline(always)]
    pub const fn blk0_wdata5(&self) -> &BLK0_WDATA5 {
        &self.blk0_wdata5
    }
    #[doc = "0x34 - "]
    #[inline(always)]
    pub const fn blk0_wdata6(&self) -> &BLK0_WDATA6 {
        &self.blk0_wdata6
    }
    #[doc = "0x38 - "]
    #[inline(always)]
    pub const fn blk1_rdata0(&self) -> &BLK1_RDATA0 {
        &self.blk1_rdata0
    }
    #[doc = "0x3c - "]
    #[inline(always)]
    pub const fn blk1_rdata1(&self) -> &BLK1_RDATA1 {
        &self.blk1_rdata1
    }
    #[doc = "0x40 - "]
    #[inline(always)]
    pub const fn blk1_rdata2(&self) -> &BLK1_RDATA2 {
        &self.blk1_rdata2
    }
    #[doc = "0x44 - "]
    #[inline(always)]
    pub const fn blk1_rdata3(&self) -> &BLK1_RDATA3 {
        &self.blk1_rdata3
    }
    #[doc = "0x48 - "]
    #[inline(always)]
    pub const fn blk1_rdata4(&self) -> &BLK1_RDATA4 {
        &self.blk1_rdata4
    }
    #[doc = "0x4c - "]
    #[inline(always)]
    pub const fn blk1_rdata5(&self) -> &BLK1_RDATA5 {
        &self.blk1_rdata5
    }
    #[doc = "0x50 - "]
    #[inline(always)]
    pub const fn blk1_rdata6(&self) -> &BLK1_RDATA6 {
        &self.blk1_rdata6
    }
    #[doc = "0x54 - "]
    #[inline(always)]
    pub const fn blk1_rdata7(&self) -> &BLK1_RDATA7 {
        &self.blk1_rdata7
    }
    #[doc = "0x58 - "]
    #[inline(always)]
    pub const fn blk2_rdata0(&self) -> &BLK2_RDATA0 {
        &self.blk2_rdata0
    }
    #[doc = "0x5c - "]
    #[inline(always)]
    pub const fn blk2_rdata1(&self) -> &BLK2_RDATA1 {
        &self.blk2_rdata1
    }
    #[doc = "0x60 - "]
    #[inline(always)]
    pub const fn blk2_rdata2(&self) -> &BLK2_RDATA2 {
        &self.blk2_rdata2
    }
    #[doc = "0x64 - "]
    #[inline(always)]
    pub const fn blk2_rdata3(&self) -> &BLK2_RDATA3 {
        &self.blk2_rdata3
    }
    #[doc = "0x68 - "]
    #[inline(always)]
    pub const fn blk2_rdata4(&self) -> &BLK2_RDATA4 {
        &self.blk2_rdata4
    }
    #[doc = "0x6c - "]
    #[inline(always)]
    pub const fn blk2_rdata5(&self) -> &BLK2_RDATA5 {
        &self.blk2_rdata5
    }
    #[doc = "0x70 - "]
    #[inline(always)]
    pub const fn blk2_rdata6(&self) -> &BLK2_RDATA6 {
        &self.blk2_rdata6
    }
    #[doc = "0x74 - "]
    #[inline(always)]
    pub const fn blk2_rdata7(&self) -> &BLK2_RDATA7 {
        &self.blk2_rdata7
    }
    #[doc = "0x78 - "]
    #[inline(always)]
    pub const fn blk3_rdata0(&self) -> &BLK3_RDATA0 {
        &self.blk3_rdata0
    }
    #[doc = "0x7c - "]
    #[inline(always)]
    pub const fn blk3_rdata1(&self) -> &BLK3_RDATA1 {
        &self.blk3_rdata1
    }
    #[doc = "0x80 - "]
    #[inline(always)]
    pub const fn blk3_rdata2(&self) -> &BLK3_RDATA2 {
        &self.blk3_rdata2
    }
    #[doc = "0x84 - "]
    #[inline(always)]
    pub const fn blk3_rdata3(&self) -> &BLK3_RDATA3 {
        &self.blk3_rdata3
    }
    #[doc = "0x88 - "]
    #[inline(always)]
    pub const fn blk3_rdata4(&self) -> &BLK3_RDATA4 {
        &self.blk3_rdata4
    }
    #[doc = "0x8c - "]
    #[inline(always)]
    pub const fn blk3_rdata5(&self) -> &BLK3_RDATA5 {
        &self.blk3_rdata5
    }
    #[doc = "0x90 - "]
    #[inline(always)]
    pub const fn blk3_rdata6(&self) -> &BLK3_RDATA6 {
        &self.blk3_rdata6
    }
    #[doc = "0x94 - "]
    #[inline(always)]
    pub const fn blk3_rdata7(&self) -> &BLK3_RDATA7 {
        &self.blk3_rdata7
    }
    #[doc = "0x98 - "]
    #[inline(always)]
    pub const fn blk1_wdata0(&self) -> &BLK1_WDATA0 {
        &self.blk1_wdata0
    }
    #[doc = "0x9c - "]
    #[inline(always)]
    pub const fn blk1_wdata1(&self) -> &BLK1_WDATA1 {
        &self.blk1_wdata1
    }
    #[doc = "0xa0 - "]
    #[inline(always)]
    pub const fn blk1_wdata2(&self) -> &BLK1_WDATA2 {
        &self.blk1_wdata2
    }
    #[doc = "0xa4 - "]
    #[inline(always)]
    pub const fn blk1_wdata3(&self) -> &BLK1_WDATA3 {
        &self.blk1_wdata3
    }
    #[doc = "0xa8 - "]
    #[inline(always)]
    pub const fn blk1_wdata4(&self) -> &BLK1_WDATA4 {
        &self.blk1_wdata4
    }
    #[doc = "0xac - "]
    #[inline(always)]
    pub const fn blk1_wdata5(&self) -> &BLK1_WDATA5 {
        &self.blk1_wdata5
    }
    #[doc = "0xb0 - "]
    #[inline(always)]
    pub const fn blk1_wdata6(&self) -> &BLK1_WDATA6 {
        &self.blk1_wdata6
    }
    #[doc = "0xb4 - "]
    #[inline(always)]
    pub const fn blk1_wdata7(&self) -> &BLK1_WDATA7 {
        &self.blk1_wdata7
    }
    #[doc = "0xb8 - "]
    #[inline(always)]
    pub const fn blk2_wdata0(&self) -> &BLK2_WDATA0 {
        &self.blk2_wdata0
    }
    #[doc = "0xbc - "]
    #[inline(always)]
    pub const fn blk2_wdata1(&self) -> &BLK2_WDATA1 {
        &self.blk2_wdata1
    }
    #[doc = "0xc0 - "]
    #[inline(always)]
    pub const fn blk2_wdata2(&self) -> &BLK2_WDATA2 {
        &self.blk2_wdata2
    }
    #[doc = "0xc4 - "]
    #[inline(always)]
    pub const fn blk2_wdata3(&self) -> &BLK2_WDATA3 {
        &self.blk2_wdata3
    }
    #[doc = "0xc8 - "]
    #[inline(always)]
    pub const fn blk2_wdata4(&self) -> &BLK2_WDATA4 {
        &self.blk2_wdata4
    }
    #[doc = "0xcc - "]
    #[inline(always)]
    pub const fn blk2_wdata5(&self) -> &BLK2_WDATA5 {
        &self.blk2_wdata5
    }
    #[doc = "0xd0 - "]
    #[inline(always)]
    pub const fn blk2_wdata6(&self) -> &BLK2_WDATA6 {
        &self.blk2_wdata6
    }
    #[doc = "0xd4 - "]
    #[inline(always)]
    pub const fn blk2_wdata7(&self) -> &BLK2_WDATA7 {
        &self.blk2_wdata7
    }
    #[doc = "0xd8 - "]
    #[inline(always)]
    pub const fn blk3_wdata0(&self) -> &BLK3_WDATA0 {
        &self.blk3_wdata0
    }
    #[doc = "0xdc - "]
    #[inline(always)]
    pub const fn blk3_wdata1(&self) -> &BLK3_WDATA1 {
        &self.blk3_wdata1
    }
    #[doc = "0xe0 - "]
    #[inline(always)]
    pub const fn blk3_wdata2(&self) -> &BLK3_WDATA2 {
        &self.blk3_wdata2
    }
    #[doc = "0xe4 - "]
    #[inline(always)]
    pub const fn blk3_wdata3(&self) -> &BLK3_WDATA3 {
        &self.blk3_wdata3
    }
    #[doc = "0xe8 - "]
    #[inline(always)]
    pub const fn blk3_wdata4(&self) -> &BLK3_WDATA4 {
        &self.blk3_wdata4
    }
    #[doc = "0xec - "]
    #[inline(always)]
    pub const fn blk3_wdata5(&self) -> &BLK3_WDATA5 {
        &self.blk3_wdata5
    }
    #[doc = "0xf0 - "]
    #[inline(always)]
    pub const fn blk3_wdata6(&self) -> &BLK3_WDATA6 {
        &self.blk3_wdata6
    }
    #[doc = "0xf4 - "]
    #[inline(always)]
    pub const fn blk3_wdata7(&self) -> &BLK3_WDATA7 {
        &self.blk3_wdata7
    }
    #[doc = "0xf8 - "]
    #[inline(always)]
    pub const fn clk(&self) -> &CLK {
        &self.clk
    }
    #[doc = "0xfc - "]
    #[inline(always)]
    pub const fn conf(&self) -> &CONF {
        &self.conf
    }
    #[doc = "0x100 - "]
    #[inline(always)]
    pub const fn status(&self) -> &STATUS {
        &self.status
    }
    #[doc = "0x104 - "]
    #[inline(always)]
    pub const fn cmd(&self) -> &CMD {
        &self.cmd
    }
    #[doc = "0x108 - "]
    #[inline(always)]
    pub const fn int_raw(&self) -> &INT_RAW {
        &self.int_raw
    }
    #[doc = "0x10c - "]
    #[inline(always)]
    pub const fn int_st(&self) -> &INT_ST {
        &self.int_st
    }
    #[doc = "0x110 - "]
    #[inline(always)]
    pub const fn int_ena(&self) -> &INT_ENA {
        &self.int_ena
    }
    #[doc = "0x114 - "]
    #[inline(always)]
    pub const fn int_clr(&self) -> &INT_CLR {
        &self.int_clr
    }
    #[doc = "0x118 - "]
    #[inline(always)]
    pub const fn dac_conf(&self) -> &DAC_CONF {
        &self.dac_conf
    }
    #[doc = "0x11c - "]
    #[inline(always)]
    pub const fn dec_status(&self) -> &DEC_STATUS {
        &self.dec_status
    }
    #[doc = "0x1fc - "]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
#[doc = "BLK0_RDATA0 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk0_rdata0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blk0_rdata0`] module"]
pub type BLK0_RDATA0 = crate::Reg<blk0_rdata0::BLK0_RDATA0_SPEC>;
#[doc = ""]
pub mod blk0_rdata0;
#[doc = "BLK0_RDATA1 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk0_rdata1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blk0_rdata1`] module"]
pub type BLK0_RDATA1 = crate::Reg<blk0_rdata1::BLK0_RDATA1_SPEC>;
#[doc = ""]
pub mod blk0_rdata1;
#[doc = "BLK0_RDATA2 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk0_rdata2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blk0_rdata2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blk0_rdata2`] module"]
pub type BLK0_RDATA2 = crate::Reg<blk0_rdata2::BLK0_RDATA2_SPEC>;
#[doc = ""]
pub mod blk0_rdata2;
#[doc = "BLK0_RDATA3 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk0_rdata3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blk0_rdata3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blk0_rdata3`] module"]
pub type BLK0_RDATA3 = crate::Reg<blk0_rdata3::BLK0_RDATA3_SPEC>;
#[doc = ""]
pub mod blk0_rdata3;
#[doc = "BLK0_RDATA4 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk0_rdata4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blk0_rdata4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blk0_rdata4`] module"]
pub type BLK0_RDATA4 = crate::Reg<blk0_rdata4::BLK0_RDATA4_SPEC>;
#[doc = ""]
pub mod blk0_rdata4;
#[doc = "BLK0_RDATA5 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk0_rdata5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blk0_rdata5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blk0_rdata5`] module"]
pub type BLK0_RDATA5 = crate::Reg<blk0_rdata5::BLK0_RDATA5_SPEC>;
#[doc = ""]
pub mod blk0_rdata5;
#[doc = "BLK0_RDATA6 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk0_rdata6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blk0_rdata6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blk0_rdata6`] module"]
pub type BLK0_RDATA6 = crate::Reg<blk0_rdata6::BLK0_RDATA6_SPEC>;
#[doc = ""]
pub mod blk0_rdata6;
#[doc = "BLK0_WDATA0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk0_wdata0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blk0_wdata0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blk0_wdata0`] module"]
pub type BLK0_WDATA0 = crate::Reg<blk0_wdata0::BLK0_WDATA0_SPEC>;
#[doc = ""]
pub mod blk0_wdata0;
#[doc = "BLK0_WDATA1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk0_wdata1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blk0_wdata1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blk0_wdata1`] module"]
pub type BLK0_WDATA1 = crate::Reg<blk0_wdata1::BLK0_WDATA1_SPEC>;
#[doc = ""]
pub mod blk0_wdata1;
#[doc = "BLK0_WDATA2 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk0_wdata2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blk0_wdata2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blk0_wdata2`] module"]
pub type BLK0_WDATA2 = crate::Reg<blk0_wdata2::BLK0_WDATA2_SPEC>;
#[doc = ""]
pub mod blk0_wdata2;
#[doc = "BLK0_WDATA3 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk0_wdata3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blk0_wdata3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blk0_wdata3`] module"]
pub type BLK0_WDATA3 = crate::Reg<blk0_wdata3::BLK0_WDATA3_SPEC>;
#[doc = ""]
pub mod blk0_wdata3;
#[doc = "BLK0_WDATA4 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk0_wdata4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blk0_wdata4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blk0_wdata4`] module"]
pub type BLK0_WDATA4 = crate::Reg<blk0_wdata4::BLK0_WDATA4_SPEC>;
#[doc = ""]
pub mod blk0_wdata4;
#[doc = "BLK0_WDATA5 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk0_wdata5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blk0_wdata5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blk0_wdata5`] module"]
pub type BLK0_WDATA5 = crate::Reg<blk0_wdata5::BLK0_WDATA5_SPEC>;
#[doc = ""]
pub mod blk0_wdata5;
#[doc = "BLK0_WDATA6 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk0_wdata6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blk0_wdata6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blk0_wdata6`] module"]
pub type BLK0_WDATA6 = crate::Reg<blk0_wdata6::BLK0_WDATA6_SPEC>;
#[doc = ""]
pub mod blk0_wdata6;
#[doc = "BLK1_RDATA0 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk1_rdata0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blk1_rdata0`] module"]
pub type BLK1_RDATA0 = crate::Reg<blk1_rdata0::BLK1_RDATA0_SPEC>;
#[doc = ""]
pub mod blk1_rdata0;
#[doc = "BLK1_RDATA1 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk1_rdata1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blk1_rdata1`] module"]
pub type BLK1_RDATA1 = crate::Reg<blk1_rdata1::BLK1_RDATA1_SPEC>;
#[doc = ""]
pub mod blk1_rdata1;
#[doc = "BLK1_RDATA2 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk1_rdata2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blk1_rdata2`] module"]
pub type BLK1_RDATA2 = crate::Reg<blk1_rdata2::BLK1_RDATA2_SPEC>;
#[doc = ""]
pub mod blk1_rdata2;
#[doc = "BLK1_RDATA3 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk1_rdata3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blk1_rdata3`] module"]
pub type BLK1_RDATA3 = crate::Reg<blk1_rdata3::BLK1_RDATA3_SPEC>;
#[doc = ""]
pub mod blk1_rdata3;
#[doc = "BLK1_RDATA4 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk1_rdata4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blk1_rdata4`] module"]
pub type BLK1_RDATA4 = crate::Reg<blk1_rdata4::BLK1_RDATA4_SPEC>;
#[doc = ""]
pub mod blk1_rdata4;
#[doc = "BLK1_RDATA5 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk1_rdata5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blk1_rdata5`] module"]
pub type BLK1_RDATA5 = crate::Reg<blk1_rdata5::BLK1_RDATA5_SPEC>;
#[doc = ""]
pub mod blk1_rdata5;
#[doc = "BLK1_RDATA6 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk1_rdata6::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blk1_rdata6`] module"]
pub type BLK1_RDATA6 = crate::Reg<blk1_rdata6::BLK1_RDATA6_SPEC>;
#[doc = ""]
pub mod blk1_rdata6;
#[doc = "BLK1_RDATA7 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk1_rdata7::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blk1_rdata7`] module"]
pub type BLK1_RDATA7 = crate::Reg<blk1_rdata7::BLK1_RDATA7_SPEC>;
#[doc = ""]
pub mod blk1_rdata7;
#[doc = "BLK2_RDATA0 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk2_rdata0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blk2_rdata0`] module"]
pub type BLK2_RDATA0 = crate::Reg<blk2_rdata0::BLK2_RDATA0_SPEC>;
#[doc = ""]
pub mod blk2_rdata0;
#[doc = "BLK2_RDATA1 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk2_rdata1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blk2_rdata1`] module"]
pub type BLK2_RDATA1 = crate::Reg<blk2_rdata1::BLK2_RDATA1_SPEC>;
#[doc = ""]
pub mod blk2_rdata1;
#[doc = "BLK2_RDATA2 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk2_rdata2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blk2_rdata2`] module"]
pub type BLK2_RDATA2 = crate::Reg<blk2_rdata2::BLK2_RDATA2_SPEC>;
#[doc = ""]
pub mod blk2_rdata2;
#[doc = "BLK2_RDATA3 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk2_rdata3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blk2_rdata3`] module"]
pub type BLK2_RDATA3 = crate::Reg<blk2_rdata3::BLK2_RDATA3_SPEC>;
#[doc = ""]
pub mod blk2_rdata3;
#[doc = "BLK2_RDATA4 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk2_rdata4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blk2_rdata4`] module"]
pub type BLK2_RDATA4 = crate::Reg<blk2_rdata4::BLK2_RDATA4_SPEC>;
#[doc = ""]
pub mod blk2_rdata4;
#[doc = "BLK2_RDATA5 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk2_rdata5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blk2_rdata5`] module"]
pub type BLK2_RDATA5 = crate::Reg<blk2_rdata5::BLK2_RDATA5_SPEC>;
#[doc = ""]
pub mod blk2_rdata5;
#[doc = "BLK2_RDATA6 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk2_rdata6::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blk2_rdata6`] module"]
pub type BLK2_RDATA6 = crate::Reg<blk2_rdata6::BLK2_RDATA6_SPEC>;
#[doc = ""]
pub mod blk2_rdata6;
#[doc = "BLK2_RDATA7 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk2_rdata7::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blk2_rdata7`] module"]
pub type BLK2_RDATA7 = crate::Reg<blk2_rdata7::BLK2_RDATA7_SPEC>;
#[doc = ""]
pub mod blk2_rdata7;
#[doc = "BLK3_RDATA0 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk3_rdata0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blk3_rdata0`] module"]
pub type BLK3_RDATA0 = crate::Reg<blk3_rdata0::BLK3_RDATA0_SPEC>;
#[doc = ""]
pub mod blk3_rdata0;
#[doc = "BLK3_RDATA1 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk3_rdata1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blk3_rdata1`] module"]
pub type BLK3_RDATA1 = crate::Reg<blk3_rdata1::BLK3_RDATA1_SPEC>;
#[doc = ""]
pub mod blk3_rdata1;
#[doc = "BLK3_RDATA2 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk3_rdata2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blk3_rdata2`] module"]
pub type BLK3_RDATA2 = crate::Reg<blk3_rdata2::BLK3_RDATA2_SPEC>;
#[doc = ""]
pub mod blk3_rdata2;
#[doc = "BLK3_RDATA3 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk3_rdata3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blk3_rdata3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blk3_rdata3`] module"]
pub type BLK3_RDATA3 = crate::Reg<blk3_rdata3::BLK3_RDATA3_SPEC>;
#[doc = ""]
pub mod blk3_rdata3;
#[doc = "BLK3_RDATA4 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk3_rdata4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blk3_rdata4`] module"]
pub type BLK3_RDATA4 = crate::Reg<blk3_rdata4::BLK3_RDATA4_SPEC>;
#[doc = ""]
pub mod blk3_rdata4;
#[doc = "BLK3_RDATA5 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk3_rdata5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blk3_rdata5`] module"]
pub type BLK3_RDATA5 = crate::Reg<blk3_rdata5::BLK3_RDATA5_SPEC>;
#[doc = ""]
pub mod blk3_rdata5;
#[doc = "BLK3_RDATA6 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk3_rdata6::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blk3_rdata6`] module"]
pub type BLK3_RDATA6 = crate::Reg<blk3_rdata6::BLK3_RDATA6_SPEC>;
#[doc = ""]
pub mod blk3_rdata6;
#[doc = "BLK3_RDATA7 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk3_rdata7::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blk3_rdata7`] module"]
pub type BLK3_RDATA7 = crate::Reg<blk3_rdata7::BLK3_RDATA7_SPEC>;
#[doc = ""]
pub mod blk3_rdata7;
#[doc = "BLK1_WDATA0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk1_wdata0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blk1_wdata0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blk1_wdata0`] module"]
pub type BLK1_WDATA0 = crate::Reg<blk1_wdata0::BLK1_WDATA0_SPEC>;
#[doc = ""]
pub mod blk1_wdata0;
#[doc = "BLK1_WDATA1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk1_wdata1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blk1_wdata1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blk1_wdata1`] module"]
pub type BLK1_WDATA1 = crate::Reg<blk1_wdata1::BLK1_WDATA1_SPEC>;
#[doc = ""]
pub mod blk1_wdata1;
#[doc = "BLK1_WDATA2 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk1_wdata2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blk1_wdata2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blk1_wdata2`] module"]
pub type BLK1_WDATA2 = crate::Reg<blk1_wdata2::BLK1_WDATA2_SPEC>;
#[doc = ""]
pub mod blk1_wdata2;
#[doc = "BLK1_WDATA3 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk1_wdata3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blk1_wdata3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blk1_wdata3`] module"]
pub type BLK1_WDATA3 = crate::Reg<blk1_wdata3::BLK1_WDATA3_SPEC>;
#[doc = ""]
pub mod blk1_wdata3;
#[doc = "BLK1_WDATA4 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk1_wdata4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blk1_wdata4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blk1_wdata4`] module"]
pub type BLK1_WDATA4 = crate::Reg<blk1_wdata4::BLK1_WDATA4_SPEC>;
#[doc = ""]
pub mod blk1_wdata4;
#[doc = "BLK1_WDATA5 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk1_wdata5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blk1_wdata5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blk1_wdata5`] module"]
pub type BLK1_WDATA5 = crate::Reg<blk1_wdata5::BLK1_WDATA5_SPEC>;
#[doc = ""]
pub mod blk1_wdata5;
#[doc = "BLK1_WDATA6 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk1_wdata6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blk1_wdata6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blk1_wdata6`] module"]
pub type BLK1_WDATA6 = crate::Reg<blk1_wdata6::BLK1_WDATA6_SPEC>;
#[doc = ""]
pub mod blk1_wdata6;
#[doc = "BLK1_WDATA7 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk1_wdata7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blk1_wdata7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blk1_wdata7`] module"]
pub type BLK1_WDATA7 = crate::Reg<blk1_wdata7::BLK1_WDATA7_SPEC>;
#[doc = ""]
pub mod blk1_wdata7;
#[doc = "BLK2_WDATA0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk2_wdata0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blk2_wdata0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blk2_wdata0`] module"]
pub type BLK2_WDATA0 = crate::Reg<blk2_wdata0::BLK2_WDATA0_SPEC>;
#[doc = ""]
pub mod blk2_wdata0;
#[doc = "BLK2_WDATA1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk2_wdata1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blk2_wdata1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blk2_wdata1`] module"]
pub type BLK2_WDATA1 = crate::Reg<blk2_wdata1::BLK2_WDATA1_SPEC>;
#[doc = ""]
pub mod blk2_wdata1;
#[doc = "BLK2_WDATA2 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk2_wdata2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blk2_wdata2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blk2_wdata2`] module"]
pub type BLK2_WDATA2 = crate::Reg<blk2_wdata2::BLK2_WDATA2_SPEC>;
#[doc = ""]
pub mod blk2_wdata2;
#[doc = "BLK2_WDATA3 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk2_wdata3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blk2_wdata3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blk2_wdata3`] module"]
pub type BLK2_WDATA3 = crate::Reg<blk2_wdata3::BLK2_WDATA3_SPEC>;
#[doc = ""]
pub mod blk2_wdata3;
#[doc = "BLK2_WDATA4 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk2_wdata4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blk2_wdata4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blk2_wdata4`] module"]
pub type BLK2_WDATA4 = crate::Reg<blk2_wdata4::BLK2_WDATA4_SPEC>;
#[doc = ""]
pub mod blk2_wdata4;
#[doc = "BLK2_WDATA5 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk2_wdata5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blk2_wdata5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blk2_wdata5`] module"]
pub type BLK2_WDATA5 = crate::Reg<blk2_wdata5::BLK2_WDATA5_SPEC>;
#[doc = ""]
pub mod blk2_wdata5;
#[doc = "BLK2_WDATA6 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk2_wdata6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blk2_wdata6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blk2_wdata6`] module"]
pub type BLK2_WDATA6 = crate::Reg<blk2_wdata6::BLK2_WDATA6_SPEC>;
#[doc = ""]
pub mod blk2_wdata6;
#[doc = "BLK2_WDATA7 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk2_wdata7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blk2_wdata7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blk2_wdata7`] module"]
pub type BLK2_WDATA7 = crate::Reg<blk2_wdata7::BLK2_WDATA7_SPEC>;
#[doc = ""]
pub mod blk2_wdata7;
#[doc = "BLK3_WDATA0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk3_wdata0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blk3_wdata0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blk3_wdata0`] module"]
pub type BLK3_WDATA0 = crate::Reg<blk3_wdata0::BLK3_WDATA0_SPEC>;
#[doc = ""]
pub mod blk3_wdata0;
#[doc = "BLK3_WDATA1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk3_wdata1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blk3_wdata1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blk3_wdata1`] module"]
pub type BLK3_WDATA1 = crate::Reg<blk3_wdata1::BLK3_WDATA1_SPEC>;
#[doc = ""]
pub mod blk3_wdata1;
#[doc = "BLK3_WDATA2 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk3_wdata2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blk3_wdata2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blk3_wdata2`] module"]
pub type BLK3_WDATA2 = crate::Reg<blk3_wdata2::BLK3_WDATA2_SPEC>;
#[doc = ""]
pub mod blk3_wdata2;
#[doc = "BLK3_WDATA3 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk3_wdata3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blk3_wdata3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blk3_wdata3`] module"]
pub type BLK3_WDATA3 = crate::Reg<blk3_wdata3::BLK3_WDATA3_SPEC>;
#[doc = ""]
pub mod blk3_wdata3;
#[doc = "BLK3_WDATA4 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk3_wdata4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blk3_wdata4`] module"]
pub type BLK3_WDATA4 = crate::Reg<blk3_wdata4::BLK3_WDATA4_SPEC>;
#[doc = ""]
pub mod blk3_wdata4;
#[doc = "BLK3_WDATA5 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk3_wdata5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blk3_wdata5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blk3_wdata5`] module"]
pub type BLK3_WDATA5 = crate::Reg<blk3_wdata5::BLK3_WDATA5_SPEC>;
#[doc = ""]
pub mod blk3_wdata5;
#[doc = "BLK3_WDATA6 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk3_wdata6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blk3_wdata6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blk3_wdata6`] module"]
pub type BLK3_WDATA6 = crate::Reg<blk3_wdata6::BLK3_WDATA6_SPEC>;
#[doc = ""]
pub mod blk3_wdata6;
#[doc = "BLK3_WDATA7 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk3_wdata7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blk3_wdata7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blk3_wdata7`] module"]
pub type BLK3_WDATA7 = crate::Reg<blk3_wdata7::BLK3_WDATA7_SPEC>;
#[doc = ""]
pub mod blk3_wdata7;
#[doc = "CLK (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk`] module"]
pub type CLK = crate::Reg<clk::CLK_SPEC>;
#[doc = ""]
pub mod clk;
#[doc = "CONF (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@conf`] module"]
pub type CONF = crate::Reg<conf::CONF_SPEC>;
#[doc = ""]
pub mod conf;
#[doc = "STATUS (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`] module"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = ""]
pub mod status;
#[doc = "CMD (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd`] module"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = ""]
pub mod cmd;
#[doc = "INT_RAW (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_raw`] module"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = ""]
pub mod int_raw;
#[doc = "INT_ST (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_st`] module"]
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
#[doc = ""]
pub mod int_st;
#[doc = "INT_ENA (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_ena`] module"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = ""]
pub mod int_ena;
#[doc = "INT_CLR (w) register accessor: \n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_clr`] module"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = ""]
pub mod int_clr;
#[doc = "DAC_CONF (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac_conf`] module"]
pub type DAC_CONF = crate::Reg<dac_conf::DAC_CONF_SPEC>;
#[doc = ""]
pub mod dac_conf;
#[doc = "DEC_STATUS (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dec_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dec_status`] module"]
pub type DEC_STATUS = crate::Reg<dec_status::DEC_STATUS_SPEC>;
#[doc = ""]
pub mod dec_status;
#[doc = "DATE (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = ""]
pub mod date;
