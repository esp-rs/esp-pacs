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
#[doc = "Field `SAR_SAR1_DAC_XPD_FSM` reader - no public"]
pub type SAR_SAR1_DAC_XPD_FSM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SAR_SAR1_DAC_XPD_FSM` writer - no public"]
pub type SAR_SAR1_DAC_XPD_FSM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SAR_AMP_CTRL3_SPEC, u8, u8, 4, O>;
#[doc = "Field `SAR_XPD_SAR_AMP_FSM` reader - no public"]
pub type SAR_XPD_SAR_AMP_FSM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SAR_XPD_SAR_AMP_FSM` writer - no public"]
pub type SAR_XPD_SAR_AMP_FSM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SAR_AMP_CTRL3_SPEC, u8, u8, 4, O>;
#[doc = "Field `SAR_AMP_RST_FB_FSM` reader - no public"]
pub type SAR_AMP_RST_FB_FSM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SAR_AMP_RST_FB_FSM` writer - no public"]
pub type SAR_AMP_RST_FB_FSM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SAR_AMP_CTRL3_SPEC, u8, u8, 4, O>;
#[doc = "Field `SAR_AMP_SHORT_REF_FSM` reader - no public"]
pub type SAR_AMP_SHORT_REF_FSM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SAR_AMP_SHORT_REF_FSM` writer - no public"]
pub type SAR_AMP_SHORT_REF_FSM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SAR_AMP_CTRL3_SPEC, u8, u8, 4, O>;
#[doc = "Field `SAR_AMP_SHORT_REF_GND_FSM` reader - no public"]
pub type SAR_AMP_SHORT_REF_GND_FSM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SAR_AMP_SHORT_REF_GND_FSM` writer - no public"]
pub type SAR_AMP_SHORT_REF_GND_FSM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SAR_AMP_CTRL3_SPEC, u8, u8, 4, O>;
#[doc = "Field `SAR_XPD_SAR_FSM` reader - no public"]
pub type SAR_XPD_SAR_FSM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SAR_XPD_SAR_FSM` writer - no public"]
pub type SAR_XPD_SAR_FSM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SAR_AMP_CTRL3_SPEC, u8, u8, 4, O>;
#[doc = "Field `SAR_RSTB_FSM` reader - no public"]
pub type SAR_RSTB_FSM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SAR_RSTB_FSM` writer - no public"]
pub type SAR_RSTB_FSM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SAR_AMP_CTRL3_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - no public"]
    #[inline(always)]
    pub fn sar_sar1_dac_xpd_fsm(&self) -> SAR_SAR1_DAC_XPD_FSM_R {
        SAR_SAR1_DAC_XPD_FSM_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - no public"]
    #[inline(always)]
    pub fn sar_xpd_sar_amp_fsm(&self) -> SAR_XPD_SAR_AMP_FSM_R {
        SAR_XPD_SAR_AMP_FSM_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - no public"]
    #[inline(always)]
    pub fn sar_amp_rst_fb_fsm(&self) -> SAR_AMP_RST_FB_FSM_R {
        SAR_AMP_RST_FB_FSM_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - no public"]
    #[inline(always)]
    pub fn sar_amp_short_ref_fsm(&self) -> SAR_AMP_SHORT_REF_FSM_R {
        SAR_AMP_SHORT_REF_FSM_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - no public"]
    #[inline(always)]
    pub fn sar_amp_short_ref_gnd_fsm(&self) -> SAR_AMP_SHORT_REF_GND_FSM_R {
        SAR_AMP_SHORT_REF_GND_FSM_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - no public"]
    #[inline(always)]
    pub fn sar_xpd_sar_fsm(&self) -> SAR_XPD_SAR_FSM_R {
        SAR_XPD_SAR_FSM_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - no public"]
    #[inline(always)]
    pub fn sar_rstb_fsm(&self) -> SAR_RSTB_FSM_R {
        SAR_RSTB_FSM_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - no public"]
    #[inline(always)]
    pub fn sar_sar1_dac_xpd_fsm(&mut self) -> SAR_SAR1_DAC_XPD_FSM_W<0> {
        SAR_SAR1_DAC_XPD_FSM_W::new(self)
    }
    #[doc = "Bits 4:7 - no public"]
    #[inline(always)]
    pub fn sar_xpd_sar_amp_fsm(&mut self) -> SAR_XPD_SAR_AMP_FSM_W<4> {
        SAR_XPD_SAR_AMP_FSM_W::new(self)
    }
    #[doc = "Bits 8:11 - no public"]
    #[inline(always)]
    pub fn sar_amp_rst_fb_fsm(&mut self) -> SAR_AMP_RST_FB_FSM_W<8> {
        SAR_AMP_RST_FB_FSM_W::new(self)
    }
    #[doc = "Bits 12:15 - no public"]
    #[inline(always)]
    pub fn sar_amp_short_ref_fsm(&mut self) -> SAR_AMP_SHORT_REF_FSM_W<12> {
        SAR_AMP_SHORT_REF_FSM_W::new(self)
    }
    #[doc = "Bits 16:19 - no public"]
    #[inline(always)]
    pub fn sar_amp_short_ref_gnd_fsm(&mut self) -> SAR_AMP_SHORT_REF_GND_FSM_W<16> {
        SAR_AMP_SHORT_REF_GND_FSM_W::new(self)
    }
    #[doc = "Bits 20:23 - no public"]
    #[inline(always)]
    pub fn sar_xpd_sar_fsm(&mut self) -> SAR_XPD_SAR_FSM_W<20> {
        SAR_XPD_SAR_FSM_W::new(self)
    }
    #[doc = "Bits 24:27 - no public"]
    #[inline(always)]
    pub fn sar_rstb_fsm(&mut self) -> SAR_RSTB_FSM_W<24> {
        SAR_RSTB_FSM_W::new(self)
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
}
#[doc = "`reset()` method sets SAR_AMP_CTRL3 to value 0x0073_38f3"]
impl crate::Resettable for SAR_AMP_CTRL3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0073_38f3
    }
}
