#[doc = "Register `L2_CACHE_ACS_FAIL_ID_ATTR` reader"]
pub struct R(crate::R<L2_CACHE_ACS_FAIL_ID_ATTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<L2_CACHE_ACS_FAIL_ID_ATTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<L2_CACHE_ACS_FAIL_ID_ATTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<L2_CACHE_ACS_FAIL_ID_ATTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `L2_CACHE_FAIL_ID` reader - The register records the ID of fail-access when L1-Cache accesses L2-Cache."]
pub type L2_CACHE_FAIL_ID_R = crate::FieldReader<u16, u16>;
#[doc = "Field `L2_CACHE_FAIL_ATTR` reader - The register records the attribution of fail-access when L1-Cache accesses L2-Cache due to cache accessing L1-Cache."]
pub type L2_CACHE_FAIL_ATTR_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - The register records the ID of fail-access when L1-Cache accesses L2-Cache."]
    #[inline(always)]
    pub fn l2_cache_fail_id(&self) -> L2_CACHE_FAIL_ID_R {
        L2_CACHE_FAIL_ID_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - The register records the attribution of fail-access when L1-Cache accesses L2-Cache due to cache accessing L1-Cache."]
    #[inline(always)]
    pub fn l2_cache_fail_attr(&self) -> L2_CACHE_FAIL_ATTR_R {
        L2_CACHE_FAIL_ATTR_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "L2-Cache Access Fail ID/attribution information register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l2_cache_acs_fail_id_attr](index.html) module"]
pub struct L2_CACHE_ACS_FAIL_ID_ATTR_SPEC;
impl crate::RegisterSpec for L2_CACHE_ACS_FAIL_ID_ATTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [l2_cache_acs_fail_id_attr::R](R) reader structure"]
impl crate::Readable for L2_CACHE_ACS_FAIL_ID_ATTR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets L2_CACHE_ACS_FAIL_ID_ATTR to value 0"]
impl crate::Resettable for L2_CACHE_ACS_FAIL_ID_ATTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
