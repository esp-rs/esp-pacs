#[doc = "Register `L2_CACHE_ACS_FAIL_INT_CLR` reader"]
pub struct R(crate::R<L2_CACHE_ACS_FAIL_INT_CLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<L2_CACHE_ACS_FAIL_INT_CLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<L2_CACHE_ACS_FAIL_INT_CLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<L2_CACHE_ACS_FAIL_INT_CLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `L2_CACHE_FAIL_INT_CLR` reader - The bit is used to clear interrupt of access fail that occurs in L2-Cache due to l1 cache accesses L2-Cache."]
pub type L2_CACHE_FAIL_INT_CLR_R = crate::BitReader;
impl R {
    #[doc = "Bit 5 - The bit is used to clear interrupt of access fail that occurs in L2-Cache due to l1 cache accesses L2-Cache."]
    #[inline(always)]
    pub fn l2_cache_fail_int_clr(&self) -> L2_CACHE_FAIL_INT_CLR_R {
        L2_CACHE_FAIL_INT_CLR_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L2_CACHE_ACS_FAIL_INT_CLR")
            .field(
                "l2_cache_fail_int_clr",
                &format_args!("{}", self.l2_cache_fail_int_clr().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<L2_CACHE_ACS_FAIL_INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "L1-Cache Access Fail Interrupt clear register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l2_cache_acs_fail_int_clr](index.html) module"]
pub struct L2_CACHE_ACS_FAIL_INT_CLR_SPEC;
impl crate::RegisterSpec for L2_CACHE_ACS_FAIL_INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [l2_cache_acs_fail_int_clr::R](R) reader structure"]
impl crate::Readable for L2_CACHE_ACS_FAIL_INT_CLR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets L2_CACHE_ACS_FAIL_INT_CLR to value 0"]
impl crate::Resettable for L2_CACHE_ACS_FAIL_INT_CLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
