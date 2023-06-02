#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - "]
    pub cfg_data0: CFG_DATA0,
    #[doc = "0x04 - "]
    pub cfg_data1: CFG_DATA1,
    _reserved2: [u8; 0x14],
    #[doc = "0x1c - "]
    pub cfg_data7: CFG_DATA7,
    #[doc = "0x20 - "]
    pub cis_conf0: CIS_CONF0,
    #[doc = "0x24 - "]
    pub cis_conf1: CIS_CONF1,
    #[doc = "0x28 - "]
    pub cis_conf2: CIS_CONF2,
    #[doc = "0x2c - "]
    pub cis_conf3: CIS_CONF3,
    #[doc = "0x30 - "]
    pub cis_conf4: CIS_CONF4,
    #[doc = "0x34 - "]
    pub cis_conf5: CIS_CONF5,
    #[doc = "0x38 - "]
    pub cis_conf6: CIS_CONF6,
    #[doc = "0x3c - "]
    pub cis_conf7: CIS_CONF7,
    #[doc = "0x40 - "]
    pub cfg_data16: CFG_DATA16,
    _reserved12: [u8; 0xb8],
    #[doc = "0xfc - "]
    pub date: DATE,
}
#[doc = "CFG_DATA0 (rw) register accessor: an alias for `Reg<CFG_DATA0_SPEC>`"]
pub type CFG_DATA0 = crate::Reg<cfg_data0::CFG_DATA0_SPEC>;
#[doc = ""]
pub mod cfg_data0;
#[doc = "CFG_DATA1 (rw) register accessor: an alias for `Reg<CFG_DATA1_SPEC>`"]
pub type CFG_DATA1 = crate::Reg<cfg_data1::CFG_DATA1_SPEC>;
#[doc = ""]
pub mod cfg_data1;
#[doc = "CFG_DATA7 (rw) register accessor: an alias for `Reg<CFG_DATA7_SPEC>`"]
pub type CFG_DATA7 = crate::Reg<cfg_data7::CFG_DATA7_SPEC>;
#[doc = ""]
pub mod cfg_data7;
#[doc = "CIS_CONF0 (rw) register accessor: an alias for `Reg<CIS_CONF0_SPEC>`"]
pub type CIS_CONF0 = crate::Reg<cis_conf0::CIS_CONF0_SPEC>;
#[doc = ""]
pub mod cis_conf0;
#[doc = "CIS_CONF1 (rw) register accessor: an alias for `Reg<CIS_CONF1_SPEC>`"]
pub type CIS_CONF1 = crate::Reg<cis_conf1::CIS_CONF1_SPEC>;
#[doc = ""]
pub mod cis_conf1;
#[doc = "CIS_CONF2 (rw) register accessor: an alias for `Reg<CIS_CONF2_SPEC>`"]
pub type CIS_CONF2 = crate::Reg<cis_conf2::CIS_CONF2_SPEC>;
#[doc = ""]
pub mod cis_conf2;
#[doc = "CIS_CONF3 (rw) register accessor: an alias for `Reg<CIS_CONF3_SPEC>`"]
pub type CIS_CONF3 = crate::Reg<cis_conf3::CIS_CONF3_SPEC>;
#[doc = ""]
pub mod cis_conf3;
#[doc = "CIS_CONF4 (rw) register accessor: an alias for `Reg<CIS_CONF4_SPEC>`"]
pub type CIS_CONF4 = crate::Reg<cis_conf4::CIS_CONF4_SPEC>;
#[doc = ""]
pub mod cis_conf4;
#[doc = "CIS_CONF5 (rw) register accessor: an alias for `Reg<CIS_CONF5_SPEC>`"]
pub type CIS_CONF5 = crate::Reg<cis_conf5::CIS_CONF5_SPEC>;
#[doc = ""]
pub mod cis_conf5;
#[doc = "CIS_CONF6 (rw) register accessor: an alias for `Reg<CIS_CONF6_SPEC>`"]
pub type CIS_CONF6 = crate::Reg<cis_conf6::CIS_CONF6_SPEC>;
#[doc = ""]
pub mod cis_conf6;
#[doc = "CIS_CONF7 (rw) register accessor: an alias for `Reg<CIS_CONF7_SPEC>`"]
pub type CIS_CONF7 = crate::Reg<cis_conf7::CIS_CONF7_SPEC>;
#[doc = ""]
pub mod cis_conf7;
#[doc = "CFG_DATA16 (rw) register accessor: an alias for `Reg<CFG_DATA16_SPEC>`"]
pub type CFG_DATA16 = crate::Reg<cfg_data16::CFG_DATA16_SPEC>;
#[doc = ""]
pub mod cfg_data16;
#[doc = "DATE (rw) register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = ""]
pub mod date;
