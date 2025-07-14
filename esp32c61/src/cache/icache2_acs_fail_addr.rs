#[doc = "Register `ICACHE2_ACS_FAIL_ADDR` reader"]
pub type R = crate::R<ICACHE2_ACS_FAIL_ADDR_SPEC>;
#[doc = "Field `ICACHE2_FAIL_ADDR` reader - The register records the address of fail-access when cache2 accesses L1-ICache."]
pub type ICACHE2_FAIL_ADDR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The register records the address of fail-access when cache2 accesses L1-ICache."]
    #[inline(always)]
    pub fn icache2_fail_addr(&self) -> ICACHE2_FAIL_ADDR_R {
        ICACHE2_FAIL_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICACHE2_ACS_FAIL_ADDR")
            .field("icache2_fail_addr", &self.icache2_fail_addr())
            .finish()
    }
}
#[doc = "L1-ICache0 Access Fail Address information register\n\nYou can [`read`](crate::Reg::read) this register and get [`icache2_acs_fail_addr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICACHE2_ACS_FAIL_ADDR_SPEC;
impl crate::RegisterSpec for ICACHE2_ACS_FAIL_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icache2_acs_fail_addr::R`](R) reader structure"]
impl crate::Readable for ICACHE2_ACS_FAIL_ADDR_SPEC {}
#[doc = "`reset()` method sets ICACHE2_ACS_FAIL_ADDR to value 0"]
impl crate::Resettable for ICACHE2_ACS_FAIL_ADDR_SPEC {}
