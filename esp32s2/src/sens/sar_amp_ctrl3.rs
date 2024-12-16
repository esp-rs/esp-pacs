#[doc = "Register `SAR_AMP_CTRL3` reader"]
pub type R = crate::R<SAR_AMP_CTRL3_SPEC>;
#[doc = "Register `SAR_AMP_CTRL3` writer"]
pub type W = crate::W<SAR_AMP_CTRL3_SPEC>;
#[doc = "Field `SAR1_DAC_XPD_FSM` reader - Control of DAC. 4’b0010: disable DAC. 4’b0000: power up DAC by FSM. 4’b0011: power up DAC by software."]
pub type SAR1_DAC_XPD_FSM_R = crate::FieldReader;
#[doc = "Field `SAR1_DAC_XPD_FSM` writer - Control of DAC. 4’b0010: disable DAC. 4’b0000: power up DAC by FSM. 4’b0011: power up DAC by software."]
pub type SAR1_DAC_XPD_FSM_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
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
impl R {
    #[doc = "Bits 0:3 - Control of DAC. 4’b0010: disable DAC. 4’b0000: power up DAC by FSM. 4’b0011: power up DAC by software."]
    #[inline(always)]
    pub fn sar1_dac_xpd_fsm(&self) -> SAR1_DAC_XPD_FSM_R {
        SAR1_DAC_XPD_FSM_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn xpd_sar_amp_fsm(&self) -> XPD_SAR_AMP_FSM_R {
        XPD_SAR_AMP_FSM_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn amp_rst_fb_fsm(&self) -> AMP_RST_FB_FSM_R {
        AMP_RST_FB_FSM_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn amp_short_ref_fsm(&self) -> AMP_SHORT_REF_FSM_R {
        AMP_SHORT_REF_FSM_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn amp_short_ref_gnd_fsm(&self) -> AMP_SHORT_REF_GND_FSM_R {
        AMP_SHORT_REF_GND_FSM_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn xpd_sar_fsm(&self) -> XPD_SAR_FSM_R {
        XPD_SAR_FSM_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn sar_rstb_fsm(&self) -> SAR_RSTB_FSM_R {
        SAR_RSTB_FSM_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_AMP_CTRL3")
            .field("sar1_dac_xpd_fsm", &self.sar1_dac_xpd_fsm())
            .field("xpd_sar_amp_fsm", &self.xpd_sar_amp_fsm())
            .field("amp_rst_fb_fsm", &self.amp_rst_fb_fsm())
            .field("amp_short_ref_fsm", &self.amp_short_ref_fsm())
            .field("amp_short_ref_gnd_fsm", &self.amp_short_ref_gnd_fsm())
            .field("xpd_sar_fsm", &self.xpd_sar_fsm())
            .field("sar_rstb_fsm", &self.sar_rstb_fsm())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - Control of DAC. 4’b0010: disable DAC. 4’b0000: power up DAC by FSM. 4’b0011: power up DAC by software."]
    #[inline(always)]
    pub fn sar1_dac_xpd_fsm(&mut self) -> SAR1_DAC_XPD_FSM_W<SAR_AMP_CTRL3_SPEC> {
        SAR1_DAC_XPD_FSM_W::new(self, 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn xpd_sar_amp_fsm(&mut self) -> XPD_SAR_AMP_FSM_W<SAR_AMP_CTRL3_SPEC> {
        XPD_SAR_AMP_FSM_W::new(self, 4)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn amp_rst_fb_fsm(&mut self) -> AMP_RST_FB_FSM_W<SAR_AMP_CTRL3_SPEC> {
        AMP_RST_FB_FSM_W::new(self, 8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn amp_short_ref_fsm(&mut self) -> AMP_SHORT_REF_FSM_W<SAR_AMP_CTRL3_SPEC> {
        AMP_SHORT_REF_FSM_W::new(self, 12)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn amp_short_ref_gnd_fsm(&mut self) -> AMP_SHORT_REF_GND_FSM_W<SAR_AMP_CTRL3_SPEC> {
        AMP_SHORT_REF_GND_FSM_W::new(self, 16)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn xpd_sar_fsm(&mut self) -> XPD_SAR_FSM_W<SAR_AMP_CTRL3_SPEC> {
        XPD_SAR_FSM_W::new(self, 20)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn sar_rstb_fsm(&mut self) -> SAR_RSTB_FSM_W<SAR_AMP_CTRL3_SPEC> {
        SAR_RSTB_FSM_W::new(self, 24)
    }
}
#[doc = "AMP control register\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_amp_ctrl3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_amp_ctrl3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAR_AMP_CTRL3_SPEC;
impl crate::RegisterSpec for SAR_AMP_CTRL3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar_amp_ctrl3::R`](R) reader structure"]
impl crate::Readable for SAR_AMP_CTRL3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sar_amp_ctrl3::W`](W) writer structure"]
impl crate::Writable for SAR_AMP_CTRL3_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SAR_AMP_CTRL3 to value 0x0073_38f3"]
impl crate::Resettable for SAR_AMP_CTRL3_SPEC {
    const RESET_VALUE: u32 = 0x0073_38f3;
}
