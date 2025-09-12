#[doc = "Register `_3_DMA_CONTINUE` writer"]
pub type W = crate::W<_3_DMA_CONTINUE_SPEC>;
#[doc = "Field `_3_DMA_CONTINUE` writer - Continue dma-sha3."]
pub type _3_DMA_CONTINUE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<_3_DMA_CONTINUE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Continue dma-sha3."]
    #[inline(always)]
    pub fn _3_dma_continue(&mut self) -> _3_DMA_CONTINUE_W<'_, _3_DMA_CONTINUE_SPEC> {
        _3_DMA_CONTINUE_W::new(self, 0)
    }
}
#[doc = "DMA configuration register 2.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_3_dma_continue::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct _3_DMA_CONTINUE_SPEC;
impl crate::RegisterSpec for _3_DMA_CONTINUE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`_3_dma_continue::W`](W) writer structure"]
impl crate::Writable for _3_DMA_CONTINUE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets _3_DMA_CONTINUE to value 0"]
impl crate::Resettable for _3_DMA_CONTINUE_SPEC {}
