#[doc = "Register `TX_RESET` writer"]
pub type W = crate::W<TX_RESET_SPEC>;
#[doc = "Field `TX_RESET` writer - Set this bit to reset tx_fifo under dma_aes working mode."]
pub type TX_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TX_RESET_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to reset tx_fifo under dma_aes working mode."]
    #[inline(always)]
    pub fn tx_reset(&mut self) -> TX_RESET_W<'_, TX_RESET_SPEC> {
        TX_RESET_W::new(self, 0)
    }
}
#[doc = "AES-DMA reset tx-fifo register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_reset::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TX_RESET_SPEC;
impl crate::RegisterSpec for TX_RESET_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tx_reset::W`](W) writer structure"]
impl crate::Writable for TX_RESET_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TX_RESET to value 0"]
impl crate::Resettable for TX_RESET_SPEC {}
