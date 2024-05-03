#[doc = "Register `HPRT` reader"]
pub type R = crate::R<HPRT_SPEC>;
#[doc = "Register `HPRT` writer"]
pub type W = crate::W<HPRT_SPEC>;
#[doc = "Field `PRTCONNSTS` reader - "]
pub type PRTCONNSTS_R = crate::BitReader;
#[doc = "Field `PRTCONNDET` reader - "]
pub type PRTCONNDET_R = crate::BitReader;
#[doc = "Field `PRTCONNDET` writer - "]
pub type PRTCONNDET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRTENA` reader - "]
pub type PRTENA_R = crate::BitReader;
#[doc = "Field `PRTENA` writer - "]
pub type PRTENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRTENCHNG` reader - "]
pub type PRTENCHNG_R = crate::BitReader;
#[doc = "Field `PRTENCHNG` writer - "]
pub type PRTENCHNG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRTOVRCURRACT` reader - "]
pub type PRTOVRCURRACT_R = crate::BitReader;
#[doc = "Field `PRTOVRCURRCHNG` reader - "]
pub type PRTOVRCURRCHNG_R = crate::BitReader;
#[doc = "Field `PRTOVRCURRCHNG` writer - "]
pub type PRTOVRCURRCHNG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRTRES` reader - "]
pub type PRTRES_R = crate::BitReader;
#[doc = "Field `PRTRES` writer - "]
pub type PRTRES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRTSUSP` reader - "]
pub type PRTSUSP_R = crate::BitReader;
#[doc = "Field `PRTSUSP` writer - "]
pub type PRTSUSP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRTRST` reader - "]
pub type PRTRST_R = crate::BitReader;
#[doc = "Field `PRTRST` writer - "]
pub type PRTRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRTLNSTS` reader - "]
pub type PRTLNSTS_R = crate::FieldReader;
#[doc = "Field `PRTPWR` reader - "]
pub type PRTPWR_R = crate::BitReader;
#[doc = "Field `PRTPWR` writer - "]
pub type PRTPWR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRTTSTCTL` reader - "]
pub type PRTTSTCTL_R = crate::FieldReader;
#[doc = "Field `PRTTSTCTL` writer - "]
pub type PRTTSTCTL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PRTSPD` reader - "]
pub type PRTSPD_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn prtconnsts(&self) -> PRTCONNSTS_R {
        PRTCONNSTS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn prtconndet(&self) -> PRTCONNDET_R {
        PRTCONNDET_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn prtena(&self) -> PRTENA_R {
        PRTENA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn prtenchng(&self) -> PRTENCHNG_R {
        PRTENCHNG_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn prtovrcurract(&self) -> PRTOVRCURRACT_R {
        PRTOVRCURRACT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn prtovrcurrchng(&self) -> PRTOVRCURRCHNG_R {
        PRTOVRCURRCHNG_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn prtres(&self) -> PRTRES_R {
        PRTRES_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn prtsusp(&self) -> PRTSUSP_R {
        PRTSUSP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn prtrst(&self) -> PRTRST_R {
        PRTRST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn prtlnsts(&self) -> PRTLNSTS_R {
        PRTLNSTS_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn prtpwr(&self) -> PRTPWR_R {
        PRTPWR_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:16"]
    #[inline(always)]
    pub fn prttstctl(&self) -> PRTTSTCTL_R {
        PRTTSTCTL_R::new(((self.bits >> 13) & 0x0f) as u8)
    }
    #[doc = "Bits 17:18"]
    #[inline(always)]
    pub fn prtspd(&self) -> PRTSPD_R {
        PRTSPD_R::new(((self.bits >> 17) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HPRT")
            .field("prtconnsts", &self.prtconnsts().bit())
            .field("prtconndet", &self.prtconndet().bit())
            .field("prtena", &self.prtena().bit())
            .field("prtenchng", &self.prtenchng().bit())
            .field("prtovrcurract", &self.prtovrcurract().bit())
            .field("prtovrcurrchng", &self.prtovrcurrchng().bit())
            .field("prtres", &self.prtres().bit())
            .field("prtsusp", &self.prtsusp().bit())
            .field("prtrst", &self.prtrst().bit())
            .field("prtlnsts", &self.prtlnsts().bits())
            .field("prtpwr", &self.prtpwr().bit())
            .field("prttstctl", &self.prttstctl().bits())
            .field("prtspd", &self.prtspd().bits())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HPRT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn prtconndet(&mut self) -> PRTCONNDET_W<HPRT_SPEC> {
        PRTCONNDET_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn prtena(&mut self) -> PRTENA_W<HPRT_SPEC> {
        PRTENA_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn prtenchng(&mut self) -> PRTENCHNG_W<HPRT_SPEC> {
        PRTENCHNG_W::new(self, 3)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn prtovrcurrchng(&mut self) -> PRTOVRCURRCHNG_W<HPRT_SPEC> {
        PRTOVRCURRCHNG_W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn prtres(&mut self) -> PRTRES_W<HPRT_SPEC> {
        PRTRES_W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn prtsusp(&mut self) -> PRTSUSP_W<HPRT_SPEC> {
        PRTSUSP_W::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn prtrst(&mut self) -> PRTRST_W<HPRT_SPEC> {
        PRTRST_W::new(self, 8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn prtpwr(&mut self) -> PRTPWR_W<HPRT_SPEC> {
        PRTPWR_W::new(self, 12)
    }
    #[doc = "Bits 13:16"]
    #[inline(always)]
    #[must_use]
    pub fn prttstctl(&mut self) -> PRTTSTCTL_W<HPRT_SPEC> {
        PRTTSTCTL_W::new(self, 13)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hprt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hprt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HPRT_SPEC;
impl crate::RegisterSpec for HPRT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hprt::R`](R) reader structure"]
impl crate::Readable for HPRT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hprt::W`](W) writer structure"]
impl crate::Writable for HPRT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HPRT to value 0"]
impl crate::Resettable for HPRT_SPEC {
    const RESET_VALUE: u32 = 0;
}
