#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
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
    #[doc = "0x00..0xa0 - Cluster HSCH%s, containing HSCH?_CONF0, HSCH?_HPOINT, HSCH?_DUTY, HSCH?_CONF1, HSCH?_DUTY_R"]
    #[inline(always)]
    pub const fn hsch(&self, n: usize) -> &HSCH {
        &self.hsch[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0xa0 - Cluster HSCH%s, containing HSCH?_CONF0, HSCH?_HPOINT, HSCH?_DUTY, HSCH?_CONF1, HSCH?_DUTY_R"]
    #[inline(always)]
    pub fn hsch_iter(&self) -> impl Iterator<Item = &HSCH> {
        self.hsch.iter()
    }
    #[doc = "0xa0..0x140 - Cluster LSCH%s, containing LSCH?_CONF0, LSCH?_HPOINT, LSCH?_DUTY, LSCH?_CONF1, LSCH?_DUTY_R"]
    #[inline(always)]
    pub const fn lsch(&self, n: usize) -> &LSCH {
        &self.lsch[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xa0..0x140 - Cluster LSCH%s, containing LSCH?_CONF0, LSCH?_HPOINT, LSCH?_DUTY, LSCH?_CONF1, LSCH?_DUTY_R"]
    #[inline(always)]
    pub fn lsch_iter(&self) -> impl Iterator<Item = &LSCH> {
        self.lsch.iter()
    }
    #[doc = "0x140..0x160 - Cluster HSTIMER%s, containing HSTIMER?_CONF, HSTIMER?_VALUE"]
    #[inline(always)]
    pub const fn hstimer(&self, n: usize) -> &HSTIMER {
        &self.hstimer[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x140..0x160 - Cluster HSTIMER%s, containing HSTIMER?_CONF, HSTIMER?_VALUE"]
    #[inline(always)]
    pub fn hstimer_iter(&self) -> impl Iterator<Item = &HSTIMER> {
        self.hstimer.iter()
    }
    #[doc = "0x160..0x180 - Cluster LSTIMER%s, containing LSTIMER?_CONF, LSTIMER?_VALUE"]
    #[inline(always)]
    pub const fn lstimer(&self, n: usize) -> &LSTIMER {
        &self.lstimer[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x160..0x180 - Cluster LSTIMER%s, containing LSTIMER?_CONF, LSTIMER?_VALUE"]
    #[inline(always)]
    pub fn lstimer_iter(&self) -> impl Iterator<Item = &LSTIMER> {
        self.lstimer.iter()
    }
    #[doc = "0x180 - "]
    #[inline(always)]
    pub const fn int_raw(&self) -> &INT_RAW {
        &self.int_raw
    }
    #[doc = "0x184 - "]
    #[inline(always)]
    pub const fn int_st(&self) -> &INT_ST {
        &self.int_st
    }
    #[doc = "0x188 - "]
    #[inline(always)]
    pub const fn int_ena(&self) -> &INT_ENA {
        &self.int_ena
    }
    #[doc = "0x18c - "]
    #[inline(always)]
    pub const fn int_clr(&self) -> &INT_CLR {
        &self.int_clr
    }
    #[doc = "0x190 - "]
    #[inline(always)]
    pub const fn conf(&self) -> &CONF {
        &self.conf
    }
    #[doc = "0x1fc - "]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
#[doc = "Cluster HSCH%s, containing HSCH?_CONF0, HSCH?_HPOINT, HSCH?_DUTY, HSCH?_CONF1, HSCH?_DUTY_R"]
pub use self::hsch::HSCH;
#[doc = r"Cluster"]
#[doc = "Cluster HSCH%s, containing HSCH?_CONF0, HSCH?_HPOINT, HSCH?_DUTY, HSCH?_CONF1, HSCH?_DUTY_R"]
pub mod hsch;
#[doc = "Cluster LSCH%s, containing LSCH?_CONF0, LSCH?_HPOINT, LSCH?_DUTY, LSCH?_CONF1, LSCH?_DUTY_R"]
pub use self::lsch::LSCH;
#[doc = r"Cluster"]
#[doc = "Cluster LSCH%s, containing LSCH?_CONF0, LSCH?_HPOINT, LSCH?_DUTY, LSCH?_CONF1, LSCH?_DUTY_R"]
pub mod lsch;
#[doc = "Cluster HSTIMER%s, containing HSTIMER?_CONF, HSTIMER?_VALUE"]
pub use self::hstimer::HSTIMER;
#[doc = r"Cluster"]
#[doc = "Cluster HSTIMER%s, containing HSTIMER?_CONF, HSTIMER?_VALUE"]
pub mod hstimer;
#[doc = "Cluster LSTIMER%s, containing LSTIMER?_CONF, LSTIMER?_VALUE"]
pub use self::lstimer::LSTIMER;
#[doc = r"Cluster"]
#[doc = "Cluster LSTIMER%s, containing LSTIMER?_CONF, LSTIMER?_VALUE"]
pub mod lstimer;
#[doc = "INT_RAW (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_raw::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_raw`] module"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = ""]
pub mod int_raw;
#[doc = "INT_ST (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_st`] module"]
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
#[doc = ""]
pub mod int_st;
#[doc = "INT_ENA (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_ena`] module"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = ""]
pub mod int_ena;
#[doc = "INT_CLR (w) register accessor: \n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_clr`] module"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = ""]
pub mod int_clr;
#[doc = "CONF (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@conf`] module"]
pub type CONF = crate::Reg<conf::CONF_SPEC>;
#[doc = ""]
pub mod conf;
#[doc = "DATE (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = ""]
pub mod date;
