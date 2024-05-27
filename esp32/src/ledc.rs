#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
///Register block
pub struct RegisterBlock {
    hsch: [HSCH; 8],
    lsch: [LSCH; 8],
    hstimer: [HSTIMER; 4],
    lstimer: [LSTIMER; 4],
    int_raw: INT_RAW,
    int_st: INT_ST,
    int_ena: INT_ENA,
    int_clr: INT_CLR,
    conf: CONF,
    _reserved9: [u8; 0x68],
    date: DATE,
}
impl RegisterBlock {
    ///0x00..0xa0 - Cluster HSCH%s, containing HSCH?_CONF0, HSCH?_HPOINT, HSCH?_DUTY, HSCH?_CONF1, HSCH?_DUTY_R
    #[inline(always)]
    pub const fn hsch(&self, n: usize) -> &HSCH {
        &self.hsch[n]
    }
    ///Iterator for array of:
    ///0x00..0xa0 - Cluster HSCH%s, containing HSCH?_CONF0, HSCH?_HPOINT, HSCH?_DUTY, HSCH?_CONF1, HSCH?_DUTY_R
    #[inline(always)]
    pub fn hsch_iter(&self) -> impl Iterator<Item = &HSCH> {
        self.hsch.iter()
    }
    ///0xa0..0x140 - Cluster LSCH%s, containing LSCH?_CONF0, LSCH?_HPOINT, LSCH?_DUTY, LSCH?_CONF1, LSCH?_DUTY_R
    #[inline(always)]
    pub const fn lsch(&self, n: usize) -> &LSCH {
        &self.lsch[n]
    }
    ///Iterator for array of:
    ///0xa0..0x140 - Cluster LSCH%s, containing LSCH?_CONF0, LSCH?_HPOINT, LSCH?_DUTY, LSCH?_CONF1, LSCH?_DUTY_R
    #[inline(always)]
    pub fn lsch_iter(&self) -> impl Iterator<Item = &LSCH> {
        self.lsch.iter()
    }
    ///0x140..0x160 - Cluster HSTIMER%s, containing HSTIMER?_CONF, HSTIMER?_VALUE
    #[inline(always)]
    pub const fn hstimer(&self, n: usize) -> &HSTIMER {
        &self.hstimer[n]
    }
    ///Iterator for array of:
    ///0x140..0x160 - Cluster HSTIMER%s, containing HSTIMER?_CONF, HSTIMER?_VALUE
    #[inline(always)]
    pub fn hstimer_iter(&self) -> impl Iterator<Item = &HSTIMER> {
        self.hstimer.iter()
    }
    ///0x160..0x180 - Cluster LSTIMER%s, containing LSTIMER?_CONF, LSTIMER?_VALUE
    #[inline(always)]
    pub const fn lstimer(&self, n: usize) -> &LSTIMER {
        &self.lstimer[n]
    }
    ///Iterator for array of:
    ///0x160..0x180 - Cluster LSTIMER%s, containing LSTIMER?_CONF, LSTIMER?_VALUE
    #[inline(always)]
    pub fn lstimer_iter(&self) -> impl Iterator<Item = &LSTIMER> {
        self.lstimer.iter()
    }
    ///0x180 -
    #[inline(always)]
    pub const fn int_raw(&self) -> &INT_RAW {
        &self.int_raw
    }
    ///0x184 -
    #[inline(always)]
    pub const fn int_st(&self) -> &INT_ST {
        &self.int_st
    }
    ///0x188 -
    #[inline(always)]
    pub const fn int_ena(&self) -> &INT_ENA {
        &self.int_ena
    }
    ///0x18c -
    #[inline(always)]
    pub const fn int_clr(&self) -> &INT_CLR {
        &self.int_clr
    }
    ///0x190 -
    #[inline(always)]
    pub const fn conf(&self) -> &CONF {
        &self.conf
    }
    ///0x1fc -
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
///Cluster HSCH%s, containing HSCH?_CONF0, HSCH?_HPOINT, HSCH?_DUTY, HSCH?_CONF1, HSCH?_DUTY_R
pub use self::hsch::HSCH;
///Cluster
///Cluster HSCH%s, containing HSCH?_CONF0, HSCH?_HPOINT, HSCH?_DUTY, HSCH?_CONF1, HSCH?_DUTY_R
pub mod hsch;
///Cluster LSCH%s, containing LSCH?_CONF0, LSCH?_HPOINT, LSCH?_DUTY, LSCH?_CONF1, LSCH?_DUTY_R
pub use self::lsch::LSCH;
///Cluster
///Cluster LSCH%s, containing LSCH?_CONF0, LSCH?_HPOINT, LSCH?_DUTY, LSCH?_CONF1, LSCH?_DUTY_R
pub mod lsch;
///Cluster HSTIMER%s, containing HSTIMER?_CONF, HSTIMER?_VALUE
pub use self::hstimer::HSTIMER;
///Cluster
///Cluster HSTIMER%s, containing HSTIMER?_CONF, HSTIMER?_VALUE
pub mod hstimer;
///Cluster LSTIMER%s, containing LSTIMER?_CONF, LSTIMER?_VALUE
pub use self::lstimer::LSTIMER;
///Cluster
///Cluster LSTIMER%s, containing LSTIMER?_CONF, LSTIMER?_VALUE
pub mod lstimer;
/**INT_RAW (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_raw::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

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
/**INT_ENA (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_ena`] module*/
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
///
pub mod int_ena;
/**INT_CLR (w) register accessor:

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_clr`] module*/
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
///
pub mod int_clr;
/**CONF (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@conf`] module*/
pub type CONF = crate::Reg<conf::CONF_SPEC>;
///
pub mod conf;
/**DATE (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@date`] module*/
pub type DATE = crate::Reg<date::DATE_SPEC>;
///
pub mod date;
