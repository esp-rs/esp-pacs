#[doc = "Register `XTAL_32P_PAD` reader"]
pub struct R(crate::R<XTAL_32P_PAD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<XTAL_32P_PAD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<XTAL_32P_PAD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<XTAL_32P_PAD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `XTAL_32P_PAD` writer"]
pub struct W(crate::W<XTAL_32P_PAD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<XTAL_32P_PAD_SPEC>;
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
impl From<crate::W<XTAL_32P_PAD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<XTAL_32P_PAD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `X32P_FUN_IE` reader - Input enable in normal execution."]
pub type X32P_FUN_IE_R = crate::BitReader;
#[doc = "Field `X32P_FUN_IE` writer - Input enable in normal execution."]
pub type X32P_FUN_IE_W<'a, const O: u8> = crate::BitWriter<'a, XTAL_32P_PAD_SPEC, O>;
#[doc = "Field `X32P_SLP_OE` reader - output enable in sleep mode."]
pub type X32P_SLP_OE_R = crate::BitReader;
#[doc = "Field `X32P_SLP_OE` writer - output enable in sleep mode."]
pub type X32P_SLP_OE_W<'a, const O: u8> = crate::BitWriter<'a, XTAL_32P_PAD_SPEC, O>;
#[doc = "Field `X32P_SLP_IE` reader - input enable in sleep mode."]
pub type X32P_SLP_IE_R = crate::BitReader;
#[doc = "Field `X32P_SLP_IE` writer - input enable in sleep mode."]
pub type X32P_SLP_IE_W<'a, const O: u8> = crate::BitWriter<'a, XTAL_32P_PAD_SPEC, O>;
#[doc = "Field `X32P_SLP_SEL` reader - 1: enable sleep mode. 0: no sleep mode."]
pub type X32P_SLP_SEL_R = crate::BitReader;
#[doc = "Field `X32P_SLP_SEL` writer - 1: enable sleep mode. 0: no sleep mode."]
pub type X32P_SLP_SEL_W<'a, const O: u8> = crate::BitWriter<'a, XTAL_32P_PAD_SPEC, O>;
#[doc = "Field `X32P_FUN_SEL` reader - Function selection."]
pub type X32P_FUN_SEL_R = crate::FieldReader;
#[doc = "Field `X32P_FUN_SEL` writer - Function selection."]
pub type X32P_FUN_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, XTAL_32P_PAD_SPEC, 2, O>;
#[doc = "Field `X32P_MUX_SEL` reader - 1: use RTC GPIO. 0: use digital GPIO."]
pub type X32P_MUX_SEL_R = crate::BitReader;
#[doc = "Field `X32P_MUX_SEL` writer - 1: use RTC GPIO. 0: use digital GPIO."]
pub type X32P_MUX_SEL_W<'a, const O: u8> = crate::BitWriter<'a, XTAL_32P_PAD_SPEC, O>;
#[doc = "Field `X32P_RUE` reader - Pull-down enable of the pad. 1: internal pull-down enabled. 0: internal pull-down disabled."]
pub type X32P_RUE_R = crate::BitReader;
#[doc = "Field `X32P_RUE` writer - Pull-down enable of the pad. 1: internal pull-down enabled. 0: internal pull-down disabled."]
pub type X32P_RUE_W<'a, const O: u8> = crate::BitWriter<'a, XTAL_32P_PAD_SPEC, O>;
#[doc = "Field `X32P_RDE` reader - Pull-up enable of the pad. 1: internal pull-up enabled. 0: internal pull-up disabled."]
pub type X32P_RDE_R = crate::BitReader;
#[doc = "Field `X32P_RDE` writer - Pull-up enable of the pad. 1: internal pull-up enabled. 0: internal pull-up disabled."]
pub type X32P_RDE_W<'a, const O: u8> = crate::BitWriter<'a, XTAL_32P_PAD_SPEC, O>;
#[doc = "Field `X32P_DRV` reader - Select the drive strength of the pad. 0: ~5 mA. 1: ~10 mA. 2: ~20 mA. 3: ~40 mA."]
pub type X32P_DRV_R = crate::FieldReader;
#[doc = "Field `X32P_DRV` writer - Select the drive strength of the pad. 0: ~5 mA. 1: ~10 mA. 2: ~20 mA. 3: ~40 mA."]
pub type X32P_DRV_W<'a, const O: u8> = crate::FieldWriter<'a, XTAL_32P_PAD_SPEC, 2, O>;
impl R {
    #[doc = "Bit 13 - Input enable in normal execution."]
    #[inline(always)]
    pub fn x32p_fun_ie(&self) -> X32P_FUN_IE_R {
        X32P_FUN_IE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - output enable in sleep mode."]
    #[inline(always)]
    pub fn x32p_slp_oe(&self) -> X32P_SLP_OE_R {
        X32P_SLP_OE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - input enable in sleep mode."]
    #[inline(always)]
    pub fn x32p_slp_ie(&self) -> X32P_SLP_IE_R {
        X32P_SLP_IE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 1: enable sleep mode. 0: no sleep mode."]
    #[inline(always)]
    pub fn x32p_slp_sel(&self) -> X32P_SLP_SEL_R {
        X32P_SLP_SEL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - Function selection."]
    #[inline(always)]
    pub fn x32p_fun_sel(&self) -> X32P_FUN_SEL_R {
        X32P_FUN_SEL_R::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bit 19 - 1: use RTC GPIO. 0: use digital GPIO."]
    #[inline(always)]
    pub fn x32p_mux_sel(&self) -> X32P_MUX_SEL_R {
        X32P_MUX_SEL_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 27 - Pull-down enable of the pad. 1: internal pull-down enabled. 0: internal pull-down disabled."]
    #[inline(always)]
    pub fn x32p_rue(&self) -> X32P_RUE_R {
        X32P_RUE_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Pull-up enable of the pad. 1: internal pull-up enabled. 0: internal pull-up disabled."]
    #[inline(always)]
    pub fn x32p_rde(&self) -> X32P_RDE_R {
        X32P_RDE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bits 29:30 - Select the drive strength of the pad. 0: ~5 mA. 1: ~10 mA. 2: ~20 mA. 3: ~40 mA."]
    #[inline(always)]
    pub fn x32p_drv(&self) -> X32P_DRV_R {
        X32P_DRV_R::new(((self.bits >> 29) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("XTAL_32P_PAD")
            .field("x32p_fun_ie", &format_args!("{}", self.x32p_fun_ie().bit()))
            .field("x32p_slp_oe", &format_args!("{}", self.x32p_slp_oe().bit()))
            .field("x32p_slp_ie", &format_args!("{}", self.x32p_slp_ie().bit()))
            .field(
                "x32p_slp_sel",
                &format_args!("{}", self.x32p_slp_sel().bit()),
            )
            .field(
                "x32p_fun_sel",
                &format_args!("{}", self.x32p_fun_sel().bits()),
            )
            .field(
                "x32p_mux_sel",
                &format_args!("{}", self.x32p_mux_sel().bit()),
            )
            .field("x32p_rue", &format_args!("{}", self.x32p_rue().bit()))
            .field("x32p_rde", &format_args!("{}", self.x32p_rde().bit()))
            .field("x32p_drv", &format_args!("{}", self.x32p_drv().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<XTAL_32P_PAD_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 13 - Input enable in normal execution."]
    #[inline(always)]
    #[must_use]
    pub fn x32p_fun_ie(&mut self) -> X32P_FUN_IE_W<13> {
        X32P_FUN_IE_W::new(self)
    }
    #[doc = "Bit 14 - output enable in sleep mode."]
    #[inline(always)]
    #[must_use]
    pub fn x32p_slp_oe(&mut self) -> X32P_SLP_OE_W<14> {
        X32P_SLP_OE_W::new(self)
    }
    #[doc = "Bit 15 - input enable in sleep mode."]
    #[inline(always)]
    #[must_use]
    pub fn x32p_slp_ie(&mut self) -> X32P_SLP_IE_W<15> {
        X32P_SLP_IE_W::new(self)
    }
    #[doc = "Bit 16 - 1: enable sleep mode. 0: no sleep mode."]
    #[inline(always)]
    #[must_use]
    pub fn x32p_slp_sel(&mut self) -> X32P_SLP_SEL_W<16> {
        X32P_SLP_SEL_W::new(self)
    }
    #[doc = "Bits 17:18 - Function selection."]
    #[inline(always)]
    #[must_use]
    pub fn x32p_fun_sel(&mut self) -> X32P_FUN_SEL_W<17> {
        X32P_FUN_SEL_W::new(self)
    }
    #[doc = "Bit 19 - 1: use RTC GPIO. 0: use digital GPIO."]
    #[inline(always)]
    #[must_use]
    pub fn x32p_mux_sel(&mut self) -> X32P_MUX_SEL_W<19> {
        X32P_MUX_SEL_W::new(self)
    }
    #[doc = "Bit 27 - Pull-down enable of the pad. 1: internal pull-down enabled. 0: internal pull-down disabled."]
    #[inline(always)]
    #[must_use]
    pub fn x32p_rue(&mut self) -> X32P_RUE_W<27> {
        X32P_RUE_W::new(self)
    }
    #[doc = "Bit 28 - Pull-up enable of the pad. 1: internal pull-up enabled. 0: internal pull-up disabled."]
    #[inline(always)]
    #[must_use]
    pub fn x32p_rde(&mut self) -> X32P_RDE_W<28> {
        X32P_RDE_W::new(self)
    }
    #[doc = "Bits 29:30 - Select the drive strength of the pad. 0: ~5 mA. 1: ~10 mA. 2: ~20 mA. 3: ~40 mA."]
    #[inline(always)]
    #[must_use]
    pub fn x32p_drv(&mut self) -> X32P_DRV_W<29> {
        X32P_DRV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "32KHz crystal P-pad configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xtal_32p_pad](index.html) module"]
pub struct XTAL_32P_PAD_SPEC;
impl crate::RegisterSpec for XTAL_32P_PAD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [xtal_32p_pad::R](R) reader structure"]
impl crate::Readable for XTAL_32P_PAD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [xtal_32p_pad::W](W) writer structure"]
impl crate::Writable for XTAL_32P_PAD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets XTAL_32P_PAD to value 0x4000_0000"]
impl crate::Resettable for XTAL_32P_PAD_SPEC {
    const RESET_VALUE: Self::Ux = 0x4000_0000;
}
