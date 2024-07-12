#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0xfc],
    rtc_ulp_cp_timer: RTC_ULP_CP_TIMER,
    rtc_ulp_cp_ctrl: RTC_ULP_CP_CTRL,
    cocpu_ctrl: COCPU_CTRL,
    _reserved3: [u8; 0x2c],
    rtc_ulp_cp_timer_1: RTC_ULP_CP_TIMER_1,
}
impl RegisterBlock {
    #[doc = "0xfc - configure ulp"]
    #[inline(always)]
    pub const fn rtc_ulp_cp_timer(&self) -> &RTC_ULP_CP_TIMER {
        &self.rtc_ulp_cp_timer
    }
    #[doc = "0x100 - configure ulp"]
    #[inline(always)]
    pub const fn rtc_ulp_cp_ctrl(&self) -> &RTC_ULP_CP_CTRL {
        &self.rtc_ulp_cp_ctrl
    }
    #[doc = "0x104 - configure ulp-riscv"]
    #[inline(always)]
    pub const fn cocpu_ctrl(&self) -> &COCPU_CTRL {
        &self.cocpu_ctrl
    }
    #[doc = "0x134 - configure ulp sleep time"]
    #[inline(always)]
    pub const fn rtc_ulp_cp_timer_1(&self) -> &RTC_ULP_CP_TIMER_1 {
        &self.rtc_ulp_cp_timer_1
    }
}
#[doc = "RTC_ULP_CP_TIMER (rw) register accessor: configure ulp\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_ulp_cp_timer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_ulp_cp_timer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_ulp_cp_timer`] module"]
pub type RTC_ULP_CP_TIMER = crate::Reg<rtc_ulp_cp_timer::RTC_ULP_CP_TIMER_SPEC>;
#[doc = "configure ulp"]
pub mod rtc_ulp_cp_timer;
#[doc = "RTC_ULP_CP_CTRL (rw) register accessor: configure ulp\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_ulp_cp_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_ulp_cp_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_ulp_cp_ctrl`] module"]
pub type RTC_ULP_CP_CTRL = crate::Reg<rtc_ulp_cp_ctrl::RTC_ULP_CP_CTRL_SPEC>;
#[doc = "configure ulp"]
pub mod rtc_ulp_cp_ctrl;
#[doc = "COCPU_CTRL (rw) register accessor: configure ulp-riscv\n\nYou can [`read`](crate::Reg::read) this register and get [`cocpu_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cocpu_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cocpu_ctrl`] module"]
pub type COCPU_CTRL = crate::Reg<cocpu_ctrl::COCPU_CTRL_SPEC>;
#[doc = "configure ulp-riscv"]
pub mod cocpu_ctrl;
#[doc = "RTC_ULP_CP_TIMER_1 (rw) register accessor: configure ulp sleep time\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_ulp_cp_timer_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_ulp_cp_timer_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_ulp_cp_timer_1`] module"]
pub type RTC_ULP_CP_TIMER_1 = crate::Reg<rtc_ulp_cp_timer_1::RTC_ULP_CP_TIMER_1_SPEC>;
#[doc = "configure ulp sleep time"]
pub mod rtc_ulp_cp_timer_1;
