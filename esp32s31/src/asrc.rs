#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    chnl0_cfg0: CHNL0_CFG0,
    chnl0_cfg1: CHNL0_CFG1,
    chnl0_cfg2: CHNL0_CFG2,
    chnl0_cfg3: CHNL0_CFG3,
    chnl0_cfg4: CHNL0_CFG4,
    chnl0_cfg5: CHNL0_CFG5,
    chnl0_cfg6: CHNL0_CFG6,
    chnl0_fifo_ctrl: CHNL0_FIFO_CTRL,
    chnl0_int_raw: CHNL0_INT_RAW,
    chnl0_int_st: CHNL0_INT_ST,
    chnl0_int_ena: CHNL0_INT_ENA,
    chnl0_int_clr: CHNL0_INT_CLR,
    chnl0_out_cnt: CHNL0_OUT_CNT,
    chnl0_trace0: CHNL0_TRACE0,
    chnl0_trace1: CHNL0_TRACE1,
    chnl1_cfg0: CHNL1_CFG0,
    chnl1_cfg1: CHNL1_CFG1,
    chnl1_cfg2: CHNL1_CFG2,
    chnl1_cfg3: CHNL1_CFG3,
    chnl1_cfg4: CHNL1_CFG4,
    chnl1_cfg5: CHNL1_CFG5,
    chnl1_cfg6: CHNL1_CFG6,
    chnl1_fifo_ctrl: CHNL1_FIFO_CTRL,
    chnl1_int_raw: CHNL1_INT_RAW,
    chnl1_int_st: CHNL1_INT_ST,
    chnl1_int_ena: CHNL1_INT_ENA,
    chnl1_int_clr: CHNL1_INT_CLR,
    chnl1_out_cnt: CHNL1_OUT_CNT,
    chnl1_trace0: CHNL1_TRACE0,
    chnl1_trace1: CHNL1_TRACE1,
    _reserved30: [u8; 0x80],
    sys: SYS,
    date: DATE,
}
impl RegisterBlock {
    #[doc = "0x00 - Control and configuration registers"]
    #[inline(always)]
    pub const fn chnl0_cfg0(&self) -> &CHNL0_CFG0 {
        &self.chnl0_cfg0
    }
    #[doc = "0x04 - Control and configuration registers"]
    #[inline(always)]
    pub const fn chnl0_cfg1(&self) -> &CHNL0_CFG1 {
        &self.chnl0_cfg1
    }
    #[doc = "0x08 - Control and configuration registers"]
    #[inline(always)]
    pub const fn chnl0_cfg2(&self) -> &CHNL0_CFG2 {
        &self.chnl0_cfg2
    }
    #[doc = "0x0c - Control and configuration registers"]
    #[inline(always)]
    pub const fn chnl0_cfg3(&self) -> &CHNL0_CFG3 {
        &self.chnl0_cfg3
    }
    #[doc = "0x10 - Control and configuration registers"]
    #[inline(always)]
    pub const fn chnl0_cfg4(&self) -> &CHNL0_CFG4 {
        &self.chnl0_cfg4
    }
    #[doc = "0x14 - Control and configuration registers"]
    #[inline(always)]
    pub const fn chnl0_cfg5(&self) -> &CHNL0_CFG5 {
        &self.chnl0_cfg5
    }
    #[doc = "0x18 - Control and configuration registers"]
    #[inline(always)]
    pub const fn chnl0_cfg6(&self) -> &CHNL0_CFG6 {
        &self.chnl0_cfg6
    }
    #[doc = "0x1c - Control and configuration registers"]
    #[inline(always)]
    pub const fn chnl0_fifo_ctrl(&self) -> &CHNL0_FIFO_CTRL {
        &self.chnl0_fifo_ctrl
    }
    #[doc = "0x20 - Raw interrupt status"]
    #[inline(always)]
    pub const fn chnl0_int_raw(&self) -> &CHNL0_INT_RAW {
        &self.chnl0_int_raw
    }
    #[doc = "0x24 - Masked interrupt status"]
    #[inline(always)]
    pub const fn chnl0_int_st(&self) -> &CHNL0_INT_ST {
        &self.chnl0_int_st
    }
    #[doc = "0x28 - Interrupt enable bits"]
    #[inline(always)]
    pub const fn chnl0_int_ena(&self) -> &CHNL0_INT_ENA {
        &self.chnl0_int_ena
    }
    #[doc = "0x2c - Interrupt clear bits"]
    #[inline(always)]
    pub const fn chnl0_int_clr(&self) -> &CHNL0_INT_CLR {
        &self.chnl0_int_clr
    }
    #[doc = "0x30 - Status Registers"]
    #[inline(always)]
    pub const fn chnl0_out_cnt(&self) -> &CHNL0_OUT_CNT {
        &self.chnl0_out_cnt
    }
    #[doc = "0x34 - Status Registers"]
    #[inline(always)]
    pub const fn chnl0_trace0(&self) -> &CHNL0_TRACE0 {
        &self.chnl0_trace0
    }
    #[doc = "0x38 - Debug Register1"]
    #[inline(always)]
    pub const fn chnl0_trace1(&self) -> &CHNL0_TRACE1 {
        &self.chnl0_trace1
    }
    #[doc = "0x3c - Control and configuration registers"]
    #[inline(always)]
    pub const fn chnl1_cfg0(&self) -> &CHNL1_CFG0 {
        &self.chnl1_cfg0
    }
    #[doc = "0x40 - Control and configuration registers"]
    #[inline(always)]
    pub const fn chnl1_cfg1(&self) -> &CHNL1_CFG1 {
        &self.chnl1_cfg1
    }
    #[doc = "0x44 - Control and configuration registers"]
    #[inline(always)]
    pub const fn chnl1_cfg2(&self) -> &CHNL1_CFG2 {
        &self.chnl1_cfg2
    }
    #[doc = "0x48 - Control and configuration registers"]
    #[inline(always)]
    pub const fn chnl1_cfg3(&self) -> &CHNL1_CFG3 {
        &self.chnl1_cfg3
    }
    #[doc = "0x4c - Control and configuration registers"]
    #[inline(always)]
    pub const fn chnl1_cfg4(&self) -> &CHNL1_CFG4 {
        &self.chnl1_cfg4
    }
    #[doc = "0x50 - Control and configuration registers"]
    #[inline(always)]
    pub const fn chnl1_cfg5(&self) -> &CHNL1_CFG5 {
        &self.chnl1_cfg5
    }
    #[doc = "0x54 - Control and configuration registers"]
    #[inline(always)]
    pub const fn chnl1_cfg6(&self) -> &CHNL1_CFG6 {
        &self.chnl1_cfg6
    }
    #[doc = "0x58 - Control and configuration registers"]
    #[inline(always)]
    pub const fn chnl1_fifo_ctrl(&self) -> &CHNL1_FIFO_CTRL {
        &self.chnl1_fifo_ctrl
    }
    #[doc = "0x5c - Raw interrupt status"]
    #[inline(always)]
    pub const fn chnl1_int_raw(&self) -> &CHNL1_INT_RAW {
        &self.chnl1_int_raw
    }
    #[doc = "0x60 - Masked interrupt status"]
    #[inline(always)]
    pub const fn chnl1_int_st(&self) -> &CHNL1_INT_ST {
        &self.chnl1_int_st
    }
    #[doc = "0x64 - Interrupt enable bits"]
    #[inline(always)]
    pub const fn chnl1_int_ena(&self) -> &CHNL1_INT_ENA {
        &self.chnl1_int_ena
    }
    #[doc = "0x68 - Interrupt clear bits"]
    #[inline(always)]
    pub const fn chnl1_int_clr(&self) -> &CHNL1_INT_CLR {
        &self.chnl1_int_clr
    }
    #[doc = "0x6c - Status Registers"]
    #[inline(always)]
    pub const fn chnl1_out_cnt(&self) -> &CHNL1_OUT_CNT {
        &self.chnl1_out_cnt
    }
    #[doc = "0x70 - Status Registers"]
    #[inline(always)]
    pub const fn chnl1_trace0(&self) -> &CHNL1_TRACE0 {
        &self.chnl1_trace0
    }
    #[doc = "0x74 - Debug Register1"]
    #[inline(always)]
    pub const fn chnl1_trace1(&self) -> &CHNL1_TRACE1 {
        &self.chnl1_trace1
    }
    #[doc = "0xf8 - Control and configuration"]
    #[inline(always)]
    pub const fn sys(&self) -> &SYS {
        &self.sys
    }
    #[doc = "0xfc - Control and configuration registers"]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
#[doc = "CHNL0_CFG0 (rw) register accessor: Control and configuration registers\n\nYou can [`read`](crate::Reg::read) this register and get [`chnl0_cfg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chnl0_cfg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chnl0_cfg0`] module"]
pub type CHNL0_CFG0 = crate::Reg<chnl0_cfg0::CHNL0_CFG0_SPEC>;
#[doc = "Control and configuration registers"]
pub mod chnl0_cfg0;
#[doc = "CHNL0_CFG1 (rw) register accessor: Control and configuration registers\n\nYou can [`read`](crate::Reg::read) this register and get [`chnl0_cfg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chnl0_cfg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chnl0_cfg1`] module"]
pub type CHNL0_CFG1 = crate::Reg<chnl0_cfg1::CHNL0_CFG1_SPEC>;
#[doc = "Control and configuration registers"]
pub mod chnl0_cfg1;
#[doc = "CHNL0_CFG2 (rw) register accessor: Control and configuration registers\n\nYou can [`read`](crate::Reg::read) this register and get [`chnl0_cfg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chnl0_cfg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chnl0_cfg2`] module"]
pub type CHNL0_CFG2 = crate::Reg<chnl0_cfg2::CHNL0_CFG2_SPEC>;
#[doc = "Control and configuration registers"]
pub mod chnl0_cfg2;
#[doc = "CHNL0_CFG3 (w) register accessor: Control and configuration registers\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chnl0_cfg3::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chnl0_cfg3`] module"]
pub type CHNL0_CFG3 = crate::Reg<chnl0_cfg3::CHNL0_CFG3_SPEC>;
#[doc = "Control and configuration registers"]
pub mod chnl0_cfg3;
#[doc = "CHNL0_CFG4 (rw) register accessor: Control and configuration registers\n\nYou can [`read`](crate::Reg::read) this register and get [`chnl0_cfg4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chnl0_cfg4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chnl0_cfg4`] module"]
pub type CHNL0_CFG4 = crate::Reg<chnl0_cfg4::CHNL0_CFG4_SPEC>;
#[doc = "Control and configuration registers"]
pub mod chnl0_cfg4;
#[doc = "CHNL0_CFG5 (rw) register accessor: Control and configuration registers\n\nYou can [`read`](crate::Reg::read) this register and get [`chnl0_cfg5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chnl0_cfg5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chnl0_cfg5`] module"]
pub type CHNL0_CFG5 = crate::Reg<chnl0_cfg5::CHNL0_CFG5_SPEC>;
#[doc = "Control and configuration registers"]
pub mod chnl0_cfg5;
#[doc = "CHNL0_CFG6 (rw) register accessor: Control and configuration registers\n\nYou can [`read`](crate::Reg::read) this register and get [`chnl0_cfg6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chnl0_cfg6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chnl0_cfg6`] module"]
pub type CHNL0_CFG6 = crate::Reg<chnl0_cfg6::CHNL0_CFG6_SPEC>;
#[doc = "Control and configuration registers"]
pub mod chnl0_cfg6;
#[doc = "CHNL0_FIFO_CTRL (w) register accessor: Control and configuration registers\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chnl0_fifo_ctrl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chnl0_fifo_ctrl`] module"]
pub type CHNL0_FIFO_CTRL = crate::Reg<chnl0_fifo_ctrl::CHNL0_FIFO_CTRL_SPEC>;
#[doc = "Control and configuration registers"]
pub mod chnl0_fifo_ctrl;
#[doc = "CHNL0_INT_RAW (rw) register accessor: Raw interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`chnl0_int_raw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chnl0_int_raw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chnl0_int_raw`] module"]
pub type CHNL0_INT_RAW = crate::Reg<chnl0_int_raw::CHNL0_INT_RAW_SPEC>;
#[doc = "Raw interrupt status"]
pub mod chnl0_int_raw;
#[doc = "CHNL0_INT_ST (r) register accessor: Masked interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`chnl0_int_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chnl0_int_st`] module"]
pub type CHNL0_INT_ST = crate::Reg<chnl0_int_st::CHNL0_INT_ST_SPEC>;
#[doc = "Masked interrupt status"]
pub mod chnl0_int_st;
#[doc = "CHNL0_INT_ENA (rw) register accessor: Interrupt enable bits\n\nYou can [`read`](crate::Reg::read) this register and get [`chnl0_int_ena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chnl0_int_ena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chnl0_int_ena`] module"]
pub type CHNL0_INT_ENA = crate::Reg<chnl0_int_ena::CHNL0_INT_ENA_SPEC>;
#[doc = "Interrupt enable bits"]
pub mod chnl0_int_ena;
#[doc = "CHNL0_INT_CLR (w) register accessor: Interrupt clear bits\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chnl0_int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chnl0_int_clr`] module"]
pub type CHNL0_INT_CLR = crate::Reg<chnl0_int_clr::CHNL0_INT_CLR_SPEC>;
#[doc = "Interrupt clear bits"]
pub mod chnl0_int_clr;
#[doc = "CHNL0_OUT_CNT (r) register accessor: Status Registers\n\nYou can [`read`](crate::Reg::read) this register and get [`chnl0_out_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chnl0_out_cnt`] module"]
pub type CHNL0_OUT_CNT = crate::Reg<chnl0_out_cnt::CHNL0_OUT_CNT_SPEC>;
#[doc = "Status Registers"]
pub mod chnl0_out_cnt;
#[doc = "CHNL0_TRACE0 (r) register accessor: Status Registers\n\nYou can [`read`](crate::Reg::read) this register and get [`chnl0_trace0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chnl0_trace0`] module"]
pub type CHNL0_TRACE0 = crate::Reg<chnl0_trace0::CHNL0_TRACE0_SPEC>;
#[doc = "Status Registers"]
pub mod chnl0_trace0;
#[doc = "CHNL0_TRACE1 (r) register accessor: Debug Register1\n\nYou can [`read`](crate::Reg::read) this register and get [`chnl0_trace1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chnl0_trace1`] module"]
pub type CHNL0_TRACE1 = crate::Reg<chnl0_trace1::CHNL0_TRACE1_SPEC>;
#[doc = "Debug Register1"]
pub mod chnl0_trace1;
#[doc = "CHNL1_CFG0 (rw) register accessor: Control and configuration registers\n\nYou can [`read`](crate::Reg::read) this register and get [`chnl1_cfg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chnl1_cfg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chnl1_cfg0`] module"]
pub type CHNL1_CFG0 = crate::Reg<chnl1_cfg0::CHNL1_CFG0_SPEC>;
#[doc = "Control and configuration registers"]
pub mod chnl1_cfg0;
#[doc = "CHNL1_CFG1 (rw) register accessor: Control and configuration registers\n\nYou can [`read`](crate::Reg::read) this register and get [`chnl1_cfg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chnl1_cfg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chnl1_cfg1`] module"]
pub type CHNL1_CFG1 = crate::Reg<chnl1_cfg1::CHNL1_CFG1_SPEC>;
#[doc = "Control and configuration registers"]
pub mod chnl1_cfg1;
#[doc = "CHNL1_CFG2 (rw) register accessor: Control and configuration registers\n\nYou can [`read`](crate::Reg::read) this register and get [`chnl1_cfg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chnl1_cfg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chnl1_cfg2`] module"]
pub type CHNL1_CFG2 = crate::Reg<chnl1_cfg2::CHNL1_CFG2_SPEC>;
#[doc = "Control and configuration registers"]
pub mod chnl1_cfg2;
#[doc = "CHNL1_CFG3 (w) register accessor: Control and configuration registers\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chnl1_cfg3::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chnl1_cfg3`] module"]
pub type CHNL1_CFG3 = crate::Reg<chnl1_cfg3::CHNL1_CFG3_SPEC>;
#[doc = "Control and configuration registers"]
pub mod chnl1_cfg3;
#[doc = "CHNL1_CFG4 (rw) register accessor: Control and configuration registers\n\nYou can [`read`](crate::Reg::read) this register and get [`chnl1_cfg4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chnl1_cfg4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chnl1_cfg4`] module"]
pub type CHNL1_CFG4 = crate::Reg<chnl1_cfg4::CHNL1_CFG4_SPEC>;
#[doc = "Control and configuration registers"]
pub mod chnl1_cfg4;
#[doc = "CHNL1_CFG5 (rw) register accessor: Control and configuration registers\n\nYou can [`read`](crate::Reg::read) this register and get [`chnl1_cfg5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chnl1_cfg5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chnl1_cfg5`] module"]
pub type CHNL1_CFG5 = crate::Reg<chnl1_cfg5::CHNL1_CFG5_SPEC>;
#[doc = "Control and configuration registers"]
pub mod chnl1_cfg5;
#[doc = "CHNL1_CFG6 (rw) register accessor: Control and configuration registers\n\nYou can [`read`](crate::Reg::read) this register and get [`chnl1_cfg6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chnl1_cfg6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chnl1_cfg6`] module"]
pub type CHNL1_CFG6 = crate::Reg<chnl1_cfg6::CHNL1_CFG6_SPEC>;
#[doc = "Control and configuration registers"]
pub mod chnl1_cfg6;
#[doc = "CHNL1_FIFO_CTRL (w) register accessor: Control and configuration registers\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chnl1_fifo_ctrl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chnl1_fifo_ctrl`] module"]
pub type CHNL1_FIFO_CTRL = crate::Reg<chnl1_fifo_ctrl::CHNL1_FIFO_CTRL_SPEC>;
#[doc = "Control and configuration registers"]
pub mod chnl1_fifo_ctrl;
#[doc = "CHNL1_INT_RAW (rw) register accessor: Raw interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`chnl1_int_raw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chnl1_int_raw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chnl1_int_raw`] module"]
pub type CHNL1_INT_RAW = crate::Reg<chnl1_int_raw::CHNL1_INT_RAW_SPEC>;
#[doc = "Raw interrupt status"]
pub mod chnl1_int_raw;
#[doc = "CHNL1_INT_ST (r) register accessor: Masked interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`chnl1_int_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chnl1_int_st`] module"]
pub type CHNL1_INT_ST = crate::Reg<chnl1_int_st::CHNL1_INT_ST_SPEC>;
#[doc = "Masked interrupt status"]
pub mod chnl1_int_st;
#[doc = "CHNL1_INT_ENA (rw) register accessor: Interrupt enable bits\n\nYou can [`read`](crate::Reg::read) this register and get [`chnl1_int_ena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chnl1_int_ena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chnl1_int_ena`] module"]
pub type CHNL1_INT_ENA = crate::Reg<chnl1_int_ena::CHNL1_INT_ENA_SPEC>;
#[doc = "Interrupt enable bits"]
pub mod chnl1_int_ena;
#[doc = "CHNL1_INT_CLR (w) register accessor: Interrupt clear bits\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chnl1_int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chnl1_int_clr`] module"]
pub type CHNL1_INT_CLR = crate::Reg<chnl1_int_clr::CHNL1_INT_CLR_SPEC>;
#[doc = "Interrupt clear bits"]
pub mod chnl1_int_clr;
#[doc = "CHNL1_OUT_CNT (r) register accessor: Status Registers\n\nYou can [`read`](crate::Reg::read) this register and get [`chnl1_out_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chnl1_out_cnt`] module"]
pub type CHNL1_OUT_CNT = crate::Reg<chnl1_out_cnt::CHNL1_OUT_CNT_SPEC>;
#[doc = "Status Registers"]
pub mod chnl1_out_cnt;
#[doc = "CHNL1_TRACE0 (r) register accessor: Status Registers\n\nYou can [`read`](crate::Reg::read) this register and get [`chnl1_trace0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chnl1_trace0`] module"]
pub type CHNL1_TRACE0 = crate::Reg<chnl1_trace0::CHNL1_TRACE0_SPEC>;
#[doc = "Status Registers"]
pub mod chnl1_trace0;
#[doc = "CHNL1_TRACE1 (r) register accessor: Debug Register1\n\nYou can [`read`](crate::Reg::read) this register and get [`chnl1_trace1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chnl1_trace1`] module"]
pub type CHNL1_TRACE1 = crate::Reg<chnl1_trace1::CHNL1_TRACE1_SPEC>;
#[doc = "Debug Register1"]
pub mod chnl1_trace1;
#[doc = "SYS (rw) register accessor: Control and configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`sys::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys`] module"]
pub type SYS = crate::Reg<sys::SYS_SPEC>;
#[doc = "Control and configuration"]
pub mod sys;
#[doc = "DATE (rw) register accessor: Control and configuration registers\n\nYou can [`read`](crate::Reg::read) this register and get [`date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "Control and configuration registers"]
pub mod date;
