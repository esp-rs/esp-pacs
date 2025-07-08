#[doc = "Register `CACHE_ACS_CNT_INT_ST` reader"]
pub type R = crate::R<CACHE_ACS_CNT_INT_ST_SPEC>;
#[doc = "Field `IBUS0_OVF_INT_ST` reader - The bit indicates the interrupt status of one of counters overflow that occurs in L1-ICache0 due to bus0 accesses L1-ICache0."]
pub type IBUS0_OVF_INT_ST_R = crate::BitReader;
#[doc = "Field `IBUS1_OVF_INT_ST` reader - The bit indicates the interrupt status of one of counters overflow that occurs in L1-ICache1 due to bus1 accesses L1-ICache1."]
pub type IBUS1_OVF_INT_ST_R = crate::BitReader;
#[doc = "Field `IBUS2_OVF_INT_ST` reader - Reserved"]
pub type IBUS2_OVF_INT_ST_R = crate::BitReader;
#[doc = "Field `IBUS3_OVF_INT_ST` reader - Reserved"]
pub type IBUS3_OVF_INT_ST_R = crate::BitReader;
#[doc = "Field `BUS0_OVF_INT_ST` reader - The bit indicates the interrupt status of one of counters overflow that occurs in L1-DCache due to bus0 accesses L1-DCache."]
pub type BUS0_OVF_INT_ST_R = crate::BitReader;
#[doc = "Field `BUS1_OVF_INT_ST` reader - The bit indicates the interrupt status of one of counters overflow that occurs in L1-DCache due to bus1 accesses L1-DCache."]
pub type BUS1_OVF_INT_ST_R = crate::BitReader;
#[doc = "Field `DBUS2_OVF_INT_ST` reader - Reserved"]
pub type DBUS2_OVF_INT_ST_R = crate::BitReader;
#[doc = "Field `DBUS3_OVF_INT_ST` reader - Reserved"]
pub type DBUS3_OVF_INT_ST_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The bit indicates the interrupt status of one of counters overflow that occurs in L1-ICache0 due to bus0 accesses L1-ICache0."]
    #[inline(always)]
    pub fn ibus0_ovf_int_st(&self) -> IBUS0_OVF_INT_ST_R {
        IBUS0_OVF_INT_ST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The bit indicates the interrupt status of one of counters overflow that occurs in L1-ICache1 due to bus1 accesses L1-ICache1."]
    #[inline(always)]
    pub fn ibus1_ovf_int_st(&self) -> IBUS1_OVF_INT_ST_R {
        IBUS1_OVF_INT_ST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn ibus2_ovf_int_st(&self) -> IBUS2_OVF_INT_ST_R {
        IBUS2_OVF_INT_ST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn ibus3_ovf_int_st(&self) -> IBUS3_OVF_INT_ST_R {
        IBUS3_OVF_INT_ST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The bit indicates the interrupt status of one of counters overflow that occurs in L1-DCache due to bus0 accesses L1-DCache."]
    #[inline(always)]
    pub fn bus0_ovf_int_st(&self) -> BUS0_OVF_INT_ST_R {
        BUS0_OVF_INT_ST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The bit indicates the interrupt status of one of counters overflow that occurs in L1-DCache due to bus1 accesses L1-DCache."]
    #[inline(always)]
    pub fn bus1_ovf_int_st(&self) -> BUS1_OVF_INT_ST_R {
        BUS1_OVF_INT_ST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Reserved"]
    #[inline(always)]
    pub fn dbus2_ovf_int_st(&self) -> DBUS2_OVF_INT_ST_R {
        DBUS2_OVF_INT_ST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Reserved"]
    #[inline(always)]
    pub fn dbus3_ovf_int_st(&self) -> DBUS3_OVF_INT_ST_R {
        DBUS3_OVF_INT_ST_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_ACS_CNT_INT_ST")
            .field("ibus0_ovf_int_st", &self.ibus0_ovf_int_st())
            .field("ibus1_ovf_int_st", &self.ibus1_ovf_int_st())
            .field("ibus2_ovf_int_st", &self.ibus2_ovf_int_st())
            .field("ibus3_ovf_int_st", &self.ibus3_ovf_int_st())
            .field("bus0_ovf_int_st", &self.bus0_ovf_int_st())
            .field("bus1_ovf_int_st", &self.bus1_ovf_int_st())
            .field("dbus2_ovf_int_st", &self.dbus2_ovf_int_st())
            .field("dbus3_ovf_int_st", &self.dbus3_ovf_int_st())
            .finish()
    }
}
#[doc = "Cache Access Counter Interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_acs_cnt_int_st::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CACHE_ACS_CNT_INT_ST_SPEC;
impl crate::RegisterSpec for CACHE_ACS_CNT_INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cache_acs_cnt_int_st::R`](R) reader structure"]
impl crate::Readable for CACHE_ACS_CNT_INT_ST_SPEC {}
#[doc = "`reset()` method sets CACHE_ACS_CNT_INT_ST to value 0"]
impl crate::Resettable for CACHE_ACS_CNT_INT_ST_SPEC {}
