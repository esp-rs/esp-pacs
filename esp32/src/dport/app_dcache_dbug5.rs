#[doc = "Register `APP_DCACHE_DBUG5` reader"]
pub type R = crate::R<APP_DCACHE_DBUG5_SPEC>;
#[doc = "Field `APP_DROM0ADDR0_IA` reader - "]
pub type APP_DROM0ADDR0_IA_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn app_drom0addr0_ia(&self) -> APP_DROM0ADDR0_IA_R {
        APP_DROM0ADDR0_IA_R::new(self.bits & 0x000f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APP_DCACHE_DBUG5")
            .field("app_drom0addr0_ia", &self.app_drom0addr0_ia())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`app_dcache_dbug5::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APP_DCACHE_DBUG5_SPEC;
impl crate::RegisterSpec for APP_DCACHE_DBUG5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`app_dcache_dbug5::R`](R) reader structure"]
impl crate::Readable for APP_DCACHE_DBUG5_SPEC {}
#[doc = "`reset()` method sets APP_DCACHE_DBUG5 to value 0"]
impl crate::Resettable for APP_DCACHE_DBUG5_SPEC {
    const RESET_VALUE: u32 = 0;
}
