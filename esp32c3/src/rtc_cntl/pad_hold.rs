#[doc = "Register `PAD_HOLD` reader"]
pub struct R(crate::R<PAD_HOLD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PAD_HOLD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PAD_HOLD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PAD_HOLD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PAD_HOLD` writer"]
pub struct W(crate::W<PAD_HOLD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PAD_HOLD_SPEC>;
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
impl From<crate::W<PAD_HOLD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PAD_HOLD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTC_GPIO_PIN0_HOLD` reader - the hold configure of rtc gpio0"]
pub type RTC_GPIO_PIN0_HOLD_R = crate::BitReader<bool>;
#[doc = "Field `RTC_GPIO_PIN0_HOLD` writer - the hold configure of rtc gpio0"]
pub type RTC_GPIO_PIN0_HOLD_W<'a, const O: u8> = crate::BitWriter<'a, u32, PAD_HOLD_SPEC, bool, O>;
#[doc = "Field `RTC_GPIO_PIN1_HOLD` reader - the hold configure of rtc gpio1"]
pub type RTC_GPIO_PIN1_HOLD_R = crate::BitReader<bool>;
#[doc = "Field `RTC_GPIO_PIN1_HOLD` writer - the hold configure of rtc gpio1"]
pub type RTC_GPIO_PIN1_HOLD_W<'a, const O: u8> = crate::BitWriter<'a, u32, PAD_HOLD_SPEC, bool, O>;
#[doc = "Field `RTC_GPIO_PIN2_HOLD` reader - the hold configure of rtc gpio2"]
pub type RTC_GPIO_PIN2_HOLD_R = crate::BitReader<bool>;
#[doc = "Field `RTC_GPIO_PIN2_HOLD` writer - the hold configure of rtc gpio2"]
pub type RTC_GPIO_PIN2_HOLD_W<'a, const O: u8> = crate::BitWriter<'a, u32, PAD_HOLD_SPEC, bool, O>;
#[doc = "Field `RTC_GPIO_PIN3_HOLD` reader - the hold configure of rtc gpio3"]
pub type RTC_GPIO_PIN3_HOLD_R = crate::BitReader<bool>;
#[doc = "Field `RTC_GPIO_PIN3_HOLD` writer - the hold configure of rtc gpio3"]
pub type RTC_GPIO_PIN3_HOLD_W<'a, const O: u8> = crate::BitWriter<'a, u32, PAD_HOLD_SPEC, bool, O>;
#[doc = "Field `RTC_GPIO_PIN4_HOLD` reader - the hold configure of rtc gpio4"]
pub type RTC_GPIO_PIN4_HOLD_R = crate::BitReader<bool>;
#[doc = "Field `RTC_GPIO_PIN4_HOLD` writer - the hold configure of rtc gpio4"]
pub type RTC_GPIO_PIN4_HOLD_W<'a, const O: u8> = crate::BitWriter<'a, u32, PAD_HOLD_SPEC, bool, O>;
#[doc = "Field `RTC_GPIO_PIN5_HOLD` reader - the hold configure of rtc gpio5"]
pub type RTC_GPIO_PIN5_HOLD_R = crate::BitReader<bool>;
#[doc = "Field `RTC_GPIO_PIN5_HOLD` writer - the hold configure of rtc gpio5"]
pub type RTC_GPIO_PIN5_HOLD_W<'a, const O: u8> = crate::BitWriter<'a, u32, PAD_HOLD_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - the hold configure of rtc gpio0"]
    #[inline(always)]
    pub fn rtc_gpio_pin0_hold(&self) -> RTC_GPIO_PIN0_HOLD_R {
        RTC_GPIO_PIN0_HOLD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - the hold configure of rtc gpio1"]
    #[inline(always)]
    pub fn rtc_gpio_pin1_hold(&self) -> RTC_GPIO_PIN1_HOLD_R {
        RTC_GPIO_PIN1_HOLD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - the hold configure of rtc gpio2"]
    #[inline(always)]
    pub fn rtc_gpio_pin2_hold(&self) -> RTC_GPIO_PIN2_HOLD_R {
        RTC_GPIO_PIN2_HOLD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - the hold configure of rtc gpio3"]
    #[inline(always)]
    pub fn rtc_gpio_pin3_hold(&self) -> RTC_GPIO_PIN3_HOLD_R {
        RTC_GPIO_PIN3_HOLD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - the hold configure of rtc gpio4"]
    #[inline(always)]
    pub fn rtc_gpio_pin4_hold(&self) -> RTC_GPIO_PIN4_HOLD_R {
        RTC_GPIO_PIN4_HOLD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - the hold configure of rtc gpio5"]
    #[inline(always)]
    pub fn rtc_gpio_pin5_hold(&self) -> RTC_GPIO_PIN5_HOLD_R {
        RTC_GPIO_PIN5_HOLD_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - the hold configure of rtc gpio0"]
    #[inline(always)]
    pub fn rtc_gpio_pin0_hold(&mut self) -> RTC_GPIO_PIN0_HOLD_W<0> {
        RTC_GPIO_PIN0_HOLD_W::new(self)
    }
    #[doc = "Bit 1 - the hold configure of rtc gpio1"]
    #[inline(always)]
    pub fn rtc_gpio_pin1_hold(&mut self) -> RTC_GPIO_PIN1_HOLD_W<1> {
        RTC_GPIO_PIN1_HOLD_W::new(self)
    }
    #[doc = "Bit 2 - the hold configure of rtc gpio2"]
    #[inline(always)]
    pub fn rtc_gpio_pin2_hold(&mut self) -> RTC_GPIO_PIN2_HOLD_W<2> {
        RTC_GPIO_PIN2_HOLD_W::new(self)
    }
    #[doc = "Bit 3 - the hold configure of rtc gpio3"]
    #[inline(always)]
    pub fn rtc_gpio_pin3_hold(&mut self) -> RTC_GPIO_PIN3_HOLD_W<3> {
        RTC_GPIO_PIN3_HOLD_W::new(self)
    }
    #[doc = "Bit 4 - the hold configure of rtc gpio4"]
    #[inline(always)]
    pub fn rtc_gpio_pin4_hold(&mut self) -> RTC_GPIO_PIN4_HOLD_W<4> {
        RTC_GPIO_PIN4_HOLD_W::new(self)
    }
    #[doc = "Bit 5 - the hold configure of rtc gpio5"]
    #[inline(always)]
    pub fn rtc_gpio_pin5_hold(&mut self) -> RTC_GPIO_PIN5_HOLD_W<5> {
        RTC_GPIO_PIN5_HOLD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rtc configure register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pad_hold](index.html) module"]
pub struct PAD_HOLD_SPEC;
impl crate::RegisterSpec for PAD_HOLD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pad_hold::R](R) reader structure"]
impl crate::Readable for PAD_HOLD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pad_hold::W](W) writer structure"]
impl crate::Writable for PAD_HOLD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PAD_HOLD to value 0"]
impl crate::Resettable for PAD_HOLD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
