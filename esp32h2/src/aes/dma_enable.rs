#[doc = "Register `DMA_ENABLE` reader"]
pub struct R(crate::R<DMA_ENABLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_ENABLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_ENABLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_ENABLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_ENABLE` writer"]
pub struct W(crate::W<DMA_ENABLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_ENABLE_SPEC>;
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
impl From<crate::W<DMA_ENABLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_ENABLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMA_ENABLE` reader - 1'b0: typical AES working mode, 1'b1: DMA-AES working mode."]
pub type DMA_ENABLE_R = crate::BitReader;
#[doc = "Field `DMA_ENABLE` writer - 1'b0: typical AES working mode, 1'b1: DMA-AES working mode."]
pub type DMA_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, DMA_ENABLE_SPEC, O>;
impl R {
    #[doc = "Bit 0 - 1'b0: typical AES working mode, 1'b1: DMA-AES working mode."]
    #[inline(always)]
    pub fn dma_enable(&self) -> DMA_ENABLE_R {
        DMA_ENABLE_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_ENABLE")
            .field("dma_enable", &format_args!("{}", self.dma_enable().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DMA_ENABLE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - 1'b0: typical AES working mode, 1'b1: DMA-AES working mode."]
    #[inline(always)]
    #[must_use]
    pub fn dma_enable(&mut self) -> DMA_ENABLE_W<0> {
        DMA_ENABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA-AES working mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_enable](index.html) module"]
pub struct DMA_ENABLE_SPEC;
impl crate::RegisterSpec for DMA_ENABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_enable::R](R) reader structure"]
impl crate::Readable for DMA_ENABLE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_enable::W](W) writer structure"]
impl crate::Writable for DMA_ENABLE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMA_ENABLE to value 0"]
impl crate::Resettable for DMA_ENABLE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
