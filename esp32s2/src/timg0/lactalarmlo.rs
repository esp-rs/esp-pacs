#[doc = "Register `LACTALARMLO` reader"]
pub type R = crate::R<LACTALARMLO_SPEC>;
#[doc = "Register `LACTALARMLO` writer"]
pub type W = crate::W<LACTALARMLO_SPEC>;
#[doc = "Field `ALARM_LO` reader - Reserved."]
pub type ALARM_LO_R = crate::FieldReader<u32>;
#[doc = "Field `ALARM_LO` writer - Reserved."]
pub type ALARM_LO_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Reserved."]
    #[inline(always)]
    pub fn alarm_lo(&self) -> ALARM_LO_R {
        ALARM_LO_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LACTALARMLO")
            .field("alarm_lo", &self.alarm_lo().bits())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LACTALARMLO_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - Reserved."]
    #[inline(always)]
    #[must_use]
    pub fn alarm_lo(&mut self) -> ALARM_LO_W<LACTALARMLO_SPEC> {
        ALARM_LO_W::new(self, 0)
    }
}
#[doc = "LACT alarm low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lactalarmlo::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lactalarmlo::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LACTALARMLO_SPEC;
impl crate::RegisterSpec for LACTALARMLO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lactalarmlo::R`](R) reader structure"]
impl crate::Readable for LACTALARMLO_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lactalarmlo::W`](W) writer structure"]
impl crate::Writable for LACTALARMLO_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LACTALARMLO to value 0"]
impl crate::Resettable for LACTALARMLO_SPEC {
    const RESET_VALUE: u32 = 0;
}
