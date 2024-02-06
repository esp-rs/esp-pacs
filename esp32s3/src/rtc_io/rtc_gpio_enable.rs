#[doc = "Register `RTC_GPIO_ENABLE` reader"]
pub type R = crate::R<RTC_GPIO_ENABLE_SPEC>;
#[doc = "Register `RTC_GPIO_ENABLE` writer"]
pub type W = crate::W<RTC_GPIO_ENABLE_SPEC>;
#[doc = "Field `RTC_GPIO_ENABLE` reader - RTC GPIO 0 ~ 21 enable"]
pub type RTC_GPIO_ENABLE_R = crate::FieldReader<u32>;
#[doc = "Field `RTC_GPIO_ENABLE` writer - RTC GPIO 0 ~ 21 enable"]
pub type RTC_GPIO_ENABLE_W<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
impl R {
    #[doc = "Bits 10:31 - RTC GPIO 0 ~ 21 enable"]
    #[inline(always)]
    pub fn rtc_gpio_enable(&self) -> RTC_GPIO_ENABLE_R {
        RTC_GPIO_ENABLE_R::new((self.bits >> 10) & 0x003f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTC_GPIO_ENABLE")
            .field(
                "rtc_gpio_enable",
                &format_args!("{}", self.rtc_gpio_enable().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RTC_GPIO_ENABLE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 10:31 - RTC GPIO 0 ~ 21 enable"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_gpio_enable(&mut self) -> RTC_GPIO_ENABLE_W<RTC_GPIO_ENABLE_SPEC> {
        RTC_GPIO_ENABLE_W::new(self, 10)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Configure RTC GPIO output enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_gpio_enable::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_gpio_enable::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RTC_GPIO_ENABLE_SPEC;
impl crate::RegisterSpec for RTC_GPIO_ENABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtc_gpio_enable::R`](R) reader structure"]
impl crate::Readable for RTC_GPIO_ENABLE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rtc_gpio_enable::W`](W) writer structure"]
impl crate::Writable for RTC_GPIO_ENABLE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RTC_GPIO_ENABLE to value 0"]
impl crate::Resettable for RTC_GPIO_ENABLE_SPEC {
    const RESET_VALUE: u32 = 0;
}
