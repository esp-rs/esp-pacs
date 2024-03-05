#[doc = "Register `SIGMADELTA%s` reader"]
pub type R = crate::R<SIGMADELTA_SPEC>;
#[doc = "Register `SIGMADELTA%s` writer"]
pub type W = crate::W<SIGMADELTA_SPEC>;
#[doc = "Field `SD0_IN` reader - This field is used to configure the duty cycle of sigma delta modulation output."]
pub type SD0_IN_R = crate::FieldReader;
#[doc = "Field `SD0_IN` writer - This field is used to configure the duty cycle of sigma delta modulation output."]
pub type SD0_IN_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SD0_PRESCALE` reader - This field is used to set a divider value to divide APB clock."]
pub type SD0_PRESCALE_R = crate::FieldReader;
#[doc = "Field `SD0_PRESCALE` writer - This field is used to set a divider value to divide APB clock."]
pub type SD0_PRESCALE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - This field is used to configure the duty cycle of sigma delta modulation output."]
    #[inline(always)]
    pub fn sd0_in(&self) -> SD0_IN_R {
        SD0_IN_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - This field is used to set a divider value to divide APB clock."]
    #[inline(always)]
    pub fn sd0_prescale(&self) -> SD0_PRESCALE_R {
        SD0_PRESCALE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SIGMADELTA")
            .field("sd0_in", &format_args!("{}", self.sd0_in().bits()))
            .field(
                "sd0_prescale",
                &format_args!("{}", self.sd0_prescale().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SIGMADELTA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - This field is used to configure the duty cycle of sigma delta modulation output."]
    #[inline(always)]
    #[must_use]
    pub fn sd0_in(&mut self) -> SD0_IN_W<SIGMADELTA_SPEC> {
        SD0_IN_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - This field is used to set a divider value to divide APB clock."]
    #[inline(always)]
    #[must_use]
    pub fn sd0_prescale(&mut self) -> SD0_PRESCALE_W<SIGMADELTA_SPEC> {
        SD0_PRESCALE_W::new(self, 8)
    }
}
#[doc = "Duty Cycle Configure Register of SDM%s\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sigmadelta::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sigmadelta::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SIGMADELTA_SPEC;
impl crate::RegisterSpec for SIGMADELTA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sigmadelta::R`](R) reader structure"]
impl crate::Readable for SIGMADELTA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sigmadelta::W`](W) writer structure"]
impl crate::Writable for SIGMADELTA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SIGMADELTA%s to value 0xff00"]
impl crate::Resettable for SIGMADELTA_SPEC {
    const RESET_VALUE: u32 = 0xff00;
}
