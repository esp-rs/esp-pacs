#[doc = "Register `ENABLE_W1TC` writer"]
pub type W = crate::W<ENABLE_W1TC_SPEC>;
#[doc = "Field `REG_RTCIO_REG_GPIO_ENABLE_W1TC` writer - GPIO0 ~ 21 output enable clear register. If the value 1 is written to a bit here, the corresponding bit in RTCIO_RTC_GPIO_ENABLE_REG will be cleared. Recommended operation: use this register to clear RTCIO_RTC_GPIO_ENABLE_REG."]
pub type REG_RTCIO_REG_GPIO_ENABLE_W1TC_W<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<ENABLE_W1TC_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 10:31 - GPIO0 ~ 21 output enable clear register. If the value 1 is written to a bit here, the corresponding bit in RTCIO_RTC_GPIO_ENABLE_REG will be cleared. Recommended operation: use this register to clear RTCIO_RTC_GPIO_ENABLE_REG."]
    #[inline(always)]
    pub fn reg_rtcio_reg_gpio_enable_w1tc(
        &mut self,
    ) -> REG_RTCIO_REG_GPIO_ENABLE_W1TC_W<'_, ENABLE_W1TC_SPEC> {
        REG_RTCIO_REG_GPIO_ENABLE_W1TC_W::new(self, 10)
    }
}
#[doc = "RTC GPIO output enable bit clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enable_w1tc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ENABLE_W1TC_SPEC;
impl crate::RegisterSpec for ENABLE_W1TC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`enable_w1tc::W`](W) writer structure"]
impl crate::Writable for ENABLE_W1TC_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ENABLE_W1TC to value 0"]
impl crate::Resettable for ENABLE_W1TC_SPEC {}
