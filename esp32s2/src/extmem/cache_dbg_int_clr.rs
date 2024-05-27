///Register `CACHE_DBG_INT_CLR` writer
pub type W = crate::W<CACHE_DBG_INT_CLR_SPEC>;
///Field `IBUS_ACS_MSK_IC_INT_CLR` writer - The bit is used to clear interrupt by cpu access icache while the corresponding ibus is disabled or icache is disabled which include speculative access.
pub type IBUS_ACS_MSK_IC_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IBUS_CNT_OVF_INT_CLR` writer - The bit is used to clear interrupt by ibus counter overflow.
pub type IBUS_CNT_OVF_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC_SYNC_SIZE_FAULT_INT_CLR` writer - The bit is used to clear interrupt by manual sync configurations fault.
pub type IC_SYNC_SIZE_FAULT_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC_PRELOAD_SIZE_FAULT_INT_CLR` writer - The bit is used to clear interrupt by manual pre-load configurations fault.
pub type IC_PRELOAD_SIZE_FAULT_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ICACHE_REJECT_INT_CLR` writer - The bit is used to clear interrupt by authentication fail.
pub type ICACHE_REJECT_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ICACHE_SET_ILG_INT_CLR` writer - The bit is used to clear interrupt by illegal writing lock registers of icache while icache is busy to issue lock,sync or pre-load operations.
pub type ICACHE_SET_ILG_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBUS_ACS_MSK_DC_INT_CLR` writer - The bit is used to clear interrupt by cpu access dcache while the corresponding dbus is disabled or dcache is disabled which include speculative access.
pub type DBUS_ACS_MSK_DC_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBUS_CNT_OVF_INT_CLR` writer - The bit is used to clear interrupt by dbus counter overflow.
pub type DBUS_CNT_OVF_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DC_SYNC_SIZE_FAULT_INT_CLR` writer - The bit is used to clear interrupt by manual sync configurations fault.
pub type DC_SYNC_SIZE_FAULT_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DC_PRELOAD_SIZE_FAULT_INT_CLR` writer - The bit is used to clear interrupt by manual pre-load configurations fault.
pub type DC_PRELOAD_SIZE_FAULT_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DCACHE_WRITE_FLASH_INT_CLR` writer - The bit is used to clear interrupt by dcache trying to write flash.
pub type DCACHE_WRITE_FLASH_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DCACHE_REJECT_INT_CLR` writer - The bit is used to clear interrupt by authentication fail.
pub type DCACHE_REJECT_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DCACHE_SET_ILG_INT_CLR` writer - The bit is used to clear interrupt by illegal writing lock registers of dcache while dcache is busy to issue lock,sync or pre-load operations.
pub type DCACHE_SET_ILG_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MMU_ENTRY_FAULT_INT_CLR` writer - The bit is used to clear interrupt by mmu entry fault.
pub type MMU_ENTRY_FAULT_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CACHE_DBG_INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - The bit is used to clear interrupt by cpu access icache while the corresponding ibus is disabled or icache is disabled which include speculative access.
    #[inline(always)]
    #[must_use]
    pub fn ibus_acs_msk_ic_int_clr(&mut self) -> IBUS_ACS_MSK_IC_INT_CLR_W<CACHE_DBG_INT_CLR_SPEC> {
        IBUS_ACS_MSK_IC_INT_CLR_W::new(self, 0)
    }
    ///Bit 1 - The bit is used to clear interrupt by ibus counter overflow.
    #[inline(always)]
    #[must_use]
    pub fn ibus_cnt_ovf_int_clr(&mut self) -> IBUS_CNT_OVF_INT_CLR_W<CACHE_DBG_INT_CLR_SPEC> {
        IBUS_CNT_OVF_INT_CLR_W::new(self, 1)
    }
    ///Bit 2 - The bit is used to clear interrupt by manual sync configurations fault.
    #[inline(always)]
    #[must_use]
    pub fn ic_sync_size_fault_int_clr(
        &mut self,
    ) -> IC_SYNC_SIZE_FAULT_INT_CLR_W<CACHE_DBG_INT_CLR_SPEC> {
        IC_SYNC_SIZE_FAULT_INT_CLR_W::new(self, 2)
    }
    ///Bit 3 - The bit is used to clear interrupt by manual pre-load configurations fault.
    #[inline(always)]
    #[must_use]
    pub fn ic_preload_size_fault_int_clr(
        &mut self,
    ) -> IC_PRELOAD_SIZE_FAULT_INT_CLR_W<CACHE_DBG_INT_CLR_SPEC> {
        IC_PRELOAD_SIZE_FAULT_INT_CLR_W::new(self, 3)
    }
    ///Bit 4 - The bit is used to clear interrupt by authentication fail.
    #[inline(always)]
    #[must_use]
    pub fn icache_reject_int_clr(&mut self) -> ICACHE_REJECT_INT_CLR_W<CACHE_DBG_INT_CLR_SPEC> {
        ICACHE_REJECT_INT_CLR_W::new(self, 4)
    }
    ///Bit 5 - The bit is used to clear interrupt by illegal writing lock registers of icache while icache is busy to issue lock,sync or pre-load operations.
    #[inline(always)]
    #[must_use]
    pub fn icache_set_ilg_int_clr(&mut self) -> ICACHE_SET_ILG_INT_CLR_W<CACHE_DBG_INT_CLR_SPEC> {
        ICACHE_SET_ILG_INT_CLR_W::new(self, 5)
    }
    ///Bit 6 - The bit is used to clear interrupt by cpu access dcache while the corresponding dbus is disabled or dcache is disabled which include speculative access.
    #[inline(always)]
    #[must_use]
    pub fn dbus_acs_msk_dc_int_clr(&mut self) -> DBUS_ACS_MSK_DC_INT_CLR_W<CACHE_DBG_INT_CLR_SPEC> {
        DBUS_ACS_MSK_DC_INT_CLR_W::new(self, 6)
    }
    ///Bit 7 - The bit is used to clear interrupt by dbus counter overflow.
    #[inline(always)]
    #[must_use]
    pub fn dbus_cnt_ovf_int_clr(&mut self) -> DBUS_CNT_OVF_INT_CLR_W<CACHE_DBG_INT_CLR_SPEC> {
        DBUS_CNT_OVF_INT_CLR_W::new(self, 7)
    }
    ///Bit 8 - The bit is used to clear interrupt by manual sync configurations fault.
    #[inline(always)]
    #[must_use]
    pub fn dc_sync_size_fault_int_clr(
        &mut self,
    ) -> DC_SYNC_SIZE_FAULT_INT_CLR_W<CACHE_DBG_INT_CLR_SPEC> {
        DC_SYNC_SIZE_FAULT_INT_CLR_W::new(self, 8)
    }
    ///Bit 9 - The bit is used to clear interrupt by manual pre-load configurations fault.
    #[inline(always)]
    #[must_use]
    pub fn dc_preload_size_fault_int_clr(
        &mut self,
    ) -> DC_PRELOAD_SIZE_FAULT_INT_CLR_W<CACHE_DBG_INT_CLR_SPEC> {
        DC_PRELOAD_SIZE_FAULT_INT_CLR_W::new(self, 9)
    }
    ///Bit 10 - The bit is used to clear interrupt by dcache trying to write flash.
    #[inline(always)]
    #[must_use]
    pub fn dcache_write_flash_int_clr(
        &mut self,
    ) -> DCACHE_WRITE_FLASH_INT_CLR_W<CACHE_DBG_INT_CLR_SPEC> {
        DCACHE_WRITE_FLASH_INT_CLR_W::new(self, 10)
    }
    ///Bit 11 - The bit is used to clear interrupt by authentication fail.
    #[inline(always)]
    #[must_use]
    pub fn dcache_reject_int_clr(&mut self) -> DCACHE_REJECT_INT_CLR_W<CACHE_DBG_INT_CLR_SPEC> {
        DCACHE_REJECT_INT_CLR_W::new(self, 11)
    }
    ///Bit 12 - The bit is used to clear interrupt by illegal writing lock registers of dcache while dcache is busy to issue lock,sync or pre-load operations.
    #[inline(always)]
    #[must_use]
    pub fn dcache_set_ilg_int_clr(&mut self) -> DCACHE_SET_ILG_INT_CLR_W<CACHE_DBG_INT_CLR_SPEC> {
        DCACHE_SET_ILG_INT_CLR_W::new(self, 12)
    }
    ///Bit 13 - The bit is used to clear interrupt by mmu entry fault.
    #[inline(always)]
    #[must_use]
    pub fn mmu_entry_fault_int_clr(&mut self) -> MMU_ENTRY_FAULT_INT_CLR_W<CACHE_DBG_INT_CLR_SPEC> {
        MMU_ENTRY_FAULT_INT_CLR_W::new(self, 13)
    }
}
/**register description

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cache_dbg_int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CACHE_DBG_INT_CLR_SPEC;
impl crate::RegisterSpec for CACHE_DBG_INT_CLR_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`cache_dbg_int_clr::W`](W) writer structure
impl crate::Writable for CACHE_DBG_INT_CLR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CACHE_DBG_INT_CLR to value 0
impl crate::Resettable for CACHE_DBG_INT_CLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
