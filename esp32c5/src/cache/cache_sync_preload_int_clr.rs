#[doc = "Register `CACHE_SYNC_PRELOAD_INT_CLR` writer"]
pub type W = crate::W<CACHE_SYNC_PRELOAD_INT_CLR_SPEC>;
#[doc = "Field `ICACHE2_PLD_DONE_INT_CLR` writer - Reserved"]
pub type ICACHE2_PLD_DONE_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CACHE_PLD_DONE_INT_CLR` writer - The bit is used to clear interrupt that occurs only when Cache preload-operation is done."]
pub type CACHE_PLD_DONE_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNC_DONE_INT_CLR` writer - The bit is used to clear interrupt that occurs only when Cache sync-operation is done."]
pub type SYNC_DONE_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICACHE2_PLD_ERR_INT_CLR` writer - Reserved"]
pub type ICACHE2_PLD_ERR_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CACHE_PLD_ERR_INT_CLR` writer - The bit is used to clear interrupt of Cache preload-operation error."]
pub type CACHE_PLD_ERR_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNC_ERR_INT_CLR` writer - The bit is used to clear interrupt of Cache sync-operation error."]
pub type SYNC_ERR_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CACHE_SYNC_PRELOAD_INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn icache2_pld_done_int_clr(
        &mut self,
    ) -> ICACHE2_PLD_DONE_INT_CLR_W<'_, CACHE_SYNC_PRELOAD_INT_CLR_SPEC> {
        ICACHE2_PLD_DONE_INT_CLR_W::new(self, 2)
    }
    #[doc = "Bit 4 - The bit is used to clear interrupt that occurs only when Cache preload-operation is done."]
    #[inline(always)]
    pub fn cache_pld_done_int_clr(
        &mut self,
    ) -> CACHE_PLD_DONE_INT_CLR_W<'_, CACHE_SYNC_PRELOAD_INT_CLR_SPEC> {
        CACHE_PLD_DONE_INT_CLR_W::new(self, 4)
    }
    #[doc = "Bit 6 - The bit is used to clear interrupt that occurs only when Cache sync-operation is done."]
    #[inline(always)]
    pub fn sync_done_int_clr(
        &mut self,
    ) -> SYNC_DONE_INT_CLR_W<'_, CACHE_SYNC_PRELOAD_INT_CLR_SPEC> {
        SYNC_DONE_INT_CLR_W::new(self, 6)
    }
    #[doc = "Bit 9 - Reserved"]
    #[inline(always)]
    pub fn icache2_pld_err_int_clr(
        &mut self,
    ) -> ICACHE2_PLD_ERR_INT_CLR_W<'_, CACHE_SYNC_PRELOAD_INT_CLR_SPEC> {
        ICACHE2_PLD_ERR_INT_CLR_W::new(self, 9)
    }
    #[doc = "Bit 11 - The bit is used to clear interrupt of Cache preload-operation error."]
    #[inline(always)]
    pub fn cache_pld_err_int_clr(
        &mut self,
    ) -> CACHE_PLD_ERR_INT_CLR_W<'_, CACHE_SYNC_PRELOAD_INT_CLR_SPEC> {
        CACHE_PLD_ERR_INT_CLR_W::new(self, 11)
    }
    #[doc = "Bit 13 - The bit is used to clear interrupt of Cache sync-operation error."]
    #[inline(always)]
    pub fn sync_err_int_clr(&mut self) -> SYNC_ERR_INT_CLR_W<'_, CACHE_SYNC_PRELOAD_INT_CLR_SPEC> {
        SYNC_ERR_INT_CLR_W::new(self, 13)
    }
}
#[doc = "Sync Preload operation Interrupt clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_sync_preload_int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CACHE_SYNC_PRELOAD_INT_CLR_SPEC;
impl crate::RegisterSpec for CACHE_SYNC_PRELOAD_INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cache_sync_preload_int_clr::W`](W) writer structure"]
impl crate::Writable for CACHE_SYNC_PRELOAD_INT_CLR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CACHE_SYNC_PRELOAD_INT_CLR to value 0"]
impl crate::Resettable for CACHE_SYNC_PRELOAD_INT_CLR_SPEC {}
