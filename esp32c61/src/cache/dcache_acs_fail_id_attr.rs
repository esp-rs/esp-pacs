#[doc = "Register `DCACHE_ACS_FAIL_ID_ATTR` reader"]
pub type R = crate::R<DCACHE_ACS_FAIL_ID_ATTR_SPEC>;
#[doc = "Field `CACHE_FAIL_ID` reader - The register records the ID of fail-access when cache accesses L1-Cache."]
pub type CACHE_FAIL_ID_R = crate::FieldReader<u16>;
#[doc = "Field `CACHE_FAIL_ATTR` reader - The register records the attribution of fail-access when cache accesses L1-Cache."]
pub type CACHE_FAIL_ATTR_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - The register records the ID of fail-access when cache accesses L1-Cache."]
    #[inline(always)]
    pub fn cache_fail_id(&self) -> CACHE_FAIL_ID_R {
        CACHE_FAIL_ID_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - The register records the attribution of fail-access when cache accesses L1-Cache."]
    #[inline(always)]
    pub fn cache_fail_attr(&self) -> CACHE_FAIL_ATTR_R {
        CACHE_FAIL_ATTR_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCACHE_ACS_FAIL_ID_ATTR")
            .field("cache_fail_id", &self.cache_fail_id())
            .field("cache_fail_attr", &self.cache_fail_attr())
            .finish()
    }
}
#[doc = "L1-Cache Access Fail ID/attribution information register\n\nYou can [`read`](crate::Reg::read) this register and get [`dcache_acs_fail_id_attr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCACHE_ACS_FAIL_ID_ATTR_SPEC;
impl crate::RegisterSpec for DCACHE_ACS_FAIL_ID_ATTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcache_acs_fail_id_attr::R`](R) reader structure"]
impl crate::Readable for DCACHE_ACS_FAIL_ID_ATTR_SPEC {}
#[doc = "`reset()` method sets DCACHE_ACS_FAIL_ID_ATTR to value 0"]
impl crate::Resettable for DCACHE_ACS_FAIL_ID_ATTR_SPEC {}
