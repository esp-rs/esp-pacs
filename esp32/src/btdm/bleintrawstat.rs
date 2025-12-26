#[doc = "Register `BLEINTRAWSTAT` reader"]
pub type R = crate::R<BLEINTRAWSTAT_SPEC>;
#[doc = "Register `BLEINTRAWSTAT` writer"]
pub type W = crate::W<BLEINTRAWSTAT_SPEC>;
#[doc = "Field `CSCNTINTRAWSTAT` reader - 625μs base time reference interrupt raw status"]
pub type CSCNTINTRAWSTAT_R = crate::BitReader;
#[doc = "Field `CSCNTINTRAWSTAT` writer - 625μs base time reference interrupt raw status"]
pub type CSCNTINTRAWSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXINTRAWSTAT` reader - Packet RX interrupt raw status"]
pub type RXINTRAWSTAT_R = crate::BitReader;
#[doc = "Field `RXINTRAWSTAT` writer - Packet RX interrupt raw status"]
pub type RXINTRAWSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLPINTRAWSTAT` reader - Sleep mode interrupt raw status"]
pub type SLPINTRAWSTAT_R = crate::BitReader;
#[doc = "Field `SLPINTRAWSTAT` writer - Sleep mode interrupt raw status"]
pub type SLPINTRAWSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVENTINTRAWSTAT` reader - End of event interrupt raw status"]
pub type EVENTINTRAWSTAT_R = crate::BitReader;
#[doc = "Field `EVENTINTRAWSTAT` writer - End of event interrupt raw status"]
pub type EVENTINTRAWSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRYPTINTRAWSTAT` reader - Encryption engine interrupt raw status"]
pub type CRYPTINTRAWSTAT_R = crate::BitReader;
#[doc = "Field `CRYPTINTRAWSTAT` writer - Encryption engine interrupt raw status"]
pub type CRYPTINTRAWSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRORINTRAWSTAT` reader - Error interrupt raw status"]
pub type ERRORINTRAWSTAT_R = crate::BitReader;
#[doc = "Field `ERRORINTRAWSTAT` writer - Error interrupt raw status"]
pub type ERRORINTRAWSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GROSSTGTIMINTRAWSTAT` reader - Gross target timer interrupt raw status"]
pub type GROSSTGTIMINTRAWSTAT_R = crate::BitReader;
#[doc = "Field `GROSSTGTIMINTRAWSTAT` writer - Gross target timer interrupt raw status"]
pub type GROSSTGTIMINTRAWSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FINETGTIMINTRAWSTAT` reader - Fine target timer interrupt raw status"]
pub type FINETGTIMINTRAWSTAT_R = crate::BitReader;
#[doc = "Field `FINETGTIMINTRAWSTAT` writer - Fine target timer interrupt raw status"]
pub type FINETGTIMINTRAWSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVENTAPFAINTRAWSTAT` reader - End of event/anticipated pre-fetch abort interrupt raw status"]
pub type EVENTAPFAINTRAWSTAT_R = crate::BitReader;
#[doc = "Field `EVENTAPFAINTRAWSTAT` writer - End of event/anticipated pre-fetch abort interrupt raw status"]
pub type EVENTAPFAINTRAWSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWINTRAWSTAT` reader - Software triggered interrupt raw status"]
pub type SWINTRAWSTAT_R = crate::BitReader;
#[doc = "Field `SWINTRAWSTAT` writer - Software triggered interrupt raw status"]
pub type SWINTRAWSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 625μs base time reference interrupt raw status"]
    #[inline(always)]
    pub fn cscntintrawstat(&self) -> CSCNTINTRAWSTAT_R {
        CSCNTINTRAWSTAT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Packet RX interrupt raw status"]
    #[inline(always)]
    pub fn rxintrawstat(&self) -> RXINTRAWSTAT_R {
        RXINTRAWSTAT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Sleep mode interrupt raw status"]
    #[inline(always)]
    pub fn slpintrawstat(&self) -> SLPINTRAWSTAT_R {
        SLPINTRAWSTAT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - End of event interrupt raw status"]
    #[inline(always)]
    pub fn eventintrawstat(&self) -> EVENTINTRAWSTAT_R {
        EVENTINTRAWSTAT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Encryption engine interrupt raw status"]
    #[inline(always)]
    pub fn cryptintrawstat(&self) -> CRYPTINTRAWSTAT_R {
        CRYPTINTRAWSTAT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Error interrupt raw status"]
    #[inline(always)]
    pub fn errorintrawstat(&self) -> ERRORINTRAWSTAT_R {
        ERRORINTRAWSTAT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Gross target timer interrupt raw status"]
    #[inline(always)]
    pub fn grosstgtimintrawstat(&self) -> GROSSTGTIMINTRAWSTAT_R {
        GROSSTGTIMINTRAWSTAT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Fine target timer interrupt raw status"]
    #[inline(always)]
    pub fn finetgtimintrawstat(&self) -> FINETGTIMINTRAWSTAT_R {
        FINETGTIMINTRAWSTAT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - End of event/anticipated pre-fetch abort interrupt raw status"]
    #[inline(always)]
    pub fn eventapfaintrawstat(&self) -> EVENTAPFAINTRAWSTAT_R {
        EVENTAPFAINTRAWSTAT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Software triggered interrupt raw status"]
    #[inline(always)]
    pub fn swintrawstat(&self) -> SWINTRAWSTAT_R {
        SWINTRAWSTAT_R::new(((self.bits >> 9) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLEINTRAWSTAT")
            .field("cscntintrawstat", &self.cscntintrawstat())
            .field("rxintrawstat", &self.rxintrawstat())
            .field("slpintrawstat", &self.slpintrawstat())
            .field("eventintrawstat", &self.eventintrawstat())
            .field("cryptintrawstat", &self.cryptintrawstat())
            .field("errorintrawstat", &self.errorintrawstat())
            .field("grosstgtimintrawstat", &self.grosstgtimintrawstat())
            .field("finetgtimintrawstat", &self.finetgtimintrawstat())
            .field("eventapfaintrawstat", &self.eventapfaintrawstat())
            .field("swintrawstat", &self.swintrawstat())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - 625μs base time reference interrupt raw status"]
    #[inline(always)]
    pub fn cscntintrawstat(&mut self) -> CSCNTINTRAWSTAT_W<'_, BLEINTRAWSTAT_SPEC> {
        CSCNTINTRAWSTAT_W::new(self, 0)
    }
    #[doc = "Bit 1 - Packet RX interrupt raw status"]
    #[inline(always)]
    pub fn rxintrawstat(&mut self) -> RXINTRAWSTAT_W<'_, BLEINTRAWSTAT_SPEC> {
        RXINTRAWSTAT_W::new(self, 1)
    }
    #[doc = "Bit 2 - Sleep mode interrupt raw status"]
    #[inline(always)]
    pub fn slpintrawstat(&mut self) -> SLPINTRAWSTAT_W<'_, BLEINTRAWSTAT_SPEC> {
        SLPINTRAWSTAT_W::new(self, 2)
    }
    #[doc = "Bit 3 - End of event interrupt raw status"]
    #[inline(always)]
    pub fn eventintrawstat(&mut self) -> EVENTINTRAWSTAT_W<'_, BLEINTRAWSTAT_SPEC> {
        EVENTINTRAWSTAT_W::new(self, 3)
    }
    #[doc = "Bit 4 - Encryption engine interrupt raw status"]
    #[inline(always)]
    pub fn cryptintrawstat(&mut self) -> CRYPTINTRAWSTAT_W<'_, BLEINTRAWSTAT_SPEC> {
        CRYPTINTRAWSTAT_W::new(self, 4)
    }
    #[doc = "Bit 5 - Error interrupt raw status"]
    #[inline(always)]
    pub fn errorintrawstat(&mut self) -> ERRORINTRAWSTAT_W<'_, BLEINTRAWSTAT_SPEC> {
        ERRORINTRAWSTAT_W::new(self, 5)
    }
    #[doc = "Bit 6 - Gross target timer interrupt raw status"]
    #[inline(always)]
    pub fn grosstgtimintrawstat(&mut self) -> GROSSTGTIMINTRAWSTAT_W<'_, BLEINTRAWSTAT_SPEC> {
        GROSSTGTIMINTRAWSTAT_W::new(self, 6)
    }
    #[doc = "Bit 7 - Fine target timer interrupt raw status"]
    #[inline(always)]
    pub fn finetgtimintrawstat(&mut self) -> FINETGTIMINTRAWSTAT_W<'_, BLEINTRAWSTAT_SPEC> {
        FINETGTIMINTRAWSTAT_W::new(self, 7)
    }
    #[doc = "Bit 8 - End of event/anticipated pre-fetch abort interrupt raw status"]
    #[inline(always)]
    pub fn eventapfaintrawstat(&mut self) -> EVENTAPFAINTRAWSTAT_W<'_, BLEINTRAWSTAT_SPEC> {
        EVENTAPFAINTRAWSTAT_W::new(self, 8)
    }
    #[doc = "Bit 9 - Software triggered interrupt raw status"]
    #[inline(always)]
    pub fn swintrawstat(&mut self) -> SWINTRAWSTAT_W<'_, BLEINTRAWSTAT_SPEC> {
        SWINTRAWSTAT_W::new(self, 9)
    }
}
#[doc = "BLE interrupt raw status register\n\nYou can [`read`](crate::Reg::read) this register and get [`bleintrawstat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bleintrawstat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLEINTRAWSTAT_SPEC;
impl crate::RegisterSpec for BLEINTRAWSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bleintrawstat::R`](R) reader structure"]
impl crate::Readable for BLEINTRAWSTAT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bleintrawstat::W`](W) writer structure"]
impl crate::Writable for BLEINTRAWSTAT_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLEINTRAWSTAT to value 0"]
impl crate::Resettable for BLEINTRAWSTAT_SPEC {}
