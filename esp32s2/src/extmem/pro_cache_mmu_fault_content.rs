#[doc = "Register `PRO_CACHE_MMU_FAULT_CONTENT` reader"]
pub struct R(crate::R<PRO_CACHE_MMU_FAULT_CONTENT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRO_CACHE_MMU_FAULT_CONTENT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRO_CACHE_MMU_FAULT_CONTENT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRO_CACHE_MMU_FAULT_CONTENT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PRO_CACHE_MMU_FAULT_CONTENT` reader - The bits are used to indicate the content of mmu entry which cause mmu fault.."]
pub type PRO_CACHE_MMU_FAULT_CONTENT_R = crate::FieldReader<u32>;
#[doc = "Field `PRO_CACHE_MMU_FAULT_CODE` reader - The bits are used to indicate the operations which cause mmu fault occurrence. 0: default, 1: cpu miss, 2: preload miss, 3: flush, 4: cpu miss evict recovery address, 5: load miss evict recovery address, 6: external dma tx, 7: external dma rx"]
pub type PRO_CACHE_MMU_FAULT_CODE_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:16 - The bits are used to indicate the content of mmu entry which cause mmu fault.."]
    #[inline(always)]
    pub fn pro_cache_mmu_fault_content(&self) -> PRO_CACHE_MMU_FAULT_CONTENT_R {
        PRO_CACHE_MMU_FAULT_CONTENT_R::new(self.bits & 0x0001_ffff)
    }
    #[doc = "Bits 17:19 - The bits are used to indicate the operations which cause mmu fault occurrence. 0: default, 1: cpu miss, 2: preload miss, 3: flush, 4: cpu miss evict recovery address, 5: load miss evict recovery address, 6: external dma tx, 7: external dma rx"]
    #[inline(always)]
    pub fn pro_cache_mmu_fault_code(&self) -> PRO_CACHE_MMU_FAULT_CODE_R {
        PRO_CACHE_MMU_FAULT_CODE_R::new(((self.bits >> 17) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRO_CACHE_MMU_FAULT_CONTENT")
            .field(
                "pro_cache_mmu_fault_content",
                &format_args!("{}", self.pro_cache_mmu_fault_content().bits()),
            )
            .field(
                "pro_cache_mmu_fault_code",
                &format_args!("{}", self.pro_cache_mmu_fault_code().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PRO_CACHE_MMU_FAULT_CONTENT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pro_cache_mmu_fault_content](index.html) module"]
pub struct PRO_CACHE_MMU_FAULT_CONTENT_SPEC;
impl crate::RegisterSpec for PRO_CACHE_MMU_FAULT_CONTENT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pro_cache_mmu_fault_content::R](R) reader structure"]
impl crate::Readable for PRO_CACHE_MMU_FAULT_CONTENT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PRO_CACHE_MMU_FAULT_CONTENT to value 0"]
impl crate::Resettable for PRO_CACHE_MMU_FAULT_CONTENT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
