#[doc = "Register `OUTLINK_DSCR_BF0` reader"]
pub type R = crate::R<OUTLINK_DSCR_BF0_SPEC>;
#[doc = "Field `DMA_OUTLINK_DSCR_BF0` reader - The content of next out descriptor pointer."]
pub type DMA_OUTLINK_DSCR_BF0_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The content of next out descriptor pointer."]
    #[inline(always)]
    pub fn dma_outlink_dscr_bf0(&self) -> DMA_OUTLINK_DSCR_BF0_R {
        DMA_OUTLINK_DSCR_BF0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUTLINK_DSCR_BF0")
            .field("dma_outlink_dscr_bf0", &self.dma_outlink_dscr_bf0())
            .finish()
    }
}
#[doc = "Next SPI DMA TX descriptor pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`outlink_dscr_bf0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUTLINK_DSCR_BF0_SPEC;
impl crate::RegisterSpec for OUTLINK_DSCR_BF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`outlink_dscr_bf0::R`](R) reader structure"]
impl crate::Readable for OUTLINK_DSCR_BF0_SPEC {}
#[doc = "`reset()` method sets OUTLINK_DSCR_BF0 to value 0"]
impl crate::Resettable for OUTLINK_DSCR_BF0_SPEC {}
