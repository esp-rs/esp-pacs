#[doc = "Register `PAD_DAC2` reader"]
pub struct R(crate::R<PAD_DAC2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PAD_DAC2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PAD_DAC2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PAD_DAC2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PAD_DAC2` writer"]
pub struct W(crate::W<PAD_DAC2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PAD_DAC2_SPEC>;
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
impl From<crate::W<PAD_DAC2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PAD_DAC2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PDAC2_DAC_XPD_FORCE` reader - Power on DAC2. Usually we need to tristate PDAC2 if we power on the DAC i.e. IE=0 OE=0 RDE=0 RUE=0"]
pub type PDAC2_DAC_XPD_FORCE_R = crate::BitReader;
#[doc = "Field `PDAC2_DAC_XPD_FORCE` writer - Power on DAC2. Usually we need to tristate PDAC2 if we power on the DAC i.e. IE=0 OE=0 RDE=0 RUE=0"]
pub type PDAC2_DAC_XPD_FORCE_W<'a, const O: u8> = crate::BitWriter<'a, PAD_DAC2_SPEC, O>;
#[doc = "Field `PDAC2_FUN_IE` reader - the input enable of the pad"]
pub type PDAC2_FUN_IE_R = crate::BitReader;
#[doc = "Field `PDAC2_FUN_IE` writer - the input enable of the pad"]
pub type PDAC2_FUN_IE_W<'a, const O: u8> = crate::BitWriter<'a, PAD_DAC2_SPEC, O>;
#[doc = "Field `PDAC2_SLP_OE` reader - the output enable of the pad in sleep status"]
pub type PDAC2_SLP_OE_R = crate::BitReader;
#[doc = "Field `PDAC2_SLP_OE` writer - the output enable of the pad in sleep status"]
pub type PDAC2_SLP_OE_W<'a, const O: u8> = crate::BitWriter<'a, PAD_DAC2_SPEC, O>;
#[doc = "Field `PDAC2_SLP_IE` reader - the input enable of the pad in sleep status"]
pub type PDAC2_SLP_IE_R = crate::BitReader;
#[doc = "Field `PDAC2_SLP_IE` writer - the input enable of the pad in sleep status"]
pub type PDAC2_SLP_IE_W<'a, const O: u8> = crate::BitWriter<'a, PAD_DAC2_SPEC, O>;
#[doc = "Field `PDAC2_SLP_SEL` reader - the sleep status selection signal of the pad"]
pub type PDAC2_SLP_SEL_R = crate::BitReader;
#[doc = "Field `PDAC2_SLP_SEL` writer - the sleep status selection signal of the pad"]
pub type PDAC2_SLP_SEL_W<'a, const O: u8> = crate::BitWriter<'a, PAD_DAC2_SPEC, O>;
#[doc = "Field `PDAC2_FUN_SEL` reader - the functional selection signal of the pad"]
pub type PDAC2_FUN_SEL_R = crate::FieldReader;
#[doc = "Field `PDAC2_FUN_SEL` writer - the functional selection signal of the pad"]
pub type PDAC2_FUN_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, PAD_DAC2_SPEC, 2, O>;
#[doc = "Field `PDAC2_MUX_SEL` reader - Ò1Ó select the digital function Ó0Óslection the rtc function"]
pub type PDAC2_MUX_SEL_R = crate::BitReader;
#[doc = "Field `PDAC2_MUX_SEL` writer - Ò1Ó select the digital function Ó0Óslection the rtc function"]
pub type PDAC2_MUX_SEL_W<'a, const O: u8> = crate::BitWriter<'a, PAD_DAC2_SPEC, O>;
#[doc = "Field `PDAC2_XPD_DAC` reader - Power on DAC2. Usually we need to tristate PDAC1 if we power on the DAC i.e. IE=0 OE=0 RDE=0 RUE=0"]
pub type PDAC2_XPD_DAC_R = crate::BitReader;
#[doc = "Field `PDAC2_XPD_DAC` writer - Power on DAC2. Usually we need to tristate PDAC1 if we power on the DAC i.e. IE=0 OE=0 RDE=0 RUE=0"]
pub type PDAC2_XPD_DAC_W<'a, const O: u8> = crate::BitWriter<'a, PAD_DAC2_SPEC, O>;
#[doc = "Field `PDAC2_DAC` reader - PAD DAC2 control code."]
pub type PDAC2_DAC_R = crate::FieldReader;
#[doc = "Field `PDAC2_DAC` writer - PAD DAC2 control code."]
pub type PDAC2_DAC_W<'a, const O: u8> = crate::FieldWriter<'a, PAD_DAC2_SPEC, 8, O>;
#[doc = "Field `PDAC2_RUE` reader - the pull up enable of the pad"]
pub type PDAC2_RUE_R = crate::BitReader;
#[doc = "Field `PDAC2_RUE` writer - the pull up enable of the pad"]
pub type PDAC2_RUE_W<'a, const O: u8> = crate::BitWriter<'a, PAD_DAC2_SPEC, O>;
#[doc = "Field `PDAC2_RDE` reader - the pull down enable of the pad"]
pub type PDAC2_RDE_R = crate::BitReader;
#[doc = "Field `PDAC2_RDE` writer - the pull down enable of the pad"]
pub type PDAC2_RDE_W<'a, const O: u8> = crate::BitWriter<'a, PAD_DAC2_SPEC, O>;
#[doc = "Field `PDAC2_HOLD` reader - hold the current value of the output when setting the hold to Ò1Ó"]
pub type PDAC2_HOLD_R = crate::BitReader;
#[doc = "Field `PDAC2_HOLD` writer - hold the current value of the output when setting the hold to Ò1Ó"]
pub type PDAC2_HOLD_W<'a, const O: u8> = crate::BitWriter<'a, PAD_DAC2_SPEC, O>;
#[doc = "Field `PDAC2_DRV` reader - the driver strength of the pad"]
pub type PDAC2_DRV_R = crate::FieldReader;
#[doc = "Field `PDAC2_DRV` writer - the driver strength of the pad"]
pub type PDAC2_DRV_W<'a, const O: u8> = crate::FieldWriter<'a, PAD_DAC2_SPEC, 2, O>;
impl R {
    #[doc = "Bit 10 - Power on DAC2. Usually we need to tristate PDAC2 if we power on the DAC i.e. IE=0 OE=0 RDE=0 RUE=0"]
    #[inline(always)]
    pub fn pdac2_dac_xpd_force(&self) -> PDAC2_DAC_XPD_FORCE_R {
        PDAC2_DAC_XPD_FORCE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - the input enable of the pad"]
    #[inline(always)]
    pub fn pdac2_fun_ie(&self) -> PDAC2_FUN_IE_R {
        PDAC2_FUN_IE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - the output enable of the pad in sleep status"]
    #[inline(always)]
    pub fn pdac2_slp_oe(&self) -> PDAC2_SLP_OE_R {
        PDAC2_SLP_OE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - the input enable of the pad in sleep status"]
    #[inline(always)]
    pub fn pdac2_slp_ie(&self) -> PDAC2_SLP_IE_R {
        PDAC2_SLP_IE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - the sleep status selection signal of the pad"]
    #[inline(always)]
    pub fn pdac2_slp_sel(&self) -> PDAC2_SLP_SEL_R {
        PDAC2_SLP_SEL_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 15:16 - the functional selection signal of the pad"]
    #[inline(always)]
    pub fn pdac2_fun_sel(&self) -> PDAC2_FUN_SEL_R {
        PDAC2_FUN_SEL_R::new(((self.bits >> 15) & 3) as u8)
    }
    #[doc = "Bit 17 - Ò1Ó select the digital function Ó0Óslection the rtc function"]
    #[inline(always)]
    pub fn pdac2_mux_sel(&self) -> PDAC2_MUX_SEL_R {
        PDAC2_MUX_SEL_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Power on DAC2. Usually we need to tristate PDAC1 if we power on the DAC i.e. IE=0 OE=0 RDE=0 RUE=0"]
    #[inline(always)]
    pub fn pdac2_xpd_dac(&self) -> PDAC2_XPD_DAC_R {
        PDAC2_XPD_DAC_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:26 - PAD DAC2 control code."]
    #[inline(always)]
    pub fn pdac2_dac(&self) -> PDAC2_DAC_R {
        PDAC2_DAC_R::new(((self.bits >> 19) & 0xff) as u8)
    }
    #[doc = "Bit 27 - the pull up enable of the pad"]
    #[inline(always)]
    pub fn pdac2_rue(&self) -> PDAC2_RUE_R {
        PDAC2_RUE_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - the pull down enable of the pad"]
    #[inline(always)]
    pub fn pdac2_rde(&self) -> PDAC2_RDE_R {
        PDAC2_RDE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - hold the current value of the output when setting the hold to Ò1Ó"]
    #[inline(always)]
    pub fn pdac2_hold(&self) -> PDAC2_HOLD_R {
        PDAC2_HOLD_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bits 30:31 - the driver strength of the pad"]
    #[inline(always)]
    pub fn pdac2_drv(&self) -> PDAC2_DRV_R {
        PDAC2_DRV_R::new(((self.bits >> 30) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PAD_DAC2")
            .field(
                "pdac2_dac_xpd_force",
                &format_args!("{}", self.pdac2_dac_xpd_force().bit()),
            )
            .field(
                "pdac2_fun_ie",
                &format_args!("{}", self.pdac2_fun_ie().bit()),
            )
            .field(
                "pdac2_slp_oe",
                &format_args!("{}", self.pdac2_slp_oe().bit()),
            )
            .field(
                "pdac2_slp_ie",
                &format_args!("{}", self.pdac2_slp_ie().bit()),
            )
            .field(
                "pdac2_slp_sel",
                &format_args!("{}", self.pdac2_slp_sel().bit()),
            )
            .field(
                "pdac2_fun_sel",
                &format_args!("{}", self.pdac2_fun_sel().bits()),
            )
            .field(
                "pdac2_mux_sel",
                &format_args!("{}", self.pdac2_mux_sel().bit()),
            )
            .field(
                "pdac2_xpd_dac",
                &format_args!("{}", self.pdac2_xpd_dac().bit()),
            )
            .field("pdac2_dac", &format_args!("{}", self.pdac2_dac().bits()))
            .field("pdac2_rue", &format_args!("{}", self.pdac2_rue().bit()))
            .field("pdac2_rde", &format_args!("{}", self.pdac2_rde().bit()))
            .field("pdac2_hold", &format_args!("{}", self.pdac2_hold().bit()))
            .field("pdac2_drv", &format_args!("{}", self.pdac2_drv().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PAD_DAC2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 10 - Power on DAC2. Usually we need to tristate PDAC2 if we power on the DAC i.e. IE=0 OE=0 RDE=0 RUE=0"]
    #[inline(always)]
    #[must_use]
    pub fn pdac2_dac_xpd_force(&mut self) -> PDAC2_DAC_XPD_FORCE_W<10> {
        PDAC2_DAC_XPD_FORCE_W::new(self)
    }
    #[doc = "Bit 11 - the input enable of the pad"]
    #[inline(always)]
    #[must_use]
    pub fn pdac2_fun_ie(&mut self) -> PDAC2_FUN_IE_W<11> {
        PDAC2_FUN_IE_W::new(self)
    }
    #[doc = "Bit 12 - the output enable of the pad in sleep status"]
    #[inline(always)]
    #[must_use]
    pub fn pdac2_slp_oe(&mut self) -> PDAC2_SLP_OE_W<12> {
        PDAC2_SLP_OE_W::new(self)
    }
    #[doc = "Bit 13 - the input enable of the pad in sleep status"]
    #[inline(always)]
    #[must_use]
    pub fn pdac2_slp_ie(&mut self) -> PDAC2_SLP_IE_W<13> {
        PDAC2_SLP_IE_W::new(self)
    }
    #[doc = "Bit 14 - the sleep status selection signal of the pad"]
    #[inline(always)]
    #[must_use]
    pub fn pdac2_slp_sel(&mut self) -> PDAC2_SLP_SEL_W<14> {
        PDAC2_SLP_SEL_W::new(self)
    }
    #[doc = "Bits 15:16 - the functional selection signal of the pad"]
    #[inline(always)]
    #[must_use]
    pub fn pdac2_fun_sel(&mut self) -> PDAC2_FUN_SEL_W<15> {
        PDAC2_FUN_SEL_W::new(self)
    }
    #[doc = "Bit 17 - Ò1Ó select the digital function Ó0Óslection the rtc function"]
    #[inline(always)]
    #[must_use]
    pub fn pdac2_mux_sel(&mut self) -> PDAC2_MUX_SEL_W<17> {
        PDAC2_MUX_SEL_W::new(self)
    }
    #[doc = "Bit 18 - Power on DAC2. Usually we need to tristate PDAC1 if we power on the DAC i.e. IE=0 OE=0 RDE=0 RUE=0"]
    #[inline(always)]
    #[must_use]
    pub fn pdac2_xpd_dac(&mut self) -> PDAC2_XPD_DAC_W<18> {
        PDAC2_XPD_DAC_W::new(self)
    }
    #[doc = "Bits 19:26 - PAD DAC2 control code."]
    #[inline(always)]
    #[must_use]
    pub fn pdac2_dac(&mut self) -> PDAC2_DAC_W<19> {
        PDAC2_DAC_W::new(self)
    }
    #[doc = "Bit 27 - the pull up enable of the pad"]
    #[inline(always)]
    #[must_use]
    pub fn pdac2_rue(&mut self) -> PDAC2_RUE_W<27> {
        PDAC2_RUE_W::new(self)
    }
    #[doc = "Bit 28 - the pull down enable of the pad"]
    #[inline(always)]
    #[must_use]
    pub fn pdac2_rde(&mut self) -> PDAC2_RDE_W<28> {
        PDAC2_RDE_W::new(self)
    }
    #[doc = "Bit 29 - hold the current value of the output when setting the hold to Ò1Ó"]
    #[inline(always)]
    #[must_use]
    pub fn pdac2_hold(&mut self) -> PDAC2_HOLD_W<29> {
        PDAC2_HOLD_W::new(self)
    }
    #[doc = "Bits 30:31 - the driver strength of the pad"]
    #[inline(always)]
    #[must_use]
    pub fn pdac2_drv(&mut self) -> PDAC2_DRV_W<30> {
        PDAC2_DRV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pad_dac2](index.html) module"]
pub struct PAD_DAC2_SPEC;
impl crate::RegisterSpec for PAD_DAC2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pad_dac2::R](R) reader structure"]
impl crate::Readable for PAD_DAC2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pad_dac2::W](W) writer structure"]
impl crate::Writable for PAD_DAC2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PAD_DAC2 to value 0x8000_0000"]
impl crate::Resettable for PAD_DAC2_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000_0000;
}
