///Register `L2_CACHE_ACS_FAIL_INT_CLR` writer
pub type W = crate::W<L2_CACHE_ACS_FAIL_INT_CLR_SPEC>;
///Field `L2_CACHE_FAIL_INT_CLR` writer - The bit is used to clear interrupt of access fail that occurs in L2-Cache due to l1 cache accesses L2-Cache.
pub type L2_CACHE_FAIL_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<L2_CACHE_ACS_FAIL_INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 5 - The bit is used to clear interrupt of access fail that occurs in L2-Cache due to l1 cache accesses L2-Cache.
    #[inline(always)]
    #[must_use]
    pub fn l2_cache_fail_int_clr(
        &mut self,
    ) -> L2_CACHE_FAIL_INT_CLR_W<L2_CACHE_ACS_FAIL_INT_CLR_SPEC> {
        L2_CACHE_FAIL_INT_CLR_W::new(self, 5)
    }
}
/**L1-Cache Access Fail Interrupt clear register

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2_cache_acs_fail_int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct L2_CACHE_ACS_FAIL_INT_CLR_SPEC;
impl crate::RegisterSpec for L2_CACHE_ACS_FAIL_INT_CLR_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`l2_cache_acs_fail_int_clr::W`](W) writer structure
impl crate::Writable for L2_CACHE_ACS_FAIL_INT_CLR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets L2_CACHE_ACS_FAIL_INT_CLR to value 0
impl crate::Resettable for L2_CACHE_ACS_FAIL_INT_CLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
