#[doc = "Register `L1_CACHE_ACS_CNT_INT_CLR` writer"]
pub type W = crate::W<L1_CACHE_ACS_CNT_INT_CLR_SPEC>;
#[doc = "Field `L1_IBUS0_OVF_INT_CLR` writer - The bit is used to clear counters overflow interrupt and counters in L1-ICache0 due to bus0 accesses L1-ICache0."]
pub type L1_IBUS0_OVF_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1_IBUS1_OVF_INT_CLR` writer - The bit is used to clear counters overflow interrupt and counters in L1-ICache1 due to bus1 accesses L1-ICache1."]
pub type L1_IBUS1_OVF_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1_IBUS2_OVF_INT_CLR` writer - Reserved"]
pub type L1_IBUS2_OVF_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1_IBUS3_OVF_INT_CLR` writer - Reserved"]
pub type L1_IBUS3_OVF_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1_DBUS0_OVF_INT_CLR` writer - The bit is used to clear counters overflow interrupt and counters in L1-DCache due to bus0 accesses L1-DCache."]
pub type L1_DBUS0_OVF_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1_DBUS1_OVF_INT_CLR` writer - The bit is used to clear counters overflow interrupt and counters in L1-DCache due to bus1 accesses L1-DCache."]
pub type L1_DBUS1_OVF_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1_DBUS2_OVF_INT_CLR` writer - Reserved"]
pub type L1_DBUS2_OVF_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1_DBUS3_OVF_INT_CLR` writer - Reserved"]
pub type L1_DBUS3_OVF_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<L1_CACHE_ACS_CNT_INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - The bit is used to clear counters overflow interrupt and counters in L1-ICache0 due to bus0 accesses L1-ICache0."]
    #[inline(always)]
    pub fn l1_ibus0_ovf_int_clr(
        &mut self,
    ) -> L1_IBUS0_OVF_INT_CLR_W<'_, L1_CACHE_ACS_CNT_INT_CLR_SPEC> {
        L1_IBUS0_OVF_INT_CLR_W::new(self, 0)
    }
    #[doc = "Bit 1 - The bit is used to clear counters overflow interrupt and counters in L1-ICache1 due to bus1 accesses L1-ICache1."]
    #[inline(always)]
    pub fn l1_ibus1_ovf_int_clr(
        &mut self,
    ) -> L1_IBUS1_OVF_INT_CLR_W<'_, L1_CACHE_ACS_CNT_INT_CLR_SPEC> {
        L1_IBUS1_OVF_INT_CLR_W::new(self, 1)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn l1_ibus2_ovf_int_clr(
        &mut self,
    ) -> L1_IBUS2_OVF_INT_CLR_W<'_, L1_CACHE_ACS_CNT_INT_CLR_SPEC> {
        L1_IBUS2_OVF_INT_CLR_W::new(self, 2)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn l1_ibus3_ovf_int_clr(
        &mut self,
    ) -> L1_IBUS3_OVF_INT_CLR_W<'_, L1_CACHE_ACS_CNT_INT_CLR_SPEC> {
        L1_IBUS3_OVF_INT_CLR_W::new(self, 3)
    }
    #[doc = "Bit 4 - The bit is used to clear counters overflow interrupt and counters in L1-DCache due to bus0 accesses L1-DCache."]
    #[inline(always)]
    pub fn l1_dbus0_ovf_int_clr(
        &mut self,
    ) -> L1_DBUS0_OVF_INT_CLR_W<'_, L1_CACHE_ACS_CNT_INT_CLR_SPEC> {
        L1_DBUS0_OVF_INT_CLR_W::new(self, 4)
    }
    #[doc = "Bit 5 - The bit is used to clear counters overflow interrupt and counters in L1-DCache due to bus1 accesses L1-DCache."]
    #[inline(always)]
    pub fn l1_dbus1_ovf_int_clr(
        &mut self,
    ) -> L1_DBUS1_OVF_INT_CLR_W<'_, L1_CACHE_ACS_CNT_INT_CLR_SPEC> {
        L1_DBUS1_OVF_INT_CLR_W::new(self, 5)
    }
    #[doc = "Bit 6 - Reserved"]
    #[inline(always)]
    pub fn l1_dbus2_ovf_int_clr(
        &mut self,
    ) -> L1_DBUS2_OVF_INT_CLR_W<'_, L1_CACHE_ACS_CNT_INT_CLR_SPEC> {
        L1_DBUS2_OVF_INT_CLR_W::new(self, 6)
    }
    #[doc = "Bit 7 - Reserved"]
    #[inline(always)]
    pub fn l1_dbus3_ovf_int_clr(
        &mut self,
    ) -> L1_DBUS3_OVF_INT_CLR_W<'_, L1_CACHE_ACS_CNT_INT_CLR_SPEC> {
        L1_DBUS3_OVF_INT_CLR_W::new(self, 7)
    }
}
#[doc = "Cache Access Counter Interrupt clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1_cache_acs_cnt_int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1_CACHE_ACS_CNT_INT_CLR_SPEC;
impl crate::RegisterSpec for L1_CACHE_ACS_CNT_INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`l1_cache_acs_cnt_int_clr::W`](W) writer structure"]
impl crate::Writable for L1_CACHE_ACS_CNT_INT_CLR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets L1_CACHE_ACS_CNT_INT_CLR to value 0"]
impl crate::Resettable for L1_CACHE_ACS_CNT_INT_CLR_SPEC {}
