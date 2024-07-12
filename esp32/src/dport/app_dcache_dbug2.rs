#[doc = "Register `APP_DCACHE_DBUG2` reader"]
pub type R = crate::R<APP_DCACHE_DBUG2_SPEC>;
#[doc = "Field `APP_CACHE_VADDR` reader - "]
pub type APP_CACHE_VADDR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:26"]
    #[inline(always)]
    pub fn app_cache_vaddr(&self) -> APP_CACHE_VADDR_R {
        APP_CACHE_VADDR_R::new(self.bits & 0x07ff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APP_DCACHE_DBUG2")
            .field("app_cache_vaddr", &self.app_cache_vaddr())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`app_dcache_dbug2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APP_DCACHE_DBUG2_SPEC;
impl crate::RegisterSpec for APP_DCACHE_DBUG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`app_dcache_dbug2::R`](R) reader structure"]
impl crate::Readable for APP_DCACHE_DBUG2_SPEC {}
#[doc = "`reset()` method sets APP_DCACHE_DBUG2 to value 0"]
impl crate::Resettable for APP_DCACHE_DBUG2_SPEC {
    const RESET_VALUE: u32 = 0;
}
