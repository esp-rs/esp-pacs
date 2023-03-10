#[doc = "Register `FH2_STATUS` reader"]
pub struct R(crate::R<FH2_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FH2_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FH2_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FH2_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FH2_CBC_ON` reader - "]
pub type FH2_CBC_ON_R = crate::BitReader<bool>;
#[doc = "Field `FH2_OST_ON` reader - "]
pub type FH2_OST_ON_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn fh2_cbc_on(&self) -> FH2_CBC_ON_R {
        FH2_CBC_ON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn fh2_ost_on(&self) -> FH2_OST_ON_R {
        FH2_OST_ON_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fh2_status](index.html) module"]
pub struct FH2_STATUS_SPEC;
impl crate::RegisterSpec for FH2_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fh2_status::R](R) reader structure"]
impl crate::Readable for FH2_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FH2_STATUS to value 0"]
impl crate::Resettable for FH2_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
