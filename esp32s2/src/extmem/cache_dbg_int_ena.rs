#[doc = "Register `CACHE_DBG_INT_ENA` reader"]
pub struct R(crate::R<CACHE_DBG_INT_ENA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CACHE_DBG_INT_ENA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CACHE_DBG_INT_ENA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CACHE_DBG_INT_ENA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CACHE_DBG_INT_ENA` writer"]
pub struct W(crate::W<CACHE_DBG_INT_ENA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CACHE_DBG_INT_ENA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CACHE_DBG_INT_ENA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CACHE_DBG_INT_ENA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CACHE_DBG_EN` reader - The bit is used to activate the cache track function. 1: enable, 0: disable."]
pub type CACHE_DBG_EN_R = crate::BitReader;
#[doc = "Field `CACHE_DBG_EN` writer - The bit is used to activate the cache track function. 1: enable, 0: disable."]
pub type CACHE_DBG_EN_W<'a, const O: u8> = crate::BitWriter<'a, CACHE_DBG_INT_ENA_SPEC, O>;
#[doc = "Field `IBUS_ACS_MSK_IC_INT_ENA` reader - The bit is used to enable interrupt by cpu access icache while the corresponding ibus is disabled which include speculative access."]
pub type IBUS_ACS_MSK_IC_INT_ENA_R = crate::BitReader;
#[doc = "Field `IBUS_ACS_MSK_IC_INT_ENA` writer - The bit is used to enable interrupt by cpu access icache while the corresponding ibus is disabled which include speculative access."]
pub type IBUS_ACS_MSK_IC_INT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, CACHE_DBG_INT_ENA_SPEC, O>;
#[doc = "Field `IBUS_CNT_OVF_INT_ENA` reader - The bit is used to enable interrupt by ibus counter overflow."]
pub type IBUS_CNT_OVF_INT_ENA_R = crate::BitReader;
#[doc = "Field `IBUS_CNT_OVF_INT_ENA` writer - The bit is used to enable interrupt by ibus counter overflow."]
pub type IBUS_CNT_OVF_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, CACHE_DBG_INT_ENA_SPEC, O>;
#[doc = "Field `IC_SYNC_SIZE_FAULT_INT_ENA` reader - The bit is used to enable interrupt by manual sync configurations fault."]
pub type IC_SYNC_SIZE_FAULT_INT_ENA_R = crate::BitReader;
#[doc = "Field `IC_SYNC_SIZE_FAULT_INT_ENA` writer - The bit is used to enable interrupt by manual sync configurations fault."]
pub type IC_SYNC_SIZE_FAULT_INT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, CACHE_DBG_INT_ENA_SPEC, O>;
#[doc = "Field `IC_PRELOAD_SIZE_FAULT_INT_ENA` reader - The bit is used to enable interrupt by manual pre-load configurations fault."]
pub type IC_PRELOAD_SIZE_FAULT_INT_ENA_R = crate::BitReader;
#[doc = "Field `IC_PRELOAD_SIZE_FAULT_INT_ENA` writer - The bit is used to enable interrupt by manual pre-load configurations fault."]
pub type IC_PRELOAD_SIZE_FAULT_INT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, CACHE_DBG_INT_ENA_SPEC, O>;
#[doc = "Field `ICACHE_REJECT_INT_ENA` reader - The bit is used to enable interrupt by authentication fail."]
pub type ICACHE_REJECT_INT_ENA_R = crate::BitReader;
#[doc = "Field `ICACHE_REJECT_INT_ENA` writer - The bit is used to enable interrupt by authentication fail."]
pub type ICACHE_REJECT_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, CACHE_DBG_INT_ENA_SPEC, O>;
#[doc = "Field `ICACHE_SET_PRELOAD_ILG_INT_ENA` reader - The bit is used to enable interrupt by illegal writing preload registers of icache while icache is busy to issue lock,sync and pre-load operations."]
pub type ICACHE_SET_PRELOAD_ILG_INT_ENA_R = crate::BitReader;
#[doc = "Field `ICACHE_SET_PRELOAD_ILG_INT_ENA` writer - The bit is used to enable interrupt by illegal writing preload registers of icache while icache is busy to issue lock,sync and pre-load operations."]
pub type ICACHE_SET_PRELOAD_ILG_INT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, CACHE_DBG_INT_ENA_SPEC, O>;
#[doc = "Field `ICACHE_SET_SYNC_ILG_INT_ENA` reader - The bit is used to enable interrupt by illegal writing sync registers of icache while icache is busy to issue lock,sync and pre-load operations."]
pub type ICACHE_SET_SYNC_ILG_INT_ENA_R = crate::BitReader;
#[doc = "Field `ICACHE_SET_SYNC_ILG_INT_ENA` writer - The bit is used to enable interrupt by illegal writing sync registers of icache while icache is busy to issue lock,sync and pre-load operations."]
pub type ICACHE_SET_SYNC_ILG_INT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, CACHE_DBG_INT_ENA_SPEC, O>;
#[doc = "Field `ICACHE_SET_LOCK_ILG_INT_ENA` reader - The bit is used to enable interrupt by illegal writing lock registers of icache while icache is busy to issue lock,sync or pre-load operations."]
pub type ICACHE_SET_LOCK_ILG_INT_ENA_R = crate::BitReader;
#[doc = "Field `ICACHE_SET_LOCK_ILG_INT_ENA` writer - The bit is used to enable interrupt by illegal writing lock registers of icache while icache is busy to issue lock,sync or pre-load operations."]
pub type ICACHE_SET_LOCK_ILG_INT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, CACHE_DBG_INT_ENA_SPEC, O>;
#[doc = "Field `DBUS_ACS_MSK_DC_INT_ENA` reader - The bit is used to enable interrupt by cpu access dcache while the corresponding dbus is disabled which include speculative access."]
pub type DBUS_ACS_MSK_DC_INT_ENA_R = crate::BitReader;
#[doc = "Field `DBUS_ACS_MSK_DC_INT_ENA` writer - The bit is used to enable interrupt by cpu access dcache while the corresponding dbus is disabled which include speculative access."]
pub type DBUS_ACS_MSK_DC_INT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, CACHE_DBG_INT_ENA_SPEC, O>;
#[doc = "Field `DBUS_CNT_OVF_INT_ENA` reader - The bit is used to enable interrupt by dbus counter overflow."]
pub type DBUS_CNT_OVF_INT_ENA_R = crate::BitReader;
#[doc = "Field `DBUS_CNT_OVF_INT_ENA` writer - The bit is used to enable interrupt by dbus counter overflow."]
pub type DBUS_CNT_OVF_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, CACHE_DBG_INT_ENA_SPEC, O>;
#[doc = "Field `DC_SYNC_SIZE_FAULT_INT_ENA` reader - The bit is used to enable interrupt by manual sync configurations fault."]
pub type DC_SYNC_SIZE_FAULT_INT_ENA_R = crate::BitReader;
#[doc = "Field `DC_SYNC_SIZE_FAULT_INT_ENA` writer - The bit is used to enable interrupt by manual sync configurations fault."]
pub type DC_SYNC_SIZE_FAULT_INT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, CACHE_DBG_INT_ENA_SPEC, O>;
#[doc = "Field `DC_PRELOAD_SIZE_FAULT_INT_ENA` reader - The bit is used to enable interrupt by manual pre-load configurations fault."]
pub type DC_PRELOAD_SIZE_FAULT_INT_ENA_R = crate::BitReader;
#[doc = "Field `DC_PRELOAD_SIZE_FAULT_INT_ENA` writer - The bit is used to enable interrupt by manual pre-load configurations fault."]
pub type DC_PRELOAD_SIZE_FAULT_INT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, CACHE_DBG_INT_ENA_SPEC, O>;
#[doc = "Field `DCACHE_WRITE_FLASH_INT_ENA` reader - The bit is used to enable interrupt by dcache trying to write flash."]
pub type DCACHE_WRITE_FLASH_INT_ENA_R = crate::BitReader;
#[doc = "Field `DCACHE_WRITE_FLASH_INT_ENA` writer - The bit is used to enable interrupt by dcache trying to write flash."]
pub type DCACHE_WRITE_FLASH_INT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, CACHE_DBG_INT_ENA_SPEC, O>;
#[doc = "Field `DCACHE_REJECT_INT_ENA` reader - The bit is used to enable interrupt by authentication fail."]
pub type DCACHE_REJECT_INT_ENA_R = crate::BitReader;
#[doc = "Field `DCACHE_REJECT_INT_ENA` writer - The bit is used to enable interrupt by authentication fail."]
pub type DCACHE_REJECT_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, CACHE_DBG_INT_ENA_SPEC, O>;
#[doc = "Field `DCACHE_SET_PRELOAD_ILG_INT_ENA` reader - The bit is used to enable interrupt by illegal writing preload registers of dcache while dcache is busy to issue lock,sync and pre-load operations."]
pub type DCACHE_SET_PRELOAD_ILG_INT_ENA_R = crate::BitReader;
#[doc = "Field `DCACHE_SET_PRELOAD_ILG_INT_ENA` writer - The bit is used to enable interrupt by illegal writing preload registers of dcache while dcache is busy to issue lock,sync and pre-load operations."]
pub type DCACHE_SET_PRELOAD_ILG_INT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, CACHE_DBG_INT_ENA_SPEC, O>;
#[doc = "Field `DCACHE_SET_SYNC_ILG_INT_ENA` reader - The bit is used to enable interrupt by illegal writing sync registers of dcache while dcache is busy to issue lock,sync and pre-load operations."]
pub type DCACHE_SET_SYNC_ILG_INT_ENA_R = crate::BitReader;
#[doc = "Field `DCACHE_SET_SYNC_ILG_INT_ENA` writer - The bit is used to enable interrupt by illegal writing sync registers of dcache while dcache is busy to issue lock,sync and pre-load operations."]
pub type DCACHE_SET_SYNC_ILG_INT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, CACHE_DBG_INT_ENA_SPEC, O>;
#[doc = "Field `DCACHE_SET_LOCK_ILG_INT_ENA` reader - The bit is used to enable interrupt by illegal writing lock registers of dcache while dcache is busy to issue lock,sync or pre-load operations."]
pub type DCACHE_SET_LOCK_ILG_INT_ENA_R = crate::BitReader;
#[doc = "Field `DCACHE_SET_LOCK_ILG_INT_ENA` writer - The bit is used to enable interrupt by illegal writing lock registers of dcache while dcache is busy to issue lock,sync or pre-load operations."]
pub type DCACHE_SET_LOCK_ILG_INT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, CACHE_DBG_INT_ENA_SPEC, O>;
#[doc = "Field `MMU_ENTRY_FAULT_INT_ENA` reader - The bit is used to enable interrupt by mmu entry fault."]
pub type MMU_ENTRY_FAULT_INT_ENA_R = crate::BitReader;
#[doc = "Field `MMU_ENTRY_FAULT_INT_ENA` writer - The bit is used to enable interrupt by mmu entry fault."]
pub type MMU_ENTRY_FAULT_INT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, CACHE_DBG_INT_ENA_SPEC, O>;
impl R {
    #[doc = "Bit 0 - The bit is used to activate the cache track function. 1: enable, 0: disable."]
    #[inline(always)]
    pub fn cache_dbg_en(&self) -> CACHE_DBG_EN_R {
        CACHE_DBG_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - The bit is used to enable interrupt by cpu access icache while the corresponding ibus is disabled which include speculative access."]
    #[inline(always)]
    pub fn ibus_acs_msk_ic_int_ena(&self) -> IBUS_ACS_MSK_IC_INT_ENA_R {
        IBUS_ACS_MSK_IC_INT_ENA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The bit is used to enable interrupt by ibus counter overflow."]
    #[inline(always)]
    pub fn ibus_cnt_ovf_int_ena(&self) -> IBUS_CNT_OVF_INT_ENA_R {
        IBUS_CNT_OVF_INT_ENA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The bit is used to enable interrupt by manual sync configurations fault."]
    #[inline(always)]
    pub fn ic_sync_size_fault_int_ena(&self) -> IC_SYNC_SIZE_FAULT_INT_ENA_R {
        IC_SYNC_SIZE_FAULT_INT_ENA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The bit is used to enable interrupt by manual pre-load configurations fault."]
    #[inline(always)]
    pub fn ic_preload_size_fault_int_ena(&self) -> IC_PRELOAD_SIZE_FAULT_INT_ENA_R {
        IC_PRELOAD_SIZE_FAULT_INT_ENA_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The bit is used to enable interrupt by authentication fail."]
    #[inline(always)]
    pub fn icache_reject_int_ena(&self) -> ICACHE_REJECT_INT_ENA_R {
        ICACHE_REJECT_INT_ENA_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The bit is used to enable interrupt by illegal writing preload registers of icache while icache is busy to issue lock,sync and pre-load operations."]
    #[inline(always)]
    pub fn icache_set_preload_ilg_int_ena(&self) -> ICACHE_SET_PRELOAD_ILG_INT_ENA_R {
        ICACHE_SET_PRELOAD_ILG_INT_ENA_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The bit is used to enable interrupt by illegal writing sync registers of icache while icache is busy to issue lock,sync and pre-load operations."]
    #[inline(always)]
    pub fn icache_set_sync_ilg_int_ena(&self) -> ICACHE_SET_SYNC_ILG_INT_ENA_R {
        ICACHE_SET_SYNC_ILG_INT_ENA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The bit is used to enable interrupt by illegal writing lock registers of icache while icache is busy to issue lock,sync or pre-load operations."]
    #[inline(always)]
    pub fn icache_set_lock_ilg_int_ena(&self) -> ICACHE_SET_LOCK_ILG_INT_ENA_R {
        ICACHE_SET_LOCK_ILG_INT_ENA_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - The bit is used to enable interrupt by cpu access dcache while the corresponding dbus is disabled which include speculative access."]
    #[inline(always)]
    pub fn dbus_acs_msk_dc_int_ena(&self) -> DBUS_ACS_MSK_DC_INT_ENA_R {
        DBUS_ACS_MSK_DC_INT_ENA_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The bit is used to enable interrupt by dbus counter overflow."]
    #[inline(always)]
    pub fn dbus_cnt_ovf_int_ena(&self) -> DBUS_CNT_OVF_INT_ENA_R {
        DBUS_CNT_OVF_INT_ENA_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - The bit is used to enable interrupt by manual sync configurations fault."]
    #[inline(always)]
    pub fn dc_sync_size_fault_int_ena(&self) -> DC_SYNC_SIZE_FAULT_INT_ENA_R {
        DC_SYNC_SIZE_FAULT_INT_ENA_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - The bit is used to enable interrupt by manual pre-load configurations fault."]
    #[inline(always)]
    pub fn dc_preload_size_fault_int_ena(&self) -> DC_PRELOAD_SIZE_FAULT_INT_ENA_R {
        DC_PRELOAD_SIZE_FAULT_INT_ENA_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - The bit is used to enable interrupt by dcache trying to write flash."]
    #[inline(always)]
    pub fn dcache_write_flash_int_ena(&self) -> DCACHE_WRITE_FLASH_INT_ENA_R {
        DCACHE_WRITE_FLASH_INT_ENA_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - The bit is used to enable interrupt by authentication fail."]
    #[inline(always)]
    pub fn dcache_reject_int_ena(&self) -> DCACHE_REJECT_INT_ENA_R {
        DCACHE_REJECT_INT_ENA_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - The bit is used to enable interrupt by illegal writing preload registers of dcache while dcache is busy to issue lock,sync and pre-load operations."]
    #[inline(always)]
    pub fn dcache_set_preload_ilg_int_ena(&self) -> DCACHE_SET_PRELOAD_ILG_INT_ENA_R {
        DCACHE_SET_PRELOAD_ILG_INT_ENA_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - The bit is used to enable interrupt by illegal writing sync registers of dcache while dcache is busy to issue lock,sync and pre-load operations."]
    #[inline(always)]
    pub fn dcache_set_sync_ilg_int_ena(&self) -> DCACHE_SET_SYNC_ILG_INT_ENA_R {
        DCACHE_SET_SYNC_ILG_INT_ENA_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - The bit is used to enable interrupt by illegal writing lock registers of dcache while dcache is busy to issue lock,sync or pre-load operations."]
    #[inline(always)]
    pub fn dcache_set_lock_ilg_int_ena(&self) -> DCACHE_SET_LOCK_ILG_INT_ENA_R {
        DCACHE_SET_LOCK_ILG_INT_ENA_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - The bit is used to enable interrupt by mmu entry fault."]
    #[inline(always)]
    pub fn mmu_entry_fault_int_ena(&self) -> MMU_ENTRY_FAULT_INT_ENA_R {
        MMU_ENTRY_FAULT_INT_ENA_R::new(((self.bits >> 19) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_DBG_INT_ENA")
            .field(
                "cache_dbg_en",
                &format_args!("{}", self.cache_dbg_en().bit()),
            )
            .field(
                "ibus_acs_msk_ic_int_ena",
                &format_args!("{}", self.ibus_acs_msk_ic_int_ena().bit()),
            )
            .field(
                "ibus_cnt_ovf_int_ena",
                &format_args!("{}", self.ibus_cnt_ovf_int_ena().bit()),
            )
            .field(
                "ic_sync_size_fault_int_ena",
                &format_args!("{}", self.ic_sync_size_fault_int_ena().bit()),
            )
            .field(
                "ic_preload_size_fault_int_ena",
                &format_args!("{}", self.ic_preload_size_fault_int_ena().bit()),
            )
            .field(
                "icache_reject_int_ena",
                &format_args!("{}", self.icache_reject_int_ena().bit()),
            )
            .field(
                "icache_set_preload_ilg_int_ena",
                &format_args!("{}", self.icache_set_preload_ilg_int_ena().bit()),
            )
            .field(
                "icache_set_sync_ilg_int_ena",
                &format_args!("{}", self.icache_set_sync_ilg_int_ena().bit()),
            )
            .field(
                "icache_set_lock_ilg_int_ena",
                &format_args!("{}", self.icache_set_lock_ilg_int_ena().bit()),
            )
            .field(
                "dbus_acs_msk_dc_int_ena",
                &format_args!("{}", self.dbus_acs_msk_dc_int_ena().bit()),
            )
            .field(
                "dbus_cnt_ovf_int_ena",
                &format_args!("{}", self.dbus_cnt_ovf_int_ena().bit()),
            )
            .field(
                "dc_sync_size_fault_int_ena",
                &format_args!("{}", self.dc_sync_size_fault_int_ena().bit()),
            )
            .field(
                "dc_preload_size_fault_int_ena",
                &format_args!("{}", self.dc_preload_size_fault_int_ena().bit()),
            )
            .field(
                "dcache_write_flash_int_ena",
                &format_args!("{}", self.dcache_write_flash_int_ena().bit()),
            )
            .field(
                "dcache_reject_int_ena",
                &format_args!("{}", self.dcache_reject_int_ena().bit()),
            )
            .field(
                "dcache_set_preload_ilg_int_ena",
                &format_args!("{}", self.dcache_set_preload_ilg_int_ena().bit()),
            )
            .field(
                "dcache_set_sync_ilg_int_ena",
                &format_args!("{}", self.dcache_set_sync_ilg_int_ena().bit()),
            )
            .field(
                "dcache_set_lock_ilg_int_ena",
                &format_args!("{}", self.dcache_set_lock_ilg_int_ena().bit()),
            )
            .field(
                "mmu_entry_fault_int_ena",
                &format_args!("{}", self.mmu_entry_fault_int_ena().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CACHE_DBG_INT_ENA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - The bit is used to activate the cache track function. 1: enable, 0: disable."]
    #[inline(always)]
    #[must_use]
    pub fn cache_dbg_en(&mut self) -> CACHE_DBG_EN_W<0> {
        CACHE_DBG_EN_W::new(self)
    }
    #[doc = "Bit 2 - The bit is used to enable interrupt by cpu access icache while the corresponding ibus is disabled which include speculative access."]
    #[inline(always)]
    #[must_use]
    pub fn ibus_acs_msk_ic_int_ena(&mut self) -> IBUS_ACS_MSK_IC_INT_ENA_W<2> {
        IBUS_ACS_MSK_IC_INT_ENA_W::new(self)
    }
    #[doc = "Bit 3 - The bit is used to enable interrupt by ibus counter overflow."]
    #[inline(always)]
    #[must_use]
    pub fn ibus_cnt_ovf_int_ena(&mut self) -> IBUS_CNT_OVF_INT_ENA_W<3> {
        IBUS_CNT_OVF_INT_ENA_W::new(self)
    }
    #[doc = "Bit 4 - The bit is used to enable interrupt by manual sync configurations fault."]
    #[inline(always)]
    #[must_use]
    pub fn ic_sync_size_fault_int_ena(&mut self) -> IC_SYNC_SIZE_FAULT_INT_ENA_W<4> {
        IC_SYNC_SIZE_FAULT_INT_ENA_W::new(self)
    }
    #[doc = "Bit 5 - The bit is used to enable interrupt by manual pre-load configurations fault."]
    #[inline(always)]
    #[must_use]
    pub fn ic_preload_size_fault_int_ena(&mut self) -> IC_PRELOAD_SIZE_FAULT_INT_ENA_W<5> {
        IC_PRELOAD_SIZE_FAULT_INT_ENA_W::new(self)
    }
    #[doc = "Bit 6 - The bit is used to enable interrupt by authentication fail."]
    #[inline(always)]
    #[must_use]
    pub fn icache_reject_int_ena(&mut self) -> ICACHE_REJECT_INT_ENA_W<6> {
        ICACHE_REJECT_INT_ENA_W::new(self)
    }
    #[doc = "Bit 7 - The bit is used to enable interrupt by illegal writing preload registers of icache while icache is busy to issue lock,sync and pre-load operations."]
    #[inline(always)]
    #[must_use]
    pub fn icache_set_preload_ilg_int_ena(&mut self) -> ICACHE_SET_PRELOAD_ILG_INT_ENA_W<7> {
        ICACHE_SET_PRELOAD_ILG_INT_ENA_W::new(self)
    }
    #[doc = "Bit 8 - The bit is used to enable interrupt by illegal writing sync registers of icache while icache is busy to issue lock,sync and pre-load operations."]
    #[inline(always)]
    #[must_use]
    pub fn icache_set_sync_ilg_int_ena(&mut self) -> ICACHE_SET_SYNC_ILG_INT_ENA_W<8> {
        ICACHE_SET_SYNC_ILG_INT_ENA_W::new(self)
    }
    #[doc = "Bit 9 - The bit is used to enable interrupt by illegal writing lock registers of icache while icache is busy to issue lock,sync or pre-load operations."]
    #[inline(always)]
    #[must_use]
    pub fn icache_set_lock_ilg_int_ena(&mut self) -> ICACHE_SET_LOCK_ILG_INT_ENA_W<9> {
        ICACHE_SET_LOCK_ILG_INT_ENA_W::new(self)
    }
    #[doc = "Bit 10 - The bit is used to enable interrupt by cpu access dcache while the corresponding dbus is disabled which include speculative access."]
    #[inline(always)]
    #[must_use]
    pub fn dbus_acs_msk_dc_int_ena(&mut self) -> DBUS_ACS_MSK_DC_INT_ENA_W<10> {
        DBUS_ACS_MSK_DC_INT_ENA_W::new(self)
    }
    #[doc = "Bit 11 - The bit is used to enable interrupt by dbus counter overflow."]
    #[inline(always)]
    #[must_use]
    pub fn dbus_cnt_ovf_int_ena(&mut self) -> DBUS_CNT_OVF_INT_ENA_W<11> {
        DBUS_CNT_OVF_INT_ENA_W::new(self)
    }
    #[doc = "Bit 12 - The bit is used to enable interrupt by manual sync configurations fault."]
    #[inline(always)]
    #[must_use]
    pub fn dc_sync_size_fault_int_ena(&mut self) -> DC_SYNC_SIZE_FAULT_INT_ENA_W<12> {
        DC_SYNC_SIZE_FAULT_INT_ENA_W::new(self)
    }
    #[doc = "Bit 13 - The bit is used to enable interrupt by manual pre-load configurations fault."]
    #[inline(always)]
    #[must_use]
    pub fn dc_preload_size_fault_int_ena(&mut self) -> DC_PRELOAD_SIZE_FAULT_INT_ENA_W<13> {
        DC_PRELOAD_SIZE_FAULT_INT_ENA_W::new(self)
    }
    #[doc = "Bit 14 - The bit is used to enable interrupt by dcache trying to write flash."]
    #[inline(always)]
    #[must_use]
    pub fn dcache_write_flash_int_ena(&mut self) -> DCACHE_WRITE_FLASH_INT_ENA_W<14> {
        DCACHE_WRITE_FLASH_INT_ENA_W::new(self)
    }
    #[doc = "Bit 15 - The bit is used to enable interrupt by authentication fail."]
    #[inline(always)]
    #[must_use]
    pub fn dcache_reject_int_ena(&mut self) -> DCACHE_REJECT_INT_ENA_W<15> {
        DCACHE_REJECT_INT_ENA_W::new(self)
    }
    #[doc = "Bit 16 - The bit is used to enable interrupt by illegal writing preload registers of dcache while dcache is busy to issue lock,sync and pre-load operations."]
    #[inline(always)]
    #[must_use]
    pub fn dcache_set_preload_ilg_int_ena(&mut self) -> DCACHE_SET_PRELOAD_ILG_INT_ENA_W<16> {
        DCACHE_SET_PRELOAD_ILG_INT_ENA_W::new(self)
    }
    #[doc = "Bit 17 - The bit is used to enable interrupt by illegal writing sync registers of dcache while dcache is busy to issue lock,sync and pre-load operations."]
    #[inline(always)]
    #[must_use]
    pub fn dcache_set_sync_ilg_int_ena(&mut self) -> DCACHE_SET_SYNC_ILG_INT_ENA_W<17> {
        DCACHE_SET_SYNC_ILG_INT_ENA_W::new(self)
    }
    #[doc = "Bit 18 - The bit is used to enable interrupt by illegal writing lock registers of dcache while dcache is busy to issue lock,sync or pre-load operations."]
    #[inline(always)]
    #[must_use]
    pub fn dcache_set_lock_ilg_int_ena(&mut self) -> DCACHE_SET_LOCK_ILG_INT_ENA_W<18> {
        DCACHE_SET_LOCK_ILG_INT_ENA_W::new(self)
    }
    #[doc = "Bit 19 - The bit is used to enable interrupt by mmu entry fault."]
    #[inline(always)]
    #[must_use]
    pub fn mmu_entry_fault_int_ena(&mut self) -> MMU_ENTRY_FAULT_INT_ENA_W<19> {
        MMU_ENTRY_FAULT_INT_ENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cache_dbg_int_ena](index.html) module"]
pub struct CACHE_DBG_INT_ENA_SPEC;
impl crate::RegisterSpec for CACHE_DBG_INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cache_dbg_int_ena::R](R) reader structure"]
impl crate::Readable for CACHE_DBG_INT_ENA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cache_dbg_int_ena::W](W) writer structure"]
impl crate::Writable for CACHE_DBG_INT_ENA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CACHE_DBG_INT_ENA to value 0x01"]
impl crate::Resettable for CACHE_DBG_INT_ENA_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
