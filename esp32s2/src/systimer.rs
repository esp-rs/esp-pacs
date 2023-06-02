#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - Configure system timer clock"]
    pub conf: CONF,
    #[doc = "0x04 - Load value to system timer"]
    pub load: LOAD,
    #[doc = "0x08 - High 32 bits to be loaded to system timer"]
    pub load_hi: LOAD_HI,
    #[doc = "0x0c - Low 32 bits to be loaded to system timer"]
    pub load_lo: LOAD_LO,
    #[doc = "0x10 - System timer accumulation step"]
    pub step: STEP,
    #[doc = "0x14 - System timer target 0, high 32 bits"]
    pub target0_hi: TARGET0_HI,
    #[doc = "0x18 - System timer target 0, low 32 bits"]
    pub target0_lo: TARGET0_LO,
    #[doc = "0x1c - System timer target 1, high 32 bits"]
    pub target1_hi: TARGET1_HI,
    #[doc = "0x20 - System timer target 1, low 32 bits"]
    pub target1_lo: TARGET1_LO,
    #[doc = "0x24 - System timer target 2, high 32 bits"]
    pub target2_hi: TARGET2_HI,
    #[doc = "0x28 - System timer target 2, low 32 bits"]
    pub target2_lo: TARGET2_LO,
    #[doc = "0x2c - Configure work mode for system timer target 0"]
    pub target0_conf: TARGET0_CONF,
    #[doc = "0x30 - Configure work mode for system timer target 1"]
    pub target1_conf: TARGET1_CONF,
    #[doc = "0x34 - Configure work mode for system timer target 2"]
    pub target2_conf: TARGET2_CONF,
    #[doc = "0x38 - Read out system timer value"]
    pub unit0_op: UNIT0_OP,
    #[doc = "0x3c - System timer value, high 32 bits"]
    pub unit0_value_hi: UNIT0_VALUE_HI,
    #[doc = "0x40 - System timer value, low 32 bits"]
    pub unit0_value_lo: UNIT0_VALUE_LO,
    #[doc = "0x44 - System timer interrupt enable"]
    pub int_ena: INT_ENA,
    #[doc = "0x48 - System timer interrupt raw"]
    pub int_raw: INT_RAW,
    #[doc = "0x4c - System timer interrupt clear"]
    pub int_clr: INT_CLR,
    _reserved20: [u8; 0xac],
    #[doc = "0xfc - Version control register"]
    pub date: DATE,
}
#[doc = "CONF (rw) register accessor: an alias for `Reg<CONF_SPEC>`"]
pub type CONF = crate::Reg<conf::CONF_SPEC>;
#[doc = "Configure system timer clock"]
pub mod conf;
#[doc = "LOAD (w) register accessor: an alias for `Reg<LOAD_SPEC>`"]
pub type LOAD = crate::Reg<load::LOAD_SPEC>;
#[doc = "Load value to system timer"]
pub mod load;
#[doc = "LOAD_HI (rw) register accessor: an alias for `Reg<LOAD_HI_SPEC>`"]
pub type LOAD_HI = crate::Reg<load_hi::LOAD_HI_SPEC>;
#[doc = "High 32 bits to be loaded to system timer"]
pub mod load_hi;
#[doc = "LOAD_LO (rw) register accessor: an alias for `Reg<LOAD_LO_SPEC>`"]
pub type LOAD_LO = crate::Reg<load_lo::LOAD_LO_SPEC>;
#[doc = "Low 32 bits to be loaded to system timer"]
pub mod load_lo;
#[doc = "STEP (rw) register accessor: an alias for `Reg<STEP_SPEC>`"]
pub type STEP = crate::Reg<step::STEP_SPEC>;
#[doc = "System timer accumulation step"]
pub mod step;
#[doc = "TARGET0_HI (rw) register accessor: an alias for `Reg<TARGET0_HI_SPEC>`"]
pub type TARGET0_HI = crate::Reg<target0_hi::TARGET0_HI_SPEC>;
#[doc = "System timer target 0, high 32 bits"]
pub mod target0_hi;
#[doc = "TARGET0_LO (rw) register accessor: an alias for `Reg<TARGET0_LO_SPEC>`"]
pub type TARGET0_LO = crate::Reg<target0_lo::TARGET0_LO_SPEC>;
#[doc = "System timer target 0, low 32 bits"]
pub mod target0_lo;
#[doc = "TARGET1_HI (rw) register accessor: an alias for `Reg<TARGET1_HI_SPEC>`"]
pub type TARGET1_HI = crate::Reg<target1_hi::TARGET1_HI_SPEC>;
#[doc = "System timer target 1, high 32 bits"]
pub mod target1_hi;
#[doc = "TARGET1_LO (rw) register accessor: an alias for `Reg<TARGET1_LO_SPEC>`"]
pub type TARGET1_LO = crate::Reg<target1_lo::TARGET1_LO_SPEC>;
#[doc = "System timer target 1, low 32 bits"]
pub mod target1_lo;
#[doc = "TARGET2_HI (rw) register accessor: an alias for `Reg<TARGET2_HI_SPEC>`"]
pub type TARGET2_HI = crate::Reg<target2_hi::TARGET2_HI_SPEC>;
#[doc = "System timer target 2, high 32 bits"]
pub mod target2_hi;
#[doc = "TARGET2_LO (rw) register accessor: an alias for `Reg<TARGET2_LO_SPEC>`"]
pub type TARGET2_LO = crate::Reg<target2_lo::TARGET2_LO_SPEC>;
#[doc = "System timer target 2, low 32 bits"]
pub mod target2_lo;
#[doc = "TARGET0_CONF (rw) register accessor: an alias for `Reg<TARGET0_CONF_SPEC>`"]
pub type TARGET0_CONF = crate::Reg<target0_conf::TARGET0_CONF_SPEC>;
#[doc = "Configure work mode for system timer target 0"]
pub mod target0_conf;
#[doc = "TARGET1_CONF (rw) register accessor: an alias for `Reg<TARGET1_CONF_SPEC>`"]
pub type TARGET1_CONF = crate::Reg<target1_conf::TARGET1_CONF_SPEC>;
#[doc = "Configure work mode for system timer target 1"]
pub mod target1_conf;
#[doc = "TARGET2_CONF (rw) register accessor: an alias for `Reg<TARGET2_CONF_SPEC>`"]
pub type TARGET2_CONF = crate::Reg<target2_conf::TARGET2_CONF_SPEC>;
#[doc = "Configure work mode for system timer target 2"]
pub mod target2_conf;
#[doc = "UNIT0_OP (rw) register accessor: an alias for `Reg<UNIT0_OP_SPEC>`"]
pub type UNIT0_OP = crate::Reg<unit0_op::UNIT0_OP_SPEC>;
#[doc = "Read out system timer value"]
pub mod unit0_op;
#[doc = "UNIT0_VALUE_HI (r) register accessor: an alias for `Reg<UNIT0_VALUE_HI_SPEC>`"]
pub type UNIT0_VALUE_HI = crate::Reg<unit0_value_hi::UNIT0_VALUE_HI_SPEC>;
#[doc = "System timer value, high 32 bits"]
pub mod unit0_value_hi;
#[doc = "UNIT0_VALUE_LO (r) register accessor: an alias for `Reg<UNIT0_VALUE_LO_SPEC>`"]
pub type UNIT0_VALUE_LO = crate::Reg<unit0_value_lo::UNIT0_VALUE_LO_SPEC>;
#[doc = "System timer value, low 32 bits"]
pub mod unit0_value_lo;
#[doc = "INT_ENA (rw) register accessor: an alias for `Reg<INT_ENA_SPEC>`"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = "System timer interrupt enable"]
pub mod int_ena;
#[doc = "INT_RAW (r) register accessor: an alias for `Reg<INT_RAW_SPEC>`"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = "System timer interrupt raw"]
pub mod int_raw;
#[doc = "INT_CLR (w) register accessor: an alias for `Reg<INT_CLR_SPEC>`"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = "System timer interrupt clear"]
pub mod int_clr;
#[doc = "DATE (rw) register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "Version control register"]
pub mod date;
