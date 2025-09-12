#[doc = "Register `SAR_AMP_CTRL2` reader"]
pub type R = crate::R<SAR_AMP_CTRL2_SPEC>;
#[doc = "Register `SAR_AMP_CTRL2` writer"]
pub type W = crate::W<SAR_AMP_CTRL2_SPEC>;
#[doc = "Field `SAR_SAR1_DAC_XPD_FSM_IDLE` reader - no public"]
pub type SAR_SAR1_DAC_XPD_FSM_IDLE_R = crate::BitReader;
#[doc = "Field `SAR_SAR1_DAC_XPD_FSM_IDLE` writer - no public"]
pub type SAR_SAR1_DAC_XPD_FSM_IDLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAR_XPD_SAR_AMP_FSM_IDLE` reader - no public"]
pub type SAR_XPD_SAR_AMP_FSM_IDLE_R = crate::BitReader;
#[doc = "Field `SAR_XPD_SAR_AMP_FSM_IDLE` writer - no public"]
pub type SAR_XPD_SAR_AMP_FSM_IDLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAR_AMP_RST_FB_FSM_IDLE` reader - no public"]
pub type SAR_AMP_RST_FB_FSM_IDLE_R = crate::BitReader;
#[doc = "Field `SAR_AMP_RST_FB_FSM_IDLE` writer - no public"]
pub type SAR_AMP_RST_FB_FSM_IDLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAR_AMP_SHORT_REF_FSM_IDLE` reader - no public"]
pub type SAR_AMP_SHORT_REF_FSM_IDLE_R = crate::BitReader;
#[doc = "Field `SAR_AMP_SHORT_REF_FSM_IDLE` writer - no public"]
pub type SAR_AMP_SHORT_REF_FSM_IDLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAR_AMP_SHORT_REF_GND_FSM_IDLE` reader - no public"]
pub type SAR_AMP_SHORT_REF_GND_FSM_IDLE_R = crate::BitReader;
#[doc = "Field `SAR_AMP_SHORT_REF_GND_FSM_IDLE` writer - no public"]
pub type SAR_AMP_SHORT_REF_GND_FSM_IDLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAR_XPD_SAR_FSM_IDLE` reader - no public"]
pub type SAR_XPD_SAR_FSM_IDLE_R = crate::BitReader;
#[doc = "Field `SAR_XPD_SAR_FSM_IDLE` writer - no public"]
pub type SAR_XPD_SAR_FSM_IDLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAR_RSTB_FSM_IDLE` reader - no public"]
pub type SAR_RSTB_FSM_IDLE_R = crate::BitReader;
#[doc = "Field `SAR_RSTB_FSM_IDLE` writer - no public"]
pub type SAR_RSTB_FSM_IDLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAR_AMP_WAIT3` reader - no public"]
pub type SAR_AMP_WAIT3_R = crate::FieldReader<u16>;
#[doc = "Field `SAR_AMP_WAIT3` writer - no public"]
pub type SAR_AMP_WAIT3_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - no public"]
    #[inline(always)]
    pub fn sar_sar1_dac_xpd_fsm_idle(&self) -> SAR_SAR1_DAC_XPD_FSM_IDLE_R {
        SAR_SAR1_DAC_XPD_FSM_IDLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - no public"]
    #[inline(always)]
    pub fn sar_xpd_sar_amp_fsm_idle(&self) -> SAR_XPD_SAR_AMP_FSM_IDLE_R {
        SAR_XPD_SAR_AMP_FSM_IDLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - no public"]
    #[inline(always)]
    pub fn sar_amp_rst_fb_fsm_idle(&self) -> SAR_AMP_RST_FB_FSM_IDLE_R {
        SAR_AMP_RST_FB_FSM_IDLE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - no public"]
    #[inline(always)]
    pub fn sar_amp_short_ref_fsm_idle(&self) -> SAR_AMP_SHORT_REF_FSM_IDLE_R {
        SAR_AMP_SHORT_REF_FSM_IDLE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - no public"]
    #[inline(always)]
    pub fn sar_amp_short_ref_gnd_fsm_idle(&self) -> SAR_AMP_SHORT_REF_GND_FSM_IDLE_R {
        SAR_AMP_SHORT_REF_GND_FSM_IDLE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - no public"]
    #[inline(always)]
    pub fn sar_xpd_sar_fsm_idle(&self) -> SAR_XPD_SAR_FSM_IDLE_R {
        SAR_XPD_SAR_FSM_IDLE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - no public"]
    #[inline(always)]
    pub fn sar_rstb_fsm_idle(&self) -> SAR_RSTB_FSM_IDLE_R {
        SAR_RSTB_FSM_IDLE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 16:31 - no public"]
    #[inline(always)]
    pub fn sar_amp_wait3(&self) -> SAR_AMP_WAIT3_R {
        SAR_AMP_WAIT3_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_AMP_CTRL2")
            .field(
                "sar_sar1_dac_xpd_fsm_idle",
                &self.sar_sar1_dac_xpd_fsm_idle(),
            )
            .field("sar_xpd_sar_amp_fsm_idle", &self.sar_xpd_sar_amp_fsm_idle())
            .field("sar_amp_rst_fb_fsm_idle", &self.sar_amp_rst_fb_fsm_idle())
            .field(
                "sar_amp_short_ref_fsm_idle",
                &self.sar_amp_short_ref_fsm_idle(),
            )
            .field(
                "sar_amp_short_ref_gnd_fsm_idle",
                &self.sar_amp_short_ref_gnd_fsm_idle(),
            )
            .field("sar_xpd_sar_fsm_idle", &self.sar_xpd_sar_fsm_idle())
            .field("sar_rstb_fsm_idle", &self.sar_rstb_fsm_idle())
            .field("sar_amp_wait3", &self.sar_amp_wait3())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - no public"]
    #[inline(always)]
    pub fn sar_sar1_dac_xpd_fsm_idle(
        &mut self,
    ) -> SAR_SAR1_DAC_XPD_FSM_IDLE_W<'_, SAR_AMP_CTRL2_SPEC> {
        SAR_SAR1_DAC_XPD_FSM_IDLE_W::new(self, 0)
    }
    #[doc = "Bit 1 - no public"]
    #[inline(always)]
    pub fn sar_xpd_sar_amp_fsm_idle(
        &mut self,
    ) -> SAR_XPD_SAR_AMP_FSM_IDLE_W<'_, SAR_AMP_CTRL2_SPEC> {
        SAR_XPD_SAR_AMP_FSM_IDLE_W::new(self, 1)
    }
    #[doc = "Bit 2 - no public"]
    #[inline(always)]
    pub fn sar_amp_rst_fb_fsm_idle(&mut self) -> SAR_AMP_RST_FB_FSM_IDLE_W<'_, SAR_AMP_CTRL2_SPEC> {
        SAR_AMP_RST_FB_FSM_IDLE_W::new(self, 2)
    }
    #[doc = "Bit 3 - no public"]
    #[inline(always)]
    pub fn sar_amp_short_ref_fsm_idle(
        &mut self,
    ) -> SAR_AMP_SHORT_REF_FSM_IDLE_W<'_, SAR_AMP_CTRL2_SPEC> {
        SAR_AMP_SHORT_REF_FSM_IDLE_W::new(self, 3)
    }
    #[doc = "Bit 4 - no public"]
    #[inline(always)]
    pub fn sar_amp_short_ref_gnd_fsm_idle(
        &mut self,
    ) -> SAR_AMP_SHORT_REF_GND_FSM_IDLE_W<'_, SAR_AMP_CTRL2_SPEC> {
        SAR_AMP_SHORT_REF_GND_FSM_IDLE_W::new(self, 4)
    }
    #[doc = "Bit 5 - no public"]
    #[inline(always)]
    pub fn sar_xpd_sar_fsm_idle(&mut self) -> SAR_XPD_SAR_FSM_IDLE_W<'_, SAR_AMP_CTRL2_SPEC> {
        SAR_XPD_SAR_FSM_IDLE_W::new(self, 5)
    }
    #[doc = "Bit 6 - no public"]
    #[inline(always)]
    pub fn sar_rstb_fsm_idle(&mut self) -> SAR_RSTB_FSM_IDLE_W<'_, SAR_AMP_CTRL2_SPEC> {
        SAR_RSTB_FSM_IDLE_W::new(self, 6)
    }
    #[doc = "Bits 16:31 - no public"]
    #[inline(always)]
    pub fn sar_amp_wait3(&mut self) -> SAR_AMP_WAIT3_W<'_, SAR_AMP_CTRL2_SPEC> {
        SAR_AMP_WAIT3_W::new(self, 16)
    }
}
#[doc = "no public\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_amp_ctrl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_amp_ctrl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAR_AMP_CTRL2_SPEC;
impl crate::RegisterSpec for SAR_AMP_CTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar_amp_ctrl2::R`](R) reader structure"]
impl crate::Readable for SAR_AMP_CTRL2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sar_amp_ctrl2::W`](W) writer structure"]
impl crate::Writable for SAR_AMP_CTRL2_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SAR_AMP_CTRL2 to value 0x000a_0000"]
impl crate::Resettable for SAR_AMP_CTRL2_SPEC {
    const RESET_VALUE: u32 = 0x000a_0000;
}
