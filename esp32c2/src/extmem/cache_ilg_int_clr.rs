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
    crate::BitWriter<'a, CACHE_ILG_INT_CLR_SPEC, O>;
#[doc = "Field `ICACHE_PRELOAD_OP_FAULT_INT_CLR` writer - The bit is used to clear interrupt by preload configurations fault."]
pub type ICACHE_PRELOAD_OP_FAULT_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, CACHE_ILG_INT_CLR_SPEC, O>;
#[doc = "Field `MMU_ENTRY_FAULT_INT_CLR` writer - The bit is used to clear interrupt by mmu entry fault."]
pub type MMU_ENTRY_FAULT_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, CACHE_ILG_INT_CLR_SPEC, O>;
#[doc = "Field `IBUS_CNT_OVF_INT_CLR` writer - The bit is used to clear interrupt by ibus counter overflow."]
pub type IBUS_CNT_OVF_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, CACHE_ILG_INT_CLR_SPEC, O>;
#[doc = "Field `DBUS_CNT_OVF_INT_CLR` writer - The bit is used to clear interrupt by dbus counter overflow."]
pub type DBUS_CNT_OVF_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, CACHE_ILG_INT_CLR_SPEC, O>;
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
    pub fn icache_sync_op_fault_int_clr(&mut self) -> ICACHE_SYNC_OP_FAULT_INT_CLR_W<0> {
        ICACHE_SYNC_OP_FAULT_INT_CLR_W::new(self)
    }
    #[doc = "Bit 1 - The bit is used to clear interrupt by preload configurations fault."]
    #[inline(always)]
    #[must_use]
    pub fn icache_preload_op_fault_int_clr(&mut self) -> ICACHE_PRELOAD_OP_FAULT_INT_CLR_W<1> {
        ICACHE_PRELOAD_OP_FAULT_INT_CLR_W::new(self)
    }
    #[doc = "Bit 5 - The bit is used to clear interrupt by mmu entry fault."]
    #[inline(always)]
    #[must_use]
    pub fn mmu_entry_fault_int_clr(&mut self) -> MMU_ENTRY_FAULT_INT_CLR_W<5> {
        MMU_ENTRY_FAULT_INT_CLR_W::new(self)
    }
    #[doc = "Bit 7 - The bit is used to clear interrupt by ibus counter overflow."]
    #[inline(always)]
    #[must_use]
    pub fn ibus_cnt_ovf_int_clr(&mut self) -> IBUS_CNT_OVF_INT_CLR_W<7> {
        IBUS_CNT_OVF_INT_CLR_W::new(self)
    }
    #[doc = "Bit 8 - The bit is used to clear interrupt by dbus counter overflow."]
    #[inline(always)]
    #[must_use]
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
#[doc = "This description will be updated in the near future.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cache_ilg_int_clr](index.html) module"]
pub struct CACHE_ILG_INT_CLR_SPEC;
impl crate::RegisterSpec for CACHE_ILG_INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [cache_ilg_int_clr::W](W) writer structure"]
impl crate::Writable for CACHE_ILG_INT_CLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CACHE_ILG_INT_CLR to value 0"]
impl crate::Resettable for CACHE_ILG_INT_CLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
