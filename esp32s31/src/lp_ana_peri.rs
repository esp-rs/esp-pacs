#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    lp_ana_bod_mode0_cntl: LP_ANA_BOD_MODE0_CNTL,
    lp_ana_bod_mode1_cntl: LP_ANA_BOD_MODE1_CNTL,
    lp_ana_vdd_source_cntl: LP_ANA_VDD_SOURCE_CNTL,
    _reserved3: [u8; 0x0c],
    lp_ana_pg_glitch_cntl: LP_ANA_PG_GLITCH_CNTL,
    lp_ana_fib_enable: LP_ANA_FIB_ENABLE,
    lp_ana_int_raw: LP_ANA_INT_RAW,
    lp_ana_int_st: LP_ANA_INT_ST,
    lp_ana_int_ena: LP_ANA_INT_ENA,
    lp_ana_int_clr: LP_ANA_INT_CLR,
    lp_ana_lp_int_raw: LP_ANA_LP_INT_RAW,
    lp_ana_lp_int_st: LP_ANA_LP_INT_ST,
    lp_ana_lp_int_ena: LP_ANA_LP_INT_ENA,
    lp_ana_lp_int_clr: LP_ANA_LP_INT_CLR,
    _reserved13: [u8; 0x03bc],
    lp_ana_date: LP_ANA_DATE,
}
impl RegisterBlock {
    #[doc = "0x00 - need_des"]
    #[inline(always)]
    pub const fn lp_ana_bod_mode0_cntl(&self) -> &LP_ANA_BOD_MODE0_CNTL {
        &self.lp_ana_bod_mode0_cntl
    }
    #[doc = "0x04 - need_des"]
    #[inline(always)]
    pub const fn lp_ana_bod_mode1_cntl(&self) -> &LP_ANA_BOD_MODE1_CNTL {
        &self.lp_ana_bod_mode1_cntl
    }
    #[doc = "0x08 - need_des"]
    #[inline(always)]
    pub const fn lp_ana_vdd_source_cntl(&self) -> &LP_ANA_VDD_SOURCE_CNTL {
        &self.lp_ana_vdd_source_cntl
    }
    #[doc = "0x18 - need_des"]
    #[inline(always)]
    pub const fn lp_ana_pg_glitch_cntl(&self) -> &LP_ANA_PG_GLITCH_CNTL {
        &self.lp_ana_pg_glitch_cntl
    }
    #[doc = "0x1c - need_des"]
    #[inline(always)]
    pub const fn lp_ana_fib_enable(&self) -> &LP_ANA_FIB_ENABLE {
        &self.lp_ana_fib_enable
    }
    #[doc = "0x20 - need_des"]
    #[inline(always)]
    pub const fn lp_ana_int_raw(&self) -> &LP_ANA_INT_RAW {
        &self.lp_ana_int_raw
    }
    #[doc = "0x24 - need_des"]
    #[inline(always)]
    pub const fn lp_ana_int_st(&self) -> &LP_ANA_INT_ST {
        &self.lp_ana_int_st
    }
    #[doc = "0x28 - need_des"]
    #[inline(always)]
    pub const fn lp_ana_int_ena(&self) -> &LP_ANA_INT_ENA {
        &self.lp_ana_int_ena
    }
    #[doc = "0x2c - need_des"]
    #[inline(always)]
    pub const fn lp_ana_int_clr(&self) -> &LP_ANA_INT_CLR {
        &self.lp_ana_int_clr
    }
    #[doc = "0x30 - need_des"]
    #[inline(always)]
    pub const fn lp_ana_lp_int_raw(&self) -> &LP_ANA_LP_INT_RAW {
        &self.lp_ana_lp_int_raw
    }
    #[doc = "0x34 - need_des"]
    #[inline(always)]
    pub const fn lp_ana_lp_int_st(&self) -> &LP_ANA_LP_INT_ST {
        &self.lp_ana_lp_int_st
    }
    #[doc = "0x38 - need_des"]
    #[inline(always)]
    pub const fn lp_ana_lp_int_ena(&self) -> &LP_ANA_LP_INT_ENA {
        &self.lp_ana_lp_int_ena
    }
    #[doc = "0x3c - need_des"]
    #[inline(always)]
    pub const fn lp_ana_lp_int_clr(&self) -> &LP_ANA_LP_INT_CLR {
        &self.lp_ana_lp_int_clr
    }
    #[doc = "0x3fc - need_des"]
    #[inline(always)]
    pub const fn lp_ana_date(&self) -> &LP_ANA_DATE {
        &self.lp_ana_date
    }
}
#[doc = "LP_ANA_BOD_MODE0_CNTL (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_ana_bod_mode0_cntl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_ana_bod_mode0_cntl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_ana_bod_mode0_cntl`] module"]
pub type LP_ANA_BOD_MODE0_CNTL = crate::Reg<lp_ana_bod_mode0_cntl::LP_ANA_BOD_MODE0_CNTL_SPEC>;
#[doc = "need_des"]
pub mod lp_ana_bod_mode0_cntl;
#[doc = "LP_ANA_BOD_MODE1_CNTL (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_ana_bod_mode1_cntl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_ana_bod_mode1_cntl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_ana_bod_mode1_cntl`] module"]
pub type LP_ANA_BOD_MODE1_CNTL = crate::Reg<lp_ana_bod_mode1_cntl::LP_ANA_BOD_MODE1_CNTL_SPEC>;
#[doc = "need_des"]
pub mod lp_ana_bod_mode1_cntl;
#[doc = "LP_ANA_VDD_SOURCE_CNTL (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_ana_vdd_source_cntl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_ana_vdd_source_cntl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_ana_vdd_source_cntl`] module"]
pub type LP_ANA_VDD_SOURCE_CNTL = crate::Reg<lp_ana_vdd_source_cntl::LP_ANA_VDD_SOURCE_CNTL_SPEC>;
#[doc = "need_des"]
pub mod lp_ana_vdd_source_cntl;
#[doc = "LP_ANA_PG_GLITCH_CNTL (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_ana_pg_glitch_cntl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_ana_pg_glitch_cntl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_ana_pg_glitch_cntl`] module"]
pub type LP_ANA_PG_GLITCH_CNTL = crate::Reg<lp_ana_pg_glitch_cntl::LP_ANA_PG_GLITCH_CNTL_SPEC>;
#[doc = "need_des"]
pub mod lp_ana_pg_glitch_cntl;
#[doc = "LP_ANA_FIB_ENABLE (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_ana_fib_enable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_ana_fib_enable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_ana_fib_enable`] module"]
pub type LP_ANA_FIB_ENABLE = crate::Reg<lp_ana_fib_enable::LP_ANA_FIB_ENABLE_SPEC>;
#[doc = "need_des"]
pub mod lp_ana_fib_enable;
#[doc = "LP_ANA_INT_RAW (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_ana_int_raw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_ana_int_raw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_ana_int_raw`] module"]
pub type LP_ANA_INT_RAW = crate::Reg<lp_ana_int_raw::LP_ANA_INT_RAW_SPEC>;
#[doc = "need_des"]
pub mod lp_ana_int_raw;
#[doc = "LP_ANA_INT_ST (r) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_ana_int_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_ana_int_st`] module"]
pub type LP_ANA_INT_ST = crate::Reg<lp_ana_int_st::LP_ANA_INT_ST_SPEC>;
#[doc = "need_des"]
pub mod lp_ana_int_st;
#[doc = "LP_ANA_INT_ENA (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_ana_int_ena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_ana_int_ena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_ana_int_ena`] module"]
pub type LP_ANA_INT_ENA = crate::Reg<lp_ana_int_ena::LP_ANA_INT_ENA_SPEC>;
#[doc = "need_des"]
pub mod lp_ana_int_ena;
#[doc = "LP_ANA_INT_CLR (w) register accessor: need_des\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_ana_int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_ana_int_clr`] module"]
pub type LP_ANA_INT_CLR = crate::Reg<lp_ana_int_clr::LP_ANA_INT_CLR_SPEC>;
#[doc = "need_des"]
pub mod lp_ana_int_clr;
#[doc = "LP_ANA_LP_INT_RAW (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_ana_lp_int_raw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_ana_lp_int_raw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_ana_lp_int_raw`] module"]
pub type LP_ANA_LP_INT_RAW = crate::Reg<lp_ana_lp_int_raw::LP_ANA_LP_INT_RAW_SPEC>;
#[doc = "need_des"]
pub mod lp_ana_lp_int_raw;
#[doc = "LP_ANA_LP_INT_ST (r) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_ana_lp_int_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_ana_lp_int_st`] module"]
pub type LP_ANA_LP_INT_ST = crate::Reg<lp_ana_lp_int_st::LP_ANA_LP_INT_ST_SPEC>;
#[doc = "need_des"]
pub mod lp_ana_lp_int_st;
#[doc = "LP_ANA_LP_INT_ENA (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_ana_lp_int_ena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_ana_lp_int_ena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_ana_lp_int_ena`] module"]
pub type LP_ANA_LP_INT_ENA = crate::Reg<lp_ana_lp_int_ena::LP_ANA_LP_INT_ENA_SPEC>;
#[doc = "need_des"]
pub mod lp_ana_lp_int_ena;
#[doc = "LP_ANA_LP_INT_CLR (w) register accessor: need_des\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_ana_lp_int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_ana_lp_int_clr`] module"]
pub type LP_ANA_LP_INT_CLR = crate::Reg<lp_ana_lp_int_clr::LP_ANA_LP_INT_CLR_SPEC>;
#[doc = "need_des"]
pub mod lp_ana_lp_int_clr;
#[doc = "LP_ANA_DATE (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_ana_date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_ana_date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_ana_date`] module"]
pub type LP_ANA_DATE = crate::Reg<lp_ana_date::LP_ANA_DATE_SPEC>;
#[doc = "need_des"]
pub mod lp_ana_date;
