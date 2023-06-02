#[doc = "Register `L2_CACHE_ACS_CNT_INT_ST` reader"]
pub struct R(crate::R<L2_CACHE_ACS_CNT_INT_ST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<L2_CACHE_ACS_CNT_INT_ST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<L2_CACHE_ACS_CNT_INT_ST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<L2_CACHE_ACS_CNT_INT_ST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `L2_IBUS0_OVF_INT_ST` reader - The bit indicates the interrupt status of one of counters overflow that occurs in L2-Cache due to bus0 accesses L2-Cache."]
pub type L2_IBUS0_OVF_INT_ST_R = crate::BitReader;
#[doc = "Field `L2_IBUS1_OVF_INT_ST` reader - The bit indicates the interrupt status of one of counters overflow that occurs in L2-Cache due to bus1 accesses L2-Cache."]
pub type L2_IBUS1_OVF_INT_ST_R = crate::BitReader;
#[doc = "Field `L2_IBUS2_OVF_INT_ST` reader - Reserved"]
pub type L2_IBUS2_OVF_INT_ST_R = crate::BitReader;
#[doc = "Field `L2_IBUS3_OVF_INT_ST` reader - Reserved"]
pub type L2_IBUS3_OVF_INT_ST_R = crate::BitReader;
#[doc = "Field `L2_DBUS0_OVF_INT_ST` reader - The bit indicates the interrupt status of one of counters overflow that occurs in L2-Cache due to bus0 accesses L2-Cache."]
pub type L2_DBUS0_OVF_INT_ST_R = crate::BitReader;
#[doc = "Field `L2_DBUS1_OVF_INT_ST` reader - The bit indicates the interrupt status of one of counters overflow that occurs in L2-Cache due to bus1 accesses L2-Cache."]
pub type L2_DBUS1_OVF_INT_ST_R = crate::BitReader;
#[doc = "Field `L2_DBUS2_OVF_INT_ST` reader - Reserved"]
pub type L2_DBUS2_OVF_INT_ST_R = crate::BitReader;
#[doc = "Field `L2_DBUS3_OVF_INT_ST` reader - Reserved"]
pub type L2_DBUS3_OVF_INT_ST_R = crate::BitReader;
impl R {
    #[doc = "Bit 8 - The bit indicates the interrupt status of one of counters overflow that occurs in L2-Cache due to bus0 accesses L2-Cache."]
    #[inline(always)]
    pub fn l2_ibus0_ovf_int_st(&self) -> L2_IBUS0_OVF_INT_ST_R {
        L2_IBUS0_OVF_INT_ST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The bit indicates the interrupt status of one of counters overflow that occurs in L2-Cache due to bus1 accesses L2-Cache."]
    #[inline(always)]
    pub fn l2_ibus1_ovf_int_st(&self) -> L2_IBUS1_OVF_INT_ST_R {
        L2_IBUS1_OVF_INT_ST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Reserved"]
    #[inline(always)]
    pub fn l2_ibus2_ovf_int_st(&self) -> L2_IBUS2_OVF_INT_ST_R {
        L2_IBUS2_OVF_INT_ST_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Reserved"]
    #[inline(always)]
    pub fn l2_ibus3_ovf_int_st(&self) -> L2_IBUS3_OVF_INT_ST_R {
        L2_IBUS3_OVF_INT_ST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - The bit indicates the interrupt status of one of counters overflow that occurs in L2-Cache due to bus0 accesses L2-Cache."]
    #[inline(always)]
    pub fn l2_dbus0_ovf_int_st(&self) -> L2_DBUS0_OVF_INT_ST_R {
        L2_DBUS0_OVF_INT_ST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - The bit indicates the interrupt status of one of counters overflow that occurs in L2-Cache due to bus1 accesses L2-Cache."]
    #[inline(always)]
    pub fn l2_dbus1_ovf_int_st(&self) -> L2_DBUS1_OVF_INT_ST_R {
        L2_DBUS1_OVF_INT_ST_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Reserved"]
    #[inline(always)]
    pub fn l2_dbus2_ovf_int_st(&self) -> L2_DBUS2_OVF_INT_ST_R {
        L2_DBUS2_OVF_INT_ST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Reserved"]
    #[inline(always)]
    pub fn l2_dbus3_ovf_int_st(&self) -> L2_DBUS3_OVF_INT_ST_R {
        L2_DBUS3_OVF_INT_ST_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L2_CACHE_ACS_CNT_INT_ST")
            .field(
                "l2_ibus0_ovf_int_st",
                &format_args!("{}", self.l2_ibus0_ovf_int_st().bit()),
            )
            .field(
                "l2_ibus1_ovf_int_st",
                &format_args!("{}", self.l2_ibus1_ovf_int_st().bit()),
            )
            .field(
                "l2_ibus2_ovf_int_st",
                &format_args!("{}", self.l2_ibus2_ovf_int_st().bit()),
            )
            .field(
                "l2_ibus3_ovf_int_st",
                &format_args!("{}", self.l2_ibus3_ovf_int_st().bit()),
            )
            .field(
                "l2_dbus0_ovf_int_st",
                &format_args!("{}", self.l2_dbus0_ovf_int_st().bit()),
            )
            .field(
                "l2_dbus1_ovf_int_st",
                &format_args!("{}", self.l2_dbus1_ovf_int_st().bit()),
            )
            .field(
                "l2_dbus2_ovf_int_st",
                &format_args!("{}", self.l2_dbus2_ovf_int_st().bit()),
            )
            .field(
                "l2_dbus3_ovf_int_st",
                &format_args!("{}", self.l2_dbus3_ovf_int_st().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<L2_CACHE_ACS_CNT_INT_ST_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Cache Access Counter Interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l2_cache_acs_cnt_int_st](index.html) module"]
pub struct L2_CACHE_ACS_CNT_INT_ST_SPEC;
impl crate::RegisterSpec for L2_CACHE_ACS_CNT_INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [l2_cache_acs_cnt_int_st::R](R) reader structure"]
impl crate::Readable for L2_CACHE_ACS_CNT_INT_ST_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets L2_CACHE_ACS_CNT_INT_ST to value 0"]
impl crate::Resettable for L2_CACHE_ACS_CNT_INT_ST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
