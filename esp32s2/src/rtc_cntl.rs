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
#[doc = "OPTIONS0 (rw) register accessor: Sets the power options of crystal and PLL clocks, and initiates reset by software\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`options0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`options0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`options0`] module"]
pub type OPTIONS0 = crate::Reg<options0::OPTIONS0_SPEC>;
#[doc = "Sets the power options of crystal and PLL clocks, and initiates reset by software"]
pub mod options0;
#[doc = "SLP_TIMER0 (rw) register accessor: RTC timer threshold register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slp_timer0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slp_timer0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`slp_timer0`] module"]
pub type SLP_TIMER0 = crate::Reg<slp_timer0::SLP_TIMER0_SPEC>;
#[doc = "RTC timer threshold register 0"]
pub mod slp_timer0;
#[doc = "SLP_TIMER1 (rw) register accessor: RTC timer threshold register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slp_timer1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slp_timer1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`slp_timer1`] module"]
pub type SLP_TIMER1 = crate::Reg<slp_timer1::SLP_TIMER1_SPEC>;
#[doc = "RTC timer threshold register 1"]
pub mod slp_timer1;
#[doc = "TIME_UPDATE (rw) register accessor: RTC timer update control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`time_update::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`time_update::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`time_update`] module"]
pub type TIME_UPDATE = crate::Reg<time_update::TIME_UPDATE_SPEC>;
#[doc = "RTC timer update control register"]
pub mod time_update;
#[doc = "TIME_LOW0 (r) register accessor: Stores the lower 32 bits of RTC timer 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`time_low0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`time_low0`] module"]
pub type TIME_LOW0 = crate::Reg<time_low0::TIME_LOW0_SPEC>;
#[doc = "Stores the lower 32 bits of RTC timer 0."]
pub mod time_low0;
#[doc = "TIME_HIGH0 (r) register accessor: Stores the higher 16 bits of RTC timer 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`time_high0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`time_high0`] module"]
pub type TIME_HIGH0 = crate::Reg<time_high0::TIME_HIGH0_SPEC>;
#[doc = "Stores the higher 16 bits of RTC timer 0"]
pub mod time_high0;
#[doc = "STATE0 (rw) register accessor: Configures the sleep / reject / wakeup state\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`state0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`state0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`state0`] module"]
pub type STATE0 = crate::Reg<state0::STATE0_SPEC>;
#[doc = "Configures the sleep / reject / wakeup state"]
pub mod state0;
#[doc = "TIMER1 (rw) register accessor: Configures CPU stall options\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`timer1`] module"]
pub type TIMER1 = crate::Reg<timer1::TIMER1_SPEC>;
#[doc = "Configures CPU stall options"]
pub mod timer1;
#[doc = "TIMER2 (rw) register accessor: Configures RTC slow clock and touch controller\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`timer2`] module"]
pub type TIMER2 = crate::Reg<timer2::TIMER2_SPEC>;
#[doc = "Configures RTC slow clock and touch controller"]
pub mod timer2;
#[doc = "TIMER3 (rw) register accessor: configure some wait time for power on\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`timer3`] module"]
pub type TIMER3 = crate::Reg<timer3::TIMER3_SPEC>;
#[doc = "configure some wait time for power on"]
pub mod timer3;
#[doc = "TIMER4 (rw) register accessor: configure some wait time for power on\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`timer4`] module"]
pub type TIMER4 = crate::Reg<timer4::TIMER4_SPEC>;
#[doc = "configure some wait time for power on"]
pub mod timer4;
#[doc = "TIMER5 (rw) register accessor: Configures the minimal sleep cycles\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`timer5`] module"]
pub type TIMER5 = crate::Reg<timer5::TIMER5_SPEC>;
#[doc = "Configures the minimal sleep cycles"]
pub mod timer5;
#[doc = "TIMER6 (rw) register accessor: Configure minimal sleep cycles register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`timer6`] module"]
pub type TIMER6 = crate::Reg<timer6::TIMER6_SPEC>;
#[doc = "Configure minimal sleep cycles register"]
pub mod timer6;
#[doc = "ANA_CONF (rw) register accessor: Configures the power options for I2C and PLLA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ana_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ana_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ana_conf`] module"]
pub type ANA_CONF = crate::Reg<ana_conf::ANA_CONF_SPEC>;
#[doc = "Configures the power options for I2C and PLLA"]
pub mod ana_conf;
#[doc = "RESET_STATE (rw) register accessor: Indicates the CPU reset source. For more information about the reset cause, please refer to Table \\ref{table:resetreasons} in Chapter \\ref{module:ResetandClock} \\textit{\nameref{module:ResetandClock}}.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reset_state::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reset_state::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`reset_state`] module"]
pub type RESET_STATE = crate::Reg<reset_state::RESET_STATE_SPEC>;
#[doc = "Indicates the CPU reset source. For more information about the reset cause, please refer to Table \\ref{table:resetreasons} in Chapter \\ref{module:ResetandClock} \\textit{\nameref{module:ResetandClock}}."]
pub mod reset_state;
#[doc = "WAKEUP_STATE (rw) register accessor: Wakeup bitmap enabling register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wakeup_state::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wakeup_state::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`wakeup_state`] module"]
pub type WAKEUP_STATE = crate::Reg<wakeup_state::WAKEUP_STATE_SPEC>;
#[doc = "Wakeup bitmap enabling register"]
pub mod wakeup_state;
#[doc = "INT_ENA_RTC (rw) register accessor: RTC interrupt enabling register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_ena_rtc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena_rtc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`int_ena_rtc`] module"]
pub type INT_ENA_RTC = crate::Reg<int_ena_rtc::INT_ENA_RTC_SPEC>;
#[doc = "RTC interrupt enabling register"]
pub mod int_ena_rtc;
#[doc = "INT_RAW_RTC (r) register accessor: RTC interrupt raw register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_raw_rtc::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`int_raw_rtc`] module"]
pub type INT_RAW_RTC = crate::Reg<int_raw_rtc::INT_RAW_RTC_SPEC>;
#[doc = "RTC interrupt raw register"]
pub mod int_raw_rtc;
#[doc = "INT_ST_RTC (r) register accessor: RTC interrupt state register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_st_rtc::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`int_st_rtc`] module"]
pub type INT_ST_RTC = crate::Reg<int_st_rtc::INT_ST_RTC_SPEC>;
#[doc = "RTC interrupt state register"]
pub mod int_st_rtc;
#[doc = "INT_CLR_RTC (w) register accessor: RTC interrupt clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr_rtc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`int_clr_rtc`] module"]
pub type INT_CLR_RTC = crate::Reg<int_clr_rtc::INT_CLR_RTC_SPEC>;
#[doc = "RTC interrupt clear register"]
pub mod int_clr_rtc;
#[doc = "STORE0 (rw) register accessor: Reservation register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`store0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`store0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`store0`] module"]
pub type STORE0 = crate::Reg<store0::STORE0_SPEC>;
#[doc = "Reservation register 0"]
pub mod store0;
#[doc = "STORE1 (rw) register accessor: Reservation register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`store1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`store1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`store1`] module"]
pub type STORE1 = crate::Reg<store1::STORE1_SPEC>;
#[doc = "Reservation register 1"]
pub mod store1;
#[doc = "STORE2 (rw) register accessor: Reservation register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`store2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`store2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`store2`] module"]
pub type STORE2 = crate::Reg<store2::STORE2_SPEC>;
#[doc = "Reservation register 2"]
pub mod store2;
#[doc = "STORE3 (rw) register accessor: Reservation register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`store3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`store3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`store3`] module"]
pub type STORE3 = crate::Reg<store3::STORE3_SPEC>;
#[doc = "Reservation register 3"]
pub mod store3;
#[doc = "EXT_XTL_CONF (rw) register accessor: 32 kHz crystal oscillator configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ext_xtl_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ext_xtl_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ext_xtl_conf`] module"]
pub type EXT_XTL_CONF = crate::Reg<ext_xtl_conf::EXT_XTL_CONF_SPEC>;
#[doc = "32 kHz crystal oscillator configuration register"]
pub mod ext_xtl_conf;
#[doc = "EXT_WAKEUP_CONF (rw) register accessor: GPIO wakeup configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ext_wakeup_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ext_wakeup_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ext_wakeup_conf`] module"]
pub type EXT_WAKEUP_CONF = crate::Reg<ext_wakeup_conf::EXT_WAKEUP_CONF_SPEC>;
#[doc = "GPIO wakeup configuration register"]
pub mod ext_wakeup_conf;
#[doc = "SLP_REJECT_CONF (rw) register accessor: Configures sleep / reject options\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slp_reject_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slp_reject_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`slp_reject_conf`] module"]
pub type SLP_REJECT_CONF = crate::Reg<slp_reject_conf::SLP_REJECT_CONF_SPEC>;
#[doc = "Configures sleep / reject options"]
pub mod slp_reject_conf;
#[doc = "CPU_PERIOD_CONF (rw) register accessor: CPU sel option\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_period_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_period_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cpu_period_conf`] module"]
pub type CPU_PERIOD_CONF = crate::Reg<cpu_period_conf::CPU_PERIOD_CONF_SPEC>;
#[doc = "CPU sel option"]
pub mod cpu_period_conf;
#[doc = "SDIO_ACT_CONF (rw) register accessor: configure sdio active register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdio_act_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdio_act_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sdio_act_conf`] module"]
pub type SDIO_ACT_CONF = crate::Reg<sdio_act_conf::SDIO_ACT_CONF_SPEC>;
#[doc = "configure sdio active register"]
pub mod sdio_act_conf;
#[doc = "CLK_CONF (rw) register accessor: RTC clock configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`clk_conf`] module"]
pub type CLK_CONF = crate::Reg<clk_conf::CLK_CONF_SPEC>;
#[doc = "RTC clock configuration register"]
pub mod clk_conf;
#[doc = "SLOW_CLK_CONF (rw) register accessor: RTC slow clock configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slow_clk_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slow_clk_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`slow_clk_conf`] module"]
pub type SLOW_CLK_CONF = crate::Reg<slow_clk_conf::SLOW_CLK_CONF_SPEC>;
#[doc = "RTC slow clock configuration register"]
pub mod slow_clk_conf;
#[doc = "SDIO_CONF (rw) register accessor: configure vddsdio register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdio_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdio_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sdio_conf`] module"]
pub type SDIO_CONF = crate::Reg<sdio_conf::SDIO_CONF_SPEC>;
#[doc = "configure vddsdio register"]
pub mod sdio_conf;
#[doc = "BIAS_CONF (rw) register accessor: configure power register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bias_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bias_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`bias_conf`] module"]
pub type BIAS_CONF = crate::Reg<bias_conf::BIAS_CONF_SPEC>;
#[doc = "configure power register"]
pub mod bias_conf;
#[doc = "REG (rw) register accessor: RTC/DIG regulator configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`reg`] module"]
pub type REG = crate::Reg<reg::REG_SPEC>;
#[doc = "RTC/DIG regulator configuration register"]
pub mod reg;
#[doc = "PWC (rw) register accessor: RTC power configuraiton register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pwc`] module"]
pub type PWC = crate::Reg<pwc::PWC_SPEC>;
#[doc = "RTC power configuraiton register"]
pub mod pwc;
#[doc = "DIG_PWC (rw) register accessor: Digital system power configuraiton register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dig_pwc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dig_pwc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dig_pwc`] module"]
pub type DIG_PWC = crate::Reg<dig_pwc::DIG_PWC_SPEC>;
#[doc = "Digital system power configuraiton register"]
pub mod dig_pwc;
#[doc = "DIG_ISO (rw) register accessor: Digital system ISO configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dig_iso::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dig_iso::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dig_iso`] module"]
pub type DIG_ISO = crate::Reg<dig_iso::DIG_ISO_SPEC>;
#[doc = "Digital system ISO configuration register"]
pub mod dig_iso;
#[doc = "WDTCONFIG0 (rw) register accessor: RTC watchdog configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtconfig0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtconfig0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`wdtconfig0`] module"]
pub type WDTCONFIG0 = crate::Reg<wdtconfig0::WDTCONFIG0_SPEC>;
#[doc = "RTC watchdog configuration register"]
pub mod wdtconfig0;
#[doc = "WDTCONFIG1 (rw) register accessor: Configures the hold time of RTC watchdog at level 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtconfig1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtconfig1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`wdtconfig1`] module"]
pub type WDTCONFIG1 = crate::Reg<wdtconfig1::WDTCONFIG1_SPEC>;
#[doc = "Configures the hold time of RTC watchdog at level 1"]
pub mod wdtconfig1;
#[doc = "WDTCONFIG2 (rw) register accessor: Configures the hold time of RTC watchdog at level 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtconfig2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtconfig2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`wdtconfig2`] module"]
pub type WDTCONFIG2 = crate::Reg<wdtconfig2::WDTCONFIG2_SPEC>;
#[doc = "Configures the hold time of RTC watchdog at level 2"]
pub mod wdtconfig2;
#[doc = "WDTCONFIG3 (rw) register accessor: Configures the hold time of RTC watchdog at level 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtconfig3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtconfig3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`wdtconfig3`] module"]
pub type WDTCONFIG3 = crate::Reg<wdtconfig3::WDTCONFIG3_SPEC>;
#[doc = "Configures the hold time of RTC watchdog at level 3"]
pub mod wdtconfig3;
#[doc = "WDTCONFIG4 (rw) register accessor: Configures the hold time of RTC watchdog at level 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtconfig4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtconfig4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`wdtconfig4`] module"]
pub type WDTCONFIG4 = crate::Reg<wdtconfig4::WDTCONFIG4_SPEC>;
#[doc = "Configures the hold time of RTC watchdog at level 4"]
pub mod wdtconfig4;
#[doc = "WDTFEED (w) register accessor: RTC watchdog SW feed configuration register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtfeed::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`wdtfeed`] module"]
pub type WDTFEED = crate::Reg<wdtfeed::WDTFEED_SPEC>;
#[doc = "RTC watchdog SW feed configuration register"]
pub mod wdtfeed;
#[doc = "WDTWPROTECT (rw) register accessor: RTC watchdog write protection configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtwprotect::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtwprotect::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`wdtwprotect`] module"]
pub type WDTWPROTECT = crate::Reg<wdtwprotect::WDTWPROTECT_SPEC>;
#[doc = "RTC watchdog write protection configuration register"]
pub mod wdtwprotect;
#[doc = "SWD_CONF (rw) register accessor: Super watchdog configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swd_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swd_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`swd_conf`] module"]
pub type SWD_CONF = crate::Reg<swd_conf::SWD_CONF_SPEC>;
#[doc = "Super watchdog configuration register"]
pub mod swd_conf;
#[doc = "SWD_WPROTECT (rw) register accessor: Super watchdog write protection configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swd_wprotect::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swd_wprotect::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`swd_wprotect`] module"]
pub type SWD_WPROTECT = crate::Reg<swd_wprotect::SWD_WPROTECT_SPEC>;
#[doc = "Super watchdog write protection configuration register"]
pub mod swd_wprotect;
#[doc = "SW_CPU_STALL (rw) register accessor: CPU stall configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sw_cpu_stall::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sw_cpu_stall::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sw_cpu_stall`] module"]
pub type SW_CPU_STALL = crate::Reg<sw_cpu_stall::SW_CPU_STALL_SPEC>;
#[doc = "CPU stall configuration register"]
pub mod sw_cpu_stall;
#[doc = "STORE4 (rw) register accessor: Reservation register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`store4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`store4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`store4`] module"]
pub type STORE4 = crate::Reg<store4::STORE4_SPEC>;
#[doc = "Reservation register 4"]
pub mod store4;
#[doc = "STORE5 (rw) register accessor: Reservation register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`store5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`store5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`store5`] module"]
pub type STORE5 = crate::Reg<store5::STORE5_SPEC>;
#[doc = "Reservation register 5"]
pub mod store5;
#[doc = "STORE6 (rw) register accessor: Reservation register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`store6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`store6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`store6`] module"]
pub type STORE6 = crate::Reg<store6::STORE6_SPEC>;
#[doc = "Reservation register 6"]
pub mod store6;
#[doc = "STORE7 (rw) register accessor: Reservation register 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`store7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`store7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`store7`] module"]
pub type STORE7 = crate::Reg<store7::STORE7_SPEC>;
#[doc = "Reservation register 7"]
pub mod store7;
#[doc = "LOW_POWER_ST (r) register accessor: RTC main state machine status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`low_power_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`low_power_st`] module"]
pub type LOW_POWER_ST = crate::Reg<low_power_st::LOW_POWER_ST_SPEC>;
#[doc = "RTC main state machine status register"]
pub mod low_power_st;
#[doc = "DIAG0 (r) register accessor: debug register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diag0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diag0`] module"]
pub type DIAG0 = crate::Reg<diag0::DIAG0_SPEC>;
#[doc = "debug register"]
pub mod diag0;
#[doc = "PAD_HOLD (rw) register accessor: Configures the hold options for RTC GPIOs\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pad_hold::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pad_hold::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pad_hold`] module"]
pub type PAD_HOLD = crate::Reg<pad_hold::PAD_HOLD_SPEC>;
#[doc = "Configures the hold options for RTC GPIOs"]
pub mod pad_hold;
#[doc = "DIG_PAD_HOLD (rw) register accessor: Configures the hold option for digital GPIOs\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dig_pad_hold::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dig_pad_hold::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dig_pad_hold`] module"]
pub type DIG_PAD_HOLD = crate::Reg<dig_pad_hold::DIG_PAD_HOLD_SPEC>;
#[doc = "Configures the hold option for digital GPIOs"]
pub mod dig_pad_hold;
#[doc = "EXT_WAKEUP1 (rw) register accessor: EXT1 wakeup configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ext_wakeup1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ext_wakeup1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ext_wakeup1`] module"]
pub type EXT_WAKEUP1 = crate::Reg<ext_wakeup1::EXT_WAKEUP1_SPEC>;
#[doc = "EXT1 wakeup configuration register"]
pub mod ext_wakeup1;
#[doc = "EXT_WAKEUP1_STATUS (r) register accessor: EXT1 wakeup source register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ext_wakeup1_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ext_wakeup1_status`] module"]
pub type EXT_WAKEUP1_STATUS = crate::Reg<ext_wakeup1_status::EXT_WAKEUP1_STATUS_SPEC>;
#[doc = "EXT1 wakeup source register"]
pub mod ext_wakeup1_status;
#[doc = "BROWN_OUT (rw) register accessor: Brownout configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`brown_out::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`brown_out::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`brown_out`] module"]
pub type BROWN_OUT = crate::Reg<brown_out::BROWN_OUT_SPEC>;
#[doc = "Brownout configuration register"]
pub mod brown_out;
#[doc = "TIME_LOW1 (r) register accessor: Stores the lower 32 bits of RTC timer 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`time_low1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`time_low1`] module"]
pub type TIME_LOW1 = crate::Reg<time_low1::TIME_LOW1_SPEC>;
#[doc = "Stores the lower 32 bits of RTC timer 1"]
pub mod time_low1;
#[doc = "TIME_HIGH1 (r) register accessor: Stores the higher 16 bits of RTC timer 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`time_high1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`time_high1`] module"]
pub type TIME_HIGH1 = crate::Reg<time_high1::TIME_HIGH1_SPEC>;
#[doc = "Stores the higher 16 bits of RTC timer 1"]
pub mod time_high1;
#[doc = "XTAL32K_CLK_FACTOR (rw) register accessor: Configures the divider factor for the backup clock of 32 kHz crystal oscillator\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`xtal32k_clk_factor::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`xtal32k_clk_factor::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`xtal32k_clk_factor`] module"]
pub type XTAL32K_CLK_FACTOR = crate::Reg<xtal32k_clk_factor::XTAL32K_CLK_FACTOR_SPEC>;
#[doc = "Configures the divider factor for the backup clock of 32 kHz crystal oscillator"]
pub mod xtal32k_clk_factor;
#[doc = "XTAL32K_CONF (rw) register accessor: 32 kHz crystal oscillator configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`xtal32k_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`xtal32k_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`xtal32k_conf`] module"]
pub type XTAL32K_CONF = crate::Reg<xtal32k_conf::XTAL32K_CONF_SPEC>;
#[doc = "32 kHz crystal oscillator configuration register"]
pub mod xtal32k_conf;
#[doc = "ULP_CP_TIMER (rw) register accessor: Configure coprocessor timer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ulp_cp_timer::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ulp_cp_timer::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ulp_cp_timer`] module"]
pub type ULP_CP_TIMER = crate::Reg<ulp_cp_timer::ULP_CP_TIMER_SPEC>;
#[doc = "Configure coprocessor timer"]
pub mod ulp_cp_timer;
#[doc = "ULP_CP_CTRL (rw) register accessor: ULP-FSM configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ulp_cp_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ulp_cp_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ulp_cp_ctrl`] module"]
pub type ULP_CP_CTRL = crate::Reg<ulp_cp_ctrl::ULP_CP_CTRL_SPEC>;
#[doc = "ULP-FSM configuration register"]
pub mod ulp_cp_ctrl;
#[doc = "COCPU_CTRL (rw) register accessor: ULP-RISCV configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cocpu_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cocpu_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cocpu_ctrl`] module"]
pub type COCPU_CTRL = crate::Reg<cocpu_ctrl::COCPU_CTRL_SPEC>;
#[doc = "ULP-RISCV configuration register"]
pub mod cocpu_ctrl;
#[doc = "TOUCH_CTRL1 (rw) register accessor: Touch control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`touch_ctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`touch_ctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`touch_ctrl1`] module"]
pub type TOUCH_CTRL1 = crate::Reg<touch_ctrl1::TOUCH_CTRL1_SPEC>;
#[doc = "Touch control register"]
pub mod touch_ctrl1;
#[doc = "TOUCH_CTRL2 (rw) register accessor: Touch control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`touch_ctrl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`touch_ctrl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`touch_ctrl2`] module"]
pub type TOUCH_CTRL2 = crate::Reg<touch_ctrl2::TOUCH_CTRL2_SPEC>;
#[doc = "Touch control register"]
pub mod touch_ctrl2;
#[doc = "TOUCH_SCAN_CTRL (rw) register accessor: Configure touch scan settings\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`touch_scan_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`touch_scan_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`touch_scan_ctrl`] module"]
pub type TOUCH_SCAN_CTRL = crate::Reg<touch_scan_ctrl::TOUCH_SCAN_CTRL_SPEC>;
#[doc = "Configure touch scan settings"]
pub mod touch_scan_ctrl;
#[doc = "TOUCH_SLP_THRES (rw) register accessor: Configure the settings of touch sleep pad\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`touch_slp_thres::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`touch_slp_thres::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`touch_slp_thres`] module"]
pub type TOUCH_SLP_THRES = crate::Reg<touch_slp_thres::TOUCH_SLP_THRES_SPEC>;
#[doc = "Configure the settings of touch sleep pad"]
pub mod touch_slp_thres;
#[doc = "TOUCH_APPROACH (rw) register accessor: Configure touch approach settings\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`touch_approach::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`touch_approach::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`touch_approach`] module"]
pub type TOUCH_APPROACH = crate::Reg<touch_approach::TOUCH_APPROACH_SPEC>;
#[doc = "Configure touch approach settings"]
pub mod touch_approach;
#[doc = "TOUCH_FILTER_CTRL (rw) register accessor: Configure touch filter settings\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`touch_filter_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`touch_filter_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`touch_filter_ctrl`] module"]
pub type TOUCH_FILTER_CTRL = crate::Reg<touch_filter_ctrl::TOUCH_FILTER_CTRL_SPEC>;
#[doc = "Configure touch filter settings"]
pub mod touch_filter_ctrl;
#[doc = "USB_CONF (rw) register accessor: configure usb control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`usb_conf`] module"]
pub type USB_CONF = crate::Reg<usb_conf::USB_CONF_SPEC>;
#[doc = "configure usb control register"]
pub mod usb_conf;
#[doc = "TOUCH_TIMEOUT_CTRL (rw) register accessor: Configure touch timeout settings\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`touch_timeout_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`touch_timeout_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`touch_timeout_ctrl`] module"]
pub type TOUCH_TIMEOUT_CTRL = crate::Reg<touch_timeout_ctrl::TOUCH_TIMEOUT_CTRL_SPEC>;
#[doc = "Configure touch timeout settings"]
pub mod touch_timeout_ctrl;
#[doc = "SLP_REJECT_CAUSE (r) register accessor: Stores the reject-to-sleep cause.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slp_reject_cause::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`slp_reject_cause`] module"]
pub type SLP_REJECT_CAUSE = crate::Reg<slp_reject_cause::SLP_REJECT_CAUSE_SPEC>;
#[doc = "Stores the reject-to-sleep cause."]
pub mod slp_reject_cause;
#[doc = "OPTIONS1 (rw) register accessor: RTC option register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`options1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`options1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`options1`] module"]
pub type OPTIONS1 = crate::Reg<options1::OPTIONS1_SPEC>;
#[doc = "RTC option register"]
pub mod options1;
#[doc = "SLP_WAKEUP_CAUSE (r) register accessor: Stores the sleep-to-wakeup cause.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slp_wakeup_cause::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`slp_wakeup_cause`] module"]
pub type SLP_WAKEUP_CAUSE = crate::Reg<slp_wakeup_cause::SLP_WAKEUP_CAUSE_SPEC>;
#[doc = "Stores the sleep-to-wakeup cause."]
pub mod slp_wakeup_cause;
#[doc = "ULP_CP_TIMER_1 (rw) register accessor: Configure sleep cycle of the timer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ulp_cp_timer_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ulp_cp_timer_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ulp_cp_timer_1`] module"]
pub type ULP_CP_TIMER_1 = crate::Reg<ulp_cp_timer_1::ULP_CP_TIMER_1_SPEC>;
#[doc = "Configure sleep cycle of the timer"]
pub mod ulp_cp_timer_1;
#[doc = "DATE (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = ""]
pub mod date;
