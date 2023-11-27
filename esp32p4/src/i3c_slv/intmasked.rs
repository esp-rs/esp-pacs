#[doc = "Register `INTMASKED` reader"]
pub type R = crate::R<INTMASKED_SPEC>;
#[doc = "Field `STOP_MASK` reader - Interrupt on STOP state on the bus. See Start as the preferred interrupt when needed. This interrupt may not trigger for quick STOP/START combination, as it relates to the state of being stopped."]
pub type STOP_MASK_R = crate::BitReader;
#[doc = "Field `RXPEND_MASK` reader - Interrupt when receiving a message from Master, which is not being handled by the block (excludes CCCs being handled automatically). If FIFO, then RX fullness trigger. If DMA, then message end."]
pub type RXPEND_MASK_R = crate::BitReader;
#[doc = "Field `TXSEND_MASK` reader - NA"]
pub type TXSEND_MASK_R = crate::BitReader;
impl R {
    #[doc = "Bit 10 - Interrupt on STOP state on the bus. See Start as the preferred interrupt when needed. This interrupt may not trigger for quick STOP/START combination, as it relates to the state of being stopped."]
    #[inline(always)]
    pub fn stop_mask(&self) -> STOP_MASK_R {
        STOP_MASK_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Interrupt when receiving a message from Master, which is not being handled by the block (excludes CCCs being handled automatically). If FIFO, then RX fullness trigger. If DMA, then message end."]
    #[inline(always)]
    pub fn rxpend_mask(&self) -> RXPEND_MASK_R {
        RXPEND_MASK_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - NA"]
    #[inline(always)]
    pub fn txsend_mask(&self) -> TXSEND_MASK_R {
        TXSEND_MASK_R::new(((self.bits >> 12) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTMASKED")
            .field("stop_mask", &format_args!("{}", self.stop_mask().bit()))
            .field("rxpend_mask", &format_args!("{}", self.rxpend_mask().bit()))
            .field("txsend_mask", &format_args!("{}", self.txsend_mask().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INTMASKED_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intmasked::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTMASKED_SPEC;
impl crate::RegisterSpec for INTMASKED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intmasked::R`](R) reader structure"]
impl crate::Readable for INTMASKED_SPEC {}
#[doc = "`reset()` method sets INTMASKED to value 0"]
impl crate::Resettable for INTMASKED_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
