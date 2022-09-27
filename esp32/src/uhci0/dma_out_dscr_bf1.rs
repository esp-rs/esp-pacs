#[doc = "Register `DMA_OUT_DSCR_BF1` reader"]
pub struct R(crate::R<DMA_OUT_DSCR_BF1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_OUT_DSCR_BF1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_OUT_DSCR_BF1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_OUT_DSCR_BF1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `OUTLINK_DSCR_BF1` reader - The content of current out link descriptor's second dword"]
pub type OUTLINK_DSCR_BF1_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - The content of current out link descriptor's second dword"]
    #[inline(always)]
    pub fn outlink_dscr_bf1(&self) -> OUTLINK_DSCR_BF1_R {
        OUTLINK_DSCR_BF1_R::new(self.bits)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_out_dscr_bf1](index.html) module"]
pub struct DMA_OUT_DSCR_BF1_SPEC;
impl crate::RegisterSpec for DMA_OUT_DSCR_BF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_out_dscr_bf1::R](R) reader structure"]
impl crate::Readable for DMA_OUT_DSCR_BF1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DMA_OUT_DSCR_BF1 to value 0"]
impl crate::Resettable for DMA_OUT_DSCR_BF1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
