#[doc = "Register `L1_DCACHE_ACS_FAIL_ADDR` reader"]
pub type R = crate::R<L1_DCACHE_ACS_FAIL_ADDR_SPEC>;
#[doc = "Field `L1_DCACHE_FAIL_ADDR` reader - The register records the address of fail-access when cache accesses L1-DCache."]
pub type L1_DCACHE_FAIL_ADDR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The register records the address of fail-access when cache accesses L1-DCache."]
    #[inline(always)]
    pub fn l1_dcache_fail_addr(&self) -> L1_DCACHE_FAIL_ADDR_R {
        L1_DCACHE_FAIL_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L1_DCACHE_ACS_FAIL_ADDR")
            .field(
                "l1_dcache_fail_addr",
                &format_args!("{}", self.l1_dcache_fail_addr().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<L1_DCACHE_ACS_FAIL_ADDR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "L1-DCache Access Fail Address information register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l1_dcache_acs_fail_addr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1_DCACHE_ACS_FAIL_ADDR_SPEC;
impl crate::RegisterSpec for L1_DCACHE_ACS_FAIL_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l1_dcache_acs_fail_addr::R`](R) reader structure"]
impl crate::Readable for L1_DCACHE_ACS_FAIL_ADDR_SPEC {}
#[doc = "`reset()` method sets L1_DCACHE_ACS_FAIL_ADDR to value 0"]
impl crate::Resettable for L1_DCACHE_ACS_FAIL_ADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
