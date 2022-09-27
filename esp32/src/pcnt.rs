#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - "]
    pub u0_conf0: U0_CONF0,
    #[doc = "0x04 - "]
    pub u0_conf1: U0_CONF1,
    #[doc = "0x08 - "]
    pub u0_conf2: U0_CONF2,
    #[doc = "0x0c - "]
    pub u1_conf0: U1_CONF0,
    #[doc = "0x10 - "]
    pub u1_conf1: U1_CONF1,
    #[doc = "0x14 - "]
    pub u1_conf2: U1_CONF2,
    #[doc = "0x18 - "]
    pub u2_conf0: U2_CONF0,
    #[doc = "0x1c - "]
    pub u2_conf1: U2_CONF1,
    #[doc = "0x20 - "]
    pub u2_conf2: U2_CONF2,
    #[doc = "0x24 - "]
    pub u3_conf0: U3_CONF0,
    #[doc = "0x28 - "]
    pub u3_conf1: U3_CONF1,
    #[doc = "0x2c - "]
    pub u3_conf2: U3_CONF2,
    #[doc = "0x30 - "]
    pub u4_conf0: U4_CONF0,
    #[doc = "0x34 - "]
    pub u4_conf1: U4_CONF1,
    #[doc = "0x38 - "]
    pub u4_conf2: U4_CONF2,
    #[doc = "0x3c - "]
    pub u5_conf0: U5_CONF0,
    #[doc = "0x40 - "]
    pub u5_conf1: U5_CONF1,
    #[doc = "0x44 - "]
    pub u5_conf2: U5_CONF2,
    #[doc = "0x48 - "]
    pub u6_conf0: U6_CONF0,
    #[doc = "0x4c - "]
    pub u6_conf1: U6_CONF1,
    #[doc = "0x50 - "]
    pub u6_conf2: U6_CONF2,
    #[doc = "0x54 - "]
    pub u7_conf0: U7_CONF0,
    #[doc = "0x58 - "]
    pub u7_conf1: U7_CONF1,
    #[doc = "0x5c - "]
    pub u7_conf2: U7_CONF2,
    #[doc = "0x60 - "]
    pub u0_cnt: U0_CNT,
    #[doc = "0x64 - "]
    pub u1_cnt: U1_CNT,
    #[doc = "0x68 - "]
    pub u2_cnt: U2_CNT,
    #[doc = "0x6c - "]
    pub u3_cnt: U3_CNT,
    #[doc = "0x70 - "]
    pub u4_cnt: U4_CNT,
    #[doc = "0x74 - "]
    pub u5_cnt: U5_CNT,
    #[doc = "0x78 - "]
    pub u6_cnt: U6_CNT,
    #[doc = "0x7c - "]
    pub u7_cnt: U7_CNT,
    #[doc = "0x80 - "]
    pub int_raw: INT_RAW,
    #[doc = "0x84 - "]
    pub int_st: INT_ST,
    #[doc = "0x88 - "]
    pub int_ena: INT_ENA,
    #[doc = "0x8c - "]
    pub int_clr: INT_CLR,
    #[doc = "0x90 - "]
    pub u0_status: U0_STATUS,
    #[doc = "0x94 - "]
    pub u1_status: U1_STATUS,
    #[doc = "0x98 - "]
    pub u2_status: U2_STATUS,
    #[doc = "0x9c - "]
    pub u3_status: U3_STATUS,
    #[doc = "0xa0 - "]
    pub u4_status: U4_STATUS,
    #[doc = "0xa4 - "]
    pub u5_status: U5_STATUS,
    #[doc = "0xa8 - "]
    pub u6_status: U6_STATUS,
    #[doc = "0xac - "]
    pub u7_status: U7_STATUS,
    #[doc = "0xb0 - "]
    pub ctrl: CTRL,
    _reserved45: [u8; 0x48],
    #[doc = "0xfc - "]
    pub date: DATE,
}
#[doc = "U0_CONF0 (rw) register accessor: an alias for `Reg<U0_CONF0_SPEC>`"]
pub type U0_CONF0 = crate::Reg<u0_conf0::U0_CONF0_SPEC>;
#[doc = ""]
pub mod u0_conf0;
#[doc = "U0_CONF1 (rw) register accessor: an alias for `Reg<U0_CONF1_SPEC>`"]
pub type U0_CONF1 = crate::Reg<u0_conf1::U0_CONF1_SPEC>;
#[doc = ""]
pub mod u0_conf1;
#[doc = "U0_CONF2 (rw) register accessor: an alias for `Reg<U0_CONF2_SPEC>`"]
pub type U0_CONF2 = crate::Reg<u0_conf2::U0_CONF2_SPEC>;
#[doc = ""]
pub mod u0_conf2;
#[doc = "U1_CONF0 (rw) register accessor: an alias for `Reg<U1_CONF0_SPEC>`"]
pub type U1_CONF0 = crate::Reg<u1_conf0::U1_CONF0_SPEC>;
#[doc = ""]
pub mod u1_conf0;
#[doc = "U1_CONF1 (rw) register accessor: an alias for `Reg<U1_CONF1_SPEC>`"]
pub type U1_CONF1 = crate::Reg<u1_conf1::U1_CONF1_SPEC>;
#[doc = ""]
pub mod u1_conf1;
#[doc = "U1_CONF2 (rw) register accessor: an alias for `Reg<U1_CONF2_SPEC>`"]
pub type U1_CONF2 = crate::Reg<u1_conf2::U1_CONF2_SPEC>;
#[doc = ""]
pub mod u1_conf2;
#[doc = "U2_CONF0 (rw) register accessor: an alias for `Reg<U2_CONF0_SPEC>`"]
pub type U2_CONF0 = crate::Reg<u2_conf0::U2_CONF0_SPEC>;
#[doc = ""]
pub mod u2_conf0;
#[doc = "U2_CONF1 (rw) register accessor: an alias for `Reg<U2_CONF1_SPEC>`"]
pub type U2_CONF1 = crate::Reg<u2_conf1::U2_CONF1_SPEC>;
#[doc = ""]
pub mod u2_conf1;
#[doc = "U2_CONF2 (rw) register accessor: an alias for `Reg<U2_CONF2_SPEC>`"]
pub type U2_CONF2 = crate::Reg<u2_conf2::U2_CONF2_SPEC>;
#[doc = ""]
pub mod u2_conf2;
#[doc = "U3_CONF0 (rw) register accessor: an alias for `Reg<U3_CONF0_SPEC>`"]
pub type U3_CONF0 = crate::Reg<u3_conf0::U3_CONF0_SPEC>;
#[doc = ""]
pub mod u3_conf0;
#[doc = "U3_CONF1 (rw) register accessor: an alias for `Reg<U3_CONF1_SPEC>`"]
pub type U3_CONF1 = crate::Reg<u3_conf1::U3_CONF1_SPEC>;
#[doc = ""]
pub mod u3_conf1;
#[doc = "U3_CONF2 (rw) register accessor: an alias for `Reg<U3_CONF2_SPEC>`"]
pub type U3_CONF2 = crate::Reg<u3_conf2::U3_CONF2_SPEC>;
#[doc = ""]
pub mod u3_conf2;
#[doc = "U4_CONF0 (rw) register accessor: an alias for `Reg<U4_CONF0_SPEC>`"]
pub type U4_CONF0 = crate::Reg<u4_conf0::U4_CONF0_SPEC>;
#[doc = ""]
pub mod u4_conf0;
#[doc = "U4_CONF1 (rw) register accessor: an alias for `Reg<U4_CONF1_SPEC>`"]
pub type U4_CONF1 = crate::Reg<u4_conf1::U4_CONF1_SPEC>;
#[doc = ""]
pub mod u4_conf1;
#[doc = "U4_CONF2 (rw) register accessor: an alias for `Reg<U4_CONF2_SPEC>`"]
pub type U4_CONF2 = crate::Reg<u4_conf2::U4_CONF2_SPEC>;
#[doc = ""]
pub mod u4_conf2;
#[doc = "U5_CONF0 (rw) register accessor: an alias for `Reg<U5_CONF0_SPEC>`"]
pub type U5_CONF0 = crate::Reg<u5_conf0::U5_CONF0_SPEC>;
#[doc = ""]
pub mod u5_conf0;
#[doc = "U5_CONF1 (rw) register accessor: an alias for `Reg<U5_CONF1_SPEC>`"]
pub type U5_CONF1 = crate::Reg<u5_conf1::U5_CONF1_SPEC>;
#[doc = ""]
pub mod u5_conf1;
#[doc = "U5_CONF2 (rw) register accessor: an alias for `Reg<U5_CONF2_SPEC>`"]
pub type U5_CONF2 = crate::Reg<u5_conf2::U5_CONF2_SPEC>;
#[doc = ""]
pub mod u5_conf2;
#[doc = "U6_CONF0 (rw) register accessor: an alias for `Reg<U6_CONF0_SPEC>`"]
pub type U6_CONF0 = crate::Reg<u6_conf0::U6_CONF0_SPEC>;
#[doc = ""]
pub mod u6_conf0;
#[doc = "U6_CONF1 (rw) register accessor: an alias for `Reg<U6_CONF1_SPEC>`"]
pub type U6_CONF1 = crate::Reg<u6_conf1::U6_CONF1_SPEC>;
#[doc = ""]
pub mod u6_conf1;
#[doc = "U6_CONF2 (rw) register accessor: an alias for `Reg<U6_CONF2_SPEC>`"]
pub type U6_CONF2 = crate::Reg<u6_conf2::U6_CONF2_SPEC>;
#[doc = ""]
pub mod u6_conf2;
#[doc = "U7_CONF0 (rw) register accessor: an alias for `Reg<U7_CONF0_SPEC>`"]
pub type U7_CONF0 = crate::Reg<u7_conf0::U7_CONF0_SPEC>;
#[doc = ""]
pub mod u7_conf0;
#[doc = "U7_CONF1 (rw) register accessor: an alias for `Reg<U7_CONF1_SPEC>`"]
pub type U7_CONF1 = crate::Reg<u7_conf1::U7_CONF1_SPEC>;
#[doc = ""]
pub mod u7_conf1;
#[doc = "U7_CONF2 (rw) register accessor: an alias for `Reg<U7_CONF2_SPEC>`"]
pub type U7_CONF2 = crate::Reg<u7_conf2::U7_CONF2_SPEC>;
#[doc = ""]
pub mod u7_conf2;
#[doc = "U0_CNT (r) register accessor: an alias for `Reg<U0_CNT_SPEC>`"]
pub type U0_CNT = crate::Reg<u0_cnt::U0_CNT_SPEC>;
#[doc = ""]
pub mod u0_cnt;
#[doc = "U1_CNT (r) register accessor: an alias for `Reg<U1_CNT_SPEC>`"]
pub type U1_CNT = crate::Reg<u1_cnt::U1_CNT_SPEC>;
#[doc = ""]
pub mod u1_cnt;
#[doc = "U2_CNT (r) register accessor: an alias for `Reg<U2_CNT_SPEC>`"]
pub type U2_CNT = crate::Reg<u2_cnt::U2_CNT_SPEC>;
#[doc = ""]
pub mod u2_cnt;
#[doc = "U3_CNT (r) register accessor: an alias for `Reg<U3_CNT_SPEC>`"]
pub type U3_CNT = crate::Reg<u3_cnt::U3_CNT_SPEC>;
#[doc = ""]
pub mod u3_cnt;
#[doc = "U4_CNT (r) register accessor: an alias for `Reg<U4_CNT_SPEC>`"]
pub type U4_CNT = crate::Reg<u4_cnt::U4_CNT_SPEC>;
#[doc = ""]
pub mod u4_cnt;
#[doc = "U5_CNT (r) register accessor: an alias for `Reg<U5_CNT_SPEC>`"]
pub type U5_CNT = crate::Reg<u5_cnt::U5_CNT_SPEC>;
#[doc = ""]
pub mod u5_cnt;
#[doc = "U6_CNT (r) register accessor: an alias for `Reg<U6_CNT_SPEC>`"]
pub type U6_CNT = crate::Reg<u6_cnt::U6_CNT_SPEC>;
#[doc = ""]
pub mod u6_cnt;
#[doc = "U7_CNT (r) register accessor: an alias for `Reg<U7_CNT_SPEC>`"]
pub type U7_CNT = crate::Reg<u7_cnt::U7_CNT_SPEC>;
#[doc = ""]
pub mod u7_cnt;
#[doc = "INT_RAW (r) register accessor: an alias for `Reg<INT_RAW_SPEC>`"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = ""]
pub mod int_raw;
#[doc = "INT_ST (r) register accessor: an alias for `Reg<INT_ST_SPEC>`"]
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
#[doc = ""]
pub mod int_st;
#[doc = "INT_ENA (rw) register accessor: an alias for `Reg<INT_ENA_SPEC>`"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = ""]
pub mod int_ena;
#[doc = "INT_CLR (w) register accessor: an alias for `Reg<INT_CLR_SPEC>`"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = ""]
pub mod int_clr;
#[doc = "U0_STATUS (rw) register accessor: an alias for `Reg<U0_STATUS_SPEC>`"]
pub type U0_STATUS = crate::Reg<u0_status::U0_STATUS_SPEC>;
#[doc = ""]
pub mod u0_status;
#[doc = "U1_STATUS (r) register accessor: an alias for `Reg<U1_STATUS_SPEC>`"]
pub type U1_STATUS = crate::Reg<u1_status::U1_STATUS_SPEC>;
#[doc = ""]
pub mod u1_status;
#[doc = "U2_STATUS (r) register accessor: an alias for `Reg<U2_STATUS_SPEC>`"]
pub type U2_STATUS = crate::Reg<u2_status::U2_STATUS_SPEC>;
#[doc = ""]
pub mod u2_status;
#[doc = "U3_STATUS (r) register accessor: an alias for `Reg<U3_STATUS_SPEC>`"]
pub type U3_STATUS = crate::Reg<u3_status::U3_STATUS_SPEC>;
#[doc = ""]
pub mod u3_status;
#[doc = "U4_STATUS (r) register accessor: an alias for `Reg<U4_STATUS_SPEC>`"]
pub type U4_STATUS = crate::Reg<u4_status::U4_STATUS_SPEC>;
#[doc = ""]
pub mod u4_status;
#[doc = "U5_STATUS (r) register accessor: an alias for `Reg<U5_STATUS_SPEC>`"]
pub type U5_STATUS = crate::Reg<u5_status::U5_STATUS_SPEC>;
#[doc = ""]
pub mod u5_status;
#[doc = "U6_STATUS (r) register accessor: an alias for `Reg<U6_STATUS_SPEC>`"]
pub type U6_STATUS = crate::Reg<u6_status::U6_STATUS_SPEC>;
#[doc = ""]
pub mod u6_status;
#[doc = "U7_STATUS (r) register accessor: an alias for `Reg<U7_STATUS_SPEC>`"]
pub type U7_STATUS = crate::Reg<u7_status::U7_STATUS_SPEC>;
#[doc = ""]
pub mod u7_status;
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = ""]
pub mod ctrl;
#[doc = "DATE (rw) register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = ""]
pub mod date;
