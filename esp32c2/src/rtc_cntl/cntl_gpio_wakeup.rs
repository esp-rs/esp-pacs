#[doc = "Register `CNTL_GPIO_WAKEUP` reader"]
pub type R = crate::R<CNTL_GPIO_WAKEUP_SPEC>;
#[doc = "Register `CNTL_GPIO_WAKEUP` writer"]
pub type W = crate::W<CNTL_GPIO_WAKEUP_SPEC>;
#[doc = "Field `GPIO_WAKEUP_STATUS` reader - Need add desc"]
pub type GPIO_WAKEUP_STATUS_R = crate::FieldReader;
#[doc = "Field `GPIO_WAKEUP_STATUS` writer - Need add desc"]
pub type GPIO_WAKEUP_STATUS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
#[doc = "Field `GPIO_WAKEUP_STATUS_CLR` reader - Need add desc"]
pub type GPIO_WAKEUP_STATUS_CLR_R = crate::BitReader;
#[doc = "Field `GPIO_WAKEUP_STATUS_CLR` writer - Need add desc"]
pub type GPIO_WAKEUP_STATUS_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GPIO_PIN_CLK_GATE` reader - Need add desc"]
pub type GPIO_PIN_CLK_GATE_R = crate::BitReader;
#[doc = "Field `GPIO_PIN_CLK_GATE` writer - Need add desc"]
pub type GPIO_PIN_CLK_GATE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GPIO_PIN5_INT_TYPE` reader - Need add desc"]
pub type GPIO_PIN5_INT_TYPE_R = crate::FieldReader;
#[doc = "Field `GPIO_PIN5_INT_TYPE` writer - Need add desc"]
pub type GPIO_PIN5_INT_TYPE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `GPIO_PIN4_INT_TYPE` reader - Need add desc"]
pub type GPIO_PIN4_INT_TYPE_R = crate::FieldReader;
#[doc = "Field `GPIO_PIN4_INT_TYPE` writer - Need add desc"]
pub type GPIO_PIN4_INT_TYPE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `GPIO_PIN3_INT_TYPE` reader - Need add desc"]
pub type GPIO_PIN3_INT_TYPE_R = crate::FieldReader;
#[doc = "Field `GPIO_PIN3_INT_TYPE` writer - Need add desc"]
pub type GPIO_PIN3_INT_TYPE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `GPIO_PIN2_INT_TYPE` reader - Need add desc"]
pub type GPIO_PIN2_INT_TYPE_R = crate::FieldReader;
#[doc = "Field `GPIO_PIN2_INT_TYPE` writer - Need add desc"]
pub type GPIO_PIN2_INT_TYPE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `GPIO_PIN1_INT_TYPE` reader - Need add desc"]
pub type GPIO_PIN1_INT_TYPE_R = crate::FieldReader;
#[doc = "Field `GPIO_PIN1_INT_TYPE` writer - Need add desc"]
pub type GPIO_PIN1_INT_TYPE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `GPIO_PIN0_INT_TYPE` reader - Need add desc"]
pub type GPIO_PIN0_INT_TYPE_R = crate::FieldReader;
#[doc = "Field `GPIO_PIN0_INT_TYPE` writer - Need add desc"]
pub type GPIO_PIN0_INT_TYPE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `GPIO_PIN5_WAKEUP_ENABLE` reader - Need add desc"]
pub type GPIO_PIN5_WAKEUP_ENABLE_R = crate::BitReader;
#[doc = "Field `GPIO_PIN5_WAKEUP_ENABLE` writer - Need add desc"]
pub type GPIO_PIN5_WAKEUP_ENABLE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GPIO_PIN4_WAKEUP_ENABLE` reader - Need add desc"]
pub type GPIO_PIN4_WAKEUP_ENABLE_R = crate::BitReader;
#[doc = "Field `GPIO_PIN4_WAKEUP_ENABLE` writer - Need add desc"]
pub type GPIO_PIN4_WAKEUP_ENABLE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GPIO_PIN3_WAKEUP_ENABLE` reader - Need add desc"]
pub type GPIO_PIN3_WAKEUP_ENABLE_R = crate::BitReader;
#[doc = "Field `GPIO_PIN3_WAKEUP_ENABLE` writer - Need add desc"]
pub type GPIO_PIN3_WAKEUP_ENABLE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GPIO_PIN2_WAKEUP_ENABLE` reader - Need add desc"]
pub type GPIO_PIN2_WAKEUP_ENABLE_R = crate::BitReader;
#[doc = "Field `GPIO_PIN2_WAKEUP_ENABLE` writer - Need add desc"]
pub type GPIO_PIN2_WAKEUP_ENABLE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GPIO_PIN1_WAKEUP_ENABLE` reader - Need add desc"]
pub type GPIO_PIN1_WAKEUP_ENABLE_R = crate::BitReader;
#[doc = "Field `GPIO_PIN1_WAKEUP_ENABLE` writer - Need add desc"]
pub type GPIO_PIN1_WAKEUP_ENABLE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GPIO_PIN0_WAKEUP_ENABLE` reader - Need add desc"]
pub type GPIO_PIN0_WAKEUP_ENABLE_R = crate::BitReader;
#[doc = "Field `GPIO_PIN0_WAKEUP_ENABLE` writer - Need add desc"]
pub type GPIO_PIN0_WAKEUP_ENABLE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
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
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CNTL_GPIO_WAKEUP")
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
impl core::fmt::Debug for crate::generic::Reg<CNTL_GPIO_WAKEUP_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:5 - Need add desc"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_wakeup_status(&mut self) -> GPIO_WAKEUP_STATUS_W<CNTL_GPIO_WAKEUP_SPEC, 0> {
        GPIO_WAKEUP_STATUS_W::new(self)
    }
    #[doc = "Bit 6 - Need add desc"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_wakeup_status_clr(&mut self) -> GPIO_WAKEUP_STATUS_CLR_W<CNTL_GPIO_WAKEUP_SPEC, 6> {
        GPIO_WAKEUP_STATUS_CLR_W::new(self)
    }
    #[doc = "Bit 7 - Need add desc"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pin_clk_gate(&mut self) -> GPIO_PIN_CLK_GATE_W<CNTL_GPIO_WAKEUP_SPEC, 7> {
        GPIO_PIN_CLK_GATE_W::new(self)
    }
    #[doc = "Bits 8:10 - Need add desc"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pin5_int_type(&mut self) -> GPIO_PIN5_INT_TYPE_W<CNTL_GPIO_WAKEUP_SPEC, 8> {
        GPIO_PIN5_INT_TYPE_W::new(self)
    }
    #[doc = "Bits 11:13 - Need add desc"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pin4_int_type(&mut self) -> GPIO_PIN4_INT_TYPE_W<CNTL_GPIO_WAKEUP_SPEC, 11> {
        GPIO_PIN4_INT_TYPE_W::new(self)
    }
    #[doc = "Bits 14:16 - Need add desc"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pin3_int_type(&mut self) -> GPIO_PIN3_INT_TYPE_W<CNTL_GPIO_WAKEUP_SPEC, 14> {
        GPIO_PIN3_INT_TYPE_W::new(self)
    }
    #[doc = "Bits 17:19 - Need add desc"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pin2_int_type(&mut self) -> GPIO_PIN2_INT_TYPE_W<CNTL_GPIO_WAKEUP_SPEC, 17> {
        GPIO_PIN2_INT_TYPE_W::new(self)
    }
    #[doc = "Bits 20:22 - Need add desc"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pin1_int_type(&mut self) -> GPIO_PIN1_INT_TYPE_W<CNTL_GPIO_WAKEUP_SPEC, 20> {
        GPIO_PIN1_INT_TYPE_W::new(self)
    }
    #[doc = "Bits 23:25 - Need add desc"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pin0_int_type(&mut self) -> GPIO_PIN0_INT_TYPE_W<CNTL_GPIO_WAKEUP_SPEC, 23> {
        GPIO_PIN0_INT_TYPE_W::new(self)
    }
    #[doc = "Bit 26 - Need add desc"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pin5_wakeup_enable(
        &mut self,
    ) -> GPIO_PIN5_WAKEUP_ENABLE_W<CNTL_GPIO_WAKEUP_SPEC, 26> {
        GPIO_PIN5_WAKEUP_ENABLE_W::new(self)
    }
    #[doc = "Bit 27 - Need add desc"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pin4_wakeup_enable(
        &mut self,
    ) -> GPIO_PIN4_WAKEUP_ENABLE_W<CNTL_GPIO_WAKEUP_SPEC, 27> {
        GPIO_PIN4_WAKEUP_ENABLE_W::new(self)
    }
    #[doc = "Bit 28 - Need add desc"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pin3_wakeup_enable(
        &mut self,
    ) -> GPIO_PIN3_WAKEUP_ENABLE_W<CNTL_GPIO_WAKEUP_SPEC, 28> {
        GPIO_PIN3_WAKEUP_ENABLE_W::new(self)
    }
    #[doc = "Bit 29 - Need add desc"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pin2_wakeup_enable(
        &mut self,
    ) -> GPIO_PIN2_WAKEUP_ENABLE_W<CNTL_GPIO_WAKEUP_SPEC, 29> {
        GPIO_PIN2_WAKEUP_ENABLE_W::new(self)
    }
    #[doc = "Bit 30 - Need add desc"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pin1_wakeup_enable(
        &mut self,
    ) -> GPIO_PIN1_WAKEUP_ENABLE_W<CNTL_GPIO_WAKEUP_SPEC, 30> {
        GPIO_PIN1_WAKEUP_ENABLE_W::new(self)
    }
    #[doc = "Bit 31 - Need add desc"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pin0_wakeup_enable(
        &mut self,
    ) -> GPIO_PIN0_WAKEUP_ENABLE_W<CNTL_GPIO_WAKEUP_SPEC, 31> {
        GPIO_PIN0_WAKEUP_ENABLE_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cntl_gpio_wakeup::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cntl_gpio_wakeup::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CNTL_GPIO_WAKEUP_SPEC;
impl crate::RegisterSpec for CNTL_GPIO_WAKEUP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cntl_gpio_wakeup::R`](R) reader structure"]
impl crate::Readable for CNTL_GPIO_WAKEUP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cntl_gpio_wakeup::W`](W) writer structure"]
impl crate::Writable for CNTL_GPIO_WAKEUP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CNTL_GPIO_WAKEUP to value 0"]
impl crate::Resettable for CNTL_GPIO_WAKEUP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
