#[doc = "Register `DBG_MAP` reader"]
pub struct R(crate::R<DBG_MAP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DBG_MAP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DBG_MAP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DBG_MAP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DBG_MAP` writer"]
pub struct W(crate::W<DBG_MAP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DBG_MAP_SPEC>;
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
impl From<crate::W<DBG_MAP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DBG_MAP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPIO_PIN5_MUX_SEL` reader - use for debug"]
pub type GPIO_PIN5_MUX_SEL_R = crate::BitReader;
#[doc = "Field `GPIO_PIN5_MUX_SEL` writer - use for debug"]
pub type GPIO_PIN5_MUX_SEL_W<'a, const O: u8> = crate::BitWriter<'a, DBG_MAP_SPEC, O>;
#[doc = "Field `GPIO_PIN4_MUX_SEL` reader - use for debug"]
pub type GPIO_PIN4_MUX_SEL_R = crate::BitReader;
#[doc = "Field `GPIO_PIN4_MUX_SEL` writer - use for debug"]
pub type GPIO_PIN4_MUX_SEL_W<'a, const O: u8> = crate::BitWriter<'a, DBG_MAP_SPEC, O>;
#[doc = "Field `GPIO_PIN3_MUX_SEL` reader - use for debug"]
pub type GPIO_PIN3_MUX_SEL_R = crate::BitReader;
#[doc = "Field `GPIO_PIN3_MUX_SEL` writer - use for debug"]
pub type GPIO_PIN3_MUX_SEL_W<'a, const O: u8> = crate::BitWriter<'a, DBG_MAP_SPEC, O>;
#[doc = "Field `GPIO_PIN2_MUX_SEL` reader - use for debug"]
pub type GPIO_PIN2_MUX_SEL_R = crate::BitReader;
#[doc = "Field `GPIO_PIN2_MUX_SEL` writer - use for debug"]
pub type GPIO_PIN2_MUX_SEL_W<'a, const O: u8> = crate::BitWriter<'a, DBG_MAP_SPEC, O>;
#[doc = "Field `GPIO_PIN1_MUX_SEL` reader - use for debug"]
pub type GPIO_PIN1_MUX_SEL_R = crate::BitReader;
#[doc = "Field `GPIO_PIN1_MUX_SEL` writer - use for debug"]
pub type GPIO_PIN1_MUX_SEL_W<'a, const O: u8> = crate::BitWriter<'a, DBG_MAP_SPEC, O>;
#[doc = "Field `GPIO_PIN0_MUX_SEL` reader - use for debug"]
pub type GPIO_PIN0_MUX_SEL_R = crate::BitReader;
#[doc = "Field `GPIO_PIN0_MUX_SEL` writer - use for debug"]
pub type GPIO_PIN0_MUX_SEL_W<'a, const O: u8> = crate::BitWriter<'a, DBG_MAP_SPEC, O>;
#[doc = "Field `GPIO_PIN5_FUN_SEL` reader - use for debug"]
pub type GPIO_PIN5_FUN_SEL_R = crate::FieldReader;
#[doc = "Field `GPIO_PIN5_FUN_SEL` writer - use for debug"]
pub type GPIO_PIN5_FUN_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, DBG_MAP_SPEC, 4, O>;
#[doc = "Field `GPIO_PIN4_FUN_SEL` reader - use for debug"]
pub type GPIO_PIN4_FUN_SEL_R = crate::FieldReader;
#[doc = "Field `GPIO_PIN4_FUN_SEL` writer - use for debug"]
pub type GPIO_PIN4_FUN_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, DBG_MAP_SPEC, 4, O>;
#[doc = "Field `GPIO_PIN3_FUN_SEL` reader - use for debug"]
pub type GPIO_PIN3_FUN_SEL_R = crate::FieldReader;
#[doc = "Field `GPIO_PIN3_FUN_SEL` writer - use for debug"]
pub type GPIO_PIN3_FUN_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, DBG_MAP_SPEC, 4, O>;
#[doc = "Field `GPIO_PIN2_FUN_SEL` reader - use for debug"]
pub type GPIO_PIN2_FUN_SEL_R = crate::FieldReader;
#[doc = "Field `GPIO_PIN2_FUN_SEL` writer - use for debug"]
pub type GPIO_PIN2_FUN_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, DBG_MAP_SPEC, 4, O>;
#[doc = "Field `GPIO_PIN1_FUN_SEL` reader - use for debug"]
pub type GPIO_PIN1_FUN_SEL_R = crate::FieldReader;
#[doc = "Field `GPIO_PIN1_FUN_SEL` writer - use for debug"]
pub type GPIO_PIN1_FUN_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, DBG_MAP_SPEC, 4, O>;
#[doc = "Field `GPIO_PIN0_FUN_SEL` reader - use for debug"]
pub type GPIO_PIN0_FUN_SEL_R = crate::FieldReader;
#[doc = "Field `GPIO_PIN0_FUN_SEL` writer - use for debug"]
pub type GPIO_PIN0_FUN_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, DBG_MAP_SPEC, 4, O>;
impl R {
    #[doc = "Bit 2 - use for debug"]
    #[inline(always)]
    pub fn gpio_pin5_mux_sel(&self) -> GPIO_PIN5_MUX_SEL_R {
        GPIO_PIN5_MUX_SEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - use for debug"]
    #[inline(always)]
    pub fn gpio_pin4_mux_sel(&self) -> GPIO_PIN4_MUX_SEL_R {
        GPIO_PIN4_MUX_SEL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - use for debug"]
    #[inline(always)]
    pub fn gpio_pin3_mux_sel(&self) -> GPIO_PIN3_MUX_SEL_R {
        GPIO_PIN3_MUX_SEL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - use for debug"]
    #[inline(always)]
    pub fn gpio_pin2_mux_sel(&self) -> GPIO_PIN2_MUX_SEL_R {
        GPIO_PIN2_MUX_SEL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - use for debug"]
    #[inline(always)]
    pub fn gpio_pin1_mux_sel(&self) -> GPIO_PIN1_MUX_SEL_R {
        GPIO_PIN1_MUX_SEL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - use for debug"]
    #[inline(always)]
    pub fn gpio_pin0_mux_sel(&self) -> GPIO_PIN0_MUX_SEL_R {
        GPIO_PIN0_MUX_SEL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - use for debug"]
    #[inline(always)]
    pub fn gpio_pin5_fun_sel(&self) -> GPIO_PIN5_FUN_SEL_R {
        GPIO_PIN5_FUN_SEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - use for debug"]
    #[inline(always)]
    pub fn gpio_pin4_fun_sel(&self) -> GPIO_PIN4_FUN_SEL_R {
        GPIO_PIN4_FUN_SEL_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - use for debug"]
    #[inline(always)]
    pub fn gpio_pin3_fun_sel(&self) -> GPIO_PIN3_FUN_SEL_R {
        GPIO_PIN3_FUN_SEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - use for debug"]
    #[inline(always)]
    pub fn gpio_pin2_fun_sel(&self) -> GPIO_PIN2_FUN_SEL_R {
        GPIO_PIN2_FUN_SEL_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - use for debug"]
    #[inline(always)]
    pub fn gpio_pin1_fun_sel(&self) -> GPIO_PIN1_FUN_SEL_R {
        GPIO_PIN1_FUN_SEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - use for debug"]
    #[inline(always)]
    pub fn gpio_pin0_fun_sel(&self) -> GPIO_PIN0_FUN_SEL_R {
        GPIO_PIN0_FUN_SEL_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DBG_MAP")
            .field(
                "gpio_pin5_mux_sel",
                &format_args!("{}", self.gpio_pin5_mux_sel().bit()),
            )
            .field(
                "gpio_pin4_mux_sel",
                &format_args!("{}", self.gpio_pin4_mux_sel().bit()),
            )
            .field(
                "gpio_pin3_mux_sel",
                &format_args!("{}", self.gpio_pin3_mux_sel().bit()),
            )
            .field(
                "gpio_pin2_mux_sel",
                &format_args!("{}", self.gpio_pin2_mux_sel().bit()),
            )
            .field(
                "gpio_pin1_mux_sel",
                &format_args!("{}", self.gpio_pin1_mux_sel().bit()),
            )
            .field(
                "gpio_pin0_mux_sel",
                &format_args!("{}", self.gpio_pin0_mux_sel().bit()),
            )
            .field(
                "gpio_pin5_fun_sel",
                &format_args!("{}", self.gpio_pin5_fun_sel().bits()),
            )
            .field(
                "gpio_pin4_fun_sel",
                &format_args!("{}", self.gpio_pin4_fun_sel().bits()),
            )
            .field(
                "gpio_pin3_fun_sel",
                &format_args!("{}", self.gpio_pin3_fun_sel().bits()),
            )
            .field(
                "gpio_pin2_fun_sel",
                &format_args!("{}", self.gpio_pin2_fun_sel().bits()),
            )
            .field(
                "gpio_pin1_fun_sel",
                &format_args!("{}", self.gpio_pin1_fun_sel().bits()),
            )
            .field(
                "gpio_pin0_fun_sel",
                &format_args!("{}", self.gpio_pin0_fun_sel().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DBG_MAP_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 2 - use for debug"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pin5_mux_sel(&mut self) -> GPIO_PIN5_MUX_SEL_W<2> {
        GPIO_PIN5_MUX_SEL_W::new(self)
    }
    #[doc = "Bit 3 - use for debug"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pin4_mux_sel(&mut self) -> GPIO_PIN4_MUX_SEL_W<3> {
        GPIO_PIN4_MUX_SEL_W::new(self)
    }
    #[doc = "Bit 4 - use for debug"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pin3_mux_sel(&mut self) -> GPIO_PIN3_MUX_SEL_W<4> {
        GPIO_PIN3_MUX_SEL_W::new(self)
    }
    #[doc = "Bit 5 - use for debug"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pin2_mux_sel(&mut self) -> GPIO_PIN2_MUX_SEL_W<5> {
        GPIO_PIN2_MUX_SEL_W::new(self)
    }
    #[doc = "Bit 6 - use for debug"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pin1_mux_sel(&mut self) -> GPIO_PIN1_MUX_SEL_W<6> {
        GPIO_PIN1_MUX_SEL_W::new(self)
    }
    #[doc = "Bit 7 - use for debug"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pin0_mux_sel(&mut self) -> GPIO_PIN0_MUX_SEL_W<7> {
        GPIO_PIN0_MUX_SEL_W::new(self)
    }
    #[doc = "Bits 8:11 - use for debug"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pin5_fun_sel(&mut self) -> GPIO_PIN5_FUN_SEL_W<8> {
        GPIO_PIN5_FUN_SEL_W::new(self)
    }
    #[doc = "Bits 12:15 - use for debug"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pin4_fun_sel(&mut self) -> GPIO_PIN4_FUN_SEL_W<12> {
        GPIO_PIN4_FUN_SEL_W::new(self)
    }
    #[doc = "Bits 16:19 - use for debug"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pin3_fun_sel(&mut self) -> GPIO_PIN3_FUN_SEL_W<16> {
        GPIO_PIN3_FUN_SEL_W::new(self)
    }
    #[doc = "Bits 20:23 - use for debug"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pin2_fun_sel(&mut self) -> GPIO_PIN2_FUN_SEL_W<20> {
        GPIO_PIN2_FUN_SEL_W::new(self)
    }
    #[doc = "Bits 24:27 - use for debug"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pin1_fun_sel(&mut self) -> GPIO_PIN1_FUN_SEL_W<24> {
        GPIO_PIN1_FUN_SEL_W::new(self)
    }
    #[doc = "Bits 28:31 - use for debug"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pin0_fun_sel(&mut self) -> GPIO_PIN0_FUN_SEL_W<28> {
        GPIO_PIN0_FUN_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rtc configure register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dbg_map](index.html) module"]
pub struct DBG_MAP_SPEC;
impl crate::RegisterSpec for DBG_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dbg_map::R](R) reader structure"]
impl crate::Readable for DBG_MAP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dbg_map::W](W) writer structure"]
impl crate::Writable for DBG_MAP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DBG_MAP to value 0"]
impl crate::Resettable for DBG_MAP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
