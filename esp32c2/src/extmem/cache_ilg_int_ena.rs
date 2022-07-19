#[doc = "Register `CACHE_ILG_INT_ENA` reader"]
pub struct R(crate::R<CACHE_ILG_INT_ENA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CACHE_ILG_INT_ENA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CACHE_ILG_INT_ENA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CACHE_ILG_INT_ENA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CACHE_ILG_INT_ENA` writer"]
pub struct W(crate::W<CACHE_ILG_INT_ENA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CACHE_ILG_INT_ENA_SPEC>;
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
impl From<crate::W<CACHE_ILG_INT_ENA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CACHE_ILG_INT_ENA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ICACHE_SYNC_OP_FAULT_INT_ENA` reader - The bit is used to enable interrupt by sync configurations fault."]
pub type ICACHE_SYNC_OP_FAULT_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `ICACHE_SYNC_OP_FAULT_INT_ENA` writer - The bit is used to enable interrupt by sync configurations fault."]
pub type ICACHE_SYNC_OP_FAULT_INT_ENA_W<'a> =
    crate::BitWriter<'a, u32, CACHE_ILG_INT_ENA_SPEC, bool, 0>;
#[doc = "Field `ICACHE_PRELOAD_OP_FAULT_INT_ENA` reader - The bit is used to enable interrupt by preload configurations fault."]
pub type ICACHE_PRELOAD_OP_FAULT_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `ICACHE_PRELOAD_OP_FAULT_INT_ENA` writer - The bit is used to enable interrupt by preload configurations fault."]
pub type ICACHE_PRELOAD_OP_FAULT_INT_ENA_W<'a> =
    crate::BitWriter<'a, u32, CACHE_ILG_INT_ENA_SPEC, bool, 1>;
#[doc = "Field `MMU_ENTRY_FAULT_INT_ENA` reader - The bit is used to enable interrupt by mmu entry fault."]
pub type MMU_ENTRY_FAULT_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `MMU_ENTRY_FAULT_INT_ENA` writer - The bit is used to enable interrupt by mmu entry fault."]
pub type MMU_ENTRY_FAULT_INT_ENA_W<'a> = crate::BitWriter<'a, u32, CACHE_ILG_INT_ENA_SPEC, bool, 5>;
#[doc = "Field `IBUS_CNT_OVF_INT_ENA` reader - The bit is used to enable interrupt by ibus counter overflow."]
pub type IBUS_CNT_OVF_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `IBUS_CNT_OVF_INT_ENA` writer - The bit is used to enable interrupt by ibus counter overflow."]
pub type IBUS_CNT_OVF_INT_ENA_W<'a> = crate::BitWriter<'a, u32, CACHE_ILG_INT_ENA_SPEC, bool, 7>;
#[doc = "Field `DBUS_CNT_OVF_INT_ENA` reader - The bit is used to enable interrupt by dbus counter overflow."]
pub type DBUS_CNT_OVF_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `DBUS_CNT_OVF_INT_ENA` writer - The bit is used to enable interrupt by dbus counter overflow."]
pub type DBUS_CNT_OVF_INT_ENA_W<'a> = crate::BitWriter<'a, u32, CACHE_ILG_INT_ENA_SPEC, bool, 8>;
impl R {
    #[doc = "Bit 0 - The bit is used to enable interrupt by sync configurations fault."]
    #[inline(always)]
    pub fn icache_sync_op_fault_int_ena(&self) -> ICACHE_SYNC_OP_FAULT_INT_ENA_R {
        ICACHE_SYNC_OP_FAULT_INT_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The bit is used to enable interrupt by preload configurations fault."]
    #[inline(always)]
    pub fn icache_preload_op_fault_int_ena(&self) -> ICACHE_PRELOAD_OP_FAULT_INT_ENA_R {
        ICACHE_PRELOAD_OP_FAULT_INT_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5 - The bit is used to enable interrupt by mmu entry fault."]
    #[inline(always)]
    pub fn mmu_entry_fault_int_ena(&self) -> MMU_ENTRY_FAULT_INT_ENA_R {
        MMU_ENTRY_FAULT_INT_ENA_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - The bit is used to enable interrupt by ibus counter overflow."]
    #[inline(always)]
    pub fn ibus_cnt_ovf_int_ena(&self) -> IBUS_CNT_OVF_INT_ENA_R {
        IBUS_CNT_OVF_INT_ENA_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The bit is used to enable interrupt by dbus counter overflow."]
    #[inline(always)]
    pub fn dbus_cnt_ovf_int_ena(&self) -> DBUS_CNT_OVF_INT_ENA_R {
        DBUS_CNT_OVF_INT_ENA_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The bit is used to enable interrupt by sync configurations fault."]
    #[inline(always)]
    pub fn icache_sync_op_fault_int_ena(&mut self) -> ICACHE_SYNC_OP_FAULT_INT_ENA_W {
        ICACHE_SYNC_OP_FAULT_INT_ENA_W::new(self)
    }
    #[doc = "Bit 1 - The bit is used to enable interrupt by preload configurations fault."]
    #[inline(always)]
    pub fn icache_preload_op_fault_int_ena(&mut self) -> ICACHE_PRELOAD_OP_FAULT_INT_ENA_W {
        ICACHE_PRELOAD_OP_FAULT_INT_ENA_W::new(self)
    }
    #[doc = "Bit 5 - The bit is used to enable interrupt by mmu entry fault."]
    #[inline(always)]
    pub fn mmu_entry_fault_int_ena(&mut self) -> MMU_ENTRY_FAULT_INT_ENA_W {
        MMU_ENTRY_FAULT_INT_ENA_W::new(self)
    }
    #[doc = "Bit 7 - The bit is used to enable interrupt by ibus counter overflow."]
    #[inline(always)]
    pub fn ibus_cnt_ovf_int_ena(&mut self) -> IBUS_CNT_OVF_INT_ENA_W {
        IBUS_CNT_OVF_INT_ENA_W::new(self)
    }
    #[doc = "Bit 8 - The bit is used to enable interrupt by dbus counter overflow."]
    #[inline(always)]
    pub fn dbus_cnt_ovf_int_ena(&mut self) -> DBUS_CNT_OVF_INT_ENA_W {
        DBUS_CNT_OVF_INT_ENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This description will be updated in the near future.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cache_ilg_int_ena](index.html) module"]
pub struct CACHE_ILG_INT_ENA_SPEC;
impl crate::RegisterSpec for CACHE_ILG_INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cache_ilg_int_ena::R](R) reader structure"]
impl crate::Readable for CACHE_ILG_INT_ENA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cache_ilg_int_ena::W](W) writer structure"]
impl crate::Writable for CACHE_ILG_INT_ENA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CACHE_ILG_INT_ENA to value 0"]
impl crate::Resettable for CACHE_ILG_INT_ENA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
