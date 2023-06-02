#[doc = "Register `L1_CACHE_ACS_CNT_INT_ST` reader"]
pub struct R(crate::R<L1_CACHE_ACS_CNT_INT_ST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<L1_CACHE_ACS_CNT_INT_ST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<L1_CACHE_ACS_CNT_INT_ST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<L1_CACHE_ACS_CNT_INT_ST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `L1_IBUS0_OVF_INT_ST` reader - The bit indicates the interrupt status of one of counters overflow that occurs in L1-ICache0 due to bus0 accesses L1-ICache0."]
pub type L1_IBUS0_OVF_INT_ST_R = crate::BitReader;
#[doc = "Field `L1_IBUS1_OVF_INT_ST` reader - The bit indicates the interrupt status of one of counters overflow that occurs in L1-ICache1 due to bus1 accesses L1-ICache1."]
pub type L1_IBUS1_OVF_INT_ST_R = crate::BitReader;
#[doc = "Field `L1_IBUS2_OVF_INT_ST` reader - Reserved"]
pub type L1_IBUS2_OVF_INT_ST_R = crate::BitReader;
#[doc = "Field `L1_IBUS3_OVF_INT_ST` reader - Reserved"]
pub type L1_IBUS3_OVF_INT_ST_R = crate::BitReader;
#[doc = "Field `L1_BUS0_OVF_INT_ST` reader - The bit indicates the interrupt status of one of counters overflow that occurs in L1-DCache due to bus0 accesses L1-DCache."]
pub type L1_BUS0_OVF_INT_ST_R = crate::BitReader;
#[doc = "Field `L1_BUS1_OVF_INT_ST` reader - The bit indicates the interrupt status of one of counters overflow that occurs in L1-DCache due to bus1 accesses L1-DCache."]
pub type L1_BUS1_OVF_INT_ST_R = crate::BitReader;
#[doc = "Field `L1_DBUS2_OVF_INT_ST` reader - Reserved"]
pub type L1_DBUS2_OVF_INT_ST_R = crate::BitReader;
#[doc = "Field `L1_DBUS3_OVF_INT_ST` reader - Reserved"]
pub type L1_DBUS3_OVF_INT_ST_R = crate::BitReader;
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
    #[doc = "Bit 4 - The bit indicates the interrupt status of one of counters overflow that occurs in L1-DCache due to bus0 accesses L1-DCache."]
    #[inline(always)]
    pub fn l1_bus0_ovf_int_st(&self) -> L1_BUS0_OVF_INT_ST_R {
        L1_BUS0_OVF_INT_ST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The bit indicates the interrupt status of one of counters overflow that occurs in L1-DCache due to bus1 accesses L1-DCache."]
    #[inline(always)]
    pub fn l1_bus1_ovf_int_st(&self) -> L1_BUS1_OVF_INT_ST_R {
        L1_BUS1_OVF_INT_ST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Reserved"]
    #[inline(always)]
    pub fn l1_dbus2_ovf_int_st(&self) -> L1_DBUS2_OVF_INT_ST_R {
        L1_DBUS2_OVF_INT_ST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Reserved"]
    #[inline(always)]
    pub fn l1_dbus3_ovf_int_st(&self) -> L1_DBUS3_OVF_INT_ST_R {
        L1_DBUS3_OVF_INT_ST_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L1_CACHE_ACS_CNT_INT_ST")
            .field(
                "l1_ibus0_ovf_int_st",
                &format_args!("{}", self.l1_ibus0_ovf_int_st().bit()),
            )
            .field(
                "l1_ibus1_ovf_int_st",
                &format_args!("{}", self.l1_ibus1_ovf_int_st().bit()),
            )
            .field(
                "l1_ibus2_ovf_int_st",
                &format_args!("{}", self.l1_ibus2_ovf_int_st().bit()),
            )
            .field(
                "l1_ibus3_ovf_int_st",
                &format_args!("{}", self.l1_ibus3_ovf_int_st().bit()),
            )
            .field(
                "l1_bus0_ovf_int_st",
                &format_args!("{}", self.l1_bus0_ovf_int_st().bit()),
            )
            .field(
                "l1_bus1_ovf_int_st",
                &format_args!("{}", self.l1_bus1_ovf_int_st().bit()),
            )
            .field(
                "l1_dbus2_ovf_int_st",
                &format_args!("{}", self.l1_dbus2_ovf_int_st().bit()),
            )
            .field(
                "l1_dbus3_ovf_int_st",
                &format_args!("{}", self.l1_dbus3_ovf_int_st().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<L1_CACHE_ACS_CNT_INT_ST_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Cache Access Counter Interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l1_cache_acs_cnt_int_st](index.html) module"]
pub struct L1_CACHE_ACS_CNT_INT_ST_SPEC;
impl crate::RegisterSpec for L1_CACHE_ACS_CNT_INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [l1_cache_acs_cnt_int_st::R](R) reader structure"]
impl crate::Readable for L1_CACHE_ACS_CNT_INT_ST_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets L1_CACHE_ACS_CNT_INT_ST to value 0"]
impl crate::Resettable for L1_CACHE_ACS_CNT_INT_ST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
