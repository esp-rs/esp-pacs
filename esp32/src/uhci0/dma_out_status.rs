#[doc = "Register `DMA_OUT_STATUS` reader"]
pub struct R(crate::R<DMA_OUT_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_OUT_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_OUT_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_OUT_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `OUT_FULL` reader - 1:DMA out link descriptor's fifo is full."]
pub type OUT_FULL_R = crate::BitReader;
#[doc = "Field `OUT_EMPTY` reader - 1:DMA in link descriptor's fifo is empty."]
pub type OUT_EMPTY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - 1:DMA out link descriptor's fifo is full."]
    #[inline(always)]
    pub fn out_full(&self) -> OUT_FULL_R {
        OUT_FULL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:DMA in link descriptor's fifo is empty."]
    #[inline(always)]
    pub fn out_empty(&self) -> OUT_EMPTY_R {
        OUT_EMPTY_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_OUT_STATUS")
            .field("out_full", &format_args!("{}", self.out_full().bit()))
            .field("out_empty", &format_args!("{}", self.out_empty().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DMA_OUT_STATUS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_out_status](index.html) module"]
pub struct DMA_OUT_STATUS_SPEC;
impl crate::RegisterSpec for DMA_OUT_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_out_status::R](R) reader structure"]
impl crate::Readable for DMA_OUT_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DMA_OUT_STATUS to value 0x02"]
impl crate::Resettable for DMA_OUT_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
