#[doc = "Register `SIGMADELTA%s` reader"]
pub type R = crate::R<SIGMADELTA_SPEC>;
#[doc = "Register `SIGMADELTA%s` writer"]
pub type W = crate::W<SIGMADELTA_SPEC>;
#[doc = "Field `GPIO_EXT_SD_IN` reader - This field is used to configure the duty cycle of sigma delta modulation output."]
pub type GPIO_EXT_SD_IN_R = crate::FieldReader;
#[doc = "Field `GPIO_EXT_SD_IN` writer - This field is used to configure the duty cycle of sigma delta modulation output."]
pub type GPIO_EXT_SD_IN_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO_EXT_SD_PRESCALE` reader - This field is used to set a divider value to divide APB clock."]
pub type GPIO_EXT_SD_PRESCALE_R = crate::FieldReader;
#[doc = "Field `GPIO_EXT_SD_PRESCALE` writer - This field is used to set a divider value to divide APB clock."]
pub type GPIO_EXT_SD_PRESCALE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - This field is used to configure the duty cycle of sigma delta modulation output."]
    #[inline(always)]
    pub fn gpio_ext_sd_in(&self) -> GPIO_EXT_SD_IN_R {
        GPIO_EXT_SD_IN_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - This field is used to set a divider value to divide APB clock."]
    #[inline(always)]
    pub fn gpio_ext_sd_prescale(&self) -> GPIO_EXT_SD_PRESCALE_R {
        GPIO_EXT_SD_PRESCALE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SIGMADELTA")
            .field("gpio_ext_sd_in", &self.gpio_ext_sd_in())
            .field("gpio_ext_sd_prescale", &self.gpio_ext_sd_prescale())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - This field is used to configure the duty cycle of sigma delta modulation output."]
    #[inline(always)]
    pub fn gpio_ext_sd_in(&mut self) -> GPIO_EXT_SD_IN_W<'_, SIGMADELTA_SPEC> {
        GPIO_EXT_SD_IN_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - This field is used to set a divider value to divide APB clock."]
    #[inline(always)]
    pub fn gpio_ext_sd_prescale(&mut self) -> GPIO_EXT_SD_PRESCALE_W<'_, SIGMADELTA_SPEC> {
        GPIO_EXT_SD_PRESCALE_W::new(self, 8)
    }
}
#[doc = "Duty Cycle Configure Register of SDM%s\n\nYou can [`read`](crate::Reg::read) this register and get [`sigmadelta::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sigmadelta::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SIGMADELTA_SPEC;
impl crate::RegisterSpec for SIGMADELTA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sigmadelta::R`](R) reader structure"]
impl crate::Readable for SIGMADELTA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sigmadelta::W`](W) writer structure"]
impl crate::Writable for SIGMADELTA_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SIGMADELTA%s to value 0xff00"]
impl crate::Resettable for SIGMADELTA_SPEC {
    const RESET_VALUE: u32 = 0xff00;
}
