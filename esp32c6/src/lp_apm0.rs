#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - Region filter enable register"]
    pub region_filter_en: REGION_FILTER_EN,
    #[doc = "0x04 - Region address register"]
    pub region0_addr_start: REGION0_ADDR_START,
    #[doc = "0x08 - Region address register"]
    pub region0_addr_end: REGION0_ADDR_END,
    #[doc = "0x0c - Region access authority attribute register"]
    pub region0_pms_attr: REGION0_PMS_ATTR,
    #[doc = "0x10 - Region address register"]
    pub region1_addr_start: REGION1_ADDR_START,
    #[doc = "0x14 - Region address register"]
    pub region1_addr_end: REGION1_ADDR_END,
    #[doc = "0x18 - Region access authority attribute register"]
    pub region1_pms_attr: REGION1_PMS_ATTR,
    #[doc = "0x1c - Region address register"]
    pub region2_addr_start: REGION2_ADDR_START,
    #[doc = "0x20 - Region address register"]
    pub region2_addr_end: REGION2_ADDR_END,
    #[doc = "0x24 - Region access authority attribute register"]
    pub region2_pms_attr: REGION2_PMS_ATTR,
    #[doc = "0x28 - Region address register"]
    pub region3_addr_start: REGION3_ADDR_START,
    #[doc = "0x2c - Region address register"]
    pub region3_addr_end: REGION3_ADDR_END,
    #[doc = "0x30 - Region access authority attribute register"]
    pub region3_pms_attr: REGION3_PMS_ATTR,
    _reserved13: [u8; 0x90],
    #[doc = "0xc4 - PMS function control register"]
    pub func_ctrl: FUNC_CTRL,
    #[doc = "0xc8 - M0 status register"]
    pub m0_status: M0_STATUS,
    #[doc = "0xcc - M0 status clear register"]
    pub m0_status_clr: M0_STATUS_CLR,
    #[doc = "0xd0 - M0 exception_info0 register"]
    pub m0_exception_info0: M0_EXCEPTION_INFO0,
    #[doc = "0xd4 - M0 exception_info1 register"]
    pub m0_exception_info1: M0_EXCEPTION_INFO1,
    #[doc = "0xd8 - APM interrupt enable register"]
    pub int_en: INT_EN,
    #[doc = "0xdc - clock gating register"]
    pub clock_gate: CLOCK_GATE,
    _reserved20: [u8; 0x071c],
    #[doc = "0x7fc - Version register"]
    pub date: DATE,
}
#[doc = "REGION_FILTER_EN (rw) register accessor: an alias for `Reg<REGION_FILTER_EN_SPEC>`"]
pub type REGION_FILTER_EN = crate::Reg<region_filter_en::REGION_FILTER_EN_SPEC>;
#[doc = "Region filter enable register"]
pub mod region_filter_en;
#[doc = "REGION0_ADDR_START (rw) register accessor: an alias for `Reg<REGION0_ADDR_START_SPEC>`"]
pub type REGION0_ADDR_START = crate::Reg<region0_addr_start::REGION0_ADDR_START_SPEC>;
#[doc = "Region address register"]
pub mod region0_addr_start;
#[doc = "REGION0_ADDR_END (rw) register accessor: an alias for `Reg<REGION0_ADDR_END_SPEC>`"]
pub type REGION0_ADDR_END = crate::Reg<region0_addr_end::REGION0_ADDR_END_SPEC>;
#[doc = "Region address register"]
pub mod region0_addr_end;
#[doc = "REGION0_PMS_ATTR (rw) register accessor: an alias for `Reg<REGION0_PMS_ATTR_SPEC>`"]
pub type REGION0_PMS_ATTR = crate::Reg<region0_pms_attr::REGION0_PMS_ATTR_SPEC>;
#[doc = "Region access authority attribute register"]
pub mod region0_pms_attr;
#[doc = "REGION1_ADDR_START (rw) register accessor: an alias for `Reg<REGION1_ADDR_START_SPEC>`"]
pub type REGION1_ADDR_START = crate::Reg<region1_addr_start::REGION1_ADDR_START_SPEC>;
#[doc = "Region address register"]
pub mod region1_addr_start;
#[doc = "REGION1_ADDR_END (rw) register accessor: an alias for `Reg<REGION1_ADDR_END_SPEC>`"]
pub type REGION1_ADDR_END = crate::Reg<region1_addr_end::REGION1_ADDR_END_SPEC>;
#[doc = "Region address register"]
pub mod region1_addr_end;
#[doc = "REGION1_PMS_ATTR (rw) register accessor: an alias for `Reg<REGION1_PMS_ATTR_SPEC>`"]
pub type REGION1_PMS_ATTR = crate::Reg<region1_pms_attr::REGION1_PMS_ATTR_SPEC>;
#[doc = "Region access authority attribute register"]
pub mod region1_pms_attr;
#[doc = "REGION2_ADDR_START (rw) register accessor: an alias for `Reg<REGION2_ADDR_START_SPEC>`"]
pub type REGION2_ADDR_START = crate::Reg<region2_addr_start::REGION2_ADDR_START_SPEC>;
#[doc = "Region address register"]
pub mod region2_addr_start;
#[doc = "REGION2_ADDR_END (rw) register accessor: an alias for `Reg<REGION2_ADDR_END_SPEC>`"]
pub type REGION2_ADDR_END = crate::Reg<region2_addr_end::REGION2_ADDR_END_SPEC>;
#[doc = "Region address register"]
pub mod region2_addr_end;
#[doc = "REGION2_PMS_ATTR (rw) register accessor: an alias for `Reg<REGION2_PMS_ATTR_SPEC>`"]
pub type REGION2_PMS_ATTR = crate::Reg<region2_pms_attr::REGION2_PMS_ATTR_SPEC>;
#[doc = "Region access authority attribute register"]
pub mod region2_pms_attr;
#[doc = "REGION3_ADDR_START (rw) register accessor: an alias for `Reg<REGION3_ADDR_START_SPEC>`"]
pub type REGION3_ADDR_START = crate::Reg<region3_addr_start::REGION3_ADDR_START_SPEC>;
#[doc = "Region address register"]
pub mod region3_addr_start;
#[doc = "REGION3_ADDR_END (rw) register accessor: an alias for `Reg<REGION3_ADDR_END_SPEC>`"]
pub type REGION3_ADDR_END = crate::Reg<region3_addr_end::REGION3_ADDR_END_SPEC>;
#[doc = "Region address register"]
pub mod region3_addr_end;
#[doc = "REGION3_PMS_ATTR (rw) register accessor: an alias for `Reg<REGION3_PMS_ATTR_SPEC>`"]
pub type REGION3_PMS_ATTR = crate::Reg<region3_pms_attr::REGION3_PMS_ATTR_SPEC>;
#[doc = "Region access authority attribute register"]
pub mod region3_pms_attr;
#[doc = "FUNC_CTRL (rw) register accessor: an alias for `Reg<FUNC_CTRL_SPEC>`"]
pub type FUNC_CTRL = crate::Reg<func_ctrl::FUNC_CTRL_SPEC>;
#[doc = "PMS function control register"]
pub mod func_ctrl;
#[doc = "M0_STATUS (r) register accessor: an alias for `Reg<M0_STATUS_SPEC>`"]
pub type M0_STATUS = crate::Reg<m0_status::M0_STATUS_SPEC>;
#[doc = "M0 status register"]
pub mod m0_status;
#[doc = "M0_STATUS_CLR (w) register accessor: an alias for `Reg<M0_STATUS_CLR_SPEC>`"]
pub type M0_STATUS_CLR = crate::Reg<m0_status_clr::M0_STATUS_CLR_SPEC>;
#[doc = "M0 status clear register"]
pub mod m0_status_clr;
#[doc = "M0_EXCEPTION_INFO0 (r) register accessor: an alias for `Reg<M0_EXCEPTION_INFO0_SPEC>`"]
pub type M0_EXCEPTION_INFO0 = crate::Reg<m0_exception_info0::M0_EXCEPTION_INFO0_SPEC>;
#[doc = "M0 exception_info0 register"]
pub mod m0_exception_info0;
#[doc = "M0_EXCEPTION_INFO1 (r) register accessor: an alias for `Reg<M0_EXCEPTION_INFO1_SPEC>`"]
pub type M0_EXCEPTION_INFO1 = crate::Reg<m0_exception_info1::M0_EXCEPTION_INFO1_SPEC>;
#[doc = "M0 exception_info1 register"]
pub mod m0_exception_info1;
#[doc = "INT_EN (rw) register accessor: an alias for `Reg<INT_EN_SPEC>`"]
pub type INT_EN = crate::Reg<int_en::INT_EN_SPEC>;
#[doc = "APM interrupt enable register"]
pub mod int_en;
#[doc = "CLOCK_GATE (rw) register accessor: an alias for `Reg<CLOCK_GATE_SPEC>`"]
pub type CLOCK_GATE = crate::Reg<clock_gate::CLOCK_GATE_SPEC>;
#[doc = "clock gating register"]
pub mod clock_gate;
#[doc = "DATE (rw) register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "Version register"]
pub mod date;
