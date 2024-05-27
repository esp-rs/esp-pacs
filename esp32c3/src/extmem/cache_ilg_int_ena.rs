///Register `CACHE_ILG_INT_ENA` reader
pub type R = crate::R<CACHE_ILG_INT_ENA_SPEC>;
///Register `CACHE_ILG_INT_ENA` writer
pub type W = crate::W<CACHE_ILG_INT_ENA_SPEC>;
///Field `ICACHE_SYNC_OP_FAULT` reader - The bit is used to enable interrupt by sync configurations fault.
pub type ICACHE_SYNC_OP_FAULT_R = crate::BitReader;
///Field `ICACHE_SYNC_OP_FAULT` writer - The bit is used to enable interrupt by sync configurations fault.
pub type ICACHE_SYNC_OP_FAULT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ICACHE_PRELOAD_OP_FAULT` reader - The bit is used to enable interrupt by preload configurations fault.
pub type ICACHE_PRELOAD_OP_FAULT_R = crate::BitReader;
///Field `ICACHE_PRELOAD_OP_FAULT` writer - The bit is used to enable interrupt by preload configurations fault.
pub type ICACHE_PRELOAD_OP_FAULT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MMU_ENTRY_FAULT` reader - The bit is used to enable interrupt by mmu entry fault.
pub type MMU_ENTRY_FAULT_R = crate::BitReader;
///Field `MMU_ENTRY_FAULT` writer - The bit is used to enable interrupt by mmu entry fault.
pub type MMU_ENTRY_FAULT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IBUS_CNT_OVF` reader - The bit is used to enable interrupt by ibus counter overflow.
pub type IBUS_CNT_OVF_R = crate::BitReader;
///Field `IBUS_CNT_OVF` writer - The bit is used to enable interrupt by ibus counter overflow.
pub type IBUS_CNT_OVF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBUS_CNT_OVF` reader - The bit is used to enable interrupt by dbus counter overflow.
pub type DBUS_CNT_OVF_R = crate::BitReader;
///Field `DBUS_CNT_OVF` writer - The bit is used to enable interrupt by dbus counter overflow.
pub type DBUS_CNT_OVF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - The bit is used to enable interrupt by sync configurations fault.
    #[inline(always)]
    pub fn icache_sync_op_fault(&self) -> ICACHE_SYNC_OP_FAULT_R {
        ICACHE_SYNC_OP_FAULT_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - The bit is used to enable interrupt by preload configurations fault.
    #[inline(always)]
    pub fn icache_preload_op_fault(&self) -> ICACHE_PRELOAD_OP_FAULT_R {
        ICACHE_PRELOAD_OP_FAULT_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 5 - The bit is used to enable interrupt by mmu entry fault.
    #[inline(always)]
    pub fn mmu_entry_fault(&self) -> MMU_ENTRY_FAULT_R {
        MMU_ENTRY_FAULT_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 7 - The bit is used to enable interrupt by ibus counter overflow.
    #[inline(always)]
    pub fn ibus_cnt_ovf(&self) -> IBUS_CNT_OVF_R {
        IBUS_CNT_OVF_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - The bit is used to enable interrupt by dbus counter overflow.
    #[inline(always)]
    pub fn dbus_cnt_ovf(&self) -> DBUS_CNT_OVF_R {
        DBUS_CNT_OVF_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_ILG_INT_ENA")
            .field("icache_sync_op_fault", &self.icache_sync_op_fault())
            .field("icache_preload_op_fault", &self.icache_preload_op_fault())
            .field("mmu_entry_fault", &self.mmu_entry_fault())
            .field("ibus_cnt_ovf", &self.ibus_cnt_ovf())
            .field("dbus_cnt_ovf", &self.dbus_cnt_ovf())
            .finish()
    }
}
impl W {
    ///Bit 0 - The bit is used to enable interrupt by sync configurations fault.
    #[inline(always)]
    #[must_use]
    pub fn icache_sync_op_fault(&mut self) -> ICACHE_SYNC_OP_FAULT_W<CACHE_ILG_INT_ENA_SPEC> {
        ICACHE_SYNC_OP_FAULT_W::new(self, 0)
    }
    ///Bit 1 - The bit is used to enable interrupt by preload configurations fault.
    #[inline(always)]
    #[must_use]
    pub fn icache_preload_op_fault(&mut self) -> ICACHE_PRELOAD_OP_FAULT_W<CACHE_ILG_INT_ENA_SPEC> {
        ICACHE_PRELOAD_OP_FAULT_W::new(self, 1)
    }
    ///Bit 5 - The bit is used to enable interrupt by mmu entry fault.
    #[inline(always)]
    #[must_use]
    pub fn mmu_entry_fault(&mut self) -> MMU_ENTRY_FAULT_W<CACHE_ILG_INT_ENA_SPEC> {
        MMU_ENTRY_FAULT_W::new(self, 5)
    }
    ///Bit 7 - The bit is used to enable interrupt by ibus counter overflow.
    #[inline(always)]
    #[must_use]
    pub fn ibus_cnt_ovf(&mut self) -> IBUS_CNT_OVF_W<CACHE_ILG_INT_ENA_SPEC> {
        IBUS_CNT_OVF_W::new(self, 7)
    }
    ///Bit 8 - The bit is used to enable interrupt by dbus counter overflow.
    #[inline(always)]
    #[must_use]
    pub fn dbus_cnt_ovf(&mut self) -> DBUS_CNT_OVF_W<CACHE_ILG_INT_ENA_SPEC> {
        DBUS_CNT_OVF_W::new(self, 8)
    }
}
/**This description will be updated in the near future.

You can [`read`](crate::generic::Reg::read) this register and get [`cache_ilg_int_ena::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cache_ilg_int_ena::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CACHE_ILG_INT_ENA_SPEC;
impl crate::RegisterSpec for CACHE_ILG_INT_ENA_SPEC {
    type Ux = u32;
}
///`read()` method returns [`cache_ilg_int_ena::R`](R) reader structure
impl crate::Readable for CACHE_ILG_INT_ENA_SPEC {}
///`write(|w| ..)` method takes [`cache_ilg_int_ena::W`](W) writer structure
impl crate::Writable for CACHE_ILG_INT_ENA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CACHE_ILG_INT_ENA to value 0
impl crate::Resettable for CACHE_ILG_INT_ENA_SPEC {
    const RESET_VALUE: u32 = 0;
}
