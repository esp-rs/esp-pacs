#[doc = "Register `CACHE_DBG_INT_ENA` reader"]
pub type R = crate::R<CACHE_DBG_INT_ENA_SPEC>;
#[doc = "Register `CACHE_DBG_INT_ENA` writer"]
pub type W = crate::W<CACHE_DBG_INT_ENA_SPEC>;
#[doc = "Field `CACHE_DBG_EN` reader - The bit is used to activate the cache track function. 1: enable, 0: disable."]
pub type CACHE_DBG_EN_R = crate::BitReader;
#[doc = "Field `CACHE_DBG_EN` writer - The bit is used to activate the cache track function. 1: enable, 0: disable."]
pub type CACHE_DBG_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IBUS_ACS_MSK_IC_INT_ENA` reader - The bit is used to enable interrupt by cpu access icache while the corresponding ibus is disabled which include speculative access."]
pub type IBUS_ACS_MSK_IC_INT_ENA_R = crate::BitReader;
#[doc = "Field `IBUS_ACS_MSK_IC_INT_ENA` writer - The bit is used to enable interrupt by cpu access icache while the corresponding ibus is disabled which include speculative access."]
pub type IBUS_ACS_MSK_IC_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IBUS_CNT_OVF_INT_ENA` reader - The bit is used to enable interrupt by ibus counter overflow."]
pub type IBUS_CNT_OVF_INT_ENA_R = crate::BitReader;
#[doc = "Field `IBUS_CNT_OVF_INT_ENA` writer - The bit is used to enable interrupt by ibus counter overflow."]
pub type IBUS_CNT_OVF_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IC_SYNC_SIZE_FAULT_INT_ENA` reader - The bit is used to enable interrupt by manual sync configurations fault."]
pub type IC_SYNC_SIZE_FAULT_INT_ENA_R = crate::BitReader;
#[doc = "Field `IC_SYNC_SIZE_FAULT_INT_ENA` writer - The bit is used to enable interrupt by manual sync configurations fault."]
pub type IC_SYNC_SIZE_FAULT_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IC_PRELOAD_SIZE_FAULT_INT_ENA` reader - The bit is used to enable interrupt by manual pre-load configurations fault."]
pub type IC_PRELOAD_SIZE_FAULT_INT_ENA_R = crate::BitReader;
#[doc = "Field `IC_PRELOAD_SIZE_FAULT_INT_ENA` writer - The bit is used to enable interrupt by manual pre-load configurations fault."]
pub type IC_PRELOAD_SIZE_FAULT_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICACHE_REJECT_INT_ENA` reader - The bit is used to enable interrupt by authentication fail."]
pub type ICACHE_REJECT_INT_ENA_R = crate::BitReader;
#[doc = "Field `ICACHE_REJECT_INT_ENA` writer - The bit is used to enable interrupt by authentication fail."]
pub type ICACHE_REJECT_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICACHE_SET_PRELOAD_ILG_INT_ENA` reader - The bit is used to enable interrupt by illegal writing preload registers of icache while icache is busy to issue lock,sync and pre-load operations."]
pub type ICACHE_SET_PRELOAD_ILG_INT_ENA_R = crate::BitReader;
#[doc = "Field `ICACHE_SET_PRELOAD_ILG_INT_ENA` writer - The bit is used to enable interrupt by illegal writing preload registers of icache while icache is busy to issue lock,sync and pre-load operations."]
pub type ICACHE_SET_PRELOAD_ILG_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICACHE_SET_SYNC_ILG_INT_ENA` reader - The bit is used to enable interrupt by illegal writing sync registers of icache while icache is busy to issue lock,sync and pre-load operations."]
pub type ICACHE_SET_SYNC_ILG_INT_ENA_R = crate::BitReader;
#[doc = "Field `ICACHE_SET_SYNC_ILG_INT_ENA` writer - The bit is used to enable interrupt by illegal writing sync registers of icache while icache is busy to issue lock,sync and pre-load operations."]
pub type ICACHE_SET_SYNC_ILG_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICACHE_SET_LOCK_ILG_INT_ENA` reader - The bit is used to enable interrupt by illegal writing lock registers of icache while icache is busy to issue lock,sync or pre-load operations."]
pub type ICACHE_SET_LOCK_ILG_INT_ENA_R = crate::BitReader;
#[doc = "Field `ICACHE_SET_LOCK_ILG_INT_ENA` writer - The bit is used to enable interrupt by illegal writing lock registers of icache while icache is busy to issue lock,sync or pre-load operations."]
pub type ICACHE_SET_LOCK_ILG_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBUS_ACS_MSK_DC_INT_ENA` reader - The bit is used to enable interrupt by cpu access dcache while the corresponding dbus is disabled which include speculative access."]
pub type DBUS_ACS_MSK_DC_INT_ENA_R = crate::BitReader;
#[doc = "Field `DBUS_ACS_MSK_DC_INT_ENA` writer - The bit is used to enable interrupt by cpu access dcache while the corresponding dbus is disabled which include speculative access."]
pub type DBUS_ACS_MSK_DC_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBUS_CNT_OVF_INT_ENA` reader - The bit is used to enable interrupt by dbus counter overflow."]
pub type DBUS_CNT_OVF_INT_ENA_R = crate::BitReader;
#[doc = "Field `DBUS_CNT_OVF_INT_ENA` writer - The bit is used to enable interrupt by dbus counter overflow."]
pub type DBUS_CNT_OVF_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DC_SYNC_SIZE_FAULT_INT_ENA` reader - The bit is used to enable interrupt by manual sync configurations fault."]
pub type DC_SYNC_SIZE_FAULT_INT_ENA_R = crate::BitReader;
#[doc = "Field `DC_SYNC_SIZE_FAULT_INT_ENA` writer - The bit is used to enable interrupt by manual sync configurations fault."]
pub type DC_SYNC_SIZE_FAULT_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DC_PRELOAD_SIZE_FAULT_INT_ENA` reader - The bit is used to enable interrupt by manual pre-load configurations fault."]
pub type DC_PRELOAD_SIZE_FAULT_INT_ENA_R = crate::BitReader;
#[doc = "Field `DC_PRELOAD_SIZE_FAULT_INT_ENA` writer - The bit is used to enable interrupt by manual pre-load configurations fault."]
pub type DC_PRELOAD_SIZE_FAULT_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCACHE_WRITE_FLASH_INT_ENA` reader - The bit is used to enable interrupt by dcache trying to write flash."]
pub type DCACHE_WRITE_FLASH_INT_ENA_R = crate::BitReader;
#[doc = "Field `DCACHE_WRITE_FLASH_INT_ENA` writer - The bit is used to enable interrupt by dcache trying to write flash."]
pub type DCACHE_WRITE_FLASH_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCACHE_REJECT_INT_ENA` reader - The bit is used to enable interrupt by authentication fail."]
pub type DCACHE_REJECT_INT_ENA_R = crate::BitReader;
#[doc = "Field `DCACHE_REJECT_INT_ENA` writer - The bit is used to enable interrupt by authentication fail."]
pub type DCACHE_REJECT_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCACHE_SET_PRELOAD_ILG_INT_ENA` reader - The bit is used to enable interrupt by illegal writing preload registers of dcache while dcache is busy to issue lock,sync and pre-load operations."]
pub type DCACHE_SET_PRELOAD_ILG_INT_ENA_R = crate::BitReader;
#[doc = "Field `DCACHE_SET_PRELOAD_ILG_INT_ENA` writer - The bit is used to enable interrupt by illegal writing preload registers of dcache while dcache is busy to issue lock,sync and pre-load operations."]
pub type DCACHE_SET_PRELOAD_ILG_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCACHE_SET_SYNC_ILG_INT_ENA` reader - The bit is used to enable interrupt by illegal writing sync registers of dcache while dcache is busy to issue lock,sync and pre-load operations."]
pub type DCACHE_SET_SYNC_ILG_INT_ENA_R = crate::BitReader;
#[doc = "Field `DCACHE_SET_SYNC_ILG_INT_ENA` writer - The bit is used to enable interrupt by illegal writing sync registers of dcache while dcache is busy to issue lock,sync and pre-load operations."]
pub type DCACHE_SET_SYNC_ILG_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCACHE_SET_LOCK_ILG_INT_ENA` reader - The bit is used to enable interrupt by illegal writing lock registers of dcache while dcache is busy to issue lock,sync or pre-load operations."]
pub type DCACHE_SET_LOCK_ILG_INT_ENA_R = crate::BitReader;
#[doc = "Field `DCACHE_SET_LOCK_ILG_INT_ENA` writer - The bit is used to enable interrupt by illegal writing lock registers of dcache while dcache is busy to issue lock,sync or pre-load operations."]
pub type DCACHE_SET_LOCK_ILG_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MMU_ENTRY_FAULT_INT_ENA` reader - The bit is used to enable interrupt by mmu entry fault."]
pub type MMU_ENTRY_FAULT_INT_ENA_R = crate::BitReader;
#[doc = "Field `MMU_ENTRY_FAULT_INT_ENA` writer - The bit is used to enable interrupt by mmu entry fault."]
pub type MMU_ENTRY_FAULT_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
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
            .field("cache_dbg_en", &self.cache_dbg_en())
            .field("ibus_acs_msk_ic_int_ena", &self.ibus_acs_msk_ic_int_ena())
            .field("ibus_cnt_ovf_int_ena", &self.ibus_cnt_ovf_int_ena())
            .field(
                "ic_sync_size_fault_int_ena",
                &self.ic_sync_size_fault_int_ena(),
            )
            .field(
                "ic_preload_size_fault_int_ena",
                &self.ic_preload_size_fault_int_ena(),
            )
            .field("icache_reject_int_ena", &self.icache_reject_int_ena())
            .field(
                "icache_set_preload_ilg_int_ena",
                &self.icache_set_preload_ilg_int_ena(),
            )
            .field(
                "icache_set_sync_ilg_int_ena",
                &self.icache_set_sync_ilg_int_ena(),
            )
            .field(
                "icache_set_lock_ilg_int_ena",
                &self.icache_set_lock_ilg_int_ena(),
            )
            .field("dbus_acs_msk_dc_int_ena", &self.dbus_acs_msk_dc_int_ena())
            .field("dbus_cnt_ovf_int_ena", &self.dbus_cnt_ovf_int_ena())
            .field(
                "dc_sync_size_fault_int_ena",
                &self.dc_sync_size_fault_int_ena(),
            )
            .field(
                "dc_preload_size_fault_int_ena",
                &self.dc_preload_size_fault_int_ena(),
            )
            .field(
                "dcache_write_flash_int_ena",
                &self.dcache_write_flash_int_ena(),
            )
            .field("dcache_reject_int_ena", &self.dcache_reject_int_ena())
            .field(
                "dcache_set_preload_ilg_int_ena",
                &self.dcache_set_preload_ilg_int_ena(),
            )
            .field(
                "dcache_set_sync_ilg_int_ena",
                &self.dcache_set_sync_ilg_int_ena(),
            )
            .field(
                "dcache_set_lock_ilg_int_ena",
                &self.dcache_set_lock_ilg_int_ena(),
            )
            .field("mmu_entry_fault_int_ena", &self.mmu_entry_fault_int_ena())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - The bit is used to activate the cache track function. 1: enable, 0: disable."]
    #[inline(always)]
    pub fn cache_dbg_en(&mut self) -> CACHE_DBG_EN_W<CACHE_DBG_INT_ENA_SPEC> {
        CACHE_DBG_EN_W::new(self, 0)
    }
    #[doc = "Bit 2 - The bit is used to enable interrupt by cpu access icache while the corresponding ibus is disabled which include speculative access."]
    #[inline(always)]
    pub fn ibus_acs_msk_ic_int_ena(&mut self) -> IBUS_ACS_MSK_IC_INT_ENA_W<CACHE_DBG_INT_ENA_SPEC> {
        IBUS_ACS_MSK_IC_INT_ENA_W::new(self, 2)
    }
    #[doc = "Bit 3 - The bit is used to enable interrupt by ibus counter overflow."]
    #[inline(always)]
    pub fn ibus_cnt_ovf_int_ena(&mut self) -> IBUS_CNT_OVF_INT_ENA_W<CACHE_DBG_INT_ENA_SPEC> {
        IBUS_CNT_OVF_INT_ENA_W::new(self, 3)
    }
    #[doc = "Bit 4 - The bit is used to enable interrupt by manual sync configurations fault."]
    #[inline(always)]
    pub fn ic_sync_size_fault_int_ena(
        &mut self,
    ) -> IC_SYNC_SIZE_FAULT_INT_ENA_W<CACHE_DBG_INT_ENA_SPEC> {
        IC_SYNC_SIZE_FAULT_INT_ENA_W::new(self, 4)
    }
    #[doc = "Bit 5 - The bit is used to enable interrupt by manual pre-load configurations fault."]
    #[inline(always)]
    pub fn ic_preload_size_fault_int_ena(
        &mut self,
    ) -> IC_PRELOAD_SIZE_FAULT_INT_ENA_W<CACHE_DBG_INT_ENA_SPEC> {
        IC_PRELOAD_SIZE_FAULT_INT_ENA_W::new(self, 5)
    }
    #[doc = "Bit 6 - The bit is used to enable interrupt by authentication fail."]
    #[inline(always)]
    pub fn icache_reject_int_ena(&mut self) -> ICACHE_REJECT_INT_ENA_W<CACHE_DBG_INT_ENA_SPEC> {
        ICACHE_REJECT_INT_ENA_W::new(self, 6)
    }
    #[doc = "Bit 7 - The bit is used to enable interrupt by illegal writing preload registers of icache while icache is busy to issue lock,sync and pre-load operations."]
    #[inline(always)]
    pub fn icache_set_preload_ilg_int_ena(
        &mut self,
    ) -> ICACHE_SET_PRELOAD_ILG_INT_ENA_W<CACHE_DBG_INT_ENA_SPEC> {
        ICACHE_SET_PRELOAD_ILG_INT_ENA_W::new(self, 7)
    }
    #[doc = "Bit 8 - The bit is used to enable interrupt by illegal writing sync registers of icache while icache is busy to issue lock,sync and pre-load operations."]
    #[inline(always)]
    pub fn icache_set_sync_ilg_int_ena(
        &mut self,
    ) -> ICACHE_SET_SYNC_ILG_INT_ENA_W<CACHE_DBG_INT_ENA_SPEC> {
        ICACHE_SET_SYNC_ILG_INT_ENA_W::new(self, 8)
    }
    #[doc = "Bit 9 - The bit is used to enable interrupt by illegal writing lock registers of icache while icache is busy to issue lock,sync or pre-load operations."]
    #[inline(always)]
    pub fn icache_set_lock_ilg_int_ena(
        &mut self,
    ) -> ICACHE_SET_LOCK_ILG_INT_ENA_W<CACHE_DBG_INT_ENA_SPEC> {
        ICACHE_SET_LOCK_ILG_INT_ENA_W::new(self, 9)
    }
    #[doc = "Bit 10 - The bit is used to enable interrupt by cpu access dcache while the corresponding dbus is disabled which include speculative access."]
    #[inline(always)]
    pub fn dbus_acs_msk_dc_int_ena(&mut self) -> DBUS_ACS_MSK_DC_INT_ENA_W<CACHE_DBG_INT_ENA_SPEC> {
        DBUS_ACS_MSK_DC_INT_ENA_W::new(self, 10)
    }
    #[doc = "Bit 11 - The bit is used to enable interrupt by dbus counter overflow."]
    #[inline(always)]
    pub fn dbus_cnt_ovf_int_ena(&mut self) -> DBUS_CNT_OVF_INT_ENA_W<CACHE_DBG_INT_ENA_SPEC> {
        DBUS_CNT_OVF_INT_ENA_W::new(self, 11)
    }
    #[doc = "Bit 12 - The bit is used to enable interrupt by manual sync configurations fault."]
    #[inline(always)]
    pub fn dc_sync_size_fault_int_ena(
        &mut self,
    ) -> DC_SYNC_SIZE_FAULT_INT_ENA_W<CACHE_DBG_INT_ENA_SPEC> {
        DC_SYNC_SIZE_FAULT_INT_ENA_W::new(self, 12)
    }
    #[doc = "Bit 13 - The bit is used to enable interrupt by manual pre-load configurations fault."]
    #[inline(always)]
    pub fn dc_preload_size_fault_int_ena(
        &mut self,
    ) -> DC_PRELOAD_SIZE_FAULT_INT_ENA_W<CACHE_DBG_INT_ENA_SPEC> {
        DC_PRELOAD_SIZE_FAULT_INT_ENA_W::new(self, 13)
    }
    #[doc = "Bit 14 - The bit is used to enable interrupt by dcache trying to write flash."]
    #[inline(always)]
    pub fn dcache_write_flash_int_ena(
        &mut self,
    ) -> DCACHE_WRITE_FLASH_INT_ENA_W<CACHE_DBG_INT_ENA_SPEC> {
        DCACHE_WRITE_FLASH_INT_ENA_W::new(self, 14)
    }
    #[doc = "Bit 15 - The bit is used to enable interrupt by authentication fail."]
    #[inline(always)]
    pub fn dcache_reject_int_ena(&mut self) -> DCACHE_REJECT_INT_ENA_W<CACHE_DBG_INT_ENA_SPEC> {
        DCACHE_REJECT_INT_ENA_W::new(self, 15)
    }
    #[doc = "Bit 16 - The bit is used to enable interrupt by illegal writing preload registers of dcache while dcache is busy to issue lock,sync and pre-load operations."]
    #[inline(always)]
    pub fn dcache_set_preload_ilg_int_ena(
        &mut self,
    ) -> DCACHE_SET_PRELOAD_ILG_INT_ENA_W<CACHE_DBG_INT_ENA_SPEC> {
        DCACHE_SET_PRELOAD_ILG_INT_ENA_W::new(self, 16)
    }
    #[doc = "Bit 17 - The bit is used to enable interrupt by illegal writing sync registers of dcache while dcache is busy to issue lock,sync and pre-load operations."]
    #[inline(always)]
    pub fn dcache_set_sync_ilg_int_ena(
        &mut self,
    ) -> DCACHE_SET_SYNC_ILG_INT_ENA_W<CACHE_DBG_INT_ENA_SPEC> {
        DCACHE_SET_SYNC_ILG_INT_ENA_W::new(self, 17)
    }
    #[doc = "Bit 18 - The bit is used to enable interrupt by illegal writing lock registers of dcache while dcache is busy to issue lock,sync or pre-load operations."]
    #[inline(always)]
    pub fn dcache_set_lock_ilg_int_ena(
        &mut self,
    ) -> DCACHE_SET_LOCK_ILG_INT_ENA_W<CACHE_DBG_INT_ENA_SPEC> {
        DCACHE_SET_LOCK_ILG_INT_ENA_W::new(self, 18)
    }
    #[doc = "Bit 19 - The bit is used to enable interrupt by mmu entry fault."]
    #[inline(always)]
    pub fn mmu_entry_fault_int_ena(&mut self) -> MMU_ENTRY_FAULT_INT_ENA_W<CACHE_DBG_INT_ENA_SPEC> {
        MMU_ENTRY_FAULT_INT_ENA_W::new(self, 19)
    }
}
#[doc = "register description\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_dbg_int_ena::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_dbg_int_ena::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CACHE_DBG_INT_ENA_SPEC;
impl crate::RegisterSpec for CACHE_DBG_INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cache_dbg_int_ena::R`](R) reader structure"]
impl crate::Readable for CACHE_DBG_INT_ENA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cache_dbg_int_ena::W`](W) writer structure"]
impl crate::Writable for CACHE_DBG_INT_ENA_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CACHE_DBG_INT_ENA to value 0x01"]
impl crate::Resettable for CACHE_DBG_INT_ENA_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
