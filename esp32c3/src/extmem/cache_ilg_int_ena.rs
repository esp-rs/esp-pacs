#[doc = "Register `CACHE_ILG_INT_ENA` reader"]
pub type R = crate::R<CACHE_ILG_INT_ENA_SPEC>;
#[doc = "Register `CACHE_ILG_INT_ENA` writer"]
pub type W = crate::W<CACHE_ILG_INT_ENA_SPEC>;
#[doc = "Field `ICACHE_SYNC_OP_FAULT_INT_ENA` reader - The bit is used to enable interrupt by sync configurations fault."]
pub type ICACHE_SYNC_OP_FAULT_INT_ENA_R = crate::BitReader;
#[doc = "Field `ICACHE_SYNC_OP_FAULT_INT_ENA` writer - The bit is used to enable interrupt by sync configurations fault."]
pub type ICACHE_SYNC_OP_FAULT_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICACHE_PRELOAD_OP_FAULT_INT_ENA` reader - The bit is used to enable interrupt by preload configurations fault."]
pub type ICACHE_PRELOAD_OP_FAULT_INT_ENA_R = crate::BitReader;
#[doc = "Field `ICACHE_PRELOAD_OP_FAULT_INT_ENA` writer - The bit is used to enable interrupt by preload configurations fault."]
pub type ICACHE_PRELOAD_OP_FAULT_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MMU_ENTRY_FAULT_INT_ENA` reader - The bit is used to enable interrupt by mmu entry fault."]
pub type MMU_ENTRY_FAULT_INT_ENA_R = crate::BitReader;
#[doc = "Field `MMU_ENTRY_FAULT_INT_ENA` writer - The bit is used to enable interrupt by mmu entry fault."]
pub type MMU_ENTRY_FAULT_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IBUS_CNT_OVF_INT_ENA` reader - The bit is used to enable interrupt by ibus counter overflow."]
pub type IBUS_CNT_OVF_INT_ENA_R = crate::BitReader;
#[doc = "Field `IBUS_CNT_OVF_INT_ENA` writer - The bit is used to enable interrupt by ibus counter overflow."]
pub type IBUS_CNT_OVF_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBUS_CNT_OVF_INT_ENA` reader - The bit is used to enable interrupt by dbus counter overflow."]
pub type DBUS_CNT_OVF_INT_ENA_R = crate::BitReader;
#[doc = "Field `DBUS_CNT_OVF_INT_ENA` writer - The bit is used to enable interrupt by dbus counter overflow."]
pub type DBUS_CNT_OVF_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
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
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_ILG_INT_ENA")
            .field(
                "icache_sync_op_fault_int_ena",
                &format_args!("{}", self.icache_sync_op_fault_int_ena().bit()),
            )
            .field(
                "icache_preload_op_fault_int_ena",
                &format_args!("{}", self.icache_preload_op_fault_int_ena().bit()),
            )
            .field(
                "mmu_entry_fault_int_ena",
                &format_args!("{}", self.mmu_entry_fault_int_ena().bit()),
            )
            .field(
                "ibus_cnt_ovf_int_ena",
                &format_args!("{}", self.ibus_cnt_ovf_int_ena().bit()),
            )
            .field(
                "dbus_cnt_ovf_int_ena",
                &format_args!("{}", self.dbus_cnt_ovf_int_ena().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CACHE_ILG_INT_ENA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - The bit is used to enable interrupt by sync configurations fault."]
    #[inline(always)]
    #[must_use]
    pub fn icache_sync_op_fault_int_ena(
        &mut self,
    ) -> ICACHE_SYNC_OP_FAULT_INT_ENA_W<CACHE_ILG_INT_ENA_SPEC> {
        ICACHE_SYNC_OP_FAULT_INT_ENA_W::new(self, 0)
    }
    #[doc = "Bit 1 - The bit is used to enable interrupt by preload configurations fault."]
    #[inline(always)]
    #[must_use]
    pub fn icache_preload_op_fault_int_ena(
        &mut self,
    ) -> ICACHE_PRELOAD_OP_FAULT_INT_ENA_W<CACHE_ILG_INT_ENA_SPEC> {
        ICACHE_PRELOAD_OP_FAULT_INT_ENA_W::new(self, 1)
    }
    #[doc = "Bit 5 - The bit is used to enable interrupt by mmu entry fault."]
    #[inline(always)]
    #[must_use]
    pub fn mmu_entry_fault_int_ena(&mut self) -> MMU_ENTRY_FAULT_INT_ENA_W<CACHE_ILG_INT_ENA_SPEC> {
        MMU_ENTRY_FAULT_INT_ENA_W::new(self, 5)
    }
    #[doc = "Bit 7 - The bit is used to enable interrupt by ibus counter overflow."]
    #[inline(always)]
    #[must_use]
    pub fn ibus_cnt_ovf_int_ena(&mut self) -> IBUS_CNT_OVF_INT_ENA_W<CACHE_ILG_INT_ENA_SPEC> {
        IBUS_CNT_OVF_INT_ENA_W::new(self, 7)
    }
    #[doc = "Bit 8 - The bit is used to enable interrupt by dbus counter overflow."]
    #[inline(always)]
    #[must_use]
    pub fn dbus_cnt_ovf_int_ena(&mut self) -> DBUS_CNT_OVF_INT_ENA_W<CACHE_ILG_INT_ENA_SPEC> {
        DBUS_CNT_OVF_INT_ENA_W::new(self, 8)
    }
}
#[doc = "This description will be updated in the near future.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cache_ilg_int_ena::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cache_ilg_int_ena::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CACHE_ILG_INT_ENA_SPEC;
impl crate::RegisterSpec for CACHE_ILG_INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cache_ilg_int_ena::R`](R) reader structure"]
impl crate::Readable for CACHE_ILG_INT_ENA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cache_ilg_int_ena::W`](W) writer structure"]
impl crate::Writable for CACHE_ILG_INT_ENA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CACHE_ILG_INT_ENA to value 0"]
impl crate::Resettable for CACHE_ILG_INT_ENA_SPEC {
    const RESET_VALUE: u32 = 0;
}
