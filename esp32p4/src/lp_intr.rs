#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    sw_int_raw: SW_INT_RAW,
    sw_int_st: SW_INT_ST,
    sw_int_ena: SW_INT_ENA,
    sw_int_clr: SW_INT_CLR,
    status: STATUS,
    _reserved5: [u8; 0x03e8],
    date: DATE,
}
impl RegisterBlock {
    #[doc = "0x00 - need_des"]
    #[inline(always)]
    pub const fn sw_int_raw(&self) -> &SW_INT_RAW {
        &self.sw_int_raw
    }
    #[doc = "0x04 - need_des"]
    #[inline(always)]
    pub const fn sw_int_st(&self) -> &SW_INT_ST {
        &self.sw_int_st
    }
    #[doc = "0x08 - need_des"]
    #[inline(always)]
    pub const fn sw_int_ena(&self) -> &SW_INT_ENA {
        &self.sw_int_ena
    }
    #[doc = "0x0c - need_des"]
    #[inline(always)]
    pub const fn sw_int_clr(&self) -> &SW_INT_CLR {
        &self.sw_int_clr
    }
    #[doc = "0x10 - need_des"]
    #[inline(always)]
    pub const fn status(&self) -> &STATUS {
        &self.status
    }
    #[doc = "0x3fc - need_des"]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
#[doc = "SW_INT_RAW (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sw_int_raw::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sw_int_raw::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sw_int_raw`] module"]
pub type SW_INT_RAW = crate::Reg<sw_int_raw::SW_INT_RAW_SPEC>;
#[doc = "need_des"]
pub mod sw_int_raw;
#[doc = "SW_INT_ST (r) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sw_int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sw_int_st`] module"]
pub type SW_INT_ST = crate::Reg<sw_int_st::SW_INT_ST_SPEC>;
#[doc = "need_des"]
pub mod sw_int_st;
#[doc = "SW_INT_ENA (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sw_int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sw_int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sw_int_ena`] module"]
pub type SW_INT_ENA = crate::Reg<sw_int_ena::SW_INT_ENA_SPEC>;
#[doc = "need_des"]
pub mod sw_int_ena;
#[doc = "SW_INT_CLR (w) register accessor: need_des\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sw_int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sw_int_clr`] module"]
pub type SW_INT_CLR = crate::Reg<sw_int_clr::SW_INT_CLR_SPEC>;
#[doc = "need_des"]
pub mod sw_int_clr;
#[doc = "STATUS (r) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`] module"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "need_des"]
pub mod status;
#[doc = "DATE (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "need_des"]
pub mod date;
