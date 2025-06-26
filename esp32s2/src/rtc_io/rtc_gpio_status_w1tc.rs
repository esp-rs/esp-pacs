#[doc = "Register `RTC_GPIO_STATUS_W1TC` writer"]
pub type W = crate::W<RTC_GPIO_STATUS_W1TC_SPEC>;
#[doc = "Field `GPIO_STATUS_INT_W1TC` writer - GPIO0 ~ 21 interrupt clear register. If the value 1 is written to a bit here, the corresponding bit in RTCIO_GPIO_STATUS_INT will be cleared. Recommended operation: use this register to clear RTCIO_GPIO_STATUS_INT."]
pub type GPIO_STATUS_INT_W1TC_W<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RTC_GPIO_STATUS_W1TC_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 10:31 - GPIO0 ~ 21 interrupt clear register. If the value 1 is written to a bit here, the corresponding bit in RTCIO_GPIO_STATUS_INT will be cleared. Recommended operation: use this register to clear RTCIO_GPIO_STATUS_INT."]
    #[inline(always)]
    pub fn gpio_status_int_w1tc(&mut self) -> GPIO_STATUS_INT_W1TC_W<RTC_GPIO_STATUS_W1TC_SPEC> {
        GPIO_STATUS_INT_W1TC_W::new(self, 10)
    }
}
#[doc = "RTC GPIO interrupt status bit clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_gpio_status_w1tc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RTC_GPIO_STATUS_W1TC_SPEC;
impl crate::RegisterSpec for RTC_GPIO_STATUS_W1TC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`rtc_gpio_status_w1tc::W`](W) writer structure"]
impl crate::Writable for RTC_GPIO_STATUS_W1TC_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RTC_GPIO_STATUS_W1TC to value 0"]
impl crate::Resettable for RTC_GPIO_STATUS_W1TC_SPEC {}
