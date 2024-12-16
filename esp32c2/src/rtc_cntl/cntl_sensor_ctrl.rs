#[doc = "Register `CNTL_SENSOR_CTRL` reader"]
pub type R = crate::R<CNTL_SENSOR_CTRL_SPEC>;
#[doc = "Register `CNTL_SENSOR_CTRL` writer"]
pub type W = crate::W<CNTL_SENSOR_CTRL_SPEC>;
#[doc = "Field `SAR2_PWDET_CCT` reader - Need add desc"]
pub type SAR2_PWDET_CCT_R = crate::FieldReader;
#[doc = "Field `SAR2_PWDET_CCT` writer - Need add desc"]
pub type SAR2_PWDET_CCT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `FORCE_XPD_SAR` reader - Need add desc"]
pub type FORCE_XPD_SAR_R = crate::FieldReader;
#[doc = "Field `FORCE_XPD_SAR` writer - Need add desc"]
pub type FORCE_XPD_SAR_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 27:29 - Need add desc"]
    #[inline(always)]
    pub fn sar2_pwdet_cct(&self) -> SAR2_PWDET_CCT_R {
        SAR2_PWDET_CCT_R::new(((self.bits >> 27) & 7) as u8)
    }
    #[doc = "Bits 30:31 - Need add desc"]
    #[inline(always)]
    pub fn force_xpd_sar(&self) -> FORCE_XPD_SAR_R {
        FORCE_XPD_SAR_R::new(((self.bits >> 30) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CNTL_SENSOR_CTRL")
            .field("sar2_pwdet_cct", &self.sar2_pwdet_cct())
            .field("force_xpd_sar", &self.force_xpd_sar())
            .finish()
    }
}
impl W {
    #[doc = "Bits 27:29 - Need add desc"]
    #[inline(always)]
    pub fn sar2_pwdet_cct(&mut self) -> SAR2_PWDET_CCT_W<CNTL_SENSOR_CTRL_SPEC> {
        SAR2_PWDET_CCT_W::new(self, 27)
    }
    #[doc = "Bits 30:31 - Need add desc"]
    #[inline(always)]
    pub fn force_xpd_sar(&mut self) -> FORCE_XPD_SAR_W<CNTL_SENSOR_CTRL_SPEC> {
        FORCE_XPD_SAR_W::new(self, 30)
    }
}
#[doc = "register description\n\nYou can [`read`](crate::Reg::read) this register and get [`cntl_sensor_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cntl_sensor_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CNTL_SENSOR_CTRL_SPEC;
impl crate::RegisterSpec for CNTL_SENSOR_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cntl_sensor_ctrl::R`](R) reader structure"]
impl crate::Readable for CNTL_SENSOR_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cntl_sensor_ctrl::W`](W) writer structure"]
impl crate::Writable for CNTL_SENSOR_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CNTL_SENSOR_CTRL to value 0"]
impl crate::Resettable for CNTL_SENSOR_CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
