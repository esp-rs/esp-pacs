#[doc = "Register `SAR_CCT` reader"]
pub struct R(crate::R<SAR_CCT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAR_CCT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAR_CCT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAR_CCT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAR_CCT` writer"]
pub struct W(crate::W<SAR_CCT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAR_CCT_SPEC>;
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
impl From<crate::W<SAR_CCT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAR_CCT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SAR2_PWDET_CCT` reader - need_des"]
pub type SAR2_PWDET_CCT_R = crate::FieldReader;
#[doc = "Field `SAR2_PWDET_CCT` writer - need_des"]
pub type SAR2_PWDET_CCT_W<'a, const O: u8> = crate::FieldWriter<'a, SAR_CCT_SPEC, 3, O>;
impl R {
    #[doc = "Bits 29:31 - need_des"]
    #[inline(always)]
    pub fn sar2_pwdet_cct(&self) -> SAR2_PWDET_CCT_R {
        SAR2_PWDET_CCT_R::new(((self.bits >> 29) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_CCT")
            .field(
                "sar2_pwdet_cct",
                &format_args!("{}", self.sar2_pwdet_cct().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SAR_CCT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 29:31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn sar2_pwdet_cct(&mut self) -> SAR2_PWDET_CCT_W<29> {
        SAR2_PWDET_CCT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "need_des\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar_cct](index.html) module"]
pub struct SAR_CCT_SPEC;
impl crate::RegisterSpec for SAR_CCT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sar_cct::R](R) reader structure"]
impl crate::Readable for SAR_CCT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sar_cct::W](W) writer structure"]
impl crate::Writable for SAR_CCT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SAR_CCT to value 0"]
impl crate::Resettable for SAR_CCT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
