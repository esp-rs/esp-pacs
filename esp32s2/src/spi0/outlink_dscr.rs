#[doc = "Register `OUTLINK_DSCR` reader"]
pub struct R(crate::R<OUTLINK_DSCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUTLINK_DSCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OUTLINK_DSCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OUTLINK_DSCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DMA_OUTLINK_DSCR` reader - The content of current out descriptor pointer."]
pub type DMA_OUTLINK_DSCR_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - The content of current out descriptor pointer."]
    #[inline(always)]
    pub fn dma_outlink_dscr(&self) -> DMA_OUTLINK_DSCR_R {
        DMA_OUTLINK_DSCR_R::new(self.bits)
    }
}
#[doc = "Current SPI DMA TX descriptor pointer\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [outlink_dscr](index.html) module"]
pub struct OUTLINK_DSCR_SPEC;
impl crate::RegisterSpec for OUTLINK_DSCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [outlink_dscr::R](R) reader structure"]
impl crate::Readable for OUTLINK_DSCR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets OUTLINK_DSCR to value 0"]
impl crate::Resettable for OUTLINK_DSCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
