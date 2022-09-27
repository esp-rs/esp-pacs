#[doc = "Register `REAL_TARGET2_LO` reader"]
pub struct R(crate::R<REAL_TARGET2_LO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REAL_TARGET2_LO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REAL_TARGET2_LO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REAL_TARGET2_LO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TARGET2_LO_RO` reader - actual target value value low 32bits"]
pub type TARGET2_LO_RO_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - actual target value value low 32bits"]
    #[inline(always)]
    pub fn target2_lo_ro(&self) -> TARGET2_LO_RO_R {
        TARGET2_LO_RO_R::new(self.bits)
    }
}
#[doc = "system timer comp2 actual target value low register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [real_target2_lo](index.html) module"]
pub struct REAL_TARGET2_LO_SPEC;
impl crate::RegisterSpec for REAL_TARGET2_LO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [real_target2_lo::R](R) reader structure"]
impl crate::Readable for REAL_TARGET2_LO_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets REAL_TARGET2_LO to value 0"]
impl crate::Resettable for REAL_TARGET2_LO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
