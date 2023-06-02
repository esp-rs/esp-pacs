#[doc = "Register `CACHE_DBG_INT_CLR` writer"]
pub struct W(crate::W<CACHE_DBG_INT_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CACHE_DBG_INT_CLR_SPEC>;
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
impl From<crate::W<CACHE_DBG_INT_CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CACHE_DBG_INT_CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IBUS_ACS_MSK_IC_INT_CLR` writer - The bit is used to clear interrupt by cpu access icache while the corresponding ibus is disabled or icache is disabled which include speculative access."]
pub type IBUS_ACS_MSK_IC_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, CACHE_DBG_INT_CLR_SPEC, O>;
#[doc = "Field `IBUS_CNT_OVF_INT_CLR` writer - The bit is used to clear interrupt by ibus counter overflow."]
pub type IBUS_CNT_OVF_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, CACHE_DBG_INT_CLR_SPEC, O>;
#[doc = "Field `IC_SYNC_SIZE_FAULT_INT_CLR` writer - The bit is used to clear interrupt by manual sync configurations fault."]
pub type IC_SYNC_SIZE_FAULT_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, CACHE_DBG_INT_CLR_SPEC, O>;
#[doc = "Field `IC_PRELOAD_SIZE_FAULT_INT_CLR` writer - The bit is used to clear interrupt by manual pre-load configurations fault."]
pub type IC_PRELOAD_SIZE_FAULT_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, CACHE_DBG_INT_CLR_SPEC, O>;
#[doc = "Field `ICACHE_REJECT_INT_CLR` writer - The bit is used to clear interrupt by authentication fail."]
pub type ICACHE_REJECT_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, CACHE_DBG_INT_CLR_SPEC, O>;
#[doc = "Field `ICACHE_SET_ILG_INT_CLR` writer - The bit is used to clear interrupt by illegal writing lock registers of icache while icache is busy to issue lock,sync or pre-load operations."]
pub type ICACHE_SET_ILG_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, CACHE_DBG_INT_CLR_SPEC, O>;
#[doc = "Field `DBUS_ACS_MSK_DC_INT_CLR` writer - The bit is used to clear interrupt by cpu access dcache while the corresponding dbus is disabled or dcache is disabled which include speculative access."]
pub type DBUS_ACS_MSK_DC_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, CACHE_DBG_INT_CLR_SPEC, O>;
#[doc = "Field `DBUS_CNT_OVF_INT_CLR` writer - The bit is used to clear interrupt by dbus counter overflow."]
pub type DBUS_CNT_OVF_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, CACHE_DBG_INT_CLR_SPEC, O>;
#[doc = "Field `DC_SYNC_SIZE_FAULT_INT_CLR` writer - The bit is used to clear interrupt by manual sync configurations fault."]
pub type DC_SYNC_SIZE_FAULT_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, CACHE_DBG_INT_CLR_SPEC, O>;
#[doc = "Field `DC_PRELOAD_SIZE_FAULT_INT_CLR` writer - The bit is used to clear interrupt by manual pre-load configurations fault."]
pub type DC_PRELOAD_SIZE_FAULT_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, CACHE_DBG_INT_CLR_SPEC, O>;
#[doc = "Field `DCACHE_WRITE_FLASH_INT_CLR` writer - The bit is used to clear interrupt by dcache trying to write flash."]
pub type DCACHE_WRITE_FLASH_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, CACHE_DBG_INT_CLR_SPEC, O>;
#[doc = "Field `DCACHE_REJECT_INT_CLR` writer - The bit is used to clear interrupt by authentication fail."]
pub type DCACHE_REJECT_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, CACHE_DBG_INT_CLR_SPEC, O>;
#[doc = "Field `DCACHE_SET_ILG_INT_CLR` writer - The bit is used to clear interrupt by illegal writing lock registers of dcache while dcache is busy to issue lock,sync or pre-load operations."]
pub type DCACHE_SET_ILG_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, CACHE_DBG_INT_CLR_SPEC, O>;
#[doc = "Field `MMU_ENTRY_FAULT_INT_CLR` writer - The bit is used to clear interrupt by mmu entry fault."]
pub type MMU_ENTRY_FAULT_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, CACHE_DBG_INT_CLR_SPEC, O>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CACHE_DBG_INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - The bit is used to clear interrupt by cpu access icache while the corresponding ibus is disabled or icache is disabled which include speculative access."]
    #[inline(always)]
    #[must_use]
    pub fn ibus_acs_msk_ic_int_clr(&mut self) -> IBUS_ACS_MSK_IC_INT_CLR_W<0> {
        IBUS_ACS_MSK_IC_INT_CLR_W::new(self)
    }
    #[doc = "Bit 1 - The bit is used to clear interrupt by ibus counter overflow."]
    #[inline(always)]
    #[must_use]
    pub fn ibus_cnt_ovf_int_clr(&mut self) -> IBUS_CNT_OVF_INT_CLR_W<1> {
        IBUS_CNT_OVF_INT_CLR_W::new(self)
    }
    #[doc = "Bit 2 - The bit is used to clear interrupt by manual sync configurations fault."]
    #[inline(always)]
    #[must_use]
    pub fn ic_sync_size_fault_int_clr(&mut self) -> IC_SYNC_SIZE_FAULT_INT_CLR_W<2> {
        IC_SYNC_SIZE_FAULT_INT_CLR_W::new(self)
    }
    #[doc = "Bit 3 - The bit is used to clear interrupt by manual pre-load configurations fault."]
    #[inline(always)]
    #[must_use]
    pub fn ic_preload_size_fault_int_clr(&mut self) -> IC_PRELOAD_SIZE_FAULT_INT_CLR_W<3> {
        IC_PRELOAD_SIZE_FAULT_INT_CLR_W::new(self)
    }
    #[doc = "Bit 4 - The bit is used to clear interrupt by authentication fail."]
    #[inline(always)]
    #[must_use]
    pub fn icache_reject_int_clr(&mut self) -> ICACHE_REJECT_INT_CLR_W<4> {
        ICACHE_REJECT_INT_CLR_W::new(self)
    }
    #[doc = "Bit 5 - The bit is used to clear interrupt by illegal writing lock registers of icache while icache is busy to issue lock,sync or pre-load operations."]
    #[inline(always)]
    #[must_use]
    pub fn icache_set_ilg_int_clr(&mut self) -> ICACHE_SET_ILG_INT_CLR_W<5> {
        ICACHE_SET_ILG_INT_CLR_W::new(self)
    }
    #[doc = "Bit 6 - The bit is used to clear interrupt by cpu access dcache while the corresponding dbus is disabled or dcache is disabled which include speculative access."]
    #[inline(always)]
    #[must_use]
    pub fn dbus_acs_msk_dc_int_clr(&mut self) -> DBUS_ACS_MSK_DC_INT_CLR_W<6> {
        DBUS_ACS_MSK_DC_INT_CLR_W::new(self)
    }
    #[doc = "Bit 7 - The bit is used to clear interrupt by dbus counter overflow."]
    #[inline(always)]
    #[must_use]
    pub fn dbus_cnt_ovf_int_clr(&mut self) -> DBUS_CNT_OVF_INT_CLR_W<7> {
        DBUS_CNT_OVF_INT_CLR_W::new(self)
    }
    #[doc = "Bit 8 - The bit is used to clear interrupt by manual sync configurations fault."]
    #[inline(always)]
    #[must_use]
    pub fn dc_sync_size_fault_int_clr(&mut self) -> DC_SYNC_SIZE_FAULT_INT_CLR_W<8> {
        DC_SYNC_SIZE_FAULT_INT_CLR_W::new(self)
    }
    #[doc = "Bit 9 - The bit is used to clear interrupt by manual pre-load configurations fault."]
    #[inline(always)]
    #[must_use]
    pub fn dc_preload_size_fault_int_clr(&mut self) -> DC_PRELOAD_SIZE_FAULT_INT_CLR_W<9> {
        DC_PRELOAD_SIZE_FAULT_INT_CLR_W::new(self)
    }
    #[doc = "Bit 10 - The bit is used to clear interrupt by dcache trying to write flash."]
    #[inline(always)]
    #[must_use]
    pub fn dcache_write_flash_int_clr(&mut self) -> DCACHE_WRITE_FLASH_INT_CLR_W<10> {
        DCACHE_WRITE_FLASH_INT_CLR_W::new(self)
    }
    #[doc = "Bit 11 - The bit is used to clear interrupt by authentication fail."]
    #[inline(always)]
    #[must_use]
    pub fn dcache_reject_int_clr(&mut self) -> DCACHE_REJECT_INT_CLR_W<11> {
        DCACHE_REJECT_INT_CLR_W::new(self)
    }
    #[doc = "Bit 12 - The bit is used to clear interrupt by illegal writing lock registers of dcache while dcache is busy to issue lock,sync or pre-load operations."]
    #[inline(always)]
    #[must_use]
    pub fn dcache_set_ilg_int_clr(&mut self) -> DCACHE_SET_ILG_INT_CLR_W<12> {
        DCACHE_SET_ILG_INT_CLR_W::new(self)
    }
    #[doc = "Bit 13 - The bit is used to clear interrupt by mmu entry fault."]
    #[inline(always)]
    #[must_use]
    pub fn mmu_entry_fault_int_clr(&mut self) -> MMU_ENTRY_FAULT_INT_CLR_W<13> {
        MMU_ENTRY_FAULT_INT_CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "register description\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cache_dbg_int_clr](index.html) module"]
pub struct CACHE_DBG_INT_CLR_SPEC;
impl crate::RegisterSpec for CACHE_DBG_INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [cache_dbg_int_clr::W](W) writer structure"]
impl crate::Writable for CACHE_DBG_INT_CLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CACHE_DBG_INT_CLR to value 0"]
impl crate::Resettable for CACHE_DBG_INT_CLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
