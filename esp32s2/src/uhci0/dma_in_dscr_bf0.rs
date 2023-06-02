#[doc = "Register `DMA_IN_DSCR_BF0` reader"]
pub struct R(crate::R<DMA_IN_DSCR_BF0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_IN_DSCR_BF0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_IN_DSCR_BF0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_IN_DSCR_BF0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `INLINK_DSCR_BF0` reader - This register stores the third word of the current receive descriptor."]
pub type INLINK_DSCR_BF0_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This register stores the third word of the current receive descriptor."]
    #[inline(always)]
    pub fn inlink_dscr_bf0(&self) -> INLINK_DSCR_BF0_R {
        INLINK_DSCR_BF0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_IN_DSCR_BF0")
            .field(
                "inlink_dscr_bf0",
                &format_args!("{}", self.inlink_dscr_bf0().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DMA_IN_DSCR_BF0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "The third word of current receive descriptor\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_in_dscr_bf0](index.html) module"]
pub struct DMA_IN_DSCR_BF0_SPEC;
impl crate::RegisterSpec for DMA_IN_DSCR_BF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_in_dscr_bf0::R](R) reader structure"]
impl crate::Readable for DMA_IN_DSCR_BF0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DMA_IN_DSCR_BF0 to value 0"]
impl crate::Resettable for DMA_IN_DSCR_BF0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
