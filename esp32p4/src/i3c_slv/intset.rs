#[doc = "Register `INTSET` reader"]
pub type R = crate::R<INTSET_SPEC>;
#[doc = "Register `INTSET` writer"]
pub type W = crate::W<INTSET_SPEC>;
#[doc = "Field `STOP_ENA` reader - Interrupt on STOP state on the bus. See Start as the preferred interrupt when needed. This interrupt may not trigger for quick STOP/START combination, as it relates to the state of being stopped."]
pub type STOP_ENA_R = crate::BitReader;
#[doc = "Field `STOP_ENA` writer - Interrupt on STOP state on the bus. See Start as the preferred interrupt when needed. This interrupt may not trigger for quick STOP/START combination, as it relates to the state of being stopped."]
pub type STOP_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXPEND_ENA` reader - Interrupt when receiving a message from Master, which is not being handled by the block (excludes CCCs being handled automatically). If FIFO, then RX fullness trigger. If DMA, then message end."]
pub type RXPEND_ENA_R = crate::BitReader;
#[doc = "Field `RXPEND_ENA` writer - Interrupt when receiving a message from Master, which is not being handled by the block (excludes CCCs being handled automatically). If FIFO, then RX fullness trigger. If DMA, then message end."]
pub type RXPEND_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXSEND_ENA` reader - NA"]
pub type TXSEND_ENA_R = crate::BitReader;
#[doc = "Field `TXSEND_ENA` writer - NA"]
pub type TXSEND_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 10 - Interrupt on STOP state on the bus. See Start as the preferred interrupt when needed. This interrupt may not trigger for quick STOP/START combination, as it relates to the state of being stopped."]
    #[inline(always)]
    pub fn stop_ena(&self) -> STOP_ENA_R {
        STOP_ENA_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Interrupt when receiving a message from Master, which is not being handled by the block (excludes CCCs being handled automatically). If FIFO, then RX fullness trigger. If DMA, then message end."]
    #[inline(always)]
    pub fn rxpend_ena(&self) -> RXPEND_ENA_R {
        RXPEND_ENA_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - NA"]
    #[inline(always)]
    pub fn txsend_ena(&self) -> TXSEND_ENA_R {
        TXSEND_ENA_R::new(((self.bits >> 12) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTSET")
            .field("stop_ena", &self.stop_ena())
            .field("rxpend_ena", &self.rxpend_ena())
            .field("txsend_ena", &self.txsend_ena())
            .finish()
    }
}
impl W {
    #[doc = "Bit 10 - Interrupt on STOP state on the bus. See Start as the preferred interrupt when needed. This interrupt may not trigger for quick STOP/START combination, as it relates to the state of being stopped."]
    #[inline(always)]
    pub fn stop_ena(&mut self) -> STOP_ENA_W<INTSET_SPEC> {
        STOP_ENA_W::new(self, 10)
    }
    #[doc = "Bit 11 - Interrupt when receiving a message from Master, which is not being handled by the block (excludes CCCs being handled automatically). If FIFO, then RX fullness trigger. If DMA, then message end."]
    #[inline(always)]
    pub fn rxpend_ena(&mut self) -> RXPEND_ENA_W<INTSET_SPEC> {
        RXPEND_ENA_W::new(self, 11)
    }
    #[doc = "Bit 12 - NA"]
    #[inline(always)]
    pub fn txsend_ena(&mut self) -> TXSEND_ENA_W<INTSET_SPEC> {
        TXSEND_ENA_W::new(self, 12)
    }
}
#[doc = "INSET allows setting enables for interrupts(connecting the corresponding STATUS source to causing an IRQ to the processor)\n\nYou can [`read`](crate::Reg::read) this register and get [`intset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTSET_SPEC;
impl crate::RegisterSpec for INTSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intset::R`](R) reader structure"]
impl crate::Readable for INTSET_SPEC {}
#[doc = "`write(|w| ..)` method takes [`intset::W`](W) writer structure"]
impl crate::Writable for INTSET_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTSET to value 0"]
impl crate::Resettable for INTSET_SPEC {
    const RESET_VALUE: u32 = 0;
}
