#[doc = "Register `BLEINTCNTL` reader"]
pub type R = crate::R<BLEINTCNTL_SPEC>;
#[doc = "Register `BLEINTCNTL` writer"]
pub type W = crate::W<BLEINTCNTL_SPEC>;
#[doc = "Field `CSCNTINTMSK` reader - 625μs base time (for active modes) interrupt mask"]
pub type CSCNTINTMSK_R = crate::BitReader;
#[doc = "Field `CSCNTINTMSK` writer - 625μs base time (for active modes) interrupt mask"]
pub type CSCNTINTMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXINTMSK` reader - RX interrupt mask"]
pub type RXINTMSK_R = crate::BitReader;
#[doc = "Field `RXINTMSK` writer - RX interrupt mask"]
pub type RXINTMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLPINTMSK` reader - Sleep mode interrupt mask"]
pub type SLPINTMSK_R = crate::BitReader;
#[doc = "Field `SLPINTMSK` writer - Sleep mode interrupt mask"]
pub type SLPINTMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVENTINTMSK` reader - End of event (adv, scan, conn) interrupt mask"]
pub type EVENTINTMSK_R = crate::BitReader;
#[doc = "Field `EVENTINTMSK` writer - End of event (adv, scan, conn) interrupt mask"]
pub type EVENTINTMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRYPTINTMSK` reader - Encryption engine interrupt mask"]
pub type CRYPTINTMSK_R = crate::BitReader;
#[doc = "Field `CRYPTINTMSK` writer - Encryption engine interrupt mask"]
pub type CRYPTINTMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRORINTMSK` reader - Error interrupt mask"]
pub type ERRORINTMSK_R = crate::BitReader;
#[doc = "Field `ERRORINTMSK` writer - Error interrupt mask"]
pub type ERRORINTMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GROSSTGTIMINTMSK` reader - Gross target timer interrupt mask"]
pub type GROSSTGTIMINTMSK_R = crate::BitReader;
#[doc = "Field `GROSSTGTIMINTMSK` writer - Gross target timer interrupt mask"]
pub type GROSSTGTIMINTMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FINETGTIMINTMSK` reader - Fine target timer interrupt mask"]
pub type FINETGTIMINTMSK_R = crate::BitReader;
#[doc = "Field `FINETGTIMINTMSK` writer - Fine target timer interrupt mask"]
pub type FINETGTIMINTMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVENTAPFAINTMSK` reader - End of event/anticipated pre-fetch abort interrupt mask"]
pub type EVENTAPFAINTMSK_R = crate::BitReader;
#[doc = "Field `EVENTAPFAINTMSK` writer - End of event/anticipated pre-fetch abort interrupt mask"]
pub type EVENTAPFAINTMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWINTMSK` reader - Software triggered interrupt mask"]
pub type SWINTMSK_R = crate::BitReader;
#[doc = "Field `SWINTMSK` writer - Software triggered interrupt mask"]
pub type SWINTMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 625μs base time (for active modes) interrupt mask"]
    #[inline(always)]
    pub fn cscntintmsk(&self) -> CSCNTINTMSK_R {
        CSCNTINTMSK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RX interrupt mask"]
    #[inline(always)]
    pub fn rxintmsk(&self) -> RXINTMSK_R {
        RXINTMSK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Sleep mode interrupt mask"]
    #[inline(always)]
    pub fn slpintmsk(&self) -> SLPINTMSK_R {
        SLPINTMSK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - End of event (adv, scan, conn) interrupt mask"]
    #[inline(always)]
    pub fn eventintmsk(&self) -> EVENTINTMSK_R {
        EVENTINTMSK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Encryption engine interrupt mask"]
    #[inline(always)]
    pub fn cryptintmsk(&self) -> CRYPTINTMSK_R {
        CRYPTINTMSK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Error interrupt mask"]
    #[inline(always)]
    pub fn errorintmsk(&self) -> ERRORINTMSK_R {
        ERRORINTMSK_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Gross target timer interrupt mask"]
    #[inline(always)]
    pub fn grosstgtimintmsk(&self) -> GROSSTGTIMINTMSK_R {
        GROSSTGTIMINTMSK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Fine target timer interrupt mask"]
    #[inline(always)]
    pub fn finetgtimintmsk(&self) -> FINETGTIMINTMSK_R {
        FINETGTIMINTMSK_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - End of event/anticipated pre-fetch abort interrupt mask"]
    #[inline(always)]
    pub fn eventapfaintmsk(&self) -> EVENTAPFAINTMSK_R {
        EVENTAPFAINTMSK_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Software triggered interrupt mask"]
    #[inline(always)]
    pub fn swintmsk(&self) -> SWINTMSK_R {
        SWINTMSK_R::new(((self.bits >> 9) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLEINTCNTL")
            .field("cscntintmsk", &self.cscntintmsk())
            .field("rxintmsk", &self.rxintmsk())
            .field("slpintmsk", &self.slpintmsk())
            .field("eventintmsk", &self.eventintmsk())
            .field("cryptintmsk", &self.cryptintmsk())
            .field("errorintmsk", &self.errorintmsk())
            .field("grosstgtimintmsk", &self.grosstgtimintmsk())
            .field("finetgtimintmsk", &self.finetgtimintmsk())
            .field("eventapfaintmsk", &self.eventapfaintmsk())
            .field("swintmsk", &self.swintmsk())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - 625μs base time (for active modes) interrupt mask"]
    #[inline(always)]
    pub fn cscntintmsk(&mut self) -> CSCNTINTMSK_W<'_, BLEINTCNTL_SPEC> {
        CSCNTINTMSK_W::new(self, 0)
    }
    #[doc = "Bit 1 - RX interrupt mask"]
    #[inline(always)]
    pub fn rxintmsk(&mut self) -> RXINTMSK_W<'_, BLEINTCNTL_SPEC> {
        RXINTMSK_W::new(self, 1)
    }
    #[doc = "Bit 2 - Sleep mode interrupt mask"]
    #[inline(always)]
    pub fn slpintmsk(&mut self) -> SLPINTMSK_W<'_, BLEINTCNTL_SPEC> {
        SLPINTMSK_W::new(self, 2)
    }
    #[doc = "Bit 3 - End of event (adv, scan, conn) interrupt mask"]
    #[inline(always)]
    pub fn eventintmsk(&mut self) -> EVENTINTMSK_W<'_, BLEINTCNTL_SPEC> {
        EVENTINTMSK_W::new(self, 3)
    }
    #[doc = "Bit 4 - Encryption engine interrupt mask"]
    #[inline(always)]
    pub fn cryptintmsk(&mut self) -> CRYPTINTMSK_W<'_, BLEINTCNTL_SPEC> {
        CRYPTINTMSK_W::new(self, 4)
    }
    #[doc = "Bit 5 - Error interrupt mask"]
    #[inline(always)]
    pub fn errorintmsk(&mut self) -> ERRORINTMSK_W<'_, BLEINTCNTL_SPEC> {
        ERRORINTMSK_W::new(self, 5)
    }
    #[doc = "Bit 6 - Gross target timer interrupt mask"]
    #[inline(always)]
    pub fn grosstgtimintmsk(&mut self) -> GROSSTGTIMINTMSK_W<'_, BLEINTCNTL_SPEC> {
        GROSSTGTIMINTMSK_W::new(self, 6)
    }
    #[doc = "Bit 7 - Fine target timer interrupt mask"]
    #[inline(always)]
    pub fn finetgtimintmsk(&mut self) -> FINETGTIMINTMSK_W<'_, BLEINTCNTL_SPEC> {
        FINETGTIMINTMSK_W::new(self, 7)
    }
    #[doc = "Bit 8 - End of event/anticipated pre-fetch abort interrupt mask"]
    #[inline(always)]
    pub fn eventapfaintmsk(&mut self) -> EVENTAPFAINTMSK_W<'_, BLEINTCNTL_SPEC> {
        EVENTAPFAINTMSK_W::new(self, 8)
    }
    #[doc = "Bit 9 - Software triggered interrupt mask"]
    #[inline(always)]
    pub fn swintmsk(&mut self) -> SWINTMSK_W<'_, BLEINTCNTL_SPEC> {
        SWINTMSK_W::new(self, 9)
    }
}
#[doc = "BLE interrupt controller register\n\nYou can [`read`](crate::Reg::read) this register and get [`bleintcntl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bleintcntl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLEINTCNTL_SPEC;
impl crate::RegisterSpec for BLEINTCNTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bleintcntl::R`](R) reader structure"]
impl crate::Readable for BLEINTCNTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bleintcntl::W`](W) writer structure"]
impl crate::Writable for BLEINTCNTL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLEINTCNTL to value 0"]
impl crate::Resettable for BLEINTCNTL_SPEC {}
