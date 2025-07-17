#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    ch: [CH; 6],
    _reserved1: [u8; 0x28],
    timer: [TIMER; 4],
    int_raw: INT_RAW,
    int_st: INT_ST,
    int_ena: INT_ENA,
    int_clr: INT_CLR,
    _reserved6: [u8; 0x30],
    ch_gamma_conf: [CH_GAMMA_CONF; 6],
    _reserved7: [u8; 0x08],
    evt_task_en0: EVT_TASK_EN0,
    evt_task_en1: EVT_TASK_EN1,
    evt_task_en2: EVT_TASK_EN2,
    _reserved10: [u8; 0x14],
    timer_cmp: [TIMER_CMP; 4],
    timer_cnt_cap: [TIMER_CNT_CAP; 4],
    _reserved12: [u8; 0x10],
    conf: CONF,
    date: DATE,
}
impl RegisterBlock {
    #[doc = "0x00..0x78 - Cluster CH%s, containing CH?_CONF0, CH?_HPOINT, CH?_DUTY, CH?_CONF1, CH?_DUTY_R"]
    #[inline(always)]
    pub const fn ch(&self, n: usize) -> &CH {
        &self.ch[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x78 - Cluster CH%s, containing CH?_CONF0, CH?_HPOINT, CH?_DUTY, CH?_CONF1, CH?_DUTY_R"]
    #[inline(always)]
    pub fn ch_iter(&self) -> impl Iterator<Item = &CH> {
        self.ch.iter()
    }
    #[doc = "0xa0..0xc0 - Cluster TIMER%s, containing TIMER?_CONF, TIMER?_VALUE"]
    #[inline(always)]
    pub const fn timer(&self, n: usize) -> &TIMER {
        &self.timer[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xa0..0xc0 - Cluster TIMER%s, containing TIMER?_CONF, TIMER?_VALUE"]
    #[inline(always)]
    pub fn timer_iter(&self) -> impl Iterator<Item = &TIMER> {
        self.timer.iter()
    }
    #[doc = "0xc0 - Interrupt raw status register"]
    #[inline(always)]
    pub const fn int_raw(&self) -> &INT_RAW {
        &self.int_raw
    }
    #[doc = "0xc4 - Interrupt masked status register"]
    #[inline(always)]
    pub const fn int_st(&self) -> &INT_ST {
        &self.int_st
    }
    #[doc = "0xc8 - Interrupt enable register"]
    #[inline(always)]
    pub const fn int_ena(&self) -> &INT_ENA {
        &self.int_ena
    }
    #[doc = "0xcc - Interrupt clear register"]
    #[inline(always)]
    pub const fn int_clr(&self) -> &INT_CLR {
        &self.int_clr
    }
    #[doc = "0x100..0x118 - Ledc ch%s gamma config register."]
    #[inline(always)]
    pub const fn ch_gamma_conf(&self, n: usize) -> &CH_GAMMA_CONF {
        &self.ch_gamma_conf[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x100..0x118 - Ledc ch%s gamma config register."]
    #[inline(always)]
    pub fn ch_gamma_conf_iter(&self) -> impl Iterator<Item = &CH_GAMMA_CONF> {
        self.ch_gamma_conf.iter()
    }
    #[doc = "0x100 - Ledc ch0 gamma config register."]
    #[inline(always)]
    pub const fn ch0_gamma_conf(&self) -> &CH_GAMMA_CONF {
        self.ch_gamma_conf(0)
    }
    #[doc = "0x104 - Ledc ch1 gamma config register."]
    #[inline(always)]
    pub const fn ch1_gamma_conf(&self) -> &CH_GAMMA_CONF {
        self.ch_gamma_conf(1)
    }
    #[doc = "0x108 - Ledc ch2 gamma config register."]
    #[inline(always)]
    pub const fn ch2_gamma_conf(&self) -> &CH_GAMMA_CONF {
        self.ch_gamma_conf(2)
    }
    #[doc = "0x10c - Ledc ch3 gamma config register."]
    #[inline(always)]
    pub const fn ch3_gamma_conf(&self) -> &CH_GAMMA_CONF {
        self.ch_gamma_conf(3)
    }
    #[doc = "0x110 - Ledc ch4 gamma config register."]
    #[inline(always)]
    pub const fn ch4_gamma_conf(&self) -> &CH_GAMMA_CONF {
        self.ch_gamma_conf(4)
    }
    #[doc = "0x114 - Ledc ch5 gamma config register."]
    #[inline(always)]
    pub const fn ch5_gamma_conf(&self) -> &CH_GAMMA_CONF {
        self.ch_gamma_conf(5)
    }
    #[doc = "0x120 - Ledc event task enable bit register0."]
    #[inline(always)]
    pub const fn evt_task_en0(&self) -> &EVT_TASK_EN0 {
        &self.evt_task_en0
    }
    #[doc = "0x124 - Ledc event task enable bit register1."]
    #[inline(always)]
    pub const fn evt_task_en1(&self) -> &EVT_TASK_EN1 {
        &self.evt_task_en1
    }
    #[doc = "0x128 - Ledc event task enable bit register2."]
    #[inline(always)]
    pub const fn evt_task_en2(&self) -> &EVT_TASK_EN2 {
        &self.evt_task_en2
    }
    #[doc = "0x140..0x150 - Ledc timer%s compare value register."]
    #[inline(always)]
    pub const fn timer_cmp(&self, n: usize) -> &TIMER_CMP {
        &self.timer_cmp[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x140..0x150 - Ledc timer%s compare value register."]
    #[inline(always)]
    pub fn timer_cmp_iter(&self) -> impl Iterator<Item = &TIMER_CMP> {
        self.timer_cmp.iter()
    }
    #[doc = "0x140 - Ledc timer0 compare value register."]
    #[inline(always)]
    pub const fn timer0_cmp(&self) -> &TIMER_CMP {
        self.timer_cmp(0)
    }
    #[doc = "0x144 - Ledc timer1 compare value register."]
    #[inline(always)]
    pub const fn timer1_cmp(&self) -> &TIMER_CMP {
        self.timer_cmp(1)
    }
    #[doc = "0x148 - Ledc timer2 compare value register."]
    #[inline(always)]
    pub const fn timer2_cmp(&self) -> &TIMER_CMP {
        self.timer_cmp(2)
    }
    #[doc = "0x14c - Ledc timer3 compare value register."]
    #[inline(always)]
    pub const fn timer3_cmp(&self) -> &TIMER_CMP {
        self.timer_cmp(3)
    }
    #[doc = "0x150..0x160 - Ledc timer%s captured count value register."]
    #[inline(always)]
    pub const fn timer_cnt_cap(&self, n: usize) -> &TIMER_CNT_CAP {
        &self.timer_cnt_cap[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x150..0x160 - Ledc timer%s captured count value register."]
    #[inline(always)]
    pub fn timer_cnt_cap_iter(&self) -> impl Iterator<Item = &TIMER_CNT_CAP> {
        self.timer_cnt_cap.iter()
    }
    #[doc = "0x150 - Ledc timer0 captured count value register."]
    #[inline(always)]
    pub const fn timer0_cnt_cap(&self) -> &TIMER_CNT_CAP {
        self.timer_cnt_cap(0)
    }
    #[doc = "0x154 - Ledc timer1 captured count value register."]
    #[inline(always)]
    pub const fn timer1_cnt_cap(&self) -> &TIMER_CNT_CAP {
        self.timer_cnt_cap(1)
    }
    #[doc = "0x158 - Ledc timer2 captured count value register."]
    #[inline(always)]
    pub const fn timer2_cnt_cap(&self) -> &TIMER_CNT_CAP {
        self.timer_cnt_cap(2)
    }
    #[doc = "0x15c - Ledc timer3 captured count value register."]
    #[inline(always)]
    pub const fn timer3_cnt_cap(&self) -> &TIMER_CNT_CAP {
        self.timer_cnt_cap(3)
    }
    #[doc = "0x170 - LEDC global configuration register"]
    #[inline(always)]
    pub const fn conf(&self) -> &CONF {
        &self.conf
    }
    #[doc = "0x174 - Version control register"]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
#[doc = "Cluster CH%s, containing CH?_CONF0, CH?_HPOINT, CH?_DUTY, CH?_CONF1, CH?_DUTY_R"]
pub use self::ch::CH;
#[doc = r"Cluster"]
#[doc = "Cluster CH%s, containing CH?_CONF0, CH?_HPOINT, CH?_DUTY, CH?_CONF1, CH?_DUTY_R"]
pub mod ch;
#[doc = "Cluster TIMER%s, containing TIMER?_CONF, TIMER?_VALUE"]
pub use self::timer::TIMER;
#[doc = r"Cluster"]
#[doc = "Cluster TIMER%s, containing TIMER?_CONF, TIMER?_VALUE"]
pub mod timer;
#[doc = "INT_RAW (rw) register accessor: Interrupt raw status register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_raw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_raw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_raw`] module"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = "Interrupt raw status register"]
pub mod int_raw;
#[doc = "INT_ST (r) register accessor: Interrupt masked status register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_st`] module"]
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
#[doc = "Interrupt masked status register"]
pub mod int_st;
#[doc = "INT_ENA (rw) register accessor: Interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_ena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_ena`] module"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = "Interrupt enable register"]
pub mod int_ena;
#[doc = "INT_CLR (w) register accessor: Interrupt clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_clr`] module"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = "Interrupt clear register"]
pub mod int_clr;
#[doc = "CH_GAMMA_CONF (rw) register accessor: Ledc ch%s gamma config register.\n\nYou can [`read`](crate::Reg::read) this register and get [`ch_gamma_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch_gamma_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch_gamma_conf`] module"]
pub type CH_GAMMA_CONF = crate::Reg<ch_gamma_conf::CH_GAMMA_CONF_SPEC>;
#[doc = "Ledc ch%s gamma config register."]
pub mod ch_gamma_conf;
#[doc = "EVT_TASK_EN0 (rw) register accessor: Ledc event task enable bit register0.\n\nYou can [`read`](crate::Reg::read) this register and get [`evt_task_en0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evt_task_en0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evt_task_en0`] module"]
pub type EVT_TASK_EN0 = crate::Reg<evt_task_en0::EVT_TASK_EN0_SPEC>;
#[doc = "Ledc event task enable bit register0."]
pub mod evt_task_en0;
#[doc = "EVT_TASK_EN1 (rw) register accessor: Ledc event task enable bit register1.\n\nYou can [`read`](crate::Reg::read) this register and get [`evt_task_en1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evt_task_en1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evt_task_en1`] module"]
pub type EVT_TASK_EN1 = crate::Reg<evt_task_en1::EVT_TASK_EN1_SPEC>;
#[doc = "Ledc event task enable bit register1."]
pub mod evt_task_en1;
#[doc = "EVT_TASK_EN2 (rw) register accessor: Ledc event task enable bit register2.\n\nYou can [`read`](crate::Reg::read) this register and get [`evt_task_en2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evt_task_en2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evt_task_en2`] module"]
pub type EVT_TASK_EN2 = crate::Reg<evt_task_en2::EVT_TASK_EN2_SPEC>;
#[doc = "Ledc event task enable bit register2."]
pub mod evt_task_en2;
#[doc = "TIMER_CMP (rw) register accessor: Ledc timer%s compare value register.\n\nYou can [`read`](crate::Reg::read) this register and get [`timer_cmp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer_cmp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer_cmp`] module"]
pub type TIMER_CMP = crate::Reg<timer_cmp::TIMER_CMP_SPEC>;
#[doc = "Ledc timer%s compare value register."]
pub mod timer_cmp;
#[doc = "TIMER_CNT_CAP (r) register accessor: Ledc timer%s captured count value register.\n\nYou can [`read`](crate::Reg::read) this register and get [`timer_cnt_cap::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer_cnt_cap`] module"]
pub type TIMER_CNT_CAP = crate::Reg<timer_cnt_cap::TIMER_CNT_CAP_SPEC>;
#[doc = "Ledc timer%s captured count value register."]
pub mod timer_cnt_cap;
#[doc = "CONF (rw) register accessor: LEDC global configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@conf`] module"]
pub type CONF = crate::Reg<conf::CONF_SPEC>;
#[doc = "LEDC global configuration register"]
pub mod conf;
pub use crate::dma::date;
pub use crate::dma::DATE;
