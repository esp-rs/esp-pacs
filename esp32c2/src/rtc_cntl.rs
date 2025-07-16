#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    options0: OPTIONS0,
    slp_timer0: SLP_TIMER0,
    slp_timer1: SLP_TIMER1,
    time_update: TIME_UPDATE,
    time_low0: TIME_LOW0,
    time_high0: TIME_HIGH0,
    state0: STATE0,
    timer1: TIMER1,
    timer2: TIMER2,
    timer4: TIMER4,
    timer5: TIMER5,
    ana_conf: ANA_CONF,
    reset_state: RESET_STATE,
    wakeup_state: WAKEUP_STATE,
    int_ena: INT_ENA,
    int_raw: INT_RAW,
    int_st: INT_ST,
    int_clr: INT_CLR,
    store0: STORE0,
    store1: STORE1,
    store2: STORE2,
    store3: STORE3,
    ext_xtl_conf: EXT_XTL_CONF,
    ext_wakeup_conf: EXT_WAKEUP_CONF,
    slp_reject_conf: SLP_REJECT_CONF,
    cpu_period_conf: CPU_PERIOD_CONF,
    clk_conf: CLK_CONF,
    slow_clk_conf: SLOW_CLK_CONF,
    bias_conf: BIAS_CONF,
    rtc_cntl: RTC_CNTL,
    pwc: PWC,
    dig_pwc: DIG_PWC,
    dig_iso: DIG_ISO,
    wdtconfig0: WDTCONFIG0,
    wdtconfig1: WDTCONFIG1,
    wdtconfig2: WDTCONFIG2,
    wdtconfig3: WDTCONFIG3,
    wdtconfig4: WDTCONFIG4,
    wdtfeed: WDTFEED,
    wdtwprotect: WDTWPROTECT,
    swd_conf: SWD_CONF,
    swd_wprotect: SWD_WPROTECT,
    sw_cpu_stall: SW_CPU_STALL,
    store4: STORE4,
    store5: STORE5,
    store6: STORE6,
    store7: STORE7,
    low_power_st: LOW_POWER_ST,
    diag0: DIAG0,
    pad_hold: PAD_HOLD,
    dig_pad_hold: DIG_PAD_HOLD,
    brown_out: BROWN_OUT,
    time_low1: TIME_LOW1,
    time_high1: TIME_HIGH1,
    usb_conf: USB_CONF,
    slp_reject_cause: SLP_REJECT_CAUSE,
    option1: OPTION1,
    slp_wakeup_cause: SLP_WAKEUP_CAUSE,
    ulp_cp_timer_1: ULP_CP_TIMER_1,
    int_ena_rtc_w1ts: INT_ENA_RTC_W1TS,
    int_ena_rtc_w1tc: INT_ENA_RTC_W1TC,
    cntl_retention_ctrl: CNTL_RETENTION_CTRL,
    fib_sel: FIB_SEL,
    cntl_gpio_wakeup: CNTL_GPIO_WAKEUP,
    cntl_dbg_sel: CNTL_DBG_SEL,
    cntl_dbg_map: CNTL_DBG_MAP,
    cntl_sensor_ctrl: CNTL_SENSOR_CTRL,
    cntl_dbg_sar_sel: CNTL_DBG_SAR_SEL,
    _reserved68: [u8; 0xec],
    cntl_date: CNTL_DATE,
}
impl RegisterBlock {
    #[doc = "0x00 - register description"]
    #[inline(always)]
    pub const fn options0(&self) -> &OPTIONS0 {
        &self.options0
    }
    #[doc = "0x04 - register description"]
    #[inline(always)]
    pub const fn slp_timer0(&self) -> &SLP_TIMER0 {
        &self.slp_timer0
    }
    #[doc = "0x08 - register description"]
    #[inline(always)]
    pub const fn slp_timer1(&self) -> &SLP_TIMER1 {
        &self.slp_timer1
    }
    #[doc = "0x0c - register description"]
    #[inline(always)]
    pub const fn time_update(&self) -> &TIME_UPDATE {
        &self.time_update
    }
    #[doc = "0x10 - register description"]
    #[inline(always)]
    pub const fn time_low0(&self) -> &TIME_LOW0 {
        &self.time_low0
    }
    #[doc = "0x14 - register description"]
    #[inline(always)]
    pub const fn time_high0(&self) -> &TIME_HIGH0 {
        &self.time_high0
    }
    #[doc = "0x18 - register description"]
    #[inline(always)]
    pub const fn state0(&self) -> &STATE0 {
        &self.state0
    }
    #[doc = "0x1c - register description"]
    #[inline(always)]
    pub const fn timer1(&self) -> &TIMER1 {
        &self.timer1
    }
    #[doc = "0x20 - register description"]
    #[inline(always)]
    pub const fn timer2(&self) -> &TIMER2 {
        &self.timer2
    }
    #[doc = "0x24 - register description"]
    #[inline(always)]
    pub const fn timer4(&self) -> &TIMER4 {
        &self.timer4
    }
    #[doc = "0x28 - register description"]
    #[inline(always)]
    pub const fn timer5(&self) -> &TIMER5 {
        &self.timer5
    }
    #[doc = "0x2c - register description"]
    #[inline(always)]
    pub const fn ana_conf(&self) -> &ANA_CONF {
        &self.ana_conf
    }
    #[doc = "0x30 - register description"]
    #[inline(always)]
    pub const fn reset_state(&self) -> &RESET_STATE {
        &self.reset_state
    }
    #[doc = "0x34 - register description"]
    #[inline(always)]
    pub const fn wakeup_state(&self) -> &WAKEUP_STATE {
        &self.wakeup_state
    }
    #[doc = "0x38 - register description"]
    #[inline(always)]
    pub const fn int_ena(&self) -> &INT_ENA {
        &self.int_ena
    }
    #[doc = "0x3c - register description"]
    #[inline(always)]
    pub const fn int_raw(&self) -> &INT_RAW {
        &self.int_raw
    }
    #[doc = "0x40 - register description"]
    #[inline(always)]
    pub const fn int_st(&self) -> &INT_ST {
        &self.int_st
    }
    #[doc = "0x44 - register description"]
    #[inline(always)]
    pub const fn int_clr(&self) -> &INT_CLR {
        &self.int_clr
    }
    #[doc = "0x48 - register description"]
    #[inline(always)]
    pub const fn store0(&self) -> &STORE0 {
        &self.store0
    }
    #[doc = "0x4c - register description"]
    #[inline(always)]
    pub const fn store1(&self) -> &STORE1 {
        &self.store1
    }
    #[doc = "0x50 - register description"]
    #[inline(always)]
    pub const fn store2(&self) -> &STORE2 {
        &self.store2
    }
    #[doc = "0x54 - register description"]
    #[inline(always)]
    pub const fn store3(&self) -> &STORE3 {
        &self.store3
    }
    #[doc = "0x58 - register description"]
    #[inline(always)]
    pub const fn ext_xtl_conf(&self) -> &EXT_XTL_CONF {
        &self.ext_xtl_conf
    }
    #[doc = "0x5c - register description"]
    #[inline(always)]
    pub const fn ext_wakeup_conf(&self) -> &EXT_WAKEUP_CONF {
        &self.ext_wakeup_conf
    }
    #[doc = "0x60 - register description"]
    #[inline(always)]
    pub const fn slp_reject_conf(&self) -> &SLP_REJECT_CONF {
        &self.slp_reject_conf
    }
    #[doc = "0x64 - register description"]
    #[inline(always)]
    pub const fn cpu_period_conf(&self) -> &CPU_PERIOD_CONF {
        &self.cpu_period_conf
    }
    #[doc = "0x68 - register description"]
    #[inline(always)]
    pub const fn clk_conf(&self) -> &CLK_CONF {
        &self.clk_conf
    }
    #[doc = "0x6c - register description"]
    #[inline(always)]
    pub const fn slow_clk_conf(&self) -> &SLOW_CLK_CONF {
        &self.slow_clk_conf
    }
    #[doc = "0x70 - register description"]
    #[inline(always)]
    pub const fn bias_conf(&self) -> &BIAS_CONF {
        &self.bias_conf
    }
    #[doc = "0x74 - register description"]
    #[inline(always)]
    pub const fn rtc_cntl(&self) -> &RTC_CNTL {
        &self.rtc_cntl
    }
    #[doc = "0x78 - register description"]
    #[inline(always)]
    pub const fn pwc(&self) -> &PWC {
        &self.pwc
    }
    #[doc = "0x7c - register description"]
    #[inline(always)]
    pub const fn dig_pwc(&self) -> &DIG_PWC {
        &self.dig_pwc
    }
    #[doc = "0x80 - register description"]
    #[inline(always)]
    pub const fn dig_iso(&self) -> &DIG_ISO {
        &self.dig_iso
    }
    #[doc = "0x84 - register description"]
    #[inline(always)]
    pub const fn wdtconfig0(&self) -> &WDTCONFIG0 {
        &self.wdtconfig0
    }
    #[doc = "0x88 - register description"]
    #[inline(always)]
    pub const fn wdtconfig1(&self) -> &WDTCONFIG1 {
        &self.wdtconfig1
    }
    #[doc = "0x8c - register description"]
    #[inline(always)]
    pub const fn wdtconfig2(&self) -> &WDTCONFIG2 {
        &self.wdtconfig2
    }
    #[doc = "0x90 - register description"]
    #[inline(always)]
    pub const fn wdtconfig3(&self) -> &WDTCONFIG3 {
        &self.wdtconfig3
    }
    #[doc = "0x94 - register description"]
    #[inline(always)]
    pub const fn wdtconfig4(&self) -> &WDTCONFIG4 {
        &self.wdtconfig4
    }
    #[doc = "0x98 - register description"]
    #[inline(always)]
    pub const fn wdtfeed(&self) -> &WDTFEED {
        &self.wdtfeed
    }
    #[doc = "0x9c - register description"]
    #[inline(always)]
    pub const fn wdtwprotect(&self) -> &WDTWPROTECT {
        &self.wdtwprotect
    }
    #[doc = "0xa0 - register description"]
    #[inline(always)]
    pub const fn swd_conf(&self) -> &SWD_CONF {
        &self.swd_conf
    }
    #[doc = "0xa4 - register description"]
    #[inline(always)]
    pub const fn swd_wprotect(&self) -> &SWD_WPROTECT {
        &self.swd_wprotect
    }
    #[doc = "0xa8 - register description"]
    #[inline(always)]
    pub const fn sw_cpu_stall(&self) -> &SW_CPU_STALL {
        &self.sw_cpu_stall
    }
    #[doc = "0xac - register description"]
    #[inline(always)]
    pub const fn store4(&self) -> &STORE4 {
        &self.store4
    }
    #[doc = "0xb0 - register description"]
    #[inline(always)]
    pub const fn store5(&self) -> &STORE5 {
        &self.store5
    }
    #[doc = "0xb4 - register description"]
    #[inline(always)]
    pub const fn store6(&self) -> &STORE6 {
        &self.store6
    }
    #[doc = "0xb8 - register description"]
    #[inline(always)]
    pub const fn store7(&self) -> &STORE7 {
        &self.store7
    }
    #[doc = "0xbc - register description"]
    #[inline(always)]
    pub const fn low_power_st(&self) -> &LOW_POWER_ST {
        &self.low_power_st
    }
    #[doc = "0xc0 - register description"]
    #[inline(always)]
    pub const fn diag0(&self) -> &DIAG0 {
        &self.diag0
    }
    #[doc = "0xc4 - register description"]
    #[inline(always)]
    pub const fn pad_hold(&self) -> &PAD_HOLD {
        &self.pad_hold
    }
    #[doc = "0xc8 - register description"]
    #[inline(always)]
    pub const fn dig_pad_hold(&self) -> &DIG_PAD_HOLD {
        &self.dig_pad_hold
    }
    #[doc = "0xcc - register description"]
    #[inline(always)]
    pub const fn brown_out(&self) -> &BROWN_OUT {
        &self.brown_out
    }
    #[doc = "0xd0 - register description"]
    #[inline(always)]
    pub const fn time_low1(&self) -> &TIME_LOW1 {
        &self.time_low1
    }
    #[doc = "0xd4 - register description"]
    #[inline(always)]
    pub const fn time_high1(&self) -> &TIME_HIGH1 {
        &self.time_high1
    }
    #[doc = "0xd8 - register description"]
    #[inline(always)]
    pub const fn usb_conf(&self) -> &USB_CONF {
        &self.usb_conf
    }
    #[doc = "0xdc - register description"]
    #[inline(always)]
    pub const fn slp_reject_cause(&self) -> &SLP_REJECT_CAUSE {
        &self.slp_reject_cause
    }
    #[doc = "0xe0 - register description"]
    #[inline(always)]
    pub const fn option1(&self) -> &OPTION1 {
        &self.option1
    }
    #[doc = "0xe4 - register description"]
    #[inline(always)]
    pub const fn slp_wakeup_cause(&self) -> &SLP_WAKEUP_CAUSE {
        &self.slp_wakeup_cause
    }
    #[doc = "0xe8 - register description"]
    #[inline(always)]
    pub const fn ulp_cp_timer_1(&self) -> &ULP_CP_TIMER_1 {
        &self.ulp_cp_timer_1
    }
    #[doc = "0xec - register description"]
    #[inline(always)]
    pub const fn int_ena_rtc_w1ts(&self) -> &INT_ENA_RTC_W1TS {
        &self.int_ena_rtc_w1ts
    }
    #[doc = "0xf0 - register description"]
    #[inline(always)]
    pub const fn int_ena_rtc_w1tc(&self) -> &INT_ENA_RTC_W1TC {
        &self.int_ena_rtc_w1tc
    }
    #[doc = "0xf4 - register description"]
    #[inline(always)]
    pub const fn cntl_retention_ctrl(&self) -> &CNTL_RETENTION_CTRL {
        &self.cntl_retention_ctrl
    }
    #[doc = "0xf8 - register description"]
    #[inline(always)]
    pub const fn fib_sel(&self) -> &FIB_SEL {
        &self.fib_sel
    }
    #[doc = "0xfc - register description"]
    #[inline(always)]
    pub const fn cntl_gpio_wakeup(&self) -> &CNTL_GPIO_WAKEUP {
        &self.cntl_gpio_wakeup
    }
    #[doc = "0x100 - register description"]
    #[inline(always)]
    pub const fn cntl_dbg_sel(&self) -> &CNTL_DBG_SEL {
        &self.cntl_dbg_sel
    }
    #[doc = "0x104 - register description"]
    #[inline(always)]
    pub const fn cntl_dbg_map(&self) -> &CNTL_DBG_MAP {
        &self.cntl_dbg_map
    }
    #[doc = "0x108 - register description"]
    #[inline(always)]
    pub const fn cntl_sensor_ctrl(&self) -> &CNTL_SENSOR_CTRL {
        &self.cntl_sensor_ctrl
    }
    #[doc = "0x10c - register description"]
    #[inline(always)]
    pub const fn cntl_dbg_sar_sel(&self) -> &CNTL_DBG_SAR_SEL {
        &self.cntl_dbg_sar_sel
    }
    #[doc = "0x1fc - register description"]
    #[inline(always)]
    pub const fn cntl_date(&self) -> &CNTL_DATE {
        &self.cntl_date
    }
}
#[doc = "OPTIONS0 (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`options0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`options0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@options0`] module"]
pub type OPTIONS0 = crate::Reg<options0::OPTIONS0_SPEC>;
#[doc = "register description"]
pub mod options0;
#[doc = "SLP_TIMER0 (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`slp_timer0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slp_timer0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slp_timer0`] module"]
pub type SLP_TIMER0 = crate::Reg<slp_timer0::SLP_TIMER0_SPEC>;
#[doc = "register description"]
pub mod slp_timer0;
#[doc = "SLP_TIMER1 (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`slp_timer1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slp_timer1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slp_timer1`] module"]
pub type SLP_TIMER1 = crate::Reg<slp_timer1::SLP_TIMER1_SPEC>;
#[doc = "register description"]
pub mod slp_timer1;
#[doc = "TIME_UPDATE (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`time_update::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`time_update::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@time_update`] module"]
pub type TIME_UPDATE = crate::Reg<time_update::TIME_UPDATE_SPEC>;
#[doc = "register description"]
pub mod time_update;
#[doc = "TIME_LOW0 (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`time_low0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`time_low0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@time_low0`] module"]
pub type TIME_LOW0 = crate::Reg<time_low0::TIME_LOW0_SPEC>;
#[doc = "register description"]
pub mod time_low0;
#[doc = "TIME_HIGH0 (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`time_high0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`time_high0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@time_high0`] module"]
pub type TIME_HIGH0 = crate::Reg<time_high0::TIME_HIGH0_SPEC>;
#[doc = "register description"]
pub mod time_high0;
#[doc = "STATE0 (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`state0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`state0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@state0`] module"]
pub type STATE0 = crate::Reg<state0::STATE0_SPEC>;
#[doc = "register description"]
pub mod state0;
#[doc = "TIMER1 (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`timer1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer1`] module"]
pub type TIMER1 = crate::Reg<timer1::TIMER1_SPEC>;
#[doc = "register description"]
pub mod timer1;
#[doc = "TIMER2 (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`timer2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer2`] module"]
pub type TIMER2 = crate::Reg<timer2::TIMER2_SPEC>;
#[doc = "register description"]
pub mod timer2;
#[doc = "TIMER4 (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`timer4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer4`] module"]
pub type TIMER4 = crate::Reg<timer4::TIMER4_SPEC>;
#[doc = "register description"]
pub mod timer4;
#[doc = "TIMER5 (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`timer5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer5`] module"]
pub type TIMER5 = crate::Reg<timer5::TIMER5_SPEC>;
#[doc = "register description"]
pub mod timer5;
#[doc = "ANA_CONF (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`ana_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ana_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ana_conf`] module"]
pub type ANA_CONF = crate::Reg<ana_conf::ANA_CONF_SPEC>;
#[doc = "register description"]
pub mod ana_conf;
#[doc = "RESET_STATE (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`reset_state::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reset_state::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reset_state`] module"]
pub type RESET_STATE = crate::Reg<reset_state::RESET_STATE_SPEC>;
#[doc = "register description"]
pub mod reset_state;
#[doc = "WAKEUP_STATE (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`wakeup_state::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wakeup_state::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wakeup_state`] module"]
pub type WAKEUP_STATE = crate::Reg<wakeup_state::WAKEUP_STATE_SPEC>;
#[doc = "register description"]
pub mod wakeup_state;
#[doc = "INT_ENA (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`int_ena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_ena`] module"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = "register description"]
pub mod int_ena;
#[doc = "INT_RAW (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`int_raw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_raw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_raw`] module"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = "register description"]
pub mod int_raw;
#[doc = "INT_ST (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`int_st::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_st::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_st`] module"]
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
#[doc = "register description"]
pub mod int_st;
#[doc = "INT_CLR (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`int_clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_clr`] module"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = "register description"]
pub mod int_clr;
#[doc = "STORE0 (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`store0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`store0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@store0`] module"]
pub type STORE0 = crate::Reg<store0::STORE0_SPEC>;
#[doc = "register description"]
pub mod store0;
pub use store0 as store1;
pub use store0 as store2;
pub use store0 as store3;
pub use STORE0 as STORE1;
pub use STORE0 as STORE2;
pub use STORE0 as STORE3;
#[doc = "EXT_XTL_CONF (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`ext_xtl_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ext_xtl_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ext_xtl_conf`] module"]
pub type EXT_XTL_CONF = crate::Reg<ext_xtl_conf::EXT_XTL_CONF_SPEC>;
#[doc = "register description"]
pub mod ext_xtl_conf;
#[doc = "EXT_WAKEUP_CONF (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`ext_wakeup_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ext_wakeup_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ext_wakeup_conf`] module"]
pub type EXT_WAKEUP_CONF = crate::Reg<ext_wakeup_conf::EXT_WAKEUP_CONF_SPEC>;
#[doc = "register description"]
pub mod ext_wakeup_conf;
#[doc = "SLP_REJECT_CONF (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`slp_reject_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slp_reject_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slp_reject_conf`] module"]
pub type SLP_REJECT_CONF = crate::Reg<slp_reject_conf::SLP_REJECT_CONF_SPEC>;
#[doc = "register description"]
pub mod slp_reject_conf;
#[doc = "CPU_PERIOD_CONF (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_period_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_period_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_period_conf`] module"]
pub type CPU_PERIOD_CONF = crate::Reg<cpu_period_conf::CPU_PERIOD_CONF_SPEC>;
#[doc = "register description"]
pub mod cpu_period_conf;
#[doc = "CLK_CONF (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_conf`] module"]
pub type CLK_CONF = crate::Reg<clk_conf::CLK_CONF_SPEC>;
#[doc = "register description"]
pub mod clk_conf;
#[doc = "SLOW_CLK_CONF (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`slow_clk_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slow_clk_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slow_clk_conf`] module"]
pub type SLOW_CLK_CONF = crate::Reg<slow_clk_conf::SLOW_CLK_CONF_SPEC>;
#[doc = "register description"]
pub mod slow_clk_conf;
#[doc = "BIAS_CONF (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`bias_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bias_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bias_conf`] module"]
pub type BIAS_CONF = crate::Reg<bias_conf::BIAS_CONF_SPEC>;
#[doc = "register description"]
pub mod bias_conf;
#[doc = "RTC_CNTL (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_cntl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_cntl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_cntl`] module"]
pub type RTC_CNTL = crate::Reg<rtc_cntl::RTC_CNTL_SPEC>;
#[doc = "register description"]
pub mod rtc_cntl;
#[doc = "PWC (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`pwc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwc`] module"]
pub type PWC = crate::Reg<pwc::PWC_SPEC>;
#[doc = "register description"]
pub mod pwc;
#[doc = "DIG_PWC (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`dig_pwc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dig_pwc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dig_pwc`] module"]
pub type DIG_PWC = crate::Reg<dig_pwc::DIG_PWC_SPEC>;
#[doc = "register description"]
pub mod dig_pwc;
#[doc = "DIG_ISO (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`dig_iso::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dig_iso::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dig_iso`] module"]
pub type DIG_ISO = crate::Reg<dig_iso::DIG_ISO_SPEC>;
#[doc = "register description"]
pub mod dig_iso;
#[doc = "WDTCONFIG0 (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`wdtconfig0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdtconfig0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdtconfig0`] module"]
pub type WDTCONFIG0 = crate::Reg<wdtconfig0::WDTCONFIG0_SPEC>;
#[doc = "register description"]
pub mod wdtconfig0;
#[doc = "WDTCONFIG1 (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`wdtconfig1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdtconfig1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdtconfig1`] module"]
pub type WDTCONFIG1 = crate::Reg<wdtconfig1::WDTCONFIG1_SPEC>;
#[doc = "register description"]
pub mod wdtconfig1;
#[doc = "WDTCONFIG2 (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`wdtconfig2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdtconfig2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdtconfig2`] module"]
pub type WDTCONFIG2 = crate::Reg<wdtconfig2::WDTCONFIG2_SPEC>;
#[doc = "register description"]
pub mod wdtconfig2;
#[doc = "WDTCONFIG3 (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`wdtconfig3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdtconfig3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdtconfig3`] module"]
pub type WDTCONFIG3 = crate::Reg<wdtconfig3::WDTCONFIG3_SPEC>;
#[doc = "register description"]
pub mod wdtconfig3;
#[doc = "WDTCONFIG4 (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`wdtconfig4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdtconfig4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdtconfig4`] module"]
pub type WDTCONFIG4 = crate::Reg<wdtconfig4::WDTCONFIG4_SPEC>;
#[doc = "register description"]
pub mod wdtconfig4;
#[doc = "WDTFEED (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`wdtfeed::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdtfeed::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdtfeed`] module"]
pub type WDTFEED = crate::Reg<wdtfeed::WDTFEED_SPEC>;
#[doc = "register description"]
pub mod wdtfeed;
#[doc = "WDTWPROTECT (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`wdtwprotect::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdtwprotect::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdtwprotect`] module"]
pub type WDTWPROTECT = crate::Reg<wdtwprotect::WDTWPROTECT_SPEC>;
#[doc = "register description"]
pub mod wdtwprotect;
#[doc = "SWD_CONF (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`swd_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swd_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swd_conf`] module"]
pub type SWD_CONF = crate::Reg<swd_conf::SWD_CONF_SPEC>;
#[doc = "register description"]
pub mod swd_conf;
#[doc = "SWD_WPROTECT (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`swd_wprotect::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swd_wprotect::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swd_wprotect`] module"]
pub type SWD_WPROTECT = crate::Reg<swd_wprotect::SWD_WPROTECT_SPEC>;
#[doc = "register description"]
pub mod swd_wprotect;
#[doc = "SW_CPU_STALL (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`sw_cpu_stall::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sw_cpu_stall::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sw_cpu_stall`] module"]
pub type SW_CPU_STALL = crate::Reg<sw_cpu_stall::SW_CPU_STALL_SPEC>;
#[doc = "register description"]
pub mod sw_cpu_stall;
pub use store0 as store4;
pub use store0 as store5;
pub use store0 as store6;
pub use store0 as store7;
pub use STORE0 as STORE4;
pub use STORE0 as STORE5;
pub use STORE0 as STORE6;
pub use STORE0 as STORE7;
#[doc = "LOW_POWER_ST (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`low_power_st::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`low_power_st::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@low_power_st`] module"]
pub type LOW_POWER_ST = crate::Reg<low_power_st::LOW_POWER_ST_SPEC>;
#[doc = "register description"]
pub mod low_power_st;
#[doc = "DIAG0 (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`diag0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diag0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diag0`] module"]
pub type DIAG0 = crate::Reg<diag0::DIAG0_SPEC>;
#[doc = "register description"]
pub mod diag0;
#[doc = "PAD_HOLD (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`pad_hold::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad_hold::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad_hold`] module"]
pub type PAD_HOLD = crate::Reg<pad_hold::PAD_HOLD_SPEC>;
#[doc = "register description"]
pub mod pad_hold;
#[doc = "DIG_PAD_HOLD (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`dig_pad_hold::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dig_pad_hold::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dig_pad_hold`] module"]
pub type DIG_PAD_HOLD = crate::Reg<dig_pad_hold::DIG_PAD_HOLD_SPEC>;
#[doc = "register description"]
pub mod dig_pad_hold;
#[doc = "BROWN_OUT (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`brown_out::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`brown_out::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@brown_out`] module"]
pub type BROWN_OUT = crate::Reg<brown_out::BROWN_OUT_SPEC>;
#[doc = "register description"]
pub mod brown_out;
#[doc = "TIME_LOW1 (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`time_low1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`time_low1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@time_low1`] module"]
pub type TIME_LOW1 = crate::Reg<time_low1::TIME_LOW1_SPEC>;
#[doc = "register description"]
pub mod time_low1;
#[doc = "TIME_HIGH1 (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`time_high1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`time_high1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@time_high1`] module"]
pub type TIME_HIGH1 = crate::Reg<time_high1::TIME_HIGH1_SPEC>;
#[doc = "register description"]
pub mod time_high1;
#[doc = "USB_CONF (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`usb_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_conf`] module"]
pub type USB_CONF = crate::Reg<usb_conf::USB_CONF_SPEC>;
#[doc = "register description"]
pub mod usb_conf;
#[doc = "SLP_REJECT_CAUSE (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`slp_reject_cause::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slp_reject_cause::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slp_reject_cause`] module"]
pub type SLP_REJECT_CAUSE = crate::Reg<slp_reject_cause::SLP_REJECT_CAUSE_SPEC>;
#[doc = "register description"]
pub mod slp_reject_cause;
#[doc = "OPTION1 (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`option1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`option1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@option1`] module"]
pub type OPTION1 = crate::Reg<option1::OPTION1_SPEC>;
#[doc = "register description"]
pub mod option1;
#[doc = "SLP_WAKEUP_CAUSE (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`slp_wakeup_cause::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slp_wakeup_cause::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slp_wakeup_cause`] module"]
pub type SLP_WAKEUP_CAUSE = crate::Reg<slp_wakeup_cause::SLP_WAKEUP_CAUSE_SPEC>;
#[doc = "register description"]
pub mod slp_wakeup_cause;
#[doc = "ULP_CP_TIMER_1 (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`ulp_cp_timer_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ulp_cp_timer_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ulp_cp_timer_1`] module"]
pub type ULP_CP_TIMER_1 = crate::Reg<ulp_cp_timer_1::ULP_CP_TIMER_1_SPEC>;
#[doc = "register description"]
pub mod ulp_cp_timer_1;
#[doc = "INT_ENA_RTC_W1TS (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`int_ena_rtc_w1ts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ena_rtc_w1ts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_ena_rtc_w1ts`] module"]
pub type INT_ENA_RTC_W1TS = crate::Reg<int_ena_rtc_w1ts::INT_ENA_RTC_W1TS_SPEC>;
#[doc = "register description"]
pub mod int_ena_rtc_w1ts;
#[doc = "INT_ENA_RTC_W1TC (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`int_ena_rtc_w1tc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ena_rtc_w1tc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_ena_rtc_w1tc`] module"]
pub type INT_ENA_RTC_W1TC = crate::Reg<int_ena_rtc_w1tc::INT_ENA_RTC_W1TC_SPEC>;
#[doc = "register description"]
pub mod int_ena_rtc_w1tc;
#[doc = "CNTL_RETENTION_CTRL (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`cntl_retention_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cntl_retention_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cntl_retention_ctrl`] module"]
pub type CNTL_RETENTION_CTRL = crate::Reg<cntl_retention_ctrl::CNTL_RETENTION_CTRL_SPEC>;
#[doc = "register description"]
pub mod cntl_retention_ctrl;
#[doc = "FIB_SEL (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`fib_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fib_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fib_sel`] module"]
pub type FIB_SEL = crate::Reg<fib_sel::FIB_SEL_SPEC>;
#[doc = "register description"]
pub mod fib_sel;
#[doc = "CNTL_GPIO_WAKEUP (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`cntl_gpio_wakeup::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cntl_gpio_wakeup::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cntl_gpio_wakeup`] module"]
pub type CNTL_GPIO_WAKEUP = crate::Reg<cntl_gpio_wakeup::CNTL_GPIO_WAKEUP_SPEC>;
#[doc = "register description"]
pub mod cntl_gpio_wakeup;
#[doc = "CNTL_DBG_SEL (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`cntl_dbg_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cntl_dbg_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cntl_dbg_sel`] module"]
pub type CNTL_DBG_SEL = crate::Reg<cntl_dbg_sel::CNTL_DBG_SEL_SPEC>;
#[doc = "register description"]
pub mod cntl_dbg_sel;
#[doc = "CNTL_DBG_MAP (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`cntl_dbg_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cntl_dbg_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cntl_dbg_map`] module"]
pub type CNTL_DBG_MAP = crate::Reg<cntl_dbg_map::CNTL_DBG_MAP_SPEC>;
#[doc = "register description"]
pub mod cntl_dbg_map;
#[doc = "CNTL_SENSOR_CTRL (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`cntl_sensor_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cntl_sensor_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cntl_sensor_ctrl`] module"]
pub type CNTL_SENSOR_CTRL = crate::Reg<cntl_sensor_ctrl::CNTL_SENSOR_CTRL_SPEC>;
#[doc = "register description"]
pub mod cntl_sensor_ctrl;
#[doc = "CNTL_DBG_SAR_SEL (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`cntl_dbg_sar_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cntl_dbg_sar_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cntl_dbg_sar_sel`] module"]
pub type CNTL_DBG_SAR_SEL = crate::Reg<cntl_dbg_sar_sel::CNTL_DBG_SAR_SEL_SPEC>;
#[doc = "register description"]
pub mod cntl_dbg_sar_sel;
pub use crate::apb_ctrl::date as cntl_date;
pub use crate::apb_ctrl::DATE as CNTL_DATE;
