#[doc = "Register `SLP_TIMER0` reader"]
pub type R = crate::R<SLP_TIMER0_SPEC>;
#[doc = "Register `SLP_TIMER0` writer"]
pub type W = crate::W<SLP_TIMER0_SPEC>;
#[doc = "Field `SLP_VAL_LO` reader - configure the sleep time"]
pub type SLP_VAL_LO_R = crate::FieldReader<u32>;
#[doc = "Field `SLP_VAL_LO` writer - configure the sleep time"]
pub type SLP_VAL_LO_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - configure the sleep time"]
    #[inline(always)]
    pub fn slp_val_lo(&self) -> SLP_VAL_LO_R {
        SLP_VAL_LO_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLP_TIMER0")
            .field("slp_val_lo", &format_args!("{}", self.slp_val_lo().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SLP_TIMER0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - configure the sleep time"]
    #[inline(always)]
    #[must_use]
    pub fn slp_val_lo(&mut self) -> SLP_VAL_LO_W<SLP_TIMER0_SPEC> {
        SLP_VAL_LO_W::new(self, 0)
    }
}
#[doc = "rtc configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slp_timer0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slp_timer0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLP_TIMER0_SPEC;
impl crate::RegisterSpec for SLP_TIMER0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slp_timer0::R`](R) reader structure"]
impl crate::Readable for SLP_TIMER0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`slp_timer0::W`](W) writer structure"]
impl crate::Writable for SLP_TIMER0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SLP_TIMER0 to value 0"]
impl crate::Resettable for SLP_TIMER0_SPEC {
    const RESET_VALUE: u32 = 0;
}
