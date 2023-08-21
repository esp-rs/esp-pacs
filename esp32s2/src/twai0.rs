#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - Mode Register"]
    pub mode: MODE,
    #[doc = "0x04 - Command Register"]
    pub cmd: CMD,
    #[doc = "0x08 - Status register"]
    pub status: STATUS,
    #[doc = "0x0c - Interrupt Register"]
    pub int_raw: INT_RAW,
    #[doc = "0x10 - Interrupt Enable Register"]
    pub int_ena: INT_ENA,
    _reserved5: [u8; 0x04],
    #[doc = "0x18 - Bus Timing Register 0"]
    pub bus_timing_0: BUS_TIMING_0,
    #[doc = "0x1c - Bus Timing Register 1"]
    pub bus_timing_1: BUS_TIMING_1,
    _reserved7: [u8; 0x0c],
    #[doc = "0x2c - Arbitration Lost Capture Register"]
    pub arb_lost_cap: ARB_LOST_CAP,
    #[doc = "0x30 - Error Code Capture Register"]
    pub err_code_cap: ERR_CODE_CAP,
    #[doc = "0x34 - Error Warning Limit Register"]
    pub err_warning_limit: ERR_WARNING_LIMIT,
    #[doc = "0x38 - Receive Error Counter Register"]
    pub rx_err_cnt: RX_ERR_CNT,
    #[doc = "0x3c - Transmit Error Counter Register"]
    pub tx_err_cnt: TX_ERR_CNT,
    #[doc = "0x40 - Data register 0"]
    pub data_0: DATA_0,
    #[doc = "0x44 - Data register 1"]
    pub data_1: DATA_1,
    #[doc = "0x48 - Data register 2"]
    pub data_2: DATA_2,
    #[doc = "0x4c - Data register 3"]
    pub data_3: DATA_3,
    #[doc = "0x50 - Data register 4"]
    pub data_4: DATA_4,
    #[doc = "0x54 - Data register 5"]
    pub data_5: DATA_5,
    #[doc = "0x58 - Data register 6"]
    pub data_6: DATA_6,
    #[doc = "0x5c - Data register 7"]
    pub data_7: DATA_7,
    #[doc = "0x60 - Data register 8"]
    pub data_8: DATA_8,
    #[doc = "0x64 - Data register 9"]
    pub data_9: DATA_9,
    #[doc = "0x68 - Data register 10"]
    pub data_10: DATA_10,
    #[doc = "0x6c - Data register 11"]
    pub data_11: DATA_11,
    #[doc = "0x70 - Data register 12"]
    pub data_12: DATA_12,
    #[doc = "0x74 - Receive Message Counter Register"]
    pub rx_message_cnt: RX_MESSAGE_CNT,
    _reserved26: [u8; 0x04],
    #[doc = "0x7c - Clock Divider register"]
    pub clock_divider: CLOCK_DIVIDER,
}
#[doc = "MODE (rw) register accessor: Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mode`] module"]
pub type MODE = crate::Reg<mode::MODE_SPEC>;
#[doc = "Mode Register"]
pub mod mode;
#[doc = "CMD (w) register accessor: Command Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cmd`] module"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "Command Register"]
pub mod cmd;
#[doc = "STATUS (r) register accessor: Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`status`] module"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status register"]
pub mod status;
#[doc = "INT_RAW (r) register accessor: Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`int_raw`] module"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = "Interrupt Register"]
pub mod int_raw;
#[doc = "INT_ENA (rw) register accessor: Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`int_ena`] module"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod int_ena;
#[doc = "BUS_TIMING_0 (rw) register accessor: Bus Timing Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bus_timing_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bus_timing_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`bus_timing_0`] module"]
pub type BUS_TIMING_0 = crate::Reg<bus_timing_0::BUS_TIMING_0_SPEC>;
#[doc = "Bus Timing Register 0"]
pub mod bus_timing_0;
#[doc = "BUS_TIMING_1 (rw) register accessor: Bus Timing Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bus_timing_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bus_timing_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`bus_timing_1`] module"]
pub type BUS_TIMING_1 = crate::Reg<bus_timing_1::BUS_TIMING_1_SPEC>;
#[doc = "Bus Timing Register 1"]
pub mod bus_timing_1;
#[doc = "ARB_LOST_CAP (r) register accessor: Arbitration Lost Capture Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arb_lost_cap::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`arb_lost_cap`] module"]
pub type ARB_LOST_CAP = crate::Reg<arb_lost_cap::ARB_LOST_CAP_SPEC>;
#[doc = "Arbitration Lost Capture Register"]
pub mod arb_lost_cap;
#[doc = "ERR_CODE_CAP (r) register accessor: Error Code Capture Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`err_code_cap::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`err_code_cap`] module"]
pub type ERR_CODE_CAP = crate::Reg<err_code_cap::ERR_CODE_CAP_SPEC>;
#[doc = "Error Code Capture Register"]
pub mod err_code_cap;
#[doc = "ERR_WARNING_LIMIT (rw) register accessor: Error Warning Limit Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`err_warning_limit::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`err_warning_limit::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`err_warning_limit`] module"]
pub type ERR_WARNING_LIMIT = crate::Reg<err_warning_limit::ERR_WARNING_LIMIT_SPEC>;
#[doc = "Error Warning Limit Register"]
pub mod err_warning_limit;
#[doc = "RX_ERR_CNT (rw) register accessor: Receive Error Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_err_cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_err_cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rx_err_cnt`] module"]
pub type RX_ERR_CNT = crate::Reg<rx_err_cnt::RX_ERR_CNT_SPEC>;
#[doc = "Receive Error Counter Register"]
pub mod rx_err_cnt;
#[doc = "TX_ERR_CNT (rw) register accessor: Transmit Error Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_err_cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_err_cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`tx_err_cnt`] module"]
pub type TX_ERR_CNT = crate::Reg<tx_err_cnt::TX_ERR_CNT_SPEC>;
#[doc = "Transmit Error Counter Register"]
pub mod tx_err_cnt;
#[doc = "DATA_0 (rw) register accessor: Data register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`data_0`] module"]
pub type DATA_0 = crate::Reg<data_0::DATA_0_SPEC>;
#[doc = "Data register 0"]
pub mod data_0;
#[doc = "DATA_1 (rw) register accessor: Data register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`data_1`] module"]
pub type DATA_1 = crate::Reg<data_1::DATA_1_SPEC>;
#[doc = "Data register 1"]
pub mod data_1;
#[doc = "DATA_2 (rw) register accessor: Data register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`data_2`] module"]
pub type DATA_2 = crate::Reg<data_2::DATA_2_SPEC>;
#[doc = "Data register 2"]
pub mod data_2;
#[doc = "DATA_3 (rw) register accessor: Data register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`data_3`] module"]
pub type DATA_3 = crate::Reg<data_3::DATA_3_SPEC>;
#[doc = "Data register 3"]
pub mod data_3;
#[doc = "DATA_4 (rw) register accessor: Data register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data_4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data_4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`data_4`] module"]
pub type DATA_4 = crate::Reg<data_4::DATA_4_SPEC>;
#[doc = "Data register 4"]
pub mod data_4;
#[doc = "DATA_5 (rw) register accessor: Data register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data_5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data_5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`data_5`] module"]
pub type DATA_5 = crate::Reg<data_5::DATA_5_SPEC>;
#[doc = "Data register 5"]
pub mod data_5;
#[doc = "DATA_6 (rw) register accessor: Data register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data_6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data_6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`data_6`] module"]
pub type DATA_6 = crate::Reg<data_6::DATA_6_SPEC>;
#[doc = "Data register 6"]
pub mod data_6;
#[doc = "DATA_7 (rw) register accessor: Data register 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data_7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data_7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`data_7`] module"]
pub type DATA_7 = crate::Reg<data_7::DATA_7_SPEC>;
#[doc = "Data register 7"]
pub mod data_7;
#[doc = "DATA_8 (rw) register accessor: Data register 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data_8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data_8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`data_8`] module"]
pub type DATA_8 = crate::Reg<data_8::DATA_8_SPEC>;
#[doc = "Data register 8"]
pub mod data_8;
#[doc = "DATA_9 (rw) register accessor: Data register 9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data_9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data_9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`data_9`] module"]
pub type DATA_9 = crate::Reg<data_9::DATA_9_SPEC>;
#[doc = "Data register 9"]
pub mod data_9;
#[doc = "DATA_10 (rw) register accessor: Data register 10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data_10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data_10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`data_10`] module"]
pub type DATA_10 = crate::Reg<data_10::DATA_10_SPEC>;
#[doc = "Data register 10"]
pub mod data_10;
#[doc = "DATA_11 (rw) register accessor: Data register 11\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data_11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data_11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`data_11`] module"]
pub type DATA_11 = crate::Reg<data_11::DATA_11_SPEC>;
#[doc = "Data register 11"]
pub mod data_11;
#[doc = "DATA_12 (rw) register accessor: Data register 12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data_12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data_12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`data_12`] module"]
pub type DATA_12 = crate::Reg<data_12::DATA_12_SPEC>;
#[doc = "Data register 12"]
pub mod data_12;
#[doc = "RX_MESSAGE_CNT (r) register accessor: Receive Message Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_message_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rx_message_cnt`] module"]
pub type RX_MESSAGE_CNT = crate::Reg<rx_message_cnt::RX_MESSAGE_CNT_SPEC>;
#[doc = "Receive Message Counter Register"]
pub mod rx_message_cnt;
#[doc = "CLOCK_DIVIDER (rw) register accessor: Clock Divider register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clock_divider::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clock_divider::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`clock_divider`] module"]
pub type CLOCK_DIVIDER = crate::Reg<clock_divider::CLOCK_DIVIDER_SPEC>;
#[doc = "Clock Divider register"]
pub mod clock_divider;
