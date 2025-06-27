#[doc = "Register `DMA_OUT_DSCR` reader"]
pub type R = crate::R<DMA_OUT_DSCR_SPEC>;
#[doc = "Field `DMA_OUT_DSCR` reader - Address of the current transmit descriptor y."]
pub type DMA_OUT_DSCR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Address of the current transmit descriptor y."]
    #[inline(always)]
    pub fn dma_out_dscr(&self) -> DMA_OUT_DSCR_R {
        DMA_OUT_DSCR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_OUT_DSCR")
            .field("dma_out_dscr", &self.dma_out_dscr())
            .finish()
    }
}
#[doc = "Address of current transmit descriptor\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_out_dscr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMA_OUT_DSCR_SPEC;
impl crate::RegisterSpec for DMA_OUT_DSCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_out_dscr::R`](R) reader structure"]
impl crate::Readable for DMA_OUT_DSCR_SPEC {}
#[doc = "`reset()` method sets DMA_OUT_DSCR to value 0"]
impl crate::Resettable for DMA_OUT_DSCR_SPEC {}
