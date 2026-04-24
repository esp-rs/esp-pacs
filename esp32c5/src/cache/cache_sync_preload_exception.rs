#[doc = "Register `CACHE_SYNC_PRELOAD_EXCEPTION` reader"]
pub type R = crate::R<CACHE_SYNC_PRELOAD_EXCEPTION_SPEC>;
#[doc = "Field `ICACHE2_PLD_ERR_CODE` reader - Reserved"]
pub type ICACHE2_PLD_ERR_CODE_R = crate::FieldReader;
#[doc = "Field `CACHE_PLD_ERR_CODE` reader - The value 2 is Only available which means preload size is error in Cache."]
pub type CACHE_PLD_ERR_CODE_R = crate::FieldReader;
#[doc = "Field `SYNC_ERR_CODE` reader - The values 0-2 are available which means sync map, command conflict and size are error in Cache System."]
pub type SYNC_ERR_CODE_R = crate::FieldReader;
impl R {
    #[doc = "Bits 4:5 - Reserved"]
    #[inline(always)]
    pub fn icache2_pld_err_code(&self) -> ICACHE2_PLD_ERR_CODE_R {
        ICACHE2_PLD_ERR_CODE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9 - The value 2 is Only available which means preload size is error in Cache."]
    #[inline(always)]
    pub fn cache_pld_err_code(&self) -> CACHE_PLD_ERR_CODE_R {
        CACHE_PLD_ERR_CODE_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - The values 0-2 are available which means sync map, command conflict and size are error in Cache System."]
    #[inline(always)]
    pub fn sync_err_code(&self) -> SYNC_ERR_CODE_R {
        SYNC_ERR_CODE_R::new(((self.bits >> 12) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_SYNC_PRELOAD_EXCEPTION")
            .field("icache2_pld_err_code", &self.icache2_pld_err_code())
            .field("cache_pld_err_code", &self.cache_pld_err_code())
            .field("sync_err_code", &self.sync_err_code())
            .finish()
    }
}
#[doc = "Cache Sync/Preload Operation exception register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_sync_preload_exception::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CACHE_SYNC_PRELOAD_EXCEPTION_SPEC;
impl crate::RegisterSpec for CACHE_SYNC_PRELOAD_EXCEPTION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cache_sync_preload_exception::R`](R) reader structure"]
impl crate::Readable for CACHE_SYNC_PRELOAD_EXCEPTION_SPEC {}
#[doc = "`reset()` method sets CACHE_SYNC_PRELOAD_EXCEPTION to value 0"]
impl crate::Resettable for CACHE_SYNC_PRELOAD_EXCEPTION_SPEC {}
