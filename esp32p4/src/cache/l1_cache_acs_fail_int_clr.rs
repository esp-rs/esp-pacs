///Register `L1_CACHE_ACS_FAIL_INT_CLR` reader
pub type R = crate::R<L1_CACHE_ACS_FAIL_INT_CLR_SPEC>;
///Register `L1_CACHE_ACS_FAIL_INT_CLR` writer
pub type W = crate::W<L1_CACHE_ACS_FAIL_INT_CLR_SPEC>;
///Field `L1_ICACHE0_FAIL_INT_CLR` writer - The bit is used to clear interrupt of access fail that occurs in L1-ICache0 due to cpu accesses L1-ICache0.
pub type L1_ICACHE0_FAIL_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `L1_ICACHE1_FAIL_INT_CLR` writer - The bit is used to clear interrupt of access fail that occurs in L1-ICache1 due to cpu accesses L1-ICache1.
pub type L1_ICACHE1_FAIL_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `L1_ICACHE2_FAIL_INT_CLR` reader - Reserved
pub type L1_ICACHE2_FAIL_INT_CLR_R = crate::BitReader;
///Field `L1_ICACHE3_FAIL_INT_CLR` reader - Reserved
pub type L1_ICACHE3_FAIL_INT_CLR_R = crate::BitReader;
///Field `L1_DCACHE_FAIL_INT_CLR` writer - The bit is used to clear interrupt of access fail that occurs in L1-DCache due to cpu accesses L1-DCache.
pub type L1_DCACHE_FAIL_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 2 - Reserved
    #[inline(always)]
    pub fn l1_icache2_fail_int_clr(&self) -> L1_ICACHE2_FAIL_INT_CLR_R {
        L1_ICACHE2_FAIL_INT_CLR_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Reserved
    #[inline(always)]
    pub fn l1_icache3_fail_int_clr(&self) -> L1_ICACHE3_FAIL_INT_CLR_R {
        L1_ICACHE3_FAIL_INT_CLR_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L1_CACHE_ACS_FAIL_INT_CLR")
            .field("l1_icache2_fail_int_clr", &self.l1_icache2_fail_int_clr())
            .field("l1_icache3_fail_int_clr", &self.l1_icache3_fail_int_clr())
            .finish()
    }
}
impl W {
    ///Bit 0 - The bit is used to clear interrupt of access fail that occurs in L1-ICache0 due to cpu accesses L1-ICache0.
    #[inline(always)]
    #[must_use]
    pub fn l1_icache0_fail_int_clr(
        &mut self,
    ) -> L1_ICACHE0_FAIL_INT_CLR_W<L1_CACHE_ACS_FAIL_INT_CLR_SPEC> {
        L1_ICACHE0_FAIL_INT_CLR_W::new(self, 0)
    }
    ///Bit 1 - The bit is used to clear interrupt of access fail that occurs in L1-ICache1 due to cpu accesses L1-ICache1.
    #[inline(always)]
    #[must_use]
    pub fn l1_icache1_fail_int_clr(
        &mut self,
    ) -> L1_ICACHE1_FAIL_INT_CLR_W<L1_CACHE_ACS_FAIL_INT_CLR_SPEC> {
        L1_ICACHE1_FAIL_INT_CLR_W::new(self, 1)
    }
    ///Bit 4 - The bit is used to clear interrupt of access fail that occurs in L1-DCache due to cpu accesses L1-DCache.
    #[inline(always)]
    #[must_use]
    pub fn l1_dcache_fail_int_clr(
        &mut self,
    ) -> L1_DCACHE_FAIL_INT_CLR_W<L1_CACHE_ACS_FAIL_INT_CLR_SPEC> {
        L1_DCACHE_FAIL_INT_CLR_W::new(self, 4)
    }
}
/**L1-Cache Access Fail Interrupt clear register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_cache_acs_fail_int_clr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1_cache_acs_fail_int_clr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct L1_CACHE_ACS_FAIL_INT_CLR_SPEC;
impl crate::RegisterSpec for L1_CACHE_ACS_FAIL_INT_CLR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`l1_cache_acs_fail_int_clr::R`](R) reader structure
impl crate::Readable for L1_CACHE_ACS_FAIL_INT_CLR_SPEC {}
///`write(|w| ..)` method takes [`l1_cache_acs_fail_int_clr::W`](W) writer structure
impl crate::Writable for L1_CACHE_ACS_FAIL_INT_CLR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets L1_CACHE_ACS_FAIL_INT_CLR to value 0
impl crate::Resettable for L1_CACHE_ACS_FAIL_INT_CLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
