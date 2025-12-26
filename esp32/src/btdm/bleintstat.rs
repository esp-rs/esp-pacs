#[doc = "Register `BLEINTSTAT` reader"]
pub type R = crate::R<BLEINTSTAT_SPEC>;
#[doc = "Register `BLEINTSTAT` writer"]
pub type W = crate::W<BLEINTSTAT_SPEC>;
#[doc = "Field `CSCNTINTSTAT` reader - Masked 625μs base time reference interrupt status"]
pub type CSCNTINTSTAT_R = crate::BitReader;
#[doc = "Field `CSCNTINTSTAT` writer - Masked 625μs base time reference interrupt status"]
pub type CSCNTINTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXINTSTAT` reader - Masked packet RX interrupt status"]
pub type RXINTSTAT_R = crate::BitReader;
#[doc = "Field `RXINTSTAT` writer - Masked packet RX interrupt status"]
pub type RXINTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLPINTSTAT` reader - Masked sleep mode interrupt status"]
pub type SLPINTSTAT_R = crate::BitReader;
#[doc = "Field `SLPINTSTAT` writer - Masked sleep mode interrupt status"]
pub type SLPINTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVENTINTSTAT` reader - Masked end of event interrupt status"]
pub type EVENTINTSTAT_R = crate::BitReader;
#[doc = "Field `EVENTINTSTAT` writer - Masked end of event interrupt status"]
pub type EVENTINTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRYPTINTSTAT` reader - Masked encryption engine interrupt status"]
pub type CRYPTINTSTAT_R = crate::BitReader;
#[doc = "Field `CRYPTINTSTAT` writer - Masked encryption engine interrupt status"]
pub type CRYPTINTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRORINTSTAT` reader - Masked error interrupt status"]
pub type ERRORINTSTAT_R = crate::BitReader;
#[doc = "Field `ERRORINTSTAT` writer - Masked error interrupt status"]
pub type ERRORINTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GROSSTGTIMINTSTAT` reader - Masked gross target timer interrupt status"]
pub type GROSSTGTIMINTSTAT_R = crate::BitReader;
#[doc = "Field `GROSSTGTIMINTSTAT` writer - Masked gross target timer interrupt status"]
pub type GROSSTGTIMINTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FINETGTIMINTSTAT` reader - Masked fine target timer interrupt status"]
pub type FINETGTIMINTSTAT_R = crate::BitReader;
#[doc = "Field `FINETGTIMINTSTAT` writer - Masked fine target timer interrupt status"]
pub type FINETGTIMINTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVENTAPFAINTSTAT` reader - Masked end of event/anticipated pre-fetch abort interrupt status"]
pub type EVENTAPFAINTSTAT_R = crate::BitReader;
#[doc = "Field `EVENTAPFAINTSTAT` writer - Masked end of event/anticipated pre-fetch abort interrupt status"]
pub type EVENTAPFAINTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWINTSTAT` reader - Masked software triggered interrupt status"]
pub type SWINTSTAT_R = crate::BitReader;
#[doc = "Field `SWINTSTAT` writer - Masked software triggered interrupt status"]
pub type SWINTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Masked 625μs base time reference interrupt status"]
    #[inline(always)]
    pub fn cscntintstat(&self) -> CSCNTINTSTAT_R {
        CSCNTINTSTAT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Masked packet RX interrupt status"]
    #[inline(always)]
    pub fn rxintstat(&self) -> RXINTSTAT_R {
        RXINTSTAT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Masked sleep mode interrupt status"]
    #[inline(always)]
    pub fn slpintstat(&self) -> SLPINTSTAT_R {
        SLPINTSTAT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Masked end of event interrupt status"]
    #[inline(always)]
    pub fn eventintstat(&self) -> EVENTINTSTAT_R {
        EVENTINTSTAT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Masked encryption engine interrupt status"]
    #[inline(always)]
    pub fn cryptintstat(&self) -> CRYPTINTSTAT_R {
        CRYPTINTSTAT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Masked error interrupt status"]
    #[inline(always)]
    pub fn errorintstat(&self) -> ERRORINTSTAT_R {
        ERRORINTSTAT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Masked gross target timer interrupt status"]
    #[inline(always)]
    pub fn grosstgtimintstat(&self) -> GROSSTGTIMINTSTAT_R {
        GROSSTGTIMINTSTAT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Masked fine target timer interrupt status"]
    #[inline(always)]
    pub fn finetgtimintstat(&self) -> FINETGTIMINTSTAT_R {
        FINETGTIMINTSTAT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Masked end of event/anticipated pre-fetch abort interrupt status"]
    #[inline(always)]
    pub fn eventapfaintstat(&self) -> EVENTAPFAINTSTAT_R {
        EVENTAPFAINTSTAT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Masked software triggered interrupt status"]
    #[inline(always)]
    pub fn swintstat(&self) -> SWINTSTAT_R {
        SWINTSTAT_R::new(((self.bits >> 9) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLEINTSTAT")
            .field("cscntintstat", &self.cscntintstat())
            .field("rxintstat", &self.rxintstat())
            .field("slpintstat", &self.slpintstat())
            .field("eventintstat", &self.eventintstat())
            .field("cryptintstat", &self.cryptintstat())
            .field("errorintstat", &self.errorintstat())
            .field("grosstgtimintstat", &self.grosstgtimintstat())
            .field("finetgtimintstat", &self.finetgtimintstat())
            .field("eventapfaintstat", &self.eventapfaintstat())
            .field("swintstat", &self.swintstat())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Masked 625μs base time reference interrupt status"]
    #[inline(always)]
    pub fn cscntintstat(&mut self) -> CSCNTINTSTAT_W<'_, BLEINTSTAT_SPEC> {
        CSCNTINTSTAT_W::new(self, 0)
    }
    #[doc = "Bit 1 - Masked packet RX interrupt status"]
    #[inline(always)]
    pub fn rxintstat(&mut self) -> RXINTSTAT_W<'_, BLEINTSTAT_SPEC> {
        RXINTSTAT_W::new(self, 1)
    }
    #[doc = "Bit 2 - Masked sleep mode interrupt status"]
    #[inline(always)]
    pub fn slpintstat(&mut self) -> SLPINTSTAT_W<'_, BLEINTSTAT_SPEC> {
        SLPINTSTAT_W::new(self, 2)
    }
    #[doc = "Bit 3 - Masked end of event interrupt status"]
    #[inline(always)]
    pub fn eventintstat(&mut self) -> EVENTINTSTAT_W<'_, BLEINTSTAT_SPEC> {
        EVENTINTSTAT_W::new(self, 3)
    }
    #[doc = "Bit 4 - Masked encryption engine interrupt status"]
    #[inline(always)]
    pub fn cryptintstat(&mut self) -> CRYPTINTSTAT_W<'_, BLEINTSTAT_SPEC> {
        CRYPTINTSTAT_W::new(self, 4)
    }
    #[doc = "Bit 5 - Masked error interrupt status"]
    #[inline(always)]
    pub fn errorintstat(&mut self) -> ERRORINTSTAT_W<'_, BLEINTSTAT_SPEC> {
        ERRORINTSTAT_W::new(self, 5)
    }
    #[doc = "Bit 6 - Masked gross target timer interrupt status"]
    #[inline(always)]
    pub fn grosstgtimintstat(&mut self) -> GROSSTGTIMINTSTAT_W<'_, BLEINTSTAT_SPEC> {
        GROSSTGTIMINTSTAT_W::new(self, 6)
    }
    #[doc = "Bit 7 - Masked fine target timer interrupt status"]
    #[inline(always)]
    pub fn finetgtimintstat(&mut self) -> FINETGTIMINTSTAT_W<'_, BLEINTSTAT_SPEC> {
        FINETGTIMINTSTAT_W::new(self, 7)
    }
    #[doc = "Bit 8 - Masked end of event/anticipated pre-fetch abort interrupt status"]
    #[inline(always)]
    pub fn eventapfaintstat(&mut self) -> EVENTAPFAINTSTAT_W<'_, BLEINTSTAT_SPEC> {
        EVENTAPFAINTSTAT_W::new(self, 8)
    }
    #[doc = "Bit 9 - Masked software triggered interrupt status"]
    #[inline(always)]
    pub fn swintstat(&mut self) -> SWINTSTAT_W<'_, BLEINTSTAT_SPEC> {
        SWINTSTAT_W::new(self, 9)
    }
}
#[doc = "BLE interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`bleintstat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bleintstat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLEINTSTAT_SPEC;
impl crate::RegisterSpec for BLEINTSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bleintstat::R`](R) reader structure"]
impl crate::Readable for BLEINTSTAT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bleintstat::W`](W) writer structure"]
impl crate::Writable for BLEINTSTAT_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLEINTSTAT to value 0"]
impl crate::Resettable for BLEINTSTAT_SPEC {}
