#[doc = "Register `RTC_GPIO_OUT_W1TC` writer"]
pub type W = crate::W<RTC_GPIO_OUT_W1TC_SPEC>;
#[doc = "Field `RTC_GPIO_OUT_DATA_W1TC` writer - RTC GPIO 0 ~ 21 output data write 1 to clear"]
pub type RTC_GPIO_OUT_DATA_W1TC_W<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RTC_GPIO_OUT_W1TC_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 10:31 - RTC GPIO 0 ~ 21 output data write 1 to clear"]
    #[inline(always)]
    pub fn rtc_gpio_out_data_w1tc(
        &mut self,
    ) -> RTC_GPIO_OUT_DATA_W1TC_W<'_, RTC_GPIO_OUT_W1TC_SPEC> {
        RTC_GPIO_OUT_DATA_W1TC_W::new(self, 10)
    }
}
#[doc = "one clear RTC GPIO output data\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_gpio_out_w1tc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RTC_GPIO_OUT_W1TC_SPEC;
impl crate::RegisterSpec for RTC_GPIO_OUT_W1TC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`rtc_gpio_out_w1tc::W`](W) writer structure"]
impl crate::Writable for RTC_GPIO_OUT_W1TC_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RTC_GPIO_OUT_W1TC to value 0"]
impl crate::Resettable for RTC_GPIO_OUT_W1TC_SPEC {}
