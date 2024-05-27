#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
///Register block
pub struct RegisterBlock {
    t: [T; 2],
    wdtconfig0: WDTCONFIG0,
    wdtconfig1: WDTCONFIG1,
    wdtconfig2: WDTCONFIG2,
    wdtconfig3: WDTCONFIG3,
    wdtconfig4: WDTCONFIG4,
    wdtconfig5: WDTCONFIG5,
    wdtfeed: WDTFEED,
    wdtwprotect: WDTWPROTECT,
    rtccalicfg: RTCCALICFG,
    rtccalicfg1: RTCCALICFG1,
    lactconfig: LACTCONFIG,
    lactrtc: LACTRTC,
    lactlo: LACTLO,
    lacthi: LACTHI,
    lactupdate: LACTUPDATE,
    lactalarmlo: LACTALARMLO,
    lactalarmhi: LACTALARMHI,
    lactloadlo: LACTLOADLO,
    lactloadhi: LACTLOADHI,
    lactload: LACTLOAD,
    int_ena_timers: INT_ENA_TIMERS,
    int_raw_timers: INT_RAW_TIMERS,
    int_st_timers: INT_ST_TIMERS,
    int_clr_timers: INT_CLR_TIMERS,
    rtccalicfg2: RTCCALICFG2,
    _reserved26: [u8; 0x4c],
    timers_date: TIMERS_DATE,
    regclk: REGCLK,
}
impl RegisterBlock {
    ///0x00..0x48 - Cluster T%s, containing T?CONFIG, T?LO, T?HI, T?UPDATE, T?ALARMLO, T?ALARMHI, T?LOADLO, T?LOADHI, T?LOAD
    #[inline(always)]
    pub const fn t(&self, n: usize) -> &T {
        &self.t[n]
    }
    ///Iterator for array of:
    ///0x00..0x48 - Cluster T%s, containing T?CONFIG, T?LO, T?HI, T?UPDATE, T?ALARMLO, T?ALARMHI, T?LOADLO, T?LOADHI, T?LOAD
    #[inline(always)]
    pub fn t_iter(&self) -> impl Iterator<Item = &T> {
        self.t.iter()
    }
    ///0x48 - Watchdog timer configuration register
    #[inline(always)]
    pub const fn wdtconfig0(&self) -> &WDTCONFIG0 {
        &self.wdtconfig0
    }
    ///0x4c - Watchdog timer prescaler register
    #[inline(always)]
    pub const fn wdtconfig1(&self) -> &WDTCONFIG1 {
        &self.wdtconfig1
    }
    ///0x50 - Watchdog timer stage 0 timeout value
    #[inline(always)]
    pub const fn wdtconfig2(&self) -> &WDTCONFIG2 {
        &self.wdtconfig2
    }
    ///0x54 - Watchdog timer stage 1 timeout value
    #[inline(always)]
    pub const fn wdtconfig3(&self) -> &WDTCONFIG3 {
        &self.wdtconfig3
    }
    ///0x58 - Watchdog timer stage 2 timeout value
    #[inline(always)]
    pub const fn wdtconfig4(&self) -> &WDTCONFIG4 {
        &self.wdtconfig4
    }
    ///0x5c - Watchdog timer stage 3 timeout value
    #[inline(always)]
    pub const fn wdtconfig5(&self) -> &WDTCONFIG5 {
        &self.wdtconfig5
    }
    ///0x60 - Write to feed the watchdog timer
    #[inline(always)]
    pub const fn wdtfeed(&self) -> &WDTFEED {
        &self.wdtfeed
    }
    ///0x64 - Watchdog write protect register
    #[inline(always)]
    pub const fn wdtwprotect(&self) -> &WDTWPROTECT {
        &self.wdtwprotect
    }
    ///0x68 - RTC calibration configuration register
    #[inline(always)]
    pub const fn rtccalicfg(&self) -> &RTCCALICFG {
        &self.rtccalicfg
    }
    ///0x6c - RTC calibration configuration register 1
    #[inline(always)]
    pub const fn rtccalicfg1(&self) -> &RTCCALICFG1 {
        &self.rtccalicfg1
    }
    ///0x70 - LACT configuration register
    #[inline(always)]
    pub const fn lactconfig(&self) -> &LACTCONFIG {
        &self.lactconfig
    }
    ///0x74 - LACT RTC register
    #[inline(always)]
    pub const fn lactrtc(&self) -> &LACTRTC {
        &self.lactrtc
    }
    ///0x78 - LACT low register
    #[inline(always)]
    pub const fn lactlo(&self) -> &LACTLO {
        &self.lactlo
    }
    ///0x7c - LACT high register
    #[inline(always)]
    pub const fn lacthi(&self) -> &LACTHI {
        &self.lacthi
    }
    ///0x80 - LACT update register
    #[inline(always)]
    pub const fn lactupdate(&self) -> &LACTUPDATE {
        &self.lactupdate
    }
    ///0x84 - LACT alarm low register
    #[inline(always)]
    pub const fn lactalarmlo(&self) -> &LACTALARMLO {
        &self.lactalarmlo
    }
    ///0x88 - LACT alarm high register
    #[inline(always)]
    pub const fn lactalarmhi(&self) -> &LACTALARMHI {
        &self.lactalarmhi
    }
    ///0x8c - LACT load low register
    #[inline(always)]
    pub const fn lactloadlo(&self) -> &LACTLOADLO {
        &self.lactloadlo
    }
    ///0x90 - Timer LACT load high register
    #[inline(always)]
    pub const fn lactloadhi(&self) -> &LACTLOADHI {
        &self.lactloadhi
    }
    ///0x94 - Timer LACT load register
    #[inline(always)]
    pub const fn lactload(&self) -> &LACTLOAD {
        &self.lactload
    }
    ///0x98 - Interrupt enable bits
    #[inline(always)]
    pub const fn int_ena_timers(&self) -> &INT_ENA_TIMERS {
        &self.int_ena_timers
    }
    ///0x9c - Raw interrupt status
    #[inline(always)]
    pub const fn int_raw_timers(&self) -> &INT_RAW_TIMERS {
        &self.int_raw_timers
    }
    ///0xa0 - Masked interrupt status
    #[inline(always)]
    pub const fn int_st_timers(&self) -> &INT_ST_TIMERS {
        &self.int_st_timers
    }
    ///0xa4 - Interrupt clear bits
    #[inline(always)]
    pub const fn int_clr_timers(&self) -> &INT_CLR_TIMERS {
        &self.int_clr_timers
    }
    ///0xa8 - Timer group calibration register
    #[inline(always)]
    pub const fn rtccalicfg2(&self) -> &RTCCALICFG2 {
        &self.rtccalicfg2
    }
    ///0xf8 - Version control register
    #[inline(always)]
    pub const fn timers_date(&self) -> &TIMERS_DATE {
        &self.timers_date
    }
    ///0xfc - Timer group clock gate register
    #[inline(always)]
    pub const fn regclk(&self) -> &REGCLK {
        &self.regclk
    }
}
///Cluster T%s, containing T?CONFIG, T?LO, T?HI, T?UPDATE, T?ALARMLO, T?ALARMHI, T?LOADLO, T?LOADHI, T?LOAD
pub use self::t::T;
///Cluster
///Cluster T%s, containing T?CONFIG, T?LO, T?HI, T?UPDATE, T?ALARMLO, T?ALARMHI, T?LOADLO, T?LOADHI, T?LOAD
pub mod t;
/**WDTCONFIG0 (rw) register accessor: Watchdog timer configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`wdtconfig0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtconfig0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@wdtconfig0`] module*/
pub type WDTCONFIG0 = crate::Reg<wdtconfig0::WDTCONFIG0_SPEC>;
///Watchdog timer configuration register
pub mod wdtconfig0;
/**WDTCONFIG1 (rw) register accessor: Watchdog timer prescaler register

You can [`read`](crate::generic::Reg::read) this register and get [`wdtconfig1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtconfig1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@wdtconfig1`] module*/
pub type WDTCONFIG1 = crate::Reg<wdtconfig1::WDTCONFIG1_SPEC>;
///Watchdog timer prescaler register
pub mod wdtconfig1;
/**WDTCONFIG2 (rw) register accessor: Watchdog timer stage 0 timeout value

You can [`read`](crate::generic::Reg::read) this register and get [`wdtconfig2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtconfig2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@wdtconfig2`] module*/
pub type WDTCONFIG2 = crate::Reg<wdtconfig2::WDTCONFIG2_SPEC>;
///Watchdog timer stage 0 timeout value
pub mod wdtconfig2;
/**WDTCONFIG3 (rw) register accessor: Watchdog timer stage 1 timeout value

You can [`read`](crate::generic::Reg::read) this register and get [`wdtconfig3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtconfig3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@wdtconfig3`] module*/
pub type WDTCONFIG3 = crate::Reg<wdtconfig3::WDTCONFIG3_SPEC>;
///Watchdog timer stage 1 timeout value
pub mod wdtconfig3;
/**WDTCONFIG4 (rw) register accessor: Watchdog timer stage 2 timeout value

You can [`read`](crate::generic::Reg::read) this register and get [`wdtconfig4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtconfig4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@wdtconfig4`] module*/
pub type WDTCONFIG4 = crate::Reg<wdtconfig4::WDTCONFIG4_SPEC>;
///Watchdog timer stage 2 timeout value
pub mod wdtconfig4;
/**WDTCONFIG5 (rw) register accessor: Watchdog timer stage 3 timeout value

You can [`read`](crate::generic::Reg::read) this register and get [`wdtconfig5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtconfig5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@wdtconfig5`] module*/
pub type WDTCONFIG5 = crate::Reg<wdtconfig5::WDTCONFIG5_SPEC>;
///Watchdog timer stage 3 timeout value
pub mod wdtconfig5;
/**WDTFEED (w) register accessor: Write to feed the watchdog timer

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtfeed::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@wdtfeed`] module*/
pub type WDTFEED = crate::Reg<wdtfeed::WDTFEED_SPEC>;
///Write to feed the watchdog timer
pub mod wdtfeed;
/**WDTWPROTECT (rw) register accessor: Watchdog write protect register

You can [`read`](crate::generic::Reg::read) this register and get [`wdtwprotect::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtwprotect::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@wdtwprotect`] module*/
pub type WDTWPROTECT = crate::Reg<wdtwprotect::WDTWPROTECT_SPEC>;
///Watchdog write protect register
pub mod wdtwprotect;
/**RTCCALICFG (rw) register accessor: RTC calibration configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`rtccalicfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtccalicfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rtccalicfg`] module*/
pub type RTCCALICFG = crate::Reg<rtccalicfg::RTCCALICFG_SPEC>;
///RTC calibration configuration register
pub mod rtccalicfg;
/**RTCCALICFG1 (r) register accessor: RTC calibration configuration register 1

You can [`read`](crate::generic::Reg::read) this register and get [`rtccalicfg1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rtccalicfg1`] module*/
pub type RTCCALICFG1 = crate::Reg<rtccalicfg1::RTCCALICFG1_SPEC>;
///RTC calibration configuration register 1
pub mod rtccalicfg1;
/**LACTCONFIG (rw) register accessor: LACT configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`lactconfig::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lactconfig::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lactconfig`] module*/
pub type LACTCONFIG = crate::Reg<lactconfig::LACTCONFIG_SPEC>;
///LACT configuration register
pub mod lactconfig;
/**LACTRTC (rw) register accessor: LACT RTC register

You can [`read`](crate::generic::Reg::read) this register and get [`lactrtc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lactrtc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lactrtc`] module*/
pub type LACTRTC = crate::Reg<lactrtc::LACTRTC_SPEC>;
///LACT RTC register
pub mod lactrtc;
/**LACTLO (r) register accessor: LACT low register

You can [`read`](crate::generic::Reg::read) this register and get [`lactlo::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lactlo`] module*/
pub type LACTLO = crate::Reg<lactlo::LACTLO_SPEC>;
///LACT low register
pub mod lactlo;
/**LACTHI (r) register accessor: LACT high register

You can [`read`](crate::generic::Reg::read) this register and get [`lacthi::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lacthi`] module*/
pub type LACTHI = crate::Reg<lacthi::LACTHI_SPEC>;
///LACT high register
pub mod lacthi;
/**LACTUPDATE (w) register accessor: LACT update register

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lactupdate::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lactupdate`] module*/
pub type LACTUPDATE = crate::Reg<lactupdate::LACTUPDATE_SPEC>;
///LACT update register
pub mod lactupdate;
/**LACTALARMLO (rw) register accessor: LACT alarm low register

You can [`read`](crate::generic::Reg::read) this register and get [`lactalarmlo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lactalarmlo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lactalarmlo`] module*/
pub type LACTALARMLO = crate::Reg<lactalarmlo::LACTALARMLO_SPEC>;
///LACT alarm low register
pub mod lactalarmlo;
/**LACTALARMHI (rw) register accessor: LACT alarm high register

You can [`read`](crate::generic::Reg::read) this register and get [`lactalarmhi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lactalarmhi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lactalarmhi`] module*/
pub type LACTALARMHI = crate::Reg<lactalarmhi::LACTALARMHI_SPEC>;
///LACT alarm high register
pub mod lactalarmhi;
/**LACTLOADLO (rw) register accessor: LACT load low register

You can [`read`](crate::generic::Reg::read) this register and get [`lactloadlo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lactloadlo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lactloadlo`] module*/
pub type LACTLOADLO = crate::Reg<lactloadlo::LACTLOADLO_SPEC>;
///LACT load low register
pub mod lactloadlo;
/**LACTLOADHI (rw) register accessor: Timer LACT load high register

You can [`read`](crate::generic::Reg::read) this register and get [`lactloadhi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lactloadhi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lactloadhi`] module*/
pub type LACTLOADHI = crate::Reg<lactloadhi::LACTLOADHI_SPEC>;
///Timer LACT load high register
pub mod lactloadhi;
/**LACTLOAD (w) register accessor: Timer LACT load register

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lactload::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lactload`] module*/
pub type LACTLOAD = crate::Reg<lactload::LACTLOAD_SPEC>;
///Timer LACT load register
pub mod lactload;
/**INT_ENA_TIMERS (rw) register accessor: Interrupt enable bits

You can [`read`](crate::generic::Reg::read) this register and get [`int_ena_timers::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena_timers::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_ena_timers`] module*/
pub type INT_ENA_TIMERS = crate::Reg<int_ena_timers::INT_ENA_TIMERS_SPEC>;
///Interrupt enable bits
pub mod int_ena_timers;
/**INT_RAW_TIMERS (r) register accessor: Raw interrupt status

You can [`read`](crate::generic::Reg::read) this register and get [`int_raw_timers::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_raw_timers`] module*/
pub type INT_RAW_TIMERS = crate::Reg<int_raw_timers::INT_RAW_TIMERS_SPEC>;
///Raw interrupt status
pub mod int_raw_timers;
/**INT_ST_TIMERS (r) register accessor: Masked interrupt status

You can [`read`](crate::generic::Reg::read) this register and get [`int_st_timers::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_st_timers`] module*/
pub type INT_ST_TIMERS = crate::Reg<int_st_timers::INT_ST_TIMERS_SPEC>;
///Masked interrupt status
pub mod int_st_timers;
/**INT_CLR_TIMERS (w) register accessor: Interrupt clear bits

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr_timers::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_clr_timers`] module*/
pub type INT_CLR_TIMERS = crate::Reg<int_clr_timers::INT_CLR_TIMERS_SPEC>;
///Interrupt clear bits
pub mod int_clr_timers;
/**RTCCALICFG2 (rw) register accessor: Timer group calibration register

You can [`read`](crate::generic::Reg::read) this register and get [`rtccalicfg2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtccalicfg2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rtccalicfg2`] module*/
pub type RTCCALICFG2 = crate::Reg<rtccalicfg2::RTCCALICFG2_SPEC>;
///Timer group calibration register
pub mod rtccalicfg2;
/**TIMERS_DATE (rw) register accessor: Version control register

You can [`read`](crate::generic::Reg::read) this register and get [`timers_date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timers_date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@timers_date`] module*/
pub type TIMERS_DATE = crate::Reg<timers_date::TIMERS_DATE_SPEC>;
///Version control register
pub mod timers_date;
/**REGCLK (rw) register accessor: Timer group clock gate register

You can [`read`](crate::generic::Reg::read) this register and get [`regclk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regclk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@regclk`] module*/
pub type REGCLK = crate::Reg<regclk::REGCLK_SPEC>;
///Timer group clock gate register
pub mod regclk;
