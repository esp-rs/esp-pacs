#[doc = "Register `INTCLR` writer"]
pub type W = crate::W<INTCLR_SPEC>;
#[doc = "Field `STOP_CLR` writer - Interrupt on STOP state on the bus. See Start as the preferred interrupt when needed. This interrupt may not trigger for quick STOP/START combination, as it relates to the state of being stopped."]
pub type STOP_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXPEND_CLR` writer - Interrupt when receiving a message from Master, which is not being handled by the block (excludes CCCs being handled automatically). If FIFO, then RX fullness trigger. If DMA, then message end."]
pub type RXPEND_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXSEND_CLR` writer - NA"]
pub type TXSEND_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INTCLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 10 - Interrupt on STOP state on the bus. See Start as the preferred interrupt when needed. This interrupt may not trigger for quick STOP/START combination, as it relates to the state of being stopped."]
    #[inline(always)]
    pub fn stop_clr(&mut self) -> STOP_CLR_W<INTCLR_SPEC> {
        STOP_CLR_W::new(self, 10)
    }
    #[doc = "Bit 11 - Interrupt when receiving a message from Master, which is not being handled by the block (excludes CCCs being handled automatically). If FIFO, then RX fullness trigger. If DMA, then message end."]
    #[inline(always)]
    pub fn rxpend_clr(&mut self) -> RXPEND_CLR_W<INTCLR_SPEC> {
        RXPEND_CLR_W::new(self, 11)
    }
    #[doc = "Bit 12 - NA"]
    #[inline(always)]
    pub fn txsend_clr(&mut self) -> TXSEND_CLR_W<INTCLR_SPEC> {
        TXSEND_CLR_W::new(self, 12)
    }
}
#[doc = "NA\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTCLR_SPEC;
impl crate::RegisterSpec for INTCLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`intclr::W`](W) writer structure"]
impl crate::Writable for INTCLR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTCLR to value 0"]
impl crate::Resettable for INTCLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
