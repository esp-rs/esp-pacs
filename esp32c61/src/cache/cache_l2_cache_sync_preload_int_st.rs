#[doc = "Register `CACHE_L2_CACHE_SYNC_PRELOAD_INT_ST` reader"]
pub type R = crate::R<CACHE_L2_CACHE_SYNC_PRELOAD_INT_ST_SPEC>;
#[doc = "Field `CACHE_L2_CACHE_PLD_DONE_INT_ST` reader - The bit indicates the status of the interrupt that occurs only when L2-Cache preload-operation is done."]
pub type CACHE_L2_CACHE_PLD_DONE_INT_ST_R = crate::BitReader;
#[doc = "Field `CACHE_L2_CACHE_PLD_ERR_INT_ST` reader - The bit indicates the status of the interrupt of L2-Cache preload-operation error."]
pub type CACHE_L2_CACHE_PLD_ERR_INT_ST_R = crate::BitReader;
impl R {
    #[doc = "Bit 5 - The bit indicates the status of the interrupt that occurs only when L2-Cache preload-operation is done."]
    #[inline(always)]
    pub fn cache_l2_cache_pld_done_int_st(&self) -> CACHE_L2_CACHE_PLD_DONE_INT_ST_R {
        CACHE_L2_CACHE_PLD_DONE_INT_ST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 12 - The bit indicates the status of the interrupt of L2-Cache preload-operation error."]
    #[inline(always)]
    pub fn cache_l2_cache_pld_err_int_st(&self) -> CACHE_L2_CACHE_PLD_ERR_INT_ST_R {
        CACHE_L2_CACHE_PLD_ERR_INT_ST_R::new(((self.bits >> 12) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_L2_CACHE_SYNC_PRELOAD_INT_ST")
            .field(
                "cache_l2_cache_pld_done_int_st",
                &self.cache_l2_cache_pld_done_int_st(),
            )
            .field(
                "cache_l2_cache_pld_err_int_st",
                &self.cache_l2_cache_pld_err_int_st(),
            )
            .finish()
    }
}
#[doc = "L1-Cache Access Fail Interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_l2_cache_sync_preload_int_st::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CACHE_L2_CACHE_SYNC_PRELOAD_INT_ST_SPEC;
impl crate::RegisterSpec for CACHE_L2_CACHE_SYNC_PRELOAD_INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cache_l2_cache_sync_preload_int_st::R`](R) reader structure"]
impl crate::Readable for CACHE_L2_CACHE_SYNC_PRELOAD_INT_ST_SPEC {}
#[doc = "`reset()` method sets CACHE_L2_CACHE_SYNC_PRELOAD_INT_ST to value 0"]
impl crate::Resettable for CACHE_L2_CACHE_SYNC_PRELOAD_INT_ST_SPEC {}
