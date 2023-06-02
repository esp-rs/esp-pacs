#[doc = "Register `CNTL_SENSOR_CTRL` reader"]
pub struct R(crate::R<CNTL_SENSOR_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CNTL_SENSOR_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CNTL_SENSOR_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CNTL_SENSOR_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CNTL_SENSOR_CTRL` writer"]
pub struct W(crate::W<CNTL_SENSOR_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CNTL_SENSOR_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CNTL_SENSOR_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CNTL_SENSOR_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SAR2_PWDET_CCT` reader - Need add desc"]
pub type SAR2_PWDET_CCT_R = crate::FieldReader;
#[doc = "Field `SAR2_PWDET_CCT` writer - Need add desc"]
pub type SAR2_PWDET_CCT_W<'a, const O: u8> = crate::FieldWriter<'a, CNTL_SENSOR_CTRL_SPEC, 3, O>;
#[doc = "Field `FORCE_XPD_SAR` reader - Need add desc"]
pub type FORCE_XPD_SAR_R = crate::FieldReader;
#[doc = "Field `FORCE_XPD_SAR` writer - Need add desc"]
pub type FORCE_XPD_SAR_W<'a, const O: u8> = crate::FieldWriter<'a, CNTL_SENSOR_CTRL_SPEC, 2, O>;
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
impl core::fmt::Debug for crate::generic::Reg<CNTL_SENSOR_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 27:29 - Need add desc"]
    #[inline(always)]
    #[must_use]
    pub fn sar2_pwdet_cct(&mut self) -> SAR2_PWDET_CCT_W<27> {
        SAR2_PWDET_CCT_W::new(self)
    }
    #[doc = "Bits 30:31 - Need add desc"]
    #[inline(always)]
    #[must_use]
    pub fn force_xpd_sar(&mut self) -> FORCE_XPD_SAR_W<30> {
        FORCE_XPD_SAR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cntl_sensor_ctrl](index.html) module"]
pub struct CNTL_SENSOR_CTRL_SPEC;
impl crate::RegisterSpec for CNTL_SENSOR_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cntl_sensor_ctrl::R](R) reader structure"]
impl crate::Readable for CNTL_SENSOR_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cntl_sensor_ctrl::W](W) writer structure"]
impl crate::Writable for CNTL_SENSOR_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CNTL_SENSOR_CTRL to value 0"]
impl crate::Resettable for CNTL_SENSOR_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
