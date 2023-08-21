#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - RTC common configure register"]
    pub options0: OPTIONS0,
    #[doc = "0x04 - configure min sleep time"]
    pub slp_timer0: SLP_TIMER0,
    #[doc = "0x08 - configure sleep time hi"]
    pub slp_timer1: SLP_TIMER1,
    #[doc = "0x0c - update rtc main timer"]
    pub time_update: TIME_UPDATE,
    #[doc = "0x10 - read rtc_main timer low bits"]
    pub time_low0: TIME_LOW0,
    #[doc = "0x14 - read rtc_main timer high bits"]
    pub time_high0: TIME_HIGH0,
    #[doc = "0x18 - configure chip sleep"]
    pub state0: STATE0,
    #[doc = "0x1c - rtc state wait time"]
    pub timer1: TIMER1,
    #[doc = "0x20 - rtc monitor state delay time"]
    pub timer2: TIMER2,
    #[doc = "0x24 - No public"]
    pub timer3: TIMER3,
    #[doc = "0x28 - No public"]
    pub timer4: TIMER4,
    #[doc = "0x2c - configure min sleep time"]
    pub timer5: TIMER5,
    #[doc = "0x30 - No public"]
    pub timer6: TIMER6,
    #[doc = "0x34 - analog configure register"]
    pub ana_conf: ANA_CONF,
    #[doc = "0x38 - get reset state"]
    pub reset_state: RESET_STATE,
    #[doc = "0x3c - configure wakeup state"]
    pub wakeup_state: WAKEUP_STATE,
    #[doc = "0x40 - configure rtc interrupt register"]
    pub int_ena_rtc: INT_ENA_RTC,
    #[doc = "0x44 - rtc interrupt register"]
    pub int_raw_rtc: INT_RAW_RTC,
    #[doc = "0x48 - rtc interrupt register"]
    pub int_st_rtc: INT_ST_RTC,
    #[doc = "0x4c - rtc interrupt register"]
    pub int_clr_rtc: INT_CLR_RTC,
    #[doc = "0x50 - Reserved register"]
    pub store0: STORE0,
    #[doc = "0x54 - Reserved register"]
    pub store1: STORE1,
    #[doc = "0x58 - Reserved register"]
    pub store2: STORE2,
    #[doc = "0x5c - Reserved register"]
    pub store3: STORE3,
    #[doc = "0x60 - Reserved register"]
    pub ext_xtl_conf: EXT_XTL_CONF,
    #[doc = "0x64 - ext wakeup configure"]
    pub ext_wakeup_conf: EXT_WAKEUP_CONF,
    #[doc = "0x68 - reject sleep register"]
    pub slp_reject_conf: SLP_REJECT_CONF,
    #[doc = "0x6c - conigure cpu freq"]
    pub cpu_period_conf: CPU_PERIOD_CONF,
    #[doc = "0x70 - No public"]
    pub sdio_act_conf: SDIO_ACT_CONF,
    #[doc = "0x74 - configure clock register"]
    pub clk_conf: CLK_CONF,
    #[doc = "0x78 - configure slow clk"]
    pub slow_clk_conf: SLOW_CLK_CONF,
    #[doc = "0x7c - configure flash power"]
    pub sdio_conf: SDIO_CONF,
    #[doc = "0x80 - No public"]
    pub bias_conf: BIAS_CONF,
    #[doc = "0x84 - configure rtc regulator"]
    pub rtc: RTC,
    #[doc = "0x88 - configure rtc power"]
    pub pwc: PWC,
    #[doc = "0x8c - No public"]
    pub regulator_drv_ctrl: REGULATOR_DRV_CTRL,
    #[doc = "0x90 - configure digital power"]
    pub dig_pwc: DIG_PWC,
    #[doc = "0x94 - congigure digital power isolation"]
    pub dig_iso: DIG_ISO,
    #[doc = "0x98 - configure rtc watch dog"]
    pub wdtconfig0: WDTCONFIG0,
    #[doc = "0x9c - stage0 hold time"]
    pub wdtconfig1: WDTCONFIG1,
    #[doc = "0xa0 - stage1 hold time"]
    pub wdtconfig2: WDTCONFIG2,
    #[doc = "0xa4 - stage2 hold time"]
    pub wdtconfig3: WDTCONFIG3,
    #[doc = "0xa8 - stage3 hold time"]
    pub wdtconfig4: WDTCONFIG4,
    #[doc = "0xac - rtc wdt feed"]
    pub wdtfeed: WDTFEED,
    #[doc = "0xb0 - configure rtc watch dog"]
    pub wdtwprotect: WDTWPROTECT,
    #[doc = "0xb4 - congfigure super watch dog"]
    pub swd_conf: SWD_CONF,
    #[doc = "0xb8 - super watch dog key"]
    pub swd_wprotect: SWD_WPROTECT,
    #[doc = "0xbc - configure cpu stall by sw"]
    pub sw_cpu_stall: SW_CPU_STALL,
    #[doc = "0xc0 - reserved register"]
    pub store4: STORE4,
    #[doc = "0xc4 - reserved register"]
    pub store5: STORE5,
    #[doc = "0xc8 - reserved register"]
    pub store6: STORE6,
    #[doc = "0xcc - reserved register"]
    pub store7: STORE7,
    #[doc = "0xd0 - reserved register"]
    pub low_power_st: LOW_POWER_ST,
    #[doc = "0xd4 - No public"]
    pub diag0: DIAG0,
    #[doc = "0xd8 - rtc pad hold configure"]
    pub pad_hold: PAD_HOLD,
    #[doc = "0xdc - configure digtal pad hold"]
    pub dig_pad_hold: DIG_PAD_HOLD,
    #[doc = "0xe0 - configure ext1 wakeup"]
    pub ext_wakeup1: EXT_WAKEUP1,
    #[doc = "0xe4 - check ext wakeup1 status"]
    pub ext_wakeup1_status: EXT_WAKEUP1_STATUS,
    #[doc = "0xe8 - congfigure brownout"]
    pub brown_out: BROWN_OUT,
    #[doc = "0xec - RTC timer low 32 bits"]
    pub time_low1: TIME_LOW1,
    #[doc = "0xf0 - RTC timer high 16 bits"]
    pub time_high1: TIME_HIGH1,
    #[doc = "0xf4 - xtal 32k watch dog backup clock factor"]
    pub xtal32k_clk_factor: XTAL32K_CLK_FACTOR,
    #[doc = "0xf8 - configure xtal32k"]
    pub xtal32k_conf: XTAL32K_CONF,
    #[doc = "0xfc - configure ulp"]
    pub ulp_cp_timer: ULP_CP_TIMER,
    #[doc = "0x100 - configure ulp"]
    pub ulp_cp_ctrl: ULP_CP_CTRL,
    #[doc = "0x104 - configure ulp-riscv"]
    pub cocpu_ctrl: COCPU_CTRL,
    #[doc = "0x108 - configure touch controller"]
    pub touch_ctrl1: TOUCH_CTRL1,
    #[doc = "0x10c - configure touch controller"]
    pub touch_ctrl2: TOUCH_CTRL2,
    #[doc = "0x110 - configure touch controller"]
    pub touch_scan_ctrl: TOUCH_SCAN_CTRL,
    #[doc = "0x114 - configure touch controller"]
    pub touch_slp_thres: TOUCH_SLP_THRES,
    #[doc = "0x118 - configure touch controller"]
    pub touch_approach: TOUCH_APPROACH,
    #[doc = "0x11c - configure touch controller"]
    pub touch_filter_ctrl: TOUCH_FILTER_CTRL,
    #[doc = "0x120 - usb configure"]
    pub usb_conf: USB_CONF,
    #[doc = "0x124 - configure touch controller"]
    pub touch_timeout_ctrl: TOUCH_TIMEOUT_CTRL,
    #[doc = "0x128 - get reject casue"]
    pub slp_reject_cause: SLP_REJECT_CAUSE,
    #[doc = "0x12c - rtc common configure"]
    pub option1: OPTION1,
    #[doc = "0x130 - get wakeup cause"]
    pub slp_wakeup_cause: SLP_WAKEUP_CAUSE,
    #[doc = "0x134 - configure ulp sleep time"]
    pub ulp_cp_timer_1: ULP_CP_TIMER_1,
    #[doc = "0x138 - oneset rtc interrupt"]
    pub int_ena_rtc_w1ts: INT_ENA_RTC_W1TS,
    #[doc = "0x13c - oneset clr rtc interrupt enable"]
    pub int_ena_rtc_w1tc: INT_ENA_RTC_W1TC,
    #[doc = "0x140 - configure retention"]
    pub retention_ctrl: RETENTION_CTRL,
    #[doc = "0x144 - configure power glitch"]
    pub pg_ctrl: PG_CTRL,
    #[doc = "0x148 - No public"]
    pub fib_sel: FIB_SEL,
    #[doc = "0x14c - configure touch dac"]
    pub touch_dac: TOUCH_DAC,
    #[doc = "0x150 - configure touch dac"]
    pub touch_dac1: TOUCH_DAC1,
    #[doc = "0x154 - configure ulp diable"]
    pub cocpu_disable: COCPU_DISABLE,
    _reserved86: [u8; 0xa4],
    #[doc = "0x1fc - version register"]
    pub date: DATE,
}
#[doc = "OPTIONS0 (rw) register accessor: RTC common configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`options0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`options0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`options0`] module"]
pub type OPTIONS0 = crate::Reg<options0::OPTIONS0_SPEC>;
#[doc = "RTC common configure register"]
pub mod options0;
#[doc = "SLP_TIMER0 (rw) register accessor: configure min sleep time\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slp_timer0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slp_timer0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`slp_timer0`] module"]
pub type SLP_TIMER0 = crate::Reg<slp_timer0::SLP_TIMER0_SPEC>;
#[doc = "configure min sleep time"]
pub mod slp_timer0;
#[doc = "SLP_TIMER1 (rw) register accessor: configure sleep time hi\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slp_timer1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slp_timer1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`slp_timer1`] module"]
pub type SLP_TIMER1 = crate::Reg<slp_timer1::SLP_TIMER1_SPEC>;
#[doc = "configure sleep time hi"]
pub mod slp_timer1;
#[doc = "TIME_UPDATE (rw) register accessor: update rtc main timer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`time_update::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`time_update::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`time_update`] module"]
pub type TIME_UPDATE = crate::Reg<time_update::TIME_UPDATE_SPEC>;
#[doc = "update rtc main timer"]
pub mod time_update;
#[doc = "TIME_LOW0 (r) register accessor: read rtc_main timer low bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`time_low0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`time_low0`] module"]
pub type TIME_LOW0 = crate::Reg<time_low0::TIME_LOW0_SPEC>;
#[doc = "read rtc_main timer low bits"]
pub mod time_low0;
#[doc = "TIME_HIGH0 (r) register accessor: read rtc_main timer high bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`time_high0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`time_high0`] module"]
pub type TIME_HIGH0 = crate::Reg<time_high0::TIME_HIGH0_SPEC>;
#[doc = "read rtc_main timer high bits"]
pub mod time_high0;
#[doc = "STATE0 (rw) register accessor: configure chip sleep\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`state0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`state0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`state0`] module"]
pub type STATE0 = crate::Reg<state0::STATE0_SPEC>;
#[doc = "configure chip sleep"]
pub mod state0;
#[doc = "TIMER1 (rw) register accessor: rtc state wait time\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`timer1`] module"]
pub type TIMER1 = crate::Reg<timer1::TIMER1_SPEC>;
#[doc = "rtc state wait time"]
pub mod timer1;
#[doc = "TIMER2 (rw) register accessor: rtc monitor state delay time\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`timer2`] module"]
pub type TIMER2 = crate::Reg<timer2::TIMER2_SPEC>;
#[doc = "rtc monitor state delay time"]
pub mod timer2;
#[doc = "TIMER3 (rw) register accessor: No public\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`timer3`] module"]
pub type TIMER3 = crate::Reg<timer3::TIMER3_SPEC>;
#[doc = "No public"]
pub mod timer3;
#[doc = "TIMER4 (rw) register accessor: No public\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`timer4`] module"]
pub type TIMER4 = crate::Reg<timer4::TIMER4_SPEC>;
#[doc = "No public"]
pub mod timer4;
#[doc = "TIMER5 (rw) register accessor: configure min sleep time\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`timer5`] module"]
pub type TIMER5 = crate::Reg<timer5::TIMER5_SPEC>;
#[doc = "configure min sleep time"]
pub mod timer5;
#[doc = "TIMER6 (rw) register accessor: No public\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`timer6`] module"]
pub type TIMER6 = crate::Reg<timer6::TIMER6_SPEC>;
#[doc = "No public"]
pub mod timer6;
#[doc = "ANA_CONF (rw) register accessor: analog configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ana_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ana_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ana_conf`] module"]
pub type ANA_CONF = crate::Reg<ana_conf::ANA_CONF_SPEC>;
#[doc = "analog configure register"]
pub mod ana_conf;
#[doc = "RESET_STATE (rw) register accessor: get reset state\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reset_state::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reset_state::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`reset_state`] module"]
pub type RESET_STATE = crate::Reg<reset_state::RESET_STATE_SPEC>;
#[doc = "get reset state"]
pub mod reset_state;
#[doc = "WAKEUP_STATE (rw) register accessor: configure wakeup state\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wakeup_state::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wakeup_state::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`wakeup_state`] module"]
pub type WAKEUP_STATE = crate::Reg<wakeup_state::WAKEUP_STATE_SPEC>;
#[doc = "configure wakeup state"]
pub mod wakeup_state;
#[doc = "INT_ENA_RTC (rw) register accessor: configure rtc interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_ena_rtc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena_rtc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`int_ena_rtc`] module"]
pub type INT_ENA_RTC = crate::Reg<int_ena_rtc::INT_ENA_RTC_SPEC>;
#[doc = "configure rtc interrupt register"]
pub mod int_ena_rtc;
#[doc = "INT_RAW_RTC (rw) register accessor: rtc interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_raw_rtc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_raw_rtc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`int_raw_rtc`] module"]
pub type INT_RAW_RTC = crate::Reg<int_raw_rtc::INT_RAW_RTC_SPEC>;
#[doc = "rtc interrupt register"]
pub mod int_raw_rtc;
#[doc = "INT_ST_RTC (r) register accessor: rtc interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_st_rtc::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`int_st_rtc`] module"]
pub type INT_ST_RTC = crate::Reg<int_st_rtc::INT_ST_RTC_SPEC>;
#[doc = "rtc interrupt register"]
pub mod int_st_rtc;
#[doc = "INT_CLR_RTC (w) register accessor: rtc interrupt register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr_rtc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`int_clr_rtc`] module"]
pub type INT_CLR_RTC = crate::Reg<int_clr_rtc::INT_CLR_RTC_SPEC>;
#[doc = "rtc interrupt register"]
pub mod int_clr_rtc;
#[doc = "STORE0 (rw) register accessor: Reserved register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`store0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`store0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`store0`] module"]
pub type STORE0 = crate::Reg<store0::STORE0_SPEC>;
#[doc = "Reserved register"]
pub mod store0;
#[doc = "STORE1 (rw) register accessor: Reserved register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`store1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`store1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`store1`] module"]
pub type STORE1 = crate::Reg<store1::STORE1_SPEC>;
#[doc = "Reserved register"]
pub mod store1;
#[doc = "STORE2 (rw) register accessor: Reserved register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`store2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`store2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`store2`] module"]
pub type STORE2 = crate::Reg<store2::STORE2_SPEC>;
#[doc = "Reserved register"]
pub mod store2;
#[doc = "STORE3 (rw) register accessor: Reserved register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`store3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`store3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`store3`] module"]
pub type STORE3 = crate::Reg<store3::STORE3_SPEC>;
#[doc = "Reserved register"]
pub mod store3;
#[doc = "EXT_XTL_CONF (rw) register accessor: Reserved register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ext_xtl_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ext_xtl_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ext_xtl_conf`] module"]
pub type EXT_XTL_CONF = crate::Reg<ext_xtl_conf::EXT_XTL_CONF_SPEC>;
#[doc = "Reserved register"]
pub mod ext_xtl_conf;
#[doc = "EXT_WAKEUP_CONF (rw) register accessor: ext wakeup configure\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ext_wakeup_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ext_wakeup_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ext_wakeup_conf`] module"]
pub type EXT_WAKEUP_CONF = crate::Reg<ext_wakeup_conf::EXT_WAKEUP_CONF_SPEC>;
#[doc = "ext wakeup configure"]
pub mod ext_wakeup_conf;
#[doc = "SLP_REJECT_CONF (rw) register accessor: reject sleep register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slp_reject_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slp_reject_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`slp_reject_conf`] module"]
pub type SLP_REJECT_CONF = crate::Reg<slp_reject_conf::SLP_REJECT_CONF_SPEC>;
#[doc = "reject sleep register"]
pub mod slp_reject_conf;
#[doc = "CPU_PERIOD_CONF (rw) register accessor: conigure cpu freq\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_period_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_period_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cpu_period_conf`] module"]
pub type CPU_PERIOD_CONF = crate::Reg<cpu_period_conf::CPU_PERIOD_CONF_SPEC>;
#[doc = "conigure cpu freq"]
pub mod cpu_period_conf;
#[doc = "SDIO_ACT_CONF (rw) register accessor: No public\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdio_act_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdio_act_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sdio_act_conf`] module"]
pub type SDIO_ACT_CONF = crate::Reg<sdio_act_conf::SDIO_ACT_CONF_SPEC>;
#[doc = "No public"]
pub mod sdio_act_conf;
#[doc = "CLK_CONF (rw) register accessor: configure clock register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`clk_conf`] module"]
pub type CLK_CONF = crate::Reg<clk_conf::CLK_CONF_SPEC>;
#[doc = "configure clock register"]
pub mod clk_conf;
#[doc = "SLOW_CLK_CONF (rw) register accessor: configure slow clk\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slow_clk_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slow_clk_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`slow_clk_conf`] module"]
pub type SLOW_CLK_CONF = crate::Reg<slow_clk_conf::SLOW_CLK_CONF_SPEC>;
#[doc = "configure slow clk"]
pub mod slow_clk_conf;
#[doc = "SDIO_CONF (rw) register accessor: configure flash power\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdio_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdio_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sdio_conf`] module"]
pub type SDIO_CONF = crate::Reg<sdio_conf::SDIO_CONF_SPEC>;
#[doc = "configure flash power"]
pub mod sdio_conf;
#[doc = "BIAS_CONF (rw) register accessor: No public\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bias_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bias_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`bias_conf`] module"]
pub type BIAS_CONF = crate::Reg<bias_conf::BIAS_CONF_SPEC>;
#[doc = "No public"]
pub mod bias_conf;
#[doc = "RTC (rw) register accessor: configure rtc regulator\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rtc`] module"]
pub type RTC = crate::Reg<rtc::RTC_SPEC>;
#[doc = "configure rtc regulator"]
pub mod rtc;
#[doc = "PWC (rw) register accessor: configure rtc power\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pwc`] module"]
pub type PWC = crate::Reg<pwc::PWC_SPEC>;
#[doc = "configure rtc power"]
pub mod pwc;
#[doc = "REGULATOR_DRV_CTRL (rw) register accessor: No public\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regulator_drv_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regulator_drv_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`regulator_drv_ctrl`] module"]
pub type REGULATOR_DRV_CTRL = crate::Reg<regulator_drv_ctrl::REGULATOR_DRV_CTRL_SPEC>;
#[doc = "No public"]
pub mod regulator_drv_ctrl;
#[doc = "DIG_PWC (rw) register accessor: configure digital power\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dig_pwc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dig_pwc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dig_pwc`] module"]
pub type DIG_PWC = crate::Reg<dig_pwc::DIG_PWC_SPEC>;
#[doc = "configure digital power"]
pub mod dig_pwc;
#[doc = "DIG_ISO (rw) register accessor: congigure digital power isolation\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dig_iso::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dig_iso::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dig_iso`] module"]
pub type DIG_ISO = crate::Reg<dig_iso::DIG_ISO_SPEC>;
#[doc = "congigure digital power isolation"]
pub mod dig_iso;
#[doc = "WDTCONFIG0 (rw) register accessor: configure rtc watch dog\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtconfig0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtconfig0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`wdtconfig0`] module"]
pub type WDTCONFIG0 = crate::Reg<wdtconfig0::WDTCONFIG0_SPEC>;
#[doc = "configure rtc watch dog"]
pub mod wdtconfig0;
#[doc = "WDTCONFIG1 (rw) register accessor: stage0 hold time\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtconfig1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtconfig1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`wdtconfig1`] module"]
pub type WDTCONFIG1 = crate::Reg<wdtconfig1::WDTCONFIG1_SPEC>;
#[doc = "stage0 hold time"]
pub mod wdtconfig1;
#[doc = "WDTCONFIG2 (rw) register accessor: stage1 hold time\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtconfig2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtconfig2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`wdtconfig2`] module"]
pub type WDTCONFIG2 = crate::Reg<wdtconfig2::WDTCONFIG2_SPEC>;
#[doc = "stage1 hold time"]
pub mod wdtconfig2;
#[doc = "WDTCONFIG3 (rw) register accessor: stage2 hold time\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtconfig3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtconfig3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`wdtconfig3`] module"]
pub type WDTCONFIG3 = crate::Reg<wdtconfig3::WDTCONFIG3_SPEC>;
#[doc = "stage2 hold time"]
pub mod wdtconfig3;
#[doc = "WDTCONFIG4 (rw) register accessor: stage3 hold time\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtconfig4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtconfig4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`wdtconfig4`] module"]
pub type WDTCONFIG4 = crate::Reg<wdtconfig4::WDTCONFIG4_SPEC>;
#[doc = "stage3 hold time"]
pub mod wdtconfig4;
#[doc = "WDTFEED (w) register accessor: rtc wdt feed\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtfeed::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`wdtfeed`] module"]
pub type WDTFEED = crate::Reg<wdtfeed::WDTFEED_SPEC>;
#[doc = "rtc wdt feed"]
pub mod wdtfeed;
#[doc = "WDTWPROTECT (rw) register accessor: configure rtc watch dog\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtwprotect::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtwprotect::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`wdtwprotect`] module"]
pub type WDTWPROTECT = crate::Reg<wdtwprotect::WDTWPROTECT_SPEC>;
#[doc = "configure rtc watch dog"]
pub mod wdtwprotect;
#[doc = "SWD_CONF (rw) register accessor: congfigure super watch dog\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swd_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swd_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`swd_conf`] module"]
pub type SWD_CONF = crate::Reg<swd_conf::SWD_CONF_SPEC>;
#[doc = "congfigure super watch dog"]
pub mod swd_conf;
#[doc = "SWD_WPROTECT (rw) register accessor: super watch dog key\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swd_wprotect::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swd_wprotect::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`swd_wprotect`] module"]
pub type SWD_WPROTECT = crate::Reg<swd_wprotect::SWD_WPROTECT_SPEC>;
#[doc = "super watch dog key"]
pub mod swd_wprotect;
#[doc = "SW_CPU_STALL (rw) register accessor: configure cpu stall by sw\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sw_cpu_stall::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sw_cpu_stall::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sw_cpu_stall`] module"]
pub type SW_CPU_STALL = crate::Reg<sw_cpu_stall::SW_CPU_STALL_SPEC>;
#[doc = "configure cpu stall by sw"]
pub mod sw_cpu_stall;
#[doc = "STORE4 (rw) register accessor: reserved register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`store4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`store4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`store4`] module"]
pub type STORE4 = crate::Reg<store4::STORE4_SPEC>;
#[doc = "reserved register"]
pub mod store4;
#[doc = "STORE5 (rw) register accessor: reserved register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`store5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`store5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`store5`] module"]
pub type STORE5 = crate::Reg<store5::STORE5_SPEC>;
#[doc = "reserved register"]
pub mod store5;
#[doc = "STORE6 (rw) register accessor: reserved register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`store6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`store6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`store6`] module"]
pub type STORE6 = crate::Reg<store6::STORE6_SPEC>;
#[doc = "reserved register"]
pub mod store6;
#[doc = "STORE7 (rw) register accessor: reserved register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`store7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`store7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`store7`] module"]
pub type STORE7 = crate::Reg<store7::STORE7_SPEC>;
#[doc = "reserved register"]
pub mod store7;
#[doc = "LOW_POWER_ST (r) register accessor: reserved register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`low_power_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`low_power_st`] module"]
pub type LOW_POWER_ST = crate::Reg<low_power_st::LOW_POWER_ST_SPEC>;
#[doc = "reserved register"]
pub mod low_power_st;
#[doc = "DIAG0 (r) register accessor: No public\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diag0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diag0`] module"]
pub type DIAG0 = crate::Reg<diag0::DIAG0_SPEC>;
#[doc = "No public"]
pub mod diag0;
#[doc = "PAD_HOLD (rw) register accessor: rtc pad hold configure\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pad_hold::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pad_hold::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pad_hold`] module"]
pub type PAD_HOLD = crate::Reg<pad_hold::PAD_HOLD_SPEC>;
#[doc = "rtc pad hold configure"]
pub mod pad_hold;
#[doc = "DIG_PAD_HOLD (rw) register accessor: configure digtal pad hold\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dig_pad_hold::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dig_pad_hold::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dig_pad_hold`] module"]
pub type DIG_PAD_HOLD = crate::Reg<dig_pad_hold::DIG_PAD_HOLD_SPEC>;
#[doc = "configure digtal pad hold"]
pub mod dig_pad_hold;
#[doc = "EXT_WAKEUP1 (rw) register accessor: configure ext1 wakeup\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ext_wakeup1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ext_wakeup1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ext_wakeup1`] module"]
pub type EXT_WAKEUP1 = crate::Reg<ext_wakeup1::EXT_WAKEUP1_SPEC>;
#[doc = "configure ext1 wakeup"]
pub mod ext_wakeup1;
#[doc = "EXT_WAKEUP1_STATUS (r) register accessor: check ext wakeup1 status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ext_wakeup1_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ext_wakeup1_status`] module"]
pub type EXT_WAKEUP1_STATUS = crate::Reg<ext_wakeup1_status::EXT_WAKEUP1_STATUS_SPEC>;
#[doc = "check ext wakeup1 status"]
pub mod ext_wakeup1_status;
#[doc = "BROWN_OUT (rw) register accessor: congfigure brownout\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`brown_out::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`brown_out::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`brown_out`] module"]
pub type BROWN_OUT = crate::Reg<brown_out::BROWN_OUT_SPEC>;
#[doc = "congfigure brownout"]
pub mod brown_out;
#[doc = "TIME_LOW1 (r) register accessor: RTC timer low 32 bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`time_low1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`time_low1`] module"]
pub type TIME_LOW1 = crate::Reg<time_low1::TIME_LOW1_SPEC>;
#[doc = "RTC timer low 32 bits"]
pub mod time_low1;
#[doc = "TIME_HIGH1 (r) register accessor: RTC timer high 16 bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`time_high1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`time_high1`] module"]
pub type TIME_HIGH1 = crate::Reg<time_high1::TIME_HIGH1_SPEC>;
#[doc = "RTC timer high 16 bits"]
pub mod time_high1;
#[doc = "XTAL32K_CLK_FACTOR (rw) register accessor: xtal 32k watch dog backup clock factor\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`xtal32k_clk_factor::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`xtal32k_clk_factor::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`xtal32k_clk_factor`] module"]
pub type XTAL32K_CLK_FACTOR = crate::Reg<xtal32k_clk_factor::XTAL32K_CLK_FACTOR_SPEC>;
#[doc = "xtal 32k watch dog backup clock factor"]
pub mod xtal32k_clk_factor;
#[doc = "XTAL32K_CONF (rw) register accessor: configure xtal32k\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`xtal32k_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`xtal32k_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`xtal32k_conf`] module"]
pub type XTAL32K_CONF = crate::Reg<xtal32k_conf::XTAL32K_CONF_SPEC>;
#[doc = "configure xtal32k"]
pub mod xtal32k_conf;
#[doc = "ULP_CP_TIMER (rw) register accessor: configure ulp\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ulp_cp_timer::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ulp_cp_timer::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ulp_cp_timer`] module"]
pub type ULP_CP_TIMER = crate::Reg<ulp_cp_timer::ULP_CP_TIMER_SPEC>;
#[doc = "configure ulp"]
pub mod ulp_cp_timer;
#[doc = "ULP_CP_CTRL (rw) register accessor: configure ulp\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ulp_cp_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ulp_cp_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ulp_cp_ctrl`] module"]
pub type ULP_CP_CTRL = crate::Reg<ulp_cp_ctrl::ULP_CP_CTRL_SPEC>;
#[doc = "configure ulp"]
pub mod ulp_cp_ctrl;
#[doc = "COCPU_CTRL (rw) register accessor: configure ulp-riscv\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cocpu_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cocpu_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cocpu_ctrl`] module"]
pub type COCPU_CTRL = crate::Reg<cocpu_ctrl::COCPU_CTRL_SPEC>;
#[doc = "configure ulp-riscv"]
pub mod cocpu_ctrl;
#[doc = "TOUCH_CTRL1 (rw) register accessor: configure touch controller\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`touch_ctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`touch_ctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`touch_ctrl1`] module"]
pub type TOUCH_CTRL1 = crate::Reg<touch_ctrl1::TOUCH_CTRL1_SPEC>;
#[doc = "configure touch controller"]
pub mod touch_ctrl1;
#[doc = "TOUCH_CTRL2 (rw) register accessor: configure touch controller\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`touch_ctrl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`touch_ctrl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`touch_ctrl2`] module"]
pub type TOUCH_CTRL2 = crate::Reg<touch_ctrl2::TOUCH_CTRL2_SPEC>;
#[doc = "configure touch controller"]
pub mod touch_ctrl2;
#[doc = "TOUCH_SCAN_CTRL (rw) register accessor: configure touch controller\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`touch_scan_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`touch_scan_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`touch_scan_ctrl`] module"]
pub type TOUCH_SCAN_CTRL = crate::Reg<touch_scan_ctrl::TOUCH_SCAN_CTRL_SPEC>;
#[doc = "configure touch controller"]
pub mod touch_scan_ctrl;
#[doc = "TOUCH_SLP_THRES (rw) register accessor: configure touch controller\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`touch_slp_thres::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`touch_slp_thres::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`touch_slp_thres`] module"]
pub type TOUCH_SLP_THRES = crate::Reg<touch_slp_thres::TOUCH_SLP_THRES_SPEC>;
#[doc = "configure touch controller"]
pub mod touch_slp_thres;
#[doc = "TOUCH_APPROACH (rw) register accessor: configure touch controller\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`touch_approach::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`touch_approach::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`touch_approach`] module"]
pub type TOUCH_APPROACH = crate::Reg<touch_approach::TOUCH_APPROACH_SPEC>;
#[doc = "configure touch controller"]
pub mod touch_approach;
#[doc = "TOUCH_FILTER_CTRL (rw) register accessor: configure touch controller\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`touch_filter_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`touch_filter_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`touch_filter_ctrl`] module"]
pub type TOUCH_FILTER_CTRL = crate::Reg<touch_filter_ctrl::TOUCH_FILTER_CTRL_SPEC>;
#[doc = "configure touch controller"]
pub mod touch_filter_ctrl;
#[doc = "USB_CONF (rw) register accessor: usb configure\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`usb_conf`] module"]
pub type USB_CONF = crate::Reg<usb_conf::USB_CONF_SPEC>;
#[doc = "usb configure"]
pub mod usb_conf;
#[doc = "TOUCH_TIMEOUT_CTRL (rw) register accessor: configure touch controller\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`touch_timeout_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`touch_timeout_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`touch_timeout_ctrl`] module"]
pub type TOUCH_TIMEOUT_CTRL = crate::Reg<touch_timeout_ctrl::TOUCH_TIMEOUT_CTRL_SPEC>;
#[doc = "configure touch controller"]
pub mod touch_timeout_ctrl;
#[doc = "SLP_REJECT_CAUSE (r) register accessor: get reject casue\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slp_reject_cause::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`slp_reject_cause`] module"]
pub type SLP_REJECT_CAUSE = crate::Reg<slp_reject_cause::SLP_REJECT_CAUSE_SPEC>;
#[doc = "get reject casue"]
pub mod slp_reject_cause;
#[doc = "OPTION1 (rw) register accessor: rtc common configure\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`option1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`option1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`option1`] module"]
pub type OPTION1 = crate::Reg<option1::OPTION1_SPEC>;
#[doc = "rtc common configure"]
pub mod option1;
#[doc = "SLP_WAKEUP_CAUSE (r) register accessor: get wakeup cause\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slp_wakeup_cause::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`slp_wakeup_cause`] module"]
pub type SLP_WAKEUP_CAUSE = crate::Reg<slp_wakeup_cause::SLP_WAKEUP_CAUSE_SPEC>;
#[doc = "get wakeup cause"]
pub mod slp_wakeup_cause;
#[doc = "ULP_CP_TIMER_1 (rw) register accessor: configure ulp sleep time\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ulp_cp_timer_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ulp_cp_timer_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ulp_cp_timer_1`] module"]
pub type ULP_CP_TIMER_1 = crate::Reg<ulp_cp_timer_1::ULP_CP_TIMER_1_SPEC>;
#[doc = "configure ulp sleep time"]
pub mod ulp_cp_timer_1;
#[doc = "INT_ENA_RTC_W1TS (w) register accessor: oneset rtc interrupt\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena_rtc_w1ts::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`int_ena_rtc_w1ts`] module"]
pub type INT_ENA_RTC_W1TS = crate::Reg<int_ena_rtc_w1ts::INT_ENA_RTC_W1TS_SPEC>;
#[doc = "oneset rtc interrupt"]
pub mod int_ena_rtc_w1ts;
#[doc = "INT_ENA_RTC_W1TC (w) register accessor: oneset clr rtc interrupt enable\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena_rtc_w1tc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`int_ena_rtc_w1tc`] module"]
pub type INT_ENA_RTC_W1TC = crate::Reg<int_ena_rtc_w1tc::INT_ENA_RTC_W1TC_SPEC>;
#[doc = "oneset clr rtc interrupt enable"]
pub mod int_ena_rtc_w1tc;
#[doc = "RETENTION_CTRL (rw) register accessor: configure retention\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`retention_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`retention_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`retention_ctrl`] module"]
pub type RETENTION_CTRL = crate::Reg<retention_ctrl::RETENTION_CTRL_SPEC>;
#[doc = "configure retention"]
pub mod retention_ctrl;
#[doc = "PG_CTRL (rw) register accessor: configure power glitch\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pg_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pg_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pg_ctrl`] module"]
pub type PG_CTRL = crate::Reg<pg_ctrl::PG_CTRL_SPEC>;
#[doc = "configure power glitch"]
pub mod pg_ctrl;
#[doc = "FIB_SEL (rw) register accessor: No public\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fib_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fib_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`fib_sel`] module"]
pub type FIB_SEL = crate::Reg<fib_sel::FIB_SEL_SPEC>;
#[doc = "No public"]
pub mod fib_sel;
#[doc = "TOUCH_DAC (rw) register accessor: configure touch dac\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`touch_dac::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`touch_dac::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`touch_dac`] module"]
pub type TOUCH_DAC = crate::Reg<touch_dac::TOUCH_DAC_SPEC>;
#[doc = "configure touch dac"]
pub mod touch_dac;
#[doc = "TOUCH_DAC1 (rw) register accessor: configure touch dac\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`touch_dac1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`touch_dac1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`touch_dac1`] module"]
pub type TOUCH_DAC1 = crate::Reg<touch_dac1::TOUCH_DAC1_SPEC>;
#[doc = "configure touch dac"]
pub mod touch_dac1;
#[doc = "COCPU_DISABLE (rw) register accessor: configure ulp diable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cocpu_disable::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cocpu_disable::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cocpu_disable`] module"]
pub type COCPU_DISABLE = crate::Reg<cocpu_disable::COCPU_DISABLE_SPEC>;
#[doc = "configure ulp diable"]
pub mod cocpu_disable;
#[doc = "DATE (rw) register accessor: version register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "version register"]
pub mod date;
