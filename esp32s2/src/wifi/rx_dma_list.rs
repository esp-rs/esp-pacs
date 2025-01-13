#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "RX_DMA_LIST"]
pub struct RX_DMA_LIST {
    rx_descr_base: RX_DESCR_BASE,
    rx_descr_next: RX_DESCR_NEXT,
    rx_descr_last: RX_DESCR_LAST,
}
impl RX_DMA_LIST {
    #[doc = "0x00 - base address of the RX DMA list"]
    #[inline(always)]
    pub const fn rx_descr_base(&self) -> &RX_DESCR_BASE {
        &self.rx_descr_base
    }
    #[doc = "0x04 - next item in the RX DMA list"]
    #[inline(always)]
    pub const fn rx_descr_next(&self) -> &RX_DESCR_NEXT {
        &self.rx_descr_next
    }
    #[doc = "0x08 - last item in RX DMA list"]
    #[inline(always)]
    pub const fn rx_descr_last(&self) -> &RX_DESCR_LAST {
        &self.rx_descr_last
    }
}
#[doc = "RX_DESCR_BASE (rw) register accessor: base address of the RX DMA list\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_descr_base::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_descr_base::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_descr_base`] module"]
pub type RX_DESCR_BASE = crate::Reg<rx_descr_base::RX_DESCR_BASE_SPEC>;
#[doc = "base address of the RX DMA list"]
pub mod rx_descr_base;
#[doc = "RX_DESCR_NEXT (rw) register accessor: next item in the RX DMA list\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_descr_next::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_descr_next::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_descr_next`] module"]
pub type RX_DESCR_NEXT = crate::Reg<rx_descr_next::RX_DESCR_NEXT_SPEC>;
#[doc = "next item in the RX DMA list"]
pub mod rx_descr_next;
#[doc = "RX_DESCR_LAST (rw) register accessor: last item in RX DMA list\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_descr_last::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_descr_last::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_descr_last`] module"]
pub type RX_DESCR_LAST = crate::Reg<rx_descr_last::RX_DESCR_LAST_SPEC>;
#[doc = "last item in RX DMA list"]
pub mod rx_descr_last;
