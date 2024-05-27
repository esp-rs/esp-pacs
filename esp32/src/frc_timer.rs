#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
///Register block
pub struct RegisterBlock {
    timer_load: TIMER_LOAD,
    timer_count: TIMER_COUNT,
    timer_ctrl: TIMER_CTRL,
    timer_int: TIMER_INT,
    timer_alarm: TIMER_ALARM,
}
impl RegisterBlock {
    ///0x00 -
    #[inline(always)]
    pub const fn timer_load(&self) -> &TIMER_LOAD {
        &self.timer_load
    }
    ///0x04 -
    #[inline(always)]
    pub const fn timer_count(&self) -> &TIMER_COUNT {
        &self.timer_count
    }
    ///0x08 -
    #[inline(always)]
    pub const fn timer_ctrl(&self) -> &TIMER_CTRL {
        &self.timer_ctrl
    }
    ///0x0c -
    #[inline(always)]
    pub const fn timer_int(&self) -> &TIMER_INT {
        &self.timer_int
    }
    ///0x10 -
    #[inline(always)]
    pub const fn timer_alarm(&self) -> &TIMER_ALARM {
        &self.timer_alarm
    }
}
/**TIMER_LOAD (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`timer_load::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer_load::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@timer_load`] module*/
pub type TIMER_LOAD = crate::Reg<timer_load::TIMER_LOAD_SPEC>;
///
pub mod timer_load;
/**TIMER_COUNT (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`timer_count::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer_count::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@timer_count`] module*/
pub type TIMER_COUNT = crate::Reg<timer_count::TIMER_COUNT_SPEC>;
///
pub mod timer_count;
/**TIMER_CTRL (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`timer_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@timer_ctrl`] module*/
pub type TIMER_CTRL = crate::Reg<timer_ctrl::TIMER_CTRL_SPEC>;
///
pub mod timer_ctrl;
/**TIMER_INT (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`timer_int::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer_int::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@timer_int`] module*/
pub type TIMER_INT = crate::Reg<timer_int::TIMER_INT_SPEC>;
///
pub mod timer_int;
/**TIMER_ALARM (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`timer_alarm::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer_alarm::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@timer_alarm`] module*/
pub type TIMER_ALARM = crate::Reg<timer_alarm::TIMER_ALARM_SPEC>;
///
pub mod timer_alarm;
