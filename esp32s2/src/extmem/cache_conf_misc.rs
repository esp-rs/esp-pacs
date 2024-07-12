#[doc = "Register `CACHE_CONF_MISC` reader"]
pub type R = crate::R<CACHE_CONF_MISC_SPEC>;
#[doc = "Register `CACHE_CONF_MISC` writer"]
pub type W = crate::W<CACHE_CONF_MISC_SPEC>;
#[doc = "Field `PRO_CACHE_IGNORE_PRELOAD_MMU_ENTRY_FAULT` reader - The bit is used to disable checking mmu entry fault by preload operation."]
pub type PRO_CACHE_IGNORE_PRELOAD_MMU_ENTRY_FAULT_R = crate::BitReader;
#[doc = "Field `PRO_CACHE_IGNORE_PRELOAD_MMU_ENTRY_FAULT` writer - The bit is used to disable checking mmu entry fault by preload operation."]
pub type PRO_CACHE_IGNORE_PRELOAD_MMU_ENTRY_FAULT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRO_CACHE_IGNORE_SYNC_MMU_ENTRY_FAULT` reader - The bit is used to disable checking mmu entry fault by sync operation."]
pub type PRO_CACHE_IGNORE_SYNC_MMU_ENTRY_FAULT_R = crate::BitReader;
#[doc = "Field `PRO_CACHE_IGNORE_SYNC_MMU_ENTRY_FAULT` writer - The bit is used to disable checking mmu entry fault by sync operation."]
pub type PRO_CACHE_IGNORE_SYNC_MMU_ENTRY_FAULT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The bit is used to disable checking mmu entry fault by preload operation."]
    #[inline(always)]
    pub fn pro_cache_ignore_preload_mmu_entry_fault(
        &self,
    ) -> PRO_CACHE_IGNORE_PRELOAD_MMU_ENTRY_FAULT_R {
        PRO_CACHE_IGNORE_PRELOAD_MMU_ENTRY_FAULT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The bit is used to disable checking mmu entry fault by sync operation."]
    #[inline(always)]
    pub fn pro_cache_ignore_sync_mmu_entry_fault(&self) -> PRO_CACHE_IGNORE_SYNC_MMU_ENTRY_FAULT_R {
        PRO_CACHE_IGNORE_SYNC_MMU_ENTRY_FAULT_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_CONF_MISC")
            .field(
                "pro_cache_ignore_preload_mmu_entry_fault",
                &self.pro_cache_ignore_preload_mmu_entry_fault(),
            )
            .field(
                "pro_cache_ignore_sync_mmu_entry_fault",
                &self.pro_cache_ignore_sync_mmu_entry_fault(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - The bit is used to disable checking mmu entry fault by preload operation."]
    #[inline(always)]
    #[must_use]
    pub fn pro_cache_ignore_preload_mmu_entry_fault(
        &mut self,
    ) -> PRO_CACHE_IGNORE_PRELOAD_MMU_ENTRY_FAULT_W<CACHE_CONF_MISC_SPEC> {
        PRO_CACHE_IGNORE_PRELOAD_MMU_ENTRY_FAULT_W::new(self, 0)
    }
    #[doc = "Bit 1 - The bit is used to disable checking mmu entry fault by sync operation."]
    #[inline(always)]
    #[must_use]
    pub fn pro_cache_ignore_sync_mmu_entry_fault(
        &mut self,
    ) -> PRO_CACHE_IGNORE_SYNC_MMU_ENTRY_FAULT_W<CACHE_CONF_MISC_SPEC> {
        PRO_CACHE_IGNORE_SYNC_MMU_ENTRY_FAULT_W::new(self, 1)
    }
}
#[doc = "register description\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_conf_misc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_conf_misc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CACHE_CONF_MISC_SPEC;
impl crate::RegisterSpec for CACHE_CONF_MISC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cache_conf_misc::R`](R) reader structure"]
impl crate::Readable for CACHE_CONF_MISC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cache_conf_misc::W`](W) writer structure"]
impl crate::Writable for CACHE_CONF_MISC_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CACHE_CONF_MISC to value 0x03"]
impl crate::Resettable for CACHE_CONF_MISC_SPEC {
    const RESET_VALUE: u32 = 0x03;
}
