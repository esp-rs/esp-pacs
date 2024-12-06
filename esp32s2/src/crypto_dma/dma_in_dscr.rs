#[doc = "Register `DMA_IN_DSCR` reader"]
pub type R = crate::R<DMA_IN_DSCR_SPEC>;
#[doc = "Field `DMA_IN_DSCR` reader - Address of the current receive descriptor x."]
pub type DMA_IN_DSCR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Address of the current receive descriptor x."]
    #[inline(always)]
    pub fn dma_in_dscr(&self) -> DMA_IN_DSCR_R {
        DMA_IN_DSCR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_IN_DSCR")
            .field("dma_in_dscr", &self.dma_in_dscr())
            .finish()
    }
}
#[doc = "Address of current receive descriptor\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_in_dscr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMA_IN_DSCR_SPEC;
impl crate::RegisterSpec for DMA_IN_DSCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_in_dscr::R`](R) reader structure"]
impl crate::Readable for DMA_IN_DSCR_SPEC {}
#[doc = "`reset()` method sets DMA_IN_DSCR to value 0"]
impl crate::Resettable for DMA_IN_DSCR_SPEC {
    const RESET_VALUE: u32 = 0;
}
