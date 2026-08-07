#[doc = "Register `L1_CACHE_ACS_FAIL_INT_CLR` writer"]
pub type W = crate::W<L1_CACHE_ACS_FAIL_INT_CLR_SPEC>;
#[doc = "Field `L1_ICACHE0_FAIL_INT_CLR` writer - The bit is used to clear interrupt of access fail that occurs in L1-ICache0 due to cpu accesses L1-ICache0."]
pub type L1_ICACHE0_FAIL_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1_ICACHE1_FAIL_INT_CLR` writer - The bit is used to clear interrupt of access fail that occurs in L1-ICache1 due to cpu accesses L1-ICache1."]
pub type L1_ICACHE1_FAIL_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1_ICACHE2_FAIL_INT_CLR` writer - Reserved"]
pub type L1_ICACHE2_FAIL_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1_ICACHE3_FAIL_INT_CLR` writer - Reserved"]
pub type L1_ICACHE3_FAIL_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1_DCACHE_FAIL_INT_CLR` writer - The bit is used to clear interrupt of access fail that occurs in L1-DCache due to cpu accesses L1-DCache."]
pub type L1_DCACHE_FAIL_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<L1_CACHE_ACS_FAIL_INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - The bit is used to clear interrupt of access fail that occurs in L1-ICache0 due to cpu accesses L1-ICache0."]
    #[inline(always)]
    pub fn l1_icache0_fail_int_clr(
        &mut self,
    ) -> L1_ICACHE0_FAIL_INT_CLR_W<'_, L1_CACHE_ACS_FAIL_INT_CLR_SPEC> {
        L1_ICACHE0_FAIL_INT_CLR_W::new(self, 0)
    }
    #[doc = "Bit 1 - The bit is used to clear interrupt of access fail that occurs in L1-ICache1 due to cpu accesses L1-ICache1."]
    #[inline(always)]
    pub fn l1_icache1_fail_int_clr(
        &mut self,
    ) -> L1_ICACHE1_FAIL_INT_CLR_W<'_, L1_CACHE_ACS_FAIL_INT_CLR_SPEC> {
        L1_ICACHE1_FAIL_INT_CLR_W::new(self, 1)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn l1_icache2_fail_int_clr(
        &mut self,
    ) -> L1_ICACHE2_FAIL_INT_CLR_W<'_, L1_CACHE_ACS_FAIL_INT_CLR_SPEC> {
        L1_ICACHE2_FAIL_INT_CLR_W::new(self, 2)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn l1_icache3_fail_int_clr(
        &mut self,
    ) -> L1_ICACHE3_FAIL_INT_CLR_W<'_, L1_CACHE_ACS_FAIL_INT_CLR_SPEC> {
        L1_ICACHE3_FAIL_INT_CLR_W::new(self, 3)
    }
    #[doc = "Bit 4 - The bit is used to clear interrupt of access fail that occurs in L1-DCache due to cpu accesses L1-DCache."]
    #[inline(always)]
    pub fn l1_dcache_fail_int_clr(
        &mut self,
    ) -> L1_DCACHE_FAIL_INT_CLR_W<'_, L1_CACHE_ACS_FAIL_INT_CLR_SPEC> {
        L1_DCACHE_FAIL_INT_CLR_W::new(self, 4)
    }
}
#[doc = "L1-Cache Access Fail Interrupt clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1_cache_acs_fail_int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1_CACHE_ACS_FAIL_INT_CLR_SPEC;
impl crate::RegisterSpec for L1_CACHE_ACS_FAIL_INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`l1_cache_acs_fail_int_clr::W`](W) writer structure"]
impl crate::Writable for L1_CACHE_ACS_FAIL_INT_CLR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets L1_CACHE_ACS_FAIL_INT_CLR to value 0"]
impl crate::Resettable for L1_CACHE_ACS_FAIL_INT_CLR_SPEC {}
