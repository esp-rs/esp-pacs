#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - Sets the power options of crystal and PLL clocks, and initiates reset by software"]
    pub options0: OPTIONS0,
    #[doc = "0x04 - RTC timer threshold register 0"]
    pub slp_timer0: SLP_TIMER0,
    #[doc = "0x08 - RTC timer threshold register 1"]
    pub slp_timer1: SLP_TIMER1,
    #[doc = "0x0c - RTC timer update control register"]
    pub time_update: TIME_UPDATE,
    #[doc = "0x10 - Stores the lower 32 bits of RTC timer 0."]
    pub time_low0: TIME_LOW0,
    #[doc = "0x14 - Stores the higher 16 bits of RTC timer 0"]
    pub time_high0: TIME_HIGH0,
    #[doc = "0x18 - Configures the sleep / reject / wakeup state"]
    pub state0: STATE0,
    #[doc = "0x1c - Configures CPU stall options"]
    pub timer1: TIMER1,
    #[doc = "0x20 - Configures RTC slow clock and touch controller"]
    pub timer2: TIMER2,
    #[doc = "0x24 - configure some wait time for power on"]
    pub timer3: TIMER3,
    #[doc = "0x28 - configure some wait time for power on"]
    pub timer4: TIMER4,
    #[doc = "0x2c - Configures the minimal sleep cycles"]
    pub timer5: TIMER5,
    #[doc = "0x30 - Configure minimal sleep cycles register"]
    pub timer6: TIMER6,
    #[doc = "0x34 - Configures the power options for I2C and PLLA"]
    pub ana_conf: ANA_CONF,
    #[doc = "0x38 - Indicates the CPU reset source. For more information about the reset cause, please refer to Table \\ref{table:resetreasons} in Chapter \\ref{module:ResetandClock} \\textit{\nameref{module:ResetandClock}}."]
    pub reset_state: RESET_STATE,
    #[doc = "0x3c - Wakeup bitmap enabling register"]
    pub wakeup_state: WAKEUP_STATE,
    #[doc = "0x40 - RTC interrupt enabling register"]
    pub int_ena_rtc: INT_ENA_RTC,
    #[doc = "0x44 - RTC interrupt raw register"]
    pub int_raw_rtc: INT_RAW_RTC,
    #[doc = "0x48 - RTC interrupt state register"]
    pub int_st_rtc: INT_ST_RTC,
    #[doc = "0x4c - RTC interrupt clear register"]
    pub int_clr_rtc: INT_CLR_RTC,
    #[doc = "0x50 - Reservation register 0"]
    pub store0: STORE0,
    #[doc = "0x54 - Reservation register 1"]
    pub store1: STORE1,
    #[doc = "0x58 - Reservation register 2"]
    pub store2: STORE2,
    #[doc = "0x5c - Reservation register 3"]
    pub store3: STORE3,
    #[doc = "0x60 - 32 kHz crystal oscillator configuration register"]
    pub ext_xtl_conf: EXT_XTL_CONF,
    #[doc = "0x64 - GPIO wakeup configuration register"]
    pub ext_wakeup_conf: EXT_WAKEUP_CONF,
    #[doc = "0x68 - Configures sleep / reject options"]
    pub slp_reject_conf: SLP_REJECT_CONF,
    #[doc = "0x6c - CPU sel option"]
    pub cpu_period_conf: CPU_PERIOD_CONF,
    #[doc = "0x70 - configure sdio active register"]
    pub sdio_act_conf: SDIO_ACT_CONF,
    #[doc = "0x74 - RTC clock configuration register"]
    pub clk_conf: CLK_CONF,
    #[doc = "0x78 - RTC slow clock configuration register"]
    pub slow_clk_conf: SLOW_CLK_CONF,
    #[doc = "0x7c - configure vddsdio register"]
    pub sdio_conf: SDIO_CONF,
    #[doc = "0x80 - configure power register"]
    pub bias_conf: BIAS_CONF,
    #[doc = "0x84 - RTC/DIG regulator configuration register"]
    pub reg: REG,
    #[doc = "0x88 - RTC power configuraiton register"]
    pub pwc: PWC,
    #[doc = "0x8c - Digital system power configuraiton register"]
    pub dig_pwc: DIG_PWC,
    #[doc = "0x90 - Digital system ISO configuration register"]
    pub dig_iso: DIG_ISO,
    #[doc = "0x94 - RTC watchdog configuration register"]
    pub wdtconfig0: WDTCONFIG0,
    #[doc = "0x98 - Configures the hold time of RTC watchdog at level 1"]
    pub wdtconfig1: WDTCONFIG1,
    #[doc = "0x9c - Configures the hold time of RTC watchdog at level 2"]
    pub wdtconfig2: WDTCONFIG2,
    #[doc = "0xa0 - Configures the hold time of RTC watchdog at level 3"]
    pub wdtconfig3: WDTCONFIG3,
    #[doc = "0xa4 - Configures the hold time of RTC watchdog at level 4"]
    pub wdtconfig4: WDTCONFIG4,
    #[doc = "0xa8 - RTC watchdog SW feed configuration register"]
    pub wdtfeed: WDTFEED,
    #[doc = "0xac - RTC watchdog write protection configuration register"]
    pub wdtwprotect: WDTWPROTECT,
    #[doc = "0xb0 - Super watchdog configuration register"]
    pub swd_conf: SWD_CONF,
    #[doc = "0xb4 - Super watchdog write protection configuration register"]
    pub swd_wprotect: SWD_WPROTECT,
    #[doc = "0xb8 - CPU stall configuration register"]
    pub sw_cpu_stall: SW_CPU_STALL,
    #[doc = "0xbc - Reservation register 4"]
    pub store4: STORE4,
    #[doc = "0xc0 - Reservation register 5"]
    pub store5: STORE5,
    #[doc = "0xc4 - Reservation register 6"]
    pub store6: STORE6,
    #[doc = "0xc8 - Reservation register 7"]
    pub store7: STORE7,
    #[doc = "0xcc - RTC main state machine status register"]
    pub low_power_st: LOW_POWER_ST,
    #[doc = "0xd0 - debug register"]
    pub diag0: DIAG0,
    #[doc = "0xd4 - Configures the hold options for RTC GPIOs"]
    pub pad_hold: PAD_HOLD,
    #[doc = "0xd8 - Configures the hold option for digital GPIOs"]
    pub dig_pad_hold: DIG_PAD_HOLD,
    #[doc = "0xdc - EXT1 wakeup configuration register"]
    pub ext_wakeup1: EXT_WAKEUP1,
    #[doc = "0xe0 - EXT1 wakeup source register"]
    pub ext_wakeup1_status: EXT_WAKEUP1_STATUS,
    #[doc = "0xe4 - Brownout configuration register"]
    pub brown_out: BROWN_OUT,
    #[doc = "0xe8 - Stores the lower 32 bits of RTC timer 1"]
    pub time_low1: TIME_LOW1,
    #[doc = "0xec - Stores the higher 16 bits of RTC timer 1"]
    pub time_high1: TIME_HIGH1,
    #[doc = "0xf0 - Configures the divider factor for the backup clock of 32 kHz crystal oscillator"]
    pub xtal32k_clk_factor: XTAL32K_CLK_FACTOR,
    #[doc = "0xf4 - 32 kHz crystal oscillator configuration register"]
    pub xtal32k_conf: XTAL32K_CONF,
    #[doc = "0xf8 - Configure coprocessor timer"]
    pub ulp_cp_timer: ULP_CP_TIMER,
    #[doc = "0xfc - ULP-FSM configuration register"]
    pub ulp_cp_ctrl: ULP_CP_CTRL,
    #[doc = "0x100 - ULP-RISCV configuration register"]
    pub cocpu_ctrl: COCPU_CTRL,
    #[doc = "0x104 - Touch control register"]
    pub touch_ctrl1: TOUCH_CTRL1,
    #[doc = "0x108 - Touch control register"]
    pub touch_ctrl2: TOUCH_CTRL2,
    #[doc = "0x10c - Configure touch scan settings"]
    pub touch_scan_ctrl: TOUCH_SCAN_CTRL,
    #[doc = "0x110 - Configure the settings of touch sleep pad"]
    pub touch_slp_thres: TOUCH_SLP_THRES,
    #[doc = "0x114 - Configure touch approach settings"]
    pub touch_approach: TOUCH_APPROACH,
    #[doc = "0x118 - Configure touch filter settings"]
    pub touch_filter_ctrl: TOUCH_FILTER_CTRL,
    #[doc = "0x11c - configure usb control register"]
    pub usb_conf: USB_CONF,
    #[doc = "0x120 - Configure touch timeout settings"]
    pub touch_timeout_ctrl: TOUCH_TIMEOUT_CTRL,
    #[doc = "0x124 - Stores the reject-to-sleep cause."]
    pub slp_reject_cause: SLP_REJECT_CAUSE,
    #[doc = "0x128 - RTC option register"]
    pub options1: OPTIONS1,
    #[doc = "0x12c - Stores the sleep-to-wakeup cause."]
    pub slp_wakeup_cause: SLP_WAKEUP_CAUSE,
    #[doc = "0x130 - Configure sleep cycle of the timer"]
    pub ulp_cp_timer_1: ULP_CP_TIMER_1,
    _reserved77: [u8; 0x04],
    #[doc = "0x138 - "]
    pub date: DATE,
}
#[doc = "OPTIONS0 (rw) register accessor: an alias for `Reg<OPTIONS0_SPEC>`"]
pub type OPTIONS0 = crate::Reg<options0::OPTIONS0_SPEC>;
#[doc = "Sets the power options of crystal and PLL clocks, and initiates reset by software"]
pub mod options0;
#[doc = "SLP_TIMER0 (rw) register accessor: an alias for `Reg<SLP_TIMER0_SPEC>`"]
pub type SLP_TIMER0 = crate::Reg<slp_timer0::SLP_TIMER0_SPEC>;
#[doc = "RTC timer threshold register 0"]
pub mod slp_timer0;
#[doc = "SLP_TIMER1 (rw) register accessor: an alias for `Reg<SLP_TIMER1_SPEC>`"]
pub type SLP_TIMER1 = crate::Reg<slp_timer1::SLP_TIMER1_SPEC>;
#[doc = "RTC timer threshold register 1"]
pub mod slp_timer1;
#[doc = "TIME_UPDATE (rw) register accessor: an alias for `Reg<TIME_UPDATE_SPEC>`"]
pub type TIME_UPDATE = crate::Reg<time_update::TIME_UPDATE_SPEC>;
#[doc = "RTC timer update control register"]
pub mod time_update;
#[doc = "TIME_LOW0 (r) register accessor: an alias for `Reg<TIME_LOW0_SPEC>`"]
pub type TIME_LOW0 = crate::Reg<time_low0::TIME_LOW0_SPEC>;
#[doc = "Stores the lower 32 bits of RTC timer 0."]
pub mod time_low0;
#[doc = "TIME_HIGH0 (r) register accessor: an alias for `Reg<TIME_HIGH0_SPEC>`"]
pub type TIME_HIGH0 = crate::Reg<time_high0::TIME_HIGH0_SPEC>;
#[doc = "Stores the higher 16 bits of RTC timer 0"]
pub mod time_high0;
#[doc = "STATE0 (rw) register accessor: an alias for `Reg<STATE0_SPEC>`"]
pub type STATE0 = crate::Reg<state0::STATE0_SPEC>;
#[doc = "Configures the sleep / reject / wakeup state"]
pub mod state0;
#[doc = "TIMER1 (rw) register accessor: an alias for `Reg<TIMER1_SPEC>`"]
pub type TIMER1 = crate::Reg<timer1::TIMER1_SPEC>;
#[doc = "Configures CPU stall options"]
pub mod timer1;
#[doc = "TIMER2 (rw) register accessor: an alias for `Reg<TIMER2_SPEC>`"]
pub type TIMER2 = crate::Reg<timer2::TIMER2_SPEC>;
#[doc = "Configures RTC slow clock and touch controller"]
pub mod timer2;
#[doc = "TIMER3 (rw) register accessor: an alias for `Reg<TIMER3_SPEC>`"]
pub type TIMER3 = crate::Reg<timer3::TIMER3_SPEC>;
#[doc = "configure some wait time for power on"]
pub mod timer3;
#[doc = "TIMER4 (rw) register accessor: an alias for `Reg<TIMER4_SPEC>`"]
pub type TIMER4 = crate::Reg<timer4::TIMER4_SPEC>;
#[doc = "configure some wait time for power on"]
pub mod timer4;
#[doc = "TIMER5 (rw) register accessor: an alias for `Reg<TIMER5_SPEC>`"]
pub type TIMER5 = crate::Reg<timer5::TIMER5_SPEC>;
#[doc = "Configures the minimal sleep cycles"]
pub mod timer5;
#[doc = "TIMER6 (rw) register accessor: an alias for `Reg<TIMER6_SPEC>`"]
pub type TIMER6 = crate::Reg<timer6::TIMER6_SPEC>;
#[doc = "Configure minimal sleep cycles register"]
pub mod timer6;
#[doc = "ANA_CONF (rw) register accessor: an alias for `Reg<ANA_CONF_SPEC>`"]
pub type ANA_CONF = crate::Reg<ana_conf::ANA_CONF_SPEC>;
#[doc = "Configures the power options for I2C and PLLA"]
pub mod ana_conf;
#[doc = "RESET_STATE (rw) register accessor: an alias for `Reg<RESET_STATE_SPEC>`"]
pub type RESET_STATE = crate::Reg<reset_state::RESET_STATE_SPEC>;
#[doc = "Indicates the CPU reset source. For more information about the reset cause, please refer to Table \\ref{table:resetreasons} in Chapter \\ref{module:ResetandClock} \\textit{\nameref{module:ResetandClock}}."]
pub mod reset_state;
#[doc = "WAKEUP_STATE (rw) register accessor: an alias for `Reg<WAKEUP_STATE_SPEC>`"]
pub type WAKEUP_STATE = crate::Reg<wakeup_state::WAKEUP_STATE_SPEC>;
#[doc = "Wakeup bitmap enabling register"]
pub mod wakeup_state;
#[doc = "INT_ENA_RTC (rw) register accessor: an alias for `Reg<INT_ENA_RTC_SPEC>`"]
pub type INT_ENA_RTC = crate::Reg<int_ena_rtc::INT_ENA_RTC_SPEC>;
#[doc = "RTC interrupt enabling register"]
pub mod int_ena_rtc;
#[doc = "INT_RAW_RTC (r) register accessor: an alias for `Reg<INT_RAW_RTC_SPEC>`"]
pub type INT_RAW_RTC = crate::Reg<int_raw_rtc::INT_RAW_RTC_SPEC>;
#[doc = "RTC interrupt raw register"]
pub mod int_raw_rtc;
#[doc = "INT_ST_RTC (r) register accessor: an alias for `Reg<INT_ST_RTC_SPEC>`"]
pub type INT_ST_RTC = crate::Reg<int_st_rtc::INT_ST_RTC_SPEC>;
#[doc = "RTC interrupt state register"]
pub mod int_st_rtc;
#[doc = "INT_CLR_RTC (w) register accessor: an alias for `Reg<INT_CLR_RTC_SPEC>`"]
pub type INT_CLR_RTC = crate::Reg<int_clr_rtc::INT_CLR_RTC_SPEC>;
#[doc = "RTC interrupt clear register"]
pub mod int_clr_rtc;
#[doc = "STORE0 (rw) register accessor: an alias for `Reg<STORE0_SPEC>`"]
pub type STORE0 = crate::Reg<store0::STORE0_SPEC>;
#[doc = "Reservation register 0"]
pub mod store0;
#[doc = "STORE1 (rw) register accessor: an alias for `Reg<STORE1_SPEC>`"]
pub type STORE1 = crate::Reg<store1::STORE1_SPEC>;
#[doc = "Reservation register 1"]
pub mod store1;
#[doc = "STORE2 (rw) register accessor: an alias for `Reg<STORE2_SPEC>`"]
pub type STORE2 = crate::Reg<store2::STORE2_SPEC>;
#[doc = "Reservation register 2"]
pub mod store2;
#[doc = "STORE3 (rw) register accessor: an alias for `Reg<STORE3_SPEC>`"]
pub type STORE3 = crate::Reg<store3::STORE3_SPEC>;
#[doc = "Reservation register 3"]
pub mod store3;
#[doc = "EXT_XTL_CONF (rw) register accessor: an alias for `Reg<EXT_XTL_CONF_SPEC>`"]
pub type EXT_XTL_CONF = crate::Reg<ext_xtl_conf::EXT_XTL_CONF_SPEC>;
#[doc = "32 kHz crystal oscillator configuration register"]
pub mod ext_xtl_conf;
#[doc = "EXT_WAKEUP_CONF (rw) register accessor: an alias for `Reg<EXT_WAKEUP_CONF_SPEC>`"]
pub type EXT_WAKEUP_CONF = crate::Reg<ext_wakeup_conf::EXT_WAKEUP_CONF_SPEC>;
#[doc = "GPIO wakeup configuration register"]
pub mod ext_wakeup_conf;
#[doc = "SLP_REJECT_CONF (rw) register accessor: an alias for `Reg<SLP_REJECT_CONF_SPEC>`"]
pub type SLP_REJECT_CONF = crate::Reg<slp_reject_conf::SLP_REJECT_CONF_SPEC>;
#[doc = "Configures sleep / reject options"]
pub mod slp_reject_conf;
#[doc = "CPU_PERIOD_CONF (rw) register accessor: an alias for `Reg<CPU_PERIOD_CONF_SPEC>`"]
pub type CPU_PERIOD_CONF = crate::Reg<cpu_period_conf::CPU_PERIOD_CONF_SPEC>;
#[doc = "CPU sel option"]
pub mod cpu_period_conf;
#[doc = "SDIO_ACT_CONF (rw) register accessor: an alias for `Reg<SDIO_ACT_CONF_SPEC>`"]
pub type SDIO_ACT_CONF = crate::Reg<sdio_act_conf::SDIO_ACT_CONF_SPEC>;
#[doc = "configure sdio active register"]
pub mod sdio_act_conf;
#[doc = "CLK_CONF (rw) register accessor: an alias for `Reg<CLK_CONF_SPEC>`"]
pub type CLK_CONF = crate::Reg<clk_conf::CLK_CONF_SPEC>;
#[doc = "RTC clock configuration register"]
pub mod clk_conf;
#[doc = "SLOW_CLK_CONF (rw) register accessor: an alias for `Reg<SLOW_CLK_CONF_SPEC>`"]
pub type SLOW_CLK_CONF = crate::Reg<slow_clk_conf::SLOW_CLK_CONF_SPEC>;
#[doc = "RTC slow clock configuration register"]
pub mod slow_clk_conf;
#[doc = "SDIO_CONF (rw) register accessor: an alias for `Reg<SDIO_CONF_SPEC>`"]
pub type SDIO_CONF = crate::Reg<sdio_conf::SDIO_CONF_SPEC>;
#[doc = "configure vddsdio register"]
pub mod sdio_conf;
#[doc = "BIAS_CONF (rw) register accessor: an alias for `Reg<BIAS_CONF_SPEC>`"]
pub type BIAS_CONF = crate::Reg<bias_conf::BIAS_CONF_SPEC>;
#[doc = "configure power register"]
pub mod bias_conf;
#[doc = "REG (rw) register accessor: an alias for `Reg<REG_SPEC>`"]
pub type REG = crate::Reg<reg::REG_SPEC>;
#[doc = "RTC/DIG regulator configuration register"]
pub mod reg;
#[doc = "PWC (rw) register accessor: an alias for `Reg<PWC_SPEC>`"]
pub type PWC = crate::Reg<pwc::PWC_SPEC>;
#[doc = "RTC power configuraiton register"]
pub mod pwc;
#[doc = "DIG_PWC (rw) register accessor: an alias for `Reg<DIG_PWC_SPEC>`"]
pub type DIG_PWC = crate::Reg<dig_pwc::DIG_PWC_SPEC>;
#[doc = "Digital system power configuraiton register"]
pub mod dig_pwc;
#[doc = "DIG_ISO (rw) register accessor: an alias for `Reg<DIG_ISO_SPEC>`"]
pub type DIG_ISO = crate::Reg<dig_iso::DIG_ISO_SPEC>;
#[doc = "Digital system ISO configuration register"]
pub mod dig_iso;
#[doc = "WDTCONFIG0 (rw) register accessor: an alias for `Reg<WDTCONFIG0_SPEC>`"]
pub type WDTCONFIG0 = crate::Reg<wdtconfig0::WDTCONFIG0_SPEC>;
#[doc = "RTC watchdog configuration register"]
pub mod wdtconfig0;
#[doc = "WDTCONFIG1 (rw) register accessor: an alias for `Reg<WDTCONFIG1_SPEC>`"]
pub type WDTCONFIG1 = crate::Reg<wdtconfig1::WDTCONFIG1_SPEC>;
#[doc = "Configures the hold time of RTC watchdog at level 1"]
pub mod wdtconfig1;
#[doc = "WDTCONFIG2 (rw) register accessor: an alias for `Reg<WDTCONFIG2_SPEC>`"]
pub type WDTCONFIG2 = crate::Reg<wdtconfig2::WDTCONFIG2_SPEC>;
#[doc = "Configures the hold time of RTC watchdog at level 2"]
pub mod wdtconfig2;
#[doc = "WDTCONFIG3 (rw) register accessor: an alias for `Reg<WDTCONFIG3_SPEC>`"]
pub type WDTCONFIG3 = crate::Reg<wdtconfig3::WDTCONFIG3_SPEC>;
#[doc = "Configures the hold time of RTC watchdog at level 3"]
pub mod wdtconfig3;
#[doc = "WDTCONFIG4 (rw) register accessor: an alias for `Reg<WDTCONFIG4_SPEC>`"]
pub type WDTCONFIG4 = crate::Reg<wdtconfig4::WDTCONFIG4_SPEC>;
#[doc = "Configures the hold time of RTC watchdog at level 4"]
pub mod wdtconfig4;
#[doc = "WDTFEED (w) register accessor: an alias for `Reg<WDTFEED_SPEC>`"]
pub type WDTFEED = crate::Reg<wdtfeed::WDTFEED_SPEC>;
#[doc = "RTC watchdog SW feed configuration register"]
pub mod wdtfeed;
#[doc = "WDTWPROTECT (rw) register accessor: an alias for `Reg<WDTWPROTECT_SPEC>`"]
pub type WDTWPROTECT = crate::Reg<wdtwprotect::WDTWPROTECT_SPEC>;
#[doc = "RTC watchdog write protection configuration register"]
pub mod wdtwprotect;
#[doc = "SWD_CONF (rw) register accessor: an alias for `Reg<SWD_CONF_SPEC>`"]
pub type SWD_CONF = crate::Reg<swd_conf::SWD_CONF_SPEC>;
#[doc = "Super watchdog configuration register"]
pub mod swd_conf;
#[doc = "SWD_WPROTECT (rw) register accessor: an alias for `Reg<SWD_WPROTECT_SPEC>`"]
pub type SWD_WPROTECT = crate::Reg<swd_wprotect::SWD_WPROTECT_SPEC>;
#[doc = "Super watchdog write protection configuration register"]
pub mod swd_wprotect;
#[doc = "SW_CPU_STALL (rw) register accessor: an alias for `Reg<SW_CPU_STALL_SPEC>`"]
pub type SW_CPU_STALL = crate::Reg<sw_cpu_stall::SW_CPU_STALL_SPEC>;
#[doc = "CPU stall configuration register"]
pub mod sw_cpu_stall;
#[doc = "STORE4 (rw) register accessor: an alias for `Reg<STORE4_SPEC>`"]
pub type STORE4 = crate::Reg<store4::STORE4_SPEC>;
#[doc = "Reservation register 4"]
pub mod store4;
#[doc = "STORE5 (rw) register accessor: an alias for `Reg<STORE5_SPEC>`"]
pub type STORE5 = crate::Reg<store5::STORE5_SPEC>;
#[doc = "Reservation register 5"]
pub mod store5;
#[doc = "STORE6 (rw) register accessor: an alias for `Reg<STORE6_SPEC>`"]
pub type STORE6 = crate::Reg<store6::STORE6_SPEC>;
#[doc = "Reservation register 6"]
pub mod store6;
#[doc = "STORE7 (rw) register accessor: an alias for `Reg<STORE7_SPEC>`"]
pub type STORE7 = crate::Reg<store7::STORE7_SPEC>;
#[doc = "Reservation register 7"]
pub mod store7;
#[doc = "LOW_POWER_ST (r) register accessor: an alias for `Reg<LOW_POWER_ST_SPEC>`"]
pub type LOW_POWER_ST = crate::Reg<low_power_st::LOW_POWER_ST_SPEC>;
#[doc = "RTC main state machine status register"]
pub mod low_power_st;
#[doc = "DIAG0 (r) register accessor: an alias for `Reg<DIAG0_SPEC>`"]
pub type DIAG0 = crate::Reg<diag0::DIAG0_SPEC>;
#[doc = "debug register"]
pub mod diag0;
#[doc = "PAD_HOLD (rw) register accessor: an alias for `Reg<PAD_HOLD_SPEC>`"]
pub type PAD_HOLD = crate::Reg<pad_hold::PAD_HOLD_SPEC>;
#[doc = "Configures the hold options for RTC GPIOs"]
pub mod pad_hold;
#[doc = "DIG_PAD_HOLD (rw) register accessor: an alias for `Reg<DIG_PAD_HOLD_SPEC>`"]
pub type DIG_PAD_HOLD = crate::Reg<dig_pad_hold::DIG_PAD_HOLD_SPEC>;
#[doc = "Configures the hold option for digital GPIOs"]
pub mod dig_pad_hold;
#[doc = "EXT_WAKEUP1 (rw) register accessor: an alias for `Reg<EXT_WAKEUP1_SPEC>`"]
pub type EXT_WAKEUP1 = crate::Reg<ext_wakeup1::EXT_WAKEUP1_SPEC>;
#[doc = "EXT1 wakeup configuration register"]
pub mod ext_wakeup1;
#[doc = "EXT_WAKEUP1_STATUS (r) register accessor: an alias for `Reg<EXT_WAKEUP1_STATUS_SPEC>`"]
pub type EXT_WAKEUP1_STATUS = crate::Reg<ext_wakeup1_status::EXT_WAKEUP1_STATUS_SPEC>;
#[doc = "EXT1 wakeup source register"]
pub mod ext_wakeup1_status;
#[doc = "BROWN_OUT (rw) register accessor: an alias for `Reg<BROWN_OUT_SPEC>`"]
pub type BROWN_OUT = crate::Reg<brown_out::BROWN_OUT_SPEC>;
#[doc = "Brownout configuration register"]
pub mod brown_out;
#[doc = "TIME_LOW1 (r) register accessor: an alias for `Reg<TIME_LOW1_SPEC>`"]
pub type TIME_LOW1 = crate::Reg<time_low1::TIME_LOW1_SPEC>;
#[doc = "Stores the lower 32 bits of RTC timer 1"]
pub mod time_low1;
#[doc = "TIME_HIGH1 (r) register accessor: an alias for `Reg<TIME_HIGH1_SPEC>`"]
pub type TIME_HIGH1 = crate::Reg<time_high1::TIME_HIGH1_SPEC>;
#[doc = "Stores the higher 16 bits of RTC timer 1"]
pub mod time_high1;
#[doc = "XTAL32K_CLK_FACTOR (rw) register accessor: an alias for `Reg<XTAL32K_CLK_FACTOR_SPEC>`"]
pub type XTAL32K_CLK_FACTOR = crate::Reg<xtal32k_clk_factor::XTAL32K_CLK_FACTOR_SPEC>;
#[doc = "Configures the divider factor for the backup clock of 32 kHz crystal oscillator"]
pub mod xtal32k_clk_factor;
#[doc = "XTAL32K_CONF (rw) register accessor: an alias for `Reg<XTAL32K_CONF_SPEC>`"]
pub type XTAL32K_CONF = crate::Reg<xtal32k_conf::XTAL32K_CONF_SPEC>;
#[doc = "32 kHz crystal oscillator configuration register"]
pub mod xtal32k_conf;
#[doc = "ULP_CP_TIMER (rw) register accessor: an alias for `Reg<ULP_CP_TIMER_SPEC>`"]
pub type ULP_CP_TIMER = crate::Reg<ulp_cp_timer::ULP_CP_TIMER_SPEC>;
#[doc = "Configure coprocessor timer"]
pub mod ulp_cp_timer;
#[doc = "ULP_CP_CTRL (rw) register accessor: an alias for `Reg<ULP_CP_CTRL_SPEC>`"]
pub type ULP_CP_CTRL = crate::Reg<ulp_cp_ctrl::ULP_CP_CTRL_SPEC>;
#[doc = "ULP-FSM configuration register"]
pub mod ulp_cp_ctrl;
#[doc = "COCPU_CTRL (rw) register accessor: an alias for `Reg<COCPU_CTRL_SPEC>`"]
pub type COCPU_CTRL = crate::Reg<cocpu_ctrl::COCPU_CTRL_SPEC>;
#[doc = "ULP-RISCV configuration register"]
pub mod cocpu_ctrl;
#[doc = "TOUCH_CTRL1 (rw) register accessor: an alias for `Reg<TOUCH_CTRL1_SPEC>`"]
pub type TOUCH_CTRL1 = crate::Reg<touch_ctrl1::TOUCH_CTRL1_SPEC>;
#[doc = "Touch control register"]
pub mod touch_ctrl1;
#[doc = "TOUCH_CTRL2 (rw) register accessor: an alias for `Reg<TOUCH_CTRL2_SPEC>`"]
pub type TOUCH_CTRL2 = crate::Reg<touch_ctrl2::TOUCH_CTRL2_SPEC>;
#[doc = "Touch control register"]
pub mod touch_ctrl2;
#[doc = "TOUCH_SCAN_CTRL (rw) register accessor: an alias for `Reg<TOUCH_SCAN_CTRL_SPEC>`"]
pub type TOUCH_SCAN_CTRL = crate::Reg<touch_scan_ctrl::TOUCH_SCAN_CTRL_SPEC>;
#[doc = "Configure touch scan settings"]
pub mod touch_scan_ctrl;
#[doc = "TOUCH_SLP_THRES (rw) register accessor: an alias for `Reg<TOUCH_SLP_THRES_SPEC>`"]
pub type TOUCH_SLP_THRES = crate::Reg<touch_slp_thres::TOUCH_SLP_THRES_SPEC>;
#[doc = "Configure the settings of touch sleep pad"]
pub mod touch_slp_thres;
#[doc = "TOUCH_APPROACH (rw) register accessor: an alias for `Reg<TOUCH_APPROACH_SPEC>`"]
pub type TOUCH_APPROACH = crate::Reg<touch_approach::TOUCH_APPROACH_SPEC>;
#[doc = "Configure touch approach settings"]
pub mod touch_approach;
#[doc = "TOUCH_FILTER_CTRL (rw) register accessor: an alias for `Reg<TOUCH_FILTER_CTRL_SPEC>`"]
pub type TOUCH_FILTER_CTRL = crate::Reg<touch_filter_ctrl::TOUCH_FILTER_CTRL_SPEC>;
#[doc = "Configure touch filter settings"]
pub mod touch_filter_ctrl;
#[doc = "USB_CONF (rw) register accessor: an alias for `Reg<USB_CONF_SPEC>`"]
pub type USB_CONF = crate::Reg<usb_conf::USB_CONF_SPEC>;
#[doc = "configure usb control register"]
pub mod usb_conf;
#[doc = "TOUCH_TIMEOUT_CTRL (rw) register accessor: an alias for `Reg<TOUCH_TIMEOUT_CTRL_SPEC>`"]
pub type TOUCH_TIMEOUT_CTRL = crate::Reg<touch_timeout_ctrl::TOUCH_TIMEOUT_CTRL_SPEC>;
#[doc = "Configure touch timeout settings"]
pub mod touch_timeout_ctrl;
#[doc = "SLP_REJECT_CAUSE (r) register accessor: an alias for `Reg<SLP_REJECT_CAUSE_SPEC>`"]
pub type SLP_REJECT_CAUSE = crate::Reg<slp_reject_cause::SLP_REJECT_CAUSE_SPEC>;
#[doc = "Stores the reject-to-sleep cause."]
pub mod slp_reject_cause;
#[doc = "OPTIONS1 (rw) register accessor: an alias for `Reg<OPTIONS1_SPEC>`"]
pub type OPTIONS1 = crate::Reg<options1::OPTIONS1_SPEC>;
#[doc = "RTC option register"]
pub mod options1;
#[doc = "SLP_WAKEUP_CAUSE (r) register accessor: an alias for `Reg<SLP_WAKEUP_CAUSE_SPEC>`"]
pub type SLP_WAKEUP_CAUSE = crate::Reg<slp_wakeup_cause::SLP_WAKEUP_CAUSE_SPEC>;
#[doc = "Stores the sleep-to-wakeup cause."]
pub mod slp_wakeup_cause;
#[doc = "ULP_CP_TIMER_1 (rw) register accessor: an alias for `Reg<ULP_CP_TIMER_1_SPEC>`"]
pub type ULP_CP_TIMER_1 = crate::Reg<ulp_cp_timer_1::ULP_CP_TIMER_1_SPEC>;
#[doc = "Configure sleep cycle of the timer"]
pub mod ulp_cp_timer_1;
#[doc = "DATE (rw) register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = ""]
pub mod date;
