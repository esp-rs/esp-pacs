#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - USB OTG Wrapper Configure Register"]
    pub otg_conf: OTG_CONF,
    #[doc = "0x04 - USB Internal PHY Testing Register"]
    pub test_conf: TEST_CONF,
    _reserved2: [u8; 0x03f4],
    #[doc = "0x3fc - Version Control Register"]
    pub date: DATE,
}
#[doc = "OTG_CONF (rw) register accessor: an alias for `Reg<OTG_CONF_SPEC>`"]
pub type OTG_CONF = crate::Reg<otg_conf::OTG_CONF_SPEC>;
#[doc = "USB OTG Wrapper Configure Register"]
pub mod otg_conf;
#[doc = "TEST_CONF (rw) register accessor: an alias for `Reg<TEST_CONF_SPEC>`"]
pub type TEST_CONF = crate::Reg<test_conf::TEST_CONF_SPEC>;
#[doc = "USB Internal PHY Testing Register"]
pub mod test_conf;
#[doc = "DATE (rw) register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "Version Control Register"]
pub mod date;
