#[doc = "Register `DMA_RX_RESET` writer"]
pub type W = crate::W<DMA_RX_RESET_SPEC>;
#[doc = "Field `DMA_RX_RESET` writer - Write 1 to reset DMA RX FIFO"]
pub type DMA_RX_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DMA_RX_RESET_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Write 1 to reset DMA RX FIFO"]
    #[inline(always)]
    pub fn dma_rx_reset(&mut self) -> DMA_RX_RESET_W<'_, DMA_RX_RESET_SPEC> {
        DMA_RX_RESET_W::new(self, 0)
    }
}
#[doc = "DMA RX FIFO Reset Signal\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_rx_reset::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMA_RX_RESET_SPEC;
impl crate::RegisterSpec for DMA_RX_RESET_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dma_rx_reset::W`](W) writer structure"]
impl crate::Writable for DMA_RX_RESET_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA_RX_RESET to value 0"]
impl crate::Resettable for DMA_RX_RESET_SPEC {}
