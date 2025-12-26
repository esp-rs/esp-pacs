#[doc = "Register `BLEINTACK` reader"]
pub type R = crate::R<BLEINTACK_SPEC>;
#[doc = "Register `BLEINTACK` writer"]
pub type W = crate::W<BLEINTACK_SPEC>;
#[doc = "Field `CSCNTINTACK` reader - 625μs base time reference interrupt acknowledgment bit"]
pub type CSCNTINTACK_R = crate::BitReader;
#[doc = "Field `CSCNTINTACK` writer - 625μs base time reference interrupt acknowledgment bit"]
pub type CSCNTINTACK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXINTACK` reader - Packet RX interrupt acknowledgment bit"]
pub type RXINTACK_R = crate::BitReader;
#[doc = "Field `RXINTACK` writer - Packet RX interrupt acknowledgment bit"]
pub type RXINTACK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLPINTACK` reader - Sleep mode interrupt acknowledgment bit"]
pub type SLPINTACK_R = crate::BitReader;
#[doc = "Field `SLPINTACK` writer - Sleep mode interrupt acknowledgment bit"]
pub type SLPINTACK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVENTINTACK` reader - End of event interrupt acknowledgment bit"]
pub type EVENTINTACK_R = crate::BitReader;
#[doc = "Field `EVENTINTACK` writer - End of event interrupt acknowledgment bit"]
pub type EVENTINTACK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRYPTINTACK` reader - Encryption engine interrupt acknowledgment bit"]
pub type CRYPTINTACK_R = crate::BitReader;
#[doc = "Field `CRYPTINTACK` writer - Encryption engine interrupt acknowledgment bit"]
pub type CRYPTINTACK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRORINTACK` reader - Error interrupt acknowledgment bit"]
pub type ERRORINTACK_R = crate::BitReader;
#[doc = "Field `ERRORINTACK` writer - Error interrupt acknowledgment bit"]
pub type ERRORINTACK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GROSSTGTIMINTACK` reader - Gross target timer interrupt acknowledgment bit"]
pub type GROSSTGTIMINTACK_R = crate::BitReader;
#[doc = "Field `GROSSTGTIMINTACK` writer - Gross target timer interrupt acknowledgment bit"]
pub type GROSSTGTIMINTACK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FINETGTIMINTACK` reader - Fine target timer interrupt acknowledgment bit"]
pub type FINETGTIMINTACK_R = crate::BitReader;
#[doc = "Field `FINETGTIMINTACK` writer - Fine target timer interrupt acknowledgment bit"]
pub type FINETGTIMINTACK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVENTAPFAINTACK` reader - End of event/anticipated pre-fetch abort interrupt acknowledgment bit"]
pub type EVENTAPFAINTACK_R = crate::BitReader;
#[doc = "Field `EVENTAPFAINTACK` writer - End of event/anticipated pre-fetch abort interrupt acknowledgment bit"]
pub type EVENTAPFAINTACK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWINTACK` reader - Software triggered interrupt acknowledgment bit"]
pub type SWINTACK_R = crate::BitReader;
#[doc = "Field `SWINTACK` writer - Software triggered interrupt acknowledgment bit"]
pub type SWINTACK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 625μs base time reference interrupt acknowledgment bit"]
    #[inline(always)]
    pub fn cscntintack(&self) -> CSCNTINTACK_R {
        CSCNTINTACK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Packet RX interrupt acknowledgment bit"]
    #[inline(always)]
    pub fn rxintack(&self) -> RXINTACK_R {
        RXINTACK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Sleep mode interrupt acknowledgment bit"]
    #[inline(always)]
    pub fn slpintack(&self) -> SLPINTACK_R {
        SLPINTACK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - End of event interrupt acknowledgment bit"]
    #[inline(always)]
    pub fn eventintack(&self) -> EVENTINTACK_R {
        EVENTINTACK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Encryption engine interrupt acknowledgment bit"]
    #[inline(always)]
    pub fn cryptintack(&self) -> CRYPTINTACK_R {
        CRYPTINTACK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Error interrupt acknowledgment bit"]
    #[inline(always)]
    pub fn errorintack(&self) -> ERRORINTACK_R {
        ERRORINTACK_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Gross target timer interrupt acknowledgment bit"]
    #[inline(always)]
    pub fn grosstgtimintack(&self) -> GROSSTGTIMINTACK_R {
        GROSSTGTIMINTACK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Fine target timer interrupt acknowledgment bit"]
    #[inline(always)]
    pub fn finetgtimintack(&self) -> FINETGTIMINTACK_R {
        FINETGTIMINTACK_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - End of event/anticipated pre-fetch abort interrupt acknowledgment bit"]
    #[inline(always)]
    pub fn eventapfaintack(&self) -> EVENTAPFAINTACK_R {
        EVENTAPFAINTACK_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Software triggered interrupt acknowledgment bit"]
    #[inline(always)]
    pub fn swintack(&self) -> SWINTACK_R {
        SWINTACK_R::new(((self.bits >> 9) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLEINTACK")
            .field("cscntintack", &self.cscntintack())
            .field("rxintack", &self.rxintack())
            .field("slpintack", &self.slpintack())
            .field("eventintack", &self.eventintack())
            .field("cryptintack", &self.cryptintack())
            .field("errorintack", &self.errorintack())
            .field("grosstgtimintack", &self.grosstgtimintack())
            .field("finetgtimintack", &self.finetgtimintack())
            .field("eventapfaintack", &self.eventapfaintack())
            .field("swintack", &self.swintack())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - 625μs base time reference interrupt acknowledgment bit"]
    #[inline(always)]
    pub fn cscntintack(&mut self) -> CSCNTINTACK_W<'_, BLEINTACK_SPEC> {
        CSCNTINTACK_W::new(self, 0)
    }
    #[doc = "Bit 1 - Packet RX interrupt acknowledgment bit"]
    #[inline(always)]
    pub fn rxintack(&mut self) -> RXINTACK_W<'_, BLEINTACK_SPEC> {
        RXINTACK_W::new(self, 1)
    }
    #[doc = "Bit 2 - Sleep mode interrupt acknowledgment bit"]
    #[inline(always)]
    pub fn slpintack(&mut self) -> SLPINTACK_W<'_, BLEINTACK_SPEC> {
        SLPINTACK_W::new(self, 2)
    }
    #[doc = "Bit 3 - End of event interrupt acknowledgment bit"]
    #[inline(always)]
    pub fn eventintack(&mut self) -> EVENTINTACK_W<'_, BLEINTACK_SPEC> {
        EVENTINTACK_W::new(self, 3)
    }
    #[doc = "Bit 4 - Encryption engine interrupt acknowledgment bit"]
    #[inline(always)]
    pub fn cryptintack(&mut self) -> CRYPTINTACK_W<'_, BLEINTACK_SPEC> {
        CRYPTINTACK_W::new(self, 4)
    }
    #[doc = "Bit 5 - Error interrupt acknowledgment bit"]
    #[inline(always)]
    pub fn errorintack(&mut self) -> ERRORINTACK_W<'_, BLEINTACK_SPEC> {
        ERRORINTACK_W::new(self, 5)
    }
    #[doc = "Bit 6 - Gross target timer interrupt acknowledgment bit"]
    #[inline(always)]
    pub fn grosstgtimintack(&mut self) -> GROSSTGTIMINTACK_W<'_, BLEINTACK_SPEC> {
        GROSSTGTIMINTACK_W::new(self, 6)
    }
    #[doc = "Bit 7 - Fine target timer interrupt acknowledgment bit"]
    #[inline(always)]
    pub fn finetgtimintack(&mut self) -> FINETGTIMINTACK_W<'_, BLEINTACK_SPEC> {
        FINETGTIMINTACK_W::new(self, 7)
    }
    #[doc = "Bit 8 - End of event/anticipated pre-fetch abort interrupt acknowledgment bit"]
    #[inline(always)]
    pub fn eventapfaintack(&mut self) -> EVENTAPFAINTACK_W<'_, BLEINTACK_SPEC> {
        EVENTAPFAINTACK_W::new(self, 8)
    }
    #[doc = "Bit 9 - Software triggered interrupt acknowledgment bit"]
    #[inline(always)]
    pub fn swintack(&mut self) -> SWINTACK_W<'_, BLEINTACK_SPEC> {
        SWINTACK_W::new(self, 9)
    }
}
#[doc = "BLE interrupt acknowledgement register\n\nYou can [`read`](crate::Reg::read) this register and get [`bleintack::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bleintack::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLEINTACK_SPEC;
impl crate::RegisterSpec for BLEINTACK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bleintack::R`](R) reader structure"]
impl crate::Readable for BLEINTACK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bleintack::W`](W) writer structure"]
impl crate::Writable for BLEINTACK_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLEINTACK to value 0"]
impl crate::Resettable for BLEINTACK_SPEC {}
