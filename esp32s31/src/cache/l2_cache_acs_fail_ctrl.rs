#[doc = "Register `L2_CACHE_ACS_FAIL_CTRL` reader"]
pub type R = crate::R<L2_CACHE_ACS_FAIL_CTRL_SPEC>;
#[doc = "Field `L2_CACHE_ACS_FAIL_CHECK_MODE` reader - Configures l2 cache access fail check mode. 0: the access fail is not propagated to the request, 1: the access fail is propagated to the request"]
pub type L2_CACHE_ACS_FAIL_CHECK_MODE_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Configures l2 cache access fail check mode. 0: the access fail is not propagated to the request, 1: the access fail is propagated to the request"]
    #[inline(always)]
    pub fn l2_cache_acs_fail_check_mode(&self) -> L2_CACHE_ACS_FAIL_CHECK_MODE_R {
        L2_CACHE_ACS_FAIL_CHECK_MODE_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L2_CACHE_ACS_FAIL_CTRL")
            .field(
                "l2_cache_acs_fail_check_mode",
                &self.l2_cache_acs_fail_check_mode(),
            )
            .finish()
    }
}
#[doc = "Cache Access Fail Configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`l2_cache_acs_fail_ctrl::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L2_CACHE_ACS_FAIL_CTRL_SPEC;
impl crate::RegisterSpec for L2_CACHE_ACS_FAIL_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l2_cache_acs_fail_ctrl::R`](R) reader structure"]
impl crate::Readable for L2_CACHE_ACS_FAIL_CTRL_SPEC {}
#[doc = "`reset()` method sets L2_CACHE_ACS_FAIL_CTRL to value 0"]
impl crate::Resettable for L2_CACHE_ACS_FAIL_CTRL_SPEC {}
