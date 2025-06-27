#[doc = "Register `SAR_MEAS_CTRL2` reader"]
pub type R = crate::R<SAR_MEAS_CTRL2_SPEC>;
#[doc = "Register `SAR_MEAS_CTRL2` writer"]
pub type W = crate::W<SAR_MEAS_CTRL2_SPEC>;
#[doc = "Field `SAR1_DAC_XPD_FSM` reader - "]
pub type SAR1_DAC_XPD_FSM_R = crate::FieldReader;
#[doc = "Field `SAR1_DAC_XPD_FSM` writer - "]
pub type SAR1_DAC_XPD_FSM_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SAR1_DAC_XPD_FSM_IDLE` reader - "]
pub type SAR1_DAC_XPD_FSM_IDLE_R = crate::BitReader;
#[doc = "Field `SAR1_DAC_XPD_FSM_IDLE` writer - "]
pub type SAR1_DAC_XPD_FSM_IDLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XPD_SAR_AMP_FSM_IDLE` reader - "]
pub type XPD_SAR_AMP_FSM_IDLE_R = crate::BitReader;
#[doc = "Field `XPD_SAR_AMP_FSM_IDLE` writer - "]
pub type XPD_SAR_AMP_FSM_IDLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AMP_RST_FB_FSM_IDLE` reader - "]
pub type AMP_RST_FB_FSM_IDLE_R = crate::BitReader;
#[doc = "Field `AMP_RST_FB_FSM_IDLE` writer - "]
pub type AMP_RST_FB_FSM_IDLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AMP_SHORT_REF_FSM_IDLE` reader - "]
pub type AMP_SHORT_REF_FSM_IDLE_R = crate::BitReader;
#[doc = "Field `AMP_SHORT_REF_FSM_IDLE` writer - "]
pub type AMP_SHORT_REF_FSM_IDLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AMP_SHORT_REF_GND_FSM_IDLE` reader - "]
pub type AMP_SHORT_REF_GND_FSM_IDLE_R = crate::BitReader;
#[doc = "Field `AMP_SHORT_REF_GND_FSM_IDLE` writer - "]
pub type AMP_SHORT_REF_GND_FSM_IDLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XPD_SAR_FSM_IDLE` reader - "]
pub type XPD_SAR_FSM_IDLE_R = crate::BitReader;
#[doc = "Field `XPD_SAR_FSM_IDLE` writer - "]
pub type XPD_SAR_FSM_IDLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAR_RSTB_FSM_IDLE` reader - "]
pub type SAR_RSTB_FSM_IDLE_R = crate::BitReader;
#[doc = "Field `SAR_RSTB_FSM_IDLE` writer - "]
pub type SAR_RSTB_FSM_IDLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAR2_RSTB_FORCE` reader - "]
pub type SAR2_RSTB_FORCE_R = crate::FieldReader;
#[doc = "Field `SAR2_RSTB_FORCE` writer - "]
pub type SAR2_RSTB_FORCE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `AMP_RST_FB_FORCE` reader - "]
pub type AMP_RST_FB_FORCE_R = crate::FieldReader;
#[doc = "Field `AMP_RST_FB_FORCE` writer - "]
pub type AMP_RST_FB_FORCE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `AMP_SHORT_REF_FORCE` reader - "]
pub type AMP_SHORT_REF_FORCE_R = crate::FieldReader;
#[doc = "Field `AMP_SHORT_REF_FORCE` writer - "]
pub type AMP_SHORT_REF_FORCE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `AMP_SHORT_REF_GND_FORCE` reader - "]
pub type AMP_SHORT_REF_GND_FORCE_R = crate::FieldReader;
#[doc = "Field `AMP_SHORT_REF_GND_FORCE` writer - "]
pub type AMP_SHORT_REF_GND_FORCE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn sar1_dac_xpd_fsm(&self) -> SAR1_DAC_XPD_FSM_R {
        SAR1_DAC_XPD_FSM_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn sar1_dac_xpd_fsm_idle(&self) -> SAR1_DAC_XPD_FSM_IDLE_R {
        SAR1_DAC_XPD_FSM_IDLE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn xpd_sar_amp_fsm_idle(&self) -> XPD_SAR_AMP_FSM_IDLE_R {
        XPD_SAR_AMP_FSM_IDLE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn amp_rst_fb_fsm_idle(&self) -> AMP_RST_FB_FSM_IDLE_R {
        AMP_RST_FB_FSM_IDLE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn amp_short_ref_fsm_idle(&self) -> AMP_SHORT_REF_FSM_IDLE_R {
        AMP_SHORT_REF_FSM_IDLE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn amp_short_ref_gnd_fsm_idle(&self) -> AMP_SHORT_REF_GND_FSM_IDLE_R {
        AMP_SHORT_REF_GND_FSM_IDLE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn xpd_sar_fsm_idle(&self) -> XPD_SAR_FSM_IDLE_R {
        XPD_SAR_FSM_IDLE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn sar_rstb_fsm_idle(&self) -> SAR_RSTB_FSM_IDLE_R {
        SAR_RSTB_FSM_IDLE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:12"]
    #[inline(always)]
    pub fn sar2_rstb_force(&self) -> SAR2_RSTB_FORCE_R {
        SAR2_RSTB_FORCE_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bits 13:14"]
    #[inline(always)]
    pub fn amp_rst_fb_force(&self) -> AMP_RST_FB_FORCE_R {
        AMP_RST_FB_FORCE_R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bits 15:16"]
    #[inline(always)]
    pub fn amp_short_ref_force(&self) -> AMP_SHORT_REF_FORCE_R {
        AMP_SHORT_REF_FORCE_R::new(((self.bits >> 15) & 3) as u8)
    }
    #[doc = "Bits 17:18"]
    #[inline(always)]
    pub fn amp_short_ref_gnd_force(&self) -> AMP_SHORT_REF_GND_FORCE_R {
        AMP_SHORT_REF_GND_FORCE_R::new(((self.bits >> 17) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_MEAS_CTRL2")
            .field("sar1_dac_xpd_fsm", &self.sar1_dac_xpd_fsm())
            .field("sar1_dac_xpd_fsm_idle", &self.sar1_dac_xpd_fsm_idle())
            .field("xpd_sar_amp_fsm_idle", &self.xpd_sar_amp_fsm_idle())
            .field("amp_rst_fb_fsm_idle", &self.amp_rst_fb_fsm_idle())
            .field("amp_short_ref_fsm_idle", &self.amp_short_ref_fsm_idle())
            .field(
                "amp_short_ref_gnd_fsm_idle",
                &self.amp_short_ref_gnd_fsm_idle(),
            )
            .field("xpd_sar_fsm_idle", &self.xpd_sar_fsm_idle())
            .field("sar_rstb_fsm_idle", &self.sar_rstb_fsm_idle())
            .field("sar2_rstb_force", &self.sar2_rstb_force())
            .field("amp_rst_fb_force", &self.amp_rst_fb_force())
            .field("amp_short_ref_force", &self.amp_short_ref_force())
            .field("amp_short_ref_gnd_force", &self.amp_short_ref_gnd_force())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn sar1_dac_xpd_fsm(&mut self) -> SAR1_DAC_XPD_FSM_W<SAR_MEAS_CTRL2_SPEC> {
        SAR1_DAC_XPD_FSM_W::new(self, 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn sar1_dac_xpd_fsm_idle(&mut self) -> SAR1_DAC_XPD_FSM_IDLE_W<SAR_MEAS_CTRL2_SPEC> {
        SAR1_DAC_XPD_FSM_IDLE_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn xpd_sar_amp_fsm_idle(&mut self) -> XPD_SAR_AMP_FSM_IDLE_W<SAR_MEAS_CTRL2_SPEC> {
        XPD_SAR_AMP_FSM_IDLE_W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn amp_rst_fb_fsm_idle(&mut self) -> AMP_RST_FB_FSM_IDLE_W<SAR_MEAS_CTRL2_SPEC> {
        AMP_RST_FB_FSM_IDLE_W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn amp_short_ref_fsm_idle(&mut self) -> AMP_SHORT_REF_FSM_IDLE_W<SAR_MEAS_CTRL2_SPEC> {
        AMP_SHORT_REF_FSM_IDLE_W::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn amp_short_ref_gnd_fsm_idle(
        &mut self,
    ) -> AMP_SHORT_REF_GND_FSM_IDLE_W<SAR_MEAS_CTRL2_SPEC> {
        AMP_SHORT_REF_GND_FSM_IDLE_W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn xpd_sar_fsm_idle(&mut self) -> XPD_SAR_FSM_IDLE_W<SAR_MEAS_CTRL2_SPEC> {
        XPD_SAR_FSM_IDLE_W::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn sar_rstb_fsm_idle(&mut self) -> SAR_RSTB_FSM_IDLE_W<SAR_MEAS_CTRL2_SPEC> {
        SAR_RSTB_FSM_IDLE_W::new(self, 10)
    }
    #[doc = "Bits 11:12"]
    #[inline(always)]
    pub fn sar2_rstb_force(&mut self) -> SAR2_RSTB_FORCE_W<SAR_MEAS_CTRL2_SPEC> {
        SAR2_RSTB_FORCE_W::new(self, 11)
    }
    #[doc = "Bits 13:14"]
    #[inline(always)]
    pub fn amp_rst_fb_force(&mut self) -> AMP_RST_FB_FORCE_W<SAR_MEAS_CTRL2_SPEC> {
        AMP_RST_FB_FORCE_W::new(self, 13)
    }
    #[doc = "Bits 15:16"]
    #[inline(always)]
    pub fn amp_short_ref_force(&mut self) -> AMP_SHORT_REF_FORCE_W<SAR_MEAS_CTRL2_SPEC> {
        AMP_SHORT_REF_FORCE_W::new(self, 15)
    }
    #[doc = "Bits 17:18"]
    #[inline(always)]
    pub fn amp_short_ref_gnd_force(&mut self) -> AMP_SHORT_REF_GND_FORCE_W<SAR_MEAS_CTRL2_SPEC> {
        AMP_SHORT_REF_GND_FORCE_W::new(self, 17)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_meas_ctrl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_meas_ctrl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAR_MEAS_CTRL2_SPEC;
impl crate::RegisterSpec for SAR_MEAS_CTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar_meas_ctrl2::R`](R) reader structure"]
impl crate::Readable for SAR_MEAS_CTRL2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sar_meas_ctrl2::W`](W) writer structure"]
impl crate::Writable for SAR_MEAS_CTRL2_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SAR_MEAS_CTRL2 to value 0x03"]
impl crate::Resettable for SAR_MEAS_CTRL2_SPEC {
    const RESET_VALUE: u32 = 0x03;
}
