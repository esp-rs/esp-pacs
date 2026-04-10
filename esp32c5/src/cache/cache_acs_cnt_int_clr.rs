#[doc = "Register `CACHE_ACS_CNT_INT_CLR` writer"]
pub type W = crate::W<CACHE_ACS_CNT_INT_CLR_SPEC>;
#[doc = "Field `IBUS2_OVF_INT_CLR` writer - Reserved"]
pub type IBUS2_OVF_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IBUS3_OVF_INT_CLR` writer - Reserved"]
pub type IBUS3_OVF_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUS0_OVF_INT_CLR` writer - The bit is used to clear counters overflow interrupt and counters in DCache due to bus0 accesses DCache."]
pub type BUS0_OVF_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUS1_OVF_INT_CLR` writer - The bit is used to clear counters overflow interrupt and counters in DCache due to bus1 accesses DCache."]
pub type BUS1_OVF_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CACHE_ACS_CNT_INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn ibus2_ovf_int_clr(&mut self) -> IBUS2_OVF_INT_CLR_W<'_, CACHE_ACS_CNT_INT_CLR_SPEC> {
        IBUS2_OVF_INT_CLR_W::new(self, 2)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn ibus3_ovf_int_clr(&mut self) -> IBUS3_OVF_INT_CLR_W<'_, CACHE_ACS_CNT_INT_CLR_SPEC> {
        IBUS3_OVF_INT_CLR_W::new(self, 3)
    }
    #[doc = "Bit 4 - The bit is used to clear counters overflow interrupt and counters in DCache due to bus0 accesses DCache."]
    #[inline(always)]
    pub fn bus0_ovf_int_clr(&mut self) -> BUS0_OVF_INT_CLR_W<'_, CACHE_ACS_CNT_INT_CLR_SPEC> {
        BUS0_OVF_INT_CLR_W::new(self, 4)
    }
    #[doc = "Bit 5 - The bit is used to clear counters overflow interrupt and counters in DCache due to bus1 accesses DCache."]
    #[inline(always)]
    pub fn bus1_ovf_int_clr(&mut self) -> BUS1_OVF_INT_CLR_W<'_, CACHE_ACS_CNT_INT_CLR_SPEC> {
        BUS1_OVF_INT_CLR_W::new(self, 5)
    }
}
#[doc = "Cache Access Counter Interrupt clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_acs_cnt_int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CACHE_ACS_CNT_INT_CLR_SPEC;
impl crate::RegisterSpec for CACHE_ACS_CNT_INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cache_acs_cnt_int_clr::W`](W) writer structure"]
impl crate::Writable for CACHE_ACS_CNT_INT_CLR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CACHE_ACS_CNT_INT_CLR to value 0"]
impl crate::Resettable for CACHE_ACS_CNT_INT_CLR_SPEC {}
