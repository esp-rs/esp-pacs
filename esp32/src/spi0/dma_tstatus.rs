#[doc = "Register `DMA_TSTATUS` reader"]
pub struct R(crate::R<DMA_TSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_TSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_TSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_TSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DMA_IN_STATUS` reader - spi dma write data to memory status."]
pub type DMA_IN_STATUS_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - spi dma write data to memory status."]
    #[inline(always)]
    pub fn dma_in_status(&self) -> DMA_IN_STATUS_R {
        DMA_IN_STATUS_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_TSTATUS")
            .field(
                "dma_in_status",
                &format_args!("{}", self.dma_in_status().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DMA_TSTATUS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_tstatus](index.html) module"]
pub struct DMA_TSTATUS_SPEC;
impl crate::RegisterSpec for DMA_TSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_tstatus::R](R) reader structure"]
impl crate::Readable for DMA_TSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DMA_TSTATUS to value 0"]
impl crate::Resettable for DMA_TSTATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
