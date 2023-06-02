#[doc = "Register `GPIO_WAKEUP` reader"]
pub struct R(crate::R<GPIO_WAKEUP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIO_WAKEUP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIO_WAKEUP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIO_WAKEUP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIO_WAKEUP` writer"]
pub struct W(crate::W<GPIO_WAKEUP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIO_WAKEUP_SPEC>;
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
impl From<crate::W<GPIO_WAKEUP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIO_WAKEUP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPIO_WAKEUP_STATUS` reader - rtc gpio wakeup flag"]
pub type GPIO_WAKEUP_STATUS_R = crate::FieldReader;
#[doc = "Field `GPIO_WAKEUP_STATUS_CLR` reader - clear rtc gpio wakeup flag"]
pub type GPIO_WAKEUP_STATUS_CLR_R = crate::BitReader;
#[doc = "Field `GPIO_WAKEUP_STATUS_CLR` writer - clear rtc gpio wakeup flag"]
pub type GPIO_WAKEUP_STATUS_CLR_W<'a, const O: u8> = crate::BitWriter<'a, GPIO_WAKEUP_SPEC, O>;
#[doc = "Field `GPIO_PIN_CLK_GATE` reader - enable rtc io clk gate"]
pub type GPIO_PIN_CLK_GATE_R = crate::BitReader;
#[doc = "Field `GPIO_PIN_CLK_GATE` writer - enable rtc io clk gate"]
pub type GPIO_PIN_CLK_GATE_W<'a, const O: u8> = crate::BitWriter<'a, GPIO_WAKEUP_SPEC, O>;
#[doc = "Field `GPIO_PIN5_INT_TYPE` reader - configure gpio wakeup type"]
pub type GPIO_PIN5_INT_TYPE_R = crate::FieldReader;
#[doc = "Field `GPIO_PIN5_INT_TYPE` writer - configure gpio wakeup type"]
pub type GPIO_PIN5_INT_TYPE_W<'a, const O: u8> = crate::FieldWriter<'a, GPIO_WAKEUP_SPEC, 3, O>;
#[doc = "Field `GPIO_PIN4_INT_TYPE` reader - configure gpio wakeup type"]
pub type GPIO_PIN4_INT_TYPE_R = crate::FieldReader;
#[doc = "Field `GPIO_PIN4_INT_TYPE` writer - configure gpio wakeup type"]
pub type GPIO_PIN4_INT_TYPE_W<'a, const O: u8> = crate::FieldWriter<'a, GPIO_WAKEUP_SPEC, 3, O>;
#[doc = "Field `GPIO_PIN3_INT_TYPE` reader - configure gpio wakeup type"]
pub type GPIO_PIN3_INT_TYPE_R = crate::FieldReader;
#[doc = "Field `GPIO_PIN3_INT_TYPE` writer - configure gpio wakeup type"]
pub type GPIO_PIN3_INT_TYPE_W<'a, const O: u8> = crate::FieldWriter<'a, GPIO_WAKEUP_SPEC, 3, O>;
#[doc = "Field `GPIO_PIN2_INT_TYPE` reader - configure gpio wakeup type"]
pub type GPIO_PIN2_INT_TYPE_R = crate::FieldReader;
#[doc = "Field `GPIO_PIN2_INT_TYPE` writer - configure gpio wakeup type"]
pub type GPIO_PIN2_INT_TYPE_W<'a, const O: u8> = crate::FieldWriter<'a, GPIO_WAKEUP_SPEC, 3, O>;
#[doc = "Field `GPIO_PIN1_INT_TYPE` reader - configure gpio wakeup type"]
pub type GPIO_PIN1_INT_TYPE_R = crate::FieldReader;
#[doc = "Field `GPIO_PIN1_INT_TYPE` writer - configure gpio wakeup type"]
pub type GPIO_PIN1_INT_TYPE_W<'a, const O: u8> = crate::FieldWriter<'a, GPIO_WAKEUP_SPEC, 3, O>;
#[doc = "Field `GPIO_PIN0_INT_TYPE` reader - configure gpio wakeup type"]
pub type GPIO_PIN0_INT_TYPE_R = crate::FieldReader;
#[doc = "Field `GPIO_PIN0_INT_TYPE` writer - configure gpio wakeup type"]
pub type GPIO_PIN0_INT_TYPE_W<'a, const O: u8> = crate::FieldWriter<'a, GPIO_WAKEUP_SPEC, 3, O>;
#[doc = "Field `GPIO_PIN5_WAKEUP_ENABLE` reader - enable wakeup from rtc gpio5"]
pub type GPIO_PIN5_WAKEUP_ENABLE_R = crate::BitReader;
#[doc = "Field `GPIO_PIN5_WAKEUP_ENABLE` writer - enable wakeup from rtc gpio5"]
pub type GPIO_PIN5_WAKEUP_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, GPIO_WAKEUP_SPEC, O>;
#[doc = "Field `GPIO_PIN4_WAKEUP_ENABLE` reader - enable wakeup from rtc gpio4"]
pub type GPIO_PIN4_WAKEUP_ENABLE_R = crate::BitReader;
#[doc = "Field `GPIO_PIN4_WAKEUP_ENABLE` writer - enable wakeup from rtc gpio4"]
pub type GPIO_PIN4_WAKEUP_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, GPIO_WAKEUP_SPEC, O>;
#[doc = "Field `GPIO_PIN3_WAKEUP_ENABLE` reader - enable wakeup from rtc gpio3"]
pub type GPIO_PIN3_WAKEUP_ENABLE_R = crate::BitReader;
#[doc = "Field `GPIO_PIN3_WAKEUP_ENABLE` writer - enable wakeup from rtc gpio3"]
pub type GPIO_PIN3_WAKEUP_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, GPIO_WAKEUP_SPEC, O>;
#[doc = "Field `GPIO_PIN2_WAKEUP_ENABLE` reader - enable wakeup from rtc gpio2"]
pub type GPIO_PIN2_WAKEUP_ENABLE_R = crate::BitReader;
#[doc = "Field `GPIO_PIN2_WAKEUP_ENABLE` writer - enable wakeup from rtc gpio2"]
pub type GPIO_PIN2_WAKEUP_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, GPIO_WAKEUP_SPEC, O>;
#[doc = "Field `GPIO_PIN1_WAKEUP_ENABLE` reader - enable wakeup from rtc gpio1"]
pub type GPIO_PIN1_WAKEUP_ENABLE_R = crate::BitReader;
#[doc = "Field `GPIO_PIN1_WAKEUP_ENABLE` writer - enable wakeup from rtc gpio1"]
pub type GPIO_PIN1_WAKEUP_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, GPIO_WAKEUP_SPEC, O>;
#[doc = "Field `GPIO_PIN0_WAKEUP_ENABLE` reader - enable wakeup from rtc gpio0"]
pub type GPIO_PIN0_WAKEUP_ENABLE_R = crate::BitReader;
#[doc = "Field `GPIO_PIN0_WAKEUP_ENABLE` writer - enable wakeup from rtc gpio0"]
pub type GPIO_PIN0_WAKEUP_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, GPIO_WAKEUP_SPEC, O>;
impl R {
    #[doc = "Bits 0:5 - rtc gpio wakeup flag"]
    #[inline(always)]
    pub fn gpio_wakeup_status(&self) -> GPIO_WAKEUP_STATUS_R {
        GPIO_WAKEUP_STATUS_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - clear rtc gpio wakeup flag"]
    #[inline(always)]
    pub fn gpio_wakeup_status_clr(&self) -> GPIO_WAKEUP_STATUS_CLR_R {
        GPIO_WAKEUP_STATUS_CLR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - enable rtc io clk gate"]
    #[inline(always)]
    pub fn gpio_pin_clk_gate(&self) -> GPIO_PIN_CLK_GATE_R {
        GPIO_PIN_CLK_GATE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - configure gpio wakeup type"]
    #[inline(always)]
    pub fn gpio_pin5_int_type(&self) -> GPIO_PIN5_INT_TYPE_R {
        GPIO_PIN5_INT_TYPE_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:13 - configure gpio wakeup type"]
    #[inline(always)]
    pub fn gpio_pin4_int_type(&self) -> GPIO_PIN4_INT_TYPE_R {
        GPIO_PIN4_INT_TYPE_R::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bits 14:16 - configure gpio wakeup type"]
    #[inline(always)]
    pub fn gpio_pin3_int_type(&self) -> GPIO_PIN3_INT_TYPE_R {
        GPIO_PIN3_INT_TYPE_R::new(((self.bits >> 14) & 7) as u8)
    }
    #[doc = "Bits 17:19 - configure gpio wakeup type"]
    #[inline(always)]
    pub fn gpio_pin2_int_type(&self) -> GPIO_PIN2_INT_TYPE_R {
        GPIO_PIN2_INT_TYPE_R::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bits 20:22 - configure gpio wakeup type"]
    #[inline(always)]
    pub fn gpio_pin1_int_type(&self) -> GPIO_PIN1_INT_TYPE_R {
        GPIO_PIN1_INT_TYPE_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 23:25 - configure gpio wakeup type"]
    #[inline(always)]
    pub fn gpio_pin0_int_type(&self) -> GPIO_PIN0_INT_TYPE_R {
        GPIO_PIN0_INT_TYPE_R::new(((self.bits >> 23) & 7) as u8)
    }
    #[doc = "Bit 26 - enable wakeup from rtc gpio5"]
    #[inline(always)]
    pub fn gpio_pin5_wakeup_enable(&self) -> GPIO_PIN5_WAKEUP_ENABLE_R {
        GPIO_PIN5_WAKEUP_ENABLE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - enable wakeup from rtc gpio4"]
    #[inline(always)]
    pub fn gpio_pin4_wakeup_enable(&self) -> GPIO_PIN4_WAKEUP_ENABLE_R {
        GPIO_PIN4_WAKEUP_ENABLE_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - enable wakeup from rtc gpio3"]
    #[inline(always)]
    pub fn gpio_pin3_wakeup_enable(&self) -> GPIO_PIN3_WAKEUP_ENABLE_R {
        GPIO_PIN3_WAKEUP_ENABLE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - enable wakeup from rtc gpio2"]
    #[inline(always)]
    pub fn gpio_pin2_wakeup_enable(&self) -> GPIO_PIN2_WAKEUP_ENABLE_R {
        GPIO_PIN2_WAKEUP_ENABLE_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - enable wakeup from rtc gpio1"]
    #[inline(always)]
    pub fn gpio_pin1_wakeup_enable(&self) -> GPIO_PIN1_WAKEUP_ENABLE_R {
        GPIO_PIN1_WAKEUP_ENABLE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - enable wakeup from rtc gpio0"]
    #[inline(always)]
    pub fn gpio_pin0_wakeup_enable(&self) -> GPIO_PIN0_WAKEUP_ENABLE_R {
        GPIO_PIN0_WAKEUP_ENABLE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO_WAKEUP")
            .field(
                "gpio_wakeup_status",
                &format_args!("{}", self.gpio_wakeup_status().bits()),
            )
            .field(
                "gpio_wakeup_status_clr",
                &format_args!("{}", self.gpio_wakeup_status_clr().bit()),
            )
            .field(
                "gpio_pin_clk_gate",
                &format_args!("{}", self.gpio_pin_clk_gate().bit()),
            )
            .field(
                "gpio_pin5_int_type",
                &format_args!("{}", self.gpio_pin5_int_type().bits()),
            )
            .field(
                "gpio_pin4_int_type",
                &format_args!("{}", self.gpio_pin4_int_type().bits()),
            )
            .field(
                "gpio_pin3_int_type",
                &format_args!("{}", self.gpio_pin3_int_type().bits()),
            )
            .field(
                "gpio_pin2_int_type",
                &format_args!("{}", self.gpio_pin2_int_type().bits()),
            )
            .field(
                "gpio_pin1_int_type",
                &format_args!("{}", self.gpio_pin1_int_type().bits()),
            )
            .field(
                "gpio_pin0_int_type",
                &format_args!("{}", self.gpio_pin0_int_type().bits()),
            )
            .field(
                "gpio_pin5_wakeup_enable",
                &format_args!("{}", self.gpio_pin5_wakeup_enable().bit()),
            )
            .field(
                "gpio_pin4_wakeup_enable",
                &format_args!("{}", self.gpio_pin4_wakeup_enable().bit()),
            )
            .field(
                "gpio_pin3_wakeup_enable",
                &format_args!("{}", self.gpio_pin3_wakeup_enable().bit()),
            )
            .field(
                "gpio_pin2_wakeup_enable",
                &format_args!("{}", self.gpio_pin2_wakeup_enable().bit()),
            )
            .field(
                "gpio_pin1_wakeup_enable",
                &format_args!("{}", self.gpio_pin1_wakeup_enable().bit()),
            )
            .field(
                "gpio_pin0_wakeup_enable",
                &format_args!("{}", self.gpio_pin0_wakeup_enable().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<GPIO_WAKEUP_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 6 - clear rtc gpio wakeup flag"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_wakeup_status_clr(&mut self) -> GPIO_WAKEUP_STATUS_CLR_W<6> {
        GPIO_WAKEUP_STATUS_CLR_W::new(self)
    }
    #[doc = "Bit 7 - enable rtc io clk gate"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pin_clk_gate(&mut self) -> GPIO_PIN_CLK_GATE_W<7> {
        GPIO_PIN_CLK_GATE_W::new(self)
    }
    #[doc = "Bits 8:10 - configure gpio wakeup type"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pin5_int_type(&mut self) -> GPIO_PIN5_INT_TYPE_W<8> {
        GPIO_PIN5_INT_TYPE_W::new(self)
    }
    #[doc = "Bits 11:13 - configure gpio wakeup type"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pin4_int_type(&mut self) -> GPIO_PIN4_INT_TYPE_W<11> {
        GPIO_PIN4_INT_TYPE_W::new(self)
    }
    #[doc = "Bits 14:16 - configure gpio wakeup type"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pin3_int_type(&mut self) -> GPIO_PIN3_INT_TYPE_W<14> {
        GPIO_PIN3_INT_TYPE_W::new(self)
    }
    #[doc = "Bits 17:19 - configure gpio wakeup type"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pin2_int_type(&mut self) -> GPIO_PIN2_INT_TYPE_W<17> {
        GPIO_PIN2_INT_TYPE_W::new(self)
    }
    #[doc = "Bits 20:22 - configure gpio wakeup type"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pin1_int_type(&mut self) -> GPIO_PIN1_INT_TYPE_W<20> {
        GPIO_PIN1_INT_TYPE_W::new(self)
    }
    #[doc = "Bits 23:25 - configure gpio wakeup type"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pin0_int_type(&mut self) -> GPIO_PIN0_INT_TYPE_W<23> {
        GPIO_PIN0_INT_TYPE_W::new(self)
    }
    #[doc = "Bit 26 - enable wakeup from rtc gpio5"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pin5_wakeup_enable(&mut self) -> GPIO_PIN5_WAKEUP_ENABLE_W<26> {
        GPIO_PIN5_WAKEUP_ENABLE_W::new(self)
    }
    #[doc = "Bit 27 - enable wakeup from rtc gpio4"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pin4_wakeup_enable(&mut self) -> GPIO_PIN4_WAKEUP_ENABLE_W<27> {
        GPIO_PIN4_WAKEUP_ENABLE_W::new(self)
    }
    #[doc = "Bit 28 - enable wakeup from rtc gpio3"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pin3_wakeup_enable(&mut self) -> GPIO_PIN3_WAKEUP_ENABLE_W<28> {
        GPIO_PIN3_WAKEUP_ENABLE_W::new(self)
    }
    #[doc = "Bit 29 - enable wakeup from rtc gpio2"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pin2_wakeup_enable(&mut self) -> GPIO_PIN2_WAKEUP_ENABLE_W<29> {
        GPIO_PIN2_WAKEUP_ENABLE_W::new(self)
    }
    #[doc = "Bit 30 - enable wakeup from rtc gpio1"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pin1_wakeup_enable(&mut self) -> GPIO_PIN1_WAKEUP_ENABLE_W<30> {
        GPIO_PIN1_WAKEUP_ENABLE_W::new(self)
    }
    #[doc = "Bit 31 - enable wakeup from rtc gpio0"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pin0_wakeup_enable(&mut self) -> GPIO_PIN0_WAKEUP_ENABLE_W<31> {
        GPIO_PIN0_WAKEUP_ENABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rtc configure register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_wakeup](index.html) module"]
pub struct GPIO_WAKEUP_SPEC;
impl crate::RegisterSpec for GPIO_WAKEUP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpio_wakeup::R](R) reader structure"]
impl crate::Readable for GPIO_WAKEUP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpio_wakeup::W](W) writer structure"]
impl crate::Writable for GPIO_WAKEUP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPIO_WAKEUP to value 0"]
impl crate::Resettable for GPIO_WAKEUP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
