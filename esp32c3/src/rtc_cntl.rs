#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - rtc configure register"]
    pub options0: OPTIONS0,
    #[doc = "0x04 - rtc configure register"]
    pub slp_timer0: SLP_TIMER0,
    #[doc = "0x08 - rtc configure register"]
    pub slp_timer1: SLP_TIMER1,
    #[doc = "0x0c - rtc configure register"]
    pub time_update: TIME_UPDATE,
    #[doc = "0x10 - rtc configure register"]
    pub time_low0: TIME_LOW0,
    #[doc = "0x14 - rtc configure register"]
    pub time_high0: TIME_HIGH0,
    #[doc = "0x18 - rtc configure register"]
    pub state0: STATE0,
    #[doc = "0x1c - rtc configure register"]
    pub timer1: TIMER1,
    #[doc = "0x20 - rtc configure register"]
    pub timer2: TIMER2,
    #[doc = "0x24 - rtc configure register"]
    pub timer3: TIMER3,
    #[doc = "0x28 - rtc configure register"]
    pub timer4: TIMER4,
    #[doc = "0x2c - rtc configure register"]
    pub timer5: TIMER5,
    #[doc = "0x30 - rtc configure register"]
    pub timer6: TIMER6,
    #[doc = "0x34 - rtc configure register"]
    pub ana_conf: ANA_CONF,
    #[doc = "0x38 - rtc configure register"]
    pub reset_state: RESET_STATE,
    #[doc = "0x3c - rtc configure register"]
    pub wakeup_state: WAKEUP_STATE,
    #[doc = "0x40 - rtc configure register"]
    pub int_ena_rtc: INT_ENA_RTC,
    #[doc = "0x44 - rtc configure register"]
    pub int_raw_rtc: INT_RAW_RTC,
    #[doc = "0x48 - rtc configure register"]
    pub int_st_rtc: INT_ST_RTC,
    #[doc = "0x4c - rtc configure register"]
    pub int_clr_rtc: INT_CLR_RTC,
    #[doc = "0x50 - rtc configure register"]
    pub store0: STORE0,
    #[doc = "0x54 - rtc configure register"]
    pub store1: STORE1,
    #[doc = "0x58 - rtc configure register"]
    pub store2: STORE2,
    #[doc = "0x5c - rtc configure register"]
    pub store3: STORE3,
    #[doc = "0x60 - rtc configure register"]
    pub ext_xtl_conf: EXT_XTL_CONF,
    #[doc = "0x64 - rtc configure register"]
    pub ext_wakeup_conf: EXT_WAKEUP_CONF,
    #[doc = "0x68 - rtc configure register"]
    pub slp_reject_conf: SLP_REJECT_CONF,
    #[doc = "0x6c - rtc configure register"]
    pub cpu_period_conf: CPU_PERIOD_CONF,
    #[doc = "0x70 - rtc configure register"]
    pub clk_conf: CLK_CONF,
    #[doc = "0x74 - rtc configure register"]
    pub slow_clk_conf: SLOW_CLK_CONF,
    #[doc = "0x78 - rtc configure register"]
    pub sdio_conf: SDIO_CONF,
    #[doc = "0x7c - rtc configure register"]
    pub bias_conf: BIAS_CONF,
    #[doc = "0x80 - rtc configure register"]
    pub rtc_cntl: RTC_CNTL,
    #[doc = "0x84 - rtc configure register"]
    pub pwc: PWC,
    #[doc = "0x88 - rtc configure register"]
    pub dig_pwc: DIG_PWC,
    #[doc = "0x8c - rtc configure register"]
    pub dig_iso: DIG_ISO,
    #[doc = "0x90 - rtc configure register"]
    pub wdtconfig0: WDTCONFIG0,
    #[doc = "0x94 - rtc configure register"]
    pub wdtconfig1: WDTCONFIG1,
    #[doc = "0x98 - rtc configure register"]
    pub wdtconfig2: WDTCONFIG2,
    #[doc = "0x9c - rtc configure register"]
    pub wdtconfig3: WDTCONFIG3,
    #[doc = "0xa0 - rtc configure register"]
    pub wdtconfig4: WDTCONFIG4,
    #[doc = "0xa4 - rtc configure register"]
    pub wdtfeed: WDTFEED,
    #[doc = "0xa8 - rtc configure register"]
    pub wdtwprotect: WDTWPROTECT,
    #[doc = "0xac - rtc configure register"]
    pub swd_conf: SWD_CONF,
    #[doc = "0xb0 - rtc configure register"]
    pub swd_wprotect: SWD_WPROTECT,
    #[doc = "0xb4 - rtc configure register"]
    pub sw_cpu_stall: SW_CPU_STALL,
    #[doc = "0xb8 - rtc configure register"]
    pub store4: STORE4,
    #[doc = "0xbc - rtc configure register"]
    pub store5: STORE5,
    #[doc = "0xc0 - rtc configure register"]
    pub store6: STORE6,
    #[doc = "0xc4 - rtc configure register"]
    pub store7: STORE7,
    #[doc = "0xc8 - rtc configure register"]
    pub low_power_st: LOW_POWER_ST,
    #[doc = "0xcc - rtc configure register"]
    pub diag0: DIAG0,
    #[doc = "0xd0 - rtc configure register"]
    pub pad_hold: PAD_HOLD,
    #[doc = "0xd4 - rtc configure register"]
    pub dig_pad_hold: DIG_PAD_HOLD,
    #[doc = "0xd8 - rtc configure register"]
    pub brown_out: BROWN_OUT,
    #[doc = "0xdc - rtc configure register"]
    pub time_low1: TIME_LOW1,
    #[doc = "0xe0 - rtc configure register"]
    pub time_high1: TIME_HIGH1,
    #[doc = "0xe4 - rtc configure register"]
    pub xtal32k_clk_factor: XTAL32K_CLK_FACTOR,
    #[doc = "0xe8 - rtc configure register"]
    pub xtal32k_conf: XTAL32K_CONF,
    #[doc = "0xec - rtc configure register"]
    pub usb_conf: USB_CONF,
    #[doc = "0xf0 - RTC_CNTL_RTC_SLP_REJECT_CAUSE_REG"]
    pub slp_reject_cause: SLP_REJECT_CAUSE,
    #[doc = "0xf4 - rtc configure register"]
    pub option1: OPTION1,
    #[doc = "0xf8 - RTC_CNTL_RTC_SLP_WAKEUP_CAUSE_REG"]
    pub slp_wakeup_cause: SLP_WAKEUP_CAUSE,
    #[doc = "0xfc - rtc configure register"]
    pub ulp_cp_timer_1: ULP_CP_TIMER_1,
    #[doc = "0x100 - rtc configure register"]
    pub int_ena_rtc_w1ts: INT_ENA_RTC_W1TS,
    #[doc = "0x104 - rtc configure register"]
    pub int_ena_rtc_w1tc: INT_ENA_RTC_W1TC,
    #[doc = "0x108 - rtc configure register"]
    pub retention_ctrl: RETENTION_CTRL,
    #[doc = "0x10c - rtc configure register"]
    pub fib_sel: FIB_SEL,
    #[doc = "0x110 - rtc configure register"]
    pub gpio_wakeup: GPIO_WAKEUP,
    #[doc = "0x114 - rtc configure register"]
    pub dbg_sel: DBG_SEL,
    #[doc = "0x118 - rtc configure register"]
    pub dbg_map: DBG_MAP,
    #[doc = "0x11c - rtc configure register"]
    pub sensor_ctrl: SENSOR_CTRL,
    #[doc = "0x120 - rtc configure register"]
    pub dbg_sar_sel: DBG_SAR_SEL,
    #[doc = "0x124 - rtc configure register"]
    pub pg_ctrl: PG_CTRL,
    _reserved74: [u8; 0xd4],
    #[doc = "0x1fc - rtc configure register"]
    pub date: DATE,
}
#[doc = "OPTIONS0 (rw) register accessor: an alias for `Reg<OPTIONS0_SPEC>`"]
pub type OPTIONS0 = crate::Reg<options0::OPTIONS0_SPEC>;
#[doc = "rtc configure register"]
pub mod options0;
#[doc = "SLP_TIMER0 (rw) register accessor: an alias for `Reg<SLP_TIMER0_SPEC>`"]
pub type SLP_TIMER0 = crate::Reg<slp_timer0::SLP_TIMER0_SPEC>;
#[doc = "rtc configure register"]
pub mod slp_timer0;
#[doc = "SLP_TIMER1 (rw) register accessor: an alias for `Reg<SLP_TIMER1_SPEC>`"]
pub type SLP_TIMER1 = crate::Reg<slp_timer1::SLP_TIMER1_SPEC>;
#[doc = "rtc configure register"]
pub mod slp_timer1;
#[doc = "TIME_UPDATE (rw) register accessor: an alias for `Reg<TIME_UPDATE_SPEC>`"]
pub type TIME_UPDATE = crate::Reg<time_update::TIME_UPDATE_SPEC>;
#[doc = "rtc configure register"]
pub mod time_update;
#[doc = "TIME_LOW0 (r) register accessor: an alias for `Reg<TIME_LOW0_SPEC>`"]
pub type TIME_LOW0 = crate::Reg<time_low0::TIME_LOW0_SPEC>;
#[doc = "rtc configure register"]
pub mod time_low0;
#[doc = "TIME_HIGH0 (r) register accessor: an alias for `Reg<TIME_HIGH0_SPEC>`"]
pub type TIME_HIGH0 = crate::Reg<time_high0::TIME_HIGH0_SPEC>;
#[doc = "rtc configure register"]
pub mod time_high0;
#[doc = "STATE0 (rw) register accessor: an alias for `Reg<STATE0_SPEC>`"]
pub type STATE0 = crate::Reg<state0::STATE0_SPEC>;
#[doc = "rtc configure register"]
pub mod state0;
#[doc = "TIMER1 (rw) register accessor: an alias for `Reg<TIMER1_SPEC>`"]
pub type TIMER1 = crate::Reg<timer1::TIMER1_SPEC>;
#[doc = "rtc configure register"]
pub mod timer1;
#[doc = "TIMER2 (rw) register accessor: an alias for `Reg<TIMER2_SPEC>`"]
pub type TIMER2 = crate::Reg<timer2::TIMER2_SPEC>;
#[doc = "rtc configure register"]
pub mod timer2;
#[doc = "TIMER3 (rw) register accessor: an alias for `Reg<TIMER3_SPEC>`"]
pub type TIMER3 = crate::Reg<timer3::TIMER3_SPEC>;
#[doc = "rtc configure register"]
pub mod timer3;
#[doc = "TIMER4 (rw) register accessor: an alias for `Reg<TIMER4_SPEC>`"]
pub type TIMER4 = crate::Reg<timer4::TIMER4_SPEC>;
#[doc = "rtc configure register"]
pub mod timer4;
#[doc = "TIMER5 (rw) register accessor: an alias for `Reg<TIMER5_SPEC>`"]
pub type TIMER5 = crate::Reg<timer5::TIMER5_SPEC>;
#[doc = "rtc configure register"]
pub mod timer5;
#[doc = "TIMER6 (rw) register accessor: an alias for `Reg<TIMER6_SPEC>`"]
pub type TIMER6 = crate::Reg<timer6::TIMER6_SPEC>;
#[doc = "rtc configure register"]
pub mod timer6;
#[doc = "ANA_CONF (rw) register accessor: an alias for `Reg<ANA_CONF_SPEC>`"]
pub type ANA_CONF = crate::Reg<ana_conf::ANA_CONF_SPEC>;
#[doc = "rtc configure register"]
pub mod ana_conf;
#[doc = "RESET_STATE (rw) register accessor: an alias for `Reg<RESET_STATE_SPEC>`"]
pub type RESET_STATE = crate::Reg<reset_state::RESET_STATE_SPEC>;
#[doc = "rtc configure register"]
pub mod reset_state;
#[doc = "WAKEUP_STATE (rw) register accessor: an alias for `Reg<WAKEUP_STATE_SPEC>`"]
pub type WAKEUP_STATE = crate::Reg<wakeup_state::WAKEUP_STATE_SPEC>;
#[doc = "rtc configure register"]
pub mod wakeup_state;
#[doc = "INT_ENA_RTC (rw) register accessor: an alias for `Reg<INT_ENA_RTC_SPEC>`"]
pub type INT_ENA_RTC = crate::Reg<int_ena_rtc::INT_ENA_RTC_SPEC>;
#[doc = "rtc configure register"]
pub mod int_ena_rtc;
#[doc = "INT_RAW_RTC (r) register accessor: an alias for `Reg<INT_RAW_RTC_SPEC>`"]
pub type INT_RAW_RTC = crate::Reg<int_raw_rtc::INT_RAW_RTC_SPEC>;
#[doc = "rtc configure register"]
pub mod int_raw_rtc;
#[doc = "INT_ST_RTC (r) register accessor: an alias for `Reg<INT_ST_RTC_SPEC>`"]
pub type INT_ST_RTC = crate::Reg<int_st_rtc::INT_ST_RTC_SPEC>;
#[doc = "rtc configure register"]
pub mod int_st_rtc;
#[doc = "INT_CLR_RTC (w) register accessor: an alias for `Reg<INT_CLR_RTC_SPEC>`"]
pub type INT_CLR_RTC = crate::Reg<int_clr_rtc::INT_CLR_RTC_SPEC>;
#[doc = "rtc configure register"]
pub mod int_clr_rtc;
#[doc = "STORE0 (rw) register accessor: an alias for `Reg<STORE0_SPEC>`"]
pub type STORE0 = crate::Reg<store0::STORE0_SPEC>;
#[doc = "rtc configure register"]
pub mod store0;
#[doc = "STORE1 (rw) register accessor: an alias for `Reg<STORE1_SPEC>`"]
pub type STORE1 = crate::Reg<store1::STORE1_SPEC>;
#[doc = "rtc configure register"]
pub mod store1;
#[doc = "STORE2 (rw) register accessor: an alias for `Reg<STORE2_SPEC>`"]
pub type STORE2 = crate::Reg<store2::STORE2_SPEC>;
#[doc = "rtc configure register"]
pub mod store2;
#[doc = "STORE3 (rw) register accessor: an alias for `Reg<STORE3_SPEC>`"]
pub type STORE3 = crate::Reg<store3::STORE3_SPEC>;
#[doc = "rtc configure register"]
pub mod store3;
#[doc = "EXT_XTL_CONF (rw) register accessor: an alias for `Reg<EXT_XTL_CONF_SPEC>`"]
pub type EXT_XTL_CONF = crate::Reg<ext_xtl_conf::EXT_XTL_CONF_SPEC>;
#[doc = "rtc configure register"]
pub mod ext_xtl_conf;
#[doc = "EXT_WAKEUP_CONF (rw) register accessor: an alias for `Reg<EXT_WAKEUP_CONF_SPEC>`"]
pub type EXT_WAKEUP_CONF = crate::Reg<ext_wakeup_conf::EXT_WAKEUP_CONF_SPEC>;
#[doc = "rtc configure register"]
pub mod ext_wakeup_conf;
#[doc = "SLP_REJECT_CONF (rw) register accessor: an alias for `Reg<SLP_REJECT_CONF_SPEC>`"]
pub type SLP_REJECT_CONF = crate::Reg<slp_reject_conf::SLP_REJECT_CONF_SPEC>;
#[doc = "rtc configure register"]
pub mod slp_reject_conf;
#[doc = "CPU_PERIOD_CONF (rw) register accessor: an alias for `Reg<CPU_PERIOD_CONF_SPEC>`"]
pub type CPU_PERIOD_CONF = crate::Reg<cpu_period_conf::CPU_PERIOD_CONF_SPEC>;
#[doc = "rtc configure register"]
pub mod cpu_period_conf;
#[doc = "CLK_CONF (rw) register accessor: an alias for `Reg<CLK_CONF_SPEC>`"]
pub type CLK_CONF = crate::Reg<clk_conf::CLK_CONF_SPEC>;
#[doc = "rtc configure register"]
pub mod clk_conf;
#[doc = "SLOW_CLK_CONF (rw) register accessor: an alias for `Reg<SLOW_CLK_CONF_SPEC>`"]
pub type SLOW_CLK_CONF = crate::Reg<slow_clk_conf::SLOW_CLK_CONF_SPEC>;
#[doc = "rtc configure register"]
pub mod slow_clk_conf;
#[doc = "SDIO_CONF (rw) register accessor: an alias for `Reg<SDIO_CONF_SPEC>`"]
pub type SDIO_CONF = crate::Reg<sdio_conf::SDIO_CONF_SPEC>;
#[doc = "rtc configure register"]
pub mod sdio_conf;
#[doc = "BIAS_CONF (rw) register accessor: an alias for `Reg<BIAS_CONF_SPEC>`"]
pub type BIAS_CONF = crate::Reg<bias_conf::BIAS_CONF_SPEC>;
#[doc = "rtc configure register"]
pub mod bias_conf;
#[doc = "RTC_CNTL (rw) register accessor: an alias for `Reg<RTC_CNTL_SPEC>`"]
pub type RTC_CNTL = crate::Reg<rtc_cntl::RTC_CNTL_SPEC>;
#[doc = "rtc configure register"]
pub mod rtc_cntl;
#[doc = "PWC (rw) register accessor: an alias for `Reg<PWC_SPEC>`"]
pub type PWC = crate::Reg<pwc::PWC_SPEC>;
#[doc = "rtc configure register"]
pub mod pwc;
#[doc = "DIG_PWC (rw) register accessor: an alias for `Reg<DIG_PWC_SPEC>`"]
pub type DIG_PWC = crate::Reg<dig_pwc::DIG_PWC_SPEC>;
#[doc = "rtc configure register"]
pub mod dig_pwc;
#[doc = "DIG_ISO (rw) register accessor: an alias for `Reg<DIG_ISO_SPEC>`"]
pub type DIG_ISO = crate::Reg<dig_iso::DIG_ISO_SPEC>;
#[doc = "rtc configure register"]
pub mod dig_iso;
#[doc = "WDTCONFIG0 (rw) register accessor: an alias for `Reg<WDTCONFIG0_SPEC>`"]
pub type WDTCONFIG0 = crate::Reg<wdtconfig0::WDTCONFIG0_SPEC>;
#[doc = "rtc configure register"]
pub mod wdtconfig0;
#[doc = "WDTCONFIG1 (rw) register accessor: an alias for `Reg<WDTCONFIG1_SPEC>`"]
pub type WDTCONFIG1 = crate::Reg<wdtconfig1::WDTCONFIG1_SPEC>;
#[doc = "rtc configure register"]
pub mod wdtconfig1;
#[doc = "WDTCONFIG2 (rw) register accessor: an alias for `Reg<WDTCONFIG2_SPEC>`"]
pub type WDTCONFIG2 = crate::Reg<wdtconfig2::WDTCONFIG2_SPEC>;
#[doc = "rtc configure register"]
pub mod wdtconfig2;
#[doc = "WDTCONFIG3 (rw) register accessor: an alias for `Reg<WDTCONFIG3_SPEC>`"]
pub type WDTCONFIG3 = crate::Reg<wdtconfig3::WDTCONFIG3_SPEC>;
#[doc = "rtc configure register"]
pub mod wdtconfig3;
#[doc = "WDTCONFIG4 (rw) register accessor: an alias for `Reg<WDTCONFIG4_SPEC>`"]
pub type WDTCONFIG4 = crate::Reg<wdtconfig4::WDTCONFIG4_SPEC>;
#[doc = "rtc configure register"]
pub mod wdtconfig4;
#[doc = "WDTFEED (w) register accessor: an alias for `Reg<WDTFEED_SPEC>`"]
pub type WDTFEED = crate::Reg<wdtfeed::WDTFEED_SPEC>;
#[doc = "rtc configure register"]
pub mod wdtfeed;
#[doc = "WDTWPROTECT (rw) register accessor: an alias for `Reg<WDTWPROTECT_SPEC>`"]
pub type WDTWPROTECT = crate::Reg<wdtwprotect::WDTWPROTECT_SPEC>;
#[doc = "rtc configure register"]
pub mod wdtwprotect;
#[doc = "SWD_CONF (rw) register accessor: an alias for `Reg<SWD_CONF_SPEC>`"]
pub type SWD_CONF = crate::Reg<swd_conf::SWD_CONF_SPEC>;
#[doc = "rtc configure register"]
pub mod swd_conf;
#[doc = "SWD_WPROTECT (rw) register accessor: an alias for `Reg<SWD_WPROTECT_SPEC>`"]
pub type SWD_WPROTECT = crate::Reg<swd_wprotect::SWD_WPROTECT_SPEC>;
#[doc = "rtc configure register"]
pub mod swd_wprotect;
#[doc = "SW_CPU_STALL (rw) register accessor: an alias for `Reg<SW_CPU_STALL_SPEC>`"]
pub type SW_CPU_STALL = crate::Reg<sw_cpu_stall::SW_CPU_STALL_SPEC>;
#[doc = "rtc configure register"]
pub mod sw_cpu_stall;
#[doc = "STORE4 (rw) register accessor: an alias for `Reg<STORE4_SPEC>`"]
pub type STORE4 = crate::Reg<store4::STORE4_SPEC>;
#[doc = "rtc configure register"]
pub mod store4;
#[doc = "STORE5 (rw) register accessor: an alias for `Reg<STORE5_SPEC>`"]
pub type STORE5 = crate::Reg<store5::STORE5_SPEC>;
#[doc = "rtc configure register"]
pub mod store5;
#[doc = "STORE6 (rw) register accessor: an alias for `Reg<STORE6_SPEC>`"]
pub type STORE6 = crate::Reg<store6::STORE6_SPEC>;
#[doc = "rtc configure register"]
pub mod store6;
#[doc = "STORE7 (rw) register accessor: an alias for `Reg<STORE7_SPEC>`"]
pub type STORE7 = crate::Reg<store7::STORE7_SPEC>;
#[doc = "rtc configure register"]
pub mod store7;
#[doc = "LOW_POWER_ST (r) register accessor: an alias for `Reg<LOW_POWER_ST_SPEC>`"]
pub type LOW_POWER_ST = crate::Reg<low_power_st::LOW_POWER_ST_SPEC>;
#[doc = "rtc configure register"]
pub mod low_power_st;
#[doc = "DIAG0 (r) register accessor: an alias for `Reg<DIAG0_SPEC>`"]
pub type DIAG0 = crate::Reg<diag0::DIAG0_SPEC>;
#[doc = "rtc configure register"]
pub mod diag0;
#[doc = "PAD_HOLD (rw) register accessor: an alias for `Reg<PAD_HOLD_SPEC>`"]
pub type PAD_HOLD = crate::Reg<pad_hold::PAD_HOLD_SPEC>;
#[doc = "rtc configure register"]
pub mod pad_hold;
#[doc = "DIG_PAD_HOLD (rw) register accessor: an alias for `Reg<DIG_PAD_HOLD_SPEC>`"]
pub type DIG_PAD_HOLD = crate::Reg<dig_pad_hold::DIG_PAD_HOLD_SPEC>;
#[doc = "rtc configure register"]
pub mod dig_pad_hold;
#[doc = "BROWN_OUT (rw) register accessor: an alias for `Reg<BROWN_OUT_SPEC>`"]
pub type BROWN_OUT = crate::Reg<brown_out::BROWN_OUT_SPEC>;
#[doc = "rtc configure register"]
pub mod brown_out;
#[doc = "TIME_LOW1 (r) register accessor: an alias for `Reg<TIME_LOW1_SPEC>`"]
pub type TIME_LOW1 = crate::Reg<time_low1::TIME_LOW1_SPEC>;
#[doc = "rtc configure register"]
pub mod time_low1;
#[doc = "TIME_HIGH1 (r) register accessor: an alias for `Reg<TIME_HIGH1_SPEC>`"]
pub type TIME_HIGH1 = crate::Reg<time_high1::TIME_HIGH1_SPEC>;
#[doc = "rtc configure register"]
pub mod time_high1;
#[doc = "XTAL32K_CLK_FACTOR (rw) register accessor: an alias for `Reg<XTAL32K_CLK_FACTOR_SPEC>`"]
pub type XTAL32K_CLK_FACTOR = crate::Reg<xtal32k_clk_factor::XTAL32K_CLK_FACTOR_SPEC>;
#[doc = "rtc configure register"]
pub mod xtal32k_clk_factor;
#[doc = "XTAL32K_CONF (rw) register accessor: an alias for `Reg<XTAL32K_CONF_SPEC>`"]
pub type XTAL32K_CONF = crate::Reg<xtal32k_conf::XTAL32K_CONF_SPEC>;
#[doc = "rtc configure register"]
pub mod xtal32k_conf;
#[doc = "USB_CONF (rw) register accessor: an alias for `Reg<USB_CONF_SPEC>`"]
pub type USB_CONF = crate::Reg<usb_conf::USB_CONF_SPEC>;
#[doc = "rtc configure register"]
pub mod usb_conf;
#[doc = "SLP_REJECT_CAUSE (r) register accessor: an alias for `Reg<SLP_REJECT_CAUSE_SPEC>`"]
pub type SLP_REJECT_CAUSE = crate::Reg<slp_reject_cause::SLP_REJECT_CAUSE_SPEC>;
#[doc = "RTC_CNTL_RTC_SLP_REJECT_CAUSE_REG"]
pub mod slp_reject_cause;
#[doc = "OPTION1 (rw) register accessor: an alias for `Reg<OPTION1_SPEC>`"]
pub type OPTION1 = crate::Reg<option1::OPTION1_SPEC>;
#[doc = "rtc configure register"]
pub mod option1;
#[doc = "SLP_WAKEUP_CAUSE (r) register accessor: an alias for `Reg<SLP_WAKEUP_CAUSE_SPEC>`"]
pub type SLP_WAKEUP_CAUSE = crate::Reg<slp_wakeup_cause::SLP_WAKEUP_CAUSE_SPEC>;
#[doc = "RTC_CNTL_RTC_SLP_WAKEUP_CAUSE_REG"]
pub mod slp_wakeup_cause;
#[doc = "ULP_CP_TIMER_1 (rw) register accessor: an alias for `Reg<ULP_CP_TIMER_1_SPEC>`"]
pub type ULP_CP_TIMER_1 = crate::Reg<ulp_cp_timer_1::ULP_CP_TIMER_1_SPEC>;
#[doc = "rtc configure register"]
pub mod ulp_cp_timer_1;
#[doc = "INT_ENA_RTC_W1TS (w) register accessor: an alias for `Reg<INT_ENA_RTC_W1TS_SPEC>`"]
pub type INT_ENA_RTC_W1TS = crate::Reg<int_ena_rtc_w1ts::INT_ENA_RTC_W1TS_SPEC>;
#[doc = "rtc configure register"]
pub mod int_ena_rtc_w1ts;
#[doc = "INT_ENA_RTC_W1TC (w) register accessor: an alias for `Reg<INT_ENA_RTC_W1TC_SPEC>`"]
pub type INT_ENA_RTC_W1TC = crate::Reg<int_ena_rtc_w1tc::INT_ENA_RTC_W1TC_SPEC>;
#[doc = "rtc configure register"]
pub mod int_ena_rtc_w1tc;
#[doc = "RETENTION_CTRL (rw) register accessor: an alias for `Reg<RETENTION_CTRL_SPEC>`"]
pub type RETENTION_CTRL = crate::Reg<retention_ctrl::RETENTION_CTRL_SPEC>;
#[doc = "rtc configure register"]
pub mod retention_ctrl;
#[doc = "FIB_SEL (rw) register accessor: an alias for `Reg<FIB_SEL_SPEC>`"]
pub type FIB_SEL = crate::Reg<fib_sel::FIB_SEL_SPEC>;
#[doc = "rtc configure register"]
pub mod fib_sel;
#[doc = "GPIO_WAKEUP (rw) register accessor: an alias for `Reg<GPIO_WAKEUP_SPEC>`"]
pub type GPIO_WAKEUP = crate::Reg<gpio_wakeup::GPIO_WAKEUP_SPEC>;
#[doc = "rtc configure register"]
pub mod gpio_wakeup;
#[doc = "DBG_SEL (rw) register accessor: an alias for `Reg<DBG_SEL_SPEC>`"]
pub type DBG_SEL = crate::Reg<dbg_sel::DBG_SEL_SPEC>;
#[doc = "rtc configure register"]
pub mod dbg_sel;
#[doc = "DBG_MAP (rw) register accessor: an alias for `Reg<DBG_MAP_SPEC>`"]
pub type DBG_MAP = crate::Reg<dbg_map::DBG_MAP_SPEC>;
#[doc = "rtc configure register"]
pub mod dbg_map;
#[doc = "SENSOR_CTRL (rw) register accessor: an alias for `Reg<SENSOR_CTRL_SPEC>`"]
pub type SENSOR_CTRL = crate::Reg<sensor_ctrl::SENSOR_CTRL_SPEC>;
#[doc = "rtc configure register"]
pub mod sensor_ctrl;
#[doc = "DBG_SAR_SEL (rw) register accessor: an alias for `Reg<DBG_SAR_SEL_SPEC>`"]
pub type DBG_SAR_SEL = crate::Reg<dbg_sar_sel::DBG_SAR_SEL_SPEC>;
#[doc = "rtc configure register"]
pub mod dbg_sar_sel;
#[doc = "PG_CTRL (rw) register accessor: an alias for `Reg<PG_CTRL_SPEC>`"]
pub type PG_CTRL = crate::Reg<pg_ctrl::PG_CTRL_SPEC>;
#[doc = "rtc configure register"]
pub mod pg_ctrl;
#[doc = "DATE (rw) register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "rtc configure register"]
pub mod date;
