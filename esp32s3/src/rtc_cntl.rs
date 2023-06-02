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
#[doc = "OPTIONS0 (rw) register accessor: an alias for `Reg<OPTIONS0_SPEC>`"]
pub type OPTIONS0 = crate::Reg<options0::OPTIONS0_SPEC>;
#[doc = "RTC common configure register"]
pub mod options0;
#[doc = "SLP_TIMER0 (rw) register accessor: an alias for `Reg<SLP_TIMER0_SPEC>`"]
pub type SLP_TIMER0 = crate::Reg<slp_timer0::SLP_TIMER0_SPEC>;
#[doc = "configure min sleep time"]
pub mod slp_timer0;
#[doc = "SLP_TIMER1 (rw) register accessor: an alias for `Reg<SLP_TIMER1_SPEC>`"]
pub type SLP_TIMER1 = crate::Reg<slp_timer1::SLP_TIMER1_SPEC>;
#[doc = "configure sleep time hi"]
pub mod slp_timer1;
#[doc = "TIME_UPDATE (rw) register accessor: an alias for `Reg<TIME_UPDATE_SPEC>`"]
pub type TIME_UPDATE = crate::Reg<time_update::TIME_UPDATE_SPEC>;
#[doc = "update rtc main timer"]
pub mod time_update;
#[doc = "TIME_LOW0 (r) register accessor: an alias for `Reg<TIME_LOW0_SPEC>`"]
pub type TIME_LOW0 = crate::Reg<time_low0::TIME_LOW0_SPEC>;
#[doc = "read rtc_main timer low bits"]
pub mod time_low0;
#[doc = "TIME_HIGH0 (r) register accessor: an alias for `Reg<TIME_HIGH0_SPEC>`"]
pub type TIME_HIGH0 = crate::Reg<time_high0::TIME_HIGH0_SPEC>;
#[doc = "read rtc_main timer high bits"]
pub mod time_high0;
#[doc = "STATE0 (rw) register accessor: an alias for `Reg<STATE0_SPEC>`"]
pub type STATE0 = crate::Reg<state0::STATE0_SPEC>;
#[doc = "configure chip sleep"]
pub mod state0;
#[doc = "TIMER1 (rw) register accessor: an alias for `Reg<TIMER1_SPEC>`"]
pub type TIMER1 = crate::Reg<timer1::TIMER1_SPEC>;
#[doc = "rtc state wait time"]
pub mod timer1;
#[doc = "TIMER2 (rw) register accessor: an alias for `Reg<TIMER2_SPEC>`"]
pub type TIMER2 = crate::Reg<timer2::TIMER2_SPEC>;
#[doc = "rtc monitor state delay time"]
pub mod timer2;
#[doc = "TIMER3 (rw) register accessor: an alias for `Reg<TIMER3_SPEC>`"]
pub type TIMER3 = crate::Reg<timer3::TIMER3_SPEC>;
#[doc = "No public"]
pub mod timer3;
#[doc = "TIMER4 (rw) register accessor: an alias for `Reg<TIMER4_SPEC>`"]
pub type TIMER4 = crate::Reg<timer4::TIMER4_SPEC>;
#[doc = "No public"]
pub mod timer4;
#[doc = "TIMER5 (rw) register accessor: an alias for `Reg<TIMER5_SPEC>`"]
pub type TIMER5 = crate::Reg<timer5::TIMER5_SPEC>;
#[doc = "configure min sleep time"]
pub mod timer5;
#[doc = "TIMER6 (rw) register accessor: an alias for `Reg<TIMER6_SPEC>`"]
pub type TIMER6 = crate::Reg<timer6::TIMER6_SPEC>;
#[doc = "No public"]
pub mod timer6;
#[doc = "ANA_CONF (rw) register accessor: an alias for `Reg<ANA_CONF_SPEC>`"]
pub type ANA_CONF = crate::Reg<ana_conf::ANA_CONF_SPEC>;
#[doc = "analog configure register"]
pub mod ana_conf;
#[doc = "RESET_STATE (rw) register accessor: an alias for `Reg<RESET_STATE_SPEC>`"]
pub type RESET_STATE = crate::Reg<reset_state::RESET_STATE_SPEC>;
#[doc = "get reset state"]
pub mod reset_state;
#[doc = "WAKEUP_STATE (rw) register accessor: an alias for `Reg<WAKEUP_STATE_SPEC>`"]
pub type WAKEUP_STATE = crate::Reg<wakeup_state::WAKEUP_STATE_SPEC>;
#[doc = "configure wakeup state"]
pub mod wakeup_state;
#[doc = "INT_ENA_RTC (rw) register accessor: an alias for `Reg<INT_ENA_RTC_SPEC>`"]
pub type INT_ENA_RTC = crate::Reg<int_ena_rtc::INT_ENA_RTC_SPEC>;
#[doc = "configure rtc interrupt register"]
pub mod int_ena_rtc;
#[doc = "INT_RAW_RTC (rw) register accessor: an alias for `Reg<INT_RAW_RTC_SPEC>`"]
pub type INT_RAW_RTC = crate::Reg<int_raw_rtc::INT_RAW_RTC_SPEC>;
#[doc = "rtc interrupt register"]
pub mod int_raw_rtc;
#[doc = "INT_ST_RTC (r) register accessor: an alias for `Reg<INT_ST_RTC_SPEC>`"]
pub type INT_ST_RTC = crate::Reg<int_st_rtc::INT_ST_RTC_SPEC>;
#[doc = "rtc interrupt register"]
pub mod int_st_rtc;
#[doc = "INT_CLR_RTC (w) register accessor: an alias for `Reg<INT_CLR_RTC_SPEC>`"]
pub type INT_CLR_RTC = crate::Reg<int_clr_rtc::INT_CLR_RTC_SPEC>;
#[doc = "rtc interrupt register"]
pub mod int_clr_rtc;
#[doc = "STORE0 (rw) register accessor: an alias for `Reg<STORE0_SPEC>`"]
pub type STORE0 = crate::Reg<store0::STORE0_SPEC>;
#[doc = "Reserved register"]
pub mod store0;
#[doc = "STORE1 (rw) register accessor: an alias for `Reg<STORE1_SPEC>`"]
pub type STORE1 = crate::Reg<store1::STORE1_SPEC>;
#[doc = "Reserved register"]
pub mod store1;
#[doc = "STORE2 (rw) register accessor: an alias for `Reg<STORE2_SPEC>`"]
pub type STORE2 = crate::Reg<store2::STORE2_SPEC>;
#[doc = "Reserved register"]
pub mod store2;
#[doc = "STORE3 (rw) register accessor: an alias for `Reg<STORE3_SPEC>`"]
pub type STORE3 = crate::Reg<store3::STORE3_SPEC>;
#[doc = "Reserved register"]
pub mod store3;
#[doc = "EXT_XTL_CONF (rw) register accessor: an alias for `Reg<EXT_XTL_CONF_SPEC>`"]
pub type EXT_XTL_CONF = crate::Reg<ext_xtl_conf::EXT_XTL_CONF_SPEC>;
#[doc = "Reserved register"]
pub mod ext_xtl_conf;
#[doc = "EXT_WAKEUP_CONF (rw) register accessor: an alias for `Reg<EXT_WAKEUP_CONF_SPEC>`"]
pub type EXT_WAKEUP_CONF = crate::Reg<ext_wakeup_conf::EXT_WAKEUP_CONF_SPEC>;
#[doc = "ext wakeup configure"]
pub mod ext_wakeup_conf;
#[doc = "SLP_REJECT_CONF (rw) register accessor: an alias for `Reg<SLP_REJECT_CONF_SPEC>`"]
pub type SLP_REJECT_CONF = crate::Reg<slp_reject_conf::SLP_REJECT_CONF_SPEC>;
#[doc = "reject sleep register"]
pub mod slp_reject_conf;
#[doc = "CPU_PERIOD_CONF (rw) register accessor: an alias for `Reg<CPU_PERIOD_CONF_SPEC>`"]
pub type CPU_PERIOD_CONF = crate::Reg<cpu_period_conf::CPU_PERIOD_CONF_SPEC>;
#[doc = "conigure cpu freq"]
pub mod cpu_period_conf;
#[doc = "SDIO_ACT_CONF (rw) register accessor: an alias for `Reg<SDIO_ACT_CONF_SPEC>`"]
pub type SDIO_ACT_CONF = crate::Reg<sdio_act_conf::SDIO_ACT_CONF_SPEC>;
#[doc = "No public"]
pub mod sdio_act_conf;
#[doc = "CLK_CONF (rw) register accessor: an alias for `Reg<CLK_CONF_SPEC>`"]
pub type CLK_CONF = crate::Reg<clk_conf::CLK_CONF_SPEC>;
#[doc = "configure clock register"]
pub mod clk_conf;
#[doc = "SLOW_CLK_CONF (rw) register accessor: an alias for `Reg<SLOW_CLK_CONF_SPEC>`"]
pub type SLOW_CLK_CONF = crate::Reg<slow_clk_conf::SLOW_CLK_CONF_SPEC>;
#[doc = "configure slow clk"]
pub mod slow_clk_conf;
#[doc = "SDIO_CONF (rw) register accessor: an alias for `Reg<SDIO_CONF_SPEC>`"]
pub type SDIO_CONF = crate::Reg<sdio_conf::SDIO_CONF_SPEC>;
#[doc = "configure flash power"]
pub mod sdio_conf;
#[doc = "BIAS_CONF (rw) register accessor: an alias for `Reg<BIAS_CONF_SPEC>`"]
pub type BIAS_CONF = crate::Reg<bias_conf::BIAS_CONF_SPEC>;
#[doc = "No public"]
pub mod bias_conf;
#[doc = "RTC (rw) register accessor: an alias for `Reg<RTC_SPEC>`"]
pub type RTC = crate::Reg<rtc::RTC_SPEC>;
#[doc = "configure rtc regulator"]
pub mod rtc;
#[doc = "PWC (rw) register accessor: an alias for `Reg<PWC_SPEC>`"]
pub type PWC = crate::Reg<pwc::PWC_SPEC>;
#[doc = "configure rtc power"]
pub mod pwc;
#[doc = "REGULATOR_DRV_CTRL (rw) register accessor: an alias for `Reg<REGULATOR_DRV_CTRL_SPEC>`"]
pub type REGULATOR_DRV_CTRL = crate::Reg<regulator_drv_ctrl::REGULATOR_DRV_CTRL_SPEC>;
#[doc = "No public"]
pub mod regulator_drv_ctrl;
#[doc = "DIG_PWC (rw) register accessor: an alias for `Reg<DIG_PWC_SPEC>`"]
pub type DIG_PWC = crate::Reg<dig_pwc::DIG_PWC_SPEC>;
#[doc = "configure digital power"]
pub mod dig_pwc;
#[doc = "DIG_ISO (rw) register accessor: an alias for `Reg<DIG_ISO_SPEC>`"]
pub type DIG_ISO = crate::Reg<dig_iso::DIG_ISO_SPEC>;
#[doc = "congigure digital power isolation"]
pub mod dig_iso;
#[doc = "WDTCONFIG0 (rw) register accessor: an alias for `Reg<WDTCONFIG0_SPEC>`"]
pub type WDTCONFIG0 = crate::Reg<wdtconfig0::WDTCONFIG0_SPEC>;
#[doc = "configure rtc watch dog"]
pub mod wdtconfig0;
#[doc = "WDTCONFIG1 (rw) register accessor: an alias for `Reg<WDTCONFIG1_SPEC>`"]
pub type WDTCONFIG1 = crate::Reg<wdtconfig1::WDTCONFIG1_SPEC>;
#[doc = "stage0 hold time"]
pub mod wdtconfig1;
#[doc = "WDTCONFIG2 (rw) register accessor: an alias for `Reg<WDTCONFIG2_SPEC>`"]
pub type WDTCONFIG2 = crate::Reg<wdtconfig2::WDTCONFIG2_SPEC>;
#[doc = "stage1 hold time"]
pub mod wdtconfig2;
#[doc = "WDTCONFIG3 (rw) register accessor: an alias for `Reg<WDTCONFIG3_SPEC>`"]
pub type WDTCONFIG3 = crate::Reg<wdtconfig3::WDTCONFIG3_SPEC>;
#[doc = "stage2 hold time"]
pub mod wdtconfig3;
#[doc = "WDTCONFIG4 (rw) register accessor: an alias for `Reg<WDTCONFIG4_SPEC>`"]
pub type WDTCONFIG4 = crate::Reg<wdtconfig4::WDTCONFIG4_SPEC>;
#[doc = "stage3 hold time"]
pub mod wdtconfig4;
#[doc = "WDTFEED (w) register accessor: an alias for `Reg<WDTFEED_SPEC>`"]
pub type WDTFEED = crate::Reg<wdtfeed::WDTFEED_SPEC>;
#[doc = "rtc wdt feed"]
pub mod wdtfeed;
#[doc = "WDTWPROTECT (rw) register accessor: an alias for `Reg<WDTWPROTECT_SPEC>`"]
pub type WDTWPROTECT = crate::Reg<wdtwprotect::WDTWPROTECT_SPEC>;
#[doc = "configure rtc watch dog"]
pub mod wdtwprotect;
#[doc = "SWD_CONF (rw) register accessor: an alias for `Reg<SWD_CONF_SPEC>`"]
pub type SWD_CONF = crate::Reg<swd_conf::SWD_CONF_SPEC>;
#[doc = "congfigure super watch dog"]
pub mod swd_conf;
#[doc = "SWD_WPROTECT (rw) register accessor: an alias for `Reg<SWD_WPROTECT_SPEC>`"]
pub type SWD_WPROTECT = crate::Reg<swd_wprotect::SWD_WPROTECT_SPEC>;
#[doc = "super watch dog key"]
pub mod swd_wprotect;
#[doc = "SW_CPU_STALL (rw) register accessor: an alias for `Reg<SW_CPU_STALL_SPEC>`"]
pub type SW_CPU_STALL = crate::Reg<sw_cpu_stall::SW_CPU_STALL_SPEC>;
#[doc = "configure cpu stall by sw"]
pub mod sw_cpu_stall;
#[doc = "STORE4 (rw) register accessor: an alias for `Reg<STORE4_SPEC>`"]
pub type STORE4 = crate::Reg<store4::STORE4_SPEC>;
#[doc = "reserved register"]
pub mod store4;
#[doc = "STORE5 (rw) register accessor: an alias for `Reg<STORE5_SPEC>`"]
pub type STORE5 = crate::Reg<store5::STORE5_SPEC>;
#[doc = "reserved register"]
pub mod store5;
#[doc = "STORE6 (rw) register accessor: an alias for `Reg<STORE6_SPEC>`"]
pub type STORE6 = crate::Reg<store6::STORE6_SPEC>;
#[doc = "reserved register"]
pub mod store6;
#[doc = "STORE7 (rw) register accessor: an alias for `Reg<STORE7_SPEC>`"]
pub type STORE7 = crate::Reg<store7::STORE7_SPEC>;
#[doc = "reserved register"]
pub mod store7;
#[doc = "LOW_POWER_ST (r) register accessor: an alias for `Reg<LOW_POWER_ST_SPEC>`"]
pub type LOW_POWER_ST = crate::Reg<low_power_st::LOW_POWER_ST_SPEC>;
#[doc = "reserved register"]
pub mod low_power_st;
#[doc = "DIAG0 (r) register accessor: an alias for `Reg<DIAG0_SPEC>`"]
pub type DIAG0 = crate::Reg<diag0::DIAG0_SPEC>;
#[doc = "No public"]
pub mod diag0;
#[doc = "PAD_HOLD (rw) register accessor: an alias for `Reg<PAD_HOLD_SPEC>`"]
pub type PAD_HOLD = crate::Reg<pad_hold::PAD_HOLD_SPEC>;
#[doc = "rtc pad hold configure"]
pub mod pad_hold;
#[doc = "DIG_PAD_HOLD (rw) register accessor: an alias for `Reg<DIG_PAD_HOLD_SPEC>`"]
pub type DIG_PAD_HOLD = crate::Reg<dig_pad_hold::DIG_PAD_HOLD_SPEC>;
#[doc = "configure digtal pad hold"]
pub mod dig_pad_hold;
#[doc = "EXT_WAKEUP1 (rw) register accessor: an alias for `Reg<EXT_WAKEUP1_SPEC>`"]
pub type EXT_WAKEUP1 = crate::Reg<ext_wakeup1::EXT_WAKEUP1_SPEC>;
#[doc = "configure ext1 wakeup"]
pub mod ext_wakeup1;
#[doc = "EXT_WAKEUP1_STATUS (r) register accessor: an alias for `Reg<EXT_WAKEUP1_STATUS_SPEC>`"]
pub type EXT_WAKEUP1_STATUS = crate::Reg<ext_wakeup1_status::EXT_WAKEUP1_STATUS_SPEC>;
#[doc = "check ext wakeup1 status"]
pub mod ext_wakeup1_status;
#[doc = "BROWN_OUT (rw) register accessor: an alias for `Reg<BROWN_OUT_SPEC>`"]
pub type BROWN_OUT = crate::Reg<brown_out::BROWN_OUT_SPEC>;
#[doc = "congfigure brownout"]
pub mod brown_out;
#[doc = "TIME_LOW1 (r) register accessor: an alias for `Reg<TIME_LOW1_SPEC>`"]
pub type TIME_LOW1 = crate::Reg<time_low1::TIME_LOW1_SPEC>;
#[doc = "RTC timer low 32 bits"]
pub mod time_low1;
#[doc = "TIME_HIGH1 (r) register accessor: an alias for `Reg<TIME_HIGH1_SPEC>`"]
pub type TIME_HIGH1 = crate::Reg<time_high1::TIME_HIGH1_SPEC>;
#[doc = "RTC timer high 16 bits"]
pub mod time_high1;
#[doc = "XTAL32K_CLK_FACTOR (rw) register accessor: an alias for `Reg<XTAL32K_CLK_FACTOR_SPEC>`"]
pub type XTAL32K_CLK_FACTOR = crate::Reg<xtal32k_clk_factor::XTAL32K_CLK_FACTOR_SPEC>;
#[doc = "xtal 32k watch dog backup clock factor"]
pub mod xtal32k_clk_factor;
#[doc = "XTAL32K_CONF (rw) register accessor: an alias for `Reg<XTAL32K_CONF_SPEC>`"]
pub type XTAL32K_CONF = crate::Reg<xtal32k_conf::XTAL32K_CONF_SPEC>;
#[doc = "configure xtal32k"]
pub mod xtal32k_conf;
#[doc = "ULP_CP_TIMER (rw) register accessor: an alias for `Reg<ULP_CP_TIMER_SPEC>`"]
pub type ULP_CP_TIMER = crate::Reg<ulp_cp_timer::ULP_CP_TIMER_SPEC>;
#[doc = "configure ulp"]
pub mod ulp_cp_timer;
#[doc = "ULP_CP_CTRL (rw) register accessor: an alias for `Reg<ULP_CP_CTRL_SPEC>`"]
pub type ULP_CP_CTRL = crate::Reg<ulp_cp_ctrl::ULP_CP_CTRL_SPEC>;
#[doc = "configure ulp"]
pub mod ulp_cp_ctrl;
#[doc = "COCPU_CTRL (rw) register accessor: an alias for `Reg<COCPU_CTRL_SPEC>`"]
pub type COCPU_CTRL = crate::Reg<cocpu_ctrl::COCPU_CTRL_SPEC>;
#[doc = "configure ulp-riscv"]
pub mod cocpu_ctrl;
#[doc = "TOUCH_CTRL1 (rw) register accessor: an alias for `Reg<TOUCH_CTRL1_SPEC>`"]
pub type TOUCH_CTRL1 = crate::Reg<touch_ctrl1::TOUCH_CTRL1_SPEC>;
#[doc = "configure touch controller"]
pub mod touch_ctrl1;
#[doc = "TOUCH_CTRL2 (rw) register accessor: an alias for `Reg<TOUCH_CTRL2_SPEC>`"]
pub type TOUCH_CTRL2 = crate::Reg<touch_ctrl2::TOUCH_CTRL2_SPEC>;
#[doc = "configure touch controller"]
pub mod touch_ctrl2;
#[doc = "TOUCH_SCAN_CTRL (rw) register accessor: an alias for `Reg<TOUCH_SCAN_CTRL_SPEC>`"]
pub type TOUCH_SCAN_CTRL = crate::Reg<touch_scan_ctrl::TOUCH_SCAN_CTRL_SPEC>;
#[doc = "configure touch controller"]
pub mod touch_scan_ctrl;
#[doc = "TOUCH_SLP_THRES (rw) register accessor: an alias for `Reg<TOUCH_SLP_THRES_SPEC>`"]
pub type TOUCH_SLP_THRES = crate::Reg<touch_slp_thres::TOUCH_SLP_THRES_SPEC>;
#[doc = "configure touch controller"]
pub mod touch_slp_thres;
#[doc = "TOUCH_APPROACH (rw) register accessor: an alias for `Reg<TOUCH_APPROACH_SPEC>`"]
pub type TOUCH_APPROACH = crate::Reg<touch_approach::TOUCH_APPROACH_SPEC>;
#[doc = "configure touch controller"]
pub mod touch_approach;
#[doc = "TOUCH_FILTER_CTRL (rw) register accessor: an alias for `Reg<TOUCH_FILTER_CTRL_SPEC>`"]
pub type TOUCH_FILTER_CTRL = crate::Reg<touch_filter_ctrl::TOUCH_FILTER_CTRL_SPEC>;
#[doc = "configure touch controller"]
pub mod touch_filter_ctrl;
#[doc = "USB_CONF (rw) register accessor: an alias for `Reg<USB_CONF_SPEC>`"]
pub type USB_CONF = crate::Reg<usb_conf::USB_CONF_SPEC>;
#[doc = "usb configure"]
pub mod usb_conf;
#[doc = "TOUCH_TIMEOUT_CTRL (rw) register accessor: an alias for `Reg<TOUCH_TIMEOUT_CTRL_SPEC>`"]
pub type TOUCH_TIMEOUT_CTRL = crate::Reg<touch_timeout_ctrl::TOUCH_TIMEOUT_CTRL_SPEC>;
#[doc = "configure touch controller"]
pub mod touch_timeout_ctrl;
#[doc = "SLP_REJECT_CAUSE (r) register accessor: an alias for `Reg<SLP_REJECT_CAUSE_SPEC>`"]
pub type SLP_REJECT_CAUSE = crate::Reg<slp_reject_cause::SLP_REJECT_CAUSE_SPEC>;
#[doc = "get reject casue"]
pub mod slp_reject_cause;
#[doc = "OPTION1 (rw) register accessor: an alias for `Reg<OPTION1_SPEC>`"]
pub type OPTION1 = crate::Reg<option1::OPTION1_SPEC>;
#[doc = "rtc common configure"]
pub mod option1;
#[doc = "SLP_WAKEUP_CAUSE (r) register accessor: an alias for `Reg<SLP_WAKEUP_CAUSE_SPEC>`"]
pub type SLP_WAKEUP_CAUSE = crate::Reg<slp_wakeup_cause::SLP_WAKEUP_CAUSE_SPEC>;
#[doc = "get wakeup cause"]
pub mod slp_wakeup_cause;
#[doc = "ULP_CP_TIMER_1 (rw) register accessor: an alias for `Reg<ULP_CP_TIMER_1_SPEC>`"]
pub type ULP_CP_TIMER_1 = crate::Reg<ulp_cp_timer_1::ULP_CP_TIMER_1_SPEC>;
#[doc = "configure ulp sleep time"]
pub mod ulp_cp_timer_1;
#[doc = "INT_ENA_RTC_W1TS (w) register accessor: an alias for `Reg<INT_ENA_RTC_W1TS_SPEC>`"]
pub type INT_ENA_RTC_W1TS = crate::Reg<int_ena_rtc_w1ts::INT_ENA_RTC_W1TS_SPEC>;
#[doc = "oneset rtc interrupt"]
pub mod int_ena_rtc_w1ts;
#[doc = "INT_ENA_RTC_W1TC (w) register accessor: an alias for `Reg<INT_ENA_RTC_W1TC_SPEC>`"]
pub type INT_ENA_RTC_W1TC = crate::Reg<int_ena_rtc_w1tc::INT_ENA_RTC_W1TC_SPEC>;
#[doc = "oneset clr rtc interrupt enable"]
pub mod int_ena_rtc_w1tc;
#[doc = "RETENTION_CTRL (rw) register accessor: an alias for `Reg<RETENTION_CTRL_SPEC>`"]
pub type RETENTION_CTRL = crate::Reg<retention_ctrl::RETENTION_CTRL_SPEC>;
#[doc = "configure retention"]
pub mod retention_ctrl;
#[doc = "PG_CTRL (rw) register accessor: an alias for `Reg<PG_CTRL_SPEC>`"]
pub type PG_CTRL = crate::Reg<pg_ctrl::PG_CTRL_SPEC>;
#[doc = "configure power glitch"]
pub mod pg_ctrl;
#[doc = "FIB_SEL (rw) register accessor: an alias for `Reg<FIB_SEL_SPEC>`"]
pub type FIB_SEL = crate::Reg<fib_sel::FIB_SEL_SPEC>;
#[doc = "No public"]
pub mod fib_sel;
#[doc = "TOUCH_DAC (rw) register accessor: an alias for `Reg<TOUCH_DAC_SPEC>`"]
pub type TOUCH_DAC = crate::Reg<touch_dac::TOUCH_DAC_SPEC>;
#[doc = "configure touch dac"]
pub mod touch_dac;
#[doc = "TOUCH_DAC1 (rw) register accessor: an alias for `Reg<TOUCH_DAC1_SPEC>`"]
pub type TOUCH_DAC1 = crate::Reg<touch_dac1::TOUCH_DAC1_SPEC>;
#[doc = "configure touch dac"]
pub mod touch_dac1;
#[doc = "COCPU_DISABLE (rw) register accessor: an alias for `Reg<COCPU_DISABLE_SPEC>`"]
pub type COCPU_DISABLE = crate::Reg<cocpu_disable::COCPU_DISABLE_SPEC>;
#[doc = "configure ulp diable"]
pub mod cocpu_disable;
#[doc = "DATE (rw) register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "version register"]
pub mod date;
