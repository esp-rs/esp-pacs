#[doc = "Register `RTC_CNTL_GPIO_WAKEUP` reader"]
pub struct R(crate::R<RTC_CNTL_GPIO_WAKEUP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_CNTL_GPIO_WAKEUP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_CNTL_GPIO_WAKEUP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_CNTL_GPIO_WAKEUP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_CNTL_GPIO_WAKEUP` writer"]
pub struct W(crate::W<RTC_CNTL_GPIO_WAKEUP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_CNTL_GPIO_WAKEUP_SPEC>;
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
impl From<crate::W<RTC_CNTL_GPIO_WAKEUP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_CNTL_GPIO_WAKEUP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTC_GPIO_WAKEUP_STATUS` reader - Need add desc"]
pub type RTC_GPIO_WAKEUP_STATUS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RTC_GPIO_WAKEUP_STATUS` writer - Need add desc"]
pub type RTC_GPIO_WAKEUP_STATUS_W<'a> =
    crate::FieldWriter<'a, u32, RTC_CNTL_GPIO_WAKEUP_SPEC, u8, u8, 6, 0>;
#[doc = "Field `RTC_GPIO_WAKEUP_STATUS_CLR` reader - Need add desc"]
pub type RTC_GPIO_WAKEUP_STATUS_CLR_R = crate::BitReader<bool>;
#[doc = "Field `RTC_GPIO_WAKEUP_STATUS_CLR` writer - Need add desc"]
pub type RTC_GPIO_WAKEUP_STATUS_CLR_W<'a> =
    crate::BitWriter<'a, u32, RTC_CNTL_GPIO_WAKEUP_SPEC, bool, 6>;
#[doc = "Field `RTC_GPIO_PIN_CLK_GATE` reader - Need add desc"]
pub type RTC_GPIO_PIN_CLK_GATE_R = crate::BitReader<bool>;
#[doc = "Field `RTC_GPIO_PIN_CLK_GATE` writer - Need add desc"]
pub type RTC_GPIO_PIN_CLK_GATE_W<'a> =
    crate::BitWriter<'a, u32, RTC_CNTL_GPIO_WAKEUP_SPEC, bool, 7>;
#[doc = "Field `RTC_GPIO_PIN5_INT_TYPE` reader - Need add desc"]
pub type RTC_GPIO_PIN5_INT_TYPE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RTC_GPIO_PIN5_INT_TYPE` writer - Need add desc"]
pub type RTC_GPIO_PIN5_INT_TYPE_W<'a> =
    crate::FieldWriter<'a, u32, RTC_CNTL_GPIO_WAKEUP_SPEC, u8, u8, 3, 8>;
#[doc = "Field `RTC_GPIO_PIN4_INT_TYPE` reader - Need add desc"]
pub type RTC_GPIO_PIN4_INT_TYPE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RTC_GPIO_PIN4_INT_TYPE` writer - Need add desc"]
pub type RTC_GPIO_PIN4_INT_TYPE_W<'a> =
    crate::FieldWriter<'a, u32, RTC_CNTL_GPIO_WAKEUP_SPEC, u8, u8, 3, 11>;
#[doc = "Field `RTC_GPIO_PIN3_INT_TYPE` reader - Need add desc"]
pub type RTC_GPIO_PIN3_INT_TYPE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RTC_GPIO_PIN3_INT_TYPE` writer - Need add desc"]
pub type RTC_GPIO_PIN3_INT_TYPE_W<'a> =
    crate::FieldWriter<'a, u32, RTC_CNTL_GPIO_WAKEUP_SPEC, u8, u8, 3, 14>;
#[doc = "Field `RTC_GPIO_PIN2_INT_TYPE` reader - Need add desc"]
pub type RTC_GPIO_PIN2_INT_TYPE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RTC_GPIO_PIN2_INT_TYPE` writer - Need add desc"]
pub type RTC_GPIO_PIN2_INT_TYPE_W<'a> =
    crate::FieldWriter<'a, u32, RTC_CNTL_GPIO_WAKEUP_SPEC, u8, u8, 3, 17>;
#[doc = "Field `RTC_GPIO_PIN1_INT_TYPE` reader - Need add desc"]
pub type RTC_GPIO_PIN1_INT_TYPE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RTC_GPIO_PIN1_INT_TYPE` writer - Need add desc"]
pub type RTC_GPIO_PIN1_INT_TYPE_W<'a> =
    crate::FieldWriter<'a, u32, RTC_CNTL_GPIO_WAKEUP_SPEC, u8, u8, 3, 20>;
#[doc = "Field `RTC_GPIO_PIN0_INT_TYPE` reader - Need add desc"]
pub type RTC_GPIO_PIN0_INT_TYPE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RTC_GPIO_PIN0_INT_TYPE` writer - Need add desc"]
pub type RTC_GPIO_PIN0_INT_TYPE_W<'a> =
    crate::FieldWriter<'a, u32, RTC_CNTL_GPIO_WAKEUP_SPEC, u8, u8, 3, 23>;
#[doc = "Field `RTC_GPIO_PIN5_WAKEUP_ENABLE` reader - Need add desc"]
pub type RTC_GPIO_PIN5_WAKEUP_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `RTC_GPIO_PIN5_WAKEUP_ENABLE` writer - Need add desc"]
pub type RTC_GPIO_PIN5_WAKEUP_ENABLE_W<'a> =
    crate::BitWriter<'a, u32, RTC_CNTL_GPIO_WAKEUP_SPEC, bool, 26>;
#[doc = "Field `RTC_GPIO_PIN4_WAKEUP_ENABLE` reader - Need add desc"]
pub type RTC_GPIO_PIN4_WAKEUP_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `RTC_GPIO_PIN4_WAKEUP_ENABLE` writer - Need add desc"]
pub type RTC_GPIO_PIN4_WAKEUP_ENABLE_W<'a> =
    crate::BitWriter<'a, u32, RTC_CNTL_GPIO_WAKEUP_SPEC, bool, 27>;
#[doc = "Field `RTC_GPIO_PIN3_WAKEUP_ENABLE` reader - Need add desc"]
pub type RTC_GPIO_PIN3_WAKEUP_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `RTC_GPIO_PIN3_WAKEUP_ENABLE` writer - Need add desc"]
pub type RTC_GPIO_PIN3_WAKEUP_ENABLE_W<'a> =
    crate::BitWriter<'a, u32, RTC_CNTL_GPIO_WAKEUP_SPEC, bool, 28>;
#[doc = "Field `RTC_GPIO_PIN2_WAKEUP_ENABLE` reader - Need add desc"]
pub type RTC_GPIO_PIN2_WAKEUP_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `RTC_GPIO_PIN2_WAKEUP_ENABLE` writer - Need add desc"]
pub type RTC_GPIO_PIN2_WAKEUP_ENABLE_W<'a> =
    crate::BitWriter<'a, u32, RTC_CNTL_GPIO_WAKEUP_SPEC, bool, 29>;
#[doc = "Field `RTC_GPIO_PIN1_WAKEUP_ENABLE` reader - Need add desc"]
pub type RTC_GPIO_PIN1_WAKEUP_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `RTC_GPIO_PIN1_WAKEUP_ENABLE` writer - Need add desc"]
pub type RTC_GPIO_PIN1_WAKEUP_ENABLE_W<'a> =
    crate::BitWriter<'a, u32, RTC_CNTL_GPIO_WAKEUP_SPEC, bool, 30>;
#[doc = "Field `RTC_GPIO_PIN0_WAKEUP_ENABLE` reader - Need add desc"]
pub type RTC_GPIO_PIN0_WAKEUP_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `RTC_GPIO_PIN0_WAKEUP_ENABLE` writer - Need add desc"]
pub type RTC_GPIO_PIN0_WAKEUP_ENABLE_W<'a> =
    crate::BitWriter<'a, u32, RTC_CNTL_GPIO_WAKEUP_SPEC, bool, 31>;
impl R {
    #[doc = "Bits 0:5 - Need add desc"]
    #[inline(always)]
    pub fn rtc_gpio_wakeup_status(&self) -> RTC_GPIO_WAKEUP_STATUS_R {
        RTC_GPIO_WAKEUP_STATUS_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - Need add desc"]
    #[inline(always)]
    pub fn rtc_gpio_wakeup_status_clr(&self) -> RTC_GPIO_WAKEUP_STATUS_CLR_R {
        RTC_GPIO_WAKEUP_STATUS_CLR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Need add desc"]
    #[inline(always)]
    pub fn rtc_gpio_pin_clk_gate(&self) -> RTC_GPIO_PIN_CLK_GATE_R {
        RTC_GPIO_PIN_CLK_GATE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Need add desc"]
    #[inline(always)]
    pub fn rtc_gpio_pin5_int_type(&self) -> RTC_GPIO_PIN5_INT_TYPE_R {
        RTC_GPIO_PIN5_INT_TYPE_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:13 - Need add desc"]
    #[inline(always)]
    pub fn rtc_gpio_pin4_int_type(&self) -> RTC_GPIO_PIN4_INT_TYPE_R {
        RTC_GPIO_PIN4_INT_TYPE_R::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bits 14:16 - Need add desc"]
    #[inline(always)]
    pub fn rtc_gpio_pin3_int_type(&self) -> RTC_GPIO_PIN3_INT_TYPE_R {
        RTC_GPIO_PIN3_INT_TYPE_R::new(((self.bits >> 14) & 7) as u8)
    }
    #[doc = "Bits 17:19 - Need add desc"]
    #[inline(always)]
    pub fn rtc_gpio_pin2_int_type(&self) -> RTC_GPIO_PIN2_INT_TYPE_R {
        RTC_GPIO_PIN2_INT_TYPE_R::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bits 20:22 - Need add desc"]
    #[inline(always)]
    pub fn rtc_gpio_pin1_int_type(&self) -> RTC_GPIO_PIN1_INT_TYPE_R {
        RTC_GPIO_PIN1_INT_TYPE_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 23:25 - Need add desc"]
    #[inline(always)]
    pub fn rtc_gpio_pin0_int_type(&self) -> RTC_GPIO_PIN0_INT_TYPE_R {
        RTC_GPIO_PIN0_INT_TYPE_R::new(((self.bits >> 23) & 7) as u8)
    }
    #[doc = "Bit 26 - Need add desc"]
    #[inline(always)]
    pub fn rtc_gpio_pin5_wakeup_enable(&self) -> RTC_GPIO_PIN5_WAKEUP_ENABLE_R {
        RTC_GPIO_PIN5_WAKEUP_ENABLE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Need add desc"]
    #[inline(always)]
    pub fn rtc_gpio_pin4_wakeup_enable(&self) -> RTC_GPIO_PIN4_WAKEUP_ENABLE_R {
        RTC_GPIO_PIN4_WAKEUP_ENABLE_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Need add desc"]
    #[inline(always)]
    pub fn rtc_gpio_pin3_wakeup_enable(&self) -> RTC_GPIO_PIN3_WAKEUP_ENABLE_R {
        RTC_GPIO_PIN3_WAKEUP_ENABLE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Need add desc"]
    #[inline(always)]
    pub fn rtc_gpio_pin2_wakeup_enable(&self) -> RTC_GPIO_PIN2_WAKEUP_ENABLE_R {
        RTC_GPIO_PIN2_WAKEUP_ENABLE_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Need add desc"]
    #[inline(always)]
    pub fn rtc_gpio_pin1_wakeup_enable(&self) -> RTC_GPIO_PIN1_WAKEUP_ENABLE_R {
        RTC_GPIO_PIN1_WAKEUP_ENABLE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Need add desc"]
    #[inline(always)]
    pub fn rtc_gpio_pin0_wakeup_enable(&self) -> RTC_GPIO_PIN0_WAKEUP_ENABLE_R {
        RTC_GPIO_PIN0_WAKEUP_ENABLE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Need add desc"]
    #[inline(always)]
    pub fn rtc_gpio_wakeup_status(&mut self) -> RTC_GPIO_WAKEUP_STATUS_W {
        RTC_GPIO_WAKEUP_STATUS_W::new(self)
    }
    #[doc = "Bit 6 - Need add desc"]
    #[inline(always)]
    pub fn rtc_gpio_wakeup_status_clr(&mut self) -> RTC_GPIO_WAKEUP_STATUS_CLR_W {
        RTC_GPIO_WAKEUP_STATUS_CLR_W::new(self)
    }
    #[doc = "Bit 7 - Need add desc"]
    #[inline(always)]
    pub fn rtc_gpio_pin_clk_gate(&mut self) -> RTC_GPIO_PIN_CLK_GATE_W {
        RTC_GPIO_PIN_CLK_GATE_W::new(self)
    }
    #[doc = "Bits 8:10 - Need add desc"]
    #[inline(always)]
    pub fn rtc_gpio_pin5_int_type(&mut self) -> RTC_GPIO_PIN5_INT_TYPE_W {
        RTC_GPIO_PIN5_INT_TYPE_W::new(self)
    }
    #[doc = "Bits 11:13 - Need add desc"]
    #[inline(always)]
    pub fn rtc_gpio_pin4_int_type(&mut self) -> RTC_GPIO_PIN4_INT_TYPE_W {
        RTC_GPIO_PIN4_INT_TYPE_W::new(self)
    }
    #[doc = "Bits 14:16 - Need add desc"]
    #[inline(always)]
    pub fn rtc_gpio_pin3_int_type(&mut self) -> RTC_GPIO_PIN3_INT_TYPE_W {
        RTC_GPIO_PIN3_INT_TYPE_W::new(self)
    }
    #[doc = "Bits 17:19 - Need add desc"]
    #[inline(always)]
    pub fn rtc_gpio_pin2_int_type(&mut self) -> RTC_GPIO_PIN2_INT_TYPE_W {
        RTC_GPIO_PIN2_INT_TYPE_W::new(self)
    }
    #[doc = "Bits 20:22 - Need add desc"]
    #[inline(always)]
    pub fn rtc_gpio_pin1_int_type(&mut self) -> RTC_GPIO_PIN1_INT_TYPE_W {
        RTC_GPIO_PIN1_INT_TYPE_W::new(self)
    }
    #[doc = "Bits 23:25 - Need add desc"]
    #[inline(always)]
    pub fn rtc_gpio_pin0_int_type(&mut self) -> RTC_GPIO_PIN0_INT_TYPE_W {
        RTC_GPIO_PIN0_INT_TYPE_W::new(self)
    }
    #[doc = "Bit 26 - Need add desc"]
    #[inline(always)]
    pub fn rtc_gpio_pin5_wakeup_enable(&mut self) -> RTC_GPIO_PIN5_WAKEUP_ENABLE_W {
        RTC_GPIO_PIN5_WAKEUP_ENABLE_W::new(self)
    }
    #[doc = "Bit 27 - Need add desc"]
    #[inline(always)]
    pub fn rtc_gpio_pin4_wakeup_enable(&mut self) -> RTC_GPIO_PIN4_WAKEUP_ENABLE_W {
        RTC_GPIO_PIN4_WAKEUP_ENABLE_W::new(self)
    }
    #[doc = "Bit 28 - Need add desc"]
    #[inline(always)]
    pub fn rtc_gpio_pin3_wakeup_enable(&mut self) -> RTC_GPIO_PIN3_WAKEUP_ENABLE_W {
        RTC_GPIO_PIN3_WAKEUP_ENABLE_W::new(self)
    }
    #[doc = "Bit 29 - Need add desc"]
    #[inline(always)]
    pub fn rtc_gpio_pin2_wakeup_enable(&mut self) -> RTC_GPIO_PIN2_WAKEUP_ENABLE_W {
        RTC_GPIO_PIN2_WAKEUP_ENABLE_W::new(self)
    }
    #[doc = "Bit 30 - Need add desc"]
    #[inline(always)]
    pub fn rtc_gpio_pin1_wakeup_enable(&mut self) -> RTC_GPIO_PIN1_WAKEUP_ENABLE_W {
        RTC_GPIO_PIN1_WAKEUP_ENABLE_W::new(self)
    }
    #[doc = "Bit 31 - Need add desc"]
    #[inline(always)]
    pub fn rtc_gpio_pin0_wakeup_enable(&mut self) -> RTC_GPIO_PIN0_WAKEUP_ENABLE_W {
        RTC_GPIO_PIN0_WAKEUP_ENABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_cntl_gpio_wakeup](index.html) module"]
pub struct RTC_CNTL_GPIO_WAKEUP_SPEC;
impl crate::RegisterSpec for RTC_CNTL_GPIO_WAKEUP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_cntl_gpio_wakeup::R](R) reader structure"]
impl crate::Readable for RTC_CNTL_GPIO_WAKEUP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_gpio_wakeup::W](W) writer structure"]
impl crate::Writable for RTC_CNTL_GPIO_WAKEUP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_CNTL_GPIO_WAKEUP to value 0"]
impl crate::Resettable for RTC_CNTL_GPIO_WAKEUP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
