#[doc = "Register `RTC_GPIO_STATUS_W1TS` writer"]
pub struct W(crate::W<RTC_GPIO_STATUS_W1TS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_GPIO_STATUS_W1TS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<RTC_GPIO_STATUS_W1TS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_GPIO_STATUS_W1TS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTC_GPIO_STATUS_INT_W1TS` writer - RTC GPIO 0 ~ 21 interrupt status write 1 to set"]
pub type RTC_GPIO_STATUS_INT_W1TS_W<'a, const O: u8> =
    crate::FieldWriter<'a, RTC_GPIO_STATUS_W1TS_SPEC, 22, O, u32>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RTC_GPIO_STATUS_W1TS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 10:31 - RTC GPIO 0 ~ 21 interrupt status write 1 to set"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_gpio_status_int_w1ts(&mut self) -> RTC_GPIO_STATUS_INT_W1TS_W<10> {
        RTC_GPIO_STATUS_INT_W1TS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "One set RTC GPIO 0 ~ 21 interrupt status\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_gpio_status_w1ts](index.html) module"]
pub struct RTC_GPIO_STATUS_W1TS_SPEC;
impl crate::RegisterSpec for RTC_GPIO_STATUS_W1TS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [rtc_gpio_status_w1ts::W](W) writer structure"]
impl crate::Writable for RTC_GPIO_STATUS_W1TS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RTC_GPIO_STATUS_W1TS to value 0"]
impl crate::Resettable for RTC_GPIO_STATUS_W1TS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
