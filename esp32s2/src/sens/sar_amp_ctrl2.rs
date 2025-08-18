#[doc = "Register `SAR_AMP_CTRL2` reader"]
pub type R = crate::R<SAR_AMP_CTRL2_SPEC>;
#[doc = "Register `SAR_AMP_CTRL2` writer"]
pub type W = crate::W<SAR_AMP_CTRL2_SPEC>;
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
#[doc = "Field `SAR_AMP_WAIT3` reader - "]
pub type SAR_AMP_WAIT3_R = crate::FieldReader<u16>;
#[doc = "Field `SAR_AMP_WAIT3` writer - "]
pub type SAR_AMP_WAIT3_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sar1_dac_xpd_fsm_idle(&self) -> SAR1_DAC_XPD_FSM_IDLE_R {
        SAR1_DAC_XPD_FSM_IDLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn xpd_sar_amp_fsm_idle(&self) -> XPD_SAR_AMP_FSM_IDLE_R {
        XPD_SAR_AMP_FSM_IDLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn amp_rst_fb_fsm_idle(&self) -> AMP_RST_FB_FSM_IDLE_R {
        AMP_RST_FB_FSM_IDLE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn amp_short_ref_fsm_idle(&self) -> AMP_SHORT_REF_FSM_IDLE_R {
        AMP_SHORT_REF_FSM_IDLE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn amp_short_ref_gnd_fsm_idle(&self) -> AMP_SHORT_REF_GND_FSM_IDLE_R {
        AMP_SHORT_REF_GND_FSM_IDLE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn xpd_sar_fsm_idle(&self) -> XPD_SAR_FSM_IDLE_R {
        XPD_SAR_FSM_IDLE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn sar_rstb_fsm_idle(&self) -> SAR_RSTB_FSM_IDLE_R {
        SAR_RSTB_FSM_IDLE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn sar_amp_wait3(&self) -> SAR_AMP_WAIT3_R {
        SAR_AMP_WAIT3_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_AMP_CTRL2")
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
            .field("sar_amp_wait3", &self.sar_amp_wait3())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sar1_dac_xpd_fsm_idle(&mut self) -> SAR1_DAC_XPD_FSM_IDLE_W<'_, SAR_AMP_CTRL2_SPEC> {
        SAR1_DAC_XPD_FSM_IDLE_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn xpd_sar_amp_fsm_idle(&mut self) -> XPD_SAR_AMP_FSM_IDLE_W<'_, SAR_AMP_CTRL2_SPEC> {
        XPD_SAR_AMP_FSM_IDLE_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn amp_rst_fb_fsm_idle(&mut self) -> AMP_RST_FB_FSM_IDLE_W<'_, SAR_AMP_CTRL2_SPEC> {
        AMP_RST_FB_FSM_IDLE_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn amp_short_ref_fsm_idle(&mut self) -> AMP_SHORT_REF_FSM_IDLE_W<'_, SAR_AMP_CTRL2_SPEC> {
        AMP_SHORT_REF_FSM_IDLE_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn amp_short_ref_gnd_fsm_idle(
        &mut self,
    ) -> AMP_SHORT_REF_GND_FSM_IDLE_W<'_, SAR_AMP_CTRL2_SPEC> {
        AMP_SHORT_REF_GND_FSM_IDLE_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn xpd_sar_fsm_idle(&mut self) -> XPD_SAR_FSM_IDLE_W<'_, SAR_AMP_CTRL2_SPEC> {
        XPD_SAR_FSM_IDLE_W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn sar_rstb_fsm_idle(&mut self) -> SAR_RSTB_FSM_IDLE_W<'_, SAR_AMP_CTRL2_SPEC> {
        SAR_RSTB_FSM_IDLE_W::new(self, 6)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn sar_amp_wait3(&mut self) -> SAR_AMP_WAIT3_W<'_, SAR_AMP_CTRL2_SPEC> {
        SAR_AMP_WAIT3_W::new(self, 16)
    }
}
#[doc = "AMP control\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_amp_ctrl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_amp_ctrl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
