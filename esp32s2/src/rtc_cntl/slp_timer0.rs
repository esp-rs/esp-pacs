#[doc = "Register `SLP_TIMER0` reader"]
pub type R = crate::R<SLP_TIMER0_SPEC>;
#[doc = "Register `SLP_TIMER0` writer"]
pub type W = crate::W<SLP_TIMER0_SPEC>;
#[doc = "Field `SLP_VAL_LO` reader - Sets the lower 32 bits of the trigger threshold for the RTC timer."]
pub type SLP_VAL_LO_R = crate::FieldReader<u32>;
#[doc = "Field `SLP_VAL_LO` writer - Sets the lower 32 bits of the trigger threshold for the RTC timer."]
pub type SLP_VAL_LO_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Sets the lower 32 bits of the trigger threshold for the RTC timer."]
    #[inline(always)]
    pub fn slp_val_lo(&self) -> SLP_VAL_LO_R {
        SLP_VAL_LO_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLP_TIMER0")
            .field("slp_val_lo", &self.slp_val_lo())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Sets the lower 32 bits of the trigger threshold for the RTC timer."]
    #[inline(always)]
    pub fn slp_val_lo(&mut self) -> SLP_VAL_LO_W<SLP_TIMER0_SPEC> {
        SLP_VAL_LO_W::new(self, 0)
    }
}
#[doc = "RTC timer threshold register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`slp_timer0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slp_timer0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
