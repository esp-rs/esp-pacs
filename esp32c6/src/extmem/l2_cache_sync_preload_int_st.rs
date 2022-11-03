#[doc = "Register `L2_CACHE_SYNC_PRELOAD_INT_ST` reader"]
pub struct R(crate::R<L2_CACHE_SYNC_PRELOAD_INT_ST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<L2_CACHE_SYNC_PRELOAD_INT_ST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<L2_CACHE_SYNC_PRELOAD_INT_ST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<L2_CACHE_SYNC_PRELOAD_INT_ST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `L2_CACHE_PLD_DONE_INT_ST` reader - The bit indicates the status of the interrupt that occurs only when L2-Cache preload-operation is done."]
pub type L2_CACHE_PLD_DONE_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `L2_CACHE_PLD_ERR_INT_ST` reader - The bit indicates the status of the interrupt of L2-Cache preload-operation error."]
pub type L2_CACHE_PLD_ERR_INT_ST_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 5 - The bit indicates the status of the interrupt that occurs only when L2-Cache preload-operation is done."]
    #[inline(always)]
    pub fn l2_cache_pld_done_int_st(&self) -> L2_CACHE_PLD_DONE_INT_ST_R {
        L2_CACHE_PLD_DONE_INT_ST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 12 - The bit indicates the status of the interrupt of L2-Cache preload-operation error."]
    #[inline(always)]
    pub fn l2_cache_pld_err_int_st(&self) -> L2_CACHE_PLD_ERR_INT_ST_R {
        L2_CACHE_PLD_ERR_INT_ST_R::new(((self.bits >> 12) & 1) != 0)
    }
}
#[doc = "L1-Cache Access Fail Interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l2_cache_sync_preload_int_st](index.html) module"]
pub struct L2_CACHE_SYNC_PRELOAD_INT_ST_SPEC;
impl crate::RegisterSpec for L2_CACHE_SYNC_PRELOAD_INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [l2_cache_sync_preload_int_st::R](R) reader structure"]
impl crate::Readable for L2_CACHE_SYNC_PRELOAD_INT_ST_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets L2_CACHE_SYNC_PRELOAD_INT_ST to value 0"]
impl crate::Resettable for L2_CACHE_SYNC_PRELOAD_INT_ST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
