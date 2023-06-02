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
#[doc = "Field `GPIO_OUT_DATA` reader - GPIO0 ~ 21 output register. Bit10 corresponds to GPIO0, bit11 corresponds to GPIO1, etc."]
pub type GPIO_OUT_DATA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `GPIO_OUT_DATA` writer - GPIO0 ~ 21 output register. Bit10 corresponds to GPIO0, bit11 corresponds to GPIO1, etc."]
pub type GPIO_OUT_DATA_W<'a, const O: u8> =
    crate::FieldWriter<'a, RTC_GPIO_OUT_SPEC, 22, O, u32, u32>;
impl R {
    #[doc = "Bits 10:31 - GPIO0 ~ 21 output register. Bit10 corresponds to GPIO0, bit11 corresponds to GPIO1, etc."]
    #[inline(always)]
    pub fn gpio_out_data(&self) -> GPIO_OUT_DATA_R {
        GPIO_OUT_DATA_R::new((self.bits >> 10) & 0x003f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTC_GPIO_OUT")
            .field(
                "gpio_out_data",
                &format_args!("{}", self.gpio_out_data().bits()),
            )
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
    #[doc = "Bits 10:31 - GPIO0 ~ 21 output register. Bit10 corresponds to GPIO0, bit11 corresponds to GPIO1, etc."]
    #[inline(always)]
    #[must_use]
    pub fn gpio_out_data(&mut self) -> GPIO_OUT_DATA_W<10> {
        GPIO_OUT_DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC GPIO output register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_gpio_out](index.html) module"]
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
