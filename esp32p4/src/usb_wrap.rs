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
    #[doc = "0x00 - USB wrapper configuration registers."]
    #[inline(always)]
    pub const fn otg_conf(&self) -> &OTG_CONF {
        &self.otg_conf
    }
    #[doc = "0x04 - USB wrapper test configuration registers."]
    #[inline(always)]
    pub const fn test_conf(&self) -> &TEST_CONF {
        &self.test_conf
    }
    #[doc = "0x3fc - Date register."]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
#[doc = "OTG_CONF (rw) register accessor: USB wrapper configuration registers.\n\nYou can [`read`](crate::Reg::read) this register and get [`otg_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_conf`] module"]
pub type OTG_CONF = crate::Reg<otg_conf::OTG_CONF_SPEC>;
#[doc = "USB wrapper configuration registers."]
pub mod otg_conf;
#[doc = "TEST_CONF (rw) register accessor: USB wrapper test configuration registers.\n\nYou can [`read`](crate::Reg::read) this register and get [`test_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`test_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@test_conf`] module"]
pub type TEST_CONF = crate::Reg<test_conf::TEST_CONF_SPEC>;
#[doc = "USB wrapper test configuration registers."]
pub mod test_conf;
#[doc = "DATE (r) register accessor: Date register.\n\nYou can [`read`](crate::Reg::read) this register and get [`date::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "Date register."]
pub mod date;
