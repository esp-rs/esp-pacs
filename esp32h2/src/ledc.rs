#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
///Register block
pub struct RegisterBlock {
    ch: [CH; 6],
    _reserved1: [u8; 0x28],
    timer: [TIMER; 4],
    int_raw: INT_RAW,
    int_st: INT_ST,
    int_ena: INT_ENA,
    int_clr: INT_CLR,
    _reserved6: [u8; 0x30],
    ch_gamma_wr: (),
    _reserved7: [u8; 0x04],
    ch_gamma_wr_addr: (),
    _reserved8: [u8; 0x04],
    ch_gamma_rd_addr: (),
    _reserved9: [u8; 0x04],
    ch_gamma_rd_data: (),
    _reserved10: [u8; 0x74],
    ch_gamma_conf: [CH_GAMMA_CONF; 6],
    _reserved11: [u8; 0x08],
    evt_task_en0: EVT_TASK_EN0,
    evt_task_en1: EVT_TASK_EN1,
    evt_task_en2: EVT_TASK_EN2,
    _reserved14: [u8; 0x04],
    timer_cmp: [TIMER_CMP; 4],
    timer_cnt_cap: [TIMER_CNT_CAP; 4],
    _reserved16: [u8; 0x20],
    conf: CONF,
    _reserved17: [u8; 0x08],
    date: DATE,
}
impl RegisterBlock {
    ///0x00..0x78 - Cluster CH%s, containing CH?_CONF0, CH?_HPOINT, CH?_DUTY, CH?_CONF1, CH?_DUTY_R
    #[inline(always)]
    pub const fn ch(&self, n: usize) -> &CH {
        &self.ch[n]
    }
    ///Iterator for array of:
    ///0x00..0x78 - Cluster CH%s, containing CH?_CONF0, CH?_HPOINT, CH?_DUTY, CH?_CONF1, CH?_DUTY_R
    #[inline(always)]
    pub fn ch_iter(&self) -> impl Iterator<Item = &CH> {
        self.ch.iter()
    }
    ///0xa0..0xc0 - Cluster TIMER%s, containing TIMER?_CONF, TIMER?_VALUE
    #[inline(always)]
    pub const fn timer(&self, n: usize) -> &TIMER {
        &self.timer[n]
    }
    ///Iterator for array of:
    ///0xa0..0xc0 - Cluster TIMER%s, containing TIMER?_CONF, TIMER?_VALUE
    #[inline(always)]
    pub fn timer_iter(&self) -> impl Iterator<Item = &TIMER> {
        self.timer.iter()
    }
    ///0xc0 - Raw interrupt status
    #[inline(always)]
    pub const fn int_raw(&self) -> &INT_RAW {
        &self.int_raw
    }
    ///0xc4 - Masked interrupt status
    #[inline(always)]
    pub const fn int_st(&self) -> &INT_ST {
        &self.int_st
    }
    ///0xc8 - Interrupt enable bits
    #[inline(always)]
    pub const fn int_ena(&self) -> &INT_ENA {
        &self.int_ena
    }
    ///0xcc - Interrupt clear bits
    #[inline(always)]
    pub const fn int_clr(&self) -> &INT_CLR {
        &self.int_clr
    }
    ///0x100..0x118 - Ledc ch%s gamma ram write register.
    #[inline(always)]
    pub const fn ch_gamma_wr(&self, n: usize) -> &CH_GAMMA_WR {
        #[allow(clippy::no_effect)]
        [(); 6][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(256)
                .add(16 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0x100..0x118 - Ledc ch%s gamma ram write register.
    #[inline(always)]
    pub fn ch_gamma_wr_iter(&self) -> impl Iterator<Item = &CH_GAMMA_WR> {
        (0..6).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(256)
                .add(16 * n)
                .cast()
        })
    }
    ///0x100 - Ledc ch0 gamma ram write register.
    #[inline(always)]
    pub const fn ch0_gamma_wr(&self) -> &CH_GAMMA_WR {
        self.ch_gamma_wr(0)
    }
    ///0x110 - Ledc ch1 gamma ram write register.
    #[inline(always)]
    pub const fn ch1_gamma_wr(&self) -> &CH_GAMMA_WR {
        self.ch_gamma_wr(1)
    }
    ///0x120 - Ledc ch2 gamma ram write register.
    #[inline(always)]
    pub const fn ch2_gamma_wr(&self) -> &CH_GAMMA_WR {
        self.ch_gamma_wr(2)
    }
    ///0x130 - Ledc ch3 gamma ram write register.
    #[inline(always)]
    pub const fn ch3_gamma_wr(&self) -> &CH_GAMMA_WR {
        self.ch_gamma_wr(3)
    }
    ///0x140 - Ledc ch4 gamma ram write register.
    #[inline(always)]
    pub const fn ch4_gamma_wr(&self) -> &CH_GAMMA_WR {
        self.ch_gamma_wr(4)
    }
    ///0x150 - Ledc ch5 gamma ram write register.
    #[inline(always)]
    pub const fn ch5_gamma_wr(&self) -> &CH_GAMMA_WR {
        self.ch_gamma_wr(5)
    }
    ///0x104..0x11c - Ledc ch%s gamma ram write address register.
    #[inline(always)]
    pub const fn ch_gamma_wr_addr(&self, n: usize) -> &CH_GAMMA_WR_ADDR {
        #[allow(clippy::no_effect)]
        [(); 6][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(260)
                .add(16 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0x104..0x11c - Ledc ch%s gamma ram write address register.
    #[inline(always)]
    pub fn ch_gamma_wr_addr_iter(&self) -> impl Iterator<Item = &CH_GAMMA_WR_ADDR> {
        (0..6).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(260)
                .add(16 * n)
                .cast()
        })
    }
    ///0x104 - Ledc ch0 gamma ram write address register.
    #[inline(always)]
    pub const fn ch0_gamma_wr_addr(&self) -> &CH_GAMMA_WR_ADDR {
        self.ch_gamma_wr_addr(0)
    }
    ///0x114 - Ledc ch1 gamma ram write address register.
    #[inline(always)]
    pub const fn ch1_gamma_wr_addr(&self) -> &CH_GAMMA_WR_ADDR {
        self.ch_gamma_wr_addr(1)
    }
    ///0x124 - Ledc ch2 gamma ram write address register.
    #[inline(always)]
    pub const fn ch2_gamma_wr_addr(&self) -> &CH_GAMMA_WR_ADDR {
        self.ch_gamma_wr_addr(2)
    }
    ///0x134 - Ledc ch3 gamma ram write address register.
    #[inline(always)]
    pub const fn ch3_gamma_wr_addr(&self) -> &CH_GAMMA_WR_ADDR {
        self.ch_gamma_wr_addr(3)
    }
    ///0x144 - Ledc ch4 gamma ram write address register.
    #[inline(always)]
    pub const fn ch4_gamma_wr_addr(&self) -> &CH_GAMMA_WR_ADDR {
        self.ch_gamma_wr_addr(4)
    }
    ///0x154 - Ledc ch5 gamma ram write address register.
    #[inline(always)]
    pub const fn ch5_gamma_wr_addr(&self) -> &CH_GAMMA_WR_ADDR {
        self.ch_gamma_wr_addr(5)
    }
    ///0x108..0x120 - Ledc ch%s gamma ram read address register.
    #[inline(always)]
    pub const fn ch_gamma_rd_addr(&self, n: usize) -> &CH_GAMMA_RD_ADDR {
        #[allow(clippy::no_effect)]
        [(); 6][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(264)
                .add(16 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0x108..0x120 - Ledc ch%s gamma ram read address register.
    #[inline(always)]
    pub fn ch_gamma_rd_addr_iter(&self) -> impl Iterator<Item = &CH_GAMMA_RD_ADDR> {
        (0..6).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(264)
                .add(16 * n)
                .cast()
        })
    }
    ///0x108 - Ledc ch0 gamma ram read address register.
    #[inline(always)]
    pub const fn ch0_gamma_rd_addr(&self) -> &CH_GAMMA_RD_ADDR {
        self.ch_gamma_rd_addr(0)
    }
    ///0x118 - Ledc ch1 gamma ram read address register.
    #[inline(always)]
    pub const fn ch1_gamma_rd_addr(&self) -> &CH_GAMMA_RD_ADDR {
        self.ch_gamma_rd_addr(1)
    }
    ///0x128 - Ledc ch2 gamma ram read address register.
    #[inline(always)]
    pub const fn ch2_gamma_rd_addr(&self) -> &CH_GAMMA_RD_ADDR {
        self.ch_gamma_rd_addr(2)
    }
    ///0x138 - Ledc ch3 gamma ram read address register.
    #[inline(always)]
    pub const fn ch3_gamma_rd_addr(&self) -> &CH_GAMMA_RD_ADDR {
        self.ch_gamma_rd_addr(3)
    }
    ///0x148 - Ledc ch4 gamma ram read address register.
    #[inline(always)]
    pub const fn ch4_gamma_rd_addr(&self) -> &CH_GAMMA_RD_ADDR {
        self.ch_gamma_rd_addr(4)
    }
    ///0x158 - Ledc ch5 gamma ram read address register.
    #[inline(always)]
    pub const fn ch5_gamma_rd_addr(&self) -> &CH_GAMMA_RD_ADDR {
        self.ch_gamma_rd_addr(5)
    }
    ///0x10c..0x124 - Ledc ch%s gamma ram read data register.
    #[inline(always)]
    pub const fn ch_gamma_rd_data(&self, n: usize) -> &CH_GAMMA_RD_DATA {
        #[allow(clippy::no_effect)]
        [(); 6][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(268)
                .add(16 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0x10c..0x124 - Ledc ch%s gamma ram read data register.
    #[inline(always)]
    pub fn ch_gamma_rd_data_iter(&self) -> impl Iterator<Item = &CH_GAMMA_RD_DATA> {
        (0..6).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(268)
                .add(16 * n)
                .cast()
        })
    }
    ///0x10c - Ledc ch0 gamma ram read data register.
    #[inline(always)]
    pub const fn ch0_gamma_rd_data(&self) -> &CH_GAMMA_RD_DATA {
        self.ch_gamma_rd_data(0)
    }
    ///0x11c - Ledc ch1 gamma ram read data register.
    #[inline(always)]
    pub const fn ch1_gamma_rd_data(&self) -> &CH_GAMMA_RD_DATA {
        self.ch_gamma_rd_data(1)
    }
    ///0x12c - Ledc ch2 gamma ram read data register.
    #[inline(always)]
    pub const fn ch2_gamma_rd_data(&self) -> &CH_GAMMA_RD_DATA {
        self.ch_gamma_rd_data(2)
    }
    ///0x13c - Ledc ch3 gamma ram read data register.
    #[inline(always)]
    pub const fn ch3_gamma_rd_data(&self) -> &CH_GAMMA_RD_DATA {
        self.ch_gamma_rd_data(3)
    }
    ///0x14c - Ledc ch4 gamma ram read data register.
    #[inline(always)]
    pub const fn ch4_gamma_rd_data(&self) -> &CH_GAMMA_RD_DATA {
        self.ch_gamma_rd_data(4)
    }
    ///0x15c - Ledc ch5 gamma ram read data register.
    #[inline(always)]
    pub const fn ch5_gamma_rd_data(&self) -> &CH_GAMMA_RD_DATA {
        self.ch_gamma_rd_data(5)
    }
    ///0x180..0x198 - Ledc ch%s gamma config register.
    #[inline(always)]
    pub const fn ch_gamma_conf(&self, n: usize) -> &CH_GAMMA_CONF {
        &self.ch_gamma_conf[n]
    }
    ///Iterator for array of:
    ///0x180..0x198 - Ledc ch%s gamma config register.
    #[inline(always)]
    pub fn ch_gamma_conf_iter(&self) -> impl Iterator<Item = &CH_GAMMA_CONF> {
        self.ch_gamma_conf.iter()
    }
    ///0x180 - Ledc ch0 gamma config register.
    #[inline(always)]
    pub const fn ch0_gamma_conf(&self) -> &CH_GAMMA_CONF {
        self.ch_gamma_conf(0)
    }
    ///0x184 - Ledc ch1 gamma config register.
    #[inline(always)]
    pub const fn ch1_gamma_conf(&self) -> &CH_GAMMA_CONF {
        self.ch_gamma_conf(1)
    }
    ///0x188 - Ledc ch2 gamma config register.
    #[inline(always)]
    pub const fn ch2_gamma_conf(&self) -> &CH_GAMMA_CONF {
        self.ch_gamma_conf(2)
    }
    ///0x18c - Ledc ch3 gamma config register.
    #[inline(always)]
    pub const fn ch3_gamma_conf(&self) -> &CH_GAMMA_CONF {
        self.ch_gamma_conf(3)
    }
    ///0x190 - Ledc ch4 gamma config register.
    #[inline(always)]
    pub const fn ch4_gamma_conf(&self) -> &CH_GAMMA_CONF {
        self.ch_gamma_conf(4)
    }
    ///0x194 - Ledc ch5 gamma config register.
    #[inline(always)]
    pub const fn ch5_gamma_conf(&self) -> &CH_GAMMA_CONF {
        self.ch_gamma_conf(5)
    }
    ///0x1a0 - Ledc event task enable bit register0.
    #[inline(always)]
    pub const fn evt_task_en0(&self) -> &EVT_TASK_EN0 {
        &self.evt_task_en0
    }
    ///0x1a4 - Ledc event task enable bit register1.
    #[inline(always)]
    pub const fn evt_task_en1(&self) -> &EVT_TASK_EN1 {
        &self.evt_task_en1
    }
    ///0x1a8 - Ledc event task enable bit register2.
    #[inline(always)]
    pub const fn evt_task_en2(&self) -> &EVT_TASK_EN2 {
        &self.evt_task_en2
    }
    ///0x1b0..0x1c0 - Ledc timer%s compare value register.
    #[inline(always)]
    pub const fn timer_cmp(&self, n: usize) -> &TIMER_CMP {
        &self.timer_cmp[n]
    }
    ///Iterator for array of:
    ///0x1b0..0x1c0 - Ledc timer%s compare value register.
    #[inline(always)]
    pub fn timer_cmp_iter(&self) -> impl Iterator<Item = &TIMER_CMP> {
        self.timer_cmp.iter()
    }
    ///0x1b0 - Ledc timer0 compare value register.
    #[inline(always)]
    pub const fn timer0_cmp(&self) -> &TIMER_CMP {
        self.timer_cmp(0)
    }
    ///0x1b4 - Ledc timer1 compare value register.
    #[inline(always)]
    pub const fn timer1_cmp(&self) -> &TIMER_CMP {
        self.timer_cmp(1)
    }
    ///0x1b8 - Ledc timer2 compare value register.
    #[inline(always)]
    pub const fn timer2_cmp(&self) -> &TIMER_CMP {
        self.timer_cmp(2)
    }
    ///0x1bc - Ledc timer3 compare value register.
    #[inline(always)]
    pub const fn timer3_cmp(&self) -> &TIMER_CMP {
        self.timer_cmp(3)
    }
    ///0x1c0..0x1d0 - Ledc timer%s count value capture register.
    #[inline(always)]
    pub const fn timer_cnt_cap(&self, n: usize) -> &TIMER_CNT_CAP {
        &self.timer_cnt_cap[n]
    }
    ///Iterator for array of:
    ///0x1c0..0x1d0 - Ledc timer%s count value capture register.
    #[inline(always)]
    pub fn timer_cnt_cap_iter(&self) -> impl Iterator<Item = &TIMER_CNT_CAP> {
        self.timer_cnt_cap.iter()
    }
    ///0x1c0 - Ledc timer0 count value capture register.
    #[inline(always)]
    pub const fn timer0_cnt_cap(&self) -> &TIMER_CNT_CAP {
        self.timer_cnt_cap(0)
    }
    ///0x1c4 - Ledc timer1 count value capture register.
    #[inline(always)]
    pub const fn timer1_cnt_cap(&self) -> &TIMER_CNT_CAP {
        self.timer_cnt_cap(1)
    }
    ///0x1c8 - Ledc timer2 count value capture register.
    #[inline(always)]
    pub const fn timer2_cnt_cap(&self) -> &TIMER_CNT_CAP {
        self.timer_cnt_cap(2)
    }
    ///0x1cc - Ledc timer3 count value capture register.
    #[inline(always)]
    pub const fn timer3_cnt_cap(&self) -> &TIMER_CNT_CAP {
        self.timer_cnt_cap(3)
    }
    ///0x1f0 - Global ledc configuration register
    #[inline(always)]
    pub const fn conf(&self) -> &CONF {
        &self.conf
    }
    ///0x1fc - Version control register
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
///Cluster CH%s, containing CH?_CONF0, CH?_HPOINT, CH?_DUTY, CH?_CONF1, CH?_DUTY_R
pub use self::ch::CH;
///Cluster
///Cluster CH%s, containing CH?_CONF0, CH?_HPOINT, CH?_DUTY, CH?_CONF1, CH?_DUTY_R
pub mod ch;
///Cluster TIMER%s, containing TIMER?_CONF, TIMER?_VALUE
pub use self::timer::TIMER;
///Cluster
///Cluster TIMER%s, containing TIMER?_CONF, TIMER?_VALUE
pub mod timer;
/**INT_RAW (rw) register accessor: Raw interrupt status

You can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_raw::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_raw`] module*/
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
///Raw interrupt status
pub mod int_raw;
/**INT_ST (r) register accessor: Masked interrupt status

You can [`read`](crate::generic::Reg::read) this register and get [`int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_st`] module*/
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
///Masked interrupt status
pub mod int_st;
/**INT_ENA (rw) register accessor: Interrupt enable bits

You can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_ena`] module*/
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
///Interrupt enable bits
pub mod int_ena;
/**INT_CLR (w) register accessor: Interrupt clear bits

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_clr`] module*/
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
///Interrupt clear bits
pub mod int_clr;
/**CH_GAMMA_WR (rw) register accessor: Ledc ch%s gamma ram write register.

You can [`read`](crate::generic::Reg::read) this register and get [`ch_gamma_wr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch_gamma_wr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ch_gamma_wr`] module*/
pub type CH_GAMMA_WR = crate::Reg<ch_gamma_wr::CH_GAMMA_WR_SPEC>;
///Ledc ch%s gamma ram write register.
pub mod ch_gamma_wr;
/**CH_GAMMA_WR_ADDR (rw) register accessor: Ledc ch%s gamma ram write address register.

You can [`read`](crate::generic::Reg::read) this register and get [`ch_gamma_wr_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch_gamma_wr_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ch_gamma_wr_addr`] module*/
pub type CH_GAMMA_WR_ADDR = crate::Reg<ch_gamma_wr_addr::CH_GAMMA_WR_ADDR_SPEC>;
///Ledc ch%s gamma ram write address register.
pub mod ch_gamma_wr_addr;
/**CH_GAMMA_RD_ADDR (rw) register accessor: Ledc ch%s gamma ram read address register.

You can [`read`](crate::generic::Reg::read) this register and get [`ch_gamma_rd_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch_gamma_rd_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ch_gamma_rd_addr`] module*/
pub type CH_GAMMA_RD_ADDR = crate::Reg<ch_gamma_rd_addr::CH_GAMMA_RD_ADDR_SPEC>;
///Ledc ch%s gamma ram read address register.
pub mod ch_gamma_rd_addr;
/**CH_GAMMA_RD_DATA (r) register accessor: Ledc ch%s gamma ram read data register.

You can [`read`](crate::generic::Reg::read) this register and get [`ch_gamma_rd_data::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ch_gamma_rd_data`] module*/
pub type CH_GAMMA_RD_DATA = crate::Reg<ch_gamma_rd_data::CH_GAMMA_RD_DATA_SPEC>;
///Ledc ch%s gamma ram read data register.
pub mod ch_gamma_rd_data;
/**CH_GAMMA_CONF (rw) register accessor: Ledc ch%s gamma config register.

You can [`read`](crate::generic::Reg::read) this register and get [`ch_gamma_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch_gamma_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ch_gamma_conf`] module*/
pub type CH_GAMMA_CONF = crate::Reg<ch_gamma_conf::CH_GAMMA_CONF_SPEC>;
///Ledc ch%s gamma config register.
pub mod ch_gamma_conf;
/**EVT_TASK_EN0 (rw) register accessor: Ledc event task enable bit register0.

You can [`read`](crate::generic::Reg::read) this register and get [`evt_task_en0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evt_task_en0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@evt_task_en0`] module*/
pub type EVT_TASK_EN0 = crate::Reg<evt_task_en0::EVT_TASK_EN0_SPEC>;
///Ledc event task enable bit register0.
pub mod evt_task_en0;
/**EVT_TASK_EN1 (rw) register accessor: Ledc event task enable bit register1.

You can [`read`](crate::generic::Reg::read) this register and get [`evt_task_en1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evt_task_en1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@evt_task_en1`] module*/
pub type EVT_TASK_EN1 = crate::Reg<evt_task_en1::EVT_TASK_EN1_SPEC>;
///Ledc event task enable bit register1.
pub mod evt_task_en1;
/**EVT_TASK_EN2 (rw) register accessor: Ledc event task enable bit register2.

You can [`read`](crate::generic::Reg::read) this register and get [`evt_task_en2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evt_task_en2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@evt_task_en2`] module*/
pub type EVT_TASK_EN2 = crate::Reg<evt_task_en2::EVT_TASK_EN2_SPEC>;
///Ledc event task enable bit register2.
pub mod evt_task_en2;
/**TIMER_CMP (rw) register accessor: Ledc timer%s compare value register.

You can [`read`](crate::generic::Reg::read) this register and get [`timer_cmp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer_cmp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@timer_cmp`] module*/
pub type TIMER_CMP = crate::Reg<timer_cmp::TIMER_CMP_SPEC>;
///Ledc timer%s compare value register.
pub mod timer_cmp;
/**TIMER_CNT_CAP (r) register accessor: Ledc timer%s count value capture register.

You can [`read`](crate::generic::Reg::read) this register and get [`timer_cnt_cap::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@timer_cnt_cap`] module*/
pub type TIMER_CNT_CAP = crate::Reg<timer_cnt_cap::TIMER_CNT_CAP_SPEC>;
///Ledc timer%s count value capture register.
pub mod timer_cnt_cap;
/**CONF (rw) register accessor: Global ledc configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@conf`] module*/
pub type CONF = crate::Reg<conf::CONF_SPEC>;
///Global ledc configuration register
pub mod conf;
/**DATE (rw) register accessor: Version control register

You can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@date`] module*/
pub type DATE = crate::Reg<date::DATE_SPEC>;
///Version control register
pub mod date;
