#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    conf: CONF,
    filter_cnt: FILTER_CNT,
    poll_period: POLL_PERIOD,
    delay_event_time: DELAY_EVENT_TIME,
    int_ena: INT_ENA,
    int_raw: INT_RAW,
    int_clr: INT_CLR,
    int_st: INT_ST,
    channel_1_timer0: CHANNEL_1_TIMER0,
    channel_1_timer1: CHANNEL_1_TIMER1,
    channel_2_timer0: CHANNEL_2_TIMER0,
    channel_2_timer1: CHANNEL_2_TIMER1,
    channel_3_timer0: CHANNEL_3_TIMER0,
    channel_3_timer1: CHANNEL_3_TIMER1,
    channel_status: CHANNEL_STATUS,
    pad_comp_cfg: PAD_COMP_CFG,
    start: START,
    _reserved17: [u8; 0x03b8],
    date: DATE,
}
impl RegisterBlock {
    #[doc = "0x00 - zero det cfg reg"]
    #[inline(always)]
    pub const fn conf(&self) -> &CONF {
        &self.conf
    }
    #[doc = "0x04 - protect time reg"]
    #[inline(always)]
    pub const fn filter_cnt(&self) -> &FILTER_CNT {
        &self.filter_cnt
    }
    #[doc = "0x08 - poll period time reg"]
    #[inline(always)]
    pub const fn poll_period(&self) -> &POLL_PERIOD {
        &self.poll_period
    }
    #[doc = "0x0c - delay time reg"]
    #[inline(always)]
    pub const fn delay_event_time(&self) -> &DELAY_EVENT_TIME {
        &self.delay_event_time
    }
    #[doc = "0x10 - zero det int ena"]
    #[inline(always)]
    pub const fn int_ena(&self) -> &INT_ENA {
        &self.int_ena
    }
    #[doc = "0x14 - zero det int raw"]
    #[inline(always)]
    pub const fn int_raw(&self) -> &INT_RAW {
        &self.int_raw
    }
    #[doc = "0x18 - zero det int clr"]
    #[inline(always)]
    pub const fn int_clr(&self) -> &INT_CLR {
        &self.int_clr
    }
    #[doc = "0x1c - zero det int st"]
    #[inline(always)]
    pub const fn int_st(&self) -> &INT_ST {
        &self.int_st
    }
    #[doc = "0x20 - record timer reg"]
    #[inline(always)]
    pub const fn channel_1_timer0(&self) -> &CHANNEL_1_TIMER0 {
        &self.channel_1_timer0
    }
    #[doc = "0x24 - record timer reg"]
    #[inline(always)]
    pub const fn channel_1_timer1(&self) -> &CHANNEL_1_TIMER1 {
        &self.channel_1_timer1
    }
    #[doc = "0x28 - record timer reg"]
    #[inline(always)]
    pub const fn channel_2_timer0(&self) -> &CHANNEL_2_TIMER0 {
        &self.channel_2_timer0
    }
    #[doc = "0x2c - record timer reg"]
    #[inline(always)]
    pub const fn channel_2_timer1(&self) -> &CHANNEL_2_TIMER1 {
        &self.channel_2_timer1
    }
    #[doc = "0x30 - record timer reg"]
    #[inline(always)]
    pub const fn channel_3_timer0(&self) -> &CHANNEL_3_TIMER0 {
        &self.channel_3_timer0
    }
    #[doc = "0x34 - record timer reg"]
    #[inline(always)]
    pub const fn channel_3_timer1(&self) -> &CHANNEL_3_TIMER1 {
        &self.channel_3_timer1
    }
    #[doc = "0x38 - pad comp status reg"]
    #[inline(always)]
    pub const fn channel_status(&self) -> &CHANNEL_STATUS {
        &self.channel_status
    }
    #[doc = "0x3c - pad comp cfg reg"]
    #[inline(always)]
    pub const fn pad_comp_cfg(&self) -> &PAD_COMP_CFG {
        &self.pad_comp_cfg
    }
    #[doc = "0x40 - zero det start configure register"]
    #[inline(always)]
    pub const fn start(&self) -> &START {
        &self.start
    }
    #[doc = "0x3fc - zero det reg change date"]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
#[doc = "CONF (rw) register accessor: zero det cfg reg\n\nYou can [`read`](crate::Reg::read) this register and get [`conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@conf`] module"]
pub type CONF = crate::Reg<conf::CONF_SPEC>;
#[doc = "zero det cfg reg"]
pub mod conf;
#[doc = "FILTER_CNT (rw) register accessor: protect time reg\n\nYou can [`read`](crate::Reg::read) this register and get [`filter_cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`filter_cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@filter_cnt`] module"]
pub type FILTER_CNT = crate::Reg<filter_cnt::FILTER_CNT_SPEC>;
#[doc = "protect time reg"]
pub mod filter_cnt;
#[doc = "POLL_PERIOD (rw) register accessor: poll period time reg\n\nYou can [`read`](crate::Reg::read) this register and get [`poll_period::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`poll_period::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@poll_period`] module"]
pub type POLL_PERIOD = crate::Reg<poll_period::POLL_PERIOD_SPEC>;
#[doc = "poll period time reg"]
pub mod poll_period;
#[doc = "DELAY_EVENT_TIME (rw) register accessor: delay time reg\n\nYou can [`read`](crate::Reg::read) this register and get [`delay_event_time::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`delay_event_time::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@delay_event_time`] module"]
pub type DELAY_EVENT_TIME = crate::Reg<delay_event_time::DELAY_EVENT_TIME_SPEC>;
#[doc = "delay time reg"]
pub mod delay_event_time;
#[doc = "INT_ENA (rw) register accessor: zero det int ena\n\nYou can [`read`](crate::Reg::read) this register and get [`int_ena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_ena`] module"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = "zero det int ena"]
pub mod int_ena;
#[doc = "INT_RAW (rw) register accessor: zero det int raw\n\nYou can [`read`](crate::Reg::read) this register and get [`int_raw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_raw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_raw`] module"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = "zero det int raw"]
pub mod int_raw;
#[doc = "INT_CLR (w) register accessor: zero det int clr\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_clr`] module"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = "zero det int clr"]
pub mod int_clr;
#[doc = "INT_ST (r) register accessor: zero det int st\n\nYou can [`read`](crate::Reg::read) this register and get [`int_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_st`] module"]
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
#[doc = "zero det int st"]
pub mod int_st;
#[doc = "CHANNEL_1_TIMER0 (r) register accessor: record timer reg\n\nYou can [`read`](crate::Reg::read) this register and get [`channel_1_timer0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@channel_1_timer0`] module"]
pub type CHANNEL_1_TIMER0 = crate::Reg<channel_1_timer0::CHANNEL_1_TIMER0_SPEC>;
#[doc = "record timer reg"]
pub mod channel_1_timer0;
#[doc = "CHANNEL_1_TIMER1 (r) register accessor: record timer reg\n\nYou can [`read`](crate::Reg::read) this register and get [`channel_1_timer1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@channel_1_timer1`] module"]
pub type CHANNEL_1_TIMER1 = crate::Reg<channel_1_timer1::CHANNEL_1_TIMER1_SPEC>;
#[doc = "record timer reg"]
pub mod channel_1_timer1;
#[doc = "CHANNEL_2_TIMER0 (r) register accessor: record timer reg\n\nYou can [`read`](crate::Reg::read) this register and get [`channel_2_timer0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@channel_2_timer0`] module"]
pub type CHANNEL_2_TIMER0 = crate::Reg<channel_2_timer0::CHANNEL_2_TIMER0_SPEC>;
#[doc = "record timer reg"]
pub mod channel_2_timer0;
#[doc = "CHANNEL_2_TIMER1 (r) register accessor: record timer reg\n\nYou can [`read`](crate::Reg::read) this register and get [`channel_2_timer1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@channel_2_timer1`] module"]
pub type CHANNEL_2_TIMER1 = crate::Reg<channel_2_timer1::CHANNEL_2_TIMER1_SPEC>;
#[doc = "record timer reg"]
pub mod channel_2_timer1;
#[doc = "CHANNEL_3_TIMER0 (r) register accessor: record timer reg\n\nYou can [`read`](crate::Reg::read) this register and get [`channel_3_timer0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@channel_3_timer0`] module"]
pub type CHANNEL_3_TIMER0 = crate::Reg<channel_3_timer0::CHANNEL_3_TIMER0_SPEC>;
#[doc = "record timer reg"]
pub mod channel_3_timer0;
#[doc = "CHANNEL_3_TIMER1 (r) register accessor: record timer reg\n\nYou can [`read`](crate::Reg::read) this register and get [`channel_3_timer1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@channel_3_timer1`] module"]
pub type CHANNEL_3_TIMER1 = crate::Reg<channel_3_timer1::CHANNEL_3_TIMER1_SPEC>;
#[doc = "record timer reg"]
pub mod channel_3_timer1;
#[doc = "CHANNEL_STATUS (r) register accessor: pad comp status reg\n\nYou can [`read`](crate::Reg::read) this register and get [`channel_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@channel_status`] module"]
pub type CHANNEL_STATUS = crate::Reg<channel_status::CHANNEL_STATUS_SPEC>;
#[doc = "pad comp status reg"]
pub mod channel_status;
#[doc = "PAD_COMP_CFG (rw) register accessor: pad comp cfg reg\n\nYou can [`read`](crate::Reg::read) this register and get [`pad_comp_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad_comp_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad_comp_cfg`] module"]
pub type PAD_COMP_CFG = crate::Reg<pad_comp_cfg::PAD_COMP_CFG_SPEC>;
#[doc = "pad comp cfg reg"]
pub mod pad_comp_cfg;
#[doc = "START (w) register accessor: zero det start configure register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`start::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@start`] module"]
pub type START = crate::Reg<start::START_SPEC>;
#[doc = "zero det start configure register"]
pub mod start;
#[doc = "DATE (rw) register accessor: zero det reg change date\n\nYou can [`read`](crate::Reg::read) this register and get [`date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "zero det reg change date"]
pub mod date;
