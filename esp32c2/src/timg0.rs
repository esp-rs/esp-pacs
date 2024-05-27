#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
///Register block
pub struct RegisterBlock {
    t: [T; 1],
    _reserved1: [u8; 0x24],
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
    int_ena_timers: INT_ENA_TIMERS,
    int_raw_timers: INT_RAW_TIMERS,
    int_st_timers: INT_ST_TIMERS,
    int_clr_timers: INT_CLR_TIMERS,
    rtccalicfg2: RTCCALICFG2,
    _reserved16: [u8; 0x74],
    ntimers_date: NTIMERS_DATE,
    regclk: REGCLK,
}
impl RegisterBlock {
    ///0x00..0x24 - Cluster T%s, containing T?CONFIG, T?LO, T?HI, T?UPDATE, T?ALARMLO, T?ALARMHI, T?LOADLO, T?LOADHI, T?LOAD
    #[inline(always)]
    pub const fn t(&self, n: usize) -> &T {
        &self.t[n]
    }
    ///Iterator for array of:
    ///0x00..0x24 - Cluster T%s, containing T?CONFIG, T?LO, T?HI, T?UPDATE, T?ALARMLO, T?ALARMHI, T?LOADLO, T?LOADHI, T?LOAD
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
    ///0x68 - RTC calibration configure register
    #[inline(always)]
    pub const fn rtccalicfg(&self) -> &RTCCALICFG {
        &self.rtccalicfg
    }
    ///0x6c - RTC calibration configure1 register
    #[inline(always)]
    pub const fn rtccalicfg1(&self) -> &RTCCALICFG1 {
        &self.rtccalicfg1
    }
    ///0x70 - Interrupt enable bits
    #[inline(always)]
    pub const fn int_ena_timers(&self) -> &INT_ENA_TIMERS {
        &self.int_ena_timers
    }
    ///0x74 - Raw interrupt status
    #[inline(always)]
    pub const fn int_raw_timers(&self) -> &INT_RAW_TIMERS {
        &self.int_raw_timers
    }
    ///0x78 - Masked interrupt status
    #[inline(always)]
    pub const fn int_st_timers(&self) -> &INT_ST_TIMERS {
        &self.int_st_timers
    }
    ///0x7c - Interrupt clear bits
    #[inline(always)]
    pub const fn int_clr_timers(&self) -> &INT_CLR_TIMERS {
        &self.int_clr_timers
    }
    ///0x80 - Timer group calibration register
    #[inline(always)]
    pub const fn rtccalicfg2(&self) -> &RTCCALICFG2 {
        &self.rtccalicfg2
    }
    ///0xf8 - Timer version control register
    #[inline(always)]
    pub const fn ntimers_date(&self) -> &NTIMERS_DATE {
        &self.ntimers_date
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
/**RTCCALICFG (rw) register accessor: RTC calibration configure register

You can [`read`](crate::generic::Reg::read) this register and get [`rtccalicfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtccalicfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rtccalicfg`] module*/
pub type RTCCALICFG = crate::Reg<rtccalicfg::RTCCALICFG_SPEC>;
///RTC calibration configure register
pub mod rtccalicfg;
/**RTCCALICFG1 (r) register accessor: RTC calibration configure1 register

You can [`read`](crate::generic::Reg::read) this register and get [`rtccalicfg1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rtccalicfg1`] module*/
pub type RTCCALICFG1 = crate::Reg<rtccalicfg1::RTCCALICFG1_SPEC>;
///RTC calibration configure1 register
pub mod rtccalicfg1;
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
/**NTIMERS_DATE (rw) register accessor: Timer version control register

You can [`read`](crate::generic::Reg::read) this register and get [`ntimers_date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ntimers_date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ntimers_date`] module*/
pub type NTIMERS_DATE = crate::Reg<ntimers_date::NTIMERS_DATE_SPEC>;
///Timer version control register
pub mod ntimers_date;
/**REGCLK (rw) register accessor: Timer group clock gate register

You can [`read`](crate::generic::Reg::read) this register and get [`regclk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regclk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@regclk`] module*/
pub type REGCLK = crate::Reg<regclk::REGCLK_SPEC>;
///Timer group clock gate register
pub mod regclk;
