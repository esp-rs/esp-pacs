#[doc = "Register `CACHE_ILG_INT_CLR` writer"]
pub type W = crate::W<CACHE_ILG_INT_CLR_SPEC>;
#[doc = "Field `ICACHE_SYNC_OP_FAULT_INT_CLR` writer - The bit is used to clear interrupt by sync configurations fault."]
pub type ICACHE_SYNC_OP_FAULT_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICACHE_PRELOAD_OP_FAULT_INT_CLR` writer - The bit is used to clear interrupt by preload configurations fault."]
pub type ICACHE_PRELOAD_OP_FAULT_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCACHE_SYNC_OP_FAULT_INT_CLR` writer - The bit is used to clear interrupt by sync configurations fault."]
pub type DCACHE_SYNC_OP_FAULT_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCACHE_PRELOAD_OP_FAULT_INT_CLR` writer - The bit is used to clear interrupt by preload configurations fault."]
pub type DCACHE_PRELOAD_OP_FAULT_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCACHE_WRITE_FLASH_INT_CLR` writer - The bit is used to clear interrupt by dcache trying to write flash."]
pub type DCACHE_WRITE_FLASH_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MMU_ENTRY_FAULT_INT_CLR` writer - The bit is used to clear interrupt by mmu entry fault."]
pub type MMU_ENTRY_FAULT_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCACHE_OCCUPY_EXC_INT_CLR` writer - The bit is used to clear interrupt by dcache trying to replace a line whose blocks all have been occupied by occupy-mode."]
pub type DCACHE_OCCUPY_EXC_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IBUS_CNT_OVF_INT_CLR` writer - The bit is used to clear interrupt by ibus counter overflow."]
pub type IBUS_CNT_OVF_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBUS_CNT_OVF_INT_CLR` writer - The bit is used to clear interrupt by dbus counter overflow."]
pub type DBUS_CNT_OVF_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CACHE_ILG_INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - The bit is used to clear interrupt by sync configurations fault."]
    #[inline(always)]
    #[must_use]
    pub fn icache_sync_op_fault_int_clr(
        &mut self,
    ) -> ICACHE_SYNC_OP_FAULT_INT_CLR_W<CACHE_ILG_INT_CLR_SPEC> {
        ICACHE_SYNC_OP_FAULT_INT_CLR_W::new(self, 0)
    }
    #[doc = "Bit 1 - The bit is used to clear interrupt by preload configurations fault."]
    #[inline(always)]
    #[must_use]
    pub fn icache_preload_op_fault_int_clr(
        &mut self,
    ) -> ICACHE_PRELOAD_OP_FAULT_INT_CLR_W<CACHE_ILG_INT_CLR_SPEC> {
        ICACHE_PRELOAD_OP_FAULT_INT_CLR_W::new(self, 1)
    }
    #[doc = "Bit 2 - The bit is used to clear interrupt by sync configurations fault."]
    #[inline(always)]
    #[must_use]
    pub fn dcache_sync_op_fault_int_clr(
        &mut self,
    ) -> DCACHE_SYNC_OP_FAULT_INT_CLR_W<CACHE_ILG_INT_CLR_SPEC> {
        DCACHE_SYNC_OP_FAULT_INT_CLR_W::new(self, 2)
    }
    #[doc = "Bit 3 - The bit is used to clear interrupt by preload configurations fault."]
    #[inline(always)]
    #[must_use]
    pub fn dcache_preload_op_fault_int_clr(
        &mut self,
    ) -> DCACHE_PRELOAD_OP_FAULT_INT_CLR_W<CACHE_ILG_INT_CLR_SPEC> {
        DCACHE_PRELOAD_OP_FAULT_INT_CLR_W::new(self, 3)
    }
    #[doc = "Bit 4 - The bit is used to clear interrupt by dcache trying to write flash."]
    #[inline(always)]
    #[must_use]
    pub fn dcache_write_flash_int_clr(
        &mut self,
    ) -> DCACHE_WRITE_FLASH_INT_CLR_W<CACHE_ILG_INT_CLR_SPEC> {
        DCACHE_WRITE_FLASH_INT_CLR_W::new(self, 4)
    }
    #[doc = "Bit 5 - The bit is used to clear interrupt by mmu entry fault."]
    #[inline(always)]
    #[must_use]
    pub fn mmu_entry_fault_int_clr(&mut self) -> MMU_ENTRY_FAULT_INT_CLR_W<CACHE_ILG_INT_CLR_SPEC> {
        MMU_ENTRY_FAULT_INT_CLR_W::new(self, 5)
    }
    #[doc = "Bit 6 - The bit is used to clear interrupt by dcache trying to replace a line whose blocks all have been occupied by occupy-mode."]
    #[inline(always)]
    #[must_use]
    pub fn dcache_occupy_exc_int_clr(
        &mut self,
    ) -> DCACHE_OCCUPY_EXC_INT_CLR_W<CACHE_ILG_INT_CLR_SPEC> {
        DCACHE_OCCUPY_EXC_INT_CLR_W::new(self, 6)
    }
    #[doc = "Bit 7 - The bit is used to clear interrupt by ibus counter overflow."]
    #[inline(always)]
    #[must_use]
    pub fn ibus_cnt_ovf_int_clr(&mut self) -> IBUS_CNT_OVF_INT_CLR_W<CACHE_ILG_INT_CLR_SPEC> {
        IBUS_CNT_OVF_INT_CLR_W::new(self, 7)
    }
    #[doc = "Bit 8 - The bit is used to clear interrupt by dbus counter overflow."]
    #[inline(always)]
    #[must_use]
    pub fn dbus_cnt_ovf_int_clr(&mut self) -> DBUS_CNT_OVF_INT_CLR_W<CACHE_ILG_INT_CLR_SPEC> {
        DBUS_CNT_OVF_INT_CLR_W::new(self, 8)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "******* Description ***********\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cache_ilg_int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CACHE_ILG_INT_CLR_SPEC;
impl crate::RegisterSpec for CACHE_ILG_INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cache_ilg_int_clr::W`](W) writer structure"]
impl crate::Writable for CACHE_ILG_INT_CLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CACHE_ILG_INT_CLR to value 0"]
impl crate::Resettable for CACHE_ILG_INT_CLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
