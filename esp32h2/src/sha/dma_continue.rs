#[doc = "Register `DMA_CONTINUE` writer"]
pub struct W(crate::W<DMA_CONTINUE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_CONTINUE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<DMA_CONTINUE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_CONTINUE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMA_CONTINUE` writer - Continue dma-sha."]
pub type DMA_CONTINUE_W<'a, const O: u8> = crate::BitWriter<'a, DMA_CONTINUE_SPEC, O>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DMA_CONTINUE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Continue dma-sha."]
    #[inline(always)]
    #[must_use]
    pub fn dma_continue(&mut self) -> DMA_CONTINUE_W<0> {
        DMA_CONTINUE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA configuration register 2.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_continue](index.html) module"]
pub struct DMA_CONTINUE_SPEC;
impl crate::RegisterSpec for DMA_CONTINUE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [dma_continue::W](W) writer structure"]
impl crate::Writable for DMA_CONTINUE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMA_CONTINUE to value 0"]
impl crate::Resettable for DMA_CONTINUE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
