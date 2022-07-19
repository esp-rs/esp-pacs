#[doc = "Register `DMA_BLOCK_NUM` reader"]
pub struct R(crate::R<DMA_BLOCK_NUM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_BLOCK_NUM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_BLOCK_NUM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_BLOCK_NUM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_BLOCK_NUM` writer"]
pub struct W(crate::W<DMA_BLOCK_NUM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_BLOCK_NUM_SPEC>;
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
impl From<crate::W<DMA_BLOCK_NUM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_BLOCK_NUM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMA_BLOCK_NUM` reader - Dma-sha block number."]
pub type DMA_BLOCK_NUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DMA_BLOCK_NUM` writer - Dma-sha block number."]
pub type DMA_BLOCK_NUM_W<'a> = crate::FieldWriter<'a, u32, DMA_BLOCK_NUM_SPEC, u8, u8, 6, 0>;
impl R {
    #[doc = "Bits 0:5 - Dma-sha block number."]
    #[inline(always)]
    pub fn dma_block_num(&self) -> DMA_BLOCK_NUM_R {
        DMA_BLOCK_NUM_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Dma-sha block number."]
    #[inline(always)]
    pub fn dma_block_num(&mut self) -> DMA_BLOCK_NUM_W {
        DMA_BLOCK_NUM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA configuration register 0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_block_num](index.html) module"]
pub struct DMA_BLOCK_NUM_SPEC;
impl crate::RegisterSpec for DMA_BLOCK_NUM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_block_num::R](R) reader structure"]
impl crate::Readable for DMA_BLOCK_NUM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_block_num::W](W) writer structure"]
impl crate::Writable for DMA_BLOCK_NUM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA_BLOCK_NUM to value 0"]
impl crate::Resettable for DMA_BLOCK_NUM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
