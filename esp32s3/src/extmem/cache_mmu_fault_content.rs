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
pub type CACHE_MMU_FAULT_CONTENT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CACHE_MMU_FAULT_CODE` reader - The right-most 3 bits are used to indicate the operations which cause mmu fault occurrence. 0: default, 1: cpu miss, 2: preload miss, 3: writeback, 4: cpu miss evict recovery address, 5: load miss evict recovery address, 6: external dma tx, 7: external dma rx. The most significant bit is used to indicate this operation occurs in which one icache."]
pub type CACHE_MMU_FAULT_CODE_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:15 - The bits are used to indicate the content of mmu entry which cause mmu fault.."]
    #[inline(always)]
    pub fn cache_mmu_fault_content(&self) -> CACHE_MMU_FAULT_CONTENT_R {
        CACHE_MMU_FAULT_CONTENT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - The right-most 3 bits are used to indicate the operations which cause mmu fault occurrence. 0: default, 1: cpu miss, 2: preload miss, 3: writeback, 4: cpu miss evict recovery address, 5: load miss evict recovery address, 6: external dma tx, 7: external dma rx. The most significant bit is used to indicate this operation occurs in which one icache."]
    #[inline(always)]
    pub fn cache_mmu_fault_code(&self) -> CACHE_MMU_FAULT_CODE_R {
        CACHE_MMU_FAULT_CODE_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
#[doc = "******* Description ***********\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cache_mmu_fault_content](index.html) module"]
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
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
