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
#[doc = "Field `FUN_IE` reader - Input enable in normal execution."]
pub type FUN_IE_R = crate::BitReader;
#[doc = "Field `FUN_IE` writer - Input enable in normal execution."]
pub type FUN_IE_W<'a, const O: u8> = crate::BitWriter<'a, TOUCH_PAD_SPEC, O>;
#[doc = "Field `SLP_OE` reader - Output enable in sleep mode."]
pub type SLP_OE_R = crate::BitReader;
#[doc = "Field `SLP_OE` writer - Output enable in sleep mode."]
pub type SLP_OE_W<'a, const O: u8> = crate::BitWriter<'a, TOUCH_PAD_SPEC, O>;
#[doc = "Field `SLP_IE` reader - Input enable in sleep mode."]
pub type SLP_IE_R = crate::BitReader;
#[doc = "Field `SLP_IE` writer - Input enable in sleep mode."]
pub type SLP_IE_W<'a, const O: u8> = crate::BitWriter<'a, TOUCH_PAD_SPEC, O>;
#[doc = "Field `SLP_SEL` reader - 0: no sleep mode. 1: enable sleep mode."]
pub type SLP_SEL_R = crate::BitReader;
#[doc = "Field `SLP_SEL` writer - 0: no sleep mode. 1: enable sleep mode."]
pub type SLP_SEL_W<'a, const O: u8> = crate::BitWriter<'a, TOUCH_PAD_SPEC, O>;
#[doc = "Field `FUN_SEL` reader - Function selection."]
pub type FUN_SEL_R = crate::FieldReader;
#[doc = "Field `FUN_SEL` writer - Function selection."]
pub type FUN_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, TOUCH_PAD_SPEC, 2, O>;
#[doc = "Field `MUX_SEL` reader - Connect the RTC pad input to digital pad input. 0 is available."]
pub type MUX_SEL_R = crate::BitReader;
#[doc = "Field `MUX_SEL` writer - Connect the RTC pad input to digital pad input. 0 is available."]
pub type MUX_SEL_W<'a, const O: u8> = crate::BitWriter<'a, TOUCH_PAD_SPEC, O>;
#[doc = "Field `XPD` reader - Touch sensor power on."]
pub type XPD_R = crate::BitReader;
#[doc = "Field `XPD` writer - Touch sensor power on."]
pub type XPD_W<'a, const O: u8> = crate::BitWriter<'a, TOUCH_PAD_SPEC, O>;
#[doc = "Field `TIE_OPT` reader - The tie option of touch sensor. 0: tie low. 1: tie high."]
pub type TIE_OPT_R = crate::BitReader;
#[doc = "Field `TIE_OPT` writer - The tie option of touch sensor. 0: tie low. 1: tie high."]
pub type TIE_OPT_W<'a, const O: u8> = crate::BitWriter<'a, TOUCH_PAD_SPEC, O>;
#[doc = "Field `START` reader - Start touch sensor."]
pub type START_R = crate::BitReader;
#[doc = "Field `START` writer - Start touch sensor."]
pub type START_W<'a, const O: u8> = crate::BitWriter<'a, TOUCH_PAD_SPEC, O>;
#[doc = "Field `DAC` reader - Touch sensor slope control. 3-bit for each touch pad, defaults to 0x4."]
pub type DAC_R = crate::FieldReader;
#[doc = "Field `DAC` writer - Touch sensor slope control. 3-bit for each touch pad, defaults to 0x4."]
pub type DAC_W<'a, const O: u8> = crate::FieldWriter<'a, TOUCH_PAD_SPEC, 3, O>;
#[doc = "Field `RUE` reader - Pull-down enable of the pad. 1: internal pull-down enabled. 0: internal pull-down disabled."]
pub type RUE_R = crate::BitReader;
#[doc = "Field `RUE` writer - Pull-down enable of the pad. 1: internal pull-down enabled. 0: internal pull-down disabled."]
pub type RUE_W<'a, const O: u8> = crate::BitWriter<'a, TOUCH_PAD_SPEC, O>;
#[doc = "Field `RDE` reader - Pull-up enable of the pad. 1: internal pull-up enabled. 0: internal pull-up disabled."]
pub type RDE_R = crate::BitReader;
#[doc = "Field `RDE` writer - Pull-up enable of the pad. 1: internal pull-up enabled. 0: internal pull-up disabled."]
pub type RDE_W<'a, const O: u8> = crate::BitWriter<'a, TOUCH_PAD_SPEC, O>;
#[doc = "Field `DRV` reader - Select the drive strength of the pad. 0: ~5 mA. 1: ~10 mA. 2: ~20 mA. 3: ~40 mA."]
pub type DRV_R = crate::FieldReader;
#[doc = "Field `DRV` writer - Select the drive strength of the pad. 0: ~5 mA. 1: ~10 mA. 2: ~20 mA. 3: ~40 mA."]
pub type DRV_W<'a, const O: u8> = crate::FieldWriter<'a, TOUCH_PAD_SPEC, 2, O>;
impl R {
    #[doc = "Bit 13 - Input enable in normal execution."]
    #[inline(always)]
    pub fn fun_ie(&self) -> FUN_IE_R {
        FUN_IE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Output enable in sleep mode."]
    #[inline(always)]
    pub fn slp_oe(&self) -> SLP_OE_R {
        SLP_OE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Input enable in sleep mode."]
    #[inline(always)]
    pub fn slp_ie(&self) -> SLP_IE_R {
        SLP_IE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 0: no sleep mode. 1: enable sleep mode."]
    #[inline(always)]
    pub fn slp_sel(&self) -> SLP_SEL_R {
        SLP_SEL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - Function selection."]
    #[inline(always)]
    pub fn fun_sel(&self) -> FUN_SEL_R {
        FUN_SEL_R::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bit 19 - Connect the RTC pad input to digital pad input. 0 is available."]
    #[inline(always)]
    pub fn mux_sel(&self) -> MUX_SEL_R {
        MUX_SEL_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Touch sensor power on."]
    #[inline(always)]
    pub fn xpd(&self) -> XPD_R {
        XPD_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - The tie option of touch sensor. 0: tie low. 1: tie high."]
    #[inline(always)]
    pub fn tie_opt(&self) -> TIE_OPT_R {
        TIE_OPT_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Start touch sensor."]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 23:25 - Touch sensor slope control. 3-bit for each touch pad, defaults to 0x4."]
    #[inline(always)]
    pub fn dac(&self) -> DAC_R {
        DAC_R::new(((self.bits >> 23) & 7) as u8)
    }
    #[doc = "Bit 27 - Pull-down enable of the pad. 1: internal pull-down enabled. 0: internal pull-down disabled."]
    #[inline(always)]
    pub fn rue(&self) -> RUE_R {
        RUE_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Pull-up enable of the pad. 1: internal pull-up enabled. 0: internal pull-up disabled."]
    #[inline(always)]
    pub fn rde(&self) -> RDE_R {
        RDE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bits 29:30 - Select the drive strength of the pad. 0: ~5 mA. 1: ~10 mA. 2: ~20 mA. 3: ~40 mA."]
    #[inline(always)]
    pub fn drv(&self) -> DRV_R {
        DRV_R::new(((self.bits >> 29) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TOUCH_PAD")
            .field("fun_ie", &format_args!("{}", self.fun_ie().bit()))
            .field("slp_oe", &format_args!("{}", self.slp_oe().bit()))
            .field("slp_ie", &format_args!("{}", self.slp_ie().bit()))
            .field("slp_sel", &format_args!("{}", self.slp_sel().bit()))
            .field("fun_sel", &format_args!("{}", self.fun_sel().bits()))
            .field("mux_sel", &format_args!("{}", self.mux_sel().bit()))
            .field("xpd", &format_args!("{}", self.xpd().bit()))
            .field("tie_opt", &format_args!("{}", self.tie_opt().bit()))
            .field("start", &format_args!("{}", self.start().bit()))
            .field("dac", &format_args!("{}", self.dac().bits()))
            .field("rue", &format_args!("{}", self.rue().bit()))
            .field("rde", &format_args!("{}", self.rde().bit()))
            .field("drv", &format_args!("{}", self.drv().bits()))
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
    pub fn fun_ie(&mut self) -> FUN_IE_W<13> {
        FUN_IE_W::new(self)
    }
    #[doc = "Bit 14 - Output enable in sleep mode."]
    #[inline(always)]
    #[must_use]
    pub fn slp_oe(&mut self) -> SLP_OE_W<14> {
        SLP_OE_W::new(self)
    }
    #[doc = "Bit 15 - Input enable in sleep mode."]
    #[inline(always)]
    #[must_use]
    pub fn slp_ie(&mut self) -> SLP_IE_W<15> {
        SLP_IE_W::new(self)
    }
    #[doc = "Bit 16 - 0: no sleep mode. 1: enable sleep mode."]
    #[inline(always)]
    #[must_use]
    pub fn slp_sel(&mut self) -> SLP_SEL_W<16> {
        SLP_SEL_W::new(self)
    }
    #[doc = "Bits 17:18 - Function selection."]
    #[inline(always)]
    #[must_use]
    pub fn fun_sel(&mut self) -> FUN_SEL_W<17> {
        FUN_SEL_W::new(self)
    }
    #[doc = "Bit 19 - Connect the RTC pad input to digital pad input. 0 is available."]
    #[inline(always)]
    #[must_use]
    pub fn mux_sel(&mut self) -> MUX_SEL_W<19> {
        MUX_SEL_W::new(self)
    }
    #[doc = "Bit 20 - Touch sensor power on."]
    #[inline(always)]
    #[must_use]
    pub fn xpd(&mut self) -> XPD_W<20> {
        XPD_W::new(self)
    }
    #[doc = "Bit 21 - The tie option of touch sensor. 0: tie low. 1: tie high."]
    #[inline(always)]
    #[must_use]
    pub fn tie_opt(&mut self) -> TIE_OPT_W<21> {
        TIE_OPT_W::new(self)
    }
    #[doc = "Bit 22 - Start touch sensor."]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<22> {
        START_W::new(self)
    }
    #[doc = "Bits 23:25 - Touch sensor slope control. 3-bit for each touch pad, defaults to 0x4."]
    #[inline(always)]
    #[must_use]
    pub fn dac(&mut self) -> DAC_W<23> {
        DAC_W::new(self)
    }
    #[doc = "Bit 27 - Pull-down enable of the pad. 1: internal pull-down enabled. 0: internal pull-down disabled."]
    #[inline(always)]
    #[must_use]
    pub fn rue(&mut self) -> RUE_W<27> {
        RUE_W::new(self)
    }
    #[doc = "Bit 28 - Pull-up enable of the pad. 1: internal pull-up enabled. 0: internal pull-up disabled."]
    #[inline(always)]
    #[must_use]
    pub fn rde(&mut self) -> RDE_W<28> {
        RDE_W::new(self)
    }
    #[doc = "Bits 29:30 - Select the drive strength of the pad. 0: ~5 mA. 1: ~10 mA. 2: ~20 mA. 3: ~40 mA."]
    #[inline(always)]
    #[must_use]
    pub fn drv(&mut self) -> DRV_W<29> {
        DRV_W::new(self)
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
