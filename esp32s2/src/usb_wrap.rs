#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    otg_conf: OTG_CONF,
    test_conf: TEST_CONF,
    _reserved2: [u8; 0x03f4],
    date: DATE,
}
impl RegisterBlock {
    #[doc = "0x00 - USB OTG Wrapper Configure Register"]
    #[inline(always)]
    pub const fn otg_conf(&self) -> &OTG_CONF {
        &self.otg_conf
    }
    #[doc = "0x04 - USB Internal PHY Testing Register"]
    #[inline(always)]
    pub const fn test_conf(&self) -> &TEST_CONF {
        &self.test_conf
    }
    #[doc = "0x3fc - Version Control Register"]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
#[doc = "OTG_CONF (rw) register accessor: USB OTG Wrapper Configure Register\n\nYou can [`read`](crate::Reg::read) this register and get [`otg_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_conf`] module"]
pub type OTG_CONF = crate::Reg<otg_conf::OTG_CONF_SPEC>;
#[doc = "USB OTG Wrapper Configure Register"]
pub mod otg_conf;
#[doc = "TEST_CONF (rw) register accessor: USB Internal PHY Testing Register\n\nYou can [`read`](crate::Reg::read) this register and get [`test_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`test_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@test_conf`] module"]
pub type TEST_CONF = crate::Reg<test_conf::TEST_CONF_SPEC>;
#[doc = "USB Internal PHY Testing Register"]
pub mod test_conf;
pub use crate::aes::date;
pub use crate::aes::DATE;
