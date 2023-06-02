#[doc = "Register `RTC_GPIO_ENABLE` reader"]
pub struct R(crate::R<RTC_GPIO_ENABLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_GPIO_ENABLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_GPIO_ENABLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_GPIO_ENABLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_GPIO_ENABLE` writer"]
pub struct W(crate::W<RTC_GPIO_ENABLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_GPIO_ENABLE_SPEC>;
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
impl From<crate::W<RTC_GPIO_ENABLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_GPIO_ENABLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTC_GPIO_ENABLE` reader - RTC GPIO 0 ~ 21 enable"]
pub type RTC_GPIO_ENABLE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RTC_GPIO_ENABLE` writer - RTC GPIO 0 ~ 21 enable"]
pub type RTC_GPIO_ENABLE_W<'a, const O: u8> =
    crate::FieldWriter<'a, RTC_GPIO_ENABLE_SPEC, 22, O, u32, u32>;
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
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 10:31 - RTC GPIO 0 ~ 21 enable"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_gpio_enable(&mut self) -> RTC_GPIO_ENABLE_W<10> {
        RTC_GPIO_ENABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configure RTC GPIO output enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_gpio_enable](index.html) module"]
pub struct RTC_GPIO_ENABLE_SPEC;
impl crate::RegisterSpec for RTC_GPIO_ENABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_gpio_enable::R](R) reader structure"]
impl crate::Readable for RTC_GPIO_ENABLE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_gpio_enable::W](W) writer structure"]
impl crate::Writable for RTC_GPIO_ENABLE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RTC_GPIO_ENABLE to value 0"]
impl crate::Resettable for RTC_GPIO_ENABLE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
