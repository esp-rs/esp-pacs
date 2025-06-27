#[doc = "Register `PRO_DCACHE_DBUG2` reader"]
pub type R = crate::R<PRO_DCACHE_DBUG2_SPEC>;
#[doc = "Field `PRO_CACHE_VADDR` reader - "]
pub type PRO_CACHE_VADDR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:26"]
    #[inline(always)]
    pub fn pro_cache_vaddr(&self) -> PRO_CACHE_VADDR_R {
        PRO_CACHE_VADDR_R::new(self.bits & 0x07ff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRO_DCACHE_DBUG2")
            .field("pro_cache_vaddr", &self.pro_cache_vaddr())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`pro_dcache_dbug2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRO_DCACHE_DBUG2_SPEC;
impl crate::RegisterSpec for PRO_DCACHE_DBUG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pro_dcache_dbug2::R`](R) reader structure"]
impl crate::Readable for PRO_DCACHE_DBUG2_SPEC {}
#[doc = "`reset()` method sets PRO_DCACHE_DBUG2 to value 0"]
impl crate::Resettable for PRO_DCACHE_DBUG2_SPEC {}
