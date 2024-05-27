///Register `SAR_MEAS1_CTRL1` reader
pub type R = crate::R<SAR_MEAS1_CTRL1_SPEC>;
///Register `SAR_MEAS1_CTRL1` writer
pub type W = crate::W<SAR_MEAS1_CTRL1_SPEC>;
///Field `RTC_SARADC_RESET` reader - SAR ADC software reset.
pub type RTC_SARADC_RESET_R = crate::BitReader;
///Field `RTC_SARADC_RESET` writer - SAR ADC software reset.
pub type RTC_SARADC_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RTC_SARADC_CLKGATE_EN` reader - Enable bit of SAR ADC clock gate.
pub type RTC_SARADC_CLKGATE_EN_R = crate::BitReader;
///Field `RTC_SARADC_CLKGATE_EN` writer - Enable bit of SAR ADC clock gate.
pub type RTC_SARADC_CLKGATE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FORCE_XPD_AMP` reader -
pub type FORCE_XPD_AMP_R = crate::FieldReader;
///Field `FORCE_XPD_AMP` writer -
pub type FORCE_XPD_AMP_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `AMP_RST_FB_FORCE` reader -
pub type AMP_RST_FB_FORCE_R = crate::FieldReader;
///Field `AMP_RST_FB_FORCE` writer -
pub type AMP_RST_FB_FORCE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `AMP_SHORT_REF_FORCE` reader -
pub type AMP_SHORT_REF_FORCE_R = crate::FieldReader;
///Field `AMP_SHORT_REF_FORCE` writer -
pub type AMP_SHORT_REF_FORCE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `AMP_SHORT_REF_GND_FORCE` reader -
pub type AMP_SHORT_REF_GND_FORCE_R = crate::FieldReader;
///Field `AMP_SHORT_REF_GND_FORCE` writer -
pub type AMP_SHORT_REF_GND_FORCE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bit 22 - SAR ADC software reset.
    #[inline(always)]
    pub fn rtc_saradc_reset(&self) -> RTC_SARADC_RESET_R {
        RTC_SARADC_RESET_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Enable bit of SAR ADC clock gate.
    #[inline(always)]
    pub fn rtc_saradc_clkgate_en(&self) -> RTC_SARADC_CLKGATE_EN_R {
        RTC_SARADC_CLKGATE_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bits 24:25
    #[inline(always)]
    pub fn force_xpd_amp(&self) -> FORCE_XPD_AMP_R {
        FORCE_XPD_AMP_R::new(((self.bits >> 24) & 3) as u8)
    }
    ///Bits 26:27
    #[inline(always)]
    pub fn amp_rst_fb_force(&self) -> AMP_RST_FB_FORCE_R {
        AMP_RST_FB_FORCE_R::new(((self.bits >> 26) & 3) as u8)
    }
    ///Bits 28:29
    #[inline(always)]
    pub fn amp_short_ref_force(&self) -> AMP_SHORT_REF_FORCE_R {
        AMP_SHORT_REF_FORCE_R::new(((self.bits >> 28) & 3) as u8)
    }
    ///Bits 30:31
    #[inline(always)]
    pub fn amp_short_ref_gnd_force(&self) -> AMP_SHORT_REF_GND_FORCE_R {
        AMP_SHORT_REF_GND_FORCE_R::new(((self.bits >> 30) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_MEAS1_CTRL1")
            .field("rtc_saradc_reset", &self.rtc_saradc_reset())
            .field("rtc_saradc_clkgate_en", &self.rtc_saradc_clkgate_en())
            .field("force_xpd_amp", &self.force_xpd_amp())
            .field("amp_rst_fb_force", &self.amp_rst_fb_force())
            .field("amp_short_ref_force", &self.amp_short_ref_force())
            .field("amp_short_ref_gnd_force", &self.amp_short_ref_gnd_force())
            .finish()
    }
}
impl W {
    ///Bit 22 - SAR ADC software reset.
    #[inline(always)]
    #[must_use]
    pub fn rtc_saradc_reset(&mut self) -> RTC_SARADC_RESET_W<SAR_MEAS1_CTRL1_SPEC> {
        RTC_SARADC_RESET_W::new(self, 22)
    }
    ///Bit 23 - Enable bit of SAR ADC clock gate.
    #[inline(always)]
    #[must_use]
    pub fn rtc_saradc_clkgate_en(&mut self) -> RTC_SARADC_CLKGATE_EN_W<SAR_MEAS1_CTRL1_SPEC> {
        RTC_SARADC_CLKGATE_EN_W::new(self, 23)
    }
    ///Bits 24:25
    #[inline(always)]
    #[must_use]
    pub fn force_xpd_amp(&mut self) -> FORCE_XPD_AMP_W<SAR_MEAS1_CTRL1_SPEC> {
        FORCE_XPD_AMP_W::new(self, 24)
    }
    ///Bits 26:27
    #[inline(always)]
    #[must_use]
    pub fn amp_rst_fb_force(&mut self) -> AMP_RST_FB_FORCE_W<SAR_MEAS1_CTRL1_SPEC> {
        AMP_RST_FB_FORCE_W::new(self, 26)
    }
    ///Bits 28:29
    #[inline(always)]
    #[must_use]
    pub fn amp_short_ref_force(&mut self) -> AMP_SHORT_REF_FORCE_W<SAR_MEAS1_CTRL1_SPEC> {
        AMP_SHORT_REF_FORCE_W::new(self, 28)
    }
    ///Bits 30:31
    #[inline(always)]
    #[must_use]
    pub fn amp_short_ref_gnd_force(&mut self) -> AMP_SHORT_REF_GND_FORCE_W<SAR_MEAS1_CTRL1_SPEC> {
        AMP_SHORT_REF_GND_FORCE_W::new(self, 30)
    }
}
/**Configure RTC ADC1 controller

You can [`read`](crate::generic::Reg::read) this register and get [`sar_meas1_ctrl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_meas1_ctrl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SAR_MEAS1_CTRL1_SPEC;
impl crate::RegisterSpec for SAR_MEAS1_CTRL1_SPEC {
    type Ux = u32;
}
///`read()` method returns [`sar_meas1_ctrl1::R`](R) reader structure
impl crate::Readable for SAR_MEAS1_CTRL1_SPEC {}
///`write(|w| ..)` method takes [`sar_meas1_ctrl1::W`](W) writer structure
impl crate::Writable for SAR_MEAS1_CTRL1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SAR_MEAS1_CTRL1 to value 0
impl crate::Resettable for SAR_MEAS1_CTRL1_SPEC {
    const RESET_VALUE: u32 = 0;
}
