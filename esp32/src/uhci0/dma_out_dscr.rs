#[doc = "Register `DMA_OUT_DSCR` reader"]
pub type R = crate::R<DMA_OUT_DSCR_SPEC>;
#[doc = "Field `OUTLINK_DSCR` reader - The content of current out link descriptor's third dword"]
pub type OUTLINK_DSCR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The content of current out link descriptor's third dword"]
    #[inline(always)]
    pub fn outlink_dscr(&self) -> OUTLINK_DSCR_R {
        OUTLINK_DSCR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_OUT_DSCR")
            .field("outlink_dscr", &self.outlink_dscr().bits())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DMA_OUT_DSCR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_out_dscr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMA_OUT_DSCR_SPEC;
impl crate::RegisterSpec for DMA_OUT_DSCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_out_dscr::R`](R) reader structure"]
impl crate::Readable for DMA_OUT_DSCR_SPEC {}
#[doc = "`reset()` method sets DMA_OUT_DSCR to value 0"]
impl crate::Resettable for DMA_OUT_DSCR_SPEC {
    const RESET_VALUE: u32 = 0;
}
