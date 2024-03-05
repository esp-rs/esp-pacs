#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    mode: MODE,
    cmd: CMD,
    status: STATUS,
    int_raw: INT_RAW,
    int_ena: INT_ENA,
    _reserved5: [u8; 0x04],
    bus_timing_0: BUS_TIMING_0,
    bus_timing_1: BUS_TIMING_1,
    _reserved7: [u8; 0x0c],
    arb_lost_cap: ARB_LOST_CAP,
    err_code_cap: ERR_CODE_CAP,
    err_warning_limit: ERR_WARNING_LIMIT,
    rx_err_cnt: RX_ERR_CNT,
    tx_err_cnt: TX_ERR_CNT,
    data_0: DATA_0,
    data_1: DATA_1,
    data_2: DATA_2,
    data_3: DATA_3,
    data_4: DATA_4,
    data_5: DATA_5,
    data_6: DATA_6,
    data_7: DATA_7,
    data_8: DATA_8,
    data_9: DATA_9,
    data_10: DATA_10,
    data_11: DATA_11,
    data_12: DATA_12,
    rx_message_cnt: RX_MESSAGE_CNT,
    _reserved26: [u8; 0x04],
    clock_divider: CLOCK_DIVIDER,
}
impl RegisterBlock {
    #[doc = "0x00 - Mode Register"]
    #[inline(always)]
    pub const fn mode(&self) -> &MODE {
        &self.mode
    }
    #[doc = "0x04 - Command Register"]
    #[inline(always)]
    pub const fn cmd(&self) -> &CMD {
        &self.cmd
    }
    #[doc = "0x08 - Status register"]
    #[inline(always)]
    pub const fn status(&self) -> &STATUS {
        &self.status
    }
    #[doc = "0x0c - Interrupt Register"]
    #[inline(always)]
    pub const fn int_raw(&self) -> &INT_RAW {
        &self.int_raw
    }
    #[doc = "0x10 - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn int_ena(&self) -> &INT_ENA {
        &self.int_ena
    }
    #[doc = "0x18 - Bus Timing Register 0"]
    #[inline(always)]
    pub const fn bus_timing_0(&self) -> &BUS_TIMING_0 {
        &self.bus_timing_0
    }
    #[doc = "0x1c - Bus Timing Register 1"]
    #[inline(always)]
    pub const fn bus_timing_1(&self) -> &BUS_TIMING_1 {
        &self.bus_timing_1
    }
    #[doc = "0x2c - Arbitration Lost Capture Register"]
    #[inline(always)]
    pub const fn arb_lost_cap(&self) -> &ARB_LOST_CAP {
        &self.arb_lost_cap
    }
    #[doc = "0x30 - Error Code Capture Register"]
    #[inline(always)]
    pub const fn err_code_cap(&self) -> &ERR_CODE_CAP {
        &self.err_code_cap
    }
    #[doc = "0x34 - Error Warning Limit Register"]
    #[inline(always)]
    pub const fn err_warning_limit(&self) -> &ERR_WARNING_LIMIT {
        &self.err_warning_limit
    }
    #[doc = "0x38 - Receive Error Counter Register"]
    #[inline(always)]
    pub const fn rx_err_cnt(&self) -> &RX_ERR_CNT {
        &self.rx_err_cnt
    }
    #[doc = "0x3c - Transmit Error Counter Register"]
    #[inline(always)]
    pub const fn tx_err_cnt(&self) -> &TX_ERR_CNT {
        &self.tx_err_cnt
    }
    #[doc = "0x40 - Data register 0"]
    #[inline(always)]
    pub const fn data_0(&self) -> &DATA_0 {
        &self.data_0
    }
    #[doc = "0x44 - Data register 1"]
    #[inline(always)]
    pub const fn data_1(&self) -> &DATA_1 {
        &self.data_1
    }
    #[doc = "0x48 - Data register 2"]
    #[inline(always)]
    pub const fn data_2(&self) -> &DATA_2 {
        &self.data_2
    }
    #[doc = "0x4c - Data register 3"]
    #[inline(always)]
    pub const fn data_3(&self) -> &DATA_3 {
        &self.data_3
    }
    #[doc = "0x50 - Data register 4"]
    #[inline(always)]
    pub const fn data_4(&self) -> &DATA_4 {
        &self.data_4
    }
    #[doc = "0x54 - Data register 5"]
    #[inline(always)]
    pub const fn data_5(&self) -> &DATA_5 {
        &self.data_5
    }
    #[doc = "0x58 - Data register 6"]
    #[inline(always)]
    pub const fn data_6(&self) -> &DATA_6 {
        &self.data_6
    }
    #[doc = "0x5c - Data register 7"]
    #[inline(always)]
    pub const fn data_7(&self) -> &DATA_7 {
        &self.data_7
    }
    #[doc = "0x60 - Data register 8"]
    #[inline(always)]
    pub const fn data_8(&self) -> &DATA_8 {
        &self.data_8
    }
    #[doc = "0x64 - Data register 9"]
    #[inline(always)]
    pub const fn data_9(&self) -> &DATA_9 {
        &self.data_9
    }
    #[doc = "0x68 - Data register 10"]
    #[inline(always)]
    pub const fn data_10(&self) -> &DATA_10 {
        &self.data_10
    }
    #[doc = "0x6c - Data register 11"]
    #[inline(always)]
    pub const fn data_11(&self) -> &DATA_11 {
        &self.data_11
    }
    #[doc = "0x70 - Data register 12"]
    #[inline(always)]
    pub const fn data_12(&self) -> &DATA_12 {
        &self.data_12
    }
    #[doc = "0x74 - Receive Message Counter Register"]
    #[inline(always)]
    pub const fn rx_message_cnt(&self) -> &RX_MESSAGE_CNT {
        &self.rx_message_cnt
    }
    #[doc = "0x7c - Clock Divider register"]
    #[inline(always)]
    pub const fn clock_divider(&self) -> &CLOCK_DIVIDER {
        &self.clock_divider
    }
}
#[doc = "MODE (rw) register accessor: Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mode`] module"]
pub type MODE = crate::Reg<mode::MODE_SPEC>;
#[doc = "Mode Register"]
pub mod mode;
#[doc = "CMD (w) register accessor: Command Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd`] module"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "Command Register"]
pub mod cmd;
#[doc = "STATUS (r) register accessor: Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`] module"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status register"]
pub mod status;
#[doc = "INT_RAW (r) register accessor: Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_raw`] module"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = "Interrupt Register"]
pub mod int_raw;
#[doc = "INT_ENA (rw) register accessor: Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_ena`] module"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod int_ena;
#[doc = "BUS_TIMING_0 (rw) register accessor: Bus Timing Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bus_timing_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bus_timing_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bus_timing_0`] module"]
pub type BUS_TIMING_0 = crate::Reg<bus_timing_0::BUS_TIMING_0_SPEC>;
#[doc = "Bus Timing Register 0"]
pub mod bus_timing_0;
#[doc = "BUS_TIMING_1 (rw) register accessor: Bus Timing Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bus_timing_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bus_timing_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bus_timing_1`] module"]
pub type BUS_TIMING_1 = crate::Reg<bus_timing_1::BUS_TIMING_1_SPEC>;
#[doc = "Bus Timing Register 1"]
pub mod bus_timing_1;
#[doc = "ARB_LOST_CAP (r) register accessor: Arbitration Lost Capture Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arb_lost_cap::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_lost_cap`] module"]
pub type ARB_LOST_CAP = crate::Reg<arb_lost_cap::ARB_LOST_CAP_SPEC>;
#[doc = "Arbitration Lost Capture Register"]
pub mod arb_lost_cap;
#[doc = "ERR_CODE_CAP (r) register accessor: Error Code Capture Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`err_code_cap::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@err_code_cap`] module"]
pub type ERR_CODE_CAP = crate::Reg<err_code_cap::ERR_CODE_CAP_SPEC>;
#[doc = "Error Code Capture Register"]
pub mod err_code_cap;
#[doc = "ERR_WARNING_LIMIT (rw) register accessor: Error Warning Limit Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`err_warning_limit::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`err_warning_limit::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@err_warning_limit`] module"]
pub type ERR_WARNING_LIMIT = crate::Reg<err_warning_limit::ERR_WARNING_LIMIT_SPEC>;
#[doc = "Error Warning Limit Register"]
pub mod err_warning_limit;
#[doc = "RX_ERR_CNT (rw) register accessor: Receive Error Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_err_cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_err_cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_err_cnt`] module"]
pub type RX_ERR_CNT = crate::Reg<rx_err_cnt::RX_ERR_CNT_SPEC>;
#[doc = "Receive Error Counter Register"]
pub mod rx_err_cnt;
#[doc = "TX_ERR_CNT (rw) register accessor: Transmit Error Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_err_cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_err_cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_err_cnt`] module"]
pub type TX_ERR_CNT = crate::Reg<tx_err_cnt::TX_ERR_CNT_SPEC>;
#[doc = "Transmit Error Counter Register"]
pub mod tx_err_cnt;
#[doc = "DATA_0 (rw) register accessor: Data register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data_0`] module"]
pub type DATA_0 = crate::Reg<data_0::DATA_0_SPEC>;
#[doc = "Data register 0"]
pub mod data_0;
#[doc = "DATA_1 (rw) register accessor: Data register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data_1`] module"]
pub type DATA_1 = crate::Reg<data_1::DATA_1_SPEC>;
#[doc = "Data register 1"]
pub mod data_1;
#[doc = "DATA_2 (rw) register accessor: Data register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data_2`] module"]
pub type DATA_2 = crate::Reg<data_2::DATA_2_SPEC>;
#[doc = "Data register 2"]
pub mod data_2;
#[doc = "DATA_3 (rw) register accessor: Data register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data_3`] module"]
pub type DATA_3 = crate::Reg<data_3::DATA_3_SPEC>;
#[doc = "Data register 3"]
pub mod data_3;
#[doc = "DATA_4 (rw) register accessor: Data register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data_4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data_4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data_4`] module"]
pub type DATA_4 = crate::Reg<data_4::DATA_4_SPEC>;
#[doc = "Data register 4"]
pub mod data_4;
#[doc = "DATA_5 (rw) register accessor: Data register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data_5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data_5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data_5`] module"]
pub type DATA_5 = crate::Reg<data_5::DATA_5_SPEC>;
#[doc = "Data register 5"]
pub mod data_5;
#[doc = "DATA_6 (rw) register accessor: Data register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data_6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data_6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data_6`] module"]
pub type DATA_6 = crate::Reg<data_6::DATA_6_SPEC>;
#[doc = "Data register 6"]
pub mod data_6;
#[doc = "DATA_7 (rw) register accessor: Data register 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data_7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data_7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data_7`] module"]
pub type DATA_7 = crate::Reg<data_7::DATA_7_SPEC>;
#[doc = "Data register 7"]
pub mod data_7;
#[doc = "DATA_8 (rw) register accessor: Data register 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data_8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data_8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data_8`] module"]
pub type DATA_8 = crate::Reg<data_8::DATA_8_SPEC>;
#[doc = "Data register 8"]
pub mod data_8;
#[doc = "DATA_9 (rw) register accessor: Data register 9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data_9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data_9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data_9`] module"]
pub type DATA_9 = crate::Reg<data_9::DATA_9_SPEC>;
#[doc = "Data register 9"]
pub mod data_9;
#[doc = "DATA_10 (rw) register accessor: Data register 10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data_10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data_10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data_10`] module"]
pub type DATA_10 = crate::Reg<data_10::DATA_10_SPEC>;
#[doc = "Data register 10"]
pub mod data_10;
#[doc = "DATA_11 (rw) register accessor: Data register 11\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data_11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data_11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data_11`] module"]
pub type DATA_11 = crate::Reg<data_11::DATA_11_SPEC>;
#[doc = "Data register 11"]
pub mod data_11;
#[doc = "DATA_12 (rw) register accessor: Data register 12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data_12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data_12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data_12`] module"]
pub type DATA_12 = crate::Reg<data_12::DATA_12_SPEC>;
#[doc = "Data register 12"]
pub mod data_12;
#[doc = "RX_MESSAGE_CNT (r) register accessor: Receive Message Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_message_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_message_cnt`] module"]
pub type RX_MESSAGE_CNT = crate::Reg<rx_message_cnt::RX_MESSAGE_CNT_SPEC>;
#[doc = "Receive Message Counter Register"]
pub mod rx_message_cnt;
#[doc = "CLOCK_DIVIDER (rw) register accessor: Clock Divider register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clock_divider::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clock_divider::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clock_divider`] module"]
pub type CLOCK_DIVIDER = crate::Reg<clock_divider::CLOCK_DIVIDER_SPEC>;
#[doc = "Clock Divider register"]
pub mod clock_divider;
