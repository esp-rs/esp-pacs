#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
///Cluster INT_CH%s, containing INT_RAW_CH?, INT_ST_CH?, INT_ENA_CH?, INT_CLR_CH?
pub struct INT_CH {
    raw: RAW,
    st: ST,
    ena: ENA,
    clr: CLR,
}
impl INT_CH {
    ///0x00 - DMA_INT_RAW_CH0_REG.
    #[inline(always)]
    pub const fn raw(&self) -> &RAW {
        &self.raw
    }
    ///0x04 - DMA_INT_ST_CH0_REG.
    #[inline(always)]
    pub const fn st(&self) -> &ST {
        &self.st
    }
    ///0x08 - DMA_INT_ENA_CH0_REG.
    #[inline(always)]
    pub const fn ena(&self) -> &ENA {
        &self.ena
    }
    ///0x0c - DMA_INT_CLR_CH0_REG.
    #[inline(always)]
    pub const fn clr(&self) -> &CLR {
        &self.clr
    }
}
/**RAW (rw) register accessor: DMA_INT_RAW_CH0_REG.

You can [`read`](crate::generic::Reg::read) this register and get [`raw::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`raw::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@raw`] module*/
pub type RAW = crate::Reg<raw::RAW_SPEC>;
///DMA_INT_RAW_CH0_REG.
pub mod raw;
/**ST (r) register accessor: DMA_INT_ST_CH0_REG.

You can [`read`](crate::generic::Reg::read) this register and get [`st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@st`] module*/
pub type ST = crate::Reg<st::ST_SPEC>;
///DMA_INT_ST_CH0_REG.
pub mod st;
/**ENA (rw) register accessor: DMA_INT_ENA_CH0_REG.

You can [`read`](crate::generic::Reg::read) this register and get [`ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ena`] module*/
pub type ENA = crate::Reg<ena::ENA_SPEC>;
///DMA_INT_ENA_CH0_REG.
pub mod ena;
/**CLR (w) register accessor: DMA_INT_CLR_CH0_REG.

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@clr`] module*/
pub type CLR = crate::Reg<clr::CLR_SPEC>;
///DMA_INT_CLR_CH0_REG.
pub mod clr;
