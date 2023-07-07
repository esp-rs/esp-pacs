#[doc = "Register `RTC_GPIO_OUT` reader"]
pub struct R(crate::R<RTC_GPIO_OUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_GPIO_OUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_GPIO_OUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_GPIO_OUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_GPIO_OUT` writer"]
pub struct W(crate::W<RTC_GPIO_OUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_GPIO_OUT_SPEC>;
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
impl From<crate::W<RTC_GPIO_OUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_GPIO_OUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA` reader - RTC GPIO 0 ~ 21 output data"]
pub type DATA_R = crate::FieldReader<u32>;
#[doc = "Field `DATA` writer - RTC GPIO 0 ~ 21 output data"]
pub type DATA_W<'a, const O: u8> = crate::FieldWriter<'a, RTC_GPIO_OUT_SPEC, 22, O, u32>;
impl R {
    #[doc = "Bits 10:31 - RTC GPIO 0 ~ 21 output data"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits >> 10) & 0x003f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTC_GPIO_OUT")
            .field("data", &format_args!("{}", self.data().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RTC_GPIO_OUT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 10:31 - RTC GPIO 0 ~ 21 output data"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<10> {
        DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC GPIO 0 ~ 21 output data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_gpio_out](index.html) module"]
pub struct RTC_GPIO_OUT_SPEC;
impl crate::RegisterSpec for RTC_GPIO_OUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_gpio_out::R](R) reader structure"]
impl crate::Readable for RTC_GPIO_OUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_gpio_out::W](W) writer structure"]
impl crate::Writable for RTC_GPIO_OUT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RTC_GPIO_OUT to value 0"]
impl crate::Resettable for RTC_GPIO_OUT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
