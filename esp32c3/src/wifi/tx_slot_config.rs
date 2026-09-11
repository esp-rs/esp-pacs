#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Used to configure the TX slot."]
pub struct TX_SLOT_CONFIG {
    config: CONFIG,
    plcp0: PLCP0,
}
impl TX_SLOT_CONFIG {
    #[doc = "0x00 - Config"]
    #[inline(always)]
    pub const fn config(&self) -> &CONFIG {
        &self.config
    }
    #[doc = "0x04 - PLCP0"]
    #[inline(always)]
    pub const fn plcp0(&self) -> &PLCP0 {
        &self.plcp0
    }
}
#[doc = "CONFIG (rw) register accessor: Config\n\nYou can [`read`](crate::Reg::read) this register and get [`config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config`] module"]
pub type CONFIG = crate::Reg<config::CONFIG_SPEC>;
#[doc = "Config"]
pub mod config;
#[doc = "PLCP0 (rw) register accessor: PLCP0\n\nYou can [`read`](crate::Reg::read) this register and get [`plcp0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`plcp0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@plcp0`] module"]
pub type PLCP0 = crate::Reg<plcp0::PLCP0_SPEC>;
#[doc = "PLCP0"]
pub mod plcp0;
