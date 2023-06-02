#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - Configure system timer clock"]
    pub conf: CONF,
    #[doc = "0x04 - system timer unit0 value update register"]
    pub unit0_op: UNIT0_OP,
    #[doc = "0x08 - system timer unit1 value update register"]
    pub unit1_op: UNIT1_OP,
    #[doc = "0x0c - system timer unit0 value high load register"]
    pub unit0_load_hi: UNIT0_LOAD_HI,
    #[doc = "0x10 - system timer unit0 value low load register"]
    pub unit0_load_lo: UNIT0_LOAD_LO,
    #[doc = "0x14 - system timer unit1 value high load register"]
    pub unit1_load_hi: UNIT1_LOAD_HI,
    #[doc = "0x18 - system timer unit1 value low load register"]
    pub unit1_load_lo: UNIT1_LOAD_LO,
    #[doc = "0x1c - system timer comp0 value high register"]
    pub target0_hi: TARGET0_HI,
    #[doc = "0x20 - system timer comp0 value low register"]
    pub target0_lo: TARGET0_LO,
    #[doc = "0x24 - system timer comp1 value high register"]
    pub target1_hi: TARGET1_HI,
    #[doc = "0x28 - system timer comp1 value low register"]
    pub target1_lo: TARGET1_LO,
    #[doc = "0x2c - system timer comp2 value high register"]
    pub target2_hi: TARGET2_HI,
    #[doc = "0x30 - system timer comp2 value low register"]
    pub target2_lo: TARGET2_LO,
    #[doc = "0x34 - system timer comp0 target mode register"]
    pub target0_conf: TARGET0_CONF,
    #[doc = "0x38 - system timer comp1 target mode register"]
    pub target1_conf: TARGET1_CONF,
    #[doc = "0x3c - system timer comp2 target mode register"]
    pub target2_conf: TARGET2_CONF,
    #[doc = "0x40 - system timer unit0 value high register"]
    pub unit0_value_hi: UNIT0_VALUE_HI,
    #[doc = "0x44 - system timer unit0 value low register"]
    pub unit0_value_lo: UNIT0_VALUE_LO,
    #[doc = "0x48 - system timer unit1 value high register"]
    pub unit1_value_hi: UNIT1_VALUE_HI,
    #[doc = "0x4c - system timer unit1 value low register"]
    pub unit1_value_lo: UNIT1_VALUE_LO,
    #[doc = "0x50 - system timer comp0 conf sync register"]
    pub comp0_load: COMP0_LOAD,
    #[doc = "0x54 - system timer comp1 conf sync register"]
    pub comp1_load: COMP1_LOAD,
    #[doc = "0x58 - system timer comp2 conf sync register"]
    pub comp2_load: COMP2_LOAD,
    #[doc = "0x5c - system timer unit0 conf sync register"]
    pub unit0_load: UNIT0_LOAD,
    #[doc = "0x60 - system timer unit1 conf sync register"]
    pub unit1_load: UNIT1_LOAD,
    #[doc = "0x64 - systimer interrupt enable register"]
    pub int_ena: INT_ENA,
    #[doc = "0x68 - systimer interrupt raw register"]
    pub int_raw: INT_RAW,
    #[doc = "0x6c - systimer interrupt clear register"]
    pub int_clr: INT_CLR,
    #[doc = "0x70 - systimer interrupt status register"]
    pub int_st: INT_ST,
    #[doc = "0x74 - system timer comp0 actual target value low register"]
    pub real_target0_lo: REAL_TARGET0_LO,
    #[doc = "0x78 - system timer comp0 actual target value high register"]
    pub real_target0_hi: REAL_TARGET0_HI,
    #[doc = "0x7c - system timer comp1 actual target value low register"]
    pub real_target1_lo: REAL_TARGET1_LO,
    #[doc = "0x80 - system timer comp1 actual target value high register"]
    pub real_target1_hi: REAL_TARGET1_HI,
    #[doc = "0x84 - system timer comp2 actual target value low register"]
    pub real_target2_lo: REAL_TARGET2_LO,
    #[doc = "0x88 - system timer comp2 actual target value high register"]
    pub real_target2_hi: REAL_TARGET2_HI,
    _reserved35: [u8; 0x70],
    #[doc = "0xfc - system timer version control register"]
    pub date: DATE,
}
#[doc = "CONF (rw) register accessor: an alias for `Reg<CONF_SPEC>`"]
pub type CONF = crate::Reg<conf::CONF_SPEC>;
#[doc = "Configure system timer clock"]
pub mod conf;
#[doc = "UNIT0_OP (rw) register accessor: an alias for `Reg<UNIT0_OP_SPEC>`"]
pub type UNIT0_OP = crate::Reg<unit0_op::UNIT0_OP_SPEC>;
#[doc = "system timer unit0 value update register"]
pub mod unit0_op;
#[doc = "UNIT1_OP (rw) register accessor: an alias for `Reg<UNIT1_OP_SPEC>`"]
pub type UNIT1_OP = crate::Reg<unit1_op::UNIT1_OP_SPEC>;
#[doc = "system timer unit1 value update register"]
pub mod unit1_op;
#[doc = "UNIT0_LOAD_HI (rw) register accessor: an alias for `Reg<UNIT0_LOAD_HI_SPEC>`"]
pub type UNIT0_LOAD_HI = crate::Reg<unit0_load_hi::UNIT0_LOAD_HI_SPEC>;
#[doc = "system timer unit0 value high load register"]
pub mod unit0_load_hi;
#[doc = "UNIT0_LOAD_LO (rw) register accessor: an alias for `Reg<UNIT0_LOAD_LO_SPEC>`"]
pub type UNIT0_LOAD_LO = crate::Reg<unit0_load_lo::UNIT0_LOAD_LO_SPEC>;
#[doc = "system timer unit0 value low load register"]
pub mod unit0_load_lo;
#[doc = "UNIT1_LOAD_HI (rw) register accessor: an alias for `Reg<UNIT1_LOAD_HI_SPEC>`"]
pub type UNIT1_LOAD_HI = crate::Reg<unit1_load_hi::UNIT1_LOAD_HI_SPEC>;
#[doc = "system timer unit1 value high load register"]
pub mod unit1_load_hi;
#[doc = "UNIT1_LOAD_LO (rw) register accessor: an alias for `Reg<UNIT1_LOAD_LO_SPEC>`"]
pub type UNIT1_LOAD_LO = crate::Reg<unit1_load_lo::UNIT1_LOAD_LO_SPEC>;
#[doc = "system timer unit1 value low load register"]
pub mod unit1_load_lo;
#[doc = "TARGET0_HI (rw) register accessor: an alias for `Reg<TARGET0_HI_SPEC>`"]
pub type TARGET0_HI = crate::Reg<target0_hi::TARGET0_HI_SPEC>;
#[doc = "system timer comp0 value high register"]
pub mod target0_hi;
#[doc = "TARGET0_LO (rw) register accessor: an alias for `Reg<TARGET0_LO_SPEC>`"]
pub type TARGET0_LO = crate::Reg<target0_lo::TARGET0_LO_SPEC>;
#[doc = "system timer comp0 value low register"]
pub mod target0_lo;
#[doc = "TARGET1_HI (rw) register accessor: an alias for `Reg<TARGET1_HI_SPEC>`"]
pub type TARGET1_HI = crate::Reg<target1_hi::TARGET1_HI_SPEC>;
#[doc = "system timer comp1 value high register"]
pub mod target1_hi;
#[doc = "TARGET1_LO (rw) register accessor: an alias for `Reg<TARGET1_LO_SPEC>`"]
pub type TARGET1_LO = crate::Reg<target1_lo::TARGET1_LO_SPEC>;
#[doc = "system timer comp1 value low register"]
pub mod target1_lo;
#[doc = "TARGET2_HI (rw) register accessor: an alias for `Reg<TARGET2_HI_SPEC>`"]
pub type TARGET2_HI = crate::Reg<target2_hi::TARGET2_HI_SPEC>;
#[doc = "system timer comp2 value high register"]
pub mod target2_hi;
#[doc = "TARGET2_LO (rw) register accessor: an alias for `Reg<TARGET2_LO_SPEC>`"]
pub type TARGET2_LO = crate::Reg<target2_lo::TARGET2_LO_SPEC>;
#[doc = "system timer comp2 value low register"]
pub mod target2_lo;
#[doc = "TARGET0_CONF (rw) register accessor: an alias for `Reg<TARGET0_CONF_SPEC>`"]
pub type TARGET0_CONF = crate::Reg<target0_conf::TARGET0_CONF_SPEC>;
#[doc = "system timer comp0 target mode register"]
pub mod target0_conf;
#[doc = "TARGET1_CONF (rw) register accessor: an alias for `Reg<TARGET1_CONF_SPEC>`"]
pub type TARGET1_CONF = crate::Reg<target1_conf::TARGET1_CONF_SPEC>;
#[doc = "system timer comp1 target mode register"]
pub mod target1_conf;
#[doc = "TARGET2_CONF (rw) register accessor: an alias for `Reg<TARGET2_CONF_SPEC>`"]
pub type TARGET2_CONF = crate::Reg<target2_conf::TARGET2_CONF_SPEC>;
#[doc = "system timer comp2 target mode register"]
pub mod target2_conf;
#[doc = "UNIT0_VALUE_HI (r) register accessor: an alias for `Reg<UNIT0_VALUE_HI_SPEC>`"]
pub type UNIT0_VALUE_HI = crate::Reg<unit0_value_hi::UNIT0_VALUE_HI_SPEC>;
#[doc = "system timer unit0 value high register"]
pub mod unit0_value_hi;
#[doc = "UNIT0_VALUE_LO (r) register accessor: an alias for `Reg<UNIT0_VALUE_LO_SPEC>`"]
pub type UNIT0_VALUE_LO = crate::Reg<unit0_value_lo::UNIT0_VALUE_LO_SPEC>;
#[doc = "system timer unit0 value low register"]
pub mod unit0_value_lo;
#[doc = "UNIT1_VALUE_HI (r) register accessor: an alias for `Reg<UNIT1_VALUE_HI_SPEC>`"]
pub type UNIT1_VALUE_HI = crate::Reg<unit1_value_hi::UNIT1_VALUE_HI_SPEC>;
#[doc = "system timer unit1 value high register"]
pub mod unit1_value_hi;
#[doc = "UNIT1_VALUE_LO (r) register accessor: an alias for `Reg<UNIT1_VALUE_LO_SPEC>`"]
pub type UNIT1_VALUE_LO = crate::Reg<unit1_value_lo::UNIT1_VALUE_LO_SPEC>;
#[doc = "system timer unit1 value low register"]
pub mod unit1_value_lo;
#[doc = "COMP0_LOAD (w) register accessor: an alias for `Reg<COMP0_LOAD_SPEC>`"]
pub type COMP0_LOAD = crate::Reg<comp0_load::COMP0_LOAD_SPEC>;
#[doc = "system timer comp0 conf sync register"]
pub mod comp0_load;
#[doc = "COMP1_LOAD (w) register accessor: an alias for `Reg<COMP1_LOAD_SPEC>`"]
pub type COMP1_LOAD = crate::Reg<comp1_load::COMP1_LOAD_SPEC>;
#[doc = "system timer comp1 conf sync register"]
pub mod comp1_load;
#[doc = "COMP2_LOAD (w) register accessor: an alias for `Reg<COMP2_LOAD_SPEC>`"]
pub type COMP2_LOAD = crate::Reg<comp2_load::COMP2_LOAD_SPEC>;
#[doc = "system timer comp2 conf sync register"]
pub mod comp2_load;
#[doc = "UNIT0_LOAD (w) register accessor: an alias for `Reg<UNIT0_LOAD_SPEC>`"]
pub type UNIT0_LOAD = crate::Reg<unit0_load::UNIT0_LOAD_SPEC>;
#[doc = "system timer unit0 conf sync register"]
pub mod unit0_load;
#[doc = "UNIT1_LOAD (w) register accessor: an alias for `Reg<UNIT1_LOAD_SPEC>`"]
pub type UNIT1_LOAD = crate::Reg<unit1_load::UNIT1_LOAD_SPEC>;
#[doc = "system timer unit1 conf sync register"]
pub mod unit1_load;
#[doc = "INT_ENA (rw) register accessor: an alias for `Reg<INT_ENA_SPEC>`"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = "systimer interrupt enable register"]
pub mod int_ena;
#[doc = "INT_RAW (rw) register accessor: an alias for `Reg<INT_RAW_SPEC>`"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = "systimer interrupt raw register"]
pub mod int_raw;
#[doc = "INT_CLR (w) register accessor: an alias for `Reg<INT_CLR_SPEC>`"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = "systimer interrupt clear register"]
pub mod int_clr;
#[doc = "INT_ST (r) register accessor: an alias for `Reg<INT_ST_SPEC>`"]
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
#[doc = "systimer interrupt status register"]
pub mod int_st;
#[doc = "REAL_TARGET0_LO (r) register accessor: an alias for `Reg<REAL_TARGET0_LO_SPEC>`"]
pub type REAL_TARGET0_LO = crate::Reg<real_target0_lo::REAL_TARGET0_LO_SPEC>;
#[doc = "system timer comp0 actual target value low register"]
pub mod real_target0_lo;
#[doc = "REAL_TARGET0_HI (r) register accessor: an alias for `Reg<REAL_TARGET0_HI_SPEC>`"]
pub type REAL_TARGET0_HI = crate::Reg<real_target0_hi::REAL_TARGET0_HI_SPEC>;
#[doc = "system timer comp0 actual target value high register"]
pub mod real_target0_hi;
#[doc = "REAL_TARGET1_LO (r) register accessor: an alias for `Reg<REAL_TARGET1_LO_SPEC>`"]
pub type REAL_TARGET1_LO = crate::Reg<real_target1_lo::REAL_TARGET1_LO_SPEC>;
#[doc = "system timer comp1 actual target value low register"]
pub mod real_target1_lo;
#[doc = "REAL_TARGET1_HI (r) register accessor: an alias for `Reg<REAL_TARGET1_HI_SPEC>`"]
pub type REAL_TARGET1_HI = crate::Reg<real_target1_hi::REAL_TARGET1_HI_SPEC>;
#[doc = "system timer comp1 actual target value high register"]
pub mod real_target1_hi;
#[doc = "REAL_TARGET2_LO (r) register accessor: an alias for `Reg<REAL_TARGET2_LO_SPEC>`"]
pub type REAL_TARGET2_LO = crate::Reg<real_target2_lo::REAL_TARGET2_LO_SPEC>;
#[doc = "system timer comp2 actual target value low register"]
pub mod real_target2_lo;
#[doc = "REAL_TARGET2_HI (r) register accessor: an alias for `Reg<REAL_TARGET2_HI_SPEC>`"]
pub type REAL_TARGET2_HI = crate::Reg<real_target2_hi::REAL_TARGET2_HI_SPEC>;
#[doc = "system timer comp2 actual target value high register"]
pub mod real_target2_hi;
#[doc = "DATE (rw) register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "system timer version control register"]
pub mod date;
