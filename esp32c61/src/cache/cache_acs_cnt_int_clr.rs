#[doc = "Register `CACHE_ACS_CNT_INT_CLR` reader"]
pub type R = crate::R<CACHE_ACS_CNT_INT_CLR_SPEC>;
#[doc = "Register `CACHE_ACS_CNT_INT_CLR` writer"]
pub type W = crate::W<CACHE_ACS_CNT_INT_CLR_SPEC>;
#[doc = "Field `IBUS0_OVF_INT_CLR` reader - The bit is used to clear counters overflow interrupt and counters in L1-ICache0 due to bus0 accesses L1-ICache0."]
pub type IBUS0_OVF_INT_CLR_R = crate::BitReader;
#[doc = "Field `IBUS1_OVF_INT_CLR` reader - The bit is used to clear counters overflow interrupt and counters in L1-ICache1 due to bus1 accesses L1-ICache1."]
pub type IBUS1_OVF_INT_CLR_R = crate::BitReader;
#[doc = "Field `IBUS2_OVF_INT_CLR` reader - Reserved"]
pub type IBUS2_OVF_INT_CLR_R = crate::BitReader;
#[doc = "Field `IBUS3_OVF_INT_CLR` reader - Reserved"]
pub type IBUS3_OVF_INT_CLR_R = crate::BitReader;
#[doc = "Field `BUS0_OVF_INT_CLR` writer - The bit is used to clear counters overflow interrupt and counters in L1-DCache due to bus0 accesses L1-DCache."]
pub type BUS0_OVF_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUS1_OVF_INT_CLR` writer - The bit is used to clear counters overflow interrupt and counters in L1-DCache due to bus1 accesses L1-DCache."]
pub type BUS1_OVF_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBUS2_OVF_INT_CLR` reader - Reserved"]
pub type DBUS2_OVF_INT_CLR_R = crate::BitReader;
#[doc = "Field `DBUS2_OVF_INT_CLR` writer - Reserved"]
pub type DBUS2_OVF_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBUS3_OVF_INT_CLR` reader - Reserved"]
pub type DBUS3_OVF_INT_CLR_R = crate::BitReader;
#[doc = "Field `DBUS3_OVF_INT_CLR` writer - Reserved"]
pub type DBUS3_OVF_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The bit is used to clear counters overflow interrupt and counters in L1-ICache0 due to bus0 accesses L1-ICache0."]
    #[inline(always)]
    pub fn ibus0_ovf_int_clr(&self) -> IBUS0_OVF_INT_CLR_R {
        IBUS0_OVF_INT_CLR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The bit is used to clear counters overflow interrupt and counters in L1-ICache1 due to bus1 accesses L1-ICache1."]
    #[inline(always)]
    pub fn ibus1_ovf_int_clr(&self) -> IBUS1_OVF_INT_CLR_R {
        IBUS1_OVF_INT_CLR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn ibus2_ovf_int_clr(&self) -> IBUS2_OVF_INT_CLR_R {
        IBUS2_OVF_INT_CLR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn ibus3_ovf_int_clr(&self) -> IBUS3_OVF_INT_CLR_R {
        IBUS3_OVF_INT_CLR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - Reserved"]
    #[inline(always)]
    pub fn dbus2_ovf_int_clr(&self) -> DBUS2_OVF_INT_CLR_R {
        DBUS2_OVF_INT_CLR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Reserved"]
    #[inline(always)]
    pub fn dbus3_ovf_int_clr(&self) -> DBUS3_OVF_INT_CLR_R {
        DBUS3_OVF_INT_CLR_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_ACS_CNT_INT_CLR")
            .field("ibus0_ovf_int_clr", &self.ibus0_ovf_int_clr())
            .field("ibus1_ovf_int_clr", &self.ibus1_ovf_int_clr())
            .field("ibus2_ovf_int_clr", &self.ibus2_ovf_int_clr())
            .field("ibus3_ovf_int_clr", &self.ibus3_ovf_int_clr())
            .field("dbus2_ovf_int_clr", &self.dbus2_ovf_int_clr())
            .field("dbus3_ovf_int_clr", &self.dbus3_ovf_int_clr())
            .finish()
    }
}
impl W {
    #[doc = "Bit 4 - The bit is used to clear counters overflow interrupt and counters in L1-DCache due to bus0 accesses L1-DCache."]
    #[inline(always)]
    pub fn bus0_ovf_int_clr(&mut self) -> BUS0_OVF_INT_CLR_W<'_, CACHE_ACS_CNT_INT_CLR_SPEC> {
        BUS0_OVF_INT_CLR_W::new(self, 4)
    }
    #[doc = "Bit 5 - The bit is used to clear counters overflow interrupt and counters in L1-DCache due to bus1 accesses L1-DCache."]
    #[inline(always)]
    pub fn bus1_ovf_int_clr(&mut self) -> BUS1_OVF_INT_CLR_W<'_, CACHE_ACS_CNT_INT_CLR_SPEC> {
        BUS1_OVF_INT_CLR_W::new(self, 5)
    }
    #[doc = "Bit 6 - Reserved"]
    #[inline(always)]
    pub fn dbus2_ovf_int_clr(&mut self) -> DBUS2_OVF_INT_CLR_W<'_, CACHE_ACS_CNT_INT_CLR_SPEC> {
        DBUS2_OVF_INT_CLR_W::new(self, 6)
    }
    #[doc = "Bit 7 - Reserved"]
    #[inline(always)]
    pub fn dbus3_ovf_int_clr(&mut self) -> DBUS3_OVF_INT_CLR_W<'_, CACHE_ACS_CNT_INT_CLR_SPEC> {
        DBUS3_OVF_INT_CLR_W::new(self, 7)
    }
}
#[doc = "Cache Access Counter Interrupt clear register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_acs_cnt_int_clr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_acs_cnt_int_clr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CACHE_ACS_CNT_INT_CLR_SPEC;
impl crate::RegisterSpec for CACHE_ACS_CNT_INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cache_acs_cnt_int_clr::R`](R) reader structure"]
impl crate::Readable for CACHE_ACS_CNT_INT_CLR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cache_acs_cnt_int_clr::W`](W) writer structure"]
impl crate::Writable for CACHE_ACS_CNT_INT_CLR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CACHE_ACS_CNT_INT_CLR to value 0"]
impl crate::Resettable for CACHE_ACS_CNT_INT_CLR_SPEC {}
