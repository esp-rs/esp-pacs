#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
///Register block
pub struct RegisterBlock {
    _reserved0: [u8; 0xf8],
    ulp_cp_timer: ULP_CP_TIMER,
    ulp_cp_ctrl: ULP_CP_CTRL,
    cocpu_ctrl: COCPU_CTRL,
    _reserved3: [u8; 0x2c],
    ulp_cp_timer_1: ULP_CP_TIMER_1,
}
impl RegisterBlock {
    ///0xf8 - Configure coprocessor timer
    #[inline(always)]
    pub const fn ulp_cp_timer(&self) -> &ULP_CP_TIMER {
        &self.ulp_cp_timer
    }
    ///0xfc - ULP-FSM configuration register
    #[inline(always)]
    pub const fn ulp_cp_ctrl(&self) -> &ULP_CP_CTRL {
        &self.ulp_cp_ctrl
    }
    ///0x100 - ULP-RISCV configuration register
    #[inline(always)]
    pub const fn cocpu_ctrl(&self) -> &COCPU_CTRL {
        &self.cocpu_ctrl
    }
    ///0x130 - Configure sleep cycle of the timer
    #[inline(always)]
    pub const fn ulp_cp_timer_1(&self) -> &ULP_CP_TIMER_1 {
        &self.ulp_cp_timer_1
    }
}
/**ULP_CP_TIMER (rw) register accessor: Configure coprocessor timer

You can [`read`](crate::generic::Reg::read) this register and get [`ulp_cp_timer::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ulp_cp_timer::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ulp_cp_timer`] module*/
pub type ULP_CP_TIMER = crate::Reg<ulp_cp_timer::ULP_CP_TIMER_SPEC>;
///Configure coprocessor timer
pub mod ulp_cp_timer;
/**ULP_CP_CTRL (rw) register accessor: ULP-FSM configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`ulp_cp_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ulp_cp_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ulp_cp_ctrl`] module*/
pub type ULP_CP_CTRL = crate::Reg<ulp_cp_ctrl::ULP_CP_CTRL_SPEC>;
///ULP-FSM configuration register
pub mod ulp_cp_ctrl;
/**COCPU_CTRL (rw) register accessor: ULP-RISCV configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`cocpu_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cocpu_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cocpu_ctrl`] module*/
pub type COCPU_CTRL = crate::Reg<cocpu_ctrl::COCPU_CTRL_SPEC>;
///ULP-RISCV configuration register
pub mod cocpu_ctrl;
/**ULP_CP_TIMER_1 (rw) register accessor: Configure sleep cycle of the timer

You can [`read`](crate::generic::Reg::read) this register and get [`ulp_cp_timer_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ulp_cp_timer_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ulp_cp_timer_1`] module*/
pub type ULP_CP_TIMER_1 = crate::Reg<ulp_cp_timer_1::ULP_CP_TIMER_1_SPEC>;
///Configure sleep cycle of the timer
pub mod ulp_cp_timer_1;
