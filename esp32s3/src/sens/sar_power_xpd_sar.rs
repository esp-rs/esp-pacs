#[doc = "Register `SAR_POWER_XPD_SAR` reader"]
pub type R = crate::R<SAR_POWER_XPD_SAR_SPEC>;
#[doc = "Register `SAR_POWER_XPD_SAR` writer"]
pub type W = crate::W<SAR_POWER_XPD_SAR_SPEC>;
#[doc = "Field `FORCE_XPD_SAR` reader - force power on/off saradc"]
pub type FORCE_XPD_SAR_R = crate::FieldReader;
#[doc = "Field `FORCE_XPD_SAR` writer - force power on/off saradc"]
pub type FORCE_XPD_SAR_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SARCLK_EN` reader - no public"]
pub type SARCLK_EN_R = crate::BitReader;
#[doc = "Field `SARCLK_EN` writer - no public"]
pub type SARCLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 29:30 - force power on/off saradc"]
    #[inline(always)]
    pub fn force_xpd_sar(&self) -> FORCE_XPD_SAR_R {
        FORCE_XPD_SAR_R::new(((self.bits >> 29) & 3) as u8)
    }
    #[doc = "Bit 31 - no public"]
    #[inline(always)]
    pub fn sarclk_en(&self) -> SARCLK_EN_R {
        SARCLK_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_POWER_XPD_SAR")
            .field("force_xpd_sar", &self.force_xpd_sar())
            .field("sarclk_en", &self.sarclk_en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 29:30 - force power on/off saradc"]
    #[inline(always)]
    #[must_use]
    pub fn force_xpd_sar(&mut self) -> FORCE_XPD_SAR_W<SAR_POWER_XPD_SAR_SPEC> {
        FORCE_XPD_SAR_W::new(self, 29)
    }
    #[doc = "Bit 31 - no public"]
    #[inline(always)]
    #[must_use]
    pub fn sarclk_en(&mut self) -> SARCLK_EN_W<SAR_POWER_XPD_SAR_SPEC> {
        SARCLK_EN_W::new(self, 31)
    }
}
#[doc = "configure power of saradc\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_power_xpd_sar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_power_xpd_sar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAR_POWER_XPD_SAR_SPEC;
impl crate::RegisterSpec for SAR_POWER_XPD_SAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar_power_xpd_sar::R`](R) reader structure"]
impl crate::Readable for SAR_POWER_XPD_SAR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sar_power_xpd_sar::W`](W) writer structure"]
impl crate::Writable for SAR_POWER_XPD_SAR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SAR_POWER_XPD_SAR to value 0"]
impl crate::Resettable for SAR_POWER_XPD_SAR_SPEC {
    const RESET_VALUE: u32 = 0;
}
