#[doc = "Register `CNTL_GPIO_WAKEUP` reader"]
pub struct R(crate::R<CNTL_GPIO_WAKEUP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CNTL_GPIO_WAKEUP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CNTL_GPIO_WAKEUP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CNTL_GPIO_WAKEUP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CNTL_GPIO_WAKEUP` writer"]
pub struct W(crate::W<CNTL_GPIO_WAKEUP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CNTL_GPIO_WAKEUP_SPEC>;
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
impl From<crate::W<CNTL_GPIO_WAKEUP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CNTL_GPIO_WAKEUP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPIO_WAKEUP_STATUS` reader - Need add desc"]
pub type GPIO_WAKEUP_STATUS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GPIO_WAKEUP_STATUS` writer - Need add desc"]
pub type GPIO_WAKEUP_STATUS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CNTL_GPIO_WAKEUP_SPEC, u8, u8, 6, O>;
#[doc = "Field `GPIO_WAKEUP_STATUS_CLR` reader - Need add desc"]
pub type GPIO_WAKEUP_STATUS_CLR_R = crate::BitReader<bool>;
#[doc = "Field `GPIO_WAKEUP_STATUS_CLR` writer - Need add desc"]
pub type GPIO_WAKEUP_STATUS_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CNTL_GPIO_WAKEUP_SPEC, bool, O>;
#[doc = "Field `GPIO_PIN_CLK_GATE` reader - Need add desc"]
pub type GPIO_PIN_CLK_GATE_R = crate::BitReader<bool>;
#[doc = "Field `GPIO_PIN_CLK_GATE` writer - Need add desc"]
pub type GPIO_PIN_CLK_GATE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CNTL_GPIO_WAKEUP_SPEC, bool, O>;
#[doc = "Field `GPIO_PIN5_INT_TYPE` reader - Need add desc"]
pub type GPIO_PIN5_INT_TYPE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GPIO_PIN5_INT_TYPE` writer - Need add desc"]
pub type GPIO_PIN5_INT_TYPE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CNTL_GPIO_WAKEUP_SPEC, u8, u8, 3, O>;
#[doc = "Field `GPIO_PIN4_INT_TYPE` reader - Need add desc"]
pub type GPIO_PIN4_INT_TYPE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GPIO_PIN4_INT_TYPE` writer - Need add desc"]
pub type GPIO_PIN4_INT_TYPE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CNTL_GPIO_WAKEUP_SPEC, u8, u8, 3, O>;
#[doc = "Field `GPIO_PIN3_INT_TYPE` reader - Need add desc"]
pub type GPIO_PIN3_INT_TYPE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GPIO_PIN3_INT_TYPE` writer - Need add desc"]
pub type GPIO_PIN3_INT_TYPE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CNTL_GPIO_WAKEUP_SPEC, u8, u8, 3, O>;
#[doc = "Field `GPIO_PIN2_INT_TYPE` reader - Need add desc"]
pub type GPIO_PIN2_INT_TYPE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GPIO_PIN2_INT_TYPE` writer - Need add desc"]
pub type GPIO_PIN2_INT_TYPE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CNTL_GPIO_WAKEUP_SPEC, u8, u8, 3, O>;
#[doc = "Field `GPIO_PIN1_INT_TYPE` reader - Need add desc"]
pub type GPIO_PIN1_INT_TYPE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GPIO_PIN1_INT_TYPE` writer - Need add desc"]
pub type GPIO_PIN1_INT_TYPE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CNTL_GPIO_WAKEUP_SPEC, u8, u8, 3, O>;
#[doc = "Field `GPIO_PIN0_INT_TYPE` reader - Need add desc"]
pub type GPIO_PIN0_INT_TYPE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GPIO_PIN0_INT_TYPE` writer - Need add desc"]
pub type GPIO_PIN0_INT_TYPE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CNTL_GPIO_WAKEUP_SPEC, u8, u8, 3, O>;
#[doc = "Field `GPIO_PIN5_WAKEUP_ENABLE` reader - Need add desc"]
pub type GPIO_PIN5_WAKEUP_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `GPIO_PIN5_WAKEUP_ENABLE` writer - Need add desc"]
pub type GPIO_PIN5_WAKEUP_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CNTL_GPIO_WAKEUP_SPEC, bool, O>;
#[doc = "Field `GPIO_PIN4_WAKEUP_ENABLE` reader - Need add desc"]
pub type GPIO_PIN4_WAKEUP_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `GPIO_PIN4_WAKEUP_ENABLE` writer - Need add desc"]
pub type GPIO_PIN4_WAKEUP_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CNTL_GPIO_WAKEUP_SPEC, bool, O>;
#[doc = "Field `GPIO_PIN3_WAKEUP_ENABLE` reader - Need add desc"]
pub type GPIO_PIN3_WAKEUP_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `GPIO_PIN3_WAKEUP_ENABLE` writer - Need add desc"]
pub type GPIO_PIN3_WAKEUP_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CNTL_GPIO_WAKEUP_SPEC, bool, O>;
#[doc = "Field `GPIO_PIN2_WAKEUP_ENABLE` reader - Need add desc"]
pub type GPIO_PIN2_WAKEUP_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `GPIO_PIN2_WAKEUP_ENABLE` writer - Need add desc"]
pub type GPIO_PIN2_WAKEUP_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CNTL_GPIO_WAKEUP_SPEC, bool, O>;
#[doc = "Field `GPIO_PIN1_WAKEUP_ENABLE` reader - Need add desc"]
pub type GPIO_PIN1_WAKEUP_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `GPIO_PIN1_WAKEUP_ENABLE` writer - Need add desc"]
pub type GPIO_PIN1_WAKEUP_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CNTL_GPIO_WAKEUP_SPEC, bool, O>;
#[doc = "Field `GPIO_PIN0_WAKEUP_ENABLE` reader - Need add desc"]
pub type GPIO_PIN0_WAKEUP_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `GPIO_PIN0_WAKEUP_ENABLE` writer - Need add desc"]
pub type GPIO_PIN0_WAKEUP_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CNTL_GPIO_WAKEUP_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:5 - Need add desc"]
    #[inline(always)]
    pub fn gpio_wakeup_status(&self) -> GPIO_WAKEUP_STATUS_R {
        GPIO_WAKEUP_STATUS_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - Need add desc"]
    #[inline(always)]
    pub fn gpio_wakeup_status_clr(&self) -> GPIO_WAKEUP_STATUS_CLR_R {
        GPIO_WAKEUP_STATUS_CLR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Need add desc"]
    #[inline(always)]
    pub fn gpio_pin_clk_gate(&self) -> GPIO_PIN_CLK_GATE_R {
        GPIO_PIN_CLK_GATE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Need add desc"]
    #[inline(always)]
    pub fn gpio_pin5_int_type(&self) -> GPIO_PIN5_INT_TYPE_R {
        GPIO_PIN5_INT_TYPE_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:13 - Need add desc"]
    #[inline(always)]
    pub fn gpio_pin4_int_type(&self) -> GPIO_PIN4_INT_TYPE_R {
        GPIO_PIN4_INT_TYPE_R::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bits 14:16 - Need add desc"]
    #[inline(always)]
    pub fn gpio_pin3_int_type(&self) -> GPIO_PIN3_INT_TYPE_R {
        GPIO_PIN3_INT_TYPE_R::new(((self.bits >> 14) & 7) as u8)
    }
    #[doc = "Bits 17:19 - Need add desc"]
    #[inline(always)]
    pub fn gpio_pin2_int_type(&self) -> GPIO_PIN2_INT_TYPE_R {
        GPIO_PIN2_INT_TYPE_R::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bits 20:22 - Need add desc"]
    #[inline(always)]
    pub fn gpio_pin1_int_type(&self) -> GPIO_PIN1_INT_TYPE_R {
        GPIO_PIN1_INT_TYPE_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 23:25 - Need add desc"]
    #[inline(always)]
    pub fn gpio_pin0_int_type(&self) -> GPIO_PIN0_INT_TYPE_R {
        GPIO_PIN0_INT_TYPE_R::new(((self.bits >> 23) & 7) as u8)
    }
    #[doc = "Bit 26 - Need add desc"]
    #[inline(always)]
    pub fn gpio_pin5_wakeup_enable(&self) -> GPIO_PIN5_WAKEUP_ENABLE_R {
        GPIO_PIN5_WAKEUP_ENABLE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Need add desc"]
    #[inline(always)]
    pub fn gpio_pin4_wakeup_enable(&self) -> GPIO_PIN4_WAKEUP_ENABLE_R {
        GPIO_PIN4_WAKEUP_ENABLE_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Need add desc"]
    #[inline(always)]
    pub fn gpio_pin3_wakeup_enable(&self) -> GPIO_PIN3_WAKEUP_ENABLE_R {
        GPIO_PIN3_WAKEUP_ENABLE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Need add desc"]
    #[inline(always)]
    pub fn gpio_pin2_wakeup_enable(&self) -> GPIO_PIN2_WAKEUP_ENABLE_R {
        GPIO_PIN2_WAKEUP_ENABLE_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Need add desc"]
    #[inline(always)]
    pub fn gpio_pin1_wakeup_enable(&self) -> GPIO_PIN1_WAKEUP_ENABLE_R {
        GPIO_PIN1_WAKEUP_ENABLE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Need add desc"]
    #[inline(always)]
    pub fn gpio_pin0_wakeup_enable(&self) -> GPIO_PIN0_WAKEUP_ENABLE_R {
        GPIO_PIN0_WAKEUP_ENABLE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Need add desc"]
    #[inline(always)]
    pub fn gpio_wakeup_status(&mut self) -> GPIO_WAKEUP_STATUS_W<0> {
        GPIO_WAKEUP_STATUS_W::new(self)
    }
    #[doc = "Bit 6 - Need add desc"]
    #[inline(always)]
    pub fn gpio_wakeup_status_clr(&mut self) -> GPIO_WAKEUP_STATUS_CLR_W<6> {
        GPIO_WAKEUP_STATUS_CLR_W::new(self)
    }
    #[doc = "Bit 7 - Need add desc"]
    #[inline(always)]
    pub fn gpio_pin_clk_gate(&mut self) -> GPIO_PIN_CLK_GATE_W<7> {
        GPIO_PIN_CLK_GATE_W::new(self)
    }
    #[doc = "Bits 8:10 - Need add desc"]
    #[inline(always)]
    pub fn gpio_pin5_int_type(&mut self) -> GPIO_PIN5_INT_TYPE_W<8> {
        GPIO_PIN5_INT_TYPE_W::new(self)
    }
    #[doc = "Bits 11:13 - Need add desc"]
    #[inline(always)]
    pub fn gpio_pin4_int_type(&mut self) -> GPIO_PIN4_INT_TYPE_W<11> {
        GPIO_PIN4_INT_TYPE_W::new(self)
    }
    #[doc = "Bits 14:16 - Need add desc"]
    #[inline(always)]
    pub fn gpio_pin3_int_type(&mut self) -> GPIO_PIN3_INT_TYPE_W<14> {
        GPIO_PIN3_INT_TYPE_W::new(self)
    }
    #[doc = "Bits 17:19 - Need add desc"]
    #[inline(always)]
    pub fn gpio_pin2_int_type(&mut self) -> GPIO_PIN2_INT_TYPE_W<17> {
        GPIO_PIN2_INT_TYPE_W::new(self)
    }
    #[doc = "Bits 20:22 - Need add desc"]
    #[inline(always)]
    pub fn gpio_pin1_int_type(&mut self) -> GPIO_PIN1_INT_TYPE_W<20> {
        GPIO_PIN1_INT_TYPE_W::new(self)
    }
    #[doc = "Bits 23:25 - Need add desc"]
    #[inline(always)]
    pub fn gpio_pin0_int_type(&mut self) -> GPIO_PIN0_INT_TYPE_W<23> {
        GPIO_PIN0_INT_TYPE_W::new(self)
    }
    #[doc = "Bit 26 - Need add desc"]
    #[inline(always)]
    pub fn gpio_pin5_wakeup_enable(&mut self) -> GPIO_PIN5_WAKEUP_ENABLE_W<26> {
        GPIO_PIN5_WAKEUP_ENABLE_W::new(self)
    }
    #[doc = "Bit 27 - Need add desc"]
    #[inline(always)]
    pub fn gpio_pin4_wakeup_enable(&mut self) -> GPIO_PIN4_WAKEUP_ENABLE_W<27> {
        GPIO_PIN4_WAKEUP_ENABLE_W::new(self)
    }
    #[doc = "Bit 28 - Need add desc"]
    #[inline(always)]
    pub fn gpio_pin3_wakeup_enable(&mut self) -> GPIO_PIN3_WAKEUP_ENABLE_W<28> {
        GPIO_PIN3_WAKEUP_ENABLE_W::new(self)
    }
    #[doc = "Bit 29 - Need add desc"]
    #[inline(always)]
    pub fn gpio_pin2_wakeup_enable(&mut self) -> GPIO_PIN2_WAKEUP_ENABLE_W<29> {
        GPIO_PIN2_WAKEUP_ENABLE_W::new(self)
    }
    #[doc = "Bit 30 - Need add desc"]
    #[inline(always)]
    pub fn gpio_pin1_wakeup_enable(&mut self) -> GPIO_PIN1_WAKEUP_ENABLE_W<30> {
        GPIO_PIN1_WAKEUP_ENABLE_W::new(self)
    }
    #[doc = "Bit 31 - Need add desc"]
    #[inline(always)]
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
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cntl_gpio_wakeup](index.html) module"]
pub struct CNTL_GPIO_WAKEUP_SPEC;
impl crate::RegisterSpec for CNTL_GPIO_WAKEUP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cntl_gpio_wakeup::R](R) reader structure"]
impl crate::Readable for CNTL_GPIO_WAKEUP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cntl_gpio_wakeup::W](W) writer structure"]
impl crate::Writable for CNTL_GPIO_WAKEUP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CNTL_GPIO_WAKEUP to value 0"]
impl crate::Resettable for CNTL_GPIO_WAKEUP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
