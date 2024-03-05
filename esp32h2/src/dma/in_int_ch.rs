#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Cluster IN_INT_CH%s, containing IN_INT_RAW_CH?, IN_INT_ST_CH?, IN_INT_ENA_CH?, IN_INT_CLR_CH?"]
pub struct IN_INT_CH {
    in_int_raw: IN_INT_RAW,
    in_int_st: IN_INT_ST,
    in_int_ena: IN_INT_ENA,
    in_int_clr: IN_INT_CLR,
}
impl IN_INT_CH {
    #[doc = "0x00 - Raw status interrupt of channel 0"]
    #[inline(always)]
    pub const fn in_int_raw(&self) -> &IN_INT_RAW {
        &self.in_int_raw
    }
    #[doc = "0x04 - Masked interrupt of channel 0"]
    #[inline(always)]
    pub const fn in_int_st(&self) -> &IN_INT_ST {
        &self.in_int_st
    }
    #[doc = "0x08 - Interrupt enable bits of channel 0"]
    #[inline(always)]
    pub const fn in_int_ena(&self) -> &IN_INT_ENA {
        &self.in_int_ena
    }
    #[doc = "0x0c - Interrupt clear bits of channel 0"]
    #[inline(always)]
    pub const fn in_int_clr(&self) -> &IN_INT_CLR {
        &self.in_int_clr
    }
}
#[doc = "IN_INT_RAW (rw) register accessor: Raw status interrupt of channel 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_int_raw::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_int_raw::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_int_raw`] module"]
pub type IN_INT_RAW = crate::Reg<in_int_raw::IN_INT_RAW_SPEC>;
#[doc = "Raw status interrupt of channel 0"]
pub mod in_int_raw;
#[doc = "IN_INT_ST (r) register accessor: Masked interrupt of channel 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_int_st`] module"]
pub type IN_INT_ST = crate::Reg<in_int_st::IN_INT_ST_SPEC>;
#[doc = "Masked interrupt of channel 0"]
pub mod in_int_st;
#[doc = "IN_INT_ENA (rw) register accessor: Interrupt enable bits of channel 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_int_ena`] module"]
pub type IN_INT_ENA = crate::Reg<in_int_ena::IN_INT_ENA_SPEC>;
#[doc = "Interrupt enable bits of channel 0"]
pub mod in_int_ena;
#[doc = "IN_INT_CLR (w) register accessor: Interrupt clear bits of channel 0\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_int_clr`] module"]
pub type IN_INT_CLR = crate::Reg<in_int_clr::IN_INT_CLR_SPEC>;
#[doc = "Interrupt clear bits of channel 0"]
pub mod in_int_clr;
