#[doc = "Register `RX_RESET` writer"]
pub type W = crate::W<RX_RESET_SPEC>;
#[doc = "Field `RX_RESET` writer - Set this bit to reset rx_fifo under dma_aes working mode."]
pub type RX_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RX_RESET_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to reset rx_fifo under dma_aes working mode."]
    #[inline(always)]
    pub fn rx_reset(&mut self) -> RX_RESET_W<RX_RESET_SPEC> {
        RX_RESET_W::new(self, 0)
    }
}
#[doc = "AES-DMA reset rx-fifo register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_reset::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RX_RESET_SPEC;
impl crate::RegisterSpec for RX_RESET_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`rx_reset::W`](W) writer structure"]
impl crate::Writable for RX_RESET_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RX_RESET to value 0"]
impl crate::Resettable for RX_RESET_SPEC {}
