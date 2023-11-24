#[doc = "Register `RTC_GPIO_OUT` reader"]
pub type R = crate::R<RTC_GPIO_OUT_SPEC>;
#[doc = "Register `RTC_GPIO_OUT` writer"]
pub type W = crate::W<RTC_GPIO_OUT_SPEC>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RTC_GPIO_OUT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
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
#[doc = "RTC_GPIO_OUT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_gpio_out::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_gpio_out::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RTC_GPIO_OUT_SPEC;
impl crate::RegisterSpec for RTC_GPIO_OUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtc_gpio_out::R`](R) reader structure"]
impl crate::Readable for RTC_GPIO_OUT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rtc_gpio_out::W`](W) writer structure"]
impl crate::Writable for RTC_GPIO_OUT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RTC_GPIO_OUT to value 0"]
impl crate::Resettable for RTC_GPIO_OUT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
