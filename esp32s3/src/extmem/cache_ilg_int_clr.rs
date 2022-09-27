#[doc = "Register `CACHE_ILG_INT_CLR` writer"]
pub struct W(crate::W<CACHE_ILG_INT_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CACHE_ILG_INT_CLR_SPEC>;
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
impl From<crate::W<CACHE_ILG_INT_CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CACHE_ILG_INT_CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ICACHE_SYNC_OP_FAULT_INT_CLR` writer - The bit is used to clear interrupt by sync configurations fault."]
pub type ICACHE_SYNC_OP_FAULT_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CACHE_ILG_INT_CLR_SPEC, bool, O>;
#[doc = "Field `ICACHE_PRELOAD_OP_FAULT_INT_CLR` writer - The bit is used to clear interrupt by preload configurations fault."]
pub type ICACHE_PRELOAD_OP_FAULT_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CACHE_ILG_INT_CLR_SPEC, bool, O>;
#[doc = "Field `DCACHE_SYNC_OP_FAULT_INT_CLR` writer - The bit is used to clear interrupt by sync configurations fault."]
pub type DCACHE_SYNC_OP_FAULT_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CACHE_ILG_INT_CLR_SPEC, bool, O>;
#[doc = "Field `DCACHE_PRELOAD_OP_FAULT_INT_CLR` writer - The bit is used to clear interrupt by preload configurations fault."]
pub type DCACHE_PRELOAD_OP_FAULT_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CACHE_ILG_INT_CLR_SPEC, bool, O>;
#[doc = "Field `DCACHE_WRITE_FLASH_INT_CLR` writer - The bit is used to clear interrupt by dcache trying to write flash."]
pub type DCACHE_WRITE_FLASH_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CACHE_ILG_INT_CLR_SPEC, bool, O>;
#[doc = "Field `MMU_ENTRY_FAULT_INT_CLR` writer - The bit is used to clear interrupt by mmu entry fault."]
pub type MMU_ENTRY_FAULT_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CACHE_ILG_INT_CLR_SPEC, bool, O>;
#[doc = "Field `DCACHE_OCCUPY_EXC_INT_CLR` writer - The bit is used to clear interrupt by dcache trying to replace a line whose blocks all have been occupied by occupy-mode."]
pub type DCACHE_OCCUPY_EXC_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CACHE_ILG_INT_CLR_SPEC, bool, O>;
#[doc = "Field `IBUS_CNT_OVF_INT_CLR` writer - The bit is used to clear interrupt by ibus counter overflow."]
pub type IBUS_CNT_OVF_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CACHE_ILG_INT_CLR_SPEC, bool, O>;
#[doc = "Field `DBUS_CNT_OVF_INT_CLR` writer - The bit is used to clear interrupt by dbus counter overflow."]
pub type DBUS_CNT_OVF_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CACHE_ILG_INT_CLR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - The bit is used to clear interrupt by sync configurations fault."]
    #[inline(always)]
    pub fn icache_sync_op_fault_int_clr(&mut self) -> ICACHE_SYNC_OP_FAULT_INT_CLR_W<0> {
        ICACHE_SYNC_OP_FAULT_INT_CLR_W::new(self)
    }
    #[doc = "Bit 1 - The bit is used to clear interrupt by preload configurations fault."]
    #[inline(always)]
    pub fn icache_preload_op_fault_int_clr(&mut self) -> ICACHE_PRELOAD_OP_FAULT_INT_CLR_W<1> {
        ICACHE_PRELOAD_OP_FAULT_INT_CLR_W::new(self)
    }
    #[doc = "Bit 2 - The bit is used to clear interrupt by sync configurations fault."]
    #[inline(always)]
    pub fn dcache_sync_op_fault_int_clr(&mut self) -> DCACHE_SYNC_OP_FAULT_INT_CLR_W<2> {
        DCACHE_SYNC_OP_FAULT_INT_CLR_W::new(self)
    }
    #[doc = "Bit 3 - The bit is used to clear interrupt by preload configurations fault."]
    #[inline(always)]
    pub fn dcache_preload_op_fault_int_clr(&mut self) -> DCACHE_PRELOAD_OP_FAULT_INT_CLR_W<3> {
        DCACHE_PRELOAD_OP_FAULT_INT_CLR_W::new(self)
    }
    #[doc = "Bit 4 - The bit is used to clear interrupt by dcache trying to write flash."]
    #[inline(always)]
    pub fn dcache_write_flash_int_clr(&mut self) -> DCACHE_WRITE_FLASH_INT_CLR_W<4> {
        DCACHE_WRITE_FLASH_INT_CLR_W::new(self)
    }
    #[doc = "Bit 5 - The bit is used to clear interrupt by mmu entry fault."]
    #[inline(always)]
    pub fn mmu_entry_fault_int_clr(&mut self) -> MMU_ENTRY_FAULT_INT_CLR_W<5> {
        MMU_ENTRY_FAULT_INT_CLR_W::new(self)
    }
    #[doc = "Bit 6 - The bit is used to clear interrupt by dcache trying to replace a line whose blocks all have been occupied by occupy-mode."]
    #[inline(always)]
    pub fn dcache_occupy_exc_int_clr(&mut self) -> DCACHE_OCCUPY_EXC_INT_CLR_W<6> {
        DCACHE_OCCUPY_EXC_INT_CLR_W::new(self)
    }
    #[doc = "Bit 7 - The bit is used to clear interrupt by ibus counter overflow."]
    #[inline(always)]
    pub fn ibus_cnt_ovf_int_clr(&mut self) -> IBUS_CNT_OVF_INT_CLR_W<7> {
        IBUS_CNT_OVF_INT_CLR_W::new(self)
    }
    #[doc = "Bit 8 - The bit is used to clear interrupt by dbus counter overflow."]
    #[inline(always)]
    pub fn dbus_cnt_ovf_int_clr(&mut self) -> DBUS_CNT_OVF_INT_CLR_W<8> {
        DBUS_CNT_OVF_INT_CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "******* Description ***********\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cache_ilg_int_clr](index.html) module"]
pub struct CACHE_ILG_INT_CLR_SPEC;
impl crate::RegisterSpec for CACHE_ILG_INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [cache_ilg_int_clr::W](W) writer structure"]
impl crate::Writable for CACHE_ILG_INT_CLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CACHE_ILG_INT_CLR to value 0"]
impl crate::Resettable for CACHE_ILG_INT_CLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
