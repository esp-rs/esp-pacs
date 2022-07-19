#[doc = "Register `RTC_CNTL_DBG_MAP` reader"]
pub struct R(crate::R<RTC_CNTL_DBG_MAP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_CNTL_DBG_MAP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_CNTL_DBG_MAP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_CNTL_DBG_MAP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_CNTL_DBG_MAP` writer"]
pub struct W(crate::W<RTC_CNTL_DBG_MAP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_CNTL_DBG_MAP_SPEC>;
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
impl From<crate::W<RTC_CNTL_DBG_MAP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_CNTL_DBG_MAP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTC_GPIO_PIN5_MUX_SEL` reader - Need add desc"]
pub type RTC_GPIO_PIN5_MUX_SEL_R = crate::BitReader<bool>;
#[doc = "Field `RTC_GPIO_PIN5_MUX_SEL` writer - Need add desc"]
pub type RTC_GPIO_PIN5_MUX_SEL_W<'a> = crate::BitWriter<'a, u32, RTC_CNTL_DBG_MAP_SPEC, bool, 2>;
#[doc = "Field `RTC_GPIO_PIN4_MUX_SEL` reader - Need add desc"]
pub type RTC_GPIO_PIN4_MUX_SEL_R = crate::BitReader<bool>;
#[doc = "Field `RTC_GPIO_PIN4_MUX_SEL` writer - Need add desc"]
pub type RTC_GPIO_PIN4_MUX_SEL_W<'a> = crate::BitWriter<'a, u32, RTC_CNTL_DBG_MAP_SPEC, bool, 3>;
#[doc = "Field `RTC_GPIO_PIN3_MUX_SEL` reader - Need add desc"]
pub type RTC_GPIO_PIN3_MUX_SEL_R = crate::BitReader<bool>;
#[doc = "Field `RTC_GPIO_PIN3_MUX_SEL` writer - Need add desc"]
pub type RTC_GPIO_PIN3_MUX_SEL_W<'a> = crate::BitWriter<'a, u32, RTC_CNTL_DBG_MAP_SPEC, bool, 4>;
#[doc = "Field `RTC_GPIO_PIN2_MUX_SEL` reader - Need add desc"]
pub type RTC_GPIO_PIN2_MUX_SEL_R = crate::BitReader<bool>;
#[doc = "Field `RTC_GPIO_PIN2_MUX_SEL` writer - Need add desc"]
pub type RTC_GPIO_PIN2_MUX_SEL_W<'a> = crate::BitWriter<'a, u32, RTC_CNTL_DBG_MAP_SPEC, bool, 5>;
#[doc = "Field `RTC_GPIO_PIN1_MUX_SEL` reader - Need add desc"]
pub type RTC_GPIO_PIN1_MUX_SEL_R = crate::BitReader<bool>;
#[doc = "Field `RTC_GPIO_PIN1_MUX_SEL` writer - Need add desc"]
pub type RTC_GPIO_PIN1_MUX_SEL_W<'a> = crate::BitWriter<'a, u32, RTC_CNTL_DBG_MAP_SPEC, bool, 6>;
#[doc = "Field `RTC_GPIO_PIN0_MUX_SEL` reader - Need add desc"]
pub type RTC_GPIO_PIN0_MUX_SEL_R = crate::BitReader<bool>;
#[doc = "Field `RTC_GPIO_PIN0_MUX_SEL` writer - Need add desc"]
pub type RTC_GPIO_PIN0_MUX_SEL_W<'a> = crate::BitWriter<'a, u32, RTC_CNTL_DBG_MAP_SPEC, bool, 7>;
#[doc = "Field `RTC_GPIO_PIN5_FUN_SEL` reader - Need add desc"]
pub type RTC_GPIO_PIN5_FUN_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RTC_GPIO_PIN5_FUN_SEL` writer - Need add desc"]
pub type RTC_GPIO_PIN5_FUN_SEL_W<'a> =
    crate::FieldWriter<'a, u32, RTC_CNTL_DBG_MAP_SPEC, u8, u8, 4, 8>;
#[doc = "Field `RTC_GPIO_PIN4_FUN_SEL` reader - Need add desc"]
pub type RTC_GPIO_PIN4_FUN_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RTC_GPIO_PIN4_FUN_SEL` writer - Need add desc"]
pub type RTC_GPIO_PIN4_FUN_SEL_W<'a> =
    crate::FieldWriter<'a, u32, RTC_CNTL_DBG_MAP_SPEC, u8, u8, 4, 12>;
#[doc = "Field `RTC_GPIO_PIN3_FUN_SEL` reader - Need add desc"]
pub type RTC_GPIO_PIN3_FUN_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RTC_GPIO_PIN3_FUN_SEL` writer - Need add desc"]
pub type RTC_GPIO_PIN3_FUN_SEL_W<'a> =
    crate::FieldWriter<'a, u32, RTC_CNTL_DBG_MAP_SPEC, u8, u8, 4, 16>;
#[doc = "Field `RTC_GPIO_PIN2_FUN_SEL` reader - Need add desc"]
pub type RTC_GPIO_PIN2_FUN_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RTC_GPIO_PIN2_FUN_SEL` writer - Need add desc"]
pub type RTC_GPIO_PIN2_FUN_SEL_W<'a> =
    crate::FieldWriter<'a, u32, RTC_CNTL_DBG_MAP_SPEC, u8, u8, 4, 20>;
#[doc = "Field `RTC_GPIO_PIN1_FUN_SEL` reader - Need add desc"]
pub type RTC_GPIO_PIN1_FUN_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RTC_GPIO_PIN1_FUN_SEL` writer - Need add desc"]
pub type RTC_GPIO_PIN1_FUN_SEL_W<'a> =
    crate::FieldWriter<'a, u32, RTC_CNTL_DBG_MAP_SPEC, u8, u8, 4, 24>;
#[doc = "Field `RTC_GPIO_PIN0_FUN_SEL` reader - Need add desc"]
pub type RTC_GPIO_PIN0_FUN_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RTC_GPIO_PIN0_FUN_SEL` writer - Need add desc"]
pub type RTC_GPIO_PIN0_FUN_SEL_W<'a> =
    crate::FieldWriter<'a, u32, RTC_CNTL_DBG_MAP_SPEC, u8, u8, 4, 28>;
impl R {
    #[doc = "Bit 2 - Need add desc"]
    #[inline(always)]
    pub fn rtc_gpio_pin5_mux_sel(&self) -> RTC_GPIO_PIN5_MUX_SEL_R {
        RTC_GPIO_PIN5_MUX_SEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Need add desc"]
    #[inline(always)]
    pub fn rtc_gpio_pin4_mux_sel(&self) -> RTC_GPIO_PIN4_MUX_SEL_R {
        RTC_GPIO_PIN4_MUX_SEL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Need add desc"]
    #[inline(always)]
    pub fn rtc_gpio_pin3_mux_sel(&self) -> RTC_GPIO_PIN3_MUX_SEL_R {
        RTC_GPIO_PIN3_MUX_SEL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Need add desc"]
    #[inline(always)]
    pub fn rtc_gpio_pin2_mux_sel(&self) -> RTC_GPIO_PIN2_MUX_SEL_R {
        RTC_GPIO_PIN2_MUX_SEL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Need add desc"]
    #[inline(always)]
    pub fn rtc_gpio_pin1_mux_sel(&self) -> RTC_GPIO_PIN1_MUX_SEL_R {
        RTC_GPIO_PIN1_MUX_SEL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Need add desc"]
    #[inline(always)]
    pub fn rtc_gpio_pin0_mux_sel(&self) -> RTC_GPIO_PIN0_MUX_SEL_R {
        RTC_GPIO_PIN0_MUX_SEL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Need add desc"]
    #[inline(always)]
    pub fn rtc_gpio_pin5_fun_sel(&self) -> RTC_GPIO_PIN5_FUN_SEL_R {
        RTC_GPIO_PIN5_FUN_SEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Need add desc"]
    #[inline(always)]
    pub fn rtc_gpio_pin4_fun_sel(&self) -> RTC_GPIO_PIN4_FUN_SEL_R {
        RTC_GPIO_PIN4_FUN_SEL_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Need add desc"]
    #[inline(always)]
    pub fn rtc_gpio_pin3_fun_sel(&self) -> RTC_GPIO_PIN3_FUN_SEL_R {
        RTC_GPIO_PIN3_FUN_SEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Need add desc"]
    #[inline(always)]
    pub fn rtc_gpio_pin2_fun_sel(&self) -> RTC_GPIO_PIN2_FUN_SEL_R {
        RTC_GPIO_PIN2_FUN_SEL_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Need add desc"]
    #[inline(always)]
    pub fn rtc_gpio_pin1_fun_sel(&self) -> RTC_GPIO_PIN1_FUN_SEL_R {
        RTC_GPIO_PIN1_FUN_SEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Need add desc"]
    #[inline(always)]
    pub fn rtc_gpio_pin0_fun_sel(&self) -> RTC_GPIO_PIN0_FUN_SEL_R {
        RTC_GPIO_PIN0_FUN_SEL_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 2 - Need add desc"]
    #[inline(always)]
    pub fn rtc_gpio_pin5_mux_sel(&mut self) -> RTC_GPIO_PIN5_MUX_SEL_W {
        RTC_GPIO_PIN5_MUX_SEL_W::new(self)
    }
    #[doc = "Bit 3 - Need add desc"]
    #[inline(always)]
    pub fn rtc_gpio_pin4_mux_sel(&mut self) -> RTC_GPIO_PIN4_MUX_SEL_W {
        RTC_GPIO_PIN4_MUX_SEL_W::new(self)
    }
    #[doc = "Bit 4 - Need add desc"]
    #[inline(always)]
    pub fn rtc_gpio_pin3_mux_sel(&mut self) -> RTC_GPIO_PIN3_MUX_SEL_W {
        RTC_GPIO_PIN3_MUX_SEL_W::new(self)
    }
    #[doc = "Bit 5 - Need add desc"]
    #[inline(always)]
    pub fn rtc_gpio_pin2_mux_sel(&mut self) -> RTC_GPIO_PIN2_MUX_SEL_W {
        RTC_GPIO_PIN2_MUX_SEL_W::new(self)
    }
    #[doc = "Bit 6 - Need add desc"]
    #[inline(always)]
    pub fn rtc_gpio_pin1_mux_sel(&mut self) -> RTC_GPIO_PIN1_MUX_SEL_W {
        RTC_GPIO_PIN1_MUX_SEL_W::new(self)
    }
    #[doc = "Bit 7 - Need add desc"]
    #[inline(always)]
    pub fn rtc_gpio_pin0_mux_sel(&mut self) -> RTC_GPIO_PIN0_MUX_SEL_W {
        RTC_GPIO_PIN0_MUX_SEL_W::new(self)
    }
    #[doc = "Bits 8:11 - Need add desc"]
    #[inline(always)]
    pub fn rtc_gpio_pin5_fun_sel(&mut self) -> RTC_GPIO_PIN5_FUN_SEL_W {
        RTC_GPIO_PIN5_FUN_SEL_W::new(self)
    }
    #[doc = "Bits 12:15 - Need add desc"]
    #[inline(always)]
    pub fn rtc_gpio_pin4_fun_sel(&mut self) -> RTC_GPIO_PIN4_FUN_SEL_W {
        RTC_GPIO_PIN4_FUN_SEL_W::new(self)
    }
    #[doc = "Bits 16:19 - Need add desc"]
    #[inline(always)]
    pub fn rtc_gpio_pin3_fun_sel(&mut self) -> RTC_GPIO_PIN3_FUN_SEL_W {
        RTC_GPIO_PIN3_FUN_SEL_W::new(self)
    }
    #[doc = "Bits 20:23 - Need add desc"]
    #[inline(always)]
    pub fn rtc_gpio_pin2_fun_sel(&mut self) -> RTC_GPIO_PIN2_FUN_SEL_W {
        RTC_GPIO_PIN2_FUN_SEL_W::new(self)
    }
    #[doc = "Bits 24:27 - Need add desc"]
    #[inline(always)]
    pub fn rtc_gpio_pin1_fun_sel(&mut self) -> RTC_GPIO_PIN1_FUN_SEL_W {
        RTC_GPIO_PIN1_FUN_SEL_W::new(self)
    }
    #[doc = "Bits 28:31 - Need add desc"]
    #[inline(always)]
    pub fn rtc_gpio_pin0_fun_sel(&mut self) -> RTC_GPIO_PIN0_FUN_SEL_W {
        RTC_GPIO_PIN0_FUN_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_cntl_dbg_map](index.html) module"]
pub struct RTC_CNTL_DBG_MAP_SPEC;
impl crate::RegisterSpec for RTC_CNTL_DBG_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_cntl_dbg_map::R](R) reader structure"]
impl crate::Readable for RTC_CNTL_DBG_MAP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_dbg_map::W](W) writer structure"]
impl crate::Writable for RTC_CNTL_DBG_MAP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_CNTL_DBG_MAP to value 0"]
impl crate::Resettable for RTC_CNTL_DBG_MAP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
