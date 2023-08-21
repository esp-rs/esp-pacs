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
#[doc = "OTG_CONF (rw) register accessor: USB OTG Wrapper Configure Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`otg_conf`] module"]
pub type OTG_CONF = crate::Reg<otg_conf::OTG_CONF_SPEC>;
#[doc = "USB OTG Wrapper Configure Register"]
pub mod otg_conf;
#[doc = "TEST_CONF (rw) register accessor: USB Internal PHY Testing Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`test_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`test_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`test_conf`] module"]
pub type TEST_CONF = crate::Reg<test_conf::TEST_CONF_SPEC>;
#[doc = "USB Internal PHY Testing Register"]
pub mod test_conf;
#[doc = "DATE (rw) register accessor: Version Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "Version Control Register"]
pub mod date;
