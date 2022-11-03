#[doc = "Register `L2_CACHE_ACS_CNT_INT_RAW` reader"]
pub struct R(crate::R<L2_CACHE_ACS_CNT_INT_RAW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<L2_CACHE_ACS_CNT_INT_RAW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<L2_CACHE_ACS_CNT_INT_RAW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<L2_CACHE_ACS_CNT_INT_RAW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `L2_IBUS0_OVF_INT_RAW` reader - The raw bit of the interrupt of one of counters overflow that occurs in L2-Cache due to bus0 accesses L2-ICache0."]
pub type L2_IBUS0_OVF_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `L2_IBUS1_OVF_INT_RAW` reader - The raw bit of the interrupt of one of counters overflow that occurs in L2-Cache due to bus1 accesses L2-ICache1."]
pub type L2_IBUS1_OVF_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `L2_IBUS2_OVF_INT_RAW` reader - The raw bit of the interrupt of one of counters overflow that occurs in L2-Cache due to bus2 accesses L2-ICache2."]
pub type L2_IBUS2_OVF_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `L2_IBUS3_OVF_INT_RAW` reader - The raw bit of the interrupt of one of counters overflow that occurs in L2-Cache due to bus3 accesses L2-ICache3."]
pub type L2_IBUS3_OVF_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `L2_DBUS0_OVF_INT_RAW` reader - The raw bit of the interrupt of one of counters overflow that occurs in L2-Cache due to bus0 accesses L2-DCache."]
pub type L2_DBUS0_OVF_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `L2_DBUS1_OVF_INT_RAW` reader - The raw bit of the interrupt of one of counters overflow that occurs in L2-Cache due to bus1 accesses L2-DCache."]
pub type L2_DBUS1_OVF_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `L2_DBUS2_OVF_INT_RAW` reader - The raw bit of the interrupt of one of counters overflow that occurs in L2-Cache due to bus2 accesses L2-DCache."]
pub type L2_DBUS2_OVF_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `L2_DBUS3_OVF_INT_RAW` reader - The raw bit of the interrupt of one of counters overflow that occurs in L2-Cache due to bus3 accesses L2-DCache."]
pub type L2_DBUS3_OVF_INT_RAW_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 8 - The raw bit of the interrupt of one of counters overflow that occurs in L2-Cache due to bus0 accesses L2-ICache0."]
    #[inline(always)]
    pub fn l2_ibus0_ovf_int_raw(&self) -> L2_IBUS0_OVF_INT_RAW_R {
        L2_IBUS0_OVF_INT_RAW_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The raw bit of the interrupt of one of counters overflow that occurs in L2-Cache due to bus1 accesses L2-ICache1."]
    #[inline(always)]
    pub fn l2_ibus1_ovf_int_raw(&self) -> L2_IBUS1_OVF_INT_RAW_R {
        L2_IBUS1_OVF_INT_RAW_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - The raw bit of the interrupt of one of counters overflow that occurs in L2-Cache due to bus2 accesses L2-ICache2."]
    #[inline(always)]
    pub fn l2_ibus2_ovf_int_raw(&self) -> L2_IBUS2_OVF_INT_RAW_R {
        L2_IBUS2_OVF_INT_RAW_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The raw bit of the interrupt of one of counters overflow that occurs in L2-Cache due to bus3 accesses L2-ICache3."]
    #[inline(always)]
    pub fn l2_ibus3_ovf_int_raw(&self) -> L2_IBUS3_OVF_INT_RAW_R {
        L2_IBUS3_OVF_INT_RAW_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - The raw bit of the interrupt of one of counters overflow that occurs in L2-Cache due to bus0 accesses L2-DCache."]
    #[inline(always)]
    pub fn l2_dbus0_ovf_int_raw(&self) -> L2_DBUS0_OVF_INT_RAW_R {
        L2_DBUS0_OVF_INT_RAW_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - The raw bit of the interrupt of one of counters overflow that occurs in L2-Cache due to bus1 accesses L2-DCache."]
    #[inline(always)]
    pub fn l2_dbus1_ovf_int_raw(&self) -> L2_DBUS1_OVF_INT_RAW_R {
        L2_DBUS1_OVF_INT_RAW_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - The raw bit of the interrupt of one of counters overflow that occurs in L2-Cache due to bus2 accesses L2-DCache."]
    #[inline(always)]
    pub fn l2_dbus2_ovf_int_raw(&self) -> L2_DBUS2_OVF_INT_RAW_R {
        L2_DBUS2_OVF_INT_RAW_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - The raw bit of the interrupt of one of counters overflow that occurs in L2-Cache due to bus3 accesses L2-DCache."]
    #[inline(always)]
    pub fn l2_dbus3_ovf_int_raw(&self) -> L2_DBUS3_OVF_INT_RAW_R {
        L2_DBUS3_OVF_INT_RAW_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "Cache Access Counter Interrupt raw register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l2_cache_acs_cnt_int_raw](index.html) module"]
pub struct L2_CACHE_ACS_CNT_INT_RAW_SPEC;
impl crate::RegisterSpec for L2_CACHE_ACS_CNT_INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [l2_cache_acs_cnt_int_raw::R](R) reader structure"]
impl crate::Readable for L2_CACHE_ACS_CNT_INT_RAW_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets L2_CACHE_ACS_CNT_INT_RAW to value 0"]
impl crate::Resettable for L2_CACHE_ACS_CNT_INT_RAW_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
