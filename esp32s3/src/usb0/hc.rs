#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Cluster HC%s, containing HCCHAR?, HCINT?, HCINTMSK?, HCTSIZ?, HCDMA?, HCDMAB?"]
pub struct HC {
    char: CHAR,
    _reserved1: [u8; 0x04],
    int: INT,
    intmsk: INTMSK,
    tsiz: TSIZ,
    dma: DMA,
    _reserved5: [u8; 0x04],
    dmab: DMAB,
}
impl HC {
    #[doc = "0x00 - "]
    #[inline(always)]
    pub const fn char(&self) -> &CHAR {
        &self.char
    }
    #[doc = "0x08 - "]
    #[inline(always)]
    pub const fn int(&self) -> &INT {
        &self.int
    }
    #[doc = "0x0c - "]
    #[inline(always)]
    pub const fn intmsk(&self) -> &INTMSK {
        &self.intmsk
    }
    #[doc = "0x10 - "]
    #[inline(always)]
    pub const fn tsiz(&self) -> &TSIZ {
        &self.tsiz
    }
    #[doc = "0x14 - "]
    #[inline(always)]
    pub const fn dma(&self) -> &DMA {
        &self.dma
    }
    #[doc = "0x1c - "]
    #[inline(always)]
    pub const fn dmab(&self) -> &DMAB {
        &self.dmab
    }
}
#[doc = "CHAR (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`char::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`char::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@char`] module"]
pub type CHAR = crate::Reg<char::CHAR_SPEC>;
#[doc = ""]
pub mod char;
#[doc = "INT (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int`] module"]
pub type INT = crate::Reg<int::INT_SPEC>;
#[doc = ""]
pub mod int;
#[doc = "INTMSK (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intmsk`] module"]
pub type INTMSK = crate::Reg<intmsk::INTMSK_SPEC>;
#[doc = ""]
pub mod intmsk;
#[doc = "TSIZ (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tsiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsiz`] module"]
pub type TSIZ = crate::Reg<tsiz::TSIZ_SPEC>;
#[doc = ""]
pub mod tsiz;
#[doc = "DMA (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma`] module"]
pub type DMA = crate::Reg<dma::DMA_SPEC>;
#[doc = ""]
pub mod dma;
#[doc = "DMAB (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmab::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmab`] module"]
pub type DMAB = crate::Reg<dmab::DMAB_SPEC>;
#[doc = ""]
pub mod dmab;
