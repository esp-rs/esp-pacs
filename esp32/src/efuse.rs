#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - "]
    pub blk0_rdata0: BLK0_RDATA0,
    #[doc = "0x04 - "]
    pub blk0_rdata1: BLK0_RDATA1,
    #[doc = "0x08 - "]
    pub blk0_rdata2: BLK0_RDATA2,
    #[doc = "0x0c - "]
    pub blk0_rdata3: BLK0_RDATA3,
    #[doc = "0x10 - "]
    pub blk0_rdata4: BLK0_RDATA4,
    #[doc = "0x14 - "]
    pub blk0_rdata5: BLK0_RDATA5,
    #[doc = "0x18 - "]
    pub blk0_rdata6: BLK0_RDATA6,
    #[doc = "0x1c - "]
    pub blk0_wdata0: BLK0_WDATA0,
    #[doc = "0x20 - "]
    pub blk0_wdata1: BLK0_WDATA1,
    #[doc = "0x24 - "]
    pub blk0_wdata2: BLK0_WDATA2,
    #[doc = "0x28 - "]
    pub blk0_wdata3: BLK0_WDATA3,
    #[doc = "0x2c - "]
    pub blk0_wdata4: BLK0_WDATA4,
    #[doc = "0x30 - "]
    pub blk0_wdata5: BLK0_WDATA5,
    #[doc = "0x34 - "]
    pub blk0_wdata6: BLK0_WDATA6,
    #[doc = "0x38 - "]
    pub blk1_rdata0: BLK1_RDATA0,
    #[doc = "0x3c - "]
    pub blk1_rdata1: BLK1_RDATA1,
    #[doc = "0x40 - "]
    pub blk1_rdata2: BLK1_RDATA2,
    #[doc = "0x44 - "]
    pub blk1_rdata3: BLK1_RDATA3,
    #[doc = "0x48 - "]
    pub blk1_rdata4: BLK1_RDATA4,
    #[doc = "0x4c - "]
    pub blk1_rdata5: BLK1_RDATA5,
    #[doc = "0x50 - "]
    pub blk1_rdata6: BLK1_RDATA6,
    #[doc = "0x54 - "]
    pub blk1_rdata7: BLK1_RDATA7,
    #[doc = "0x58 - "]
    pub blk2_rdata0: BLK2_RDATA0,
    #[doc = "0x5c - "]
    pub blk2_rdata1: BLK2_RDATA1,
    #[doc = "0x60 - "]
    pub blk2_rdata2: BLK2_RDATA2,
    #[doc = "0x64 - "]
    pub blk2_rdata3: BLK2_RDATA3,
    #[doc = "0x68 - "]
    pub blk2_rdata4: BLK2_RDATA4,
    #[doc = "0x6c - "]
    pub blk2_rdata5: BLK2_RDATA5,
    #[doc = "0x70 - "]
    pub blk2_rdata6: BLK2_RDATA6,
    #[doc = "0x74 - "]
    pub blk2_rdata7: BLK2_RDATA7,
    #[doc = "0x78 - "]
    pub blk3_rdata0: BLK3_RDATA0,
    #[doc = "0x7c - "]
    pub blk3_rdata1: BLK3_RDATA1,
    #[doc = "0x80 - "]
    pub blk3_rdata2: BLK3_RDATA2,
    #[doc = "0x84 - "]
    pub blk3_rdata3: BLK3_RDATA3,
    #[doc = "0x88 - "]
    pub blk3_rdata4: BLK3_RDATA4,
    #[doc = "0x8c - "]
    pub blk3_rdata5: BLK3_RDATA5,
    #[doc = "0x90 - "]
    pub blk3_rdata6: BLK3_RDATA6,
    #[doc = "0x94 - "]
    pub blk3_rdata7: BLK3_RDATA7,
    #[doc = "0x98 - "]
    pub blk1_wdata0: BLK1_WDATA0,
    #[doc = "0x9c - "]
    pub blk1_wdata1: BLK1_WDATA1,
    #[doc = "0xa0 - "]
    pub blk1_wdata2: BLK1_WDATA2,
    #[doc = "0xa4 - "]
    pub blk1_wdata3: BLK1_WDATA3,
    #[doc = "0xa8 - "]
    pub blk1_wdata4: BLK1_WDATA4,
    #[doc = "0xac - "]
    pub blk1_wdata5: BLK1_WDATA5,
    #[doc = "0xb0 - "]
    pub blk1_wdata6: BLK1_WDATA6,
    #[doc = "0xb4 - "]
    pub blk1_wdata7: BLK1_WDATA7,
    #[doc = "0xb8 - "]
    pub blk2_wdata0: BLK2_WDATA0,
    #[doc = "0xbc - "]
    pub blk2_wdata1: BLK2_WDATA1,
    #[doc = "0xc0 - "]
    pub blk2_wdata2: BLK2_WDATA2,
    #[doc = "0xc4 - "]
    pub blk2_wdata3: BLK2_WDATA3,
    #[doc = "0xc8 - "]
    pub blk2_wdata4: BLK2_WDATA4,
    #[doc = "0xcc - "]
    pub blk2_wdata5: BLK2_WDATA5,
    #[doc = "0xd0 - "]
    pub blk2_wdata6: BLK2_WDATA6,
    #[doc = "0xd4 - "]
    pub blk2_wdata7: BLK2_WDATA7,
    #[doc = "0xd8 - "]
    pub blk3_wdata0: BLK3_WDATA0,
    #[doc = "0xdc - "]
    pub blk3_wdata1: BLK3_WDATA1,
    #[doc = "0xe0 - "]
    pub blk3_wdata2: BLK3_WDATA2,
    #[doc = "0xe4 - "]
    pub blk3_wdata3: BLK3_WDATA3,
    #[doc = "0xe8 - "]
    pub blk3_wdata4: BLK3_WDATA4,
    #[doc = "0xec - "]
    pub blk3_wdata5: BLK3_WDATA5,
    #[doc = "0xf0 - "]
    pub blk3_wdata6: BLK3_WDATA6,
    #[doc = "0xf4 - "]
    pub blk3_wdata7: BLK3_WDATA7,
    #[doc = "0xf8 - "]
    pub clk: CLK,
    #[doc = "0xfc - "]
    pub conf: CONF,
    #[doc = "0x100 - "]
    pub status: STATUS,
    #[doc = "0x104 - "]
    pub cmd: CMD,
    #[doc = "0x108 - "]
    pub int_raw: INT_RAW,
    #[doc = "0x10c - "]
    pub int_st: INT_ST,
    #[doc = "0x110 - "]
    pub int_ena: INT_ENA,
    #[doc = "0x114 - "]
    pub int_clr: INT_CLR,
    #[doc = "0x118 - "]
    pub dac_conf: DAC_CONF,
    #[doc = "0x11c - "]
    pub dec_status: DEC_STATUS,
    _reserved72: [u8; 0xdc],
    #[doc = "0x1fc - "]
    pub date: DATE,
}
#[doc = "BLK0_RDATA0 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk0_rdata0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`blk0_rdata0`] module"]
pub type BLK0_RDATA0 = crate::Reg<blk0_rdata0::BLK0_RDATA0_SPEC>;
#[doc = ""]
pub mod blk0_rdata0;
#[doc = "BLK0_RDATA1 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk0_rdata1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`blk0_rdata1`] module"]
pub type BLK0_RDATA1 = crate::Reg<blk0_rdata1::BLK0_RDATA1_SPEC>;
#[doc = ""]
pub mod blk0_rdata1;
#[doc = "BLK0_RDATA2 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk0_rdata2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blk0_rdata2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`blk0_rdata2`] module"]
pub type BLK0_RDATA2 = crate::Reg<blk0_rdata2::BLK0_RDATA2_SPEC>;
#[doc = ""]
pub mod blk0_rdata2;
#[doc = "BLK0_RDATA3 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk0_rdata3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blk0_rdata3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`blk0_rdata3`] module"]
pub type BLK0_RDATA3 = crate::Reg<blk0_rdata3::BLK0_RDATA3_SPEC>;
#[doc = ""]
pub mod blk0_rdata3;
#[doc = "BLK0_RDATA4 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk0_rdata4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blk0_rdata4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`blk0_rdata4`] module"]
pub type BLK0_RDATA4 = crate::Reg<blk0_rdata4::BLK0_RDATA4_SPEC>;
#[doc = ""]
pub mod blk0_rdata4;
#[doc = "BLK0_RDATA5 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk0_rdata5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blk0_rdata5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`blk0_rdata5`] module"]
pub type BLK0_RDATA5 = crate::Reg<blk0_rdata5::BLK0_RDATA5_SPEC>;
#[doc = ""]
pub mod blk0_rdata5;
#[doc = "BLK0_RDATA6 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk0_rdata6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blk0_rdata6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`blk0_rdata6`] module"]
pub type BLK0_RDATA6 = crate::Reg<blk0_rdata6::BLK0_RDATA6_SPEC>;
#[doc = ""]
pub mod blk0_rdata6;
#[doc = "BLK0_WDATA0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk0_wdata0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blk0_wdata0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`blk0_wdata0`] module"]
pub type BLK0_WDATA0 = crate::Reg<blk0_wdata0::BLK0_WDATA0_SPEC>;
#[doc = ""]
pub mod blk0_wdata0;
#[doc = "BLK0_WDATA1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk0_wdata1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blk0_wdata1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`blk0_wdata1`] module"]
pub type BLK0_WDATA1 = crate::Reg<blk0_wdata1::BLK0_WDATA1_SPEC>;
#[doc = ""]
pub mod blk0_wdata1;
#[doc = "BLK0_WDATA2 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk0_wdata2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blk0_wdata2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`blk0_wdata2`] module"]
pub type BLK0_WDATA2 = crate::Reg<blk0_wdata2::BLK0_WDATA2_SPEC>;
#[doc = ""]
pub mod blk0_wdata2;
#[doc = "BLK0_WDATA3 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk0_wdata3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blk0_wdata3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`blk0_wdata3`] module"]
pub type BLK0_WDATA3 = crate::Reg<blk0_wdata3::BLK0_WDATA3_SPEC>;
#[doc = ""]
pub mod blk0_wdata3;
#[doc = "BLK0_WDATA4 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk0_wdata4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blk0_wdata4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`blk0_wdata4`] module"]
pub type BLK0_WDATA4 = crate::Reg<blk0_wdata4::BLK0_WDATA4_SPEC>;
#[doc = ""]
pub mod blk0_wdata4;
#[doc = "BLK0_WDATA5 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk0_wdata5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blk0_wdata5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`blk0_wdata5`] module"]
pub type BLK0_WDATA5 = crate::Reg<blk0_wdata5::BLK0_WDATA5_SPEC>;
#[doc = ""]
pub mod blk0_wdata5;
#[doc = "BLK0_WDATA6 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk0_wdata6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blk0_wdata6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`blk0_wdata6`] module"]
pub type BLK0_WDATA6 = crate::Reg<blk0_wdata6::BLK0_WDATA6_SPEC>;
#[doc = ""]
pub mod blk0_wdata6;
#[doc = "BLK1_RDATA0 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk1_rdata0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`blk1_rdata0`] module"]
pub type BLK1_RDATA0 = crate::Reg<blk1_rdata0::BLK1_RDATA0_SPEC>;
#[doc = ""]
pub mod blk1_rdata0;
#[doc = "BLK1_RDATA1 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk1_rdata1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`blk1_rdata1`] module"]
pub type BLK1_RDATA1 = crate::Reg<blk1_rdata1::BLK1_RDATA1_SPEC>;
#[doc = ""]
pub mod blk1_rdata1;
#[doc = "BLK1_RDATA2 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk1_rdata2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`blk1_rdata2`] module"]
pub type BLK1_RDATA2 = crate::Reg<blk1_rdata2::BLK1_RDATA2_SPEC>;
#[doc = ""]
pub mod blk1_rdata2;
#[doc = "BLK1_RDATA3 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk1_rdata3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`blk1_rdata3`] module"]
pub type BLK1_RDATA3 = crate::Reg<blk1_rdata3::BLK1_RDATA3_SPEC>;
#[doc = ""]
pub mod blk1_rdata3;
#[doc = "BLK1_RDATA4 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk1_rdata4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`blk1_rdata4`] module"]
pub type BLK1_RDATA4 = crate::Reg<blk1_rdata4::BLK1_RDATA4_SPEC>;
#[doc = ""]
pub mod blk1_rdata4;
#[doc = "BLK1_RDATA5 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk1_rdata5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`blk1_rdata5`] module"]
pub type BLK1_RDATA5 = crate::Reg<blk1_rdata5::BLK1_RDATA5_SPEC>;
#[doc = ""]
pub mod blk1_rdata5;
#[doc = "BLK1_RDATA6 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk1_rdata6::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`blk1_rdata6`] module"]
pub type BLK1_RDATA6 = crate::Reg<blk1_rdata6::BLK1_RDATA6_SPEC>;
#[doc = ""]
pub mod blk1_rdata6;
#[doc = "BLK1_RDATA7 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk1_rdata7::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`blk1_rdata7`] module"]
pub type BLK1_RDATA7 = crate::Reg<blk1_rdata7::BLK1_RDATA7_SPEC>;
#[doc = ""]
pub mod blk1_rdata7;
#[doc = "BLK2_RDATA0 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk2_rdata0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`blk2_rdata0`] module"]
pub type BLK2_RDATA0 = crate::Reg<blk2_rdata0::BLK2_RDATA0_SPEC>;
#[doc = ""]
pub mod blk2_rdata0;
#[doc = "BLK2_RDATA1 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk2_rdata1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`blk2_rdata1`] module"]
pub type BLK2_RDATA1 = crate::Reg<blk2_rdata1::BLK2_RDATA1_SPEC>;
#[doc = ""]
pub mod blk2_rdata1;
#[doc = "BLK2_RDATA2 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk2_rdata2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`blk2_rdata2`] module"]
pub type BLK2_RDATA2 = crate::Reg<blk2_rdata2::BLK2_RDATA2_SPEC>;
#[doc = ""]
pub mod blk2_rdata2;
#[doc = "BLK2_RDATA3 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk2_rdata3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`blk2_rdata3`] module"]
pub type BLK2_RDATA3 = crate::Reg<blk2_rdata3::BLK2_RDATA3_SPEC>;
#[doc = ""]
pub mod blk2_rdata3;
#[doc = "BLK2_RDATA4 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk2_rdata4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`blk2_rdata4`] module"]
pub type BLK2_RDATA4 = crate::Reg<blk2_rdata4::BLK2_RDATA4_SPEC>;
#[doc = ""]
pub mod blk2_rdata4;
#[doc = "BLK2_RDATA5 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk2_rdata5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`blk2_rdata5`] module"]
pub type BLK2_RDATA5 = crate::Reg<blk2_rdata5::BLK2_RDATA5_SPEC>;
#[doc = ""]
pub mod blk2_rdata5;
#[doc = "BLK2_RDATA6 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk2_rdata6::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`blk2_rdata6`] module"]
pub type BLK2_RDATA6 = crate::Reg<blk2_rdata6::BLK2_RDATA6_SPEC>;
#[doc = ""]
pub mod blk2_rdata6;
#[doc = "BLK2_RDATA7 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk2_rdata7::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`blk2_rdata7`] module"]
pub type BLK2_RDATA7 = crate::Reg<blk2_rdata7::BLK2_RDATA7_SPEC>;
#[doc = ""]
pub mod blk2_rdata7;
#[doc = "BLK3_RDATA0 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk3_rdata0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`blk3_rdata0`] module"]
pub type BLK3_RDATA0 = crate::Reg<blk3_rdata0::BLK3_RDATA0_SPEC>;
#[doc = ""]
pub mod blk3_rdata0;
#[doc = "BLK3_RDATA1 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk3_rdata1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`blk3_rdata1`] module"]
pub type BLK3_RDATA1 = crate::Reg<blk3_rdata1::BLK3_RDATA1_SPEC>;
#[doc = ""]
pub mod blk3_rdata1;
#[doc = "BLK3_RDATA2 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk3_rdata2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`blk3_rdata2`] module"]
pub type BLK3_RDATA2 = crate::Reg<blk3_rdata2::BLK3_RDATA2_SPEC>;
#[doc = ""]
pub mod blk3_rdata2;
#[doc = "BLK3_RDATA3 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk3_rdata3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blk3_rdata3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`blk3_rdata3`] module"]
pub type BLK3_RDATA3 = crate::Reg<blk3_rdata3::BLK3_RDATA3_SPEC>;
#[doc = ""]
pub mod blk3_rdata3;
#[doc = "BLK3_RDATA4 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk3_rdata4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`blk3_rdata4`] module"]
pub type BLK3_RDATA4 = crate::Reg<blk3_rdata4::BLK3_RDATA4_SPEC>;
#[doc = ""]
pub mod blk3_rdata4;
#[doc = "BLK3_RDATA5 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk3_rdata5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`blk3_rdata5`] module"]
pub type BLK3_RDATA5 = crate::Reg<blk3_rdata5::BLK3_RDATA5_SPEC>;
#[doc = ""]
pub mod blk3_rdata5;
#[doc = "BLK3_RDATA6 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk3_rdata6::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`blk3_rdata6`] module"]
pub type BLK3_RDATA6 = crate::Reg<blk3_rdata6::BLK3_RDATA6_SPEC>;
#[doc = ""]
pub mod blk3_rdata6;
#[doc = "BLK3_RDATA7 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk3_rdata7::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`blk3_rdata7`] module"]
pub type BLK3_RDATA7 = crate::Reg<blk3_rdata7::BLK3_RDATA7_SPEC>;
#[doc = ""]
pub mod blk3_rdata7;
#[doc = "BLK1_WDATA0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk1_wdata0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blk1_wdata0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`blk1_wdata0`] module"]
pub type BLK1_WDATA0 = crate::Reg<blk1_wdata0::BLK1_WDATA0_SPEC>;
#[doc = ""]
pub mod blk1_wdata0;
#[doc = "BLK1_WDATA1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk1_wdata1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blk1_wdata1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`blk1_wdata1`] module"]
pub type BLK1_WDATA1 = crate::Reg<blk1_wdata1::BLK1_WDATA1_SPEC>;
#[doc = ""]
pub mod blk1_wdata1;
#[doc = "BLK1_WDATA2 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk1_wdata2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blk1_wdata2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`blk1_wdata2`] module"]
pub type BLK1_WDATA2 = crate::Reg<blk1_wdata2::BLK1_WDATA2_SPEC>;
#[doc = ""]
pub mod blk1_wdata2;
#[doc = "BLK1_WDATA3 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk1_wdata3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blk1_wdata3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`blk1_wdata3`] module"]
pub type BLK1_WDATA3 = crate::Reg<blk1_wdata3::BLK1_WDATA3_SPEC>;
#[doc = ""]
pub mod blk1_wdata3;
#[doc = "BLK1_WDATA4 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk1_wdata4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blk1_wdata4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`blk1_wdata4`] module"]
pub type BLK1_WDATA4 = crate::Reg<blk1_wdata4::BLK1_WDATA4_SPEC>;
#[doc = ""]
pub mod blk1_wdata4;
#[doc = "BLK1_WDATA5 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk1_wdata5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blk1_wdata5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`blk1_wdata5`] module"]
pub type BLK1_WDATA5 = crate::Reg<blk1_wdata5::BLK1_WDATA5_SPEC>;
#[doc = ""]
pub mod blk1_wdata5;
#[doc = "BLK1_WDATA6 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk1_wdata6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blk1_wdata6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`blk1_wdata6`] module"]
pub type BLK1_WDATA6 = crate::Reg<blk1_wdata6::BLK1_WDATA6_SPEC>;
#[doc = ""]
pub mod blk1_wdata6;
#[doc = "BLK1_WDATA7 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk1_wdata7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blk1_wdata7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`blk1_wdata7`] module"]
pub type BLK1_WDATA7 = crate::Reg<blk1_wdata7::BLK1_WDATA7_SPEC>;
#[doc = ""]
pub mod blk1_wdata7;
#[doc = "BLK2_WDATA0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk2_wdata0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blk2_wdata0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`blk2_wdata0`] module"]
pub type BLK2_WDATA0 = crate::Reg<blk2_wdata0::BLK2_WDATA0_SPEC>;
#[doc = ""]
pub mod blk2_wdata0;
#[doc = "BLK2_WDATA1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk2_wdata1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blk2_wdata1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`blk2_wdata1`] module"]
pub type BLK2_WDATA1 = crate::Reg<blk2_wdata1::BLK2_WDATA1_SPEC>;
#[doc = ""]
pub mod blk2_wdata1;
#[doc = "BLK2_WDATA2 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk2_wdata2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blk2_wdata2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`blk2_wdata2`] module"]
pub type BLK2_WDATA2 = crate::Reg<blk2_wdata2::BLK2_WDATA2_SPEC>;
#[doc = ""]
pub mod blk2_wdata2;
#[doc = "BLK2_WDATA3 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk2_wdata3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blk2_wdata3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`blk2_wdata3`] module"]
pub type BLK2_WDATA3 = crate::Reg<blk2_wdata3::BLK2_WDATA3_SPEC>;
#[doc = ""]
pub mod blk2_wdata3;
#[doc = "BLK2_WDATA4 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk2_wdata4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blk2_wdata4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`blk2_wdata4`] module"]
pub type BLK2_WDATA4 = crate::Reg<blk2_wdata4::BLK2_WDATA4_SPEC>;
#[doc = ""]
pub mod blk2_wdata4;
#[doc = "BLK2_WDATA5 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk2_wdata5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blk2_wdata5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`blk2_wdata5`] module"]
pub type BLK2_WDATA5 = crate::Reg<blk2_wdata5::BLK2_WDATA5_SPEC>;
#[doc = ""]
pub mod blk2_wdata5;
#[doc = "BLK2_WDATA6 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk2_wdata6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blk2_wdata6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`blk2_wdata6`] module"]
pub type BLK2_WDATA6 = crate::Reg<blk2_wdata6::BLK2_WDATA6_SPEC>;
#[doc = ""]
pub mod blk2_wdata6;
#[doc = "BLK2_WDATA7 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk2_wdata7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blk2_wdata7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`blk2_wdata7`] module"]
pub type BLK2_WDATA7 = crate::Reg<blk2_wdata7::BLK2_WDATA7_SPEC>;
#[doc = ""]
pub mod blk2_wdata7;
#[doc = "BLK3_WDATA0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk3_wdata0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blk3_wdata0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`blk3_wdata0`] module"]
pub type BLK3_WDATA0 = crate::Reg<blk3_wdata0::BLK3_WDATA0_SPEC>;
#[doc = ""]
pub mod blk3_wdata0;
#[doc = "BLK3_WDATA1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk3_wdata1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blk3_wdata1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`blk3_wdata1`] module"]
pub type BLK3_WDATA1 = crate::Reg<blk3_wdata1::BLK3_WDATA1_SPEC>;
#[doc = ""]
pub mod blk3_wdata1;
#[doc = "BLK3_WDATA2 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk3_wdata2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blk3_wdata2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`blk3_wdata2`] module"]
pub type BLK3_WDATA2 = crate::Reg<blk3_wdata2::BLK3_WDATA2_SPEC>;
#[doc = ""]
pub mod blk3_wdata2;
#[doc = "BLK3_WDATA3 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk3_wdata3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blk3_wdata3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`blk3_wdata3`] module"]
pub type BLK3_WDATA3 = crate::Reg<blk3_wdata3::BLK3_WDATA3_SPEC>;
#[doc = ""]
pub mod blk3_wdata3;
#[doc = "BLK3_WDATA4 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk3_wdata4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`blk3_wdata4`] module"]
pub type BLK3_WDATA4 = crate::Reg<blk3_wdata4::BLK3_WDATA4_SPEC>;
#[doc = ""]
pub mod blk3_wdata4;
#[doc = "BLK3_WDATA5 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk3_wdata5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blk3_wdata5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`blk3_wdata5`] module"]
pub type BLK3_WDATA5 = crate::Reg<blk3_wdata5::BLK3_WDATA5_SPEC>;
#[doc = ""]
pub mod blk3_wdata5;
#[doc = "BLK3_WDATA6 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk3_wdata6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blk3_wdata6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`blk3_wdata6`] module"]
pub type BLK3_WDATA6 = crate::Reg<blk3_wdata6::BLK3_WDATA6_SPEC>;
#[doc = ""]
pub mod blk3_wdata6;
#[doc = "BLK3_WDATA7 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk3_wdata7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blk3_wdata7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`blk3_wdata7`] module"]
pub type BLK3_WDATA7 = crate::Reg<blk3_wdata7::BLK3_WDATA7_SPEC>;
#[doc = ""]
pub mod blk3_wdata7;
#[doc = "CLK (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`clk`] module"]
pub type CLK = crate::Reg<clk::CLK_SPEC>;
#[doc = ""]
pub mod clk;
#[doc = "CONF (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`conf`] module"]
pub type CONF = crate::Reg<conf::CONF_SPEC>;
#[doc = ""]
pub mod conf;
#[doc = "STATUS (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`status`] module"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = ""]
pub mod status;
#[doc = "CMD (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cmd`] module"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = ""]
pub mod cmd;
#[doc = "INT_RAW (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`int_raw`] module"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = ""]
pub mod int_raw;
#[doc = "INT_ST (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`int_st`] module"]
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
#[doc = ""]
pub mod int_st;
#[doc = "INT_ENA (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`int_ena`] module"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = ""]
pub mod int_ena;
#[doc = "INT_CLR (w) register accessor: \n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`int_clr`] module"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = ""]
pub mod int_clr;
#[doc = "DAC_CONF (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dac_conf`] module"]
pub type DAC_CONF = crate::Reg<dac_conf::DAC_CONF_SPEC>;
#[doc = ""]
pub mod dac_conf;
#[doc = "DEC_STATUS (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dec_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dec_status`] module"]
pub type DEC_STATUS = crate::Reg<dec_status::DEC_STATUS_SPEC>;
#[doc = ""]
pub mod dec_status;
#[doc = "DATE (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = ""]
pub mod date;
