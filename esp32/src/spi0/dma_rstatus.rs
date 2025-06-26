#[doc = "Register `DMA_RSTATUS` reader"]
pub type R = crate::R<DMA_RSTATUS_SPEC>;
#[doc = "Field `DMA_OUT_STATUS` reader - spi dma read data from memory status."]
pub type DMA_OUT_STATUS_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - spi dma read data from memory status."]
    #[inline(always)]
    pub fn dma_out_status(&self) -> DMA_OUT_STATUS_R {
        DMA_OUT_STATUS_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_RSTATUS")
            .field("dma_out_status", &self.dma_out_status())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_rstatus::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMA_RSTATUS_SPEC;
impl crate::RegisterSpec for DMA_RSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_rstatus::R`](R) reader structure"]
impl crate::Readable for DMA_RSTATUS_SPEC {}
#[doc = "`reset()` method sets DMA_RSTATUS to value 0"]
impl crate::Resettable for DMA_RSTATUS_SPEC {}
