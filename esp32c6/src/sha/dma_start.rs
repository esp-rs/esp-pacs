#[doc = "Register `DMA_START` writer"]
pub type W = crate::W<DMA_START_SPEC>;
#[doc = "Field `DMA_START` writer - Start dma-sha."]
pub type DMA_START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DMA_START_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Start dma-sha."]
    #[inline(always)]
    #[must_use]
    pub fn dma_start(&mut self) -> DMA_START_W<DMA_START_SPEC> {
        DMA_START_W::new(self, 0)
    }
}
#[doc = "DMA configuration register 1.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_start::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMA_START_SPEC;
impl crate::RegisterSpec for DMA_START_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dma_start::W`](W) writer structure"]
impl crate::Writable for DMA_START_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMA_START to value 0"]
impl crate::Resettable for DMA_START_SPEC {
    const RESET_VALUE: u32 = 0;
}
