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
#[doc = "MODE (rw) register accessor: an alias for `Reg<MODE_SPEC>`"]
pub type MODE = crate::Reg<mode::MODE_SPEC>;
#[doc = "Mode Register"]
pub mod mode;
#[doc = "CMD (w) register accessor: an alias for `Reg<CMD_SPEC>`"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "Command Register"]
pub mod cmd;
#[doc = "STATUS (r) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status register"]
pub mod status;
#[doc = "INT_RAW (r) register accessor: an alias for `Reg<INT_RAW_SPEC>`"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = "Interrupt Register"]
pub mod int_raw;
#[doc = "INT_ENA (rw) register accessor: an alias for `Reg<INT_ENA_SPEC>`"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod int_ena;
#[doc = "BUS_TIMING_0 (rw) register accessor: an alias for `Reg<BUS_TIMING_0_SPEC>`"]
pub type BUS_TIMING_0 = crate::Reg<bus_timing_0::BUS_TIMING_0_SPEC>;
#[doc = "Bus Timing Register 0"]
pub mod bus_timing_0;
#[doc = "BUS_TIMING_1 (rw) register accessor: an alias for `Reg<BUS_TIMING_1_SPEC>`"]
pub type BUS_TIMING_1 = crate::Reg<bus_timing_1::BUS_TIMING_1_SPEC>;
#[doc = "Bus Timing Register 1"]
pub mod bus_timing_1;
#[doc = "ARB_LOST_CAP (r) register accessor: an alias for `Reg<ARB_LOST_CAP_SPEC>`"]
pub type ARB_LOST_CAP = crate::Reg<arb_lost_cap::ARB_LOST_CAP_SPEC>;
#[doc = "Arbitration Lost Capture Register"]
pub mod arb_lost_cap;
#[doc = "ERR_CODE_CAP (r) register accessor: an alias for `Reg<ERR_CODE_CAP_SPEC>`"]
pub type ERR_CODE_CAP = crate::Reg<err_code_cap::ERR_CODE_CAP_SPEC>;
#[doc = "Error Code Capture Register"]
pub mod err_code_cap;
#[doc = "ERR_WARNING_LIMIT (rw) register accessor: an alias for `Reg<ERR_WARNING_LIMIT_SPEC>`"]
pub type ERR_WARNING_LIMIT = crate::Reg<err_warning_limit::ERR_WARNING_LIMIT_SPEC>;
#[doc = "Error Warning Limit Register"]
pub mod err_warning_limit;
#[doc = "RX_ERR_CNT (rw) register accessor: an alias for `Reg<RX_ERR_CNT_SPEC>`"]
pub type RX_ERR_CNT = crate::Reg<rx_err_cnt::RX_ERR_CNT_SPEC>;
#[doc = "Receive Error Counter Register"]
pub mod rx_err_cnt;
#[doc = "TX_ERR_CNT (rw) register accessor: an alias for `Reg<TX_ERR_CNT_SPEC>`"]
pub type TX_ERR_CNT = crate::Reg<tx_err_cnt::TX_ERR_CNT_SPEC>;
#[doc = "Transmit Error Counter Register"]
pub mod tx_err_cnt;
#[doc = "DATA_0 (rw) register accessor: an alias for `Reg<DATA_0_SPEC>`"]
pub type DATA_0 = crate::Reg<data_0::DATA_0_SPEC>;
#[doc = "Data register 0"]
pub mod data_0;
#[doc = "DATA_1 (rw) register accessor: an alias for `Reg<DATA_1_SPEC>`"]
pub type DATA_1 = crate::Reg<data_1::DATA_1_SPEC>;
#[doc = "Data register 1"]
pub mod data_1;
#[doc = "DATA_2 (rw) register accessor: an alias for `Reg<DATA_2_SPEC>`"]
pub type DATA_2 = crate::Reg<data_2::DATA_2_SPEC>;
#[doc = "Data register 2"]
pub mod data_2;
#[doc = "DATA_3 (rw) register accessor: an alias for `Reg<DATA_3_SPEC>`"]
pub type DATA_3 = crate::Reg<data_3::DATA_3_SPEC>;
#[doc = "Data register 3"]
pub mod data_3;
#[doc = "DATA_4 (rw) register accessor: an alias for `Reg<DATA_4_SPEC>`"]
pub type DATA_4 = crate::Reg<data_4::DATA_4_SPEC>;
#[doc = "Data register 4"]
pub mod data_4;
#[doc = "DATA_5 (rw) register accessor: an alias for `Reg<DATA_5_SPEC>`"]
pub type DATA_5 = crate::Reg<data_5::DATA_5_SPEC>;
#[doc = "Data register 5"]
pub mod data_5;
#[doc = "DATA_6 (rw) register accessor: an alias for `Reg<DATA_6_SPEC>`"]
pub type DATA_6 = crate::Reg<data_6::DATA_6_SPEC>;
#[doc = "Data register 6"]
pub mod data_6;
#[doc = "DATA_7 (rw) register accessor: an alias for `Reg<DATA_7_SPEC>`"]
pub type DATA_7 = crate::Reg<data_7::DATA_7_SPEC>;
#[doc = "Data register 7"]
pub mod data_7;
#[doc = "DATA_8 (rw) register accessor: an alias for `Reg<DATA_8_SPEC>`"]
pub type DATA_8 = crate::Reg<data_8::DATA_8_SPEC>;
#[doc = "Data register 8"]
pub mod data_8;
#[doc = "DATA_9 (rw) register accessor: an alias for `Reg<DATA_9_SPEC>`"]
pub type DATA_9 = crate::Reg<data_9::DATA_9_SPEC>;
#[doc = "Data register 9"]
pub mod data_9;
#[doc = "DATA_10 (rw) register accessor: an alias for `Reg<DATA_10_SPEC>`"]
pub type DATA_10 = crate::Reg<data_10::DATA_10_SPEC>;
#[doc = "Data register 10"]
pub mod data_10;
#[doc = "DATA_11 (rw) register accessor: an alias for `Reg<DATA_11_SPEC>`"]
pub type DATA_11 = crate::Reg<data_11::DATA_11_SPEC>;
#[doc = "Data register 11"]
pub mod data_11;
#[doc = "DATA_12 (rw) register accessor: an alias for `Reg<DATA_12_SPEC>`"]
pub type DATA_12 = crate::Reg<data_12::DATA_12_SPEC>;
#[doc = "Data register 12"]
pub mod data_12;
#[doc = "RX_MESSAGE_CNT (r) register accessor: an alias for `Reg<RX_MESSAGE_CNT_SPEC>`"]
pub type RX_MESSAGE_CNT = crate::Reg<rx_message_cnt::RX_MESSAGE_CNT_SPEC>;
#[doc = "Receive Message Counter Register"]
pub mod rx_message_cnt;
#[doc = "CLOCK_DIVIDER (rw) register accessor: an alias for `Reg<CLOCK_DIVIDER_SPEC>`"]
pub type CLOCK_DIVIDER = crate::Reg<clock_divider::CLOCK_DIVIDER_SPEC>;
#[doc = "Clock Divider register"]
pub mod clock_divider;
