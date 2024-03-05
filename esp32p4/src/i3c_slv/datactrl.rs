#[doc = "Register `DATACTRL` reader"]
pub type R = crate::R<DATACTRL_SPEC>;
#[doc = "Register `DATACTRL` writer"]
pub type W = crate::W<DATACTRL_SPEC>;
#[doc = "Field `FLUSHTB` writer - Flushes the from-bus buffer/FIFO. Not normally used"]
pub type FLUSHTB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLUSHFB` writer - Flushes the to-bus buffer/FIFO. Used when Master terminates a to-bus (read) message prematurely"]
pub type FLUSHFB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNLOCK` writer - If this bit is not written 1, the register bits from 7 to 4 are not changed on write."]
pub type UNLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXTRIG` reader - Trigger level for tx emptiness when FIFOed, Affects interrupt and DMA(if enabled). The defaults is 3"]
pub type TXTRIG_R = crate::FieldReader;
#[doc = "Field `TXTRIG` writer - Trigger level for tx emptiness when FIFOed, Affects interrupt and DMA(if enabled). The defaults is 3"]
pub type TXTRIG_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RXTRIG` reader - Trigger level for rx fulless when FIFOed, Affects interrupt and DMA(if enabled). The defaults is 3"]
pub type RXTRIG_R = crate::FieldReader;
#[doc = "Field `RXTRIG` writer - Trigger level for rx fulless when FIFOed, Affects interrupt and DMA(if enabled). The defaults is 3"]
pub type RXTRIG_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TXCOUNT` reader - NA"]
pub type TXCOUNT_R = crate::FieldReader;
#[doc = "Field `RXCOUNT` reader - NA"]
pub type RXCOUNT_R = crate::FieldReader;
#[doc = "Field `TXFULL` reader - NA"]
pub type TXFULL_R = crate::BitReader;
#[doc = "Field `RXEMPTY` reader - NA"]
pub type RXEMPTY_R = crate::BitReader;
impl R {
    #[doc = "Bits 4:5 - Trigger level for tx emptiness when FIFOed, Affects interrupt and DMA(if enabled). The defaults is 3"]
    #[inline(always)]
    pub fn txtrig(&self) -> TXTRIG_R {
        TXTRIG_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Trigger level for rx fulless when FIFOed, Affects interrupt and DMA(if enabled). The defaults is 3"]
    #[inline(always)]
    pub fn rxtrig(&self) -> RXTRIG_R {
        RXTRIG_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 16:20 - NA"]
    #[inline(always)]
    pub fn txcount(&self) -> TXCOUNT_R {
        TXCOUNT_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - NA"]
    #[inline(always)]
    pub fn rxcount(&self) -> RXCOUNT_R {
        RXCOUNT_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bit 30 - NA"]
    #[inline(always)]
    pub fn txfull(&self) -> TXFULL_R {
        TXFULL_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - NA"]
    #[inline(always)]
    pub fn rxempty(&self) -> RXEMPTY_R {
        RXEMPTY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DATACTRL")
            .field("txtrig", &format_args!("{}", self.txtrig().bits()))
            .field("rxtrig", &format_args!("{}", self.rxtrig().bits()))
            .field("txcount", &format_args!("{}", self.txcount().bits()))
            .field("rxcount", &format_args!("{}", self.rxcount().bits()))
            .field("txfull", &format_args!("{}", self.txfull().bit()))
            .field("rxempty", &format_args!("{}", self.rxempty().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DATACTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - Flushes the from-bus buffer/FIFO. Not normally used"]
    #[inline(always)]
    #[must_use]
    pub fn flushtb(&mut self) -> FLUSHTB_W<DATACTRL_SPEC> {
        FLUSHTB_W::new(self, 0)
    }
    #[doc = "Bit 1 - Flushes the to-bus buffer/FIFO. Used when Master terminates a to-bus (read) message prematurely"]
    #[inline(always)]
    #[must_use]
    pub fn flushfb(&mut self) -> FLUSHFB_W<DATACTRL_SPEC> {
        FLUSHFB_W::new(self, 1)
    }
    #[doc = "Bit 3 - If this bit is not written 1, the register bits from 7 to 4 are not changed on write."]
    #[inline(always)]
    #[must_use]
    pub fn unlock(&mut self) -> UNLOCK_W<DATACTRL_SPEC> {
        UNLOCK_W::new(self, 3)
    }
    #[doc = "Bits 4:5 - Trigger level for tx emptiness when FIFOed, Affects interrupt and DMA(if enabled). The defaults is 3"]
    #[inline(always)]
    #[must_use]
    pub fn txtrig(&mut self) -> TXTRIG_W<DATACTRL_SPEC> {
        TXTRIG_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Trigger level for rx fulless when FIFOed, Affects interrupt and DMA(if enabled). The defaults is 3"]
    #[inline(always)]
    #[must_use]
    pub fn rxtrig(&mut self) -> RXTRIG_W<DATACTRL_SPEC> {
        RXTRIG_W::new(self, 6)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`datactrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`datactrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DATACTRL_SPEC;
impl crate::RegisterSpec for DATACTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`datactrl::R`](R) reader structure"]
impl crate::Readable for DATACTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`datactrl::W`](W) writer structure"]
impl crate::Writable for DATACTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DATACTRL to value 0xb0"]
impl crate::Resettable for DATACTRL_SPEC {
    const RESET_VALUE: u32 = 0xb0;
}
