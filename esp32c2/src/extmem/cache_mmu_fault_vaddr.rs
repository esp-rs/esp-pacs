#[doc = "Register `CACHE_MMU_FAULT_VADDR` reader"]
pub type R = crate::R<CACHE_MMU_FAULT_VADDR_SPEC>;
#[doc = "Field `CACHE_MMU_FAULT_VADDR` reader - The bits are used to indicate the virtual address which cause mmu fault.."]
pub type CACHE_MMU_FAULT_VADDR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The bits are used to indicate the virtual address which cause mmu fault.."]
    #[inline(always)]
    pub fn cache_mmu_fault_vaddr(&self) -> CACHE_MMU_FAULT_VADDR_R {
        CACHE_MMU_FAULT_VADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_MMU_FAULT_VADDR")
            .field(
                "cache_mmu_fault_vaddr",
                &format_args!("{}", self.cache_mmu_fault_vaddr().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CACHE_MMU_FAULT_VADDR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "This description will be updated in the near future.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cache_mmu_fault_vaddr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CACHE_MMU_FAULT_VADDR_SPEC;
impl crate::RegisterSpec for CACHE_MMU_FAULT_VADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cache_mmu_fault_vaddr::R`](R) reader structure"]
impl crate::Readable for CACHE_MMU_FAULT_VADDR_SPEC {}
#[doc = "`reset()` method sets CACHE_MMU_FAULT_VADDR to value 0"]
impl crate::Resettable for CACHE_MMU_FAULT_VADDR_SPEC {
    const RESET_VALUE: u32 = 0;
}
