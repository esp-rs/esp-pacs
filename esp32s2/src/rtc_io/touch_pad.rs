#[doc = "Register `TOUCH_PAD%s` reader"]
pub struct R(crate::R<TOUCH_PAD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TOUCH_PAD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TOUCH_PAD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TOUCH_PAD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TOUCH_PAD%s` writer"]
pub struct W(crate::W<TOUCH_PAD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TOUCH_PAD_SPEC>;
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
impl From<crate::W<TOUCH_PAD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TOUCH_PAD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TOUCH_PAD0_FUN_IE` reader - Input enable in normal execution."]
pub type TOUCH_PAD0_FUN_IE_R = crate::BitReader;
#[doc = "Field `TOUCH_PAD0_FUN_IE` writer - Input enable in normal execution."]
pub type TOUCH_PAD0_FUN_IE_W<'a, const O: u8> = crate::BitWriter<'a, TOUCH_PAD_SPEC, O>;
#[doc = "Field `TOUCH_PAD0_SLP_OE` reader - Output enable in sleep mode."]
pub type TOUCH_PAD0_SLP_OE_R = crate::BitReader;
#[doc = "Field `TOUCH_PAD0_SLP_OE` writer - Output enable in sleep mode."]
pub type TOUCH_PAD0_SLP_OE_W<'a, const O: u8> = crate::BitWriter<'a, TOUCH_PAD_SPEC, O>;
#[doc = "Field `TOUCH_PAD0_SLP_IE` reader - Input enable in sleep mode."]
pub type TOUCH_PAD0_SLP_IE_R = crate::BitReader;
#[doc = "Field `TOUCH_PAD0_SLP_IE` writer - Input enable in sleep mode."]
pub type TOUCH_PAD0_SLP_IE_W<'a, const O: u8> = crate::BitWriter<'a, TOUCH_PAD_SPEC, O>;
#[doc = "Field `TOUCH_PAD0_SLP_SEL` reader - 0: no sleep mode. 1: enable sleep mode."]
pub type TOUCH_PAD0_SLP_SEL_R = crate::BitReader;
#[doc = "Field `TOUCH_PAD0_SLP_SEL` writer - 0: no sleep mode. 1: enable sleep mode."]
pub type TOUCH_PAD0_SLP_SEL_W<'a, const O: u8> = crate::BitWriter<'a, TOUCH_PAD_SPEC, O>;
#[doc = "Field `TOUCH_PAD0_FUN_SEL` reader - Function selection."]
pub type TOUCH_PAD0_FUN_SEL_R = crate::FieldReader;
#[doc = "Field `TOUCH_PAD0_FUN_SEL` writer - Function selection."]
pub type TOUCH_PAD0_FUN_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, TOUCH_PAD_SPEC, 2, O>;
#[doc = "Field `TOUCH_PAD0_MUX_SEL` reader - Connect the RTC pad input to digital pad input. 0 is available."]
pub type TOUCH_PAD0_MUX_SEL_R = crate::BitReader;
#[doc = "Field `TOUCH_PAD0_MUX_SEL` writer - Connect the RTC pad input to digital pad input. 0 is available."]
pub type TOUCH_PAD0_MUX_SEL_W<'a, const O: u8> = crate::BitWriter<'a, TOUCH_PAD_SPEC, O>;
#[doc = "Field `TOUCH_PAD0_XPD` reader - Touch sensor power on."]
pub type TOUCH_PAD0_XPD_R = crate::BitReader;
#[doc = "Field `TOUCH_PAD0_XPD` writer - Touch sensor power on."]
pub type TOUCH_PAD0_XPD_W<'a, const O: u8> = crate::BitWriter<'a, TOUCH_PAD_SPEC, O>;
#[doc = "Field `TOUCH_PAD0_TIE_OPT` reader - The tie option of touch sensor. 0: tie low. 1: tie high."]
pub type TOUCH_PAD0_TIE_OPT_R = crate::BitReader;
#[doc = "Field `TOUCH_PAD0_TIE_OPT` writer - The tie option of touch sensor. 0: tie low. 1: tie high."]
pub type TOUCH_PAD0_TIE_OPT_W<'a, const O: u8> = crate::BitWriter<'a, TOUCH_PAD_SPEC, O>;
#[doc = "Field `TOUCH_PAD0_START` reader - Start touch sensor."]
pub type TOUCH_PAD0_START_R = crate::BitReader;
#[doc = "Field `TOUCH_PAD0_START` writer - Start touch sensor."]
pub type TOUCH_PAD0_START_W<'a, const O: u8> = crate::BitWriter<'a, TOUCH_PAD_SPEC, O>;
#[doc = "Field `TOUCH_PAD0_DAC` reader - Touch sensor slope control. 3-bit for each touch pad, defaults to 0x4."]
pub type TOUCH_PAD0_DAC_R = crate::FieldReader;
#[doc = "Field `TOUCH_PAD0_DAC` writer - Touch sensor slope control. 3-bit for each touch pad, defaults to 0x4."]
pub type TOUCH_PAD0_DAC_W<'a, const O: u8> = crate::FieldWriter<'a, TOUCH_PAD_SPEC, 3, O>;
#[doc = "Field `TOUCH_PAD0_RUE` reader - Pull-down enable of the pad. 1: internal pull-down enabled. 0: internal pull-down disabled."]
pub type TOUCH_PAD0_RUE_R = crate::BitReader;
#[doc = "Field `TOUCH_PAD0_RUE` writer - Pull-down enable of the pad. 1: internal pull-down enabled. 0: internal pull-down disabled."]
pub type TOUCH_PAD0_RUE_W<'a, const O: u8> = crate::BitWriter<'a, TOUCH_PAD_SPEC, O>;
#[doc = "Field `TOUCH_PAD0_RDE` reader - Pull-up enable of the pad. 1: internal pull-up enabled. 0: internal pull-up disabled."]
pub type TOUCH_PAD0_RDE_R = crate::BitReader;
#[doc = "Field `TOUCH_PAD0_RDE` writer - Pull-up enable of the pad. 1: internal pull-up enabled. 0: internal pull-up disabled."]
pub type TOUCH_PAD0_RDE_W<'a, const O: u8> = crate::BitWriter<'a, TOUCH_PAD_SPEC, O>;
#[doc = "Field `TOUCH_PAD0_DRV` reader - Select the drive strength of the pad. 0: ~5 mA. 1: ~10 mA. 2: ~20 mA. 3: ~40 mA."]
pub type TOUCH_PAD0_DRV_R = crate::FieldReader;
#[doc = "Field `TOUCH_PAD0_DRV` writer - Select the drive strength of the pad. 0: ~5 mA. 1: ~10 mA. 2: ~20 mA. 3: ~40 mA."]
pub type TOUCH_PAD0_DRV_W<'a, const O: u8> = crate::FieldWriter<'a, TOUCH_PAD_SPEC, 2, O>;
impl R {
    #[doc = "Bit 13 - Input enable in normal execution."]
    #[inline(always)]
    pub fn touch_pad0_fun_ie(&self) -> TOUCH_PAD0_FUN_IE_R {
        TOUCH_PAD0_FUN_IE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Output enable in sleep mode."]
    #[inline(always)]
    pub fn touch_pad0_slp_oe(&self) -> TOUCH_PAD0_SLP_OE_R {
        TOUCH_PAD0_SLP_OE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Input enable in sleep mode."]
    #[inline(always)]
    pub fn touch_pad0_slp_ie(&self) -> TOUCH_PAD0_SLP_IE_R {
        TOUCH_PAD0_SLP_IE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 0: no sleep mode. 1: enable sleep mode."]
    #[inline(always)]
    pub fn touch_pad0_slp_sel(&self) -> TOUCH_PAD0_SLP_SEL_R {
        TOUCH_PAD0_SLP_SEL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - Function selection."]
    #[inline(always)]
    pub fn touch_pad0_fun_sel(&self) -> TOUCH_PAD0_FUN_SEL_R {
        TOUCH_PAD0_FUN_SEL_R::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bit 19 - Connect the RTC pad input to digital pad input. 0 is available."]
    #[inline(always)]
    pub fn touch_pad0_mux_sel(&self) -> TOUCH_PAD0_MUX_SEL_R {
        TOUCH_PAD0_MUX_SEL_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Touch sensor power on."]
    #[inline(always)]
    pub fn touch_pad0_xpd(&self) -> TOUCH_PAD0_XPD_R {
        TOUCH_PAD0_XPD_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - The tie option of touch sensor. 0: tie low. 1: tie high."]
    #[inline(always)]
    pub fn touch_pad0_tie_opt(&self) -> TOUCH_PAD0_TIE_OPT_R {
        TOUCH_PAD0_TIE_OPT_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Start touch sensor."]
    #[inline(always)]
    pub fn touch_pad0_start(&self) -> TOUCH_PAD0_START_R {
        TOUCH_PAD0_START_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 23:25 - Touch sensor slope control. 3-bit for each touch pad, defaults to 0x4."]
    #[inline(always)]
    pub fn touch_pad0_dac(&self) -> TOUCH_PAD0_DAC_R {
        TOUCH_PAD0_DAC_R::new(((self.bits >> 23) & 7) as u8)
    }
    #[doc = "Bit 27 - Pull-down enable of the pad. 1: internal pull-down enabled. 0: internal pull-down disabled."]
    #[inline(always)]
    pub fn touch_pad0_rue(&self) -> TOUCH_PAD0_RUE_R {
        TOUCH_PAD0_RUE_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Pull-up enable of the pad. 1: internal pull-up enabled. 0: internal pull-up disabled."]
    #[inline(always)]
    pub fn touch_pad0_rde(&self) -> TOUCH_PAD0_RDE_R {
        TOUCH_PAD0_RDE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bits 29:30 - Select the drive strength of the pad. 0: ~5 mA. 1: ~10 mA. 2: ~20 mA. 3: ~40 mA."]
    #[inline(always)]
    pub fn touch_pad0_drv(&self) -> TOUCH_PAD0_DRV_R {
        TOUCH_PAD0_DRV_R::new(((self.bits >> 29) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TOUCH_PAD")
            .field(
                "touch_pad0_fun_ie",
                &format_args!("{}", self.touch_pad0_fun_ie().bit()),
            )
            .field(
                "touch_pad0_slp_oe",
                &format_args!("{}", self.touch_pad0_slp_oe().bit()),
            )
            .field(
                "touch_pad0_slp_ie",
                &format_args!("{}", self.touch_pad0_slp_ie().bit()),
            )
            .field(
                "touch_pad0_slp_sel",
                &format_args!("{}", self.touch_pad0_slp_sel().bit()),
            )
            .field(
                "touch_pad0_fun_sel",
                &format_args!("{}", self.touch_pad0_fun_sel().bits()),
            )
            .field(
                "touch_pad0_mux_sel",
                &format_args!("{}", self.touch_pad0_mux_sel().bit()),
            )
            .field(
                "touch_pad0_xpd",
                &format_args!("{}", self.touch_pad0_xpd().bit()),
            )
            .field(
                "touch_pad0_tie_opt",
                &format_args!("{}", self.touch_pad0_tie_opt().bit()),
            )
            .field(
                "touch_pad0_start",
                &format_args!("{}", self.touch_pad0_start().bit()),
            )
            .field(
                "touch_pad0_dac",
                &format_args!("{}", self.touch_pad0_dac().bits()),
            )
            .field(
                "touch_pad0_rue",
                &format_args!("{}", self.touch_pad0_rue().bit()),
            )
            .field(
                "touch_pad0_rde",
                &format_args!("{}", self.touch_pad0_rde().bit()),
            )
            .field(
                "touch_pad0_drv",
                &format_args!("{}", self.touch_pad0_drv().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TOUCH_PAD_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 13 - Input enable in normal execution."]
    #[inline(always)]
    #[must_use]
    pub fn touch_pad0_fun_ie(&mut self) -> TOUCH_PAD0_FUN_IE_W<13> {
        TOUCH_PAD0_FUN_IE_W::new(self)
    }
    #[doc = "Bit 14 - Output enable in sleep mode."]
    #[inline(always)]
    #[must_use]
    pub fn touch_pad0_slp_oe(&mut self) -> TOUCH_PAD0_SLP_OE_W<14> {
        TOUCH_PAD0_SLP_OE_W::new(self)
    }
    #[doc = "Bit 15 - Input enable in sleep mode."]
    #[inline(always)]
    #[must_use]
    pub fn touch_pad0_slp_ie(&mut self) -> TOUCH_PAD0_SLP_IE_W<15> {
        TOUCH_PAD0_SLP_IE_W::new(self)
    }
    #[doc = "Bit 16 - 0: no sleep mode. 1: enable sleep mode."]
    #[inline(always)]
    #[must_use]
    pub fn touch_pad0_slp_sel(&mut self) -> TOUCH_PAD0_SLP_SEL_W<16> {
        TOUCH_PAD0_SLP_SEL_W::new(self)
    }
    #[doc = "Bits 17:18 - Function selection."]
    #[inline(always)]
    #[must_use]
    pub fn touch_pad0_fun_sel(&mut self) -> TOUCH_PAD0_FUN_SEL_W<17> {
        TOUCH_PAD0_FUN_SEL_W::new(self)
    }
    #[doc = "Bit 19 - Connect the RTC pad input to digital pad input. 0 is available."]
    #[inline(always)]
    #[must_use]
    pub fn touch_pad0_mux_sel(&mut self) -> TOUCH_PAD0_MUX_SEL_W<19> {
        TOUCH_PAD0_MUX_SEL_W::new(self)
    }
    #[doc = "Bit 20 - Touch sensor power on."]
    #[inline(always)]
    #[must_use]
    pub fn touch_pad0_xpd(&mut self) -> TOUCH_PAD0_XPD_W<20> {
        TOUCH_PAD0_XPD_W::new(self)
    }
    #[doc = "Bit 21 - The tie option of touch sensor. 0: tie low. 1: tie high."]
    #[inline(always)]
    #[must_use]
    pub fn touch_pad0_tie_opt(&mut self) -> TOUCH_PAD0_TIE_OPT_W<21> {
        TOUCH_PAD0_TIE_OPT_W::new(self)
    }
    #[doc = "Bit 22 - Start touch sensor."]
    #[inline(always)]
    #[must_use]
    pub fn touch_pad0_start(&mut self) -> TOUCH_PAD0_START_W<22> {
        TOUCH_PAD0_START_W::new(self)
    }
    #[doc = "Bits 23:25 - Touch sensor slope control. 3-bit for each touch pad, defaults to 0x4."]
    #[inline(always)]
    #[must_use]
    pub fn touch_pad0_dac(&mut self) -> TOUCH_PAD0_DAC_W<23> {
        TOUCH_PAD0_DAC_W::new(self)
    }
    #[doc = "Bit 27 - Pull-down enable of the pad. 1: internal pull-down enabled. 0: internal pull-down disabled."]
    #[inline(always)]
    #[must_use]
    pub fn touch_pad0_rue(&mut self) -> TOUCH_PAD0_RUE_W<27> {
        TOUCH_PAD0_RUE_W::new(self)
    }
    #[doc = "Bit 28 - Pull-up enable of the pad. 1: internal pull-up enabled. 0: internal pull-up disabled."]
    #[inline(always)]
    #[must_use]
    pub fn touch_pad0_rde(&mut self) -> TOUCH_PAD0_RDE_W<28> {
        TOUCH_PAD0_RDE_W::new(self)
    }
    #[doc = "Bits 29:30 - Select the drive strength of the pad. 0: ~5 mA. 1: ~10 mA. 2: ~20 mA. 3: ~40 mA."]
    #[inline(always)]
    #[must_use]
    pub fn touch_pad0_drv(&mut self) -> TOUCH_PAD0_DRV_W<29> {
        TOUCH_PAD0_DRV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Touch pad %s configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [touch_pad](index.html) module"]
pub struct TOUCH_PAD_SPEC;
impl crate::RegisterSpec for TOUCH_PAD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [touch_pad::R](R) reader structure"]
impl crate::Readable for TOUCH_PAD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [touch_pad::W](W) writer structure"]
impl crate::Writable for TOUCH_PAD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TOUCH_PAD%s to value 0x5200_0000"]
impl crate::Resettable for TOUCH_PAD_SPEC {
    const RESET_VALUE: Self::Ux = 0x5200_0000;
}
