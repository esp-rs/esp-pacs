#[doc = "Register `CACHE_MMU_FAULT_CONTENT` reader"]
pub struct R(crate::R<CACHE_MMU_FAULT_CONTENT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CACHE_MMU_FAULT_CONTENT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CACHE_MMU_FAULT_CONTENT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CACHE_MMU_FAULT_CONTENT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CACHE_MMU_FAULT_CONTENT` reader - The bits are used to indicate the content of mmu entry which cause mmu fault.."]
pub type CACHE_MMU_FAULT_CONTENT_R = crate::FieldReader;
#[doc = "Field `CACHE_MMU_FAULT_CODE` reader - The right-most 3 bits are used to indicate the operations which cause mmu fault occurrence. 0: default, 1: cpu miss, 2: preload miss, 3: writeback, 4: cpu miss evict recovery address, 5: load miss evict recovery address, 6: external dma tx, 7: external dma rx. The most significant bit is used to indicate this operation occurs in which one icache."]
pub type CACHE_MMU_FAULT_CODE_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - The bits are used to indicate the content of mmu entry which cause mmu fault.."]
    #[inline(always)]
    pub fn cache_mmu_fault_content(&self) -> CACHE_MMU_FAULT_CONTENT_R {
        CACHE_MMU_FAULT_CONTENT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 10:13 - The right-most 3 bits are used to indicate the operations which cause mmu fault occurrence. 0: default, 1: cpu miss, 2: preload miss, 3: writeback, 4: cpu miss evict recovery address, 5: load miss evict recovery address, 6: external dma tx, 7: external dma rx. The most significant bit is used to indicate this operation occurs in which one icache."]
    #[inline(always)]
    pub fn cache_mmu_fault_code(&self) -> CACHE_MMU_FAULT_CODE_R {
        CACHE_MMU_FAULT_CODE_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_MMU_FAULT_CONTENT")
            .field(
                "cache_mmu_fault_content",
                &format_args!("{}", self.cache_mmu_fault_content().bits()),
            )
            .field(
                "cache_mmu_fault_code",
                &format_args!("{}", self.cache_mmu_fault_code().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CACHE_MMU_FAULT_CONTENT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "This description will be updated in the near future.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cache_mmu_fault_content](index.html) module"]
pub struct CACHE_MMU_FAULT_CONTENT_SPEC;
impl crate::RegisterSpec for CACHE_MMU_FAULT_CONTENT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cache_mmu_fault_content::R](R) reader structure"]
impl crate::Readable for CACHE_MMU_FAULT_CONTENT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CACHE_MMU_FAULT_CONTENT to value 0"]
impl crate::Resettable for CACHE_MMU_FAULT_CONTENT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
