#[doc = "Register `M1_STATUS` reader"]
pub struct R(crate::R<M1_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<M1_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<M1_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<M1_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `M1_EXCEPTION_STATUS` reader - Exception status"]
pub type M1_EXCEPTION_STATUS_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:1 - Exception status"]
    #[inline(always)]
    pub fn m1_exception_status(&self) -> M1_EXCEPTION_STATUS_R {
        M1_EXCEPTION_STATUS_R::new((self.bits & 3) as u8)
    }
}
#[doc = "M1 status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m1_status](index.html) module"]
pub struct M1_STATUS_SPEC;
impl crate::RegisterSpec for M1_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [m1_status::R](R) reader structure"]
impl crate::Readable for M1_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets M1_STATUS to value 0"]
impl crate::Resettable for M1_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
