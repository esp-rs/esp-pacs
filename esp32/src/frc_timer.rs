#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    timer_load: TIMER_LOAD,
    timer_count: TIMER_COUNT,
    timer_ctrl: TIMER_CTRL,
    timer_int: TIMER_INT,
    timer_alarm: TIMER_ALARM,
}
impl RegisterBlock {
    #[doc = "0x00 - "]
    #[inline(always)]
    pub const fn timer_load(&self) -> &TIMER_LOAD {
        &self.timer_load
    }
    #[doc = "0x04 - "]
    #[inline(always)]
    pub const fn timer_count(&self) -> &TIMER_COUNT {
        &self.timer_count
    }
    #[doc = "0x08 - "]
    #[inline(always)]
    pub const fn timer_ctrl(&self) -> &TIMER_CTRL {
        &self.timer_ctrl
    }
    #[doc = "0x0c - "]
    #[inline(always)]
    pub const fn timer_int(&self) -> &TIMER_INT {
        &self.timer_int
    }
    #[doc = "0x10 - "]
    #[inline(always)]
    pub const fn timer_alarm(&self) -> &TIMER_ALARM {
        &self.timer_alarm
    }
}
#[doc = "TIMER_LOAD (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer_load::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer_load::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer_load`] module"]
pub type TIMER_LOAD = crate::Reg<timer_load::TIMER_LOAD_SPEC>;
#[doc = ""]
pub mod timer_load;
#[doc = "TIMER_COUNT (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer_count::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer_count::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer_count`] module"]
pub type TIMER_COUNT = crate::Reg<timer_count::TIMER_COUNT_SPEC>;
#[doc = ""]
pub mod timer_count;
#[doc = "TIMER_CTRL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer_ctrl`] module"]
pub type TIMER_CTRL = crate::Reg<timer_ctrl::TIMER_CTRL_SPEC>;
#[doc = ""]
pub mod timer_ctrl;
#[doc = "TIMER_INT (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer_int::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer_int::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer_int`] module"]
pub type TIMER_INT = crate::Reg<timer_int::TIMER_INT_SPEC>;
#[doc = ""]
pub mod timer_int;
#[doc = "TIMER_ALARM (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer_alarm::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer_alarm::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer_alarm`] module"]
pub type TIMER_ALARM = crate::Reg<timer_alarm::TIMER_ALARM_SPEC>;
#[doc = ""]
pub mod timer_alarm;
