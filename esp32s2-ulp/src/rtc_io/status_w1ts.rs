#[doc = "Register `STATUS_W1TS` writer"]
pub type W = crate::W<STATUS_W1TS_SPEC>;
#[doc = "Field `GPIO_STATUS_INT_W1TS` writer - GPIO0 ~ 21 interrupt set register. If the value 1 is written to a bit here, the corresponding bit in RTCIO_GPIO_STATUS_INT will be set to 1. Recommended operation: use this register to set RTCIO_GPIO_STATUS_INT."]
pub type GPIO_STATUS_INT_W1TS_W<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<STATUS_W1TS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 10:31 - GPIO0 ~ 21 interrupt set register. If the value 1 is written to a bit here, the corresponding bit in RTCIO_GPIO_STATUS_INT will be set to 1. Recommended operation: use this register to set RTCIO_GPIO_STATUS_INT."]
    #[inline(always)]
    pub fn gpio_status_int_w1ts(&mut self) -> GPIO_STATUS_INT_W1TS_W<STATUS_W1TS_SPEC> {
        GPIO_STATUS_INT_W1TS_W::new(self, 10)
    }
}
#[doc = "RTC GPIO interrupt status bit set register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status_w1ts::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUS_W1TS_SPEC;
impl crate::RegisterSpec for STATUS_W1TS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`status_w1ts::W`](W) writer structure"]
impl crate::Writable for STATUS_W1TS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STATUS_W1TS to value 0"]
impl crate::Resettable for STATUS_W1TS_SPEC {
    const RESET_VALUE: u32 = 0;
}
