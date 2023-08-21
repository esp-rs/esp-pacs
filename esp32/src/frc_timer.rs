#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - "]
    pub timer_load: TIMER_LOAD,
    #[doc = "0x04 - "]
    pub timer_count: TIMER_COUNT,
    #[doc = "0x08 - "]
    pub timer_ctrl: TIMER_CTRL,
    #[doc = "0x0c - "]
    pub timer_int: TIMER_INT,
    #[doc = "0x10 - "]
    pub timer_alarm: TIMER_ALARM,
}
#[doc = "TIMER_LOAD (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer_load::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer_load::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`timer_load`] module"]
pub type TIMER_LOAD = crate::Reg<timer_load::TIMER_LOAD_SPEC>;
#[doc = ""]
pub mod timer_load;
#[doc = "TIMER_COUNT (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer_count::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer_count::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`timer_count`] module"]
pub type TIMER_COUNT = crate::Reg<timer_count::TIMER_COUNT_SPEC>;
#[doc = ""]
pub mod timer_count;
#[doc = "TIMER_CTRL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`timer_ctrl`] module"]
pub type TIMER_CTRL = crate::Reg<timer_ctrl::TIMER_CTRL_SPEC>;
#[doc = ""]
pub mod timer_ctrl;
#[doc = "TIMER_INT (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer_int::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer_int::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`timer_int`] module"]
pub type TIMER_INT = crate::Reg<timer_int::TIMER_INT_SPEC>;
#[doc = ""]
pub mod timer_int;
#[doc = "TIMER_ALARM (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer_alarm::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer_alarm::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`timer_alarm`] module"]
pub type TIMER_ALARM = crate::Reg<timer_alarm::TIMER_ALARM_SPEC>;
#[doc = ""]
pub mod timer_alarm;
