#[doc = "Register `SENSOR_CTRL` reader"]
pub type R = crate::R<SENSOR_CTRL_SPEC>;
#[doc = "Register `SENSOR_CTRL` writer"]
pub type W = crate::W<SENSOR_CTRL_SPEC>;
#[doc = "Field `SAR2_PWDET_CCT` reader - reg_sar2_pwdet_cct"]
pub type SAR2_PWDET_CCT_R = crate::FieldReader;
#[doc = "Field `SAR2_PWDET_CCT` writer - reg_sar2_pwdet_cct"]
pub type SAR2_PWDET_CCT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `FORCE_XPD_SAR` reader - force power up SAR"]
pub type FORCE_XPD_SAR_R = crate::FieldReader;
#[doc = "Field `FORCE_XPD_SAR` writer - force power up SAR"]
pub type FORCE_XPD_SAR_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 27:29 - reg_sar2_pwdet_cct"]
    #[inline(always)]
    pub fn sar2_pwdet_cct(&self) -> SAR2_PWDET_CCT_R {
        SAR2_PWDET_CCT_R::new(((self.bits >> 27) & 7) as u8)
    }
    #[doc = "Bits 30:31 - force power up SAR"]
    #[inline(always)]
    pub fn force_xpd_sar(&self) -> FORCE_XPD_SAR_R {
        FORCE_XPD_SAR_R::new(((self.bits >> 30) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SENSOR_CTRL")
            .field(
                "sar2_pwdet_cct",
                &format_args!("{}", self.sar2_pwdet_cct().bits()),
            )
            .field(
                "force_xpd_sar",
                &format_args!("{}", self.force_xpd_sar().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SENSOR_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 27:29 - reg_sar2_pwdet_cct"]
    #[inline(always)]
    #[must_use]
    pub fn sar2_pwdet_cct(&mut self) -> SAR2_PWDET_CCT_W<SENSOR_CTRL_SPEC> {
        SAR2_PWDET_CCT_W::new(self, 27)
    }
    #[doc = "Bits 30:31 - force power up SAR"]
    #[inline(always)]
    #[must_use]
    pub fn force_xpd_sar(&mut self) -> FORCE_XPD_SAR_W<SENSOR_CTRL_SPEC> {
        FORCE_XPD_SAR_W::new(self, 30)
    }
}
#[doc = "rtc configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sensor_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sensor_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SENSOR_CTRL_SPEC;
impl crate::RegisterSpec for SENSOR_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sensor_ctrl::R`](R) reader structure"]
impl crate::Readable for SENSOR_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sensor_ctrl::W`](W) writer structure"]
impl crate::Writable for SENSOR_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SENSOR_CTRL to value 0"]
impl crate::Resettable for SENSOR_CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
