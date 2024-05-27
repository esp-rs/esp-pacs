#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
///Cluster TIMER%s, containing TIMER?_CFG0, TIMER?_CFG1, TIMER?_SYNC, TIMER?_STATUS
pub struct TIMER {
    cfg0: CFG0,
    cfg1: CFG1,
    sync: SYNC,
    status: STATUS,
}
impl TIMER {
    ///0x00 - PWM TIMERx period and update method configuration register.
    #[inline(always)]
    pub const fn cfg0(&self) -> &CFG0 {
        &self.cfg0
    }
    ///0x04 - PWM TIMERx working mode and start/stop control configuration register.
    #[inline(always)]
    pub const fn cfg1(&self) -> &CFG1 {
        &self.cfg1
    }
    ///0x08 - PWM TIMERx sync function configuration register.
    #[inline(always)]
    pub const fn sync(&self) -> &SYNC {
        &self.sync
    }
    ///0x0c - PWM TIMERx status register.
    #[inline(always)]
    pub const fn status(&self) -> &STATUS {
        &self.status
    }
}
/**CFG0 (rw) register accessor: PWM TIMERx period and update method configuration register.

You can [`read`](crate::generic::Reg::read) this register and get [`cfg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfg0`] module*/
pub type CFG0 = crate::Reg<cfg0::CFG0_SPEC>;
///PWM TIMERx period and update method configuration register.
pub mod cfg0;
/**CFG1 (rw) register accessor: PWM TIMERx working mode and start/stop control configuration register.

You can [`read`](crate::generic::Reg::read) this register and get [`cfg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfg1`] module*/
pub type CFG1 = crate::Reg<cfg1::CFG1_SPEC>;
///PWM TIMERx working mode and start/stop control configuration register.
pub mod cfg1;
/**SYNC (rw) register accessor: PWM TIMERx sync function configuration register.

You can [`read`](crate::generic::Reg::read) this register and get [`sync::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sync::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sync`] module*/
pub type SYNC = crate::Reg<sync::SYNC_SPEC>;
///PWM TIMERx sync function configuration register.
pub mod sync;
/**STATUS (r) register accessor: PWM TIMERx status register.

You can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@status`] module*/
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
///PWM TIMERx status register.
pub mod status;
