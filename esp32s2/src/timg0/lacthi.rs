#[doc = "Register `LACTHI` reader"]
pub struct R(crate::R<LACTHI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LACTHI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LACTHI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LACTHI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LACT_HI` reader - Reserved."]
pub type LACT_HI_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Reserved."]
    #[inline(always)]
    pub fn lact_hi(&self) -> LACT_HI_R {
        LACT_HI_R::new(self.bits)
    }
}
#[doc = "LACT high register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lacthi](index.html) module"]
pub struct LACTHI_SPEC;
impl crate::RegisterSpec for LACTHI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lacthi::R](R) reader structure"]
impl crate::Readable for LACTHI_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LACTHI to value 0"]
impl crate::Resettable for LACTHI_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
