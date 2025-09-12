#[doc = "Register `CACHE_ACS_CNT_INT_ENA` reader"]
pub type R = crate::R<CACHE_ACS_CNT_INT_ENA_SPEC>;
#[doc = "Register `CACHE_ACS_CNT_INT_ENA` writer"]
pub type W = crate::W<CACHE_ACS_CNT_INT_ENA_SPEC>;
#[doc = "Field `IBUS0_OVF_INT_ENA` reader - The bit is used to enable interrupt of one of counters overflow that occurs in L1-ICache0 due to bus0 accesses L1-ICache0."]
pub type IBUS0_OVF_INT_ENA_R = crate::BitReader;
#[doc = "Field `IBUS1_OVF_INT_ENA` reader - The bit is used to enable interrupt of one of counters overflow that occurs in L1-ICache1 due to bus1 accesses L1-ICache1."]
pub type IBUS1_OVF_INT_ENA_R = crate::BitReader;
#[doc = "Field `IBUS2_OVF_INT_ENA` reader - Reserved"]
pub type IBUS2_OVF_INT_ENA_R = crate::BitReader;
#[doc = "Field `IBUS3_OVF_INT_ENA` reader - Reserved"]
pub type IBUS3_OVF_INT_ENA_R = crate::BitReader;
#[doc = "Field `BUS0_OVF_INT_ENA` reader - The bit is used to enable interrupt of one of counters overflow that occurs in L1-DCache due to bus0 accesses L1-DCache."]
pub type BUS0_OVF_INT_ENA_R = crate::BitReader;
#[doc = "Field `BUS0_OVF_INT_ENA` writer - The bit is used to enable interrupt of one of counters overflow that occurs in L1-DCache due to bus0 accesses L1-DCache."]
pub type BUS0_OVF_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUS1_OVF_INT_ENA` reader - The bit is used to enable interrupt of one of counters overflow that occurs in L1-DCache due to bus1 accesses L1-DCache."]
pub type BUS1_OVF_INT_ENA_R = crate::BitReader;
#[doc = "Field `BUS1_OVF_INT_ENA` writer - The bit is used to enable interrupt of one of counters overflow that occurs in L1-DCache due to bus1 accesses L1-DCache."]
pub type BUS1_OVF_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBUS2_OVF_INT_ENA` reader - Reserved"]
pub type DBUS2_OVF_INT_ENA_R = crate::BitReader;
#[doc = "Field `DBUS3_OVF_INT_ENA` reader - Reserved"]
pub type DBUS3_OVF_INT_ENA_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The bit is used to enable interrupt of one of counters overflow that occurs in L1-ICache0 due to bus0 accesses L1-ICache0."]
    #[inline(always)]
    pub fn ibus0_ovf_int_ena(&self) -> IBUS0_OVF_INT_ENA_R {
        IBUS0_OVF_INT_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The bit is used to enable interrupt of one of counters overflow that occurs in L1-ICache1 due to bus1 accesses L1-ICache1."]
    #[inline(always)]
    pub fn ibus1_ovf_int_ena(&self) -> IBUS1_OVF_INT_ENA_R {
        IBUS1_OVF_INT_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn ibus2_ovf_int_ena(&self) -> IBUS2_OVF_INT_ENA_R {
        IBUS2_OVF_INT_ENA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn ibus3_ovf_int_ena(&self) -> IBUS3_OVF_INT_ENA_R {
        IBUS3_OVF_INT_ENA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The bit is used to enable interrupt of one of counters overflow that occurs in L1-DCache due to bus0 accesses L1-DCache."]
    #[inline(always)]
    pub fn bus0_ovf_int_ena(&self) -> BUS0_OVF_INT_ENA_R {
        BUS0_OVF_INT_ENA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The bit is used to enable interrupt of one of counters overflow that occurs in L1-DCache due to bus1 accesses L1-DCache."]
    #[inline(always)]
    pub fn bus1_ovf_int_ena(&self) -> BUS1_OVF_INT_ENA_R {
        BUS1_OVF_INT_ENA_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Reserved"]
    #[inline(always)]
    pub fn dbus2_ovf_int_ena(&self) -> DBUS2_OVF_INT_ENA_R {
        DBUS2_OVF_INT_ENA_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Reserved"]
    #[inline(always)]
    pub fn dbus3_ovf_int_ena(&self) -> DBUS3_OVF_INT_ENA_R {
        DBUS3_OVF_INT_ENA_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_ACS_CNT_INT_ENA")
            .field("ibus0_ovf_int_ena", &self.ibus0_ovf_int_ena())
            .field("ibus1_ovf_int_ena", &self.ibus1_ovf_int_ena())
            .field("ibus2_ovf_int_ena", &self.ibus2_ovf_int_ena())
            .field("ibus3_ovf_int_ena", &self.ibus3_ovf_int_ena())
            .field("bus0_ovf_int_ena", &self.bus0_ovf_int_ena())
            .field("bus1_ovf_int_ena", &self.bus1_ovf_int_ena())
            .field("dbus2_ovf_int_ena", &self.dbus2_ovf_int_ena())
            .field("dbus3_ovf_int_ena", &self.dbus3_ovf_int_ena())
            .finish()
    }
}
impl W {
    #[doc = "Bit 4 - The bit is used to enable interrupt of one of counters overflow that occurs in L1-DCache due to bus0 accesses L1-DCache."]
    #[inline(always)]
    pub fn bus0_ovf_int_ena(&mut self) -> BUS0_OVF_INT_ENA_W<'_, CACHE_ACS_CNT_INT_ENA_SPEC> {
        BUS0_OVF_INT_ENA_W::new(self, 4)
    }
    #[doc = "Bit 5 - The bit is used to enable interrupt of one of counters overflow that occurs in L1-DCache due to bus1 accesses L1-DCache."]
    #[inline(always)]
    pub fn bus1_ovf_int_ena(&mut self) -> BUS1_OVF_INT_ENA_W<'_, CACHE_ACS_CNT_INT_ENA_SPEC> {
        BUS1_OVF_INT_ENA_W::new(self, 5)
    }
}
#[doc = "Cache Access Counter Interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_acs_cnt_int_ena::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_acs_cnt_int_ena::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CACHE_ACS_CNT_INT_ENA_SPEC;
impl crate::RegisterSpec for CACHE_ACS_CNT_INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cache_acs_cnt_int_ena::R`](R) reader structure"]
impl crate::Readable for CACHE_ACS_CNT_INT_ENA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cache_acs_cnt_int_ena::W`](W) writer structure"]
impl crate::Writable for CACHE_ACS_CNT_INT_ENA_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CACHE_ACS_CNT_INT_ENA to value 0"]
impl crate::Resettable for CACHE_ACS_CNT_INT_ENA_SPEC {}
