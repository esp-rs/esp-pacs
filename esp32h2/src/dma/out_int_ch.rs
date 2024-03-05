#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Cluster OUT_INT_CH%s, containing OUT_INT_RAW_CH?, OUT_INT_ST_CH?, OUT_INT_ENA_CH?, OUT_INT_CLR_CH?"]
pub struct OUT_INT_CH {
    out_int_raw: OUT_INT_RAW,
    out_int_st: OUT_INT_ST,
    out_int_ena: OUT_INT_ENA,
    out_int_clr: OUT_INT_CLR,
}
impl OUT_INT_CH {
    #[doc = "0x00 - Raw status interrupt of channel 0"]
    #[inline(always)]
    pub const fn out_int_raw(&self) -> &OUT_INT_RAW {
        &self.out_int_raw
    }
    #[doc = "0x04 - Masked interrupt of channel 0"]
    #[inline(always)]
    pub const fn out_int_st(&self) -> &OUT_INT_ST {
        &self.out_int_st
    }
    #[doc = "0x08 - Interrupt enable bits of channel 0"]
    #[inline(always)]
    pub const fn out_int_ena(&self) -> &OUT_INT_ENA {
        &self.out_int_ena
    }
    #[doc = "0x0c - Interrupt clear bits of channel 0"]
    #[inline(always)]
    pub const fn out_int_clr(&self) -> &OUT_INT_CLR {
        &self.out_int_clr
    }
}
#[doc = "OUT_INT_RAW (rw) register accessor: Raw status interrupt of channel 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_int_raw::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_int_raw::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_int_raw`] module"]
pub type OUT_INT_RAW = crate::Reg<out_int_raw::OUT_INT_RAW_SPEC>;
#[doc = "Raw status interrupt of channel 0"]
pub mod out_int_raw;
#[doc = "OUT_INT_ST (r) register accessor: Masked interrupt of channel 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_int_st`] module"]
pub type OUT_INT_ST = crate::Reg<out_int_st::OUT_INT_ST_SPEC>;
#[doc = "Masked interrupt of channel 0"]
pub mod out_int_st;
#[doc = "OUT_INT_ENA (rw) register accessor: Interrupt enable bits of channel 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_int_ena`] module"]
pub type OUT_INT_ENA = crate::Reg<out_int_ena::OUT_INT_ENA_SPEC>;
#[doc = "Interrupt enable bits of channel 0"]
pub mod out_int_ena;
#[doc = "OUT_INT_CLR (w) register accessor: Interrupt clear bits of channel 0\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_int_clr`] module"]
pub type OUT_INT_CLR = crate::Reg<out_int_clr::OUT_INT_CLR_SPEC>;
#[doc = "Interrupt clear bits of channel 0"]
pub mod out_int_clr;
