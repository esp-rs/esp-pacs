#[doc = "Register `SIGMADELTA_VERSION` reader"]
pub type R = crate::R<SIGMADELTA_VERSION_SPEC>;
#[doc = "Register `SIGMADELTA_VERSION` writer"]
pub type W = crate::W<SIGMADELTA_VERSION_SPEC>;
#[doc = "Field `GPIO_SD_DATE` reader - Version control register."]
pub type GPIO_SD_DATE_R = crate::FieldReader<u32>;
#[doc = "Field `GPIO_SD_DATE` writer - Version control register."]
pub type GPIO_SD_DATE_W<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 0:27 - Version control register."]
    #[inline(always)]
    pub fn gpio_sd_date(&self) -> GPIO_SD_DATE_R {
        GPIO_SD_DATE_R::new(self.bits & 0x0fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SIGMADELTA_VERSION")
            .field(
                "gpio_sd_date",
                &format_args!("{}", self.gpio_sd_date().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SIGMADELTA_VERSION_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:27 - Version control register."]
    #[inline(always)]
    #[must_use]
    pub fn gpio_sd_date(&mut self) -> GPIO_SD_DATE_W<SIGMADELTA_VERSION_SPEC> {
        GPIO_SD_DATE_W::new(self, 0)
    }
}
#[doc = "Version Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sigmadelta_version::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sigmadelta_version::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SIGMADELTA_VERSION_SPEC;
impl crate::RegisterSpec for SIGMADELTA_VERSION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sigmadelta_version::R`](R) reader structure"]
impl crate::Readable for SIGMADELTA_VERSION_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sigmadelta_version::W`](W) writer structure"]
impl crate::Writable for SIGMADELTA_VERSION_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SIGMADELTA_VERSION to value 0x0200_6230"]
impl crate::Resettable for SIGMADELTA_VERSION_SPEC {
    const RESET_VALUE: u32 = 0x0200_6230;
}
