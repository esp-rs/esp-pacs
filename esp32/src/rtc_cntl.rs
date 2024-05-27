#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
///Register block
pub struct RegisterBlock {
    options0: OPTIONS0,
    slp_timer0: SLP_TIMER0,
    slp_timer1: SLP_TIMER1,
    time_update: TIME_UPDATE,
    time0: TIME0,
    time1: TIME1,
    state0: STATE0,
    timer1: TIMER1,
    timer2: TIMER2,
    timer3: TIMER3,
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
    sdio_act_conf: SDIO_ACT_CONF,
    clk_conf: CLK_CONF,
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
    test_mux: TEST_MUX,
    sw_cpu_stall: SW_CPU_STALL,
    store4: STORE4,
    store5: STORE5,
    store6: STORE6,
    store7: STORE7,
    low_power_st: LOW_POWER_ST,
    diag1: DIAG1,
    hold_force: HOLD_FORCE,
    ext_wakeup1: EXT_WAKEUP1,
    ext_wakeup1_status: EXT_WAKEUP1_STATUS,
    brown_out: BROWN_OUT,
    _reserved54: [u8; 0x64],
    date: DATE,
}
impl RegisterBlock {
    ///0x00 -
    #[inline(always)]
    pub const fn options0(&self) -> &OPTIONS0 {
        &self.options0
    }
    ///0x04 -
    #[inline(always)]
    pub const fn slp_timer0(&self) -> &SLP_TIMER0 {
        &self.slp_timer0
    }
    ///0x08 -
    #[inline(always)]
    pub const fn slp_timer1(&self) -> &SLP_TIMER1 {
        &self.slp_timer1
    }
    ///0x0c -
    #[inline(always)]
    pub const fn time_update(&self) -> &TIME_UPDATE {
        &self.time_update
    }
    ///0x10 -
    #[inline(always)]
    pub const fn time0(&self) -> &TIME0 {
        &self.time0
    }
    ///0x14 -
    #[inline(always)]
    pub const fn time1(&self) -> &TIME1 {
        &self.time1
    }
    ///0x18 -
    #[inline(always)]
    pub const fn state0(&self) -> &STATE0 {
        &self.state0
    }
    ///0x1c -
    #[inline(always)]
    pub const fn timer1(&self) -> &TIMER1 {
        &self.timer1
    }
    ///0x20 -
    #[inline(always)]
    pub const fn timer2(&self) -> &TIMER2 {
        &self.timer2
    }
    ///0x24 -
    #[inline(always)]
    pub const fn timer3(&self) -> &TIMER3 {
        &self.timer3
    }
    ///0x28 -
    #[inline(always)]
    pub const fn timer4(&self) -> &TIMER4 {
        &self.timer4
    }
    ///0x2c -
    #[inline(always)]
    pub const fn timer5(&self) -> &TIMER5 {
        &self.timer5
    }
    ///0x30 -
    #[inline(always)]
    pub const fn ana_conf(&self) -> &ANA_CONF {
        &self.ana_conf
    }
    ///0x34 -
    #[inline(always)]
    pub const fn reset_state(&self) -> &RESET_STATE {
        &self.reset_state
    }
    ///0x38 -
    #[inline(always)]
    pub const fn wakeup_state(&self) -> &WAKEUP_STATE {
        &self.wakeup_state
    }
    ///0x3c -
    #[inline(always)]
    pub const fn int_ena(&self) -> &INT_ENA {
        &self.int_ena
    }
    ///0x40 -
    #[inline(always)]
    pub const fn int_raw(&self) -> &INT_RAW {
        &self.int_raw
    }
    ///0x44 -
    #[inline(always)]
    pub const fn int_st(&self) -> &INT_ST {
        &self.int_st
    }
    ///0x48 -
    #[inline(always)]
    pub const fn int_clr(&self) -> &INT_CLR {
        &self.int_clr
    }
    ///0x4c -
    #[inline(always)]
    pub const fn store0(&self) -> &STORE0 {
        &self.store0
    }
    ///0x50 -
    #[inline(always)]
    pub const fn store1(&self) -> &STORE1 {
        &self.store1
    }
    ///0x54 -
    #[inline(always)]
    pub const fn store2(&self) -> &STORE2 {
        &self.store2
    }
    ///0x58 -
    #[inline(always)]
    pub const fn store3(&self) -> &STORE3 {
        &self.store3
    }
    ///0x5c -
    #[inline(always)]
    pub const fn ext_xtl_conf(&self) -> &EXT_XTL_CONF {
        &self.ext_xtl_conf
    }
    ///0x60 -
    #[inline(always)]
    pub const fn ext_wakeup_conf(&self) -> &EXT_WAKEUP_CONF {
        &self.ext_wakeup_conf
    }
    ///0x64 -
    #[inline(always)]
    pub const fn slp_reject_conf(&self) -> &SLP_REJECT_CONF {
        &self.slp_reject_conf
    }
    ///0x68 -
    #[inline(always)]
    pub const fn cpu_period_conf(&self) -> &CPU_PERIOD_CONF {
        &self.cpu_period_conf
    }
    ///0x6c -
    #[inline(always)]
    pub const fn sdio_act_conf(&self) -> &SDIO_ACT_CONF {
        &self.sdio_act_conf
    }
    ///0x70 -
    #[inline(always)]
    pub const fn clk_conf(&self) -> &CLK_CONF {
        &self.clk_conf
    }
    ///0x74 -
    #[inline(always)]
    pub const fn sdio_conf(&self) -> &SDIO_CONF {
        &self.sdio_conf
    }
    ///0x78 -
    #[inline(always)]
    pub const fn bias_conf(&self) -> &BIAS_CONF {
        &self.bias_conf
    }
    ///0x7c -
    #[inline(always)]
    pub const fn reg(&self) -> &REG {
        &self.reg
    }
    ///0x80 -
    #[inline(always)]
    pub const fn pwc(&self) -> &PWC {
        &self.pwc
    }
    ///0x84 -
    #[inline(always)]
    pub const fn dig_pwc(&self) -> &DIG_PWC {
        &self.dig_pwc
    }
    ///0x88 -
    #[inline(always)]
    pub const fn dig_iso(&self) -> &DIG_ISO {
        &self.dig_iso
    }
    ///0x8c -
    #[inline(always)]
    pub const fn wdtconfig0(&self) -> &WDTCONFIG0 {
        &self.wdtconfig0
    }
    ///0x90 -
    #[inline(always)]
    pub const fn wdtconfig1(&self) -> &WDTCONFIG1 {
        &self.wdtconfig1
    }
    ///0x94 -
    #[inline(always)]
    pub const fn wdtconfig2(&self) -> &WDTCONFIG2 {
        &self.wdtconfig2
    }
    ///0x98 -
    #[inline(always)]
    pub const fn wdtconfig3(&self) -> &WDTCONFIG3 {
        &self.wdtconfig3
    }
    ///0x9c -
    #[inline(always)]
    pub const fn wdtconfig4(&self) -> &WDTCONFIG4 {
        &self.wdtconfig4
    }
    ///0xa0 -
    #[inline(always)]
    pub const fn wdtfeed(&self) -> &WDTFEED {
        &self.wdtfeed
    }
    ///0xa4 -
    #[inline(always)]
    pub const fn wdtwprotect(&self) -> &WDTWPROTECT {
        &self.wdtwprotect
    }
    ///0xa8 -
    #[inline(always)]
    pub const fn test_mux(&self) -> &TEST_MUX {
        &self.test_mux
    }
    ///0xac -
    #[inline(always)]
    pub const fn sw_cpu_stall(&self) -> &SW_CPU_STALL {
        &self.sw_cpu_stall
    }
    ///0xb0 -
    #[inline(always)]
    pub const fn store4(&self) -> &STORE4 {
        &self.store4
    }
    ///0xb4 -
    #[inline(always)]
    pub const fn store5(&self) -> &STORE5 {
        &self.store5
    }
    ///0xb8 -
    #[inline(always)]
    pub const fn store6(&self) -> &STORE6 {
        &self.store6
    }
    ///0xbc -
    #[inline(always)]
    pub const fn store7(&self) -> &STORE7 {
        &self.store7
    }
    ///0xc0 -
    #[inline(always)]
    pub const fn low_power_st(&self) -> &LOW_POWER_ST {
        &self.low_power_st
    }
    ///0xc4 -
    #[inline(always)]
    pub const fn diag1(&self) -> &DIAG1 {
        &self.diag1
    }
    ///0xc8 -
    #[inline(always)]
    pub const fn hold_force(&self) -> &HOLD_FORCE {
        &self.hold_force
    }
    ///0xcc -
    #[inline(always)]
    pub const fn ext_wakeup1(&self) -> &EXT_WAKEUP1 {
        &self.ext_wakeup1
    }
    ///0xd0 -
    #[inline(always)]
    pub const fn ext_wakeup1_status(&self) -> &EXT_WAKEUP1_STATUS {
        &self.ext_wakeup1_status
    }
    ///0xd4 -
    #[inline(always)]
    pub const fn brown_out(&self) -> &BROWN_OUT {
        &self.brown_out
    }
    ///0x13c -
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
/**OPTIONS0 (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`options0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`options0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@options0`] module*/
pub type OPTIONS0 = crate::Reg<options0::OPTIONS0_SPEC>;
///
pub mod options0;
/**SLP_TIMER0 (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`slp_timer0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slp_timer0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@slp_timer0`] module*/
pub type SLP_TIMER0 = crate::Reg<slp_timer0::SLP_TIMER0_SPEC>;
///
pub mod slp_timer0;
/**SLP_TIMER1 (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`slp_timer1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slp_timer1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@slp_timer1`] module*/
pub type SLP_TIMER1 = crate::Reg<slp_timer1::SLP_TIMER1_SPEC>;
///
pub mod slp_timer1;
/**TIME_UPDATE (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`time_update::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`time_update::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@time_update`] module*/
pub type TIME_UPDATE = crate::Reg<time_update::TIME_UPDATE_SPEC>;
///
pub mod time_update;
/**TIME0 (r) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`time0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@time0`] module*/
pub type TIME0 = crate::Reg<time0::TIME0_SPEC>;
///
pub mod time0;
/**TIME1 (r) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`time1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@time1`] module*/
pub type TIME1 = crate::Reg<time1::TIME1_SPEC>;
///
pub mod time1;
/**STATE0 (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`state0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`state0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@state0`] module*/
pub type STATE0 = crate::Reg<state0::STATE0_SPEC>;
///
pub mod state0;
/**TIMER1 (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`timer1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@timer1`] module*/
pub type TIMER1 = crate::Reg<timer1::TIMER1_SPEC>;
///
pub mod timer1;
/**TIMER2 (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`timer2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@timer2`] module*/
pub type TIMER2 = crate::Reg<timer2::TIMER2_SPEC>;
///
pub mod timer2;
/**TIMER3 (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`timer3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@timer3`] module*/
pub type TIMER3 = crate::Reg<timer3::TIMER3_SPEC>;
///
pub mod timer3;
/**TIMER4 (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`timer4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@timer4`] module*/
pub type TIMER4 = crate::Reg<timer4::TIMER4_SPEC>;
///
pub mod timer4;
/**TIMER5 (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`timer5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@timer5`] module*/
pub type TIMER5 = crate::Reg<timer5::TIMER5_SPEC>;
///
pub mod timer5;
/**ANA_CONF (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`ana_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ana_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ana_conf`] module*/
pub type ANA_CONF = crate::Reg<ana_conf::ANA_CONF_SPEC>;
///
pub mod ana_conf;
/**RESET_STATE (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`reset_state::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reset_state::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@reset_state`] module*/
pub type RESET_STATE = crate::Reg<reset_state::RESET_STATE_SPEC>;
///
pub mod reset_state;
/**WAKEUP_STATE (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`wakeup_state::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wakeup_state::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@wakeup_state`] module*/
pub type WAKEUP_STATE = crate::Reg<wakeup_state::WAKEUP_STATE_SPEC>;
///
pub mod wakeup_state;
/**INT_ENA (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_ena`] module*/
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
///
pub mod int_ena;
/**INT_RAW (r) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_raw`] module*/
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
///
pub mod int_raw;
/**INT_ST (r) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_st`] module*/
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
///
pub mod int_st;
/**INT_CLR (w) register accessor:

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_clr`] module*/
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
///
pub mod int_clr;
/**STORE0 (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`store0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`store0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@store0`] module*/
pub type STORE0 = crate::Reg<store0::STORE0_SPEC>;
///
pub mod store0;
/**STORE1 (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`store1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`store1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@store1`] module*/
pub type STORE1 = crate::Reg<store1::STORE1_SPEC>;
///
pub mod store1;
/**STORE2 (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`store2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`store2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@store2`] module*/
pub type STORE2 = crate::Reg<store2::STORE2_SPEC>;
///
pub mod store2;
/**STORE3 (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`store3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`store3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@store3`] module*/
pub type STORE3 = crate::Reg<store3::STORE3_SPEC>;
///
pub mod store3;
/**EXT_XTL_CONF (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`ext_xtl_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ext_xtl_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ext_xtl_conf`] module*/
pub type EXT_XTL_CONF = crate::Reg<ext_xtl_conf::EXT_XTL_CONF_SPEC>;
///
pub mod ext_xtl_conf;
/**EXT_WAKEUP_CONF (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`ext_wakeup_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ext_wakeup_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ext_wakeup_conf`] module*/
pub type EXT_WAKEUP_CONF = crate::Reg<ext_wakeup_conf::EXT_WAKEUP_CONF_SPEC>;
///
pub mod ext_wakeup_conf;
/**SLP_REJECT_CONF (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`slp_reject_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slp_reject_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@slp_reject_conf`] module*/
pub type SLP_REJECT_CONF = crate::Reg<slp_reject_conf::SLP_REJECT_CONF_SPEC>;
///
pub mod slp_reject_conf;
/**CPU_PERIOD_CONF (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`cpu_period_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_period_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cpu_period_conf`] module*/
pub type CPU_PERIOD_CONF = crate::Reg<cpu_period_conf::CPU_PERIOD_CONF_SPEC>;
///
pub mod cpu_period_conf;
/**SDIO_ACT_CONF (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`sdio_act_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdio_act_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sdio_act_conf`] module*/
pub type SDIO_ACT_CONF = crate::Reg<sdio_act_conf::SDIO_ACT_CONF_SPEC>;
///
pub mod sdio_act_conf;
/**CLK_CONF (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`clk_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@clk_conf`] module*/
pub type CLK_CONF = crate::Reg<clk_conf::CLK_CONF_SPEC>;
///
pub mod clk_conf;
/**SDIO_CONF (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`sdio_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdio_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sdio_conf`] module*/
pub type SDIO_CONF = crate::Reg<sdio_conf::SDIO_CONF_SPEC>;
///
pub mod sdio_conf;
/**BIAS_CONF (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`bias_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bias_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@bias_conf`] module*/
pub type BIAS_CONF = crate::Reg<bias_conf::BIAS_CONF_SPEC>;
///
pub mod bias_conf;
/**REG (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@reg`] module*/
pub type REG = crate::Reg<reg::REG_SPEC>;
///
pub mod reg;
/**PWC (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`pwc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pwc`] module*/
pub type PWC = crate::Reg<pwc::PWC_SPEC>;
///
pub mod pwc;
/**DIG_PWC (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`dig_pwc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dig_pwc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dig_pwc`] module*/
pub type DIG_PWC = crate::Reg<dig_pwc::DIG_PWC_SPEC>;
///
pub mod dig_pwc;
/**DIG_ISO (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`dig_iso::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dig_iso::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dig_iso`] module*/
pub type DIG_ISO = crate::Reg<dig_iso::DIG_ISO_SPEC>;
///
pub mod dig_iso;
/**WDTCONFIG0 (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`wdtconfig0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtconfig0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@wdtconfig0`] module*/
pub type WDTCONFIG0 = crate::Reg<wdtconfig0::WDTCONFIG0_SPEC>;
///
pub mod wdtconfig0;
/**WDTCONFIG1 (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`wdtconfig1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtconfig1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@wdtconfig1`] module*/
pub type WDTCONFIG1 = crate::Reg<wdtconfig1::WDTCONFIG1_SPEC>;
///
pub mod wdtconfig1;
/**WDTCONFIG2 (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`wdtconfig2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtconfig2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@wdtconfig2`] module*/
pub type WDTCONFIG2 = crate::Reg<wdtconfig2::WDTCONFIG2_SPEC>;
///
pub mod wdtconfig2;
/**WDTCONFIG3 (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`wdtconfig3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtconfig3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@wdtconfig3`] module*/
pub type WDTCONFIG3 = crate::Reg<wdtconfig3::WDTCONFIG3_SPEC>;
///
pub mod wdtconfig3;
/**WDTCONFIG4 (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`wdtconfig4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtconfig4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@wdtconfig4`] module*/
pub type WDTCONFIG4 = crate::Reg<wdtconfig4::WDTCONFIG4_SPEC>;
///
pub mod wdtconfig4;
/**WDTFEED (w) register accessor:

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtfeed::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@wdtfeed`] module*/
pub type WDTFEED = crate::Reg<wdtfeed::WDTFEED_SPEC>;
///
pub mod wdtfeed;
/**WDTWPROTECT (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`wdtwprotect::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtwprotect::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@wdtwprotect`] module*/
pub type WDTWPROTECT = crate::Reg<wdtwprotect::WDTWPROTECT_SPEC>;
///
pub mod wdtwprotect;
/**TEST_MUX (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`test_mux::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`test_mux::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@test_mux`] module*/
pub type TEST_MUX = crate::Reg<test_mux::TEST_MUX_SPEC>;
///
pub mod test_mux;
/**SW_CPU_STALL (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`sw_cpu_stall::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sw_cpu_stall::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sw_cpu_stall`] module*/
pub type SW_CPU_STALL = crate::Reg<sw_cpu_stall::SW_CPU_STALL_SPEC>;
///
pub mod sw_cpu_stall;
/**STORE4 (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`store4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`store4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@store4`] module*/
pub type STORE4 = crate::Reg<store4::STORE4_SPEC>;
///
pub mod store4;
/**STORE5 (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`store5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`store5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@store5`] module*/
pub type STORE5 = crate::Reg<store5::STORE5_SPEC>;
///
pub mod store5;
/**STORE6 (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`store6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`store6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@store6`] module*/
pub type STORE6 = crate::Reg<store6::STORE6_SPEC>;
///
pub mod store6;
/**STORE7 (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`store7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`store7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@store7`] module*/
pub type STORE7 = crate::Reg<store7::STORE7_SPEC>;
///
pub mod store7;
/**LOW_POWER_ST (r) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`low_power_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@low_power_st`] module*/
pub type LOW_POWER_ST = crate::Reg<low_power_st::LOW_POWER_ST_SPEC>;
///
pub mod low_power_st;
/**DIAG1 (r) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`diag1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@diag1`] module*/
pub type DIAG1 = crate::Reg<diag1::DIAG1_SPEC>;
///
pub mod diag1;
/**HOLD_FORCE (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`hold_force::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hold_force::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hold_force`] module*/
pub type HOLD_FORCE = crate::Reg<hold_force::HOLD_FORCE_SPEC>;
///
pub mod hold_force;
/**EXT_WAKEUP1 (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`ext_wakeup1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ext_wakeup1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ext_wakeup1`] module*/
pub type EXT_WAKEUP1 = crate::Reg<ext_wakeup1::EXT_WAKEUP1_SPEC>;
///
pub mod ext_wakeup1;
/**EXT_WAKEUP1_STATUS (r) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`ext_wakeup1_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ext_wakeup1_status`] module*/
pub type EXT_WAKEUP1_STATUS = crate::Reg<ext_wakeup1_status::EXT_WAKEUP1_STATUS_SPEC>;
///
pub mod ext_wakeup1_status;
/**BROWN_OUT (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`brown_out::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`brown_out::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@brown_out`] module*/
pub type BROWN_OUT = crate::Reg<brown_out::BROWN_OUT_SPEC>;
///
pub mod brown_out;
/**DATE (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@date`] module*/
pub type DATE = crate::Reg<date::DATE_SPEC>;
///
pub mod date;
