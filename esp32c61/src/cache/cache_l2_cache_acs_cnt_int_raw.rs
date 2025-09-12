#[doc = "Register `CACHE_L2_CACHE_ACS_CNT_INT_RAW` reader"]
pub type R = crate::R<CACHE_L2_CACHE_ACS_CNT_INT_RAW_SPEC>;
#[doc = "Register `CACHE_L2_CACHE_ACS_CNT_INT_RAW` writer"]
pub type W = crate::W<CACHE_L2_CACHE_ACS_CNT_INT_RAW_SPEC>;
#[doc = "Field `CACHE_L2_IBUS0_OVF_INT_RAW` reader - The raw bit of the interrupt of one of counters overflow that occurs in L2-Cache due to bus0 accesses L2-ICache0."]
pub type CACHE_L2_IBUS0_OVF_INT_RAW_R = crate::BitReader;
#[doc = "Field `CACHE_L2_IBUS0_OVF_INT_RAW` writer - The raw bit of the interrupt of one of counters overflow that occurs in L2-Cache due to bus0 accesses L2-ICache0."]
pub type CACHE_L2_IBUS0_OVF_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CACHE_L2_IBUS1_OVF_INT_RAW` reader - The raw bit of the interrupt of one of counters overflow that occurs in L2-Cache due to bus1 accesses L2-ICache1."]
pub type CACHE_L2_IBUS1_OVF_INT_RAW_R = crate::BitReader;
#[doc = "Field `CACHE_L2_IBUS1_OVF_INT_RAW` writer - The raw bit of the interrupt of one of counters overflow that occurs in L2-Cache due to bus1 accesses L2-ICache1."]
pub type CACHE_L2_IBUS1_OVF_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CACHE_L2_IBUS2_OVF_INT_RAW` reader - The raw bit of the interrupt of one of counters overflow that occurs in L2-Cache due to bus2 accesses L2-ICache2."]
pub type CACHE_L2_IBUS2_OVF_INT_RAW_R = crate::BitReader;
#[doc = "Field `CACHE_L2_IBUS2_OVF_INT_RAW` writer - The raw bit of the interrupt of one of counters overflow that occurs in L2-Cache due to bus2 accesses L2-ICache2."]
pub type CACHE_L2_IBUS2_OVF_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CACHE_L2_IBUS3_OVF_INT_RAW` reader - The raw bit of the interrupt of one of counters overflow that occurs in L2-Cache due to bus3 accesses L2-ICache3."]
pub type CACHE_L2_IBUS3_OVF_INT_RAW_R = crate::BitReader;
#[doc = "Field `CACHE_L2_IBUS3_OVF_INT_RAW` writer - The raw bit of the interrupt of one of counters overflow that occurs in L2-Cache due to bus3 accesses L2-ICache3."]
pub type CACHE_L2_IBUS3_OVF_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CACHE_L2_DBUS0_OVF_INT_RAW` reader - The raw bit of the interrupt of one of counters overflow that occurs in L2-Cache due to bus0 accesses L2-DCache."]
pub type CACHE_L2_DBUS0_OVF_INT_RAW_R = crate::BitReader;
#[doc = "Field `CACHE_L2_DBUS0_OVF_INT_RAW` writer - The raw bit of the interrupt of one of counters overflow that occurs in L2-Cache due to bus0 accesses L2-DCache."]
pub type CACHE_L2_DBUS0_OVF_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CACHE_L2_DBUS1_OVF_INT_RAW` reader - The raw bit of the interrupt of one of counters overflow that occurs in L2-Cache due to bus1 accesses L2-DCache."]
pub type CACHE_L2_DBUS1_OVF_INT_RAW_R = crate::BitReader;
#[doc = "Field `CACHE_L2_DBUS1_OVF_INT_RAW` writer - The raw bit of the interrupt of one of counters overflow that occurs in L2-Cache due to bus1 accesses L2-DCache."]
pub type CACHE_L2_DBUS1_OVF_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CACHE_L2_DBUS2_OVF_INT_RAW` reader - The raw bit of the interrupt of one of counters overflow that occurs in L2-Cache due to bus2 accesses L2-DCache."]
pub type CACHE_L2_DBUS2_OVF_INT_RAW_R = crate::BitReader;
#[doc = "Field `CACHE_L2_DBUS2_OVF_INT_RAW` writer - The raw bit of the interrupt of one of counters overflow that occurs in L2-Cache due to bus2 accesses L2-DCache."]
pub type CACHE_L2_DBUS2_OVF_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CACHE_L2_DBUS3_OVF_INT_RAW` reader - The raw bit of the interrupt of one of counters overflow that occurs in L2-Cache due to bus3 accesses L2-DCache."]
pub type CACHE_L2_DBUS3_OVF_INT_RAW_R = crate::BitReader;
#[doc = "Field `CACHE_L2_DBUS3_OVF_INT_RAW` writer - The raw bit of the interrupt of one of counters overflow that occurs in L2-Cache due to bus3 accesses L2-DCache."]
pub type CACHE_L2_DBUS3_OVF_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 8 - The raw bit of the interrupt of one of counters overflow that occurs in L2-Cache due to bus0 accesses L2-ICache0."]
    #[inline(always)]
    pub fn cache_l2_ibus0_ovf_int_raw(&self) -> CACHE_L2_IBUS0_OVF_INT_RAW_R {
        CACHE_L2_IBUS0_OVF_INT_RAW_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The raw bit of the interrupt of one of counters overflow that occurs in L2-Cache due to bus1 accesses L2-ICache1."]
    #[inline(always)]
    pub fn cache_l2_ibus1_ovf_int_raw(&self) -> CACHE_L2_IBUS1_OVF_INT_RAW_R {
        CACHE_L2_IBUS1_OVF_INT_RAW_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - The raw bit of the interrupt of one of counters overflow that occurs in L2-Cache due to bus2 accesses L2-ICache2."]
    #[inline(always)]
    pub fn cache_l2_ibus2_ovf_int_raw(&self) -> CACHE_L2_IBUS2_OVF_INT_RAW_R {
        CACHE_L2_IBUS2_OVF_INT_RAW_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The raw bit of the interrupt of one of counters overflow that occurs in L2-Cache due to bus3 accesses L2-ICache3."]
    #[inline(always)]
    pub fn cache_l2_ibus3_ovf_int_raw(&self) -> CACHE_L2_IBUS3_OVF_INT_RAW_R {
        CACHE_L2_IBUS3_OVF_INT_RAW_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - The raw bit of the interrupt of one of counters overflow that occurs in L2-Cache due to bus0 accesses L2-DCache."]
    #[inline(always)]
    pub fn cache_l2_dbus0_ovf_int_raw(&self) -> CACHE_L2_DBUS0_OVF_INT_RAW_R {
        CACHE_L2_DBUS0_OVF_INT_RAW_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - The raw bit of the interrupt of one of counters overflow that occurs in L2-Cache due to bus1 accesses L2-DCache."]
    #[inline(always)]
    pub fn cache_l2_dbus1_ovf_int_raw(&self) -> CACHE_L2_DBUS1_OVF_INT_RAW_R {
        CACHE_L2_DBUS1_OVF_INT_RAW_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - The raw bit of the interrupt of one of counters overflow that occurs in L2-Cache due to bus2 accesses L2-DCache."]
    #[inline(always)]
    pub fn cache_l2_dbus2_ovf_int_raw(&self) -> CACHE_L2_DBUS2_OVF_INT_RAW_R {
        CACHE_L2_DBUS2_OVF_INT_RAW_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - The raw bit of the interrupt of one of counters overflow that occurs in L2-Cache due to bus3 accesses L2-DCache."]
    #[inline(always)]
    pub fn cache_l2_dbus3_ovf_int_raw(&self) -> CACHE_L2_DBUS3_OVF_INT_RAW_R {
        CACHE_L2_DBUS3_OVF_INT_RAW_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_L2_CACHE_ACS_CNT_INT_RAW")
            .field(
                "cache_l2_ibus0_ovf_int_raw",
                &self.cache_l2_ibus0_ovf_int_raw(),
            )
            .field(
                "cache_l2_ibus1_ovf_int_raw",
                &self.cache_l2_ibus1_ovf_int_raw(),
            )
            .field(
                "cache_l2_ibus2_ovf_int_raw",
                &self.cache_l2_ibus2_ovf_int_raw(),
            )
            .field(
                "cache_l2_ibus3_ovf_int_raw",
                &self.cache_l2_ibus3_ovf_int_raw(),
            )
            .field(
                "cache_l2_dbus0_ovf_int_raw",
                &self.cache_l2_dbus0_ovf_int_raw(),
            )
            .field(
                "cache_l2_dbus1_ovf_int_raw",
                &self.cache_l2_dbus1_ovf_int_raw(),
            )
            .field(
                "cache_l2_dbus2_ovf_int_raw",
                &self.cache_l2_dbus2_ovf_int_raw(),
            )
            .field(
                "cache_l2_dbus3_ovf_int_raw",
                &self.cache_l2_dbus3_ovf_int_raw(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 8 - The raw bit of the interrupt of one of counters overflow that occurs in L2-Cache due to bus0 accesses L2-ICache0."]
    #[inline(always)]
    pub fn cache_l2_ibus0_ovf_int_raw(
        &mut self,
    ) -> CACHE_L2_IBUS0_OVF_INT_RAW_W<'_, CACHE_L2_CACHE_ACS_CNT_INT_RAW_SPEC> {
        CACHE_L2_IBUS0_OVF_INT_RAW_W::new(self, 8)
    }
    #[doc = "Bit 9 - The raw bit of the interrupt of one of counters overflow that occurs in L2-Cache due to bus1 accesses L2-ICache1."]
    #[inline(always)]
    pub fn cache_l2_ibus1_ovf_int_raw(
        &mut self,
    ) -> CACHE_L2_IBUS1_OVF_INT_RAW_W<'_, CACHE_L2_CACHE_ACS_CNT_INT_RAW_SPEC> {
        CACHE_L2_IBUS1_OVF_INT_RAW_W::new(self, 9)
    }
    #[doc = "Bit 10 - The raw bit of the interrupt of one of counters overflow that occurs in L2-Cache due to bus2 accesses L2-ICache2."]
    #[inline(always)]
    pub fn cache_l2_ibus2_ovf_int_raw(
        &mut self,
    ) -> CACHE_L2_IBUS2_OVF_INT_RAW_W<'_, CACHE_L2_CACHE_ACS_CNT_INT_RAW_SPEC> {
        CACHE_L2_IBUS2_OVF_INT_RAW_W::new(self, 10)
    }
    #[doc = "Bit 11 - The raw bit of the interrupt of one of counters overflow that occurs in L2-Cache due to bus3 accesses L2-ICache3."]
    #[inline(always)]
    pub fn cache_l2_ibus3_ovf_int_raw(
        &mut self,
    ) -> CACHE_L2_IBUS3_OVF_INT_RAW_W<'_, CACHE_L2_CACHE_ACS_CNT_INT_RAW_SPEC> {
        CACHE_L2_IBUS3_OVF_INT_RAW_W::new(self, 11)
    }
    #[doc = "Bit 12 - The raw bit of the interrupt of one of counters overflow that occurs in L2-Cache due to bus0 accesses L2-DCache."]
    #[inline(always)]
    pub fn cache_l2_dbus0_ovf_int_raw(
        &mut self,
    ) -> CACHE_L2_DBUS0_OVF_INT_RAW_W<'_, CACHE_L2_CACHE_ACS_CNT_INT_RAW_SPEC> {
        CACHE_L2_DBUS0_OVF_INT_RAW_W::new(self, 12)
    }
    #[doc = "Bit 13 - The raw bit of the interrupt of one of counters overflow that occurs in L2-Cache due to bus1 accesses L2-DCache."]
    #[inline(always)]
    pub fn cache_l2_dbus1_ovf_int_raw(
        &mut self,
    ) -> CACHE_L2_DBUS1_OVF_INT_RAW_W<'_, CACHE_L2_CACHE_ACS_CNT_INT_RAW_SPEC> {
        CACHE_L2_DBUS1_OVF_INT_RAW_W::new(self, 13)
    }
    #[doc = "Bit 14 - The raw bit of the interrupt of one of counters overflow that occurs in L2-Cache due to bus2 accesses L2-DCache."]
    #[inline(always)]
    pub fn cache_l2_dbus2_ovf_int_raw(
        &mut self,
    ) -> CACHE_L2_DBUS2_OVF_INT_RAW_W<'_, CACHE_L2_CACHE_ACS_CNT_INT_RAW_SPEC> {
        CACHE_L2_DBUS2_OVF_INT_RAW_W::new(self, 14)
    }
    #[doc = "Bit 15 - The raw bit of the interrupt of one of counters overflow that occurs in L2-Cache due to bus3 accesses L2-DCache."]
    #[inline(always)]
    pub fn cache_l2_dbus3_ovf_int_raw(
        &mut self,
    ) -> CACHE_L2_DBUS3_OVF_INT_RAW_W<'_, CACHE_L2_CACHE_ACS_CNT_INT_RAW_SPEC> {
        CACHE_L2_DBUS3_OVF_INT_RAW_W::new(self, 15)
    }
}
#[doc = "Cache Access Counter Interrupt raw register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_l2_cache_acs_cnt_int_raw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_l2_cache_acs_cnt_int_raw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CACHE_L2_CACHE_ACS_CNT_INT_RAW_SPEC;
impl crate::RegisterSpec for CACHE_L2_CACHE_ACS_CNT_INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cache_l2_cache_acs_cnt_int_raw::R`](R) reader structure"]
impl crate::Readable for CACHE_L2_CACHE_ACS_CNT_INT_RAW_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cache_l2_cache_acs_cnt_int_raw::W`](W) writer structure"]
impl crate::Writable for CACHE_L2_CACHE_ACS_CNT_INT_RAW_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CACHE_L2_CACHE_ACS_CNT_INT_RAW to value 0"]
impl crate::Resettable for CACHE_L2_CACHE_ACS_CNT_INT_RAW_SPEC {}
