#[doc = "Register `SAR_AMP_CTRL3` reader"]
pub struct R(crate::R<SAR_AMP_CTRL3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAR_AMP_CTRL3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAR_AMP_CTRL3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAR_AMP_CTRL3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAR_AMP_CTRL3` writer"]
pub struct W(crate::W<SAR_AMP_CTRL3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAR_AMP_CTRL3_SPEC>;
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
impl From<crate::W<SAR_AMP_CTRL3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAR_AMP_CTRL3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SAR1_DAC_XPD_FSM` reader - no public"]
pub type SAR1_DAC_XPD_FSM_R = crate::FieldReader;
#[doc = "Field `SAR1_DAC_XPD_FSM` writer - no public"]
pub type SAR1_DAC_XPD_FSM_W<'a, const O: u8> = crate::FieldWriter<'a, SAR_AMP_CTRL3_SPEC, 4, O>;
#[doc = "Field `XPD_SAR_AMP_FSM` reader - no public"]
pub type XPD_SAR_AMP_FSM_R = crate::FieldReader;
#[doc = "Field `XPD_SAR_AMP_FSM` writer - no public"]
pub type XPD_SAR_AMP_FSM_W<'a, const O: u8> = crate::FieldWriter<'a, SAR_AMP_CTRL3_SPEC, 4, O>;
#[doc = "Field `AMP_RST_FB_FSM` reader - no public"]
pub type AMP_RST_FB_FSM_R = crate::FieldReader;
#[doc = "Field `AMP_RST_FB_FSM` writer - no public"]
pub type AMP_RST_FB_FSM_W<'a, const O: u8> = crate::FieldWriter<'a, SAR_AMP_CTRL3_SPEC, 4, O>;
#[doc = "Field `AMP_SHORT_REF_FSM` reader - no public"]
pub type AMP_SHORT_REF_FSM_R = crate::FieldReader;
#[doc = "Field `AMP_SHORT_REF_FSM` writer - no public"]
pub type AMP_SHORT_REF_FSM_W<'a, const O: u8> = crate::FieldWriter<'a, SAR_AMP_CTRL3_SPEC, 4, O>;
#[doc = "Field `AMP_SHORT_REF_GND_FSM` reader - no public"]
pub type AMP_SHORT_REF_GND_FSM_R = crate::FieldReader;
#[doc = "Field `AMP_SHORT_REF_GND_FSM` writer - no public"]
pub type AMP_SHORT_REF_GND_FSM_W<'a, const O: u8> =
    crate::FieldWriter<'a, SAR_AMP_CTRL3_SPEC, 4, O>;
#[doc = "Field `XPD_SAR_FSM` reader - no public"]
pub type XPD_SAR_FSM_R = crate::FieldReader;
#[doc = "Field `XPD_SAR_FSM` writer - no public"]
pub type XPD_SAR_FSM_W<'a, const O: u8> = crate::FieldWriter<'a, SAR_AMP_CTRL3_SPEC, 4, O>;
#[doc = "Field `RSTB_FSM` reader - no public"]
pub type RSTB_FSM_R = crate::FieldReader;
#[doc = "Field `RSTB_FSM` writer - no public"]
pub type RSTB_FSM_W<'a, const O: u8> = crate::FieldWriter<'a, SAR_AMP_CTRL3_SPEC, 4, O>;
impl R {
    #[doc = "Bits 0:3 - no public"]
    #[inline(always)]
    pub fn sar1_dac_xpd_fsm(&self) -> SAR1_DAC_XPD_FSM_R {
        SAR1_DAC_XPD_FSM_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - no public"]
    #[inline(always)]
    pub fn xpd_sar_amp_fsm(&self) -> XPD_SAR_AMP_FSM_R {
        XPD_SAR_AMP_FSM_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - no public"]
    #[inline(always)]
    pub fn amp_rst_fb_fsm(&self) -> AMP_RST_FB_FSM_R {
        AMP_RST_FB_FSM_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - no public"]
    #[inline(always)]
    pub fn amp_short_ref_fsm(&self) -> AMP_SHORT_REF_FSM_R {
        AMP_SHORT_REF_FSM_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - no public"]
    #[inline(always)]
    pub fn amp_short_ref_gnd_fsm(&self) -> AMP_SHORT_REF_GND_FSM_R {
        AMP_SHORT_REF_GND_FSM_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - no public"]
    #[inline(always)]
    pub fn xpd_sar_fsm(&self) -> XPD_SAR_FSM_R {
        XPD_SAR_FSM_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - no public"]
    #[inline(always)]
    pub fn rstb_fsm(&self) -> RSTB_FSM_R {
        RSTB_FSM_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_AMP_CTRL3")
            .field(
                "sar1_dac_xpd_fsm",
                &format_args!("{}", self.sar1_dac_xpd_fsm().bits()),
            )
            .field(
                "xpd_sar_amp_fsm",
                &format_args!("{}", self.xpd_sar_amp_fsm().bits()),
            )
            .field(
                "amp_rst_fb_fsm",
                &format_args!("{}", self.amp_rst_fb_fsm().bits()),
            )
            .field(
                "amp_short_ref_fsm",
                &format_args!("{}", self.amp_short_ref_fsm().bits()),
            )
            .field(
                "amp_short_ref_gnd_fsm",
                &format_args!("{}", self.amp_short_ref_gnd_fsm().bits()),
            )
            .field(
                "xpd_sar_fsm",
                &format_args!("{}", self.xpd_sar_fsm().bits()),
            )
            .field("rstb_fsm", &format_args!("{}", self.rstb_fsm().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SAR_AMP_CTRL3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:3 - no public"]
    #[inline(always)]
    #[must_use]
    pub fn sar1_dac_xpd_fsm(&mut self) -> SAR1_DAC_XPD_FSM_W<0> {
        SAR1_DAC_XPD_FSM_W::new(self)
    }
    #[doc = "Bits 4:7 - no public"]
    #[inline(always)]
    #[must_use]
    pub fn xpd_sar_amp_fsm(&mut self) -> XPD_SAR_AMP_FSM_W<4> {
        XPD_SAR_AMP_FSM_W::new(self)
    }
    #[doc = "Bits 8:11 - no public"]
    #[inline(always)]
    #[must_use]
    pub fn amp_rst_fb_fsm(&mut self) -> AMP_RST_FB_FSM_W<8> {
        AMP_RST_FB_FSM_W::new(self)
    }
    #[doc = "Bits 12:15 - no public"]
    #[inline(always)]
    #[must_use]
    pub fn amp_short_ref_fsm(&mut self) -> AMP_SHORT_REF_FSM_W<12> {
        AMP_SHORT_REF_FSM_W::new(self)
    }
    #[doc = "Bits 16:19 - no public"]
    #[inline(always)]
    #[must_use]
    pub fn amp_short_ref_gnd_fsm(&mut self) -> AMP_SHORT_REF_GND_FSM_W<16> {
        AMP_SHORT_REF_GND_FSM_W::new(self)
    }
    #[doc = "Bits 20:23 - no public"]
    #[inline(always)]
    #[must_use]
    pub fn xpd_sar_fsm(&mut self) -> XPD_SAR_FSM_W<20> {
        XPD_SAR_FSM_W::new(self)
    }
    #[doc = "Bits 24:27 - no public"]
    #[inline(always)]
    #[must_use]
    pub fn rstb_fsm(&mut self) -> RSTB_FSM_W<24> {
        RSTB_FSM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "no public\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar_amp_ctrl3](index.html) module"]
pub struct SAR_AMP_CTRL3_SPEC;
impl crate::RegisterSpec for SAR_AMP_CTRL3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sar_amp_ctrl3::R](R) reader structure"]
impl crate::Readable for SAR_AMP_CTRL3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sar_amp_ctrl3::W](W) writer structure"]
impl crate::Writable for SAR_AMP_CTRL3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SAR_AMP_CTRL3 to value 0x0073_38f3"]
impl crate::Resettable for SAR_AMP_CTRL3_SPEC {
    const RESET_VALUE: Self::Ux = 0x0073_38f3;
}
