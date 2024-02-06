#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    tconfig: (),
    _reserved1: [u8; 0x04],
    tlo: (),
    _reserved2: [u8; 0x04],
    thi: (),
    _reserved3: [u8; 0x04],
    tupdate: (),
    _reserved4: [u8; 0x04],
    talarmlo: (),
    _reserved5: [u8; 0x04],
    talarmhi: (),
    _reserved6: [u8; 0x04],
    tloadlo: (),
    _reserved7: [u8; 0x04],
    tloadhi: (),
    _reserved8: [u8; 0x04],
    tload: (),
    _reserved9: [u8; 0x28],
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
    _reserved34: [u8; 0x4c],
    timers_date: TIMERS_DATE,
    regclk: REGCLK,
}
impl RegisterBlock {
    #[doc = "0x00..0x08 - Timer %s configuration register"]
    #[inline(always)]
    pub const fn tconfig(&self, n: usize) -> &TCONFIG {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe { &*(self as *const Self).cast::<u8>().add(0).add(36 * n).cast() }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x08 - Timer %s configuration register"]
    #[inline(always)]
    pub fn tconfig_iter(&self) -> impl Iterator<Item = &TCONFIG> {
        (0..2)
            .map(move |n| unsafe { &*(self as *const Self).cast::<u8>().add(0).add(36 * n).cast() })
    }
    #[doc = "0x00 - Timer 0 configuration register"]
    #[inline(always)]
    pub const fn t0config(&self) -> &TCONFIG {
        self.tconfig(0)
    }
    #[doc = "0x24 - Timer 1 configuration register"]
    #[inline(always)]
    pub const fn t1config(&self) -> &TCONFIG {
        self.tconfig(1)
    }
    #[doc = "0x04..0x0c - Timer %s current value, low 32 bits"]
    #[inline(always)]
    pub const fn tlo(&self, n: usize) -> &TLO {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe { &*(self as *const Self).cast::<u8>().add(4).add(36 * n).cast() }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x04..0x0c - Timer %s current value, low 32 bits"]
    #[inline(always)]
    pub fn tlo_iter(&self) -> impl Iterator<Item = &TLO> {
        (0..2)
            .map(move |n| unsafe { &*(self as *const Self).cast::<u8>().add(4).add(36 * n).cast() })
    }
    #[doc = "0x04 - Timer 0 current value, low 32 bits"]
    #[inline(always)]
    pub const fn t0lo(&self) -> &TLO {
        self.tlo(0)
    }
    #[doc = "0x28 - Timer 1 current value, low 32 bits"]
    #[inline(always)]
    pub const fn t1lo(&self) -> &TLO {
        self.tlo(1)
    }
    #[doc = "0x08..0x10 - Timer %s current value, high 32 bits"]
    #[inline(always)]
    pub const fn thi(&self, n: usize) -> &THI {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe { &*(self as *const Self).cast::<u8>().add(8).add(36 * n).cast() }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x08..0x10 - Timer %s current value, high 32 bits"]
    #[inline(always)]
    pub fn thi_iter(&self) -> impl Iterator<Item = &THI> {
        (0..2)
            .map(move |n| unsafe { &*(self as *const Self).cast::<u8>().add(8).add(36 * n).cast() })
    }
    #[doc = "0x08 - Timer 0 current value, high 32 bits"]
    #[inline(always)]
    pub const fn t0hi(&self) -> &THI {
        self.thi(0)
    }
    #[doc = "0x2c - Timer 1 current value, high 32 bits"]
    #[inline(always)]
    pub const fn t1hi(&self) -> &THI {
        self.thi(1)
    }
    #[doc = "0x0c..0x14 - Write to copy current timer value to TIMG_T%sLO_REG or TIMGn_T%sHI_REG"]
    #[inline(always)]
    pub const fn tupdate(&self, n: usize) -> &TUPDATE {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(12)
                .add(36 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x0c..0x14 - Write to copy current timer value to TIMG_T%sLO_REG or TIMGn_T%sHI_REG"]
    #[inline(always)]
    pub fn tupdate_iter(&self) -> impl Iterator<Item = &TUPDATE> {
        (0..2).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(12)
                .add(36 * n)
                .cast()
        })
    }
    #[doc = "0x0c - Write to copy current timer value to TIMG_T0LO_REG or TIMGn_T0HI_REG"]
    #[inline(always)]
    pub const fn t0update(&self) -> &TUPDATE {
        self.tupdate(0)
    }
    #[doc = "0x30 - Write to copy current timer value to TIMG_T1LO_REG or TIMGn_T1HI_REG"]
    #[inline(always)]
    pub const fn t1update(&self) -> &TUPDATE {
        self.tupdate(1)
    }
    #[doc = "0x10..0x18 - Timer %s alarm value, low 32 bits"]
    #[inline(always)]
    pub const fn talarmlo(&self, n: usize) -> &TALARMLO {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(16)
                .add(36 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x10..0x18 - Timer %s alarm value, low 32 bits"]
    #[inline(always)]
    pub fn talarmlo_iter(&self) -> impl Iterator<Item = &TALARMLO> {
        (0..2).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(16)
                .add(36 * n)
                .cast()
        })
    }
    #[doc = "0x10 - Timer 0 alarm value, low 32 bits"]
    #[inline(always)]
    pub const fn t0alarmlo(&self) -> &TALARMLO {
        self.talarmlo(0)
    }
    #[doc = "0x34 - Timer 1 alarm value, low 32 bits"]
    #[inline(always)]
    pub const fn t1alarmlo(&self) -> &TALARMLO {
        self.talarmlo(1)
    }
    #[doc = "0x14..0x1c - Timer %s alarm value, high bits"]
    #[inline(always)]
    pub const fn talarmhi(&self, n: usize) -> &TALARMHI {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(20)
                .add(36 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x14..0x1c - Timer %s alarm value, high bits"]
    #[inline(always)]
    pub fn talarmhi_iter(&self) -> impl Iterator<Item = &TALARMHI> {
        (0..2).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(20)
                .add(36 * n)
                .cast()
        })
    }
    #[doc = "0x14 - Timer 0 alarm value, high bits"]
    #[inline(always)]
    pub const fn t0alarmhi(&self) -> &TALARMHI {
        self.talarmhi(0)
    }
    #[doc = "0x38 - Timer 1 alarm value, high bits"]
    #[inline(always)]
    pub const fn t1alarmhi(&self) -> &TALARMHI {
        self.talarmhi(1)
    }
    #[doc = "0x18..0x20 - Timer %s reload value, low 32 bits"]
    #[inline(always)]
    pub const fn tloadlo(&self, n: usize) -> &TLOADLO {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(24)
                .add(36 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x18..0x20 - Timer %s reload value, low 32 bits"]
    #[inline(always)]
    pub fn tloadlo_iter(&self) -> impl Iterator<Item = &TLOADLO> {
        (0..2).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(24)
                .add(36 * n)
                .cast()
        })
    }
    #[doc = "0x18 - Timer 0 reload value, low 32 bits"]
    #[inline(always)]
    pub const fn t0loadlo(&self) -> &TLOADLO {
        self.tloadlo(0)
    }
    #[doc = "0x3c - Timer 1 reload value, low 32 bits"]
    #[inline(always)]
    pub const fn t1loadlo(&self) -> &TLOADLO {
        self.tloadlo(1)
    }
    #[doc = "0x1c..0x24 - Timer %s reload value, high 32 bits"]
    #[inline(always)]
    pub const fn tloadhi(&self, n: usize) -> &TLOADHI {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(28)
                .add(36 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1c..0x24 - Timer %s reload value, high 32 bits"]
    #[inline(always)]
    pub fn tloadhi_iter(&self) -> impl Iterator<Item = &TLOADHI> {
        (0..2).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(28)
                .add(36 * n)
                .cast()
        })
    }
    #[doc = "0x1c - Timer 0 reload value, high 32 bits"]
    #[inline(always)]
    pub const fn t0loadhi(&self) -> &TLOADHI {
        self.tloadhi(0)
    }
    #[doc = "0x40 - Timer 1 reload value, high 32 bits"]
    #[inline(always)]
    pub const fn t1loadhi(&self) -> &TLOADHI {
        self.tloadhi(1)
    }
    #[doc = "0x20..0x28 - Write to reload timer from TIMG_T%sLOADLO_REG or TIMG_T%sLOADHI_REG"]
    #[inline(always)]
    pub const fn tload(&self, n: usize) -> &TLOAD {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(32)
                .add(36 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x20..0x28 - Write to reload timer from TIMG_T%sLOADLO_REG or TIMG_T%sLOADHI_REG"]
    #[inline(always)]
    pub fn tload_iter(&self) -> impl Iterator<Item = &TLOAD> {
        (0..2).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(32)
                .add(36 * n)
                .cast()
        })
    }
    #[doc = "0x20 - Write to reload timer from TIMG_T0LOADLO_REG or TIMG_T0LOADHI_REG"]
    #[inline(always)]
    pub const fn t0load(&self) -> &TLOAD {
        self.tload(0)
    }
    #[doc = "0x44 - Write to reload timer from TIMG_T1LOADLO_REG or TIMG_T1LOADHI_REG"]
    #[inline(always)]
    pub const fn t1load(&self) -> &TLOAD {
        self.tload(1)
    }
    #[doc = "0x48 - Watchdog timer configuration register"]
    #[inline(always)]
    pub const fn wdtconfig0(&self) -> &WDTCONFIG0 {
        &self.wdtconfig0
    }
    #[doc = "0x4c - Watchdog timer prescaler register"]
    #[inline(always)]
    pub const fn wdtconfig1(&self) -> &WDTCONFIG1 {
        &self.wdtconfig1
    }
    #[doc = "0x50 - Watchdog timer stage 0 timeout value"]
    #[inline(always)]
    pub const fn wdtconfig2(&self) -> &WDTCONFIG2 {
        &self.wdtconfig2
    }
    #[doc = "0x54 - Watchdog timer stage 1 timeout value"]
    #[inline(always)]
    pub const fn wdtconfig3(&self) -> &WDTCONFIG3 {
        &self.wdtconfig3
    }
    #[doc = "0x58 - Watchdog timer stage 2 timeout value"]
    #[inline(always)]
    pub const fn wdtconfig4(&self) -> &WDTCONFIG4 {
        &self.wdtconfig4
    }
    #[doc = "0x5c - Watchdog timer stage 3 timeout value"]
    #[inline(always)]
    pub const fn wdtconfig5(&self) -> &WDTCONFIG5 {
        &self.wdtconfig5
    }
    #[doc = "0x60 - Write to feed the watchdog timer"]
    #[inline(always)]
    pub const fn wdtfeed(&self) -> &WDTFEED {
        &self.wdtfeed
    }
    #[doc = "0x64 - Watchdog write protect register"]
    #[inline(always)]
    pub const fn wdtwprotect(&self) -> &WDTWPROTECT {
        &self.wdtwprotect
    }
    #[doc = "0x68 - RTC calibration configuration register"]
    #[inline(always)]
    pub const fn rtccalicfg(&self) -> &RTCCALICFG {
        &self.rtccalicfg
    }
    #[doc = "0x6c - RTC calibration configuration register 1"]
    #[inline(always)]
    pub const fn rtccalicfg1(&self) -> &RTCCALICFG1 {
        &self.rtccalicfg1
    }
    #[doc = "0x70 - LACT configuration register"]
    #[inline(always)]
    pub const fn lactconfig(&self) -> &LACTCONFIG {
        &self.lactconfig
    }
    #[doc = "0x74 - LACT RTC register"]
    #[inline(always)]
    pub const fn lactrtc(&self) -> &LACTRTC {
        &self.lactrtc
    }
    #[doc = "0x78 - LACT low register"]
    #[inline(always)]
    pub const fn lactlo(&self) -> &LACTLO {
        &self.lactlo
    }
    #[doc = "0x7c - LACT high register"]
    #[inline(always)]
    pub const fn lacthi(&self) -> &LACTHI {
        &self.lacthi
    }
    #[doc = "0x80 - LACT update register"]
    #[inline(always)]
    pub const fn lactupdate(&self) -> &LACTUPDATE {
        &self.lactupdate
    }
    #[doc = "0x84 - LACT alarm low register"]
    #[inline(always)]
    pub const fn lactalarmlo(&self) -> &LACTALARMLO {
        &self.lactalarmlo
    }
    #[doc = "0x88 - LACT alarm high register"]
    #[inline(always)]
    pub const fn lactalarmhi(&self) -> &LACTALARMHI {
        &self.lactalarmhi
    }
    #[doc = "0x8c - LACT load low register"]
    #[inline(always)]
    pub const fn lactloadlo(&self) -> &LACTLOADLO {
        &self.lactloadlo
    }
    #[doc = "0x90 - Timer LACT load high register"]
    #[inline(always)]
    pub const fn lactloadhi(&self) -> &LACTLOADHI {
        &self.lactloadhi
    }
    #[doc = "0x94 - Timer LACT load register"]
    #[inline(always)]
    pub const fn lactload(&self) -> &LACTLOAD {
        &self.lactload
    }
    #[doc = "0x98 - Interrupt enable bits"]
    #[inline(always)]
    pub const fn int_ena_timers(&self) -> &INT_ENA_TIMERS {
        &self.int_ena_timers
    }
    #[doc = "0x9c - Raw interrupt status"]
    #[inline(always)]
    pub const fn int_raw_timers(&self) -> &INT_RAW_TIMERS {
        &self.int_raw_timers
    }
    #[doc = "0xa0 - Masked interrupt status"]
    #[inline(always)]
    pub const fn int_st_timers(&self) -> &INT_ST_TIMERS {
        &self.int_st_timers
    }
    #[doc = "0xa4 - Interrupt clear bits"]
    #[inline(always)]
    pub const fn int_clr_timers(&self) -> &INT_CLR_TIMERS {
        &self.int_clr_timers
    }
    #[doc = "0xa8 - Timer group calibration register"]
    #[inline(always)]
    pub const fn rtccalicfg2(&self) -> &RTCCALICFG2 {
        &self.rtccalicfg2
    }
    #[doc = "0xf8 - Version control register"]
    #[inline(always)]
    pub const fn timers_date(&self) -> &TIMERS_DATE {
        &self.timers_date
    }
    #[doc = "0xfc - Timer group clock gate register"]
    #[inline(always)]
    pub const fn regclk(&self) -> &REGCLK {
        &self.regclk
    }
}
#[doc = "TCONFIG (rw) register accessor: Timer %s configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tconfig::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tconfig::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tconfig`] module"]
pub type TCONFIG = crate::Reg<tconfig::TCONFIG_SPEC>;
#[doc = "Timer %s configuration register"]
pub mod tconfig;
#[doc = "TLO (r) register accessor: Timer %s current value, low 32 bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tlo::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tlo`] module"]
pub type TLO = crate::Reg<tlo::TLO_SPEC>;
#[doc = "Timer %s current value, low 32 bits"]
pub mod tlo;
#[doc = "THI (r) register accessor: Timer %s current value, high 32 bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`thi::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@thi`] module"]
pub type THI = crate::Reg<thi::THI_SPEC>;
#[doc = "Timer %s current value, high 32 bits"]
pub mod thi;
#[doc = "TUPDATE (rw) register accessor: Write to copy current timer value to TIMG_T%sLO_REG or TIMGn_T%sHI_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tupdate::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tupdate::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tupdate`] module"]
pub type TUPDATE = crate::Reg<tupdate::TUPDATE_SPEC>;
#[doc = "Write to copy current timer value to TIMG_T%sLO_REG or TIMGn_T%sHI_REG"]
pub mod tupdate;
#[doc = "TALARMLO (rw) register accessor: Timer %s alarm value, low 32 bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`talarmlo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`talarmlo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@talarmlo`] module"]
pub type TALARMLO = crate::Reg<talarmlo::TALARMLO_SPEC>;
#[doc = "Timer %s alarm value, low 32 bits"]
pub mod talarmlo;
#[doc = "TALARMHI (rw) register accessor: Timer %s alarm value, high bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`talarmhi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`talarmhi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@talarmhi`] module"]
pub type TALARMHI = crate::Reg<talarmhi::TALARMHI_SPEC>;
#[doc = "Timer %s alarm value, high bits"]
pub mod talarmhi;
#[doc = "TLOADLO (rw) register accessor: Timer %s reload value, low 32 bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tloadlo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tloadlo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tloadlo`] module"]
pub type TLOADLO = crate::Reg<tloadlo::TLOADLO_SPEC>;
#[doc = "Timer %s reload value, low 32 bits"]
pub mod tloadlo;
#[doc = "TLOADHI (rw) register accessor: Timer %s reload value, high 32 bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tloadhi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tloadhi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tloadhi`] module"]
pub type TLOADHI = crate::Reg<tloadhi::TLOADHI_SPEC>;
#[doc = "Timer %s reload value, high 32 bits"]
pub mod tloadhi;
#[doc = "TLOAD (w) register accessor: Write to reload timer from TIMG_T%sLOADLO_REG or TIMG_T%sLOADHI_REG\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tload::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tload`] module"]
pub type TLOAD = crate::Reg<tload::TLOAD_SPEC>;
#[doc = "Write to reload timer from TIMG_T%sLOADLO_REG or TIMG_T%sLOADHI_REG"]
pub mod tload;
#[doc = "WDTCONFIG0 (rw) register accessor: Watchdog timer configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtconfig0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtconfig0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdtconfig0`] module"]
pub type WDTCONFIG0 = crate::Reg<wdtconfig0::WDTCONFIG0_SPEC>;
#[doc = "Watchdog timer configuration register"]
pub mod wdtconfig0;
#[doc = "WDTCONFIG1 (rw) register accessor: Watchdog timer prescaler register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtconfig1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtconfig1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdtconfig1`] module"]
pub type WDTCONFIG1 = crate::Reg<wdtconfig1::WDTCONFIG1_SPEC>;
#[doc = "Watchdog timer prescaler register"]
pub mod wdtconfig1;
#[doc = "WDTCONFIG2 (rw) register accessor: Watchdog timer stage 0 timeout value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtconfig2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtconfig2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdtconfig2`] module"]
pub type WDTCONFIG2 = crate::Reg<wdtconfig2::WDTCONFIG2_SPEC>;
#[doc = "Watchdog timer stage 0 timeout value"]
pub mod wdtconfig2;
#[doc = "WDTCONFIG3 (rw) register accessor: Watchdog timer stage 1 timeout value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtconfig3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtconfig3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdtconfig3`] module"]
pub type WDTCONFIG3 = crate::Reg<wdtconfig3::WDTCONFIG3_SPEC>;
#[doc = "Watchdog timer stage 1 timeout value"]
pub mod wdtconfig3;
#[doc = "WDTCONFIG4 (rw) register accessor: Watchdog timer stage 2 timeout value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtconfig4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtconfig4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdtconfig4`] module"]
pub type WDTCONFIG4 = crate::Reg<wdtconfig4::WDTCONFIG4_SPEC>;
#[doc = "Watchdog timer stage 2 timeout value"]
pub mod wdtconfig4;
#[doc = "WDTCONFIG5 (rw) register accessor: Watchdog timer stage 3 timeout value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtconfig5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtconfig5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdtconfig5`] module"]
pub type WDTCONFIG5 = crate::Reg<wdtconfig5::WDTCONFIG5_SPEC>;
#[doc = "Watchdog timer stage 3 timeout value"]
pub mod wdtconfig5;
#[doc = "WDTFEED (w) register accessor: Write to feed the watchdog timer\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtfeed::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdtfeed`] module"]
pub type WDTFEED = crate::Reg<wdtfeed::WDTFEED_SPEC>;
#[doc = "Write to feed the watchdog timer"]
pub mod wdtfeed;
#[doc = "WDTWPROTECT (rw) register accessor: Watchdog write protect register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtwprotect::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtwprotect::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdtwprotect`] module"]
pub type WDTWPROTECT = crate::Reg<wdtwprotect::WDTWPROTECT_SPEC>;
#[doc = "Watchdog write protect register"]
pub mod wdtwprotect;
#[doc = "RTCCALICFG (rw) register accessor: RTC calibration configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtccalicfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtccalicfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtccalicfg`] module"]
pub type RTCCALICFG = crate::Reg<rtccalicfg::RTCCALICFG_SPEC>;
#[doc = "RTC calibration configuration register"]
pub mod rtccalicfg;
#[doc = "RTCCALICFG1 (r) register accessor: RTC calibration configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtccalicfg1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtccalicfg1`] module"]
pub type RTCCALICFG1 = crate::Reg<rtccalicfg1::RTCCALICFG1_SPEC>;
#[doc = "RTC calibration configuration register 1"]
pub mod rtccalicfg1;
#[doc = "LACTCONFIG (rw) register accessor: LACT configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lactconfig::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lactconfig::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lactconfig`] module"]
pub type LACTCONFIG = crate::Reg<lactconfig::LACTCONFIG_SPEC>;
#[doc = "LACT configuration register"]
pub mod lactconfig;
#[doc = "LACTRTC (rw) register accessor: LACT RTC register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lactrtc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lactrtc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lactrtc`] module"]
pub type LACTRTC = crate::Reg<lactrtc::LACTRTC_SPEC>;
#[doc = "LACT RTC register"]
pub mod lactrtc;
#[doc = "LACTLO (r) register accessor: LACT low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lactlo::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lactlo`] module"]
pub type LACTLO = crate::Reg<lactlo::LACTLO_SPEC>;
#[doc = "LACT low register"]
pub mod lactlo;
#[doc = "LACTHI (r) register accessor: LACT high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lacthi::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lacthi`] module"]
pub type LACTHI = crate::Reg<lacthi::LACTHI_SPEC>;
#[doc = "LACT high register"]
pub mod lacthi;
#[doc = "LACTUPDATE (w) register accessor: LACT update register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lactupdate::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lactupdate`] module"]
pub type LACTUPDATE = crate::Reg<lactupdate::LACTUPDATE_SPEC>;
#[doc = "LACT update register"]
pub mod lactupdate;
#[doc = "LACTALARMLO (rw) register accessor: LACT alarm low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lactalarmlo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lactalarmlo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lactalarmlo`] module"]
pub type LACTALARMLO = crate::Reg<lactalarmlo::LACTALARMLO_SPEC>;
#[doc = "LACT alarm low register"]
pub mod lactalarmlo;
#[doc = "LACTALARMHI (rw) register accessor: LACT alarm high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lactalarmhi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lactalarmhi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lactalarmhi`] module"]
pub type LACTALARMHI = crate::Reg<lactalarmhi::LACTALARMHI_SPEC>;
#[doc = "LACT alarm high register"]
pub mod lactalarmhi;
#[doc = "LACTLOADLO (rw) register accessor: LACT load low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lactloadlo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lactloadlo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lactloadlo`] module"]
pub type LACTLOADLO = crate::Reg<lactloadlo::LACTLOADLO_SPEC>;
#[doc = "LACT load low register"]
pub mod lactloadlo;
#[doc = "LACTLOADHI (rw) register accessor: Timer LACT load high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lactloadhi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lactloadhi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lactloadhi`] module"]
pub type LACTLOADHI = crate::Reg<lactloadhi::LACTLOADHI_SPEC>;
#[doc = "Timer LACT load high register"]
pub mod lactloadhi;
#[doc = "LACTLOAD (w) register accessor: Timer LACT load register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lactload::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lactload`] module"]
pub type LACTLOAD = crate::Reg<lactload::LACTLOAD_SPEC>;
#[doc = "Timer LACT load register"]
pub mod lactload;
#[doc = "INT_ENA_TIMERS (rw) register accessor: Interrupt enable bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_ena_timers::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena_timers::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_ena_timers`] module"]
pub type INT_ENA_TIMERS = crate::Reg<int_ena_timers::INT_ENA_TIMERS_SPEC>;
#[doc = "Interrupt enable bits"]
pub mod int_ena_timers;
#[doc = "INT_RAW_TIMERS (r) register accessor: Raw interrupt status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_raw_timers::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_raw_timers`] module"]
pub type INT_RAW_TIMERS = crate::Reg<int_raw_timers::INT_RAW_TIMERS_SPEC>;
#[doc = "Raw interrupt status"]
pub mod int_raw_timers;
#[doc = "INT_ST_TIMERS (r) register accessor: Masked interrupt status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_st_timers::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_st_timers`] module"]
pub type INT_ST_TIMERS = crate::Reg<int_st_timers::INT_ST_TIMERS_SPEC>;
#[doc = "Masked interrupt status"]
pub mod int_st_timers;
#[doc = "INT_CLR_TIMERS (w) register accessor: Interrupt clear bits\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr_timers::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_clr_timers`] module"]
pub type INT_CLR_TIMERS = crate::Reg<int_clr_timers::INT_CLR_TIMERS_SPEC>;
#[doc = "Interrupt clear bits"]
pub mod int_clr_timers;
#[doc = "RTCCALICFG2 (rw) register accessor: Timer group calibration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtccalicfg2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtccalicfg2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtccalicfg2`] module"]
pub type RTCCALICFG2 = crate::Reg<rtccalicfg2::RTCCALICFG2_SPEC>;
#[doc = "Timer group calibration register"]
pub mod rtccalicfg2;
#[doc = "TIMERS_DATE (rw) register accessor: Version control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timers_date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timers_date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timers_date`] module"]
pub type TIMERS_DATE = crate::Reg<timers_date::TIMERS_DATE_SPEC>;
#[doc = "Version control register"]
pub mod timers_date;
#[doc = "REGCLK (rw) register accessor: Timer group clock gate register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regclk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regclk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regclk`] module"]
pub type REGCLK = crate::Reg<regclk::REGCLK_SPEC>;
#[doc = "Timer group clock gate register"]
pub mod regclk;
