#[doc = "Register `CACHE_ACS_CNT_CLR` writer"]
pub type W = crate::W<CACHE_ACS_CNT_CLR_SPEC>;
#[doc = "Field `DCACHE_ACS_CNT_CLR` writer - The bit is used to clear dcache counter."]
pub type DCACHE_ACS_CNT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICACHE_ACS_CNT_CLR` writer - The bit is used to clear icache counter."]
pub type ICACHE_ACS_CNT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CACHE_ACS_CNT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - The bit is used to clear dcache counter."]
    #[inline(always)]
    pub fn dcache_acs_cnt_clr(&mut self) -> DCACHE_ACS_CNT_CLR_W<'_, CACHE_ACS_CNT_CLR_SPEC> {
        DCACHE_ACS_CNT_CLR_W::new(self, 0)
    }
    #[doc = "Bit 1 - The bit is used to clear icache counter."]
    #[inline(always)]
    pub fn icache_acs_cnt_clr(&mut self) -> ICACHE_ACS_CNT_CLR_W<'_, CACHE_ACS_CNT_CLR_SPEC> {
        ICACHE_ACS_CNT_CLR_W::new(self, 1)
    }
}
#[doc = "******* Description ***********\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_acs_cnt_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CACHE_ACS_CNT_CLR_SPEC;
impl crate::RegisterSpec for CACHE_ACS_CNT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cache_acs_cnt_clr::W`](W) writer structure"]
impl crate::Writable for CACHE_ACS_CNT_CLR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CACHE_ACS_CNT_CLR to value 0"]
impl crate::Resettable for CACHE_ACS_CNT_CLR_SPEC {}
