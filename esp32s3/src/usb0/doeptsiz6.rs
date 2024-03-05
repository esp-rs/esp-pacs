#[doc = "Register `DOEPTSIZ6` reader"]
pub type R = crate::R<DOEPTSIZ6_SPEC>;
#[doc = "Register `DOEPTSIZ6` writer"]
pub type W = crate::W<DOEPTSIZ6_SPEC>;
#[doc = "Field `XFERSIZE6` reader - "]
pub type XFERSIZE6_R = crate::FieldReader;
#[doc = "Field `XFERSIZE6` writer - "]
pub type XFERSIZE6_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `PKTCNT6` reader - "]
pub type PKTCNT6_R = crate::BitReader;
#[doc = "Field `PKTCNT6` writer - "]
pub type PKTCNT6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SUPCNT6` reader - "]
pub type SUPCNT6_R = crate::FieldReader;
#[doc = "Field `SUPCNT6` writer - "]
pub type SUPCNT6_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn xfersize6(&self) -> XFERSIZE6_R {
        XFERSIZE6_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn pktcnt6(&self) -> PKTCNT6_R {
        PKTCNT6_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 29:30"]
    #[inline(always)]
    pub fn supcnt6(&self) -> SUPCNT6_R {
        SUPCNT6_R::new(((self.bits >> 29) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOEPTSIZ6")
            .field("xfersize6", &format_args!("{}", self.xfersize6().bits()))
            .field("pktcnt6", &format_args!("{}", self.pktcnt6().bit()))
            .field("supcnt6", &format_args!("{}", self.supcnt6().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DOEPTSIZ6_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    #[must_use]
    pub fn xfersize6(&mut self) -> XFERSIZE6_W<DOEPTSIZ6_SPEC> {
        XFERSIZE6_W::new(self, 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn pktcnt6(&mut self) -> PKTCNT6_W<DOEPTSIZ6_SPEC> {
        PKTCNT6_W::new(self, 19)
    }
    #[doc = "Bits 29:30"]
    #[inline(always)]
    #[must_use]
    pub fn supcnt6(&mut self) -> SUPCNT6_W<DOEPTSIZ6_SPEC> {
        SUPCNT6_W::new(self, 29)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doeptsiz6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doeptsiz6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOEPTSIZ6_SPEC;
impl crate::RegisterSpec for DOEPTSIZ6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doeptsiz6::R`](R) reader structure"]
impl crate::Readable for DOEPTSIZ6_SPEC {}
#[doc = "`write(|w| ..)` method takes [`doeptsiz6::W`](W) writer structure"]
impl crate::Writable for DOEPTSIZ6_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOEPTSIZ6 to value 0"]
impl crate::Resettable for DOEPTSIZ6_SPEC {
    const RESET_VALUE: u32 = 0;
}
