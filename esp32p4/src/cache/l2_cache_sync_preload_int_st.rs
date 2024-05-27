///Register `L2_CACHE_SYNC_PRELOAD_INT_ST` reader
pub type R = crate::R<L2_CACHE_SYNC_PRELOAD_INT_ST_SPEC>;
///Field `L2_CACHE_PLD_DONE_INT_ST` reader - The bit indicates the status of the interrupt that occurs only when L2-Cache preload-operation is done.
pub type L2_CACHE_PLD_DONE_INT_ST_R = crate::BitReader;
///Field `L2_CACHE_PLD_ERR_INT_ST` reader - The bit indicates the status of the interrupt of L2-Cache preload-operation error.
pub type L2_CACHE_PLD_ERR_INT_ST_R = crate::BitReader;
impl R {
    ///Bit 5 - The bit indicates the status of the interrupt that occurs only when L2-Cache preload-operation is done.
    #[inline(always)]
    pub fn l2_cache_pld_done_int_st(&self) -> L2_CACHE_PLD_DONE_INT_ST_R {
        L2_CACHE_PLD_DONE_INT_ST_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 12 - The bit indicates the status of the interrupt of L2-Cache preload-operation error.
    #[inline(always)]
    pub fn l2_cache_pld_err_int_st(&self) -> L2_CACHE_PLD_ERR_INT_ST_R {
        L2_CACHE_PLD_ERR_INT_ST_R::new(((self.bits >> 12) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L2_CACHE_SYNC_PRELOAD_INT_ST")
            .field("l2_cache_pld_done_int_st", &self.l2_cache_pld_done_int_st())
            .field("l2_cache_pld_err_int_st", &self.l2_cache_pld_err_int_st())
            .finish()
    }
}
/**L1-Cache Access Fail Interrupt status register

You can [`read`](crate::generic::Reg::read) this register and get [`l2_cache_sync_preload_int_st::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct L2_CACHE_SYNC_PRELOAD_INT_ST_SPEC;
impl crate::RegisterSpec for L2_CACHE_SYNC_PRELOAD_INT_ST_SPEC {
    type Ux = u32;
}
///`read()` method returns [`l2_cache_sync_preload_int_st::R`](R) reader structure
impl crate::Readable for L2_CACHE_SYNC_PRELOAD_INT_ST_SPEC {}
///`reset()` method sets L2_CACHE_SYNC_PRELOAD_INT_ST to value 0
impl crate::Resettable for L2_CACHE_SYNC_PRELOAD_INT_ST_SPEC {
    const RESET_VALUE: u32 = 0;
}
