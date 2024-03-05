#[doc = "Register `DOEPTSIZ3` reader"]
pub type R = crate::R<DOEPTSIZ3_SPEC>;
#[doc = "Register `DOEPTSIZ3` writer"]
pub type W = crate::W<DOEPTSIZ3_SPEC>;
#[doc = "Field `XFERSIZE3` reader - "]
pub type XFERSIZE3_R = crate::FieldReader;
#[doc = "Field `XFERSIZE3` writer - "]
pub type XFERSIZE3_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `PKTCNT3` reader - "]
pub type PKTCNT3_R = crate::BitReader;
#[doc = "Field `PKTCNT3` writer - "]
pub type PKTCNT3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SUPCNT3` reader - "]
pub type SUPCNT3_R = crate::FieldReader;
#[doc = "Field `SUPCNT3` writer - "]
pub type SUPCNT3_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn xfersize3(&self) -> XFERSIZE3_R {
        XFERSIZE3_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn pktcnt3(&self) -> PKTCNT3_R {
        PKTCNT3_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 29:30"]
    #[inline(always)]
    pub fn supcnt3(&self) -> SUPCNT3_R {
        SUPCNT3_R::new(((self.bits >> 29) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOEPTSIZ3")
            .field("xfersize3", &format_args!("{}", self.xfersize3().bits()))
            .field("pktcnt3", &format_args!("{}", self.pktcnt3().bit()))
            .field("supcnt3", &format_args!("{}", self.supcnt3().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DOEPTSIZ3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    #[must_use]
    pub fn xfersize3(&mut self) -> XFERSIZE3_W<DOEPTSIZ3_SPEC> {
        XFERSIZE3_W::new(self, 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn pktcnt3(&mut self) -> PKTCNT3_W<DOEPTSIZ3_SPEC> {
        PKTCNT3_W::new(self, 19)
    }
    #[doc = "Bits 29:30"]
    #[inline(always)]
    #[must_use]
    pub fn supcnt3(&mut self) -> SUPCNT3_W<DOEPTSIZ3_SPEC> {
        SUPCNT3_W::new(self, 29)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doeptsiz3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doeptsiz3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOEPTSIZ3_SPEC;
impl crate::RegisterSpec for DOEPTSIZ3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doeptsiz3::R`](R) reader structure"]
impl crate::Readable for DOEPTSIZ3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`doeptsiz3::W`](W) writer structure"]
impl crate::Writable for DOEPTSIZ3_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOEPTSIZ3 to value 0"]
impl crate::Resettable for DOEPTSIZ3_SPEC {
    const RESET_VALUE: u32 = 0;
}
