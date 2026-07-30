#[doc = "Register `L1_CACHE_ACS_CNT_INT_ST` reader"]
pub type R = crate::R<L1_CACHE_ACS_CNT_INT_ST_SPEC>;
#[doc = "Field `L1_IBUS0_OVF_INT_ST` reader - The bit indicates the interrupt status of one of counters overflow that occurs in L1-ICache0 due to bus0 accesses L1-ICache0."]
pub type L1_IBUS0_OVF_INT_ST_R = crate::BitReader;
#[doc = "Field `L1_IBUS1_OVF_INT_ST` reader - The bit indicates the interrupt status of one of counters overflow that occurs in L1-ICache1 due to bus1 accesses L1-ICache1."]
pub type L1_IBUS1_OVF_INT_ST_R = crate::BitReader;
#[doc = "Field `L1_IBUS2_OVF_INT_ST` reader - Reserved"]
pub type L1_IBUS2_OVF_INT_ST_R = crate::BitReader;
#[doc = "Field `L1_IBUS3_OVF_INT_ST` reader - Reserved"]
pub type L1_IBUS3_OVF_INT_ST_R = crate::BitReader;
#[doc = "Field `L1_BUS0_OVF_INT_ST` reader - The masked interrupt status of L1_BUS0_OVF_INT."]
pub type L1_BUS0_OVF_INT_ST_R = crate::BitReader;
#[doc = "Field `L1_BUS1_OVF_INT_ST` reader - The masked interrupt status of L1_BUS1_OVF_INT."]
pub type L1_BUS1_OVF_INT_ST_R = crate::BitReader;
#[doc = "Field `L1_BUS2_OVF_INT_ST` reader - Reserved"]
pub type L1_BUS2_OVF_INT_ST_R = crate::BitReader;
#[doc = "Field `L1_BUS3_OVF_INT_ST` reader - Reserved"]
pub type L1_BUS3_OVF_INT_ST_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The bit indicates the interrupt status of one of counters overflow that occurs in L1-ICache0 due to bus0 accesses L1-ICache0."]
    #[inline(always)]
    pub fn l1_ibus0_ovf_int_st(&self) -> L1_IBUS0_OVF_INT_ST_R {
        L1_IBUS0_OVF_INT_ST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The bit indicates the interrupt status of one of counters overflow that occurs in L1-ICache1 due to bus1 accesses L1-ICache1."]
    #[inline(always)]
    pub fn l1_ibus1_ovf_int_st(&self) -> L1_IBUS1_OVF_INT_ST_R {
        L1_IBUS1_OVF_INT_ST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn l1_ibus2_ovf_int_st(&self) -> L1_IBUS2_OVF_INT_ST_R {
        L1_IBUS2_OVF_INT_ST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn l1_ibus3_ovf_int_st(&self) -> L1_IBUS3_OVF_INT_ST_R {
        L1_IBUS3_OVF_INT_ST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The masked interrupt status of L1_BUS0_OVF_INT."]
    #[inline(always)]
    pub fn l1_bus0_ovf_int_st(&self) -> L1_BUS0_OVF_INT_ST_R {
        L1_BUS0_OVF_INT_ST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The masked interrupt status of L1_BUS1_OVF_INT."]
    #[inline(always)]
    pub fn l1_bus1_ovf_int_st(&self) -> L1_BUS1_OVF_INT_ST_R {
        L1_BUS1_OVF_INT_ST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Reserved"]
    #[inline(always)]
    pub fn l1_bus2_ovf_int_st(&self) -> L1_BUS2_OVF_INT_ST_R {
        L1_BUS2_OVF_INT_ST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Reserved"]
    #[inline(always)]
    pub fn l1_bus3_ovf_int_st(&self) -> L1_BUS3_OVF_INT_ST_R {
        L1_BUS3_OVF_INT_ST_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L1_CACHE_ACS_CNT_INT_ST")
            .field("l1_ibus0_ovf_int_st", &self.l1_ibus0_ovf_int_st())
            .field("l1_ibus1_ovf_int_st", &self.l1_ibus1_ovf_int_st())
            .field("l1_ibus2_ovf_int_st", &self.l1_ibus2_ovf_int_st())
            .field("l1_ibus3_ovf_int_st", &self.l1_ibus3_ovf_int_st())
            .field("l1_bus0_ovf_int_st", &self.l1_bus0_ovf_int_st())
            .field("l1_bus1_ovf_int_st", &self.l1_bus1_ovf_int_st())
            .field("l1_bus2_ovf_int_st", &self.l1_bus2_ovf_int_st())
            .field("l1_bus3_ovf_int_st", &self.l1_bus3_ovf_int_st())
            .finish()
    }
}
#[doc = "Cache access counter interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`l1_cache_acs_cnt_int_st::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1_CACHE_ACS_CNT_INT_ST_SPEC;
impl crate::RegisterSpec for L1_CACHE_ACS_CNT_INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l1_cache_acs_cnt_int_st::R`](R) reader structure"]
impl crate::Readable for L1_CACHE_ACS_CNT_INT_ST_SPEC {}
#[doc = "`reset()` method sets L1_CACHE_ACS_CNT_INT_ST to value 0"]
impl crate::Resettable for L1_CACHE_ACS_CNT_INT_ST_SPEC {}
