#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - register description"]
    pub rtc_options0: crate::Reg<rtc_options0::RTC_OPTIONS0_SPEC>,
    #[doc = "0x04 - register description"]
    pub rtc_slp_timer0: crate::Reg<rtc_slp_timer0::RTC_SLP_TIMER0_SPEC>,
    #[doc = "0x08 - register description"]
    pub rtc_slp_timer1: crate::Reg<rtc_slp_timer1::RTC_SLP_TIMER1_SPEC>,
    #[doc = "0x0c - register description"]
    pub rtc_time_update: crate::Reg<rtc_time_update::RTC_TIME_UPDATE_SPEC>,
    #[doc = "0x10 - register description"]
    pub rtc_time_low0: crate::Reg<rtc_time_low0::RTC_TIME_LOW0_SPEC>,
    #[doc = "0x14 - register description"]
    pub rtc_time_high0: crate::Reg<rtc_time_high0::RTC_TIME_HIGH0_SPEC>,
    #[doc = "0x18 - register description"]
    pub rtc_state0: crate::Reg<rtc_state0::RTC_STATE0_SPEC>,
    #[doc = "0x1c - register description"]
    pub rtc_timer1: crate::Reg<rtc_timer1::RTC_TIMER1_SPEC>,
    #[doc = "0x20 - register description"]
    pub rtc_timer2: crate::Reg<rtc_timer2::RTC_TIMER2_SPEC>,
    #[doc = "0x24 - register description"]
    pub rtc_timer4: crate::Reg<rtc_timer4::RTC_TIMER4_SPEC>,
    #[doc = "0x28 - register description"]
    pub rtc_timer5: crate::Reg<rtc_timer5::RTC_TIMER5_SPEC>,
    #[doc = "0x2c - register description"]
    pub rtc_ana_conf: crate::Reg<rtc_ana_conf::RTC_ANA_CONF_SPEC>,
    #[doc = "0x30 - register description"]
    pub rtc_reset_state: crate::Reg<rtc_reset_state::RTC_RESET_STATE_SPEC>,
    #[doc = "0x34 - register description"]
    pub rtc_wakeup_state: crate::Reg<rtc_wakeup_state::RTC_WAKEUP_STATE_SPEC>,
    #[doc = "0x38 - register description"]
    pub int_ena_rtc: crate::Reg<int_ena_rtc::INT_ENA_RTC_SPEC>,
    #[doc = "0x3c - register description"]
    pub int_raw_rtc: crate::Reg<int_raw_rtc::INT_RAW_RTC_SPEC>,
    #[doc = "0x40 - register description"]
    pub int_st_rtc: crate::Reg<int_st_rtc::INT_ST_RTC_SPEC>,
    #[doc = "0x44 - register description"]
    pub int_clr_rtc: crate::Reg<int_clr_rtc::INT_CLR_RTC_SPEC>,
    #[doc = "0x48 - register description"]
    pub rtc_store0: crate::Reg<rtc_store0::RTC_STORE0_SPEC>,
    #[doc = "0x4c - register description"]
    pub rtc_store1: crate::Reg<rtc_store1::RTC_STORE1_SPEC>,
    #[doc = "0x50 - register description"]
    pub rtc_store2: crate::Reg<rtc_store2::RTC_STORE2_SPEC>,
    #[doc = "0x54 - register description"]
    pub rtc_store3: crate::Reg<rtc_store3::RTC_STORE3_SPEC>,
    #[doc = "0x58 - register description"]
    pub rtc_ext_xtl_conf: crate::Reg<rtc_ext_xtl_conf::RTC_EXT_XTL_CONF_SPEC>,
    #[doc = "0x5c - register description"]
    pub rtc_ext_wakeup_conf: crate::Reg<rtc_ext_wakeup_conf::RTC_EXT_WAKEUP_CONF_SPEC>,
    #[doc = "0x60 - register description"]
    pub rtc_slp_reject_conf: crate::Reg<rtc_slp_reject_conf::RTC_SLP_REJECT_CONF_SPEC>,
    #[doc = "0x64 - register description"]
    pub rtc_cpu_period_conf: crate::Reg<rtc_cpu_period_conf::RTC_CPU_PERIOD_CONF_SPEC>,
    #[doc = "0x68 - register description"]
    pub rtc_clk_conf: crate::Reg<rtc_clk_conf::RTC_CLK_CONF_SPEC>,
    #[doc = "0x6c - register description"]
    pub rtc_slow_clk_conf: crate::Reg<rtc_slow_clk_conf::RTC_SLOW_CLK_CONF_SPEC>,
    #[doc = "0x70 - register description"]
    pub rtc_bias_conf: crate::Reg<rtc_bias_conf::RTC_BIAS_CONF_SPEC>,
    #[doc = "0x74 - register description"]
    pub rtc: crate::Reg<rtc::RTC_SPEC>,
    #[doc = "0x78 - register description"]
    pub rtc_pwc: crate::Reg<rtc_pwc::RTC_PWC_SPEC>,
    #[doc = "0x7c - register description"]
    pub dig_pwc: crate::Reg<dig_pwc::DIG_PWC_SPEC>,
    #[doc = "0x80 - register description"]
    pub dig_iso: crate::Reg<dig_iso::DIG_ISO_SPEC>,
    #[doc = "0x84 - register description"]
    pub rtc_wdtconfig0: crate::Reg<rtc_wdtconfig0::RTC_WDTCONFIG0_SPEC>,
    #[doc = "0x88 - register description"]
    pub rtc_wdtconfig1: crate::Reg<rtc_wdtconfig1::RTC_WDTCONFIG1_SPEC>,
    #[doc = "0x8c - register description"]
    pub rtc_wdtconfig2: crate::Reg<rtc_wdtconfig2::RTC_WDTCONFIG2_SPEC>,
    #[doc = "0x90 - register description"]
    pub rtc_wdtconfig3: crate::Reg<rtc_wdtconfig3::RTC_WDTCONFIG3_SPEC>,
    #[doc = "0x94 - register description"]
    pub rtc_wdtconfig4: crate::Reg<rtc_wdtconfig4::RTC_WDTCONFIG4_SPEC>,
    #[doc = "0x98 - register description"]
    pub rtc_wdtfeed: crate::Reg<rtc_wdtfeed::RTC_WDTFEED_SPEC>,
    #[doc = "0x9c - register description"]
    pub rtc_wdtwprotect: crate::Reg<rtc_wdtwprotect::RTC_WDTWPROTECT_SPEC>,
    #[doc = "0xa0 - register description"]
    pub rtc_swd_conf: crate::Reg<rtc_swd_conf::RTC_SWD_CONF_SPEC>,
    #[doc = "0xa4 - register description"]
    pub rtc_swd_wprotect: crate::Reg<rtc_swd_wprotect::RTC_SWD_WPROTECT_SPEC>,
    #[doc = "0xa8 - register description"]
    pub rtc_sw_cpu_stall: crate::Reg<rtc_sw_cpu_stall::RTC_SW_CPU_STALL_SPEC>,
    #[doc = "0xac - register description"]
    pub rtc_store4: crate::Reg<rtc_store4::RTC_STORE4_SPEC>,
    #[doc = "0xb0 - register description"]
    pub rtc_store5: crate::Reg<rtc_store5::RTC_STORE5_SPEC>,
    #[doc = "0xb4 - register description"]
    pub rtc_store6: crate::Reg<rtc_store6::RTC_STORE6_SPEC>,
    #[doc = "0xb8 - register description"]
    pub rtc_store7: crate::Reg<rtc_store7::RTC_STORE7_SPEC>,
    #[doc = "0xbc - register description"]
    pub rtc_low_power_st: crate::Reg<rtc_low_power_st::RTC_LOW_POWER_ST_SPEC>,
    #[doc = "0xc0 - register description"]
    pub rtc_diag0: crate::Reg<rtc_diag0::RTC_DIAG0_SPEC>,
    #[doc = "0xc4 - register description"]
    pub rtc_pad_hold: crate::Reg<rtc_pad_hold::RTC_PAD_HOLD_SPEC>,
    #[doc = "0xc8 - register description"]
    pub dig_pad_hold: crate::Reg<dig_pad_hold::DIG_PAD_HOLD_SPEC>,
    #[doc = "0xcc - register description"]
    pub rtc_brown_out: crate::Reg<rtc_brown_out::RTC_BROWN_OUT_SPEC>,
    #[doc = "0xd0 - register description"]
    pub rtc_time_low1: crate::Reg<rtc_time_low1::RTC_TIME_LOW1_SPEC>,
    #[doc = "0xd4 - register description"]
    pub rtc_time_high1: crate::Reg<rtc_time_high1::RTC_TIME_HIGH1_SPEC>,
    #[doc = "0xd8 - register description"]
    pub rtc_usb_conf: crate::Reg<rtc_usb_conf::RTC_USB_CONF_SPEC>,
    #[doc = "0xdc - register description"]
    pub rtc_slp_reject_cause: crate::Reg<rtc_slp_reject_cause::RTC_SLP_REJECT_CAUSE_SPEC>,
    #[doc = "0xe0 - register description"]
    pub rtc_option1: crate::Reg<rtc_option1::RTC_OPTION1_SPEC>,
    #[doc = "0xe4 - register description"]
    pub rtc_slp_wakeup_cause: crate::Reg<rtc_slp_wakeup_cause::RTC_SLP_WAKEUP_CAUSE_SPEC>,
    #[doc = "0xe8 - register description"]
    pub rtc_ulp_cp_timer_1: crate::Reg<rtc_ulp_cp_timer_1::RTC_ULP_CP_TIMER_1_SPEC>,
    #[doc = "0xec - register description"]
    pub int_ena_rtc_w1ts: crate::Reg<int_ena_rtc_w1ts::INT_ENA_RTC_W1TS_SPEC>,
    #[doc = "0xf0 - register description"]
    pub int_ena_rtc_w1tc: crate::Reg<int_ena_rtc_w1tc::INT_ENA_RTC_W1TC_SPEC>,
    #[doc = "0xf4 - register description"]
    pub rtc_cntl_retention_ctrl: crate::Reg<rtc_cntl_retention_ctrl::RTC_CNTL_RETENTION_CTRL_SPEC>,
    #[doc = "0xf8 - register description"]
    pub rtc_fib_sel: crate::Reg<rtc_fib_sel::RTC_FIB_SEL_SPEC>,
    #[doc = "0xfc - register description"]
    pub rtc_cntl_gpio_wakeup: crate::Reg<rtc_cntl_gpio_wakeup::RTC_CNTL_GPIO_WAKEUP_SPEC>,
    #[doc = "0x100 - register description"]
    pub rtc_cntl_dbg_sel: crate::Reg<rtc_cntl_dbg_sel::RTC_CNTL_DBG_SEL_SPEC>,
    #[doc = "0x104 - register description"]
    pub rtc_cntl_dbg_map: crate::Reg<rtc_cntl_dbg_map::RTC_CNTL_DBG_MAP_SPEC>,
    #[doc = "0x108 - register description"]
    pub rtc_cntl_sensor_ctrl: crate::Reg<rtc_cntl_sensor_ctrl::RTC_CNTL_SENSOR_CTRL_SPEC>,
    #[doc = "0x10c - register description"]
    pub rtc_cntl_dbg_sar_sel: crate::Reg<rtc_cntl_dbg_sar_sel::RTC_CNTL_DBG_SAR_SEL_SPEC>,
    _reserved68: [u8; 0xec],
    #[doc = "0x1fc - register description"]
    pub rtc_cntl_date: crate::Reg<rtc_cntl_date::RTC_CNTL_DATE_SPEC>,
}
#[doc = "RTC_OPTIONS0 register accessor: an alias for `Reg<RTC_OPTIONS0_SPEC>`"]
pub type RTC_OPTIONS0 = crate::Reg<rtc_options0::RTC_OPTIONS0_SPEC>;
#[doc = "register description"]
pub mod rtc_options0;
#[doc = "RTC_SLP_TIMER0 register accessor: an alias for `Reg<RTC_SLP_TIMER0_SPEC>`"]
pub type RTC_SLP_TIMER0 = crate::Reg<rtc_slp_timer0::RTC_SLP_TIMER0_SPEC>;
#[doc = "register description"]
pub mod rtc_slp_timer0;
#[doc = "RTC_SLP_TIMER1 register accessor: an alias for `Reg<RTC_SLP_TIMER1_SPEC>`"]
pub type RTC_SLP_TIMER1 = crate::Reg<rtc_slp_timer1::RTC_SLP_TIMER1_SPEC>;
#[doc = "register description"]
pub mod rtc_slp_timer1;
#[doc = "RTC_TIME_UPDATE register accessor: an alias for `Reg<RTC_TIME_UPDATE_SPEC>`"]
pub type RTC_TIME_UPDATE = crate::Reg<rtc_time_update::RTC_TIME_UPDATE_SPEC>;
#[doc = "register description"]
pub mod rtc_time_update;
#[doc = "RTC_TIME_LOW0 register accessor: an alias for `Reg<RTC_TIME_LOW0_SPEC>`"]
pub type RTC_TIME_LOW0 = crate::Reg<rtc_time_low0::RTC_TIME_LOW0_SPEC>;
#[doc = "register description"]
pub mod rtc_time_low0;
#[doc = "RTC_TIME_HIGH0 register accessor: an alias for `Reg<RTC_TIME_HIGH0_SPEC>`"]
pub type RTC_TIME_HIGH0 = crate::Reg<rtc_time_high0::RTC_TIME_HIGH0_SPEC>;
#[doc = "register description"]
pub mod rtc_time_high0;
#[doc = "RTC_STATE0 register accessor: an alias for `Reg<RTC_STATE0_SPEC>`"]
pub type RTC_STATE0 = crate::Reg<rtc_state0::RTC_STATE0_SPEC>;
#[doc = "register description"]
pub mod rtc_state0;
#[doc = "RTC_TIMER1 register accessor: an alias for `Reg<RTC_TIMER1_SPEC>`"]
pub type RTC_TIMER1 = crate::Reg<rtc_timer1::RTC_TIMER1_SPEC>;
#[doc = "register description"]
pub mod rtc_timer1;
#[doc = "RTC_TIMER2 register accessor: an alias for `Reg<RTC_TIMER2_SPEC>`"]
pub type RTC_TIMER2 = crate::Reg<rtc_timer2::RTC_TIMER2_SPEC>;
#[doc = "register description"]
pub mod rtc_timer2;
#[doc = "RTC_TIMER4 register accessor: an alias for `Reg<RTC_TIMER4_SPEC>`"]
pub type RTC_TIMER4 = crate::Reg<rtc_timer4::RTC_TIMER4_SPEC>;
#[doc = "register description"]
pub mod rtc_timer4;
#[doc = "RTC_TIMER5 register accessor: an alias for `Reg<RTC_TIMER5_SPEC>`"]
pub type RTC_TIMER5 = crate::Reg<rtc_timer5::RTC_TIMER5_SPEC>;
#[doc = "register description"]
pub mod rtc_timer5;
#[doc = "RTC_ANA_CONF register accessor: an alias for `Reg<RTC_ANA_CONF_SPEC>`"]
pub type RTC_ANA_CONF = crate::Reg<rtc_ana_conf::RTC_ANA_CONF_SPEC>;
#[doc = "register description"]
pub mod rtc_ana_conf;
#[doc = "RTC_RESET_STATE register accessor: an alias for `Reg<RTC_RESET_STATE_SPEC>`"]
pub type RTC_RESET_STATE = crate::Reg<rtc_reset_state::RTC_RESET_STATE_SPEC>;
#[doc = "register description"]
pub mod rtc_reset_state;
#[doc = "RTC_WAKEUP_STATE register accessor: an alias for `Reg<RTC_WAKEUP_STATE_SPEC>`"]
pub type RTC_WAKEUP_STATE = crate::Reg<rtc_wakeup_state::RTC_WAKEUP_STATE_SPEC>;
#[doc = "register description"]
pub mod rtc_wakeup_state;
#[doc = "INT_ENA_RTC register accessor: an alias for `Reg<INT_ENA_RTC_SPEC>`"]
pub type INT_ENA_RTC = crate::Reg<int_ena_rtc::INT_ENA_RTC_SPEC>;
#[doc = "register description"]
pub mod int_ena_rtc;
#[doc = "INT_RAW_RTC register accessor: an alias for `Reg<INT_RAW_RTC_SPEC>`"]
pub type INT_RAW_RTC = crate::Reg<int_raw_rtc::INT_RAW_RTC_SPEC>;
#[doc = "register description"]
pub mod int_raw_rtc;
#[doc = "INT_ST_RTC register accessor: an alias for `Reg<INT_ST_RTC_SPEC>`"]
pub type INT_ST_RTC = crate::Reg<int_st_rtc::INT_ST_RTC_SPEC>;
#[doc = "register description"]
pub mod int_st_rtc;
#[doc = "INT_CLR_RTC register accessor: an alias for `Reg<INT_CLR_RTC_SPEC>`"]
pub type INT_CLR_RTC = crate::Reg<int_clr_rtc::INT_CLR_RTC_SPEC>;
#[doc = "register description"]
pub mod int_clr_rtc;
#[doc = "RTC_STORE0 register accessor: an alias for `Reg<RTC_STORE0_SPEC>`"]
pub type RTC_STORE0 = crate::Reg<rtc_store0::RTC_STORE0_SPEC>;
#[doc = "register description"]
pub mod rtc_store0;
#[doc = "RTC_STORE1 register accessor: an alias for `Reg<RTC_STORE1_SPEC>`"]
pub type RTC_STORE1 = crate::Reg<rtc_store1::RTC_STORE1_SPEC>;
#[doc = "register description"]
pub mod rtc_store1;
#[doc = "RTC_STORE2 register accessor: an alias for `Reg<RTC_STORE2_SPEC>`"]
pub type RTC_STORE2 = crate::Reg<rtc_store2::RTC_STORE2_SPEC>;
#[doc = "register description"]
pub mod rtc_store2;
#[doc = "RTC_STORE3 register accessor: an alias for `Reg<RTC_STORE3_SPEC>`"]
pub type RTC_STORE3 = crate::Reg<rtc_store3::RTC_STORE3_SPEC>;
#[doc = "register description"]
pub mod rtc_store3;
#[doc = "RTC_EXT_XTL_CONF register accessor: an alias for `Reg<RTC_EXT_XTL_CONF_SPEC>`"]
pub type RTC_EXT_XTL_CONF = crate::Reg<rtc_ext_xtl_conf::RTC_EXT_XTL_CONF_SPEC>;
#[doc = "register description"]
pub mod rtc_ext_xtl_conf;
#[doc = "RTC_EXT_WAKEUP_CONF register accessor: an alias for `Reg<RTC_EXT_WAKEUP_CONF_SPEC>`"]
pub type RTC_EXT_WAKEUP_CONF = crate::Reg<rtc_ext_wakeup_conf::RTC_EXT_WAKEUP_CONF_SPEC>;
#[doc = "register description"]
pub mod rtc_ext_wakeup_conf;
#[doc = "RTC_SLP_REJECT_CONF register accessor: an alias for `Reg<RTC_SLP_REJECT_CONF_SPEC>`"]
pub type RTC_SLP_REJECT_CONF = crate::Reg<rtc_slp_reject_conf::RTC_SLP_REJECT_CONF_SPEC>;
#[doc = "register description"]
pub mod rtc_slp_reject_conf;
#[doc = "RTC_CPU_PERIOD_CONF register accessor: an alias for `Reg<RTC_CPU_PERIOD_CONF_SPEC>`"]
pub type RTC_CPU_PERIOD_CONF = crate::Reg<rtc_cpu_period_conf::RTC_CPU_PERIOD_CONF_SPEC>;
#[doc = "register description"]
pub mod rtc_cpu_period_conf;
#[doc = "RTC_CLK_CONF register accessor: an alias for `Reg<RTC_CLK_CONF_SPEC>`"]
pub type RTC_CLK_CONF = crate::Reg<rtc_clk_conf::RTC_CLK_CONF_SPEC>;
#[doc = "register description"]
pub mod rtc_clk_conf;
#[doc = "RTC_SLOW_CLK_CONF register accessor: an alias for `Reg<RTC_SLOW_CLK_CONF_SPEC>`"]
pub type RTC_SLOW_CLK_CONF = crate::Reg<rtc_slow_clk_conf::RTC_SLOW_CLK_CONF_SPEC>;
#[doc = "register description"]
pub mod rtc_slow_clk_conf;
#[doc = "RTC_BIAS_CONF register accessor: an alias for `Reg<RTC_BIAS_CONF_SPEC>`"]
pub type RTC_BIAS_CONF = crate::Reg<rtc_bias_conf::RTC_BIAS_CONF_SPEC>;
#[doc = "register description"]
pub mod rtc_bias_conf;
#[doc = "RTC register accessor: an alias for `Reg<RTC_SPEC>`"]
pub type RTC = crate::Reg<rtc::RTC_SPEC>;
#[doc = "register description"]
pub mod rtc;
#[doc = "RTC_PWC register accessor: an alias for `Reg<RTC_PWC_SPEC>`"]
pub type RTC_PWC = crate::Reg<rtc_pwc::RTC_PWC_SPEC>;
#[doc = "register description"]
pub mod rtc_pwc;
#[doc = "DIG_PWC register accessor: an alias for `Reg<DIG_PWC_SPEC>`"]
pub type DIG_PWC = crate::Reg<dig_pwc::DIG_PWC_SPEC>;
#[doc = "register description"]
pub mod dig_pwc;
#[doc = "DIG_ISO register accessor: an alias for `Reg<DIG_ISO_SPEC>`"]
pub type DIG_ISO = crate::Reg<dig_iso::DIG_ISO_SPEC>;
#[doc = "register description"]
pub mod dig_iso;
#[doc = "RTC_WDTCONFIG0 register accessor: an alias for `Reg<RTC_WDTCONFIG0_SPEC>`"]
pub type RTC_WDTCONFIG0 = crate::Reg<rtc_wdtconfig0::RTC_WDTCONFIG0_SPEC>;
#[doc = "register description"]
pub mod rtc_wdtconfig0;
#[doc = "RTC_WDTCONFIG1 register accessor: an alias for `Reg<RTC_WDTCONFIG1_SPEC>`"]
pub type RTC_WDTCONFIG1 = crate::Reg<rtc_wdtconfig1::RTC_WDTCONFIG1_SPEC>;
#[doc = "register description"]
pub mod rtc_wdtconfig1;
#[doc = "RTC_WDTCONFIG2 register accessor: an alias for `Reg<RTC_WDTCONFIG2_SPEC>`"]
pub type RTC_WDTCONFIG2 = crate::Reg<rtc_wdtconfig2::RTC_WDTCONFIG2_SPEC>;
#[doc = "register description"]
pub mod rtc_wdtconfig2;
#[doc = "RTC_WDTCONFIG3 register accessor: an alias for `Reg<RTC_WDTCONFIG3_SPEC>`"]
pub type RTC_WDTCONFIG3 = crate::Reg<rtc_wdtconfig3::RTC_WDTCONFIG3_SPEC>;
#[doc = "register description"]
pub mod rtc_wdtconfig3;
#[doc = "RTC_WDTCONFIG4 register accessor: an alias for `Reg<RTC_WDTCONFIG4_SPEC>`"]
pub type RTC_WDTCONFIG4 = crate::Reg<rtc_wdtconfig4::RTC_WDTCONFIG4_SPEC>;
#[doc = "register description"]
pub mod rtc_wdtconfig4;
#[doc = "RTC_WDTFEED register accessor: an alias for `Reg<RTC_WDTFEED_SPEC>`"]
pub type RTC_WDTFEED = crate::Reg<rtc_wdtfeed::RTC_WDTFEED_SPEC>;
#[doc = "register description"]
pub mod rtc_wdtfeed;
#[doc = "RTC_WDTWPROTECT register accessor: an alias for `Reg<RTC_WDTWPROTECT_SPEC>`"]
pub type RTC_WDTWPROTECT = crate::Reg<rtc_wdtwprotect::RTC_WDTWPROTECT_SPEC>;
#[doc = "register description"]
pub mod rtc_wdtwprotect;
#[doc = "RTC_SWD_CONF register accessor: an alias for `Reg<RTC_SWD_CONF_SPEC>`"]
pub type RTC_SWD_CONF = crate::Reg<rtc_swd_conf::RTC_SWD_CONF_SPEC>;
#[doc = "register description"]
pub mod rtc_swd_conf;
#[doc = "RTC_SWD_WPROTECT register accessor: an alias for `Reg<RTC_SWD_WPROTECT_SPEC>`"]
pub type RTC_SWD_WPROTECT = crate::Reg<rtc_swd_wprotect::RTC_SWD_WPROTECT_SPEC>;
#[doc = "register description"]
pub mod rtc_swd_wprotect;
#[doc = "RTC_SW_CPU_STALL register accessor: an alias for `Reg<RTC_SW_CPU_STALL_SPEC>`"]
pub type RTC_SW_CPU_STALL = crate::Reg<rtc_sw_cpu_stall::RTC_SW_CPU_STALL_SPEC>;
#[doc = "register description"]
pub mod rtc_sw_cpu_stall;
#[doc = "RTC_STORE4 register accessor: an alias for `Reg<RTC_STORE4_SPEC>`"]
pub type RTC_STORE4 = crate::Reg<rtc_store4::RTC_STORE4_SPEC>;
#[doc = "register description"]
pub mod rtc_store4;
#[doc = "RTC_STORE5 register accessor: an alias for `Reg<RTC_STORE5_SPEC>`"]
pub type RTC_STORE5 = crate::Reg<rtc_store5::RTC_STORE5_SPEC>;
#[doc = "register description"]
pub mod rtc_store5;
#[doc = "RTC_STORE6 register accessor: an alias for `Reg<RTC_STORE6_SPEC>`"]
pub type RTC_STORE6 = crate::Reg<rtc_store6::RTC_STORE6_SPEC>;
#[doc = "register description"]
pub mod rtc_store6;
#[doc = "RTC_STORE7 register accessor: an alias for `Reg<RTC_STORE7_SPEC>`"]
pub type RTC_STORE7 = crate::Reg<rtc_store7::RTC_STORE7_SPEC>;
#[doc = "register description"]
pub mod rtc_store7;
#[doc = "RTC_LOW_POWER_ST register accessor: an alias for `Reg<RTC_LOW_POWER_ST_SPEC>`"]
pub type RTC_LOW_POWER_ST = crate::Reg<rtc_low_power_st::RTC_LOW_POWER_ST_SPEC>;
#[doc = "register description"]
pub mod rtc_low_power_st;
#[doc = "RTC_DIAG0 register accessor: an alias for `Reg<RTC_DIAG0_SPEC>`"]
pub type RTC_DIAG0 = crate::Reg<rtc_diag0::RTC_DIAG0_SPEC>;
#[doc = "register description"]
pub mod rtc_diag0;
#[doc = "RTC_PAD_HOLD register accessor: an alias for `Reg<RTC_PAD_HOLD_SPEC>`"]
pub type RTC_PAD_HOLD = crate::Reg<rtc_pad_hold::RTC_PAD_HOLD_SPEC>;
#[doc = "register description"]
pub mod rtc_pad_hold;
#[doc = "DIG_PAD_HOLD register accessor: an alias for `Reg<DIG_PAD_HOLD_SPEC>`"]
pub type DIG_PAD_HOLD = crate::Reg<dig_pad_hold::DIG_PAD_HOLD_SPEC>;
#[doc = "register description"]
pub mod dig_pad_hold;
#[doc = "RTC_BROWN_OUT register accessor: an alias for `Reg<RTC_BROWN_OUT_SPEC>`"]
pub type RTC_BROWN_OUT = crate::Reg<rtc_brown_out::RTC_BROWN_OUT_SPEC>;
#[doc = "register description"]
pub mod rtc_brown_out;
#[doc = "RTC_TIME_LOW1 register accessor: an alias for `Reg<RTC_TIME_LOW1_SPEC>`"]
pub type RTC_TIME_LOW1 = crate::Reg<rtc_time_low1::RTC_TIME_LOW1_SPEC>;
#[doc = "register description"]
pub mod rtc_time_low1;
#[doc = "RTC_TIME_HIGH1 register accessor: an alias for `Reg<RTC_TIME_HIGH1_SPEC>`"]
pub type RTC_TIME_HIGH1 = crate::Reg<rtc_time_high1::RTC_TIME_HIGH1_SPEC>;
#[doc = "register description"]
pub mod rtc_time_high1;
#[doc = "RTC_USB_CONF register accessor: an alias for `Reg<RTC_USB_CONF_SPEC>`"]
pub type RTC_USB_CONF = crate::Reg<rtc_usb_conf::RTC_USB_CONF_SPEC>;
#[doc = "register description"]
pub mod rtc_usb_conf;
#[doc = "RTC_SLP_REJECT_CAUSE register accessor: an alias for `Reg<RTC_SLP_REJECT_CAUSE_SPEC>`"]
pub type RTC_SLP_REJECT_CAUSE = crate::Reg<rtc_slp_reject_cause::RTC_SLP_REJECT_CAUSE_SPEC>;
#[doc = "register description"]
pub mod rtc_slp_reject_cause;
#[doc = "RTC_OPTION1 register accessor: an alias for `Reg<RTC_OPTION1_SPEC>`"]
pub type RTC_OPTION1 = crate::Reg<rtc_option1::RTC_OPTION1_SPEC>;
#[doc = "register description"]
pub mod rtc_option1;
#[doc = "RTC_SLP_WAKEUP_CAUSE register accessor: an alias for `Reg<RTC_SLP_WAKEUP_CAUSE_SPEC>`"]
pub type RTC_SLP_WAKEUP_CAUSE = crate::Reg<rtc_slp_wakeup_cause::RTC_SLP_WAKEUP_CAUSE_SPEC>;
#[doc = "register description"]
pub mod rtc_slp_wakeup_cause;
#[doc = "RTC_ULP_CP_TIMER_1 register accessor: an alias for `Reg<RTC_ULP_CP_TIMER_1_SPEC>`"]
pub type RTC_ULP_CP_TIMER_1 = crate::Reg<rtc_ulp_cp_timer_1::RTC_ULP_CP_TIMER_1_SPEC>;
#[doc = "register description"]
pub mod rtc_ulp_cp_timer_1;
#[doc = "INT_ENA_RTC_W1TS register accessor: an alias for `Reg<INT_ENA_RTC_W1TS_SPEC>`"]
pub type INT_ENA_RTC_W1TS = crate::Reg<int_ena_rtc_w1ts::INT_ENA_RTC_W1TS_SPEC>;
#[doc = "register description"]
pub mod int_ena_rtc_w1ts;
#[doc = "INT_ENA_RTC_W1TC register accessor: an alias for `Reg<INT_ENA_RTC_W1TC_SPEC>`"]
pub type INT_ENA_RTC_W1TC = crate::Reg<int_ena_rtc_w1tc::INT_ENA_RTC_W1TC_SPEC>;
#[doc = "register description"]
pub mod int_ena_rtc_w1tc;
#[doc = "RTC_CNTL_RETENTION_CTRL register accessor: an alias for `Reg<RTC_CNTL_RETENTION_CTRL_SPEC>`"]
pub type RTC_CNTL_RETENTION_CTRL =
    crate::Reg<rtc_cntl_retention_ctrl::RTC_CNTL_RETENTION_CTRL_SPEC>;
#[doc = "register description"]
pub mod rtc_cntl_retention_ctrl;
#[doc = "RTC_FIB_SEL register accessor: an alias for `Reg<RTC_FIB_SEL_SPEC>`"]
pub type RTC_FIB_SEL = crate::Reg<rtc_fib_sel::RTC_FIB_SEL_SPEC>;
#[doc = "register description"]
pub mod rtc_fib_sel;
#[doc = "RTC_CNTL_GPIO_WAKEUP register accessor: an alias for `Reg<RTC_CNTL_GPIO_WAKEUP_SPEC>`"]
pub type RTC_CNTL_GPIO_WAKEUP = crate::Reg<rtc_cntl_gpio_wakeup::RTC_CNTL_GPIO_WAKEUP_SPEC>;
#[doc = "register description"]
pub mod rtc_cntl_gpio_wakeup;
#[doc = "RTC_CNTL_DBG_SEL register accessor: an alias for `Reg<RTC_CNTL_DBG_SEL_SPEC>`"]
pub type RTC_CNTL_DBG_SEL = crate::Reg<rtc_cntl_dbg_sel::RTC_CNTL_DBG_SEL_SPEC>;
#[doc = "register description"]
pub mod rtc_cntl_dbg_sel;
#[doc = "RTC_CNTL_DBG_MAP register accessor: an alias for `Reg<RTC_CNTL_DBG_MAP_SPEC>`"]
pub type RTC_CNTL_DBG_MAP = crate::Reg<rtc_cntl_dbg_map::RTC_CNTL_DBG_MAP_SPEC>;
#[doc = "register description"]
pub mod rtc_cntl_dbg_map;
#[doc = "RTC_CNTL_SENSOR_CTRL register accessor: an alias for `Reg<RTC_CNTL_SENSOR_CTRL_SPEC>`"]
pub type RTC_CNTL_SENSOR_CTRL = crate::Reg<rtc_cntl_sensor_ctrl::RTC_CNTL_SENSOR_CTRL_SPEC>;
#[doc = "register description"]
pub mod rtc_cntl_sensor_ctrl;
#[doc = "RTC_CNTL_DBG_SAR_SEL register accessor: an alias for `Reg<RTC_CNTL_DBG_SAR_SEL_SPEC>`"]
pub type RTC_CNTL_DBG_SAR_SEL = crate::Reg<rtc_cntl_dbg_sar_sel::RTC_CNTL_DBG_SAR_SEL_SPEC>;
#[doc = "register description"]
pub mod rtc_cntl_dbg_sar_sel;
#[doc = "RTC_CNTL_DATE register accessor: an alias for `Reg<RTC_CNTL_DATE_SPEC>`"]
pub type RTC_CNTL_DATE = crate::Reg<rtc_cntl_date::RTC_CNTL_DATE_SPEC>;
#[doc = "register description"]
pub mod rtc_cntl_date;
