#[doc = "Register `SAR_MEAS_CTRL` reader"]
pub type R = crate::R<SAR_MEAS_CTRL_SPEC>;
#[doc = "Register `SAR_MEAS_CTRL` writer"]
pub type W = crate::W<SAR_MEAS_CTRL_SPEC>;
#[doc = "Field `XPD_SAR_AMP_FSM` reader - "]
pub type XPD_SAR_AMP_FSM_R = crate::FieldReader;
#[doc = "Field `XPD_SAR_AMP_FSM` writer - "]
pub type XPD_SAR_AMP_FSM_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AMP_RST_FB_FSM` reader - "]
pub type AMP_RST_FB_FSM_R = crate::FieldReader;
#[doc = "Field `AMP_RST_FB_FSM` writer - "]
pub type AMP_RST_FB_FSM_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AMP_SHORT_REF_FSM` reader - "]
pub type AMP_SHORT_REF_FSM_R = crate::FieldReader;
#[doc = "Field `AMP_SHORT_REF_FSM` writer - "]
pub type AMP_SHORT_REF_FSM_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AMP_SHORT_REF_GND_FSM` reader - "]
pub type AMP_SHORT_REF_GND_FSM_R = crate::FieldReader;
#[doc = "Field `AMP_SHORT_REF_GND_FSM` writer - "]
pub type AMP_SHORT_REF_GND_FSM_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `XPD_SAR_FSM` reader - "]
pub type XPD_SAR_FSM_R = crate::FieldReader;
#[doc = "Field `XPD_SAR_FSM` writer - "]
pub type XPD_SAR_FSM_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SAR_RSTB_FSM` reader - "]
pub type SAR_RSTB_FSM_R = crate::FieldReader;
#[doc = "Field `SAR_RSTB_FSM` writer - "]
pub type SAR_RSTB_FSM_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SAR2_XPD_WAIT` reader - "]
pub type SAR2_XPD_WAIT_R = crate::FieldReader;
#[doc = "Field `SAR2_XPD_WAIT` writer - "]
pub type SAR2_XPD_WAIT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
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
            .field("xpd_sar_amp_fsm", &self.xpd_sar_amp_fsm())
            .field("amp_rst_fb_fsm", &self.amp_rst_fb_fsm())
            .field("amp_short_ref_fsm", &self.amp_short_ref_fsm())
            .field("amp_short_ref_gnd_fsm", &self.amp_short_ref_gnd_fsm())
            .field("xpd_sar_fsm", &self.xpd_sar_fsm())
            .field("sar_rstb_fsm", &self.sar_rstb_fsm())
            .field("sar2_xpd_wait", &self.sar2_xpd_wait())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn xpd_sar_amp_fsm(&mut self) -> XPD_SAR_AMP_FSM_W<SAR_MEAS_CTRL_SPEC> {
        XPD_SAR_AMP_FSM_W::new(self, 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn amp_rst_fb_fsm(&mut self) -> AMP_RST_FB_FSM_W<SAR_MEAS_CTRL_SPEC> {
        AMP_RST_FB_FSM_W::new(self, 4)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn amp_short_ref_fsm(&mut self) -> AMP_SHORT_REF_FSM_W<SAR_MEAS_CTRL_SPEC> {
        AMP_SHORT_REF_FSM_W::new(self, 8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn amp_short_ref_gnd_fsm(&mut self) -> AMP_SHORT_REF_GND_FSM_W<SAR_MEAS_CTRL_SPEC> {
        AMP_SHORT_REF_GND_FSM_W::new(self, 12)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn xpd_sar_fsm(&mut self) -> XPD_SAR_FSM_W<SAR_MEAS_CTRL_SPEC> {
        XPD_SAR_FSM_W::new(self, 16)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn sar_rstb_fsm(&mut self) -> SAR_RSTB_FSM_W<SAR_MEAS_CTRL_SPEC> {
        SAR_RSTB_FSM_W::new(self, 20)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn sar2_xpd_wait(&mut self) -> SAR2_XPD_WAIT_W<SAR_MEAS_CTRL_SPEC> {
        SAR2_XPD_WAIT_W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_meas_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_meas_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAR_MEAS_CTRL_SPEC;
impl crate::RegisterSpec for SAR_MEAS_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar_meas_ctrl::R`](R) reader structure"]
impl crate::Readable for SAR_MEAS_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sar_meas_ctrl::W`](W) writer structure"]
impl crate::Writable for SAR_MEAS_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SAR_MEAS_CTRL to value 0x0707_338f"]
impl crate::Resettable for SAR_MEAS_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x0707_338f;
}
