#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
///Register block
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
    timer3: TIMER3,
    timer4: TIMER4,
    timer5: TIMER5,
    timer6: TIMER6,
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
    sdio_act_conf: SDIO_ACT_CONF,
    clk_conf: CLK_CONF,
    slow_clk_conf: SLOW_CLK_CONF,
    sdio_conf: SDIO_CONF,
    bias_conf: BIAS_CONF,
    reg: REG,
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
    ext_wakeup1: EXT_WAKEUP1,
    ext_wakeup1_status: EXT_WAKEUP1_STATUS,
    brown_out: BROWN_OUT,
    time_low1: TIME_LOW1,
    time_high1: TIME_HIGH1,
    xtal32k_clk_factor: XTAL32K_CLK_FACTOR,
    xtal32k_conf: XTAL32K_CONF,
    ulp_cp_timer: ULP_CP_TIMER,
    ulp_cp_ctrl: ULP_CP_CTRL,
    cocpu_ctrl: COCPU_CTRL,
    touch_ctrl1: TOUCH_CTRL1,
    touch_ctrl2: TOUCH_CTRL2,
    touch_scan_ctrl: TOUCH_SCAN_CTRL,
    touch_slp_thres: TOUCH_SLP_THRES,
    touch_approach: TOUCH_APPROACH,
    touch_filter_ctrl: TOUCH_FILTER_CTRL,
    usb_conf: USB_CONF,
    touch_timeout_ctrl: TOUCH_TIMEOUT_CTRL,
    slp_reject_cause: SLP_REJECT_CAUSE,
    options1: OPTIONS1,
    slp_wakeup_cause: SLP_WAKEUP_CAUSE,
    ulp_cp_timer_1: ULP_CP_TIMER_1,
    _reserved77: [u8; 0x04],
    date: DATE,
}
impl RegisterBlock {
    ///0x00 - Sets the power options of crystal and PLL clocks, and initiates reset by software
    #[inline(always)]
    pub const fn options0(&self) -> &OPTIONS0 {
        &self.options0
    }
    ///0x04 - RTC timer threshold register 0
    #[inline(always)]
    pub const fn slp_timer0(&self) -> &SLP_TIMER0 {
        &self.slp_timer0
    }
    ///0x08 - RTC timer threshold register 1
    #[inline(always)]
    pub const fn slp_timer1(&self) -> &SLP_TIMER1 {
        &self.slp_timer1
    }
    ///0x0c - RTC timer update control register
    #[inline(always)]
    pub const fn time_update(&self) -> &TIME_UPDATE {
        &self.time_update
    }
    ///0x10 - Stores the lower 32 bits of RTC timer 0.
    #[inline(always)]
    pub const fn time_low0(&self) -> &TIME_LOW0 {
        &self.time_low0
    }
    ///0x14 - Stores the higher 16 bits of RTC timer 0
    #[inline(always)]
    pub const fn time_high0(&self) -> &TIME_HIGH0 {
        &self.time_high0
    }
    ///0x18 - Configures the sleep / reject / wakeup state
    #[inline(always)]
    pub const fn state0(&self) -> &STATE0 {
        &self.state0
    }
    ///0x1c - Configures CPU stall options
    #[inline(always)]
    pub const fn timer1(&self) -> &TIMER1 {
        &self.timer1
    }
    ///0x20 - Configures RTC slow clock and touch controller
    #[inline(always)]
    pub const fn timer2(&self) -> &TIMER2 {
        &self.timer2
    }
    ///0x24 - configure some wait time for power on
    #[inline(always)]
    pub const fn timer3(&self) -> &TIMER3 {
        &self.timer3
    }
    ///0x28 - configure some wait time for power on
    #[inline(always)]
    pub const fn timer4(&self) -> &TIMER4 {
        &self.timer4
    }
    ///0x2c - Configures the minimal sleep cycles
    #[inline(always)]
    pub const fn timer5(&self) -> &TIMER5 {
        &self.timer5
    }
    ///0x30 - Configure minimal sleep cycles register
    #[inline(always)]
    pub const fn timer6(&self) -> &TIMER6 {
        &self.timer6
    }
    ///0x34 - Configures the power options for I2C and PLLA
    #[inline(always)]
    pub const fn ana_conf(&self) -> &ANA_CONF {
        &self.ana_conf
    }
    /**0x38 - Indicates the CPU reset source. For more information about the reset cause, please refer to Table \ref{table:resetreasons} in Chapter \ref{module:ResetandClock} \textit{
    ameref{module:ResetandClock}}.*/
    #[inline(always)]
    pub const fn reset_state(&self) -> &RESET_STATE {
        &self.reset_state
    }
    ///0x3c - Wakeup bitmap enabling register
    #[inline(always)]
    pub const fn wakeup_state(&self) -> &WAKEUP_STATE {
        &self.wakeup_state
    }
    ///0x40 - RTC interrupt enabling register
    #[inline(always)]
    pub const fn int_ena(&self) -> &INT_ENA {
        &self.int_ena
    }
    ///0x44 - RTC interrupt raw register
    #[inline(always)]
    pub const fn int_raw(&self) -> &INT_RAW {
        &self.int_raw
    }
    ///0x48 - RTC interrupt state register
    #[inline(always)]
    pub const fn int_st(&self) -> &INT_ST {
        &self.int_st
    }
    ///0x4c - RTC interrupt clear register
    #[inline(always)]
    pub const fn int_clr(&self) -> &INT_CLR {
        &self.int_clr
    }
    ///0x50 - Reservation register 0
    #[inline(always)]
    pub const fn store0(&self) -> &STORE0 {
        &self.store0
    }
    ///0x54 - Reservation register 1
    #[inline(always)]
    pub const fn store1(&self) -> &STORE1 {
        &self.store1
    }
    ///0x58 - Reservation register 2
    #[inline(always)]
    pub const fn store2(&self) -> &STORE2 {
        &self.store2
    }
    ///0x5c - Reservation register 3
    #[inline(always)]
    pub const fn store3(&self) -> &STORE3 {
        &self.store3
    }
    ///0x60 - 32 kHz crystal oscillator configuration register
    #[inline(always)]
    pub const fn ext_xtl_conf(&self) -> &EXT_XTL_CONF {
        &self.ext_xtl_conf
    }
    ///0x64 - GPIO wakeup configuration register
    #[inline(always)]
    pub const fn ext_wakeup_conf(&self) -> &EXT_WAKEUP_CONF {
        &self.ext_wakeup_conf
    }
    ///0x68 - Configures sleep / reject options
    #[inline(always)]
    pub const fn slp_reject_conf(&self) -> &SLP_REJECT_CONF {
        &self.slp_reject_conf
    }
    ///0x6c - CPU sel option
    #[inline(always)]
    pub const fn cpu_period_conf(&self) -> &CPU_PERIOD_CONF {
        &self.cpu_period_conf
    }
    ///0x70 - configure sdio active register
    #[inline(always)]
    pub const fn sdio_act_conf(&self) -> &SDIO_ACT_CONF {
        &self.sdio_act_conf
    }
    ///0x74 - RTC clock configuration register
    #[inline(always)]
    pub const fn clk_conf(&self) -> &CLK_CONF {
        &self.clk_conf
    }
    ///0x78 - RTC slow clock configuration register
    #[inline(always)]
    pub const fn slow_clk_conf(&self) -> &SLOW_CLK_CONF {
        &self.slow_clk_conf
    }
    ///0x7c - configure vddsdio register
    #[inline(always)]
    pub const fn sdio_conf(&self) -> &SDIO_CONF {
        &self.sdio_conf
    }
    ///0x80 - configure power register
    #[inline(always)]
    pub const fn bias_conf(&self) -> &BIAS_CONF {
        &self.bias_conf
    }
    ///0x84 - RTC/DIG regulator configuration register
    #[inline(always)]
    pub const fn reg(&self) -> &REG {
        &self.reg
    }
    ///0x88 - RTC power configuraiton register
    #[inline(always)]
    pub const fn pwc(&self) -> &PWC {
        &self.pwc
    }
    ///0x8c - Digital system power configuraiton register
    #[inline(always)]
    pub const fn dig_pwc(&self) -> &DIG_PWC {
        &self.dig_pwc
    }
    ///0x90 - Digital system ISO configuration register
    #[inline(always)]
    pub const fn dig_iso(&self) -> &DIG_ISO {
        &self.dig_iso
    }
    ///0x94 - RTC watchdog configuration register
    #[inline(always)]
    pub const fn wdtconfig0(&self) -> &WDTCONFIG0 {
        &self.wdtconfig0
    }
    ///0x98 - Configures the hold time of RTC watchdog at level 1
    #[inline(always)]
    pub const fn wdtconfig1(&self) -> &WDTCONFIG1 {
        &self.wdtconfig1
    }
    ///0x9c - Configures the hold time of RTC watchdog at level 2
    #[inline(always)]
    pub const fn wdtconfig2(&self) -> &WDTCONFIG2 {
        &self.wdtconfig2
    }
    ///0xa0 - Configures the hold time of RTC watchdog at level 3
    #[inline(always)]
    pub const fn wdtconfig3(&self) -> &WDTCONFIG3 {
        &self.wdtconfig3
    }
    ///0xa4 - Configures the hold time of RTC watchdog at level 4
    #[inline(always)]
    pub const fn wdtconfig4(&self) -> &WDTCONFIG4 {
        &self.wdtconfig4
    }
    ///0xa8 - RTC watchdog SW feed configuration register
    #[inline(always)]
    pub const fn wdtfeed(&self) -> &WDTFEED {
        &self.wdtfeed
    }
    ///0xac - RTC watchdog write protection configuration register
    #[inline(always)]
    pub const fn wdtwprotect(&self) -> &WDTWPROTECT {
        &self.wdtwprotect
    }
    ///0xb0 - Super watchdog configuration register
    #[inline(always)]
    pub const fn swd_conf(&self) -> &SWD_CONF {
        &self.swd_conf
    }
    ///0xb4 - Super watchdog write protection configuration register
    #[inline(always)]
    pub const fn swd_wprotect(&self) -> &SWD_WPROTECT {
        &self.swd_wprotect
    }
    ///0xb8 - CPU stall configuration register
    #[inline(always)]
    pub const fn sw_cpu_stall(&self) -> &SW_CPU_STALL {
        &self.sw_cpu_stall
    }
    ///0xbc - Reservation register 4
    #[inline(always)]
    pub const fn store4(&self) -> &STORE4 {
        &self.store4
    }
    ///0xc0 - Reservation register 5
    #[inline(always)]
    pub const fn store5(&self) -> &STORE5 {
        &self.store5
    }
    ///0xc4 - Reservation register 6
    #[inline(always)]
    pub const fn store6(&self) -> &STORE6 {
        &self.store6
    }
    ///0xc8 - Reservation register 7
    #[inline(always)]
    pub const fn store7(&self) -> &STORE7 {
        &self.store7
    }
    ///0xcc - RTC main state machine status register
    #[inline(always)]
    pub const fn low_power_st(&self) -> &LOW_POWER_ST {
        &self.low_power_st
    }
    ///0xd0 - debug register
    #[inline(always)]
    pub const fn diag0(&self) -> &DIAG0 {
        &self.diag0
    }
    ///0xd4 - Configures the hold options for RTC GPIOs
    #[inline(always)]
    pub const fn pad_hold(&self) -> &PAD_HOLD {
        &self.pad_hold
    }
    ///0xd8 - Configures the hold option for digital GPIOs
    #[inline(always)]
    pub const fn dig_pad_hold(&self) -> &DIG_PAD_HOLD {
        &self.dig_pad_hold
    }
    ///0xdc - EXT1 wakeup configuration register
    #[inline(always)]
    pub const fn ext_wakeup1(&self) -> &EXT_WAKEUP1 {
        &self.ext_wakeup1
    }
    ///0xe0 - EXT1 wakeup source register
    #[inline(always)]
    pub const fn ext_wakeup1_status(&self) -> &EXT_WAKEUP1_STATUS {
        &self.ext_wakeup1_status
    }
    ///0xe4 - Brownout configuration register
    #[inline(always)]
    pub const fn brown_out(&self) -> &BROWN_OUT {
        &self.brown_out
    }
    ///0xe8 - Stores the lower 32 bits of RTC timer 1
    #[inline(always)]
    pub const fn time_low1(&self) -> &TIME_LOW1 {
        &self.time_low1
    }
    ///0xec - Stores the higher 16 bits of RTC timer 1
    #[inline(always)]
    pub const fn time_high1(&self) -> &TIME_HIGH1 {
        &self.time_high1
    }
    ///0xf0 - Configures the divider factor for the backup clock of 32 kHz crystal oscillator
    #[inline(always)]
    pub const fn xtal32k_clk_factor(&self) -> &XTAL32K_CLK_FACTOR {
        &self.xtal32k_clk_factor
    }
    ///0xf4 - 32 kHz crystal oscillator configuration register
    #[inline(always)]
    pub const fn xtal32k_conf(&self) -> &XTAL32K_CONF {
        &self.xtal32k_conf
    }
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
    ///0x104 - Touch control register
    #[inline(always)]
    pub const fn touch_ctrl1(&self) -> &TOUCH_CTRL1 {
        &self.touch_ctrl1
    }
    ///0x108 - Touch control register
    #[inline(always)]
    pub const fn touch_ctrl2(&self) -> &TOUCH_CTRL2 {
        &self.touch_ctrl2
    }
    ///0x10c - Configure touch scan settings
    #[inline(always)]
    pub const fn touch_scan_ctrl(&self) -> &TOUCH_SCAN_CTRL {
        &self.touch_scan_ctrl
    }
    ///0x110 - Configure the settings of touch sleep pad
    #[inline(always)]
    pub const fn touch_slp_thres(&self) -> &TOUCH_SLP_THRES {
        &self.touch_slp_thres
    }
    ///0x114 - Configure touch approach settings
    #[inline(always)]
    pub const fn touch_approach(&self) -> &TOUCH_APPROACH {
        &self.touch_approach
    }
    ///0x118 - Configure touch filter settings
    #[inline(always)]
    pub const fn touch_filter_ctrl(&self) -> &TOUCH_FILTER_CTRL {
        &self.touch_filter_ctrl
    }
    ///0x11c - configure usb control register
    #[inline(always)]
    pub const fn usb_conf(&self) -> &USB_CONF {
        &self.usb_conf
    }
    ///0x120 - Configure touch timeout settings
    #[inline(always)]
    pub const fn touch_timeout_ctrl(&self) -> &TOUCH_TIMEOUT_CTRL {
        &self.touch_timeout_ctrl
    }
    ///0x124 - Stores the reject-to-sleep cause.
    #[inline(always)]
    pub const fn slp_reject_cause(&self) -> &SLP_REJECT_CAUSE {
        &self.slp_reject_cause
    }
    ///0x128 - RTC option register
    #[inline(always)]
    pub const fn options1(&self) -> &OPTIONS1 {
        &self.options1
    }
    ///0x12c - Stores the sleep-to-wakeup cause.
    #[inline(always)]
    pub const fn slp_wakeup_cause(&self) -> &SLP_WAKEUP_CAUSE {
        &self.slp_wakeup_cause
    }
    ///0x130 - Configure sleep cycle of the timer
    #[inline(always)]
    pub const fn ulp_cp_timer_1(&self) -> &ULP_CP_TIMER_1 {
        &self.ulp_cp_timer_1
    }
    ///0x138 -
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
/**OPTIONS0 (rw) register accessor: Sets the power options of crystal and PLL clocks, and initiates reset by software

You can [`read`](crate::generic::Reg::read) this register and get [`options0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`options0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@options0`] module*/
pub type OPTIONS0 = crate::Reg<options0::OPTIONS0_SPEC>;
///Sets the power options of crystal and PLL clocks, and initiates reset by software
pub mod options0;
/**SLP_TIMER0 (rw) register accessor: RTC timer threshold register 0

You can [`read`](crate::generic::Reg::read) this register and get [`slp_timer0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slp_timer0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@slp_timer0`] module*/
pub type SLP_TIMER0 = crate::Reg<slp_timer0::SLP_TIMER0_SPEC>;
///RTC timer threshold register 0
pub mod slp_timer0;
/**SLP_TIMER1 (rw) register accessor: RTC timer threshold register 1

You can [`read`](crate::generic::Reg::read) this register and get [`slp_timer1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slp_timer1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@slp_timer1`] module*/
pub type SLP_TIMER1 = crate::Reg<slp_timer1::SLP_TIMER1_SPEC>;
///RTC timer threshold register 1
pub mod slp_timer1;
/**TIME_UPDATE (rw) register accessor: RTC timer update control register

You can [`read`](crate::generic::Reg::read) this register and get [`time_update::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`time_update::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@time_update`] module*/
pub type TIME_UPDATE = crate::Reg<time_update::TIME_UPDATE_SPEC>;
///RTC timer update control register
pub mod time_update;
/**TIME_LOW0 (r) register accessor: Stores the lower 32 bits of RTC timer 0.

You can [`read`](crate::generic::Reg::read) this register and get [`time_low0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@time_low0`] module*/
pub type TIME_LOW0 = crate::Reg<time_low0::TIME_LOW0_SPEC>;
///Stores the lower 32 bits of RTC timer 0.
pub mod time_low0;
/**TIME_HIGH0 (r) register accessor: Stores the higher 16 bits of RTC timer 0

You can [`read`](crate::generic::Reg::read) this register and get [`time_high0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@time_high0`] module*/
pub type TIME_HIGH0 = crate::Reg<time_high0::TIME_HIGH0_SPEC>;
///Stores the higher 16 bits of RTC timer 0
pub mod time_high0;
/**STATE0 (rw) register accessor: Configures the sleep / reject / wakeup state

You can [`read`](crate::generic::Reg::read) this register and get [`state0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`state0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@state0`] module*/
pub type STATE0 = crate::Reg<state0::STATE0_SPEC>;
///Configures the sleep / reject / wakeup state
pub mod state0;
/**TIMER1 (rw) register accessor: Configures CPU stall options

You can [`read`](crate::generic::Reg::read) this register and get [`timer1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@timer1`] module*/
pub type TIMER1 = crate::Reg<timer1::TIMER1_SPEC>;
///Configures CPU stall options
pub mod timer1;
/**TIMER2 (rw) register accessor: Configures RTC slow clock and touch controller

You can [`read`](crate::generic::Reg::read) this register and get [`timer2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@timer2`] module*/
pub type TIMER2 = crate::Reg<timer2::TIMER2_SPEC>;
///Configures RTC slow clock and touch controller
pub mod timer2;
/**TIMER3 (rw) register accessor: configure some wait time for power on

You can [`read`](crate::generic::Reg::read) this register and get [`timer3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@timer3`] module*/
pub type TIMER3 = crate::Reg<timer3::TIMER3_SPEC>;
///configure some wait time for power on
pub mod timer3;
/**TIMER4 (rw) register accessor: configure some wait time for power on

You can [`read`](crate::generic::Reg::read) this register and get [`timer4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@timer4`] module*/
pub type TIMER4 = crate::Reg<timer4::TIMER4_SPEC>;
///configure some wait time for power on
pub mod timer4;
/**TIMER5 (rw) register accessor: Configures the minimal sleep cycles

You can [`read`](crate::generic::Reg::read) this register and get [`timer5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@timer5`] module*/
pub type TIMER5 = crate::Reg<timer5::TIMER5_SPEC>;
///Configures the minimal sleep cycles
pub mod timer5;
/**TIMER6 (rw) register accessor: Configure minimal sleep cycles register

You can [`read`](crate::generic::Reg::read) this register and get [`timer6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@timer6`] module*/
pub type TIMER6 = crate::Reg<timer6::TIMER6_SPEC>;
///Configure minimal sleep cycles register
pub mod timer6;
/**ANA_CONF (rw) register accessor: Configures the power options for I2C and PLLA

You can [`read`](crate::generic::Reg::read) this register and get [`ana_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ana_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ana_conf`] module*/
pub type ANA_CONF = crate::Reg<ana_conf::ANA_CONF_SPEC>;
///Configures the power options for I2C and PLLA
pub mod ana_conf;
/**RESET_STATE (rw) register accessor: Indicates the CPU reset source. For more information about the reset cause, please refer to Table \ref{table:resetreasons} in Chapter \ref{module:ResetandClock} \textit{
ameref{module:ResetandClock}}.

You can [`read`](crate::generic::Reg::read) this register and get [`reset_state::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reset_state::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@reset_state`] module*/
pub type RESET_STATE = crate::Reg<reset_state::RESET_STATE_SPEC>;
/**Indicates the CPU reset source. For more information about the reset cause, please refer to Table \ref{table:resetreasons} in Chapter \ref{module:ResetandClock} \textit{
ameref{module:ResetandClock}}.*/
pub mod reset_state;
/**WAKEUP_STATE (rw) register accessor: Wakeup bitmap enabling register

You can [`read`](crate::generic::Reg::read) this register and get [`wakeup_state::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wakeup_state::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@wakeup_state`] module*/
pub type WAKEUP_STATE = crate::Reg<wakeup_state::WAKEUP_STATE_SPEC>;
///Wakeup bitmap enabling register
pub mod wakeup_state;
/**INT_ENA (rw) register accessor: RTC interrupt enabling register

You can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_ena`] module*/
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
///RTC interrupt enabling register
pub mod int_ena;
/**INT_RAW (r) register accessor: RTC interrupt raw register

You can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_raw`] module*/
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
///RTC interrupt raw register
pub mod int_raw;
/**INT_ST (r) register accessor: RTC interrupt state register

You can [`read`](crate::generic::Reg::read) this register and get [`int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_st`] module*/
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
///RTC interrupt state register
pub mod int_st;
/**INT_CLR (w) register accessor: RTC interrupt clear register

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_clr`] module*/
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
///RTC interrupt clear register
pub mod int_clr;
/**STORE0 (rw) register accessor: Reservation register 0

You can [`read`](crate::generic::Reg::read) this register and get [`store0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`store0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@store0`] module*/
pub type STORE0 = crate::Reg<store0::STORE0_SPEC>;
///Reservation register 0
pub mod store0;
/**STORE1 (rw) register accessor: Reservation register 1

You can [`read`](crate::generic::Reg::read) this register and get [`store1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`store1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@store1`] module*/
pub type STORE1 = crate::Reg<store1::STORE1_SPEC>;
///Reservation register 1
pub mod store1;
/**STORE2 (rw) register accessor: Reservation register 2

You can [`read`](crate::generic::Reg::read) this register and get [`store2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`store2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@store2`] module*/
pub type STORE2 = crate::Reg<store2::STORE2_SPEC>;
///Reservation register 2
pub mod store2;
/**STORE3 (rw) register accessor: Reservation register 3

You can [`read`](crate::generic::Reg::read) this register and get [`store3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`store3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@store3`] module*/
pub type STORE3 = crate::Reg<store3::STORE3_SPEC>;
///Reservation register 3
pub mod store3;
/**EXT_XTL_CONF (rw) register accessor: 32 kHz crystal oscillator configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`ext_xtl_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ext_xtl_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ext_xtl_conf`] module*/
pub type EXT_XTL_CONF = crate::Reg<ext_xtl_conf::EXT_XTL_CONF_SPEC>;
///32 kHz crystal oscillator configuration register
pub mod ext_xtl_conf;
/**EXT_WAKEUP_CONF (rw) register accessor: GPIO wakeup configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`ext_wakeup_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ext_wakeup_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ext_wakeup_conf`] module*/
pub type EXT_WAKEUP_CONF = crate::Reg<ext_wakeup_conf::EXT_WAKEUP_CONF_SPEC>;
///GPIO wakeup configuration register
pub mod ext_wakeup_conf;
/**SLP_REJECT_CONF (rw) register accessor: Configures sleep / reject options

You can [`read`](crate::generic::Reg::read) this register and get [`slp_reject_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slp_reject_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@slp_reject_conf`] module*/
pub type SLP_REJECT_CONF = crate::Reg<slp_reject_conf::SLP_REJECT_CONF_SPEC>;
///Configures sleep / reject options
pub mod slp_reject_conf;
/**CPU_PERIOD_CONF (rw) register accessor: CPU sel option

You can [`read`](crate::generic::Reg::read) this register and get [`cpu_period_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_period_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cpu_period_conf`] module*/
pub type CPU_PERIOD_CONF = crate::Reg<cpu_period_conf::CPU_PERIOD_CONF_SPEC>;
///CPU sel option
pub mod cpu_period_conf;
/**SDIO_ACT_CONF (rw) register accessor: configure sdio active register

You can [`read`](crate::generic::Reg::read) this register and get [`sdio_act_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdio_act_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sdio_act_conf`] module*/
pub type SDIO_ACT_CONF = crate::Reg<sdio_act_conf::SDIO_ACT_CONF_SPEC>;
///configure sdio active register
pub mod sdio_act_conf;
/**CLK_CONF (rw) register accessor: RTC clock configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`clk_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@clk_conf`] module*/
pub type CLK_CONF = crate::Reg<clk_conf::CLK_CONF_SPEC>;
///RTC clock configuration register
pub mod clk_conf;
/**SLOW_CLK_CONF (rw) register accessor: RTC slow clock configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`slow_clk_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slow_clk_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@slow_clk_conf`] module*/
pub type SLOW_CLK_CONF = crate::Reg<slow_clk_conf::SLOW_CLK_CONF_SPEC>;
///RTC slow clock configuration register
pub mod slow_clk_conf;
/**SDIO_CONF (rw) register accessor: configure vddsdio register

You can [`read`](crate::generic::Reg::read) this register and get [`sdio_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdio_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sdio_conf`] module*/
pub type SDIO_CONF = crate::Reg<sdio_conf::SDIO_CONF_SPEC>;
///configure vddsdio register
pub mod sdio_conf;
/**BIAS_CONF (rw) register accessor: configure power register

You can [`read`](crate::generic::Reg::read) this register and get [`bias_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bias_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@bias_conf`] module*/
pub type BIAS_CONF = crate::Reg<bias_conf::BIAS_CONF_SPEC>;
///configure power register
pub mod bias_conf;
/**REG (rw) register accessor: RTC/DIG regulator configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@reg`] module*/
pub type REG = crate::Reg<reg::REG_SPEC>;
///RTC/DIG regulator configuration register
pub mod reg;
/**PWC (rw) register accessor: RTC power configuraiton register

You can [`read`](crate::generic::Reg::read) this register and get [`pwc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pwc`] module*/
pub type PWC = crate::Reg<pwc::PWC_SPEC>;
///RTC power configuraiton register
pub mod pwc;
/**DIG_PWC (rw) register accessor: Digital system power configuraiton register

You can [`read`](crate::generic::Reg::read) this register and get [`dig_pwc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dig_pwc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dig_pwc`] module*/
pub type DIG_PWC = crate::Reg<dig_pwc::DIG_PWC_SPEC>;
///Digital system power configuraiton register
pub mod dig_pwc;
/**DIG_ISO (rw) register accessor: Digital system ISO configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`dig_iso::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dig_iso::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dig_iso`] module*/
pub type DIG_ISO = crate::Reg<dig_iso::DIG_ISO_SPEC>;
///Digital system ISO configuration register
pub mod dig_iso;
/**WDTCONFIG0 (rw) register accessor: RTC watchdog configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`wdtconfig0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtconfig0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@wdtconfig0`] module*/
pub type WDTCONFIG0 = crate::Reg<wdtconfig0::WDTCONFIG0_SPEC>;
///RTC watchdog configuration register
pub mod wdtconfig0;
/**WDTCONFIG1 (rw) register accessor: Configures the hold time of RTC watchdog at level 1

You can [`read`](crate::generic::Reg::read) this register and get [`wdtconfig1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtconfig1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@wdtconfig1`] module*/
pub type WDTCONFIG1 = crate::Reg<wdtconfig1::WDTCONFIG1_SPEC>;
///Configures the hold time of RTC watchdog at level 1
pub mod wdtconfig1;
/**WDTCONFIG2 (rw) register accessor: Configures the hold time of RTC watchdog at level 2

You can [`read`](crate::generic::Reg::read) this register and get [`wdtconfig2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtconfig2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@wdtconfig2`] module*/
pub type WDTCONFIG2 = crate::Reg<wdtconfig2::WDTCONFIG2_SPEC>;
///Configures the hold time of RTC watchdog at level 2
pub mod wdtconfig2;
/**WDTCONFIG3 (rw) register accessor: Configures the hold time of RTC watchdog at level 3

You can [`read`](crate::generic::Reg::read) this register and get [`wdtconfig3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtconfig3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@wdtconfig3`] module*/
pub type WDTCONFIG3 = crate::Reg<wdtconfig3::WDTCONFIG3_SPEC>;
///Configures the hold time of RTC watchdog at level 3
pub mod wdtconfig3;
/**WDTCONFIG4 (rw) register accessor: Configures the hold time of RTC watchdog at level 4

You can [`read`](crate::generic::Reg::read) this register and get [`wdtconfig4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtconfig4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@wdtconfig4`] module*/
pub type WDTCONFIG4 = crate::Reg<wdtconfig4::WDTCONFIG4_SPEC>;
///Configures the hold time of RTC watchdog at level 4
pub mod wdtconfig4;
/**WDTFEED (w) register accessor: RTC watchdog SW feed configuration register

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtfeed::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@wdtfeed`] module*/
pub type WDTFEED = crate::Reg<wdtfeed::WDTFEED_SPEC>;
///RTC watchdog SW feed configuration register
pub mod wdtfeed;
/**WDTWPROTECT (rw) register accessor: RTC watchdog write protection configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`wdtwprotect::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtwprotect::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@wdtwprotect`] module*/
pub type WDTWPROTECT = crate::Reg<wdtwprotect::WDTWPROTECT_SPEC>;
///RTC watchdog write protection configuration register
pub mod wdtwprotect;
/**SWD_CONF (rw) register accessor: Super watchdog configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`swd_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swd_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@swd_conf`] module*/
pub type SWD_CONF = crate::Reg<swd_conf::SWD_CONF_SPEC>;
///Super watchdog configuration register
pub mod swd_conf;
/**SWD_WPROTECT (rw) register accessor: Super watchdog write protection configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`swd_wprotect::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swd_wprotect::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@swd_wprotect`] module*/
pub type SWD_WPROTECT = crate::Reg<swd_wprotect::SWD_WPROTECT_SPEC>;
///Super watchdog write protection configuration register
pub mod swd_wprotect;
/**SW_CPU_STALL (rw) register accessor: CPU stall configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`sw_cpu_stall::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sw_cpu_stall::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sw_cpu_stall`] module*/
pub type SW_CPU_STALL = crate::Reg<sw_cpu_stall::SW_CPU_STALL_SPEC>;
///CPU stall configuration register
pub mod sw_cpu_stall;
/**STORE4 (rw) register accessor: Reservation register 4

You can [`read`](crate::generic::Reg::read) this register and get [`store4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`store4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@store4`] module*/
pub type STORE4 = crate::Reg<store4::STORE4_SPEC>;
///Reservation register 4
pub mod store4;
/**STORE5 (rw) register accessor: Reservation register 5

You can [`read`](crate::generic::Reg::read) this register and get [`store5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`store5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@store5`] module*/
pub type STORE5 = crate::Reg<store5::STORE5_SPEC>;
///Reservation register 5
pub mod store5;
/**STORE6 (rw) register accessor: Reservation register 6

You can [`read`](crate::generic::Reg::read) this register and get [`store6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`store6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@store6`] module*/
pub type STORE6 = crate::Reg<store6::STORE6_SPEC>;
///Reservation register 6
pub mod store6;
/**STORE7 (rw) register accessor: Reservation register 7

You can [`read`](crate::generic::Reg::read) this register and get [`store7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`store7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@store7`] module*/
pub type STORE7 = crate::Reg<store7::STORE7_SPEC>;
///Reservation register 7
pub mod store7;
/**LOW_POWER_ST (r) register accessor: RTC main state machine status register

You can [`read`](crate::generic::Reg::read) this register and get [`low_power_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@low_power_st`] module*/
pub type LOW_POWER_ST = crate::Reg<low_power_st::LOW_POWER_ST_SPEC>;
///RTC main state machine status register
pub mod low_power_st;
/**DIAG0 (r) register accessor: debug register

You can [`read`](crate::generic::Reg::read) this register and get [`diag0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@diag0`] module*/
pub type DIAG0 = crate::Reg<diag0::DIAG0_SPEC>;
///debug register
pub mod diag0;
/**PAD_HOLD (rw) register accessor: Configures the hold options for RTC GPIOs

You can [`read`](crate::generic::Reg::read) this register and get [`pad_hold::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pad_hold::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pad_hold`] module*/
pub type PAD_HOLD = crate::Reg<pad_hold::PAD_HOLD_SPEC>;
///Configures the hold options for RTC GPIOs
pub mod pad_hold;
/**DIG_PAD_HOLD (rw) register accessor: Configures the hold option for digital GPIOs

You can [`read`](crate::generic::Reg::read) this register and get [`dig_pad_hold::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dig_pad_hold::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dig_pad_hold`] module*/
pub type DIG_PAD_HOLD = crate::Reg<dig_pad_hold::DIG_PAD_HOLD_SPEC>;
///Configures the hold option for digital GPIOs
pub mod dig_pad_hold;
/**EXT_WAKEUP1 (rw) register accessor: EXT1 wakeup configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`ext_wakeup1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ext_wakeup1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ext_wakeup1`] module*/
pub type EXT_WAKEUP1 = crate::Reg<ext_wakeup1::EXT_WAKEUP1_SPEC>;
///EXT1 wakeup configuration register
pub mod ext_wakeup1;
/**EXT_WAKEUP1_STATUS (r) register accessor: EXT1 wakeup source register

You can [`read`](crate::generic::Reg::read) this register and get [`ext_wakeup1_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ext_wakeup1_status`] module*/
pub type EXT_WAKEUP1_STATUS = crate::Reg<ext_wakeup1_status::EXT_WAKEUP1_STATUS_SPEC>;
///EXT1 wakeup source register
pub mod ext_wakeup1_status;
/**BROWN_OUT (rw) register accessor: Brownout configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`brown_out::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`brown_out::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@brown_out`] module*/
pub type BROWN_OUT = crate::Reg<brown_out::BROWN_OUT_SPEC>;
///Brownout configuration register
pub mod brown_out;
/**TIME_LOW1 (r) register accessor: Stores the lower 32 bits of RTC timer 1

You can [`read`](crate::generic::Reg::read) this register and get [`time_low1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@time_low1`] module*/
pub type TIME_LOW1 = crate::Reg<time_low1::TIME_LOW1_SPEC>;
///Stores the lower 32 bits of RTC timer 1
pub mod time_low1;
/**TIME_HIGH1 (r) register accessor: Stores the higher 16 bits of RTC timer 1

You can [`read`](crate::generic::Reg::read) this register and get [`time_high1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@time_high1`] module*/
pub type TIME_HIGH1 = crate::Reg<time_high1::TIME_HIGH1_SPEC>;
///Stores the higher 16 bits of RTC timer 1
pub mod time_high1;
/**XTAL32K_CLK_FACTOR (rw) register accessor: Configures the divider factor for the backup clock of 32 kHz crystal oscillator

You can [`read`](crate::generic::Reg::read) this register and get [`xtal32k_clk_factor::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`xtal32k_clk_factor::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@xtal32k_clk_factor`] module*/
pub type XTAL32K_CLK_FACTOR = crate::Reg<xtal32k_clk_factor::XTAL32K_CLK_FACTOR_SPEC>;
///Configures the divider factor for the backup clock of 32 kHz crystal oscillator
pub mod xtal32k_clk_factor;
/**XTAL32K_CONF (rw) register accessor: 32 kHz crystal oscillator configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`xtal32k_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`xtal32k_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@xtal32k_conf`] module*/
pub type XTAL32K_CONF = crate::Reg<xtal32k_conf::XTAL32K_CONF_SPEC>;
///32 kHz crystal oscillator configuration register
pub mod xtal32k_conf;
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
/**TOUCH_CTRL1 (rw) register accessor: Touch control register

You can [`read`](crate::generic::Reg::read) this register and get [`touch_ctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`touch_ctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@touch_ctrl1`] module*/
pub type TOUCH_CTRL1 = crate::Reg<touch_ctrl1::TOUCH_CTRL1_SPEC>;
///Touch control register
pub mod touch_ctrl1;
/**TOUCH_CTRL2 (rw) register accessor: Touch control register

You can [`read`](crate::generic::Reg::read) this register and get [`touch_ctrl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`touch_ctrl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@touch_ctrl2`] module*/
pub type TOUCH_CTRL2 = crate::Reg<touch_ctrl2::TOUCH_CTRL2_SPEC>;
///Touch control register
pub mod touch_ctrl2;
/**TOUCH_SCAN_CTRL (rw) register accessor: Configure touch scan settings

You can [`read`](crate::generic::Reg::read) this register and get [`touch_scan_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`touch_scan_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@touch_scan_ctrl`] module*/
pub type TOUCH_SCAN_CTRL = crate::Reg<touch_scan_ctrl::TOUCH_SCAN_CTRL_SPEC>;
///Configure touch scan settings
pub mod touch_scan_ctrl;
/**TOUCH_SLP_THRES (rw) register accessor: Configure the settings of touch sleep pad

You can [`read`](crate::generic::Reg::read) this register and get [`touch_slp_thres::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`touch_slp_thres::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@touch_slp_thres`] module*/
pub type TOUCH_SLP_THRES = crate::Reg<touch_slp_thres::TOUCH_SLP_THRES_SPEC>;
///Configure the settings of touch sleep pad
pub mod touch_slp_thres;
/**TOUCH_APPROACH (rw) register accessor: Configure touch approach settings

You can [`read`](crate::generic::Reg::read) this register and get [`touch_approach::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`touch_approach::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@touch_approach`] module*/
pub type TOUCH_APPROACH = crate::Reg<touch_approach::TOUCH_APPROACH_SPEC>;
///Configure touch approach settings
pub mod touch_approach;
/**TOUCH_FILTER_CTRL (rw) register accessor: Configure touch filter settings

You can [`read`](crate::generic::Reg::read) this register and get [`touch_filter_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`touch_filter_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@touch_filter_ctrl`] module*/
pub type TOUCH_FILTER_CTRL = crate::Reg<touch_filter_ctrl::TOUCH_FILTER_CTRL_SPEC>;
///Configure touch filter settings
pub mod touch_filter_ctrl;
/**USB_CONF (rw) register accessor: configure usb control register

You can [`read`](crate::generic::Reg::read) this register and get [`usb_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@usb_conf`] module*/
pub type USB_CONF = crate::Reg<usb_conf::USB_CONF_SPEC>;
///configure usb control register
pub mod usb_conf;
/**TOUCH_TIMEOUT_CTRL (rw) register accessor: Configure touch timeout settings

You can [`read`](crate::generic::Reg::read) this register and get [`touch_timeout_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`touch_timeout_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@touch_timeout_ctrl`] module*/
pub type TOUCH_TIMEOUT_CTRL = crate::Reg<touch_timeout_ctrl::TOUCH_TIMEOUT_CTRL_SPEC>;
///Configure touch timeout settings
pub mod touch_timeout_ctrl;
/**SLP_REJECT_CAUSE (r) register accessor: Stores the reject-to-sleep cause.

You can [`read`](crate::generic::Reg::read) this register and get [`slp_reject_cause::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@slp_reject_cause`] module*/
pub type SLP_REJECT_CAUSE = crate::Reg<slp_reject_cause::SLP_REJECT_CAUSE_SPEC>;
///Stores the reject-to-sleep cause.
pub mod slp_reject_cause;
/**OPTIONS1 (rw) register accessor: RTC option register

You can [`read`](crate::generic::Reg::read) this register and get [`options1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`options1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@options1`] module*/
pub type OPTIONS1 = crate::Reg<options1::OPTIONS1_SPEC>;
///RTC option register
pub mod options1;
/**SLP_WAKEUP_CAUSE (r) register accessor: Stores the sleep-to-wakeup cause.

You can [`read`](crate::generic::Reg::read) this register and get [`slp_wakeup_cause::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@slp_wakeup_cause`] module*/
pub type SLP_WAKEUP_CAUSE = crate::Reg<slp_wakeup_cause::SLP_WAKEUP_CAUSE_SPEC>;
///Stores the sleep-to-wakeup cause.
pub mod slp_wakeup_cause;
/**ULP_CP_TIMER_1 (rw) register accessor: Configure sleep cycle of the timer

You can [`read`](crate::generic::Reg::read) this register and get [`ulp_cp_timer_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ulp_cp_timer_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ulp_cp_timer_1`] module*/
pub type ULP_CP_TIMER_1 = crate::Reg<ulp_cp_timer_1::ULP_CP_TIMER_1_SPEC>;
///Configure sleep cycle of the timer
pub mod ulp_cp_timer_1;
/**DATE (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@date`] module*/
pub type DATE = crate::Reg<date::DATE_SPEC>;
///
pub mod date;
