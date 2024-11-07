#[doc = "Register `AMP_CTRL2` reader"]
pub type R = crate::R<AMP_CTRL2_SPEC>;
#[doc = "Register `AMP_CTRL2` writer"]
pub type W = crate::W<AMP_CTRL2_SPEC>;
#[doc = "Field `SAR1_DAC_XPD_FSM_IDLE` reader - N/A"]
pub type SAR1_DAC_XPD_FSM_IDLE_R = crate::BitReader;
#[doc = "Field `SAR1_DAC_XPD_FSM_IDLE` writer - N/A"]
pub type SAR1_DAC_XPD_FSM_IDLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XPD_SAR_AMP_FSM_IDLE` reader - N/A"]
pub type XPD_SAR_AMP_FSM_IDLE_R = crate::BitReader;
#[doc = "Field `XPD_SAR_AMP_FSM_IDLE` writer - N/A"]
pub type XPD_SAR_AMP_FSM_IDLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AMP_RST_FB_FSM_IDLE` reader - N/A"]
pub type AMP_RST_FB_FSM_IDLE_R = crate::BitReader;
#[doc = "Field `AMP_RST_FB_FSM_IDLE` writer - N/A"]
pub type AMP_RST_FB_FSM_IDLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AMP_SHORT_REF_FSM_IDLE` reader - N/A"]
pub type AMP_SHORT_REF_FSM_IDLE_R = crate::BitReader;
#[doc = "Field `AMP_SHORT_REF_FSM_IDLE` writer - N/A"]
pub type AMP_SHORT_REF_FSM_IDLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AMP_SHORT_REF_GND_FSM_IDLE` reader - N/A"]
pub type AMP_SHORT_REF_GND_FSM_IDLE_R = crate::BitReader;
#[doc = "Field `AMP_SHORT_REF_GND_FSM_IDLE` writer - N/A"]
pub type AMP_SHORT_REF_GND_FSM_IDLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XPD_SAR_FSM_IDLE` reader - N/A"]
pub type XPD_SAR_FSM_IDLE_R = crate::BitReader;
#[doc = "Field `XPD_SAR_FSM_IDLE` writer - N/A"]
pub type XPD_SAR_FSM_IDLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAR_RSTB_FSM_IDLE` reader - N/A"]
pub type SAR_RSTB_FSM_IDLE_R = crate::BitReader;
#[doc = "Field `SAR_RSTB_FSM_IDLE` writer - N/A"]
pub type SAR_RSTB_FSM_IDLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAR_AMP_WAIT3` reader - N/A"]
pub type SAR_AMP_WAIT3_R = crate::FieldReader<u16>;
#[doc = "Field `SAR_AMP_WAIT3` writer - N/A"]
pub type SAR_AMP_WAIT3_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - N/A"]
    #[inline(always)]
    pub fn sar1_dac_xpd_fsm_idle(&self) -> SAR1_DAC_XPD_FSM_IDLE_R {
        SAR1_DAC_XPD_FSM_IDLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - N/A"]
    #[inline(always)]
    pub fn xpd_sar_amp_fsm_idle(&self) -> XPD_SAR_AMP_FSM_IDLE_R {
        XPD_SAR_AMP_FSM_IDLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - N/A"]
    #[inline(always)]
    pub fn amp_rst_fb_fsm_idle(&self) -> AMP_RST_FB_FSM_IDLE_R {
        AMP_RST_FB_FSM_IDLE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - N/A"]
    #[inline(always)]
    pub fn amp_short_ref_fsm_idle(&self) -> AMP_SHORT_REF_FSM_IDLE_R {
        AMP_SHORT_REF_FSM_IDLE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - N/A"]
    #[inline(always)]
    pub fn amp_short_ref_gnd_fsm_idle(&self) -> AMP_SHORT_REF_GND_FSM_IDLE_R {
        AMP_SHORT_REF_GND_FSM_IDLE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - N/A"]
    #[inline(always)]
    pub fn xpd_sar_fsm_idle(&self) -> XPD_SAR_FSM_IDLE_R {
        XPD_SAR_FSM_IDLE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - N/A"]
    #[inline(always)]
    pub fn sar_rstb_fsm_idle(&self) -> SAR_RSTB_FSM_IDLE_R {
        SAR_RSTB_FSM_IDLE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 16:31 - N/A"]
    #[inline(always)]
    pub fn sar_amp_wait3(&self) -> SAR_AMP_WAIT3_R {
        SAR_AMP_WAIT3_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AMP_CTRL2")
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
    #[doc = "Bit 0 - N/A"]
    #[inline(always)]
    pub fn sar1_dac_xpd_fsm_idle(&mut self) -> SAR1_DAC_XPD_FSM_IDLE_W<AMP_CTRL2_SPEC> {
        SAR1_DAC_XPD_FSM_IDLE_W::new(self, 0)
    }
    #[doc = "Bit 1 - N/A"]
    #[inline(always)]
    pub fn xpd_sar_amp_fsm_idle(&mut self) -> XPD_SAR_AMP_FSM_IDLE_W<AMP_CTRL2_SPEC> {
        XPD_SAR_AMP_FSM_IDLE_W::new(self, 1)
    }
    #[doc = "Bit 2 - N/A"]
    #[inline(always)]
    pub fn amp_rst_fb_fsm_idle(&mut self) -> AMP_RST_FB_FSM_IDLE_W<AMP_CTRL2_SPEC> {
        AMP_RST_FB_FSM_IDLE_W::new(self, 2)
    }
    #[doc = "Bit 3 - N/A"]
    #[inline(always)]
    pub fn amp_short_ref_fsm_idle(&mut self) -> AMP_SHORT_REF_FSM_IDLE_W<AMP_CTRL2_SPEC> {
        AMP_SHORT_REF_FSM_IDLE_W::new(self, 3)
    }
    #[doc = "Bit 4 - N/A"]
    #[inline(always)]
    pub fn amp_short_ref_gnd_fsm_idle(&mut self) -> AMP_SHORT_REF_GND_FSM_IDLE_W<AMP_CTRL2_SPEC> {
        AMP_SHORT_REF_GND_FSM_IDLE_W::new(self, 4)
    }
    #[doc = "Bit 5 - N/A"]
    #[inline(always)]
    pub fn xpd_sar_fsm_idle(&mut self) -> XPD_SAR_FSM_IDLE_W<AMP_CTRL2_SPEC> {
        XPD_SAR_FSM_IDLE_W::new(self, 5)
    }
    #[doc = "Bit 6 - N/A"]
    #[inline(always)]
    pub fn sar_rstb_fsm_idle(&mut self) -> SAR_RSTB_FSM_IDLE_W<AMP_CTRL2_SPEC> {
        SAR_RSTB_FSM_IDLE_W::new(self, 6)
    }
    #[doc = "Bits 16:31 - N/A"]
    #[inline(always)]
    pub fn sar_amp_wait3(&mut self) -> SAR_AMP_WAIT3_W<AMP_CTRL2_SPEC> {
        SAR_AMP_WAIT3_W::new(self, 16)
    }
}
#[doc = "N/A\n\nYou can [`read`](crate::Reg::read) this register and get [`amp_ctrl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`amp_ctrl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AMP_CTRL2_SPEC;
impl crate::RegisterSpec for AMP_CTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`amp_ctrl2::R`](R) reader structure"]
impl crate::Readable for AMP_CTRL2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`amp_ctrl2::W`](W) writer structure"]
impl crate::Writable for AMP_CTRL2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AMP_CTRL2 to value 0x000a_0000"]
impl crate::Resettable for AMP_CTRL2_SPEC {
    const RESET_VALUE: u32 = 0x000a_0000;
}
