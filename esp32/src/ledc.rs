#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - "]
    pub hsch0_conf0: HSCH_CONF0,
    #[doc = "0x04 - "]
    pub hsch0_hpoint: HSCH_HPOINT,
    #[doc = "0x08 - "]
    pub hsch0_duty: HSCH_DUTY,
    #[doc = "0x0c - "]
    pub hsch0_conf1: HSCH_CONF1,
    #[doc = "0x10 - "]
    pub hsch0_duty_r: HSCH_DUTY_R,
    #[doc = "0x14 - "]
    pub hsch1_conf0: HSCH_CONF0,
    #[doc = "0x18 - "]
    pub hsch1_hpoint: HSCH_HPOINT,
    #[doc = "0x1c - "]
    pub hsch1_duty: HSCH_DUTY,
    #[doc = "0x20 - "]
    pub hsch1_conf1: HSCH_CONF1,
    #[doc = "0x24 - "]
    pub hsch1_duty_r: HSCH_DUTY_R,
    #[doc = "0x28 - "]
    pub hsch2_conf0: HSCH_CONF0,
    #[doc = "0x2c - "]
    pub hsch2_hpoint: HSCH_HPOINT,
    #[doc = "0x30 - "]
    pub hsch2_duty: HSCH_DUTY,
    #[doc = "0x34 - "]
    pub hsch2_conf1: HSCH_CONF1,
    #[doc = "0x38 - "]
    pub hsch2_duty_r: HSCH_DUTY_R,
    #[doc = "0x3c - "]
    pub hsch3_conf0: HSCH_CONF0,
    #[doc = "0x40 - "]
    pub hsch3_hpoint: HSCH_HPOINT,
    #[doc = "0x44 - "]
    pub hsch3_duty: HSCH_DUTY,
    #[doc = "0x48 - "]
    pub hsch3_conf1: HSCH_CONF1,
    #[doc = "0x4c - "]
    pub hsch3_duty_r: HSCH_DUTY_R,
    #[doc = "0x50 - "]
    pub hsch4_conf0: HSCH_CONF0,
    #[doc = "0x54 - "]
    pub hsch4_hpoint: HSCH_HPOINT,
    #[doc = "0x58 - "]
    pub hsch4_duty: HSCH_DUTY,
    #[doc = "0x5c - "]
    pub hsch4_conf1: HSCH_CONF1,
    #[doc = "0x60 - "]
    pub hsch4_duty_r: HSCH_DUTY_R,
    #[doc = "0x64 - "]
    pub hsch5_conf0: HSCH_CONF0,
    #[doc = "0x68 - "]
    pub hsch5_hpoint: HSCH_HPOINT,
    #[doc = "0x6c - "]
    pub hsch5_duty: HSCH_DUTY,
    #[doc = "0x70 - "]
    pub hsch5_conf1: HSCH_CONF1,
    #[doc = "0x74 - "]
    pub hsch5_duty_r: HSCH_DUTY_R,
    #[doc = "0x78 - "]
    pub hsch6_conf0: HSCH_CONF0,
    #[doc = "0x7c - "]
    pub hsch6_hpoint: HSCH_HPOINT,
    #[doc = "0x80 - "]
    pub hsch6_duty: HSCH_DUTY,
    #[doc = "0x84 - "]
    pub hsch6_conf1: HSCH_CONF1,
    #[doc = "0x88 - "]
    pub hsch6_duty_r: HSCH_DUTY_R,
    #[doc = "0x8c - "]
    pub hsch7_conf0: HSCH_CONF0,
    #[doc = "0x90 - "]
    pub hsch7_hpoint: HSCH_HPOINT,
    #[doc = "0x94 - "]
    pub hsch7_duty: HSCH_DUTY,
    #[doc = "0x98 - "]
    pub hsch7_conf1: HSCH_CONF1,
    #[doc = "0x9c - "]
    pub hsch7_duty_r: HSCH_DUTY_R,
    #[doc = "0xa0 - "]
    pub lsch0_conf0: LSCH_CONF0,
    #[doc = "0xa4 - "]
    pub lsch0_hpoint: LSCH_HPOINT,
    #[doc = "0xa8 - "]
    pub lsch0_duty: LSCH_DUTY,
    #[doc = "0xac - "]
    pub lsch0_conf1: LSCH_CONF1,
    #[doc = "0xb0 - "]
    pub lsch0_duty_r: LSCH_DUTY_R,
    #[doc = "0xb4 - "]
    pub lsch1_conf0: LSCH_CONF0,
    #[doc = "0xb8 - "]
    pub lsch1_hpoint: LSCH_HPOINT,
    #[doc = "0xbc - "]
    pub lsch1_duty: LSCH_DUTY,
    #[doc = "0xc0 - "]
    pub lsch1_conf1: LSCH_CONF1,
    #[doc = "0xc4 - "]
    pub lsch1_duty_r: LSCH_DUTY_R,
    #[doc = "0xc8 - "]
    pub lsch2_conf0: LSCH_CONF0,
    #[doc = "0xcc - "]
    pub lsch2_hpoint: LSCH_HPOINT,
    #[doc = "0xd0 - "]
    pub lsch2_duty: LSCH_DUTY,
    #[doc = "0xd4 - "]
    pub lsch2_conf1: LSCH_CONF1,
    #[doc = "0xd8 - "]
    pub lsch2_duty_r: LSCH_DUTY_R,
    #[doc = "0xdc - "]
    pub lsch3_conf0: LSCH_CONF0,
    #[doc = "0xe0 - "]
    pub lsch3_hpoint: LSCH_HPOINT,
    #[doc = "0xe4 - "]
    pub lsch3_duty: LSCH_DUTY,
    #[doc = "0xe8 - "]
    pub lsch3_conf1: LSCH_CONF1,
    #[doc = "0xec - "]
    pub lsch3_duty_r: LSCH_DUTY_R,
    #[doc = "0xf0 - "]
    pub lsch4_conf0: LSCH_CONF0,
    #[doc = "0xf4 - "]
    pub lsch4_hpoint: LSCH_HPOINT,
    #[doc = "0xf8 - "]
    pub lsch4_duty: LSCH_DUTY,
    #[doc = "0xfc - "]
    pub lsch4_conf1: LSCH_CONF1,
    #[doc = "0x100 - "]
    pub lsch4_duty_r: LSCH_DUTY_R,
    #[doc = "0x104 - "]
    pub lsch5_conf0: LSCH_CONF0,
    #[doc = "0x108 - "]
    pub lsch5_hpoint: LSCH_HPOINT,
    #[doc = "0x10c - "]
    pub lsch5_duty: LSCH_DUTY,
    #[doc = "0x110 - "]
    pub lsch5_conf1: LSCH_CONF1,
    #[doc = "0x114 - "]
    pub lsch5_duty_r: LSCH_DUTY_R,
    #[doc = "0x118 - "]
    pub lsch6_conf0: LSCH_CONF0,
    #[doc = "0x11c - "]
    pub lsch6_hpoint: LSCH_HPOINT,
    #[doc = "0x120 - "]
    pub lsch6_duty: LSCH_DUTY,
    #[doc = "0x124 - "]
    pub lsch6_conf1: LSCH_CONF1,
    #[doc = "0x128 - "]
    pub lsch6_duty_r: LSCH_DUTY_R,
    #[doc = "0x12c - "]
    pub lsch7_conf0: LSCH_CONF0,
    #[doc = "0x130 - "]
    pub lsch7_hpoint: LSCH_HPOINT,
    #[doc = "0x134 - "]
    pub lsch7_duty: LSCH_DUTY,
    #[doc = "0x138 - "]
    pub lsch7_conf1: LSCH_CONF1,
    #[doc = "0x13c - "]
    pub lsch7_duty_r: LSCH_DUTY_R,
    #[doc = "0x140 - "]
    pub hstimer0_conf: HSTIMER_CONF,
    #[doc = "0x144 - "]
    pub hstimer0_value: HSTIMER_VALUE,
    #[doc = "0x148 - "]
    pub hstimer1_conf: HSTIMER_CONF,
    #[doc = "0x14c - "]
    pub hstimer1_value: HSTIMER_VALUE,
    #[doc = "0x150 - "]
    pub hstimer2_conf: HSTIMER_CONF,
    #[doc = "0x154 - "]
    pub hstimer2_value: HSTIMER_VALUE,
    #[doc = "0x158 - "]
    pub hstimer3_conf: HSTIMER_CONF,
    #[doc = "0x15c - "]
    pub hstimer3_value: HSTIMER_VALUE,
    #[doc = "0x160 - "]
    pub lstimer0_conf: LSTIMER_CONF,
    #[doc = "0x164 - "]
    pub lstimer0_value: LSTIMER_VALUE,
    #[doc = "0x168 - "]
    pub lstimer1_conf: LSTIMER_CONF,
    #[doc = "0x16c - "]
    pub lstimer1_value: LSTIMER_VALUE,
    #[doc = "0x170 - "]
    pub lstimer2_conf: LSTIMER_CONF,
    #[doc = "0x174 - "]
    pub lstimer2_value: LSTIMER_VALUE,
    #[doc = "0x178 - "]
    pub lstimer3_conf: LSTIMER_CONF,
    #[doc = "0x17c - "]
    pub lstimer3_value: LSTIMER_VALUE,
    #[doc = "0x180 - "]
    pub int_raw: INT_RAW,
    #[doc = "0x184 - "]
    pub int_st: INT_ST,
    #[doc = "0x188 - "]
    pub int_ena: INT_ENA,
    #[doc = "0x18c - "]
    pub int_clr: INT_CLR,
    #[doc = "0x190 - "]
    pub conf: CONF,
    _reserved101: [u8; 0x68],
    #[doc = "0x1fc - "]
    pub date: DATE,
}
#[doc = "HSCH_CONF0 (rw) register accessor: an alias for `Reg<HSCH_CONF0_SPEC>`"]
pub type HSCH_CONF0 = crate::Reg<hsch_conf0::HSCH_CONF0_SPEC>;
#[doc = ""]
pub mod hsch_conf0;
#[doc = "HSCH_HPOINT (rw) register accessor: an alias for `Reg<HSCH_HPOINT_SPEC>`"]
pub type HSCH_HPOINT = crate::Reg<hsch_hpoint::HSCH_HPOINT_SPEC>;
#[doc = ""]
pub mod hsch_hpoint;
#[doc = "HSCH_DUTY (rw) register accessor: an alias for `Reg<HSCH_DUTY_SPEC>`"]
pub type HSCH_DUTY = crate::Reg<hsch_duty::HSCH_DUTY_SPEC>;
#[doc = ""]
pub mod hsch_duty;
#[doc = "HSCH_CONF1 (rw) register accessor: an alias for `Reg<HSCH_CONF1_SPEC>`"]
pub type HSCH_CONF1 = crate::Reg<hsch_conf1::HSCH_CONF1_SPEC>;
#[doc = ""]
pub mod hsch_conf1;
#[doc = "HSCH_DUTY_R (r) register accessor: an alias for `Reg<HSCH_DUTY_R_SPEC>`"]
pub type HSCH_DUTY_R = crate::Reg<hsch_duty_r::HSCH_DUTY_R_SPEC>;
#[doc = ""]
pub mod hsch_duty_r;
#[doc = "LSCH_CONF0 (rw) register accessor: an alias for `Reg<LSCH_CONF0_SPEC>`"]
pub type LSCH_CONF0 = crate::Reg<lsch_conf0::LSCH_CONF0_SPEC>;
#[doc = ""]
pub mod lsch_conf0;
#[doc = "LSCH_HPOINT (rw) register accessor: an alias for `Reg<LSCH_HPOINT_SPEC>`"]
pub type LSCH_HPOINT = crate::Reg<lsch_hpoint::LSCH_HPOINT_SPEC>;
#[doc = ""]
pub mod lsch_hpoint;
#[doc = "LSCH_DUTY (rw) register accessor: an alias for `Reg<LSCH_DUTY_SPEC>`"]
pub type LSCH_DUTY = crate::Reg<lsch_duty::LSCH_DUTY_SPEC>;
#[doc = ""]
pub mod lsch_duty;
#[doc = "LSCH_CONF1 (rw) register accessor: an alias for `Reg<LSCH_CONF1_SPEC>`"]
pub type LSCH_CONF1 = crate::Reg<lsch_conf1::LSCH_CONF1_SPEC>;
#[doc = ""]
pub mod lsch_conf1;
#[doc = "LSCH_DUTY_R (r) register accessor: an alias for `Reg<LSCH_DUTY_R_SPEC>`"]
pub type LSCH_DUTY_R = crate::Reg<lsch_duty_r::LSCH_DUTY_R_SPEC>;
#[doc = ""]
pub mod lsch_duty_r;
#[doc = "HSTIMER_CONF (rw) register accessor: an alias for `Reg<HSTIMER_CONF_SPEC>`"]
pub type HSTIMER_CONF = crate::Reg<hstimer_conf::HSTIMER_CONF_SPEC>;
#[doc = ""]
pub mod hstimer_conf;
#[doc = "HSTIMER_VALUE (r) register accessor: an alias for `Reg<HSTIMER_VALUE_SPEC>`"]
pub type HSTIMER_VALUE = crate::Reg<hstimer_value::HSTIMER_VALUE_SPEC>;
#[doc = ""]
pub mod hstimer_value;
#[doc = "LSTIMER_CONF (rw) register accessor: an alias for `Reg<LSTIMER_CONF_SPEC>`"]
pub type LSTIMER_CONF = crate::Reg<lstimer_conf::LSTIMER_CONF_SPEC>;
#[doc = ""]
pub mod lstimer_conf;
#[doc = "LSTIMER_VALUE (r) register accessor: an alias for `Reg<LSTIMER_VALUE_SPEC>`"]
pub type LSTIMER_VALUE = crate::Reg<lstimer_value::LSTIMER_VALUE_SPEC>;
#[doc = ""]
pub mod lstimer_value;
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
#[doc = "CONF (rw) register accessor: an alias for `Reg<CONF_SPEC>`"]
pub type CONF = crate::Reg<conf::CONF_SPEC>;
#[doc = ""]
pub mod conf;
#[doc = "DATE (rw) register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = ""]
pub mod date;
