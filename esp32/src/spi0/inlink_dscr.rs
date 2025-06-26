#[doc = "Register `INLINK_DSCR` reader"]
pub type R = crate::R<INLINK_DSCR_SPEC>;
#[doc = "Field `DMA_INLINK_DSCR` reader - The content of current in descriptor pointer."]
pub type DMA_INLINK_DSCR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The content of current in descriptor pointer."]
    #[inline(always)]
    pub fn dma_inlink_dscr(&self) -> DMA_INLINK_DSCR_R {
        DMA_INLINK_DSCR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INLINK_DSCR")
            .field("dma_inlink_dscr", &self.dma_inlink_dscr())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`inlink_dscr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INLINK_DSCR_SPEC;
impl crate::RegisterSpec for INLINK_DSCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inlink_dscr::R`](R) reader structure"]
impl crate::Readable for INLINK_DSCR_SPEC {}
#[doc = "`reset()` method sets INLINK_DSCR to value 0"]
impl crate::Resettable for INLINK_DSCR_SPEC {}
