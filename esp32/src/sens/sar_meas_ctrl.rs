#[doc = "Register `SAR_MEAS_CTRL` reader"]
pub struct R(crate::R<SAR_MEAS_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAR_MEAS_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAR_MEAS_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAR_MEAS_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAR_MEAS_CTRL` writer"]
pub struct W(crate::W<SAR_MEAS_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAR_MEAS_CTRL_SPEC>;
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
impl From<crate::W<SAR_MEAS_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAR_MEAS_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XPD_SAR_AMP_FSM` reader - "]
pub type XPD_SAR_AMP_FSM_R = crate::FieldReader;
#[doc = "Field `XPD_SAR_AMP_FSM` writer - "]
pub type XPD_SAR_AMP_FSM_W<'a, const O: u8> = crate::FieldWriter<'a, SAR_MEAS_CTRL_SPEC, 4, O>;
#[doc = "Field `AMP_RST_FB_FSM` reader - "]
pub type AMP_RST_FB_FSM_R = crate::FieldReader;
#[doc = "Field `AMP_RST_FB_FSM` writer - "]
pub type AMP_RST_FB_FSM_W<'a, const O: u8> = crate::FieldWriter<'a, SAR_MEAS_CTRL_SPEC, 4, O>;
#[doc = "Field `AMP_SHORT_REF_FSM` reader - "]
pub type AMP_SHORT_REF_FSM_R = crate::FieldReader;
#[doc = "Field `AMP_SHORT_REF_FSM` writer - "]
pub type AMP_SHORT_REF_FSM_W<'a, const O: u8> = crate::FieldWriter<'a, SAR_MEAS_CTRL_SPEC, 4, O>;
#[doc = "Field `AMP_SHORT_REF_GND_FSM` reader - "]
pub type AMP_SHORT_REF_GND_FSM_R = crate::FieldReader;
#[doc = "Field `AMP_SHORT_REF_GND_FSM` writer - "]
pub type AMP_SHORT_REF_GND_FSM_W<'a, const O: u8> =
    crate::FieldWriter<'a, SAR_MEAS_CTRL_SPEC, 4, O>;
#[doc = "Field `XPD_SAR_FSM` reader - "]
pub type XPD_SAR_FSM_R = crate::FieldReader;
#[doc = "Field `XPD_SAR_FSM` writer - "]
pub type XPD_SAR_FSM_W<'a, const O: u8> = crate::FieldWriter<'a, SAR_MEAS_CTRL_SPEC, 4, O>;
#[doc = "Field `SAR_RSTB_FSM` reader - "]
pub type SAR_RSTB_FSM_R = crate::FieldReader;
#[doc = "Field `SAR_RSTB_FSM` writer - "]
pub type SAR_RSTB_FSM_W<'a, const O: u8> = crate::FieldWriter<'a, SAR_MEAS_CTRL_SPEC, 4, O>;
#[doc = "Field `SAR2_XPD_WAIT` reader - "]
pub type SAR2_XPD_WAIT_R = crate::FieldReader;
#[doc = "Field `SAR2_XPD_WAIT` writer - "]
pub type SAR2_XPD_WAIT_W<'a, const O: u8> = crate::FieldWriter<'a, SAR_MEAS_CTRL_SPEC, 8, O>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn xpd_sar_amp_fsm(&self) -> XPD_SAR_AMP_FSM_R {
        XPD_SAR_AMP_FSM_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn amp_rst_fb_fsm(&self) -> AMP_RST_FB_FSM_R {
        AMP_RST_FB_FSM_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn amp_short_ref_fsm(&self) -> AMP_SHORT_REF_FSM_R {
        AMP_SHORT_REF_FSM_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn amp_short_ref_gnd_fsm(&self) -> AMP_SHORT_REF_GND_FSM_R {
        AMP_SHORT_REF_GND_FSM_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn xpd_sar_fsm(&self) -> XPD_SAR_FSM_R {
        XPD_SAR_FSM_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn sar_rstb_fsm(&self) -> SAR_RSTB_FSM_R {
        SAR_RSTB_FSM_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn sar2_xpd_wait(&self) -> SAR2_XPD_WAIT_R {
        SAR2_XPD_WAIT_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_MEAS_CTRL")
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
            .field(
                "sar_rstb_fsm",
                &format_args!("{}", self.sar_rstb_fsm().bits()),
            )
            .field(
                "sar2_xpd_wait",
                &format_args!("{}", self.sar2_xpd_wait().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SAR_MEAS_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    #[must_use]
    pub fn xpd_sar_amp_fsm(&mut self) -> XPD_SAR_AMP_FSM_W<0> {
        XPD_SAR_AMP_FSM_W::new(self)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    #[must_use]
    pub fn amp_rst_fb_fsm(&mut self) -> AMP_RST_FB_FSM_W<4> {
        AMP_RST_FB_FSM_W::new(self)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    #[must_use]
    pub fn amp_short_ref_fsm(&mut self) -> AMP_SHORT_REF_FSM_W<8> {
        AMP_SHORT_REF_FSM_W::new(self)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    #[must_use]
    pub fn amp_short_ref_gnd_fsm(&mut self) -> AMP_SHORT_REF_GND_FSM_W<12> {
        AMP_SHORT_REF_GND_FSM_W::new(self)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    #[must_use]
    pub fn xpd_sar_fsm(&mut self) -> XPD_SAR_FSM_W<16> {
        XPD_SAR_FSM_W::new(self)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    #[must_use]
    pub fn sar_rstb_fsm(&mut self) -> SAR_RSTB_FSM_W<20> {
        SAR_RSTB_FSM_W::new(self)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    #[must_use]
    pub fn sar2_xpd_wait(&mut self) -> SAR2_XPD_WAIT_W<24> {
        SAR2_XPD_WAIT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar_meas_ctrl](index.html) module"]
pub struct SAR_MEAS_CTRL_SPEC;
impl crate::RegisterSpec for SAR_MEAS_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sar_meas_ctrl::R](R) reader structure"]
impl crate::Readable for SAR_MEAS_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sar_meas_ctrl::W](W) writer structure"]
impl crate::Writable for SAR_MEAS_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SAR_MEAS_CTRL to value 0x0707_338f"]
impl crate::Resettable for SAR_MEAS_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0707_338f;
}
