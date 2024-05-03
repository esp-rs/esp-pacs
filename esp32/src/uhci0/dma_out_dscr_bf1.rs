#[doc = "Register `DMA_OUT_DSCR_BF1` reader"]
pub type R = crate::R<DMA_OUT_DSCR_BF1_SPEC>;
#[doc = "Field `OUTLINK_DSCR_BF1` reader - The content of current out link descriptor's second dword"]
pub type OUTLINK_DSCR_BF1_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The content of current out link descriptor's second dword"]
    #[inline(always)]
    pub fn outlink_dscr_bf1(&self) -> OUTLINK_DSCR_BF1_R {
        OUTLINK_DSCR_BF1_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_OUT_DSCR_BF1")
            .field("outlink_dscr_bf1", &self.outlink_dscr_bf1().bits())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DMA_OUT_DSCR_BF1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_out_dscr_bf1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMA_OUT_DSCR_BF1_SPEC;
impl crate::RegisterSpec for DMA_OUT_DSCR_BF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_out_dscr_bf1::R`](R) reader structure"]
impl crate::Readable for DMA_OUT_DSCR_BF1_SPEC {}
#[doc = "`reset()` method sets DMA_OUT_DSCR_BF1 to value 0"]
impl crate::Resettable for DMA_OUT_DSCR_BF1_SPEC {
    const RESET_VALUE: u32 = 0;
}
