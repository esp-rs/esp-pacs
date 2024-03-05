#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x54],
    bbpd_ctrl: BBPD_CTRL,
}
impl RegisterBlock {
    #[doc = "0x54 - Baseband control register"]
    #[inline(always)]
    pub const fn bbpd_ctrl(&self) -> &BBPD_CTRL {
        &self.bbpd_ctrl
    }
}
#[doc = "BBPD_CTRL (rw) register accessor: Baseband control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bbpd_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bbpd_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bbpd_ctrl`] module"]
pub type BBPD_CTRL = crate::Reg<bbpd_ctrl::BBPD_CTRL_SPEC>;
#[doc = "Baseband control register"]
pub mod bbpd_ctrl;
