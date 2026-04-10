#[doc = "Register `CACHE_ACS_CNT_INT_RAW` reader"]
pub type R = crate::R<CACHE_ACS_CNT_INT_RAW_SPEC>;
#[doc = "Register `CACHE_ACS_CNT_INT_RAW` writer"]
pub type W = crate::W<CACHE_ACS_CNT_INT_RAW_SPEC>;
#[doc = "Field `IBUS2_OVF_INT_RAW` reader - The raw bit of the interrupt of one of counters overflow that occurs in ICache2 due to bus2 accesses ICache2."]
pub type IBUS2_OVF_INT_RAW_R = crate::BitReader;
#[doc = "Field `IBUS2_OVF_INT_RAW` writer - The raw bit of the interrupt of one of counters overflow that occurs in ICache2 due to bus2 accesses ICache2."]
pub type IBUS2_OVF_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IBUS3_OVF_INT_RAW` reader - The raw bit of the interrupt of one of counters overflow that occurs in ICache3 due to bus3 accesses ICache3."]
pub type IBUS3_OVF_INT_RAW_R = crate::BitReader;
#[doc = "Field `IBUS3_OVF_INT_RAW` writer - The raw bit of the interrupt of one of counters overflow that occurs in ICache3 due to bus3 accesses ICache3."]
pub type IBUS3_OVF_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUS0_OVF_INT_RAW` reader - The raw bit of the interrupt of one of counters overflow that occurs in DCache due to bus0 accesses DCache."]
pub type BUS0_OVF_INT_RAW_R = crate::BitReader;
#[doc = "Field `BUS0_OVF_INT_RAW` writer - The raw bit of the interrupt of one of counters overflow that occurs in DCache due to bus0 accesses DCache."]
pub type BUS0_OVF_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUS1_OVF_INT_RAW` reader - The raw bit of the interrupt of one of counters overflow that occurs in DCache due to bus1 accesses DCache."]
pub type BUS1_OVF_INT_RAW_R = crate::BitReader;
#[doc = "Field `BUS1_OVF_INT_RAW` writer - The raw bit of the interrupt of one of counters overflow that occurs in DCache due to bus1 accesses DCache."]
pub type BUS1_OVF_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - The raw bit of the interrupt of one of counters overflow that occurs in ICache2 due to bus2 accesses ICache2."]
    #[inline(always)]
    pub fn ibus2_ovf_int_raw(&self) -> IBUS2_OVF_INT_RAW_R {
        IBUS2_OVF_INT_RAW_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The raw bit of the interrupt of one of counters overflow that occurs in ICache3 due to bus3 accesses ICache3."]
    #[inline(always)]
    pub fn ibus3_ovf_int_raw(&self) -> IBUS3_OVF_INT_RAW_R {
        IBUS3_OVF_INT_RAW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The raw bit of the interrupt of one of counters overflow that occurs in DCache due to bus0 accesses DCache."]
    #[inline(always)]
    pub fn bus0_ovf_int_raw(&self) -> BUS0_OVF_INT_RAW_R {
        BUS0_OVF_INT_RAW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The raw bit of the interrupt of one of counters overflow that occurs in DCache due to bus1 accesses DCache."]
    #[inline(always)]
    pub fn bus1_ovf_int_raw(&self) -> BUS1_OVF_INT_RAW_R {
        BUS1_OVF_INT_RAW_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_ACS_CNT_INT_RAW")
            .field("ibus2_ovf_int_raw", &self.ibus2_ovf_int_raw())
            .field("ibus3_ovf_int_raw", &self.ibus3_ovf_int_raw())
            .field("bus0_ovf_int_raw", &self.bus0_ovf_int_raw())
            .field("bus1_ovf_int_raw", &self.bus1_ovf_int_raw())
            .finish()
    }
}
impl W {
    #[doc = "Bit 2 - The raw bit of the interrupt of one of counters overflow that occurs in ICache2 due to bus2 accesses ICache2."]
    #[inline(always)]
    pub fn ibus2_ovf_int_raw(&mut self) -> IBUS2_OVF_INT_RAW_W<'_, CACHE_ACS_CNT_INT_RAW_SPEC> {
        IBUS2_OVF_INT_RAW_W::new(self, 2)
    }
    #[doc = "Bit 3 - The raw bit of the interrupt of one of counters overflow that occurs in ICache3 due to bus3 accesses ICache3."]
    #[inline(always)]
    pub fn ibus3_ovf_int_raw(&mut self) -> IBUS3_OVF_INT_RAW_W<'_, CACHE_ACS_CNT_INT_RAW_SPEC> {
        IBUS3_OVF_INT_RAW_W::new(self, 3)
    }
    #[doc = "Bit 4 - The raw bit of the interrupt of one of counters overflow that occurs in DCache due to bus0 accesses DCache."]
    #[inline(always)]
    pub fn bus0_ovf_int_raw(&mut self) -> BUS0_OVF_INT_RAW_W<'_, CACHE_ACS_CNT_INT_RAW_SPEC> {
        BUS0_OVF_INT_RAW_W::new(self, 4)
    }
    #[doc = "Bit 5 - The raw bit of the interrupt of one of counters overflow that occurs in DCache due to bus1 accesses DCache."]
    #[inline(always)]
    pub fn bus1_ovf_int_raw(&mut self) -> BUS1_OVF_INT_RAW_W<'_, CACHE_ACS_CNT_INT_RAW_SPEC> {
        BUS1_OVF_INT_RAW_W::new(self, 5)
    }
}
#[doc = "Cache Access Counter Interrupt raw register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_acs_cnt_int_raw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_acs_cnt_int_raw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CACHE_ACS_CNT_INT_RAW_SPEC;
impl crate::RegisterSpec for CACHE_ACS_CNT_INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cache_acs_cnt_int_raw::R`](R) reader structure"]
impl crate::Readable for CACHE_ACS_CNT_INT_RAW_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cache_acs_cnt_int_raw::W`](W) writer structure"]
impl crate::Writable for CACHE_ACS_CNT_INT_RAW_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CACHE_ACS_CNT_INT_RAW to value 0"]
impl crate::Resettable for CACHE_ACS_CNT_INT_RAW_SPEC {}
