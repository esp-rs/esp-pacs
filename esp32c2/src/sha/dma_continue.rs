#[doc = "Register `DMA_CONTINUE` writer"]
pub type W = crate::W<DMA_CONTINUE_SPEC>;
#[doc = "Field `DMA_CONTINUE` writer - Continue dma-sha."]
pub type DMA_CONTINUE_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    pub fn dma_continue(&mut self) -> DMA_CONTINUE_W<DMA_CONTINUE_SPEC> {
        DMA_CONTINUE_W::new(self, 0)
    }
}
#[doc = "DMA configuration register 2.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_continue::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMA_CONTINUE_SPEC;
impl crate::RegisterSpec for DMA_CONTINUE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dma_continue::W`](W) writer structure"]
impl crate::Writable for DMA_CONTINUE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMA_CONTINUE to value 0"]
impl crate::Resettable for DMA_CONTINUE_SPEC {
    const RESET_VALUE: u32 = 0;
}
