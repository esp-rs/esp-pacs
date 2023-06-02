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
    #[doc = "0x34 - Region address register"]
    pub region4_addr_start: REGION4_ADDR_START,
    #[doc = "0x38 - Region address register"]
    pub region4_addr_end: REGION4_ADDR_END,
    #[doc = "0x3c - Region access authority attribute register"]
    pub region4_pms_attr: REGION4_PMS_ATTR,
    #[doc = "0x40 - Region address register"]
    pub region5_addr_start: REGION5_ADDR_START,
    #[doc = "0x44 - Region address register"]
    pub region5_addr_end: REGION5_ADDR_END,
    #[doc = "0x48 - Region access authority attribute register"]
    pub region5_pms_attr: REGION5_PMS_ATTR,
    #[doc = "0x4c - Region address register"]
    pub region6_addr_start: REGION6_ADDR_START,
    #[doc = "0x50 - Region address register"]
    pub region6_addr_end: REGION6_ADDR_END,
    #[doc = "0x54 - Region access authority attribute register"]
    pub region6_pms_attr: REGION6_PMS_ATTR,
    #[doc = "0x58 - Region address register"]
    pub region7_addr_start: REGION7_ADDR_START,
    #[doc = "0x5c - Region address register"]
    pub region7_addr_end: REGION7_ADDR_END,
    #[doc = "0x60 - Region access authority attribute register"]
    pub region7_pms_attr: REGION7_PMS_ATTR,
    #[doc = "0x64 - Region address register"]
    pub region8_addr_start: REGION8_ADDR_START,
    #[doc = "0x68 - Region address register"]
    pub region8_addr_end: REGION8_ADDR_END,
    #[doc = "0x6c - Region access authority attribute register"]
    pub region8_pms_attr: REGION8_PMS_ATTR,
    #[doc = "0x70 - Region address register"]
    pub region9_addr_start: REGION9_ADDR_START,
    #[doc = "0x74 - Region address register"]
    pub region9_addr_end: REGION9_ADDR_END,
    #[doc = "0x78 - Region access authority attribute register"]
    pub region9_pms_attr: REGION9_PMS_ATTR,
    #[doc = "0x7c - Region address register"]
    pub region10_addr_start: REGION10_ADDR_START,
    #[doc = "0x80 - Region address register"]
    pub region10_addr_end: REGION10_ADDR_END,
    #[doc = "0x84 - Region access authority attribute register"]
    pub region10_pms_attr: REGION10_PMS_ATTR,
    #[doc = "0x88 - Region address register"]
    pub region11_addr_start: REGION11_ADDR_START,
    #[doc = "0x8c - Region address register"]
    pub region11_addr_end: REGION11_ADDR_END,
    #[doc = "0x90 - Region access authority attribute register"]
    pub region11_pms_attr: REGION11_PMS_ATTR,
    #[doc = "0x94 - Region address register"]
    pub region12_addr_start: REGION12_ADDR_START,
    #[doc = "0x98 - Region address register"]
    pub region12_addr_end: REGION12_ADDR_END,
    #[doc = "0x9c - Region access authority attribute register"]
    pub region12_pms_attr: REGION12_PMS_ATTR,
    #[doc = "0xa0 - Region address register"]
    pub region13_addr_start: REGION13_ADDR_START,
    #[doc = "0xa4 - Region address register"]
    pub region13_addr_end: REGION13_ADDR_END,
    #[doc = "0xa8 - Region access authority attribute register"]
    pub region13_pms_attr: REGION13_PMS_ATTR,
    #[doc = "0xac - Region address register"]
    pub region14_addr_start: REGION14_ADDR_START,
    #[doc = "0xb0 - Region address register"]
    pub region14_addr_end: REGION14_ADDR_END,
    #[doc = "0xb4 - Region access authority attribute register"]
    pub region14_pms_attr: REGION14_PMS_ATTR,
    #[doc = "0xb8 - Region address register"]
    pub region15_addr_start: REGION15_ADDR_START,
    #[doc = "0xbc - Region address register"]
    pub region15_addr_end: REGION15_ADDR_END,
    #[doc = "0xc0 - Region access authority attribute register"]
    pub region15_pms_attr: REGION15_PMS_ATTR,
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
    #[doc = "0xd8 - M1 status register"]
    pub m1_status: M1_STATUS,
    #[doc = "0xdc - M1 status clear register"]
    pub m1_status_clr: M1_STATUS_CLR,
    #[doc = "0xe0 - M1 exception_info0 register"]
    pub m1_exception_info0: M1_EXCEPTION_INFO0,
    #[doc = "0xe4 - M1 exception_info1 register"]
    pub m1_exception_info1: M1_EXCEPTION_INFO1,
    #[doc = "0xe8 - M2 status register"]
    pub m2_status: M2_STATUS,
    #[doc = "0xec - M2 status clear register"]
    pub m2_status_clr: M2_STATUS_CLR,
    #[doc = "0xf0 - M2 exception_info0 register"]
    pub m2_exception_info0: M2_EXCEPTION_INFO0,
    #[doc = "0xf4 - M2 exception_info1 register"]
    pub m2_exception_info1: M2_EXCEPTION_INFO1,
    #[doc = "0xf8 - M3 status register"]
    pub m3_status: M3_STATUS,
    #[doc = "0xfc - M3 status clear register"]
    pub m3_status_clr: M3_STATUS_CLR,
    #[doc = "0x100 - M3 exception_info0 register"]
    pub m3_exception_info0: M3_EXCEPTION_INFO0,
    #[doc = "0x104 - M3 exception_info1 register"]
    pub m3_exception_info1: M3_EXCEPTION_INFO1,
    #[doc = "0x108 - APM interrupt enable register"]
    pub int_en: INT_EN,
    #[doc = "0x10c - clock gating register"]
    pub clock_gate: CLOCK_GATE,
    _reserved68: [u8; 0x06ec],
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
#[doc = "REGION4_ADDR_START (rw) register accessor: an alias for `Reg<REGION4_ADDR_START_SPEC>`"]
pub type REGION4_ADDR_START = crate::Reg<region4_addr_start::REGION4_ADDR_START_SPEC>;
#[doc = "Region address register"]
pub mod region4_addr_start;
#[doc = "REGION4_ADDR_END (rw) register accessor: an alias for `Reg<REGION4_ADDR_END_SPEC>`"]
pub type REGION4_ADDR_END = crate::Reg<region4_addr_end::REGION4_ADDR_END_SPEC>;
#[doc = "Region address register"]
pub mod region4_addr_end;
#[doc = "REGION4_PMS_ATTR (rw) register accessor: an alias for `Reg<REGION4_PMS_ATTR_SPEC>`"]
pub type REGION4_PMS_ATTR = crate::Reg<region4_pms_attr::REGION4_PMS_ATTR_SPEC>;
#[doc = "Region access authority attribute register"]
pub mod region4_pms_attr;
#[doc = "REGION5_ADDR_START (rw) register accessor: an alias for `Reg<REGION5_ADDR_START_SPEC>`"]
pub type REGION5_ADDR_START = crate::Reg<region5_addr_start::REGION5_ADDR_START_SPEC>;
#[doc = "Region address register"]
pub mod region5_addr_start;
#[doc = "REGION5_ADDR_END (rw) register accessor: an alias for `Reg<REGION5_ADDR_END_SPEC>`"]
pub type REGION5_ADDR_END = crate::Reg<region5_addr_end::REGION5_ADDR_END_SPEC>;
#[doc = "Region address register"]
pub mod region5_addr_end;
#[doc = "REGION5_PMS_ATTR (rw) register accessor: an alias for `Reg<REGION5_PMS_ATTR_SPEC>`"]
pub type REGION5_PMS_ATTR = crate::Reg<region5_pms_attr::REGION5_PMS_ATTR_SPEC>;
#[doc = "Region access authority attribute register"]
pub mod region5_pms_attr;
#[doc = "REGION6_ADDR_START (rw) register accessor: an alias for `Reg<REGION6_ADDR_START_SPEC>`"]
pub type REGION6_ADDR_START = crate::Reg<region6_addr_start::REGION6_ADDR_START_SPEC>;
#[doc = "Region address register"]
pub mod region6_addr_start;
#[doc = "REGION6_ADDR_END (rw) register accessor: an alias for `Reg<REGION6_ADDR_END_SPEC>`"]
pub type REGION6_ADDR_END = crate::Reg<region6_addr_end::REGION6_ADDR_END_SPEC>;
#[doc = "Region address register"]
pub mod region6_addr_end;
#[doc = "REGION6_PMS_ATTR (rw) register accessor: an alias for `Reg<REGION6_PMS_ATTR_SPEC>`"]
pub type REGION6_PMS_ATTR = crate::Reg<region6_pms_attr::REGION6_PMS_ATTR_SPEC>;
#[doc = "Region access authority attribute register"]
pub mod region6_pms_attr;
#[doc = "REGION7_ADDR_START (rw) register accessor: an alias for `Reg<REGION7_ADDR_START_SPEC>`"]
pub type REGION7_ADDR_START = crate::Reg<region7_addr_start::REGION7_ADDR_START_SPEC>;
#[doc = "Region address register"]
pub mod region7_addr_start;
#[doc = "REGION7_ADDR_END (rw) register accessor: an alias for `Reg<REGION7_ADDR_END_SPEC>`"]
pub type REGION7_ADDR_END = crate::Reg<region7_addr_end::REGION7_ADDR_END_SPEC>;
#[doc = "Region address register"]
pub mod region7_addr_end;
#[doc = "REGION7_PMS_ATTR (rw) register accessor: an alias for `Reg<REGION7_PMS_ATTR_SPEC>`"]
pub type REGION7_PMS_ATTR = crate::Reg<region7_pms_attr::REGION7_PMS_ATTR_SPEC>;
#[doc = "Region access authority attribute register"]
pub mod region7_pms_attr;
#[doc = "REGION8_ADDR_START (rw) register accessor: an alias for `Reg<REGION8_ADDR_START_SPEC>`"]
pub type REGION8_ADDR_START = crate::Reg<region8_addr_start::REGION8_ADDR_START_SPEC>;
#[doc = "Region address register"]
pub mod region8_addr_start;
#[doc = "REGION8_ADDR_END (rw) register accessor: an alias for `Reg<REGION8_ADDR_END_SPEC>`"]
pub type REGION8_ADDR_END = crate::Reg<region8_addr_end::REGION8_ADDR_END_SPEC>;
#[doc = "Region address register"]
pub mod region8_addr_end;
#[doc = "REGION8_PMS_ATTR (rw) register accessor: an alias for `Reg<REGION8_PMS_ATTR_SPEC>`"]
pub type REGION8_PMS_ATTR = crate::Reg<region8_pms_attr::REGION8_PMS_ATTR_SPEC>;
#[doc = "Region access authority attribute register"]
pub mod region8_pms_attr;
#[doc = "REGION9_ADDR_START (rw) register accessor: an alias for `Reg<REGION9_ADDR_START_SPEC>`"]
pub type REGION9_ADDR_START = crate::Reg<region9_addr_start::REGION9_ADDR_START_SPEC>;
#[doc = "Region address register"]
pub mod region9_addr_start;
#[doc = "REGION9_ADDR_END (rw) register accessor: an alias for `Reg<REGION9_ADDR_END_SPEC>`"]
pub type REGION9_ADDR_END = crate::Reg<region9_addr_end::REGION9_ADDR_END_SPEC>;
#[doc = "Region address register"]
pub mod region9_addr_end;
#[doc = "REGION9_PMS_ATTR (rw) register accessor: an alias for `Reg<REGION9_PMS_ATTR_SPEC>`"]
pub type REGION9_PMS_ATTR = crate::Reg<region9_pms_attr::REGION9_PMS_ATTR_SPEC>;
#[doc = "Region access authority attribute register"]
pub mod region9_pms_attr;
#[doc = "REGION10_ADDR_START (rw) register accessor: an alias for `Reg<REGION10_ADDR_START_SPEC>`"]
pub type REGION10_ADDR_START = crate::Reg<region10_addr_start::REGION10_ADDR_START_SPEC>;
#[doc = "Region address register"]
pub mod region10_addr_start;
#[doc = "REGION10_ADDR_END (rw) register accessor: an alias for `Reg<REGION10_ADDR_END_SPEC>`"]
pub type REGION10_ADDR_END = crate::Reg<region10_addr_end::REGION10_ADDR_END_SPEC>;
#[doc = "Region address register"]
pub mod region10_addr_end;
#[doc = "REGION10_PMS_ATTR (rw) register accessor: an alias for `Reg<REGION10_PMS_ATTR_SPEC>`"]
pub type REGION10_PMS_ATTR = crate::Reg<region10_pms_attr::REGION10_PMS_ATTR_SPEC>;
#[doc = "Region access authority attribute register"]
pub mod region10_pms_attr;
#[doc = "REGION11_ADDR_START (rw) register accessor: an alias for `Reg<REGION11_ADDR_START_SPEC>`"]
pub type REGION11_ADDR_START = crate::Reg<region11_addr_start::REGION11_ADDR_START_SPEC>;
#[doc = "Region address register"]
pub mod region11_addr_start;
#[doc = "REGION11_ADDR_END (rw) register accessor: an alias for `Reg<REGION11_ADDR_END_SPEC>`"]
pub type REGION11_ADDR_END = crate::Reg<region11_addr_end::REGION11_ADDR_END_SPEC>;
#[doc = "Region address register"]
pub mod region11_addr_end;
#[doc = "REGION11_PMS_ATTR (rw) register accessor: an alias for `Reg<REGION11_PMS_ATTR_SPEC>`"]
pub type REGION11_PMS_ATTR = crate::Reg<region11_pms_attr::REGION11_PMS_ATTR_SPEC>;
#[doc = "Region access authority attribute register"]
pub mod region11_pms_attr;
#[doc = "REGION12_ADDR_START (rw) register accessor: an alias for `Reg<REGION12_ADDR_START_SPEC>`"]
pub type REGION12_ADDR_START = crate::Reg<region12_addr_start::REGION12_ADDR_START_SPEC>;
#[doc = "Region address register"]
pub mod region12_addr_start;
#[doc = "REGION12_ADDR_END (rw) register accessor: an alias for `Reg<REGION12_ADDR_END_SPEC>`"]
pub type REGION12_ADDR_END = crate::Reg<region12_addr_end::REGION12_ADDR_END_SPEC>;
#[doc = "Region address register"]
pub mod region12_addr_end;
#[doc = "REGION12_PMS_ATTR (rw) register accessor: an alias for `Reg<REGION12_PMS_ATTR_SPEC>`"]
pub type REGION12_PMS_ATTR = crate::Reg<region12_pms_attr::REGION12_PMS_ATTR_SPEC>;
#[doc = "Region access authority attribute register"]
pub mod region12_pms_attr;
#[doc = "REGION13_ADDR_START (rw) register accessor: an alias for `Reg<REGION13_ADDR_START_SPEC>`"]
pub type REGION13_ADDR_START = crate::Reg<region13_addr_start::REGION13_ADDR_START_SPEC>;
#[doc = "Region address register"]
pub mod region13_addr_start;
#[doc = "REGION13_ADDR_END (rw) register accessor: an alias for `Reg<REGION13_ADDR_END_SPEC>`"]
pub type REGION13_ADDR_END = crate::Reg<region13_addr_end::REGION13_ADDR_END_SPEC>;
#[doc = "Region address register"]
pub mod region13_addr_end;
#[doc = "REGION13_PMS_ATTR (rw) register accessor: an alias for `Reg<REGION13_PMS_ATTR_SPEC>`"]
pub type REGION13_PMS_ATTR = crate::Reg<region13_pms_attr::REGION13_PMS_ATTR_SPEC>;
#[doc = "Region access authority attribute register"]
pub mod region13_pms_attr;
#[doc = "REGION14_ADDR_START (rw) register accessor: an alias for `Reg<REGION14_ADDR_START_SPEC>`"]
pub type REGION14_ADDR_START = crate::Reg<region14_addr_start::REGION14_ADDR_START_SPEC>;
#[doc = "Region address register"]
pub mod region14_addr_start;
#[doc = "REGION14_ADDR_END (rw) register accessor: an alias for `Reg<REGION14_ADDR_END_SPEC>`"]
pub type REGION14_ADDR_END = crate::Reg<region14_addr_end::REGION14_ADDR_END_SPEC>;
#[doc = "Region address register"]
pub mod region14_addr_end;
#[doc = "REGION14_PMS_ATTR (rw) register accessor: an alias for `Reg<REGION14_PMS_ATTR_SPEC>`"]
pub type REGION14_PMS_ATTR = crate::Reg<region14_pms_attr::REGION14_PMS_ATTR_SPEC>;
#[doc = "Region access authority attribute register"]
pub mod region14_pms_attr;
#[doc = "REGION15_ADDR_START (rw) register accessor: an alias for `Reg<REGION15_ADDR_START_SPEC>`"]
pub type REGION15_ADDR_START = crate::Reg<region15_addr_start::REGION15_ADDR_START_SPEC>;
#[doc = "Region address register"]
pub mod region15_addr_start;
#[doc = "REGION15_ADDR_END (rw) register accessor: an alias for `Reg<REGION15_ADDR_END_SPEC>`"]
pub type REGION15_ADDR_END = crate::Reg<region15_addr_end::REGION15_ADDR_END_SPEC>;
#[doc = "Region address register"]
pub mod region15_addr_end;
#[doc = "REGION15_PMS_ATTR (rw) register accessor: an alias for `Reg<REGION15_PMS_ATTR_SPEC>`"]
pub type REGION15_PMS_ATTR = crate::Reg<region15_pms_attr::REGION15_PMS_ATTR_SPEC>;
#[doc = "Region access authority attribute register"]
pub mod region15_pms_attr;
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
#[doc = "M1_STATUS (r) register accessor: an alias for `Reg<M1_STATUS_SPEC>`"]
pub type M1_STATUS = crate::Reg<m1_status::M1_STATUS_SPEC>;
#[doc = "M1 status register"]
pub mod m1_status;
#[doc = "M1_STATUS_CLR (w) register accessor: an alias for `Reg<M1_STATUS_CLR_SPEC>`"]
pub type M1_STATUS_CLR = crate::Reg<m1_status_clr::M1_STATUS_CLR_SPEC>;
#[doc = "M1 status clear register"]
pub mod m1_status_clr;
#[doc = "M1_EXCEPTION_INFO0 (r) register accessor: an alias for `Reg<M1_EXCEPTION_INFO0_SPEC>`"]
pub type M1_EXCEPTION_INFO0 = crate::Reg<m1_exception_info0::M1_EXCEPTION_INFO0_SPEC>;
#[doc = "M1 exception_info0 register"]
pub mod m1_exception_info0;
#[doc = "M1_EXCEPTION_INFO1 (r) register accessor: an alias for `Reg<M1_EXCEPTION_INFO1_SPEC>`"]
pub type M1_EXCEPTION_INFO1 = crate::Reg<m1_exception_info1::M1_EXCEPTION_INFO1_SPEC>;
#[doc = "M1 exception_info1 register"]
pub mod m1_exception_info1;
#[doc = "M2_STATUS (r) register accessor: an alias for `Reg<M2_STATUS_SPEC>`"]
pub type M2_STATUS = crate::Reg<m2_status::M2_STATUS_SPEC>;
#[doc = "M2 status register"]
pub mod m2_status;
#[doc = "M2_STATUS_CLR (w) register accessor: an alias for `Reg<M2_STATUS_CLR_SPEC>`"]
pub type M2_STATUS_CLR = crate::Reg<m2_status_clr::M2_STATUS_CLR_SPEC>;
#[doc = "M2 status clear register"]
pub mod m2_status_clr;
#[doc = "M2_EXCEPTION_INFO0 (r) register accessor: an alias for `Reg<M2_EXCEPTION_INFO0_SPEC>`"]
pub type M2_EXCEPTION_INFO0 = crate::Reg<m2_exception_info0::M2_EXCEPTION_INFO0_SPEC>;
#[doc = "M2 exception_info0 register"]
pub mod m2_exception_info0;
#[doc = "M2_EXCEPTION_INFO1 (r) register accessor: an alias for `Reg<M2_EXCEPTION_INFO1_SPEC>`"]
pub type M2_EXCEPTION_INFO1 = crate::Reg<m2_exception_info1::M2_EXCEPTION_INFO1_SPEC>;
#[doc = "M2 exception_info1 register"]
pub mod m2_exception_info1;
#[doc = "M3_STATUS (r) register accessor: an alias for `Reg<M3_STATUS_SPEC>`"]
pub type M3_STATUS = crate::Reg<m3_status::M3_STATUS_SPEC>;
#[doc = "M3 status register"]
pub mod m3_status;
#[doc = "M3_STATUS_CLR (w) register accessor: an alias for `Reg<M3_STATUS_CLR_SPEC>`"]
pub type M3_STATUS_CLR = crate::Reg<m3_status_clr::M3_STATUS_CLR_SPEC>;
#[doc = "M3 status clear register"]
pub mod m3_status_clr;
#[doc = "M3_EXCEPTION_INFO0 (r) register accessor: an alias for `Reg<M3_EXCEPTION_INFO0_SPEC>`"]
pub type M3_EXCEPTION_INFO0 = crate::Reg<m3_exception_info0::M3_EXCEPTION_INFO0_SPEC>;
#[doc = "M3 exception_info0 register"]
pub mod m3_exception_info0;
#[doc = "M3_EXCEPTION_INFO1 (r) register accessor: an alias for `Reg<M3_EXCEPTION_INFO1_SPEC>`"]
pub type M3_EXCEPTION_INFO1 = crate::Reg<m3_exception_info1::M3_EXCEPTION_INFO1_SPEC>;
#[doc = "M3 exception_info1 register"]
pub mod m3_exception_info1;
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
