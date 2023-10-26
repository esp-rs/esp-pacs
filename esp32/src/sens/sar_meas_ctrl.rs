#[doc = "Register `SAR_MEAS_CTRL` reader"]
pub type R = crate::R<SAR_MEAS_CTRL_SPEC>;
#[doc = "Register `SAR_MEAS_CTRL` writer"]
pub type W = crate::W<SAR_MEAS_CTRL_SPEC>;
#[doc = "Field `XPD_SAR_AMP_FSM` reader - "]
pub type XPD_SAR_AMP_FSM_R = crate::FieldReader;
#[doc = "Field `XPD_SAR_AMP_FSM` writer - "]
pub type XPD_SAR_AMP_FSM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `AMP_RST_FB_FSM` reader - "]
pub type AMP_RST_FB_FSM_R = crate::FieldReader;
#[doc = "Field `AMP_RST_FB_FSM` writer - "]
pub type AMP_RST_FB_FSM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `AMP_SHORT_REF_FSM` reader - "]
pub type AMP_SHORT_REF_FSM_R = crate::FieldReader;
#[doc = "Field `AMP_SHORT_REF_FSM` writer - "]
pub type AMP_SHORT_REF_FSM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `AMP_SHORT_REF_GND_FSM` reader - "]
pub type AMP_SHORT_REF_GND_FSM_R = crate::FieldReader;
#[doc = "Field `AMP_SHORT_REF_GND_FSM` writer - "]
pub type AMP_SHORT_REF_GND_FSM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `XPD_SAR_FSM` reader - "]
pub type XPD_SAR_FSM_R = crate::FieldReader;
#[doc = "Field `XPD_SAR_FSM` writer - "]
pub type XPD_SAR_FSM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `SAR_RSTB_FSM` reader - "]
pub type SAR_RSTB_FSM_R = crate::FieldReader;
#[doc = "Field `SAR_RSTB_FSM` writer - "]
pub type SAR_RSTB_FSM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `SAR2_XPD_WAIT` reader - "]
pub type SAR2_XPD_WAIT_R = crate::FieldReader;
#[doc = "Field `SAR2_XPD_WAIT` writer - "]
pub type SAR2_XPD_WAIT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
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
    pub fn xpd_sar_amp_fsm(&mut self) -> XPD_SAR_AMP_FSM_W<SAR_MEAS_CTRL_SPEC, 0> {
        XPD_SAR_AMP_FSM_W::new(self)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    #[must_use]
    pub fn amp_rst_fb_fsm(&mut self) -> AMP_RST_FB_FSM_W<SAR_MEAS_CTRL_SPEC, 4> {
        AMP_RST_FB_FSM_W::new(self)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    #[must_use]
    pub fn amp_short_ref_fsm(&mut self) -> AMP_SHORT_REF_FSM_W<SAR_MEAS_CTRL_SPEC, 8> {
        AMP_SHORT_REF_FSM_W::new(self)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    #[must_use]
    pub fn amp_short_ref_gnd_fsm(&mut self) -> AMP_SHORT_REF_GND_FSM_W<SAR_MEAS_CTRL_SPEC, 12> {
        AMP_SHORT_REF_GND_FSM_W::new(self)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    #[must_use]
    pub fn xpd_sar_fsm(&mut self) -> XPD_SAR_FSM_W<SAR_MEAS_CTRL_SPEC, 16> {
        XPD_SAR_FSM_W::new(self)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    #[must_use]
    pub fn sar_rstb_fsm(&mut self) -> SAR_RSTB_FSM_W<SAR_MEAS_CTRL_SPEC, 20> {
        SAR_RSTB_FSM_W::new(self)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    #[must_use]
    pub fn sar2_xpd_wait(&mut self) -> SAR2_XPD_WAIT_W<SAR_MEAS_CTRL_SPEC, 24> {
        SAR2_XPD_WAIT_W::new(self)
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
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_meas_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_meas_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAR_MEAS_CTRL_SPEC;
impl crate::RegisterSpec for SAR_MEAS_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar_meas_ctrl::R`](R) reader structure"]
impl crate::Readable for SAR_MEAS_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sar_meas_ctrl::W`](W) writer structure"]
impl crate::Writable for SAR_MEAS_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SAR_MEAS_CTRL to value 0x0707_338f"]
impl crate::Resettable for SAR_MEAS_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0707_338f;
}
