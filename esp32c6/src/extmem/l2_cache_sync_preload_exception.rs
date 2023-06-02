#[doc = "Register `L2_CACHE_SYNC_PRELOAD_EXCEPTION` reader"]
pub struct R(crate::R<L2_CACHE_SYNC_PRELOAD_EXCEPTION_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<L2_CACHE_SYNC_PRELOAD_EXCEPTION_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<L2_CACHE_SYNC_PRELOAD_EXCEPTION_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<L2_CACHE_SYNC_PRELOAD_EXCEPTION_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `L2_CACHE_PLD_ERR_CODE` reader - The value 2 is Only available which means preload size is error in L2-Cache."]
pub type L2_CACHE_PLD_ERR_CODE_R = crate::FieldReader;
impl R {
    #[doc = "Bits 10:11 - The value 2 is Only available which means preload size is error in L2-Cache."]
    #[inline(always)]
    pub fn l2_cache_pld_err_code(&self) -> L2_CACHE_PLD_ERR_CODE_R {
        L2_CACHE_PLD_ERR_CODE_R::new(((self.bits >> 10) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L2_CACHE_SYNC_PRELOAD_EXCEPTION")
            .field(
                "l2_cache_pld_err_code",
                &format_args!("{}", self.l2_cache_pld_err_code().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<L2_CACHE_SYNC_PRELOAD_EXCEPTION_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Cache Sync/Preload Operation exception register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l2_cache_sync_preload_exception](index.html) module"]
pub struct L2_CACHE_SYNC_PRELOAD_EXCEPTION_SPEC;
impl crate::RegisterSpec for L2_CACHE_SYNC_PRELOAD_EXCEPTION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [l2_cache_sync_preload_exception::R](R) reader structure"]
impl crate::Readable for L2_CACHE_SYNC_PRELOAD_EXCEPTION_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets L2_CACHE_SYNC_PRELOAD_EXCEPTION to value 0"]
impl crate::Resettable for L2_CACHE_SYNC_PRELOAD_EXCEPTION_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
