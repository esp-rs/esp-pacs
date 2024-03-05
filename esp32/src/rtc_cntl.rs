#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
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
    #[doc = "0x00 - "]
    #[inline(always)]
    pub const fn options0(&self) -> &OPTIONS0 {
        &self.options0
    }
    #[doc = "0x04 - "]
    #[inline(always)]
    pub const fn slp_timer0(&self) -> &SLP_TIMER0 {
        &self.slp_timer0
    }
    #[doc = "0x08 - "]
    #[inline(always)]
    pub const fn slp_timer1(&self) -> &SLP_TIMER1 {
        &self.slp_timer1
    }
    #[doc = "0x0c - "]
    #[inline(always)]
    pub const fn time_update(&self) -> &TIME_UPDATE {
        &self.time_update
    }
    #[doc = "0x10 - "]
    #[inline(always)]
    pub const fn time0(&self) -> &TIME0 {
        &self.time0
    }
    #[doc = "0x14 - "]
    #[inline(always)]
    pub const fn time1(&self) -> &TIME1 {
        &self.time1
    }
    #[doc = "0x18 - "]
    #[inline(always)]
    pub const fn state0(&self) -> &STATE0 {
        &self.state0
    }
    #[doc = "0x1c - "]
    #[inline(always)]
    pub const fn timer1(&self) -> &TIMER1 {
        &self.timer1
    }
    #[doc = "0x20 - "]
    #[inline(always)]
    pub const fn timer2(&self) -> &TIMER2 {
        &self.timer2
    }
    #[doc = "0x24 - "]
    #[inline(always)]
    pub const fn timer3(&self) -> &TIMER3 {
        &self.timer3
    }
    #[doc = "0x28 - "]
    #[inline(always)]
    pub const fn timer4(&self) -> &TIMER4 {
        &self.timer4
    }
    #[doc = "0x2c - "]
    #[inline(always)]
    pub const fn timer5(&self) -> &TIMER5 {
        &self.timer5
    }
    #[doc = "0x30 - "]
    #[inline(always)]
    pub const fn ana_conf(&self) -> &ANA_CONF {
        &self.ana_conf
    }
    #[doc = "0x34 - "]
    #[inline(always)]
    pub const fn reset_state(&self) -> &RESET_STATE {
        &self.reset_state
    }
    #[doc = "0x38 - "]
    #[inline(always)]
    pub const fn wakeup_state(&self) -> &WAKEUP_STATE {
        &self.wakeup_state
    }
    #[doc = "0x3c - "]
    #[inline(always)]
    pub const fn int_ena(&self) -> &INT_ENA {
        &self.int_ena
    }
    #[doc = "0x40 - "]
    #[inline(always)]
    pub const fn int_raw(&self) -> &INT_RAW {
        &self.int_raw
    }
    #[doc = "0x44 - "]
    #[inline(always)]
    pub const fn int_st(&self) -> &INT_ST {
        &self.int_st
    }
    #[doc = "0x48 - "]
    #[inline(always)]
    pub const fn int_clr(&self) -> &INT_CLR {
        &self.int_clr
    }
    #[doc = "0x4c - "]
    #[inline(always)]
    pub const fn store0(&self) -> &STORE0 {
        &self.store0
    }
    #[doc = "0x50 - "]
    #[inline(always)]
    pub const fn store1(&self) -> &STORE1 {
        &self.store1
    }
    #[doc = "0x54 - "]
    #[inline(always)]
    pub const fn store2(&self) -> &STORE2 {
        &self.store2
    }
    #[doc = "0x58 - "]
    #[inline(always)]
    pub const fn store3(&self) -> &STORE3 {
        &self.store3
    }
    #[doc = "0x5c - "]
    #[inline(always)]
    pub const fn ext_xtl_conf(&self) -> &EXT_XTL_CONF {
        &self.ext_xtl_conf
    }
    #[doc = "0x60 - "]
    #[inline(always)]
    pub const fn ext_wakeup_conf(&self) -> &EXT_WAKEUP_CONF {
        &self.ext_wakeup_conf
    }
    #[doc = "0x64 - "]
    #[inline(always)]
    pub const fn slp_reject_conf(&self) -> &SLP_REJECT_CONF {
        &self.slp_reject_conf
    }
    #[doc = "0x68 - "]
    #[inline(always)]
    pub const fn cpu_period_conf(&self) -> &CPU_PERIOD_CONF {
        &self.cpu_period_conf
    }
    #[doc = "0x6c - "]
    #[inline(always)]
    pub const fn sdio_act_conf(&self) -> &SDIO_ACT_CONF {
        &self.sdio_act_conf
    }
    #[doc = "0x70 - "]
    #[inline(always)]
    pub const fn clk_conf(&self) -> &CLK_CONF {
        &self.clk_conf
    }
    #[doc = "0x74 - "]
    #[inline(always)]
    pub const fn sdio_conf(&self) -> &SDIO_CONF {
        &self.sdio_conf
    }
    #[doc = "0x78 - "]
    #[inline(always)]
    pub const fn bias_conf(&self) -> &BIAS_CONF {
        &self.bias_conf
    }
    #[doc = "0x7c - "]
    #[inline(always)]
    pub const fn reg(&self) -> &REG {
        &self.reg
    }
    #[doc = "0x80 - "]
    #[inline(always)]
    pub const fn pwc(&self) -> &PWC {
        &self.pwc
    }
    #[doc = "0x84 - "]
    #[inline(always)]
    pub const fn dig_pwc(&self) -> &DIG_PWC {
        &self.dig_pwc
    }
    #[doc = "0x88 - "]
    #[inline(always)]
    pub const fn dig_iso(&self) -> &DIG_ISO {
        &self.dig_iso
    }
    #[doc = "0x8c - "]
    #[inline(always)]
    pub const fn wdtconfig0(&self) -> &WDTCONFIG0 {
        &self.wdtconfig0
    }
    #[doc = "0x90 - "]
    #[inline(always)]
    pub const fn wdtconfig1(&self) -> &WDTCONFIG1 {
        &self.wdtconfig1
    }
    #[doc = "0x94 - "]
    #[inline(always)]
    pub const fn wdtconfig2(&self) -> &WDTCONFIG2 {
        &self.wdtconfig2
    }
    #[doc = "0x98 - "]
    #[inline(always)]
    pub const fn wdtconfig3(&self) -> &WDTCONFIG3 {
        &self.wdtconfig3
    }
    #[doc = "0x9c - "]
    #[inline(always)]
    pub const fn wdtconfig4(&self) -> &WDTCONFIG4 {
        &self.wdtconfig4
    }
    #[doc = "0xa0 - "]
    #[inline(always)]
    pub const fn wdtfeed(&self) -> &WDTFEED {
        &self.wdtfeed
    }
    #[doc = "0xa4 - "]
    #[inline(always)]
    pub const fn wdtwprotect(&self) -> &WDTWPROTECT {
        &self.wdtwprotect
    }
    #[doc = "0xa8 - "]
    #[inline(always)]
    pub const fn test_mux(&self) -> &TEST_MUX {
        &self.test_mux
    }
    #[doc = "0xac - "]
    #[inline(always)]
    pub const fn sw_cpu_stall(&self) -> &SW_CPU_STALL {
        &self.sw_cpu_stall
    }
    #[doc = "0xb0 - "]
    #[inline(always)]
    pub const fn store4(&self) -> &STORE4 {
        &self.store4
    }
    #[doc = "0xb4 - "]
    #[inline(always)]
    pub const fn store5(&self) -> &STORE5 {
        &self.store5
    }
    #[doc = "0xb8 - "]
    #[inline(always)]
    pub const fn store6(&self) -> &STORE6 {
        &self.store6
    }
    #[doc = "0xbc - "]
    #[inline(always)]
    pub const fn store7(&self) -> &STORE7 {
        &self.store7
    }
    #[doc = "0xc0 - "]
    #[inline(always)]
    pub const fn low_power_st(&self) -> &LOW_POWER_ST {
        &self.low_power_st
    }
    #[doc = "0xc4 - "]
    #[inline(always)]
    pub const fn diag1(&self) -> &DIAG1 {
        &self.diag1
    }
    #[doc = "0xc8 - "]
    #[inline(always)]
    pub const fn hold_force(&self) -> &HOLD_FORCE {
        &self.hold_force
    }
    #[doc = "0xcc - "]
    #[inline(always)]
    pub const fn ext_wakeup1(&self) -> &EXT_WAKEUP1 {
        &self.ext_wakeup1
    }
    #[doc = "0xd0 - "]
    #[inline(always)]
    pub const fn ext_wakeup1_status(&self) -> &EXT_WAKEUP1_STATUS {
        &self.ext_wakeup1_status
    }
    #[doc = "0xd4 - "]
    #[inline(always)]
    pub const fn brown_out(&self) -> &BROWN_OUT {
        &self.brown_out
    }
    #[doc = "0x13c - "]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
#[doc = "OPTIONS0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`options0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`options0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@options0`] module"]
pub type OPTIONS0 = crate::Reg<options0::OPTIONS0_SPEC>;
#[doc = ""]
pub mod options0;
#[doc = "SLP_TIMER0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slp_timer0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slp_timer0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slp_timer0`] module"]
pub type SLP_TIMER0 = crate::Reg<slp_timer0::SLP_TIMER0_SPEC>;
#[doc = ""]
pub mod slp_timer0;
#[doc = "SLP_TIMER1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slp_timer1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slp_timer1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slp_timer1`] module"]
pub type SLP_TIMER1 = crate::Reg<slp_timer1::SLP_TIMER1_SPEC>;
#[doc = ""]
pub mod slp_timer1;
#[doc = "TIME_UPDATE (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`time_update::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`time_update::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@time_update`] module"]
pub type TIME_UPDATE = crate::Reg<time_update::TIME_UPDATE_SPEC>;
#[doc = ""]
pub mod time_update;
#[doc = "TIME0 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`time0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@time0`] module"]
pub type TIME0 = crate::Reg<time0::TIME0_SPEC>;
#[doc = ""]
pub mod time0;
#[doc = "TIME1 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`time1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@time1`] module"]
pub type TIME1 = crate::Reg<time1::TIME1_SPEC>;
#[doc = ""]
pub mod time1;
#[doc = "STATE0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`state0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`state0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@state0`] module"]
pub type STATE0 = crate::Reg<state0::STATE0_SPEC>;
#[doc = ""]
pub mod state0;
#[doc = "TIMER1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer1`] module"]
pub type TIMER1 = crate::Reg<timer1::TIMER1_SPEC>;
#[doc = ""]
pub mod timer1;
#[doc = "TIMER2 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer2`] module"]
pub type TIMER2 = crate::Reg<timer2::TIMER2_SPEC>;
#[doc = ""]
pub mod timer2;
#[doc = "TIMER3 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer3`] module"]
pub type TIMER3 = crate::Reg<timer3::TIMER3_SPEC>;
#[doc = ""]
pub mod timer3;
#[doc = "TIMER4 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer4`] module"]
pub type TIMER4 = crate::Reg<timer4::TIMER4_SPEC>;
#[doc = ""]
pub mod timer4;
#[doc = "TIMER5 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer5`] module"]
pub type TIMER5 = crate::Reg<timer5::TIMER5_SPEC>;
#[doc = ""]
pub mod timer5;
#[doc = "ANA_CONF (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ana_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ana_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ana_conf`] module"]
pub type ANA_CONF = crate::Reg<ana_conf::ANA_CONF_SPEC>;
#[doc = ""]
pub mod ana_conf;
#[doc = "RESET_STATE (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reset_state::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reset_state::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reset_state`] module"]
pub type RESET_STATE = crate::Reg<reset_state::RESET_STATE_SPEC>;
#[doc = ""]
pub mod reset_state;
#[doc = "WAKEUP_STATE (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wakeup_state::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wakeup_state::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wakeup_state`] module"]
pub type WAKEUP_STATE = crate::Reg<wakeup_state::WAKEUP_STATE_SPEC>;
#[doc = ""]
pub mod wakeup_state;
#[doc = "INT_ENA (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_ena`] module"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = ""]
pub mod int_ena;
#[doc = "INT_RAW (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_raw`] module"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = ""]
pub mod int_raw;
#[doc = "INT_ST (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_st`] module"]
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
#[doc = ""]
pub mod int_st;
#[doc = "INT_CLR (w) register accessor: \n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_clr`] module"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = ""]
pub mod int_clr;
#[doc = "STORE0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`store0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`store0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@store0`] module"]
pub type STORE0 = crate::Reg<store0::STORE0_SPEC>;
#[doc = ""]
pub mod store0;
#[doc = "STORE1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`store1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`store1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@store1`] module"]
pub type STORE1 = crate::Reg<store1::STORE1_SPEC>;
#[doc = ""]
pub mod store1;
#[doc = "STORE2 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`store2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`store2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@store2`] module"]
pub type STORE2 = crate::Reg<store2::STORE2_SPEC>;
#[doc = ""]
pub mod store2;
#[doc = "STORE3 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`store3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`store3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@store3`] module"]
pub type STORE3 = crate::Reg<store3::STORE3_SPEC>;
#[doc = ""]
pub mod store3;
#[doc = "EXT_XTL_CONF (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ext_xtl_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ext_xtl_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ext_xtl_conf`] module"]
pub type EXT_XTL_CONF = crate::Reg<ext_xtl_conf::EXT_XTL_CONF_SPEC>;
#[doc = ""]
pub mod ext_xtl_conf;
#[doc = "EXT_WAKEUP_CONF (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ext_wakeup_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ext_wakeup_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ext_wakeup_conf`] module"]
pub type EXT_WAKEUP_CONF = crate::Reg<ext_wakeup_conf::EXT_WAKEUP_CONF_SPEC>;
#[doc = ""]
pub mod ext_wakeup_conf;
#[doc = "SLP_REJECT_CONF (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slp_reject_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slp_reject_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slp_reject_conf`] module"]
pub type SLP_REJECT_CONF = crate::Reg<slp_reject_conf::SLP_REJECT_CONF_SPEC>;
#[doc = ""]
pub mod slp_reject_conf;
#[doc = "CPU_PERIOD_CONF (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_period_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_period_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_period_conf`] module"]
pub type CPU_PERIOD_CONF = crate::Reg<cpu_period_conf::CPU_PERIOD_CONF_SPEC>;
#[doc = ""]
pub mod cpu_period_conf;
#[doc = "SDIO_ACT_CONF (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdio_act_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdio_act_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdio_act_conf`] module"]
pub type SDIO_ACT_CONF = crate::Reg<sdio_act_conf::SDIO_ACT_CONF_SPEC>;
#[doc = ""]
pub mod sdio_act_conf;
#[doc = "CLK_CONF (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_conf`] module"]
pub type CLK_CONF = crate::Reg<clk_conf::CLK_CONF_SPEC>;
#[doc = ""]
pub mod clk_conf;
#[doc = "SDIO_CONF (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdio_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdio_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdio_conf`] module"]
pub type SDIO_CONF = crate::Reg<sdio_conf::SDIO_CONF_SPEC>;
#[doc = ""]
pub mod sdio_conf;
#[doc = "BIAS_CONF (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bias_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bias_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bias_conf`] module"]
pub type BIAS_CONF = crate::Reg<bias_conf::BIAS_CONF_SPEC>;
#[doc = ""]
pub mod bias_conf;
#[doc = "REG (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg`] module"]
pub type REG = crate::Reg<reg::REG_SPEC>;
#[doc = ""]
pub mod reg;
#[doc = "PWC (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwc`] module"]
pub type PWC = crate::Reg<pwc::PWC_SPEC>;
#[doc = ""]
pub mod pwc;
#[doc = "DIG_PWC (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dig_pwc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dig_pwc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dig_pwc`] module"]
pub type DIG_PWC = crate::Reg<dig_pwc::DIG_PWC_SPEC>;
#[doc = ""]
pub mod dig_pwc;
#[doc = "DIG_ISO (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dig_iso::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dig_iso::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dig_iso`] module"]
pub type DIG_ISO = crate::Reg<dig_iso::DIG_ISO_SPEC>;
#[doc = ""]
pub mod dig_iso;
#[doc = "WDTCONFIG0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtconfig0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtconfig0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdtconfig0`] module"]
pub type WDTCONFIG0 = crate::Reg<wdtconfig0::WDTCONFIG0_SPEC>;
#[doc = ""]
pub mod wdtconfig0;
#[doc = "WDTCONFIG1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtconfig1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtconfig1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdtconfig1`] module"]
pub type WDTCONFIG1 = crate::Reg<wdtconfig1::WDTCONFIG1_SPEC>;
#[doc = ""]
pub mod wdtconfig1;
#[doc = "WDTCONFIG2 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtconfig2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtconfig2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdtconfig2`] module"]
pub type WDTCONFIG2 = crate::Reg<wdtconfig2::WDTCONFIG2_SPEC>;
#[doc = ""]
pub mod wdtconfig2;
#[doc = "WDTCONFIG3 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtconfig3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtconfig3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdtconfig3`] module"]
pub type WDTCONFIG3 = crate::Reg<wdtconfig3::WDTCONFIG3_SPEC>;
#[doc = ""]
pub mod wdtconfig3;
#[doc = "WDTCONFIG4 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtconfig4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtconfig4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdtconfig4`] module"]
pub type WDTCONFIG4 = crate::Reg<wdtconfig4::WDTCONFIG4_SPEC>;
#[doc = ""]
pub mod wdtconfig4;
#[doc = "WDTFEED (w) register accessor: \n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtfeed::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdtfeed`] module"]
pub type WDTFEED = crate::Reg<wdtfeed::WDTFEED_SPEC>;
#[doc = ""]
pub mod wdtfeed;
#[doc = "WDTWPROTECT (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtwprotect::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtwprotect::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdtwprotect`] module"]
pub type WDTWPROTECT = crate::Reg<wdtwprotect::WDTWPROTECT_SPEC>;
#[doc = ""]
pub mod wdtwprotect;
#[doc = "TEST_MUX (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`test_mux::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`test_mux::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@test_mux`] module"]
pub type TEST_MUX = crate::Reg<test_mux::TEST_MUX_SPEC>;
#[doc = ""]
pub mod test_mux;
#[doc = "SW_CPU_STALL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sw_cpu_stall::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sw_cpu_stall::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sw_cpu_stall`] module"]
pub type SW_CPU_STALL = crate::Reg<sw_cpu_stall::SW_CPU_STALL_SPEC>;
#[doc = ""]
pub mod sw_cpu_stall;
#[doc = "STORE4 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`store4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`store4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@store4`] module"]
pub type STORE4 = crate::Reg<store4::STORE4_SPEC>;
#[doc = ""]
pub mod store4;
#[doc = "STORE5 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`store5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`store5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@store5`] module"]
pub type STORE5 = crate::Reg<store5::STORE5_SPEC>;
#[doc = ""]
pub mod store5;
#[doc = "STORE6 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`store6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`store6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@store6`] module"]
pub type STORE6 = crate::Reg<store6::STORE6_SPEC>;
#[doc = ""]
pub mod store6;
#[doc = "STORE7 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`store7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`store7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@store7`] module"]
pub type STORE7 = crate::Reg<store7::STORE7_SPEC>;
#[doc = ""]
pub mod store7;
#[doc = "LOW_POWER_ST (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`low_power_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@low_power_st`] module"]
pub type LOW_POWER_ST = crate::Reg<low_power_st::LOW_POWER_ST_SPEC>;
#[doc = ""]
pub mod low_power_st;
#[doc = "DIAG1 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diag1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diag1`] module"]
pub type DIAG1 = crate::Reg<diag1::DIAG1_SPEC>;
#[doc = ""]
pub mod diag1;
#[doc = "HOLD_FORCE (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hold_force::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hold_force::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hold_force`] module"]
pub type HOLD_FORCE = crate::Reg<hold_force::HOLD_FORCE_SPEC>;
#[doc = ""]
pub mod hold_force;
#[doc = "EXT_WAKEUP1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ext_wakeup1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ext_wakeup1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ext_wakeup1`] module"]
pub type EXT_WAKEUP1 = crate::Reg<ext_wakeup1::EXT_WAKEUP1_SPEC>;
#[doc = ""]
pub mod ext_wakeup1;
#[doc = "EXT_WAKEUP1_STATUS (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ext_wakeup1_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ext_wakeup1_status`] module"]
pub type EXT_WAKEUP1_STATUS = crate::Reg<ext_wakeup1_status::EXT_WAKEUP1_STATUS_SPEC>;
#[doc = ""]
pub mod ext_wakeup1_status;
#[doc = "BROWN_OUT (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`brown_out::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`brown_out::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@brown_out`] module"]
pub type BROWN_OUT = crate::Reg<brown_out::BROWN_OUT_SPEC>;
#[doc = ""]
pub mod brown_out;
#[doc = "DATE (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = ""]
pub mod date;
