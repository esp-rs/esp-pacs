#[doc = "Register `DMA_START` writer"]
pub struct W(crate::W<DMA_START_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_START_SPEC>;
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
impl From<crate::W<DMA_START_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_START_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMA_START` writer - start dma-sha"]
pub type DMA_START_W<'a, const O: u8> = crate::BitWriter<'a, DMA_START_SPEC, O>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DMA_START_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - start dma-sha"]
    #[inline(always)]
    #[must_use]
    pub fn dma_start(&mut self) -> DMA_START_W<0> {
        DMA_START_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA configuration register 1.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_start](index.html) module"]
pub struct DMA_START_SPEC;
impl crate::RegisterSpec for DMA_START_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [dma_start::W](W) writer structure"]
impl crate::Writable for DMA_START_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMA_START to value 0"]
impl crate::Resettable for DMA_START_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
