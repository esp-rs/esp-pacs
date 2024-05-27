///Register `APP_DCACHE_DBUG8` reader
pub type R = crate::R<APP_DCACHE_DBUG8_SPEC>;
///Field `APP_IROM0ADDR_IA` reader -
pub type APP_IROM0ADDR_IA_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:19
    #[inline(always)]
    pub fn app_irom0addr_ia(&self) -> APP_IROM0ADDR_IA_R {
        APP_IROM0ADDR_IA_R::new(self.bits & 0x000f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APP_DCACHE_DBUG8")
            .field("app_irom0addr_ia", &self.app_irom0addr_ia())
            .finish()
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`app_dcache_dbug8::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct APP_DCACHE_DBUG8_SPEC;
impl crate::RegisterSpec for APP_DCACHE_DBUG8_SPEC {
    type Ux = u32;
}
///`read()` method returns [`app_dcache_dbug8::R`](R) reader structure
impl crate::Readable for APP_DCACHE_DBUG8_SPEC {}
///`reset()` method sets APP_DCACHE_DBUG8 to value 0
impl crate::Resettable for APP_DCACHE_DBUG8_SPEC {
    const RESET_VALUE: u32 = 0;
}
