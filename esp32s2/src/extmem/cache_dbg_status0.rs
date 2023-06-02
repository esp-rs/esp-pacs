#[doc = "Register `CACHE_DBG_STATUS0` reader"]
pub struct R(crate::R<CACHE_DBG_STATUS0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CACHE_DBG_STATUS0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CACHE_DBG_STATUS0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CACHE_DBG_STATUS0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `IBUS0_ACS_MSK_ICACHE_ST` reader - The bit is used to indicate interrupt by cpu access icache while the ibus0 is disabled or icache is disabled which include speculative access."]
pub type IBUS0_ACS_MSK_ICACHE_ST_R = crate::BitReader;
#[doc = "Field `IBUS1_ACS_MSK_ICACHE_ST` reader - The bit is used to indicate interrupt by cpu access icache while the ibus1 is disabled or icache is disabled which include speculative access."]
pub type IBUS1_ACS_MSK_ICACHE_ST_R = crate::BitReader;
#[doc = "Field `IBUS2_ACS_MSK_ICACHE_ST` reader - The bit is used to indicate interrupt by cpu access icache while the ibus2 is disabled or icache is disabled which include speculative access."]
pub type IBUS2_ACS_MSK_ICACHE_ST_R = crate::BitReader;
#[doc = "Field `IBUS0_ACS_CNT_OVF_ST` reader - The bit is used to indicate interrupt by ibus0 counter overflow."]
pub type IBUS0_ACS_CNT_OVF_ST_R = crate::BitReader;
#[doc = "Field `IBUS1_ACS_CNT_OVF_ST` reader - The bit is used to indicate interrupt by ibus1 counter overflow."]
pub type IBUS1_ACS_CNT_OVF_ST_R = crate::BitReader;
#[doc = "Field `IBUS2_ACS_CNT_OVF_ST` reader - The bit is used to indicate interrupt by ibus2 counter overflow."]
pub type IBUS2_ACS_CNT_OVF_ST_R = crate::BitReader;
#[doc = "Field `IBUS0_ACS_MISS_CNT_OVF_ST` reader - The bit is used to indicate interrupt by ibus0 miss counter overflow."]
pub type IBUS0_ACS_MISS_CNT_OVF_ST_R = crate::BitReader;
#[doc = "Field `IBUS1_ACS_MISS_CNT_OVF_ST` reader - The bit is used to indicate interrupt by ibus1 miss counter overflow."]
pub type IBUS1_ACS_MISS_CNT_OVF_ST_R = crate::BitReader;
#[doc = "Field `IBUS2_ACS_MISS_CNT_OVF_ST` reader - The bit is used to indicate interrupt by ibus2 miss counter overflow."]
pub type IBUS2_ACS_MISS_CNT_OVF_ST_R = crate::BitReader;
#[doc = "Field `IBUS0_ABANDON_CNT_OVF_ST` reader - The bit is used to indicate interrupt by ibus0 abandon counter overflow."]
pub type IBUS0_ABANDON_CNT_OVF_ST_R = crate::BitReader;
#[doc = "Field `IBUS1_ABANDON_CNT_OVF_ST` reader - The bit is used to indicate interrupt by ibus1 abandon counter overflow."]
pub type IBUS1_ABANDON_CNT_OVF_ST_R = crate::BitReader;
#[doc = "Field `IBUS2_ABANDON_CNT_OVF_ST` reader - The bit is used to indicate interrupt by ibus2 abandon counter overflow."]
pub type IBUS2_ABANDON_CNT_OVF_ST_R = crate::BitReader;
#[doc = "Field `IC_PRELOAD_MISS_CNT_OVF_ST` reader - The bit is used to indicate interrupt by pre-load miss counter overflow."]
pub type IC_PRELOAD_MISS_CNT_OVF_ST_R = crate::BitReader;
#[doc = "Field `IC_PRELOAD_CNT_OVF_ST` reader - The bit is used to indicate interrupt by pre-load counter overflow."]
pub type IC_PRELOAD_CNT_OVF_ST_R = crate::BitReader;
#[doc = "Field `IC_SYNC_SIZE_FAULT_ST` reader - The bit is used to indicate interrupt by manual sync configurations fault."]
pub type IC_SYNC_SIZE_FAULT_ST_R = crate::BitReader;
#[doc = "Field `IC_PRELOAD_SIZE_FAULT_ST` reader - The bit is used to indicate interrupt by manual pre-load configurations fault."]
pub type IC_PRELOAD_SIZE_FAULT_ST_R = crate::BitReader;
#[doc = "Field `ICACHE_REJECT_ST` reader - The bit is used to indicate interrupt by authentication fail."]
pub type ICACHE_REJECT_ST_R = crate::BitReader;
#[doc = "Field `ICACHE_SET_PRELOAD_ILG_ST` reader - The bit is used to indicate interrupt by illegal writing preload registers of icache while icache is busy to issue lock,sync and pre-load operations."]
pub type ICACHE_SET_PRELOAD_ILG_ST_R = crate::BitReader;
#[doc = "Field `ICACHE_SET_SYNC_ILG_ST` reader - The bit is used to indicate interrupt by illegal writing sync registers of icache while icache is busy to issue lock,sync and pre-load operations."]
pub type ICACHE_SET_SYNC_ILG_ST_R = crate::BitReader;
#[doc = "Field `ICACHE_SET_LOCK_ILG_ST` reader - The bit is used to indicate interrupt by illegal writing lock registers of icache while icache is busy to issue lock,sync or pre-load operations."]
pub type ICACHE_SET_LOCK_ILG_ST_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The bit is used to indicate interrupt by cpu access icache while the ibus0 is disabled or icache is disabled which include speculative access."]
    #[inline(always)]
    pub fn ibus0_acs_msk_icache_st(&self) -> IBUS0_ACS_MSK_ICACHE_ST_R {
        IBUS0_ACS_MSK_ICACHE_ST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The bit is used to indicate interrupt by cpu access icache while the ibus1 is disabled or icache is disabled which include speculative access."]
    #[inline(always)]
    pub fn ibus1_acs_msk_icache_st(&self) -> IBUS1_ACS_MSK_ICACHE_ST_R {
        IBUS1_ACS_MSK_ICACHE_ST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The bit is used to indicate interrupt by cpu access icache while the ibus2 is disabled or icache is disabled which include speculative access."]
    #[inline(always)]
    pub fn ibus2_acs_msk_icache_st(&self) -> IBUS2_ACS_MSK_ICACHE_ST_R {
        IBUS2_ACS_MSK_ICACHE_ST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - The bit is used to indicate interrupt by ibus0 counter overflow."]
    #[inline(always)]
    pub fn ibus0_acs_cnt_ovf_st(&self) -> IBUS0_ACS_CNT_OVF_ST_R {
        IBUS0_ACS_CNT_OVF_ST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The bit is used to indicate interrupt by ibus1 counter overflow."]
    #[inline(always)]
    pub fn ibus1_acs_cnt_ovf_st(&self) -> IBUS1_ACS_CNT_OVF_ST_R {
        IBUS1_ACS_CNT_OVF_ST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The bit is used to indicate interrupt by ibus2 counter overflow."]
    #[inline(always)]
    pub fn ibus2_acs_cnt_ovf_st(&self) -> IBUS2_ACS_CNT_OVF_ST_R {
        IBUS2_ACS_CNT_OVF_ST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - The bit is used to indicate interrupt by ibus0 miss counter overflow."]
    #[inline(always)]
    pub fn ibus0_acs_miss_cnt_ovf_st(&self) -> IBUS0_ACS_MISS_CNT_OVF_ST_R {
        IBUS0_ACS_MISS_CNT_OVF_ST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The bit is used to indicate interrupt by ibus1 miss counter overflow."]
    #[inline(always)]
    pub fn ibus1_acs_miss_cnt_ovf_st(&self) -> IBUS1_ACS_MISS_CNT_OVF_ST_R {
        IBUS1_ACS_MISS_CNT_OVF_ST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - The bit is used to indicate interrupt by ibus2 miss counter overflow."]
    #[inline(always)]
    pub fn ibus2_acs_miss_cnt_ovf_st(&self) -> IBUS2_ACS_MISS_CNT_OVF_ST_R {
        IBUS2_ACS_MISS_CNT_OVF_ST_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - The bit is used to indicate interrupt by ibus0 abandon counter overflow."]
    #[inline(always)]
    pub fn ibus0_abandon_cnt_ovf_st(&self) -> IBUS0_ABANDON_CNT_OVF_ST_R {
        IBUS0_ABANDON_CNT_OVF_ST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - The bit is used to indicate interrupt by ibus1 abandon counter overflow."]
    #[inline(always)]
    pub fn ibus1_abandon_cnt_ovf_st(&self) -> IBUS1_ABANDON_CNT_OVF_ST_R {
        IBUS1_ABANDON_CNT_OVF_ST_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - The bit is used to indicate interrupt by ibus2 abandon counter overflow."]
    #[inline(always)]
    pub fn ibus2_abandon_cnt_ovf_st(&self) -> IBUS2_ABANDON_CNT_OVF_ST_R {
        IBUS2_ABANDON_CNT_OVF_ST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - The bit is used to indicate interrupt by pre-load miss counter overflow."]
    #[inline(always)]
    pub fn ic_preload_miss_cnt_ovf_st(&self) -> IC_PRELOAD_MISS_CNT_OVF_ST_R {
        IC_PRELOAD_MISS_CNT_OVF_ST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - The bit is used to indicate interrupt by pre-load counter overflow."]
    #[inline(always)]
    pub fn ic_preload_cnt_ovf_st(&self) -> IC_PRELOAD_CNT_OVF_ST_R {
        IC_PRELOAD_CNT_OVF_ST_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - The bit is used to indicate interrupt by manual sync configurations fault."]
    #[inline(always)]
    pub fn ic_sync_size_fault_st(&self) -> IC_SYNC_SIZE_FAULT_ST_R {
        IC_SYNC_SIZE_FAULT_ST_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - The bit is used to indicate interrupt by manual pre-load configurations fault."]
    #[inline(always)]
    pub fn ic_preload_size_fault_st(&self) -> IC_PRELOAD_SIZE_FAULT_ST_R {
        IC_PRELOAD_SIZE_FAULT_ST_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - The bit is used to indicate interrupt by authentication fail."]
    #[inline(always)]
    pub fn icache_reject_st(&self) -> ICACHE_REJECT_ST_R {
        ICACHE_REJECT_ST_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - The bit is used to indicate interrupt by illegal writing preload registers of icache while icache is busy to issue lock,sync and pre-load operations."]
    #[inline(always)]
    pub fn icache_set_preload_ilg_st(&self) -> ICACHE_SET_PRELOAD_ILG_ST_R {
        ICACHE_SET_PRELOAD_ILG_ST_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - The bit is used to indicate interrupt by illegal writing sync registers of icache while icache is busy to issue lock,sync and pre-load operations."]
    #[inline(always)]
    pub fn icache_set_sync_ilg_st(&self) -> ICACHE_SET_SYNC_ILG_ST_R {
        ICACHE_SET_SYNC_ILG_ST_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - The bit is used to indicate interrupt by illegal writing lock registers of icache while icache is busy to issue lock,sync or pre-load operations."]
    #[inline(always)]
    pub fn icache_set_lock_ilg_st(&self) -> ICACHE_SET_LOCK_ILG_ST_R {
        ICACHE_SET_LOCK_ILG_ST_R::new(((self.bits >> 24) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_DBG_STATUS0")
            .field(
                "ibus0_acs_msk_icache_st",
                &format_args!("{}", self.ibus0_acs_msk_icache_st().bit()),
            )
            .field(
                "ibus1_acs_msk_icache_st",
                &format_args!("{}", self.ibus1_acs_msk_icache_st().bit()),
            )
            .field(
                "ibus2_acs_msk_icache_st",
                &format_args!("{}", self.ibus2_acs_msk_icache_st().bit()),
            )
            .field(
                "ibus0_acs_cnt_ovf_st",
                &format_args!("{}", self.ibus0_acs_cnt_ovf_st().bit()),
            )
            .field(
                "ibus1_acs_cnt_ovf_st",
                &format_args!("{}", self.ibus1_acs_cnt_ovf_st().bit()),
            )
            .field(
                "ibus2_acs_cnt_ovf_st",
                &format_args!("{}", self.ibus2_acs_cnt_ovf_st().bit()),
            )
            .field(
                "ibus0_acs_miss_cnt_ovf_st",
                &format_args!("{}", self.ibus0_acs_miss_cnt_ovf_st().bit()),
            )
            .field(
                "ibus1_acs_miss_cnt_ovf_st",
                &format_args!("{}", self.ibus1_acs_miss_cnt_ovf_st().bit()),
            )
            .field(
                "ibus2_acs_miss_cnt_ovf_st",
                &format_args!("{}", self.ibus2_acs_miss_cnt_ovf_st().bit()),
            )
            .field(
                "ibus0_abandon_cnt_ovf_st",
                &format_args!("{}", self.ibus0_abandon_cnt_ovf_st().bit()),
            )
            .field(
                "ibus1_abandon_cnt_ovf_st",
                &format_args!("{}", self.ibus1_abandon_cnt_ovf_st().bit()),
            )
            .field(
                "ibus2_abandon_cnt_ovf_st",
                &format_args!("{}", self.ibus2_abandon_cnt_ovf_st().bit()),
            )
            .field(
                "ic_preload_miss_cnt_ovf_st",
                &format_args!("{}", self.ic_preload_miss_cnt_ovf_st().bit()),
            )
            .field(
                "ic_preload_cnt_ovf_st",
                &format_args!("{}", self.ic_preload_cnt_ovf_st().bit()),
            )
            .field(
                "ic_sync_size_fault_st",
                &format_args!("{}", self.ic_sync_size_fault_st().bit()),
            )
            .field(
                "ic_preload_size_fault_st",
                &format_args!("{}", self.ic_preload_size_fault_st().bit()),
            )
            .field(
                "icache_reject_st",
                &format_args!("{}", self.icache_reject_st().bit()),
            )
            .field(
                "icache_set_preload_ilg_st",
                &format_args!("{}", self.icache_set_preload_ilg_st().bit()),
            )
            .field(
                "icache_set_sync_ilg_st",
                &format_args!("{}", self.icache_set_sync_ilg_st().bit()),
            )
            .field(
                "icache_set_lock_ilg_st",
                &format_args!("{}", self.icache_set_lock_ilg_st().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CACHE_DBG_STATUS0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cache_dbg_status0](index.html) module"]
pub struct CACHE_DBG_STATUS0_SPEC;
impl crate::RegisterSpec for CACHE_DBG_STATUS0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cache_dbg_status0::R](R) reader structure"]
impl crate::Readable for CACHE_DBG_STATUS0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CACHE_DBG_STATUS0 to value 0"]
impl crate::Resettable for CACHE_DBG_STATUS0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
