#[doc = "Register `CACHE_ILG_INT_ST` reader"]
pub struct R(crate::R<CACHE_ILG_INT_ST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CACHE_ILG_INT_ST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CACHE_ILG_INT_ST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CACHE_ILG_INT_ST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ICACHE_SYNC_OP_FAULT_ST` reader - The bit is used to indicate interrupt by sync configurations fault."]
pub type ICACHE_SYNC_OP_FAULT_ST_R = crate::BitReader;
#[doc = "Field `ICACHE_PRELOAD_OP_FAULT_ST` reader - The bit is used to indicate interrupt by preload configurations fault."]
pub type ICACHE_PRELOAD_OP_FAULT_ST_R = crate::BitReader;
#[doc = "Field `MMU_ENTRY_FAULT_ST` reader - The bit is used to indicate interrupt by mmu entry fault."]
pub type MMU_ENTRY_FAULT_ST_R = crate::BitReader;
#[doc = "Field `IBUS_ACS_CNT_OVF_ST` reader - The bit is used to indicate interrupt by ibus access flash/spiram counter overflow."]
pub type IBUS_ACS_CNT_OVF_ST_R = crate::BitReader;
#[doc = "Field `IBUS_ACS_MISS_CNT_OVF_ST` reader - The bit is used to indicate interrupt by ibus access flash/spiram miss counter overflow."]
pub type IBUS_ACS_MISS_CNT_OVF_ST_R = crate::BitReader;
#[doc = "Field `DBUS_ACS_CNT_OVF_ST` reader - The bit is used to indicate interrupt by dbus access flash/spiram counter overflow."]
pub type DBUS_ACS_CNT_OVF_ST_R = crate::BitReader;
#[doc = "Field `DBUS_ACS_FLASH_MISS_CNT_OVF_ST` reader - The bit is used to indicate interrupt by dbus access flash miss counter overflow."]
pub type DBUS_ACS_FLASH_MISS_CNT_OVF_ST_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The bit is used to indicate interrupt by sync configurations fault."]
    #[inline(always)]
    pub fn icache_sync_op_fault_st(&self) -> ICACHE_SYNC_OP_FAULT_ST_R {
        ICACHE_SYNC_OP_FAULT_ST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The bit is used to indicate interrupt by preload configurations fault."]
    #[inline(always)]
    pub fn icache_preload_op_fault_st(&self) -> ICACHE_PRELOAD_OP_FAULT_ST_R {
        ICACHE_PRELOAD_OP_FAULT_ST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5 - The bit is used to indicate interrupt by mmu entry fault."]
    #[inline(always)]
    pub fn mmu_entry_fault_st(&self) -> MMU_ENTRY_FAULT_ST_R {
        MMU_ENTRY_FAULT_ST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - The bit is used to indicate interrupt by ibus access flash/spiram counter overflow."]
    #[inline(always)]
    pub fn ibus_acs_cnt_ovf_st(&self) -> IBUS_ACS_CNT_OVF_ST_R {
        IBUS_ACS_CNT_OVF_ST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The bit is used to indicate interrupt by ibus access flash/spiram miss counter overflow."]
    #[inline(always)]
    pub fn ibus_acs_miss_cnt_ovf_st(&self) -> IBUS_ACS_MISS_CNT_OVF_ST_R {
        IBUS_ACS_MISS_CNT_OVF_ST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The bit is used to indicate interrupt by dbus access flash/spiram counter overflow."]
    #[inline(always)]
    pub fn dbus_acs_cnt_ovf_st(&self) -> DBUS_ACS_CNT_OVF_ST_R {
        DBUS_ACS_CNT_OVF_ST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - The bit is used to indicate interrupt by dbus access flash miss counter overflow."]
    #[inline(always)]
    pub fn dbus_acs_flash_miss_cnt_ovf_st(&self) -> DBUS_ACS_FLASH_MISS_CNT_OVF_ST_R {
        DBUS_ACS_FLASH_MISS_CNT_OVF_ST_R::new(((self.bits >> 10) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_ILG_INT_ST")
            .field(
                "icache_sync_op_fault_st",
                &format_args!("{}", self.icache_sync_op_fault_st().bit()),
            )
            .field(
                "icache_preload_op_fault_st",
                &format_args!("{}", self.icache_preload_op_fault_st().bit()),
            )
            .field(
                "mmu_entry_fault_st",
                &format_args!("{}", self.mmu_entry_fault_st().bit()),
            )
            .field(
                "ibus_acs_cnt_ovf_st",
                &format_args!("{}", self.ibus_acs_cnt_ovf_st().bit()),
            )
            .field(
                "ibus_acs_miss_cnt_ovf_st",
                &format_args!("{}", self.ibus_acs_miss_cnt_ovf_st().bit()),
            )
            .field(
                "dbus_acs_cnt_ovf_st",
                &format_args!("{}", self.dbus_acs_cnt_ovf_st().bit()),
            )
            .field(
                "dbus_acs_flash_miss_cnt_ovf_st",
                &format_args!("{}", self.dbus_acs_flash_miss_cnt_ovf_st().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CACHE_ILG_INT_ST_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "This description will be updated in the near future.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cache_ilg_int_st](index.html) module"]
pub struct CACHE_ILG_INT_ST_SPEC;
impl crate::RegisterSpec for CACHE_ILG_INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cache_ilg_int_st::R](R) reader structure"]
impl crate::Readable for CACHE_ILG_INT_ST_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CACHE_ILG_INT_ST to value 0"]
impl crate::Resettable for CACHE_ILG_INT_ST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
