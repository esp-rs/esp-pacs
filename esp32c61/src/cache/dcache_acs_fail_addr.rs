#[doc = "Register `DCACHE_ACS_FAIL_ADDR` reader"]
pub type R = crate::R<DCACHE_ACS_FAIL_ADDR_SPEC>;
#[doc = "Field `CACHE_FAIL_ADDR` reader - The register records the address of fail-access when cache accesses L1-Cache."]
pub type CACHE_FAIL_ADDR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The register records the address of fail-access when cache accesses L1-Cache."]
    #[inline(always)]
    pub fn cache_fail_addr(&self) -> CACHE_FAIL_ADDR_R {
        CACHE_FAIL_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCACHE_ACS_FAIL_ADDR")
            .field("cache_fail_addr", &self.cache_fail_addr())
            .finish()
    }
}
#[doc = "L1-Cache Access Fail Address information register\n\nYou can [`read`](crate::Reg::read) this register and get [`dcache_acs_fail_addr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCACHE_ACS_FAIL_ADDR_SPEC;
impl crate::RegisterSpec for DCACHE_ACS_FAIL_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcache_acs_fail_addr::R`](R) reader structure"]
impl crate::Readable for DCACHE_ACS_FAIL_ADDR_SPEC {}
#[doc = "`reset()` method sets DCACHE_ACS_FAIL_ADDR to value 0"]
impl crate::Resettable for DCACHE_ACS_FAIL_ADDR_SPEC {}
