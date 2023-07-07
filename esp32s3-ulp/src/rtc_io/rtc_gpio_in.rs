#[doc = "Register `RTC_GPIO_IN` reader"]
pub struct R(crate::R<RTC_GPIO_IN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_GPIO_IN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_GPIO_IN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_GPIO_IN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `NEXT` reader - RTC GPIO input data"]
pub type NEXT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 10:31 - RTC GPIO input data"]
    #[inline(always)]
    pub fn next(&self) -> NEXT_R {
        NEXT_R::new((self.bits >> 10) & 0x003f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTC_GPIO_IN")
            .field("next", &format_args!("{}", self.next().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RTC_GPIO_IN_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "RTC GPIO input data\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_gpio_in](index.html) module"]
pub struct RTC_GPIO_IN_SPEC;
impl crate::RegisterSpec for RTC_GPIO_IN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_gpio_in::R](R) reader structure"]
impl crate::Readable for RTC_GPIO_IN_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RTC_GPIO_IN to value 0"]
impl crate::Resettable for RTC_GPIO_IN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
