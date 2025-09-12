#[doc = "Register `_3_DMA_START` writer"]
pub type W = crate::W<_3_DMA_START_SPEC>;
#[doc = "Field `_3_DMA_START` writer - Start dma-sha3."]
pub type _3_DMA_START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<_3_DMA_START_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Start dma-sha3."]
    #[inline(always)]
    pub fn _3_dma_start(&mut self) -> _3_DMA_START_W<'_, _3_DMA_START_SPEC> {
        _3_DMA_START_W::new(self, 0)
    }
}
#[doc = "DMA configuration register 1.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_3_dma_start::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct _3_DMA_START_SPEC;
impl crate::RegisterSpec for _3_DMA_START_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`_3_dma_start::W`](W) writer structure"]
impl crate::Writable for _3_DMA_START_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets _3_DMA_START to value 0"]
impl crate::Resettable for _3_DMA_START_SPEC {}
