#[doc = "Register `APP_DCACHE_DBUG7` reader"]
pub type R = crate::R<APP_DCACHE_DBUG7_SPEC>;
#[doc = "Field `APP_IRAM1ADDR_IA` reader - "]
pub type APP_IRAM1ADDR_IA_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn app_iram1addr_ia(&self) -> APP_IRAM1ADDR_IA_R {
        APP_IRAM1ADDR_IA_R::new(self.bits & 0x000f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APP_DCACHE_DBUG7")
            .field("app_iram1addr_ia", &self.app_iram1addr_ia().bits())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<APP_DCACHE_DBUG7_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_dcache_dbug7::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APP_DCACHE_DBUG7_SPEC;
impl crate::RegisterSpec for APP_DCACHE_DBUG7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`app_dcache_dbug7::R`](R) reader structure"]
impl crate::Readable for APP_DCACHE_DBUG7_SPEC {}
#[doc = "`reset()` method sets APP_DCACHE_DBUG7 to value 0"]
impl crate::Resettable for APP_DCACHE_DBUG7_SPEC {
    const RESET_VALUE: u32 = 0;
}
