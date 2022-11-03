#[doc = "Register `L2_CACHE_ACS_FAIL_INT_RAW` reader"]
pub struct R(crate::R<L2_CACHE_ACS_FAIL_INT_RAW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<L2_CACHE_ACS_FAIL_INT_RAW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<L2_CACHE_ACS_FAIL_INT_RAW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<L2_CACHE_ACS_FAIL_INT_RAW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `L2_CACHE_FAIL_INT_RAW` reader - The raw bit of the interrupt of access fail that occurs in L2-Cache."]
pub type L2_CACHE_FAIL_INT_RAW_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 5 - The raw bit of the interrupt of access fail that occurs in L2-Cache."]
    #[inline(always)]
    pub fn l2_cache_fail_int_raw(&self) -> L2_CACHE_FAIL_INT_RAW_R {
        L2_CACHE_FAIL_INT_RAW_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "Cache Access Fail Interrupt raw register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l2_cache_acs_fail_int_raw](index.html) module"]
pub struct L2_CACHE_ACS_FAIL_INT_RAW_SPEC;
impl crate::RegisterSpec for L2_CACHE_ACS_FAIL_INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [l2_cache_acs_fail_int_raw::R](R) reader structure"]
impl crate::Readable for L2_CACHE_ACS_FAIL_INT_RAW_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets L2_CACHE_ACS_FAIL_INT_RAW to value 0"]
impl crate::Resettable for L2_CACHE_ACS_FAIL_INT_RAW_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
