#[doc = "Register `SAR_MEAS1_CTRL1` reader"]
pub type R = crate::R<SAR_MEAS1_CTRL1_SPEC>;
#[doc = "Register `SAR_MEAS1_CTRL1` writer"]
pub type W = crate::W<SAR_MEAS1_CTRL1_SPEC>;
#[doc = "Field `FORCE_XPD_AMP` reader - no public"]
pub type FORCE_XPD_AMP_R = crate::FieldReader;
#[doc = "Field `FORCE_XPD_AMP` writer - no public"]
pub type FORCE_XPD_AMP_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `AMP_RST_FB_FORCE` reader - no public"]
pub type AMP_RST_FB_FORCE_R = crate::FieldReader;
#[doc = "Field `AMP_RST_FB_FORCE` writer - no public"]
pub type AMP_RST_FB_FORCE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `AMP_SHORT_REF_FORCE` reader - no public"]
pub type AMP_SHORT_REF_FORCE_R = crate::FieldReader;
#[doc = "Field `AMP_SHORT_REF_FORCE` writer - no public"]
pub type AMP_SHORT_REF_FORCE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `AMP_SHORT_REF_GND_FORCE` reader - no public"]
pub type AMP_SHORT_REF_GND_FORCE_R = crate::FieldReader;
#[doc = "Field `AMP_SHORT_REF_GND_FORCE` writer - no public"]
pub type AMP_SHORT_REF_GND_FORCE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 24:25 - no public"]
    #[inline(always)]
    pub fn force_xpd_amp(&self) -> FORCE_XPD_AMP_R {
        FORCE_XPD_AMP_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - no public"]
    #[inline(always)]
    pub fn amp_rst_fb_force(&self) -> AMP_RST_FB_FORCE_R {
        AMP_RST_FB_FORCE_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - no public"]
    #[inline(always)]
    pub fn amp_short_ref_force(&self) -> AMP_SHORT_REF_FORCE_R {
        AMP_SHORT_REF_FORCE_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - no public"]
    #[inline(always)]
    pub fn amp_short_ref_gnd_force(&self) -> AMP_SHORT_REF_GND_FORCE_R {
        AMP_SHORT_REF_GND_FORCE_R::new(((self.bits >> 30) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_MEAS1_CTRL1")
            .field("force_xpd_amp", &self.force_xpd_amp())
            .field("amp_rst_fb_force", &self.amp_rst_fb_force())
            .field("amp_short_ref_force", &self.amp_short_ref_force())
            .field("amp_short_ref_gnd_force", &self.amp_short_ref_gnd_force())
            .finish()
    }
}
impl W {
    #[doc = "Bits 24:25 - no public"]
    #[inline(always)]
    pub fn force_xpd_amp(&mut self) -> FORCE_XPD_AMP_W<'_, SAR_MEAS1_CTRL1_SPEC> {
        FORCE_XPD_AMP_W::new(self, 24)
    }
    #[doc = "Bits 26:27 - no public"]
    #[inline(always)]
    pub fn amp_rst_fb_force(&mut self) -> AMP_RST_FB_FORCE_W<'_, SAR_MEAS1_CTRL1_SPEC> {
        AMP_RST_FB_FORCE_W::new(self, 26)
    }
    #[doc = "Bits 28:29 - no public"]
    #[inline(always)]
    pub fn amp_short_ref_force(&mut self) -> AMP_SHORT_REF_FORCE_W<'_, SAR_MEAS1_CTRL1_SPEC> {
        AMP_SHORT_REF_FORCE_W::new(self, 28)
    }
    #[doc = "Bits 30:31 - no public"]
    #[inline(always)]
    pub fn amp_short_ref_gnd_force(
        &mut self,
    ) -> AMP_SHORT_REF_GND_FORCE_W<'_, SAR_MEAS1_CTRL1_SPEC> {
        AMP_SHORT_REF_GND_FORCE_W::new(self, 30)
    }
}
#[doc = "no public\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_meas1_ctrl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_meas1_ctrl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAR_MEAS1_CTRL1_SPEC;
impl crate::RegisterSpec for SAR_MEAS1_CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar_meas1_ctrl1::R`](R) reader structure"]
impl crate::Readable for SAR_MEAS1_CTRL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sar_meas1_ctrl1::W`](W) writer structure"]
impl crate::Writable for SAR_MEAS1_CTRL1_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SAR_MEAS1_CTRL1 to value 0"]
impl crate::Resettable for SAR_MEAS1_CTRL1_SPEC {}
