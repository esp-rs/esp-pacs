#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Configure system timer clock"]
    pub conf: crate::Reg<conf::CONF_SPEC>,
    #[doc = "0x04 - system timer unit0 value update register"]
    pub unit0_op: crate::Reg<unit0_op::UNIT0_OP_SPEC>,
    #[doc = "0x08 - system timer unit1 value update register"]
    pub unit1_op: crate::Reg<unit1_op::UNIT1_OP_SPEC>,
    #[doc = "0x0c - system timer unit0 value high load register"]
    pub unit0_load_hi: crate::Reg<unit0_load_hi::UNIT0_LOAD_HI_SPEC>,
    #[doc = "0x10 - system timer unit0 value low load register"]
    pub unit0_load_lo: crate::Reg<unit0_load_lo::UNIT0_LOAD_LO_SPEC>,
    #[doc = "0x14 - system timer unit1 value high load register"]
    pub unit1_load_hi: crate::Reg<unit1_load_hi::UNIT1_LOAD_HI_SPEC>,
    #[doc = "0x18 - system timer unit1 value low load register"]
    pub unit1_load_lo: crate::Reg<unit1_load_lo::UNIT1_LOAD_LO_SPEC>,
    #[doc = "0x1c - system timer comp0 value high register"]
    pub target0_hi: crate::Reg<target0_hi::TARGET0_HI_SPEC>,
    #[doc = "0x20 - system timer comp0 value low register"]
    pub target0_lo: crate::Reg<target0_lo::TARGET0_LO_SPEC>,
    #[doc = "0x24 - system timer comp1 value high register"]
    pub target1_hi: crate::Reg<target1_hi::TARGET1_HI_SPEC>,
    #[doc = "0x28 - system timer comp1 value low register"]
    pub target1_lo: crate::Reg<target1_lo::TARGET1_LO_SPEC>,
    #[doc = "0x2c - system timer comp2 value high register"]
    pub target2_hi: crate::Reg<target2_hi::TARGET2_HI_SPEC>,
    #[doc = "0x30 - system timer comp2 value low register"]
    pub target2_lo: crate::Reg<target2_lo::TARGET2_LO_SPEC>,
    #[doc = "0x34 - system timer comp0 target mode register"]
    pub target0_conf: crate::Reg<target0_conf::TARGET0_CONF_SPEC>,
    #[doc = "0x38 - system timer comp1 target mode register"]
    pub target1_conf: crate::Reg<target1_conf::TARGET1_CONF_SPEC>,
    #[doc = "0x3c - system timer comp2 target mode register"]
    pub target2_conf: crate::Reg<target2_conf::TARGET2_CONF_SPEC>,
    #[doc = "0x40 - system timer unit0 value high register"]
    pub unit0_value_hi: crate::Reg<unit0_value_hi::UNIT0_VALUE_HI_SPEC>,
    #[doc = "0x44 - system timer unit0 value low register"]
    pub unit0_value_lo: crate::Reg<unit0_value_lo::UNIT0_VALUE_LO_SPEC>,
    #[doc = "0x48 - system timer unit1 value high register"]
    pub unit1_value_hi: crate::Reg<unit1_value_hi::UNIT1_VALUE_HI_SPEC>,
    #[doc = "0x4c - system timer unit1 value low register"]
    pub unit1_value_lo: crate::Reg<unit1_value_lo::UNIT1_VALUE_LO_SPEC>,
    #[doc = "0x50 - system timer comp0 conf sync register"]
    pub comp0_load: crate::Reg<comp0_load::COMP0_LOAD_SPEC>,
    #[doc = "0x54 - system timer comp1 conf sync register"]
    pub comp1_load: crate::Reg<comp1_load::COMP1_LOAD_SPEC>,
    #[doc = "0x58 - system timer comp2 conf sync register"]
    pub comp2_load: crate::Reg<comp2_load::COMP2_LOAD_SPEC>,
    #[doc = "0x5c - system timer unit0 conf sync register"]
    pub unit0_load: crate::Reg<unit0_load::UNIT0_LOAD_SPEC>,
    #[doc = "0x60 - system timer unit1 conf sync register"]
    pub unit1_load: crate::Reg<unit1_load::UNIT1_LOAD_SPEC>,
    #[doc = "0x64 - systimer interrupt enable register"]
    pub int_ena: crate::Reg<int_ena::INT_ENA_SPEC>,
    #[doc = "0x68 - systimer interrupt raw register"]
    pub int_raw: crate::Reg<int_raw::INT_RAW_SPEC>,
    #[doc = "0x6c - systimer interrupt clear register"]
    pub int_clr: crate::Reg<int_clr::INT_CLR_SPEC>,
    #[doc = "0x70 - systimer interrupt status register"]
    pub int_st: crate::Reg<int_st::INT_ST_SPEC>,
    _reserved29: [u8; 0x88],
    #[doc = "0xfc - system timer version control register"]
    pub date: crate::Reg<date::DATE_SPEC>,
}
#[doc = "CONF register accessor: an alias for `Reg<CONF_SPEC>`"]
pub type CONF = crate::Reg<conf::CONF_SPEC>;
#[doc = "Configure system timer clock"]
pub mod conf;
#[doc = "UNIT0_OP register accessor: an alias for `Reg<UNIT0_OP_SPEC>`"]
pub type UNIT0_OP = crate::Reg<unit0_op::UNIT0_OP_SPEC>;
#[doc = "system timer unit0 value update register"]
pub mod unit0_op;
#[doc = "UNIT1_OP register accessor: an alias for `Reg<UNIT1_OP_SPEC>`"]
pub type UNIT1_OP = crate::Reg<unit1_op::UNIT1_OP_SPEC>;
#[doc = "system timer unit1 value update register"]
pub mod unit1_op;
#[doc = "UNIT0_LOAD_HI register accessor: an alias for `Reg<UNIT0_LOAD_HI_SPEC>`"]
pub type UNIT0_LOAD_HI = crate::Reg<unit0_load_hi::UNIT0_LOAD_HI_SPEC>;
#[doc = "system timer unit0 value high load register"]
pub mod unit0_load_hi;
#[doc = "UNIT0_LOAD_LO register accessor: an alias for `Reg<UNIT0_LOAD_LO_SPEC>`"]
pub type UNIT0_LOAD_LO = crate::Reg<unit0_load_lo::UNIT0_LOAD_LO_SPEC>;
#[doc = "system timer unit0 value low load register"]
pub mod unit0_load_lo;
#[doc = "UNIT1_LOAD_HI register accessor: an alias for `Reg<UNIT1_LOAD_HI_SPEC>`"]
pub type UNIT1_LOAD_HI = crate::Reg<unit1_load_hi::UNIT1_LOAD_HI_SPEC>;
#[doc = "system timer unit1 value high load register"]
pub mod unit1_load_hi;
#[doc = "UNIT1_LOAD_LO register accessor: an alias for `Reg<UNIT1_LOAD_LO_SPEC>`"]
pub type UNIT1_LOAD_LO = crate::Reg<unit1_load_lo::UNIT1_LOAD_LO_SPEC>;
#[doc = "system timer unit1 value low load register"]
pub mod unit1_load_lo;
#[doc = "TARGET0_HI register accessor: an alias for `Reg<TARGET0_HI_SPEC>`"]
pub type TARGET0_HI = crate::Reg<target0_hi::TARGET0_HI_SPEC>;
#[doc = "system timer comp0 value high register"]
pub mod target0_hi;
#[doc = "TARGET0_LO register accessor: an alias for `Reg<TARGET0_LO_SPEC>`"]
pub type TARGET0_LO = crate::Reg<target0_lo::TARGET0_LO_SPEC>;
#[doc = "system timer comp0 value low register"]
pub mod target0_lo;
#[doc = "TARGET1_HI register accessor: an alias for `Reg<TARGET1_HI_SPEC>`"]
pub type TARGET1_HI = crate::Reg<target1_hi::TARGET1_HI_SPEC>;
#[doc = "system timer comp1 value high register"]
pub mod target1_hi;
#[doc = "TARGET1_LO register accessor: an alias for `Reg<TARGET1_LO_SPEC>`"]
pub type TARGET1_LO = crate::Reg<target1_lo::TARGET1_LO_SPEC>;
#[doc = "system timer comp1 value low register"]
pub mod target1_lo;
#[doc = "TARGET2_HI register accessor: an alias for `Reg<TARGET2_HI_SPEC>`"]
pub type TARGET2_HI = crate::Reg<target2_hi::TARGET2_HI_SPEC>;
#[doc = "system timer comp2 value high register"]
pub mod target2_hi;
#[doc = "TARGET2_LO register accessor: an alias for `Reg<TARGET2_LO_SPEC>`"]
pub type TARGET2_LO = crate::Reg<target2_lo::TARGET2_LO_SPEC>;
#[doc = "system timer comp2 value low register"]
pub mod target2_lo;
#[doc = "TARGET0_CONF register accessor: an alias for `Reg<TARGET0_CONF_SPEC>`"]
pub type TARGET0_CONF = crate::Reg<target0_conf::TARGET0_CONF_SPEC>;
#[doc = "system timer comp0 target mode register"]
pub mod target0_conf;
#[doc = "TARGET1_CONF register accessor: an alias for `Reg<TARGET1_CONF_SPEC>`"]
pub type TARGET1_CONF = crate::Reg<target1_conf::TARGET1_CONF_SPEC>;
#[doc = "system timer comp1 target mode register"]
pub mod target1_conf;
#[doc = "TARGET2_CONF register accessor: an alias for `Reg<TARGET2_CONF_SPEC>`"]
pub type TARGET2_CONF = crate::Reg<target2_conf::TARGET2_CONF_SPEC>;
#[doc = "system timer comp2 target mode register"]
pub mod target2_conf;
#[doc = "UNIT0_VALUE_HI register accessor: an alias for `Reg<UNIT0_VALUE_HI_SPEC>`"]
pub type UNIT0_VALUE_HI = crate::Reg<unit0_value_hi::UNIT0_VALUE_HI_SPEC>;
#[doc = "system timer unit0 value high register"]
pub mod unit0_value_hi;
#[doc = "UNIT0_VALUE_LO register accessor: an alias for `Reg<UNIT0_VALUE_LO_SPEC>`"]
pub type UNIT0_VALUE_LO = crate::Reg<unit0_value_lo::UNIT0_VALUE_LO_SPEC>;
#[doc = "system timer unit0 value low register"]
pub mod unit0_value_lo;
#[doc = "UNIT1_VALUE_HI register accessor: an alias for `Reg<UNIT1_VALUE_HI_SPEC>`"]
pub type UNIT1_VALUE_HI = crate::Reg<unit1_value_hi::UNIT1_VALUE_HI_SPEC>;
#[doc = "system timer unit1 value high register"]
pub mod unit1_value_hi;
#[doc = "UNIT1_VALUE_LO register accessor: an alias for `Reg<UNIT1_VALUE_LO_SPEC>`"]
pub type UNIT1_VALUE_LO = crate::Reg<unit1_value_lo::UNIT1_VALUE_LO_SPEC>;
#[doc = "system timer unit1 value low register"]
pub mod unit1_value_lo;
#[doc = "COMP0_LOAD register accessor: an alias for `Reg<COMP0_LOAD_SPEC>`"]
pub type COMP0_LOAD = crate::Reg<comp0_load::COMP0_LOAD_SPEC>;
#[doc = "system timer comp0 conf sync register"]
pub mod comp0_load;
#[doc = "COMP1_LOAD register accessor: an alias for `Reg<COMP1_LOAD_SPEC>`"]
pub type COMP1_LOAD = crate::Reg<comp1_load::COMP1_LOAD_SPEC>;
#[doc = "system timer comp1 conf sync register"]
pub mod comp1_load;
#[doc = "COMP2_LOAD register accessor: an alias for `Reg<COMP2_LOAD_SPEC>`"]
pub type COMP2_LOAD = crate::Reg<comp2_load::COMP2_LOAD_SPEC>;
#[doc = "system timer comp2 conf sync register"]
pub mod comp2_load;
#[doc = "UNIT0_LOAD register accessor: an alias for `Reg<UNIT0_LOAD_SPEC>`"]
pub type UNIT0_LOAD = crate::Reg<unit0_load::UNIT0_LOAD_SPEC>;
#[doc = "system timer unit0 conf sync register"]
pub mod unit0_load;
#[doc = "UNIT1_LOAD register accessor: an alias for `Reg<UNIT1_LOAD_SPEC>`"]
pub type UNIT1_LOAD = crate::Reg<unit1_load::UNIT1_LOAD_SPEC>;
#[doc = "system timer unit1 conf sync register"]
pub mod unit1_load;
#[doc = "INT_ENA register accessor: an alias for `Reg<INT_ENA_SPEC>`"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = "systimer interrupt enable register"]
pub mod int_ena;
#[doc = "INT_RAW register accessor: an alias for `Reg<INT_RAW_SPEC>`"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = "systimer interrupt raw register"]
pub mod int_raw;
#[doc = "INT_CLR register accessor: an alias for `Reg<INT_CLR_SPEC>`"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = "systimer interrupt clear register"]
pub mod int_clr;
#[doc = "INT_ST register accessor: an alias for `Reg<INT_ST_SPEC>`"]
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
#[doc = "systimer interrupt status register"]
pub mod int_st;
#[doc = "DATE register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "system timer version control register"]
pub mod date;
