#[doc = "Register `APP_DCACHE_DBUG9` reader"]
pub type R = crate::R<APP_DCACHE_DBUG9_SPEC>;
#[doc = "Field `APP_OPSDRAMADDR_IA` reader - "]
pub type APP_OPSDRAMADDR_IA_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn app_opsdramaddr_ia(&self) -> APP_OPSDRAMADDR_IA_R {
        APP_OPSDRAMADDR_IA_R::new(self.bits & 0x000f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APP_DCACHE_DBUG9")
            .field("app_opsdramaddr_ia", &self.app_opsdramaddr_ia())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_dcache_dbug9::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APP_DCACHE_DBUG9_SPEC;
impl crate::RegisterSpec for APP_DCACHE_DBUG9_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`app_dcache_dbug9::R`](R) reader structure"]
impl crate::Readable for APP_DCACHE_DBUG9_SPEC {}
#[doc = "`reset()` method sets APP_DCACHE_DBUG9 to value 0"]
impl crate::Resettable for APP_DCACHE_DBUG9_SPEC {
    const RESET_VALUE: u32 = 0;
}
