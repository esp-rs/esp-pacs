#[doc = "Register `RTC_PAD_HOLD` reader"]
pub struct R(crate::R<RTC_PAD_HOLD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_PAD_HOLD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_PAD_HOLD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_PAD_HOLD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_PAD_HOLD` writer"]
pub struct W(crate::W<RTC_PAD_HOLD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_PAD_HOLD_SPEC>;
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
impl From<crate::W<RTC_PAD_HOLD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_PAD_HOLD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTC_GPIO_PIN0_HOLD` reader - Need add desc"]
pub type RTC_GPIO_PIN0_HOLD_R = crate::BitReader<bool>;
#[doc = "Field `RTC_GPIO_PIN0_HOLD` writer - Need add desc"]
pub type RTC_GPIO_PIN0_HOLD_W<'a> = crate::BitWriter<'a, u32, RTC_PAD_HOLD_SPEC, bool, 0>;
#[doc = "Field `RTC_GPIO_PIN1_HOLD` reader - Need add desc"]
pub type RTC_GPIO_PIN1_HOLD_R = crate::BitReader<bool>;
#[doc = "Field `RTC_GPIO_PIN1_HOLD` writer - Need add desc"]
pub type RTC_GPIO_PIN1_HOLD_W<'a> = crate::BitWriter<'a, u32, RTC_PAD_HOLD_SPEC, bool, 1>;
#[doc = "Field `RTC_GPIO_PIN2_HOLD` reader - Need add desc"]
pub type RTC_GPIO_PIN2_HOLD_R = crate::BitReader<bool>;
#[doc = "Field `RTC_GPIO_PIN2_HOLD` writer - Need add desc"]
pub type RTC_GPIO_PIN2_HOLD_W<'a> = crate::BitWriter<'a, u32, RTC_PAD_HOLD_SPEC, bool, 2>;
#[doc = "Field `RTC_GPIO_PIN3_HOLD` reader - Need add desc"]
pub type RTC_GPIO_PIN3_HOLD_R = crate::BitReader<bool>;
#[doc = "Field `RTC_GPIO_PIN3_HOLD` writer - Need add desc"]
pub type RTC_GPIO_PIN3_HOLD_W<'a> = crate::BitWriter<'a, u32, RTC_PAD_HOLD_SPEC, bool, 3>;
#[doc = "Field `RTC_GPIO_PIN4_HOLD` reader - Need add desc"]
pub type RTC_GPIO_PIN4_HOLD_R = crate::BitReader<bool>;
#[doc = "Field `RTC_GPIO_PIN4_HOLD` writer - Need add desc"]
pub type RTC_GPIO_PIN4_HOLD_W<'a> = crate::BitWriter<'a, u32, RTC_PAD_HOLD_SPEC, bool, 4>;
#[doc = "Field `RTC_GPIO_PIN5_HOLD` reader - Need add desc"]
pub type RTC_GPIO_PIN5_HOLD_R = crate::BitReader<bool>;
#[doc = "Field `RTC_GPIO_PIN5_HOLD` writer - Need add desc"]
pub type RTC_GPIO_PIN5_HOLD_W<'a> = crate::BitWriter<'a, u32, RTC_PAD_HOLD_SPEC, bool, 5>;
impl R {
    #[doc = "Bit 0 - Need add desc"]
    #[inline(always)]
    pub fn rtc_gpio_pin0_hold(&self) -> RTC_GPIO_PIN0_HOLD_R {
        RTC_GPIO_PIN0_HOLD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Need add desc"]
    #[inline(always)]
    pub fn rtc_gpio_pin1_hold(&self) -> RTC_GPIO_PIN1_HOLD_R {
        RTC_GPIO_PIN1_HOLD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Need add desc"]
    #[inline(always)]
    pub fn rtc_gpio_pin2_hold(&self) -> RTC_GPIO_PIN2_HOLD_R {
        RTC_GPIO_PIN2_HOLD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Need add desc"]
    #[inline(always)]
    pub fn rtc_gpio_pin3_hold(&self) -> RTC_GPIO_PIN3_HOLD_R {
        RTC_GPIO_PIN3_HOLD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Need add desc"]
    #[inline(always)]
    pub fn rtc_gpio_pin4_hold(&self) -> RTC_GPIO_PIN4_HOLD_R {
        RTC_GPIO_PIN4_HOLD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Need add desc"]
    #[inline(always)]
    pub fn rtc_gpio_pin5_hold(&self) -> RTC_GPIO_PIN5_HOLD_R {
        RTC_GPIO_PIN5_HOLD_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Need add desc"]
    #[inline(always)]
    pub fn rtc_gpio_pin0_hold(&mut self) -> RTC_GPIO_PIN0_HOLD_W {
        RTC_GPIO_PIN0_HOLD_W::new(self)
    }
    #[doc = "Bit 1 - Need add desc"]
    #[inline(always)]
    pub fn rtc_gpio_pin1_hold(&mut self) -> RTC_GPIO_PIN1_HOLD_W {
        RTC_GPIO_PIN1_HOLD_W::new(self)
    }
    #[doc = "Bit 2 - Need add desc"]
    #[inline(always)]
    pub fn rtc_gpio_pin2_hold(&mut self) -> RTC_GPIO_PIN2_HOLD_W {
        RTC_GPIO_PIN2_HOLD_W::new(self)
    }
    #[doc = "Bit 3 - Need add desc"]
    #[inline(always)]
    pub fn rtc_gpio_pin3_hold(&mut self) -> RTC_GPIO_PIN3_HOLD_W {
        RTC_GPIO_PIN3_HOLD_W::new(self)
    }
    #[doc = "Bit 4 - Need add desc"]
    #[inline(always)]
    pub fn rtc_gpio_pin4_hold(&mut self) -> RTC_GPIO_PIN4_HOLD_W {
        RTC_GPIO_PIN4_HOLD_W::new(self)
    }
    #[doc = "Bit 5 - Need add desc"]
    #[inline(always)]
    pub fn rtc_gpio_pin5_hold(&mut self) -> RTC_GPIO_PIN5_HOLD_W {
        RTC_GPIO_PIN5_HOLD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_pad_hold](index.html) module"]
pub struct RTC_PAD_HOLD_SPEC;
impl crate::RegisterSpec for RTC_PAD_HOLD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_pad_hold::R](R) reader structure"]
impl crate::Readable for RTC_PAD_HOLD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_pad_hold::W](W) writer structure"]
impl crate::Writable for RTC_PAD_HOLD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_PAD_HOLD to value 0"]
impl crate::Resettable for RTC_PAD_HOLD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
