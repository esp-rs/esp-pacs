#[doc = "Register `CACHE_L2_CACHE_SYNC_PRELOAD_EXCEPTION` reader"]
pub type R = crate::R<CACHE_L2_CACHE_SYNC_PRELOAD_EXCEPTION_SPEC>;
#[doc = "Field `CACHE_L2_CACHE_PLD_ERR_CODE` reader - The value 2 is Only available which means preload size is error in L2-Cache."]
pub type CACHE_L2_CACHE_PLD_ERR_CODE_R = crate::FieldReader;
impl R {
    #[doc = "Bits 10:11 - The value 2 is Only available which means preload size is error in L2-Cache."]
    #[inline(always)]
    pub fn cache_l2_cache_pld_err_code(&self) -> CACHE_L2_CACHE_PLD_ERR_CODE_R {
        CACHE_L2_CACHE_PLD_ERR_CODE_R::new(((self.bits >> 10) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_L2_CACHE_SYNC_PRELOAD_EXCEPTION")
            .field(
                "cache_l2_cache_pld_err_code",
                &self.cache_l2_cache_pld_err_code(),
            )
            .finish()
    }
}
#[doc = "Cache Sync/Preload Operation exception register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_l2_cache_sync_preload_exception::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CACHE_L2_CACHE_SYNC_PRELOAD_EXCEPTION_SPEC;
impl crate::RegisterSpec for CACHE_L2_CACHE_SYNC_PRELOAD_EXCEPTION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cache_l2_cache_sync_preload_exception::R`](R) reader structure"]
impl crate::Readable for CACHE_L2_CACHE_SYNC_PRELOAD_EXCEPTION_SPEC {}
#[doc = "`reset()` method sets CACHE_L2_CACHE_SYNC_PRELOAD_EXCEPTION to value 0"]
impl crate::Resettable for CACHE_L2_CACHE_SYNC_PRELOAD_EXCEPTION_SPEC {}
