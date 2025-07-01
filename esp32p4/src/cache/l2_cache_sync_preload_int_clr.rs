#[doc = "Register `L2_CACHE_SYNC_PRELOAD_INT_CLR` writer"]
pub type W = crate::W<L2_CACHE_SYNC_PRELOAD_INT_CLR_SPEC>;
#[doc = "Field `L2_CACHE_PLD_DONE_INT_CLR` writer - The bit is used to clear interrupt that occurs only when L2-Cache preload-operation is done."]
pub type L2_CACHE_PLD_DONE_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L2_CACHE_PLD_ERR_INT_CLR` writer - The bit is used to clear interrupt of L2-Cache preload-operation error."]
pub type L2_CACHE_PLD_ERR_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<L2_CACHE_SYNC_PRELOAD_INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 5 - The bit is used to clear interrupt that occurs only when L2-Cache preload-operation is done."]
    #[inline(always)]
    pub fn l2_cache_pld_done_int_clr(
        &mut self,
    ) -> L2_CACHE_PLD_DONE_INT_CLR_W<L2_CACHE_SYNC_PRELOAD_INT_CLR_SPEC> {
        L2_CACHE_PLD_DONE_INT_CLR_W::new(self, 5)
    }
    #[doc = "Bit 12 - The bit is used to clear interrupt of L2-Cache preload-operation error."]
    #[inline(always)]
    pub fn l2_cache_pld_err_int_clr(
        &mut self,
    ) -> L2_CACHE_PLD_ERR_INT_CLR_W<L2_CACHE_SYNC_PRELOAD_INT_CLR_SPEC> {
        L2_CACHE_PLD_ERR_INT_CLR_W::new(self, 12)
    }
}
#[doc = "Sync Preload operation Interrupt clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2_cache_sync_preload_int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L2_CACHE_SYNC_PRELOAD_INT_CLR_SPEC;
impl crate::RegisterSpec for L2_CACHE_SYNC_PRELOAD_INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`l2_cache_sync_preload_int_clr::W`](W) writer structure"]
impl crate::Writable for L2_CACHE_SYNC_PRELOAD_INT_CLR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets L2_CACHE_SYNC_PRELOAD_INT_CLR to value 0"]
impl crate::Resettable for L2_CACHE_SYNC_PRELOAD_INT_CLR_SPEC {}
