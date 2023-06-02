#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - SYSTIMER_CONF."]
    pub conf: CONF,
    #[doc = "0x04 - SYSTIMER_UNIT0_OP."]
    pub unit0_op: UNIT0_OP,
    #[doc = "0x08 - SYSTIMER_UNIT1_OP."]
    pub unit1_op: UNIT1_OP,
    #[doc = "0x0c - SYSTIMER_UNIT0_LOAD_HI."]
    pub unit0_load_hi: UNIT0_LOAD_HI,
    #[doc = "0x10 - SYSTIMER_UNIT0_LOAD_LO."]
    pub unit0_load_lo: UNIT0_LOAD_LO,
    #[doc = "0x14 - SYSTIMER_UNIT1_LOAD_HI."]
    pub unit1_load_hi: UNIT1_LOAD_HI,
    #[doc = "0x18 - SYSTIMER_UNIT1_LOAD_LO."]
    pub unit1_load_lo: UNIT1_LOAD_LO,
    #[doc = "0x1c - SYSTIMER_TARGET0_HI."]
    pub target0_hi: TARGET0_HI,
    #[doc = "0x20 - SYSTIMER_TARGET0_LO."]
    pub target0_lo: TARGET0_LO,
    #[doc = "0x24 - SYSTIMER_TARGET1_HI."]
    pub target1_hi: TARGET1_HI,
    #[doc = "0x28 - SYSTIMER_TARGET1_LO."]
    pub target1_lo: TARGET1_LO,
    #[doc = "0x2c - SYSTIMER_TARGET2_HI."]
    pub target2_hi: TARGET2_HI,
    #[doc = "0x30 - SYSTIMER_TARGET2_LO."]
    pub target2_lo: TARGET2_LO,
    #[doc = "0x34 - SYSTIMER_TARGET0_CONF."]
    pub target0_conf: TARGET0_CONF,
    #[doc = "0x38 - SYSTIMER_TARGET1_CONF."]
    pub target1_conf: TARGET1_CONF,
    #[doc = "0x3c - SYSTIMER_TARGET2_CONF."]
    pub target2_conf: TARGET2_CONF,
    #[doc = "0x40 - SYSTIMER_UNIT0_VALUE_HI."]
    pub unit0_value_hi: UNIT0_VALUE_HI,
    #[doc = "0x44 - SYSTIMER_UNIT0_VALUE_LO."]
    pub unit0_value_lo: UNIT0_VALUE_LO,
    #[doc = "0x48 - SYSTIMER_UNIT1_VALUE_HI."]
    pub unit1_value_hi: UNIT1_VALUE_HI,
    #[doc = "0x4c - SYSTIMER_UNIT1_VALUE_LO."]
    pub unit1_value_lo: UNIT1_VALUE_LO,
    #[doc = "0x50 - SYSTIMER_COMP0_LOAD."]
    pub comp0_load: COMP0_LOAD,
    #[doc = "0x54 - SYSTIMER_COMP1_LOAD."]
    pub comp1_load: COMP1_LOAD,
    #[doc = "0x58 - SYSTIMER_COMP2_LOAD."]
    pub comp2_load: COMP2_LOAD,
    #[doc = "0x5c - SYSTIMER_UNIT0_LOAD."]
    pub unit0_load: UNIT0_LOAD,
    #[doc = "0x60 - SYSTIMER_UNIT1_LOAD."]
    pub unit1_load: UNIT1_LOAD,
    #[doc = "0x64 - SYSTIMER_INT_ENA."]
    pub int_ena: INT_ENA,
    #[doc = "0x68 - SYSTIMER_INT_RAW."]
    pub int_raw: INT_RAW,
    #[doc = "0x6c - SYSTIMER_INT_CLR."]
    pub int_clr: INT_CLR,
    #[doc = "0x70 - SYSTIMER_INT_ST."]
    pub int_st: INT_ST,
    _reserved29: [u8; 0x88],
    #[doc = "0xfc - SYSTIMER_DATE."]
    pub date: DATE,
}
#[doc = "CONF (rw) register accessor: an alias for `Reg<CONF_SPEC>`"]
pub type CONF = crate::Reg<conf::CONF_SPEC>;
#[doc = "SYSTIMER_CONF."]
pub mod conf;
#[doc = "UNIT0_OP (rw) register accessor: an alias for `Reg<UNIT0_OP_SPEC>`"]
pub type UNIT0_OP = crate::Reg<unit0_op::UNIT0_OP_SPEC>;
#[doc = "SYSTIMER_UNIT0_OP."]
pub mod unit0_op;
#[doc = "UNIT1_OP (rw) register accessor: an alias for `Reg<UNIT1_OP_SPEC>`"]
pub type UNIT1_OP = crate::Reg<unit1_op::UNIT1_OP_SPEC>;
#[doc = "SYSTIMER_UNIT1_OP."]
pub mod unit1_op;
#[doc = "UNIT0_LOAD_HI (rw) register accessor: an alias for `Reg<UNIT0_LOAD_HI_SPEC>`"]
pub type UNIT0_LOAD_HI = crate::Reg<unit0_load_hi::UNIT0_LOAD_HI_SPEC>;
#[doc = "SYSTIMER_UNIT0_LOAD_HI."]
pub mod unit0_load_hi;
#[doc = "UNIT0_LOAD_LO (rw) register accessor: an alias for `Reg<UNIT0_LOAD_LO_SPEC>`"]
pub type UNIT0_LOAD_LO = crate::Reg<unit0_load_lo::UNIT0_LOAD_LO_SPEC>;
#[doc = "SYSTIMER_UNIT0_LOAD_LO."]
pub mod unit0_load_lo;
#[doc = "UNIT1_LOAD_HI (rw) register accessor: an alias for `Reg<UNIT1_LOAD_HI_SPEC>`"]
pub type UNIT1_LOAD_HI = crate::Reg<unit1_load_hi::UNIT1_LOAD_HI_SPEC>;
#[doc = "SYSTIMER_UNIT1_LOAD_HI."]
pub mod unit1_load_hi;
#[doc = "UNIT1_LOAD_LO (rw) register accessor: an alias for `Reg<UNIT1_LOAD_LO_SPEC>`"]
pub type UNIT1_LOAD_LO = crate::Reg<unit1_load_lo::UNIT1_LOAD_LO_SPEC>;
#[doc = "SYSTIMER_UNIT1_LOAD_LO."]
pub mod unit1_load_lo;
#[doc = "TARGET0_HI (rw) register accessor: an alias for `Reg<TARGET0_HI_SPEC>`"]
pub type TARGET0_HI = crate::Reg<target0_hi::TARGET0_HI_SPEC>;
#[doc = "SYSTIMER_TARGET0_HI."]
pub mod target0_hi;
#[doc = "TARGET0_LO (rw) register accessor: an alias for `Reg<TARGET0_LO_SPEC>`"]
pub type TARGET0_LO = crate::Reg<target0_lo::TARGET0_LO_SPEC>;
#[doc = "SYSTIMER_TARGET0_LO."]
pub mod target0_lo;
#[doc = "TARGET1_HI (rw) register accessor: an alias for `Reg<TARGET1_HI_SPEC>`"]
pub type TARGET1_HI = crate::Reg<target1_hi::TARGET1_HI_SPEC>;
#[doc = "SYSTIMER_TARGET1_HI."]
pub mod target1_hi;
#[doc = "TARGET1_LO (rw) register accessor: an alias for `Reg<TARGET1_LO_SPEC>`"]
pub type TARGET1_LO = crate::Reg<target1_lo::TARGET1_LO_SPEC>;
#[doc = "SYSTIMER_TARGET1_LO."]
pub mod target1_lo;
#[doc = "TARGET2_HI (rw) register accessor: an alias for `Reg<TARGET2_HI_SPEC>`"]
pub type TARGET2_HI = crate::Reg<target2_hi::TARGET2_HI_SPEC>;
#[doc = "SYSTIMER_TARGET2_HI."]
pub mod target2_hi;
#[doc = "TARGET2_LO (rw) register accessor: an alias for `Reg<TARGET2_LO_SPEC>`"]
pub type TARGET2_LO = crate::Reg<target2_lo::TARGET2_LO_SPEC>;
#[doc = "SYSTIMER_TARGET2_LO."]
pub mod target2_lo;
#[doc = "TARGET0_CONF (rw) register accessor: an alias for `Reg<TARGET0_CONF_SPEC>`"]
pub type TARGET0_CONF = crate::Reg<target0_conf::TARGET0_CONF_SPEC>;
#[doc = "SYSTIMER_TARGET0_CONF."]
pub mod target0_conf;
#[doc = "TARGET1_CONF (rw) register accessor: an alias for `Reg<TARGET1_CONF_SPEC>`"]
pub type TARGET1_CONF = crate::Reg<target1_conf::TARGET1_CONF_SPEC>;
#[doc = "SYSTIMER_TARGET1_CONF."]
pub mod target1_conf;
#[doc = "TARGET2_CONF (rw) register accessor: an alias for `Reg<TARGET2_CONF_SPEC>`"]
pub type TARGET2_CONF = crate::Reg<target2_conf::TARGET2_CONF_SPEC>;
#[doc = "SYSTIMER_TARGET2_CONF."]
pub mod target2_conf;
#[doc = "UNIT0_VALUE_HI (r) register accessor: an alias for `Reg<UNIT0_VALUE_HI_SPEC>`"]
pub type UNIT0_VALUE_HI = crate::Reg<unit0_value_hi::UNIT0_VALUE_HI_SPEC>;
#[doc = "SYSTIMER_UNIT0_VALUE_HI."]
pub mod unit0_value_hi;
#[doc = "UNIT0_VALUE_LO (r) register accessor: an alias for `Reg<UNIT0_VALUE_LO_SPEC>`"]
pub type UNIT0_VALUE_LO = crate::Reg<unit0_value_lo::UNIT0_VALUE_LO_SPEC>;
#[doc = "SYSTIMER_UNIT0_VALUE_LO."]
pub mod unit0_value_lo;
#[doc = "UNIT1_VALUE_HI (r) register accessor: an alias for `Reg<UNIT1_VALUE_HI_SPEC>`"]
pub type UNIT1_VALUE_HI = crate::Reg<unit1_value_hi::UNIT1_VALUE_HI_SPEC>;
#[doc = "SYSTIMER_UNIT1_VALUE_HI."]
pub mod unit1_value_hi;
#[doc = "UNIT1_VALUE_LO (r) register accessor: an alias for `Reg<UNIT1_VALUE_LO_SPEC>`"]
pub type UNIT1_VALUE_LO = crate::Reg<unit1_value_lo::UNIT1_VALUE_LO_SPEC>;
#[doc = "SYSTIMER_UNIT1_VALUE_LO."]
pub mod unit1_value_lo;
#[doc = "COMP0_LOAD (w) register accessor: an alias for `Reg<COMP0_LOAD_SPEC>`"]
pub type COMP0_LOAD = crate::Reg<comp0_load::COMP0_LOAD_SPEC>;
#[doc = "SYSTIMER_COMP0_LOAD."]
pub mod comp0_load;
#[doc = "COMP1_LOAD (w) register accessor: an alias for `Reg<COMP1_LOAD_SPEC>`"]
pub type COMP1_LOAD = crate::Reg<comp1_load::COMP1_LOAD_SPEC>;
#[doc = "SYSTIMER_COMP1_LOAD."]
pub mod comp1_load;
#[doc = "COMP2_LOAD (w) register accessor: an alias for `Reg<COMP2_LOAD_SPEC>`"]
pub type COMP2_LOAD = crate::Reg<comp2_load::COMP2_LOAD_SPEC>;
#[doc = "SYSTIMER_COMP2_LOAD."]
pub mod comp2_load;
#[doc = "UNIT0_LOAD (w) register accessor: an alias for `Reg<UNIT0_LOAD_SPEC>`"]
pub type UNIT0_LOAD = crate::Reg<unit0_load::UNIT0_LOAD_SPEC>;
#[doc = "SYSTIMER_UNIT0_LOAD."]
pub mod unit0_load;
#[doc = "UNIT1_LOAD (w) register accessor: an alias for `Reg<UNIT1_LOAD_SPEC>`"]
pub type UNIT1_LOAD = crate::Reg<unit1_load::UNIT1_LOAD_SPEC>;
#[doc = "SYSTIMER_UNIT1_LOAD."]
pub mod unit1_load;
#[doc = "INT_ENA (rw) register accessor: an alias for `Reg<INT_ENA_SPEC>`"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = "SYSTIMER_INT_ENA."]
pub mod int_ena;
#[doc = "INT_RAW (rw) register accessor: an alias for `Reg<INT_RAW_SPEC>`"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = "SYSTIMER_INT_RAW."]
pub mod int_raw;
#[doc = "INT_CLR (w) register accessor: an alias for `Reg<INT_CLR_SPEC>`"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = "SYSTIMER_INT_CLR."]
pub mod int_clr;
#[doc = "INT_ST (r) register accessor: an alias for `Reg<INT_ST_SPEC>`"]
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
#[doc = "SYSTIMER_INT_ST."]
pub mod int_st;
#[doc = "DATE (rw) register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "SYSTIMER_DATE."]
pub mod date;
