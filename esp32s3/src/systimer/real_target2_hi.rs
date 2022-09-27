#[doc = "Register `REAL_TARGET2_HI` reader"]
pub struct R(crate::R<REAL_TARGET2_HI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REAL_TARGET2_HI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REAL_TARGET2_HI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REAL_TARGET2_HI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TARGET2_HI_RO` reader - actual target value value high 20bits"]
pub type TARGET2_HI_RO_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:19 - actual target value value high 20bits"]
    #[inline(always)]
    pub fn target2_hi_ro(&self) -> TARGET2_HI_RO_R {
        TARGET2_HI_RO_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
#[doc = "system timer comp2 actual target value high register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [real_target2_hi](index.html) module"]
pub struct REAL_TARGET2_HI_SPEC;
impl crate::RegisterSpec for REAL_TARGET2_HI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [real_target2_hi::R](R) reader structure"]
impl crate::Readable for REAL_TARGET2_HI_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets REAL_TARGET2_HI to value 0"]
impl crate::Resettable for REAL_TARGET2_HI_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
