#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    int_config: INT_CONFIG,
    int_info: INT_INFO,
    int_thresh: INT_THRESH,
    _reserved3: [u8; 0x0ff4],
    int_ctrl: [INT_CTRL; 48],
}
impl RegisterBlock {
    #[doc = "0x00 - Interrupt configuration register"]
    #[inline(always)]
    pub const fn int_config(&self) -> &INT_CONFIG {
        &self.int_config
    }
    #[doc = "0x04 - Interrupt information register"]
    #[inline(always)]
    pub const fn int_info(&self) -> &INT_INFO {
        &self.int_info
    }
    #[doc = "0x08 - Interrupt threshold register"]
    #[inline(always)]
    pub const fn int_thresh(&self) -> &INT_THRESH {
        &self.int_thresh
    }
    #[doc = "0x1000..0x10c0 - Interrupt control register for each interrupt source"]
    #[inline(always)]
    pub const fn int_ctrl(&self, n: usize) -> &INT_CTRL {
        &self.int_ctrl[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1000..0x10c0 - Interrupt control register for each interrupt source"]
    #[inline(always)]
    pub fn int_ctrl_iter(&self) -> impl Iterator<Item = &INT_CTRL> {
        self.int_ctrl.iter()
    }
}
#[doc = "INT_CONFIG (rw) register accessor: Interrupt configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_config`] module"]
pub type INT_CONFIG = crate::Reg<int_config::INT_CONFIG_SPEC>;
#[doc = "Interrupt configuration register"]
pub mod int_config;
#[doc = "INT_INFO (rw) register accessor: Interrupt information register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_info::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_info::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_info`] module"]
pub type INT_INFO = crate::Reg<int_info::INT_INFO_SPEC>;
#[doc = "Interrupt information register"]
pub mod int_info;
#[doc = "INT_THRESH (rw) register accessor: Interrupt threshold register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_thresh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_thresh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_thresh`] module"]
pub type INT_THRESH = crate::Reg<int_thresh::INT_THRESH_SPEC>;
#[doc = "Interrupt threshold register"]
pub mod int_thresh;
#[doc = "INT_CTRL (rw) register accessor: Interrupt control register for each interrupt source\n\nYou can [`read`](crate::Reg::read) this register and get [`int_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_ctrl`] module"]
pub type INT_CTRL = crate::Reg<int_ctrl::INT_CTRL_SPEC>;
#[doc = "Interrupt control register for each interrupt source"]
pub mod int_ctrl;
