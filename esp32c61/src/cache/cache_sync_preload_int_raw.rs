#[doc = "Register `CACHE_SYNC_PRELOAD_INT_RAW` reader"]
pub type R = crate::R<CACHE_SYNC_PRELOAD_INT_RAW_SPEC>;
#[doc = "Register `CACHE_SYNC_PRELOAD_INT_RAW` writer"]
pub type W = crate::W<CACHE_SYNC_PRELOAD_INT_RAW_SPEC>;
#[doc = "Field `ICACHE0_PLD_DONE_INT_RAW` reader - The raw bit of the interrupt that occurs only when L1-ICache0 preload-operation is done."]
pub type ICACHE0_PLD_DONE_INT_RAW_R = crate::BitReader;
#[doc = "Field `ICACHE0_PLD_DONE_INT_RAW` writer - The raw bit of the interrupt that occurs only when L1-ICache0 preload-operation is done."]
pub type ICACHE0_PLD_DONE_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICACHE1_PLD_DONE_INT_RAW` reader - The raw bit of the interrupt that occurs only when L1-ICache1 preload-operation is done."]
pub type ICACHE1_PLD_DONE_INT_RAW_R = crate::BitReader;
#[doc = "Field `ICACHE1_PLD_DONE_INT_RAW` writer - The raw bit of the interrupt that occurs only when L1-ICache1 preload-operation is done."]
pub type ICACHE1_PLD_DONE_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICACHE2_PLD_DONE_INT_RAW` reader - Reserved"]
pub type ICACHE2_PLD_DONE_INT_RAW_R = crate::BitReader;
#[doc = "Field `ICACHE2_PLD_DONE_INT_RAW` writer - Reserved"]
pub type ICACHE2_PLD_DONE_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICACHE3_PLD_DONE_INT_RAW` reader - Reserved"]
pub type ICACHE3_PLD_DONE_INT_RAW_R = crate::BitReader;
#[doc = "Field `ICACHE3_PLD_DONE_INT_RAW` writer - Reserved"]
pub type ICACHE3_PLD_DONE_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CACHE_PLD_DONE_INT_RAW` reader - The raw bit of the interrupt that occurs only when L1-Cache preload-operation is done."]
pub type CACHE_PLD_DONE_INT_RAW_R = crate::BitReader;
#[doc = "Field `CACHE_PLD_DONE_INT_RAW` writer - The raw bit of the interrupt that occurs only when L1-Cache preload-operation is done."]
pub type CACHE_PLD_DONE_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CACHE_SYNC_DONE_INT_RAW` reader - The raw bit of the interrupt that occurs only when Cache sync-operation is done."]
pub type CACHE_SYNC_DONE_INT_RAW_R = crate::BitReader;
#[doc = "Field `CACHE_SYNC_DONE_INT_RAW` writer - The raw bit of the interrupt that occurs only when Cache sync-operation is done."]
pub type CACHE_SYNC_DONE_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICACHE0_PLD_ERR_INT_RAW` reader - The raw bit of the interrupt that occurs only when L1-ICache0 preload-operation error occurs."]
pub type ICACHE0_PLD_ERR_INT_RAW_R = crate::BitReader;
#[doc = "Field `ICACHE0_PLD_ERR_INT_RAW` writer - The raw bit of the interrupt that occurs only when L1-ICache0 preload-operation error occurs."]
pub type ICACHE0_PLD_ERR_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICACHE1_PLD_ERR_INT_RAW` reader - The raw bit of the interrupt that occurs only when L1-ICache1 preload-operation error occurs."]
pub type ICACHE1_PLD_ERR_INT_RAW_R = crate::BitReader;
#[doc = "Field `ICACHE1_PLD_ERR_INT_RAW` writer - The raw bit of the interrupt that occurs only when L1-ICache1 preload-operation error occurs."]
pub type ICACHE1_PLD_ERR_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICACHE2_PLD_ERR_INT_RAW` reader - Reserved"]
pub type ICACHE2_PLD_ERR_INT_RAW_R = crate::BitReader;
#[doc = "Field `ICACHE2_PLD_ERR_INT_RAW` writer - Reserved"]
pub type ICACHE2_PLD_ERR_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICACHE3_PLD_ERR_INT_RAW` reader - Reserved"]
pub type ICACHE3_PLD_ERR_INT_RAW_R = crate::BitReader;
#[doc = "Field `ICACHE3_PLD_ERR_INT_RAW` writer - Reserved"]
pub type ICACHE3_PLD_ERR_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CACHE_PLD_ERR_INT_RAW` reader - The raw bit of the interrupt that occurs only when L1-Cache preload-operation error occurs."]
pub type CACHE_PLD_ERR_INT_RAW_R = crate::BitReader;
#[doc = "Field `CACHE_PLD_ERR_INT_RAW` writer - The raw bit of the interrupt that occurs only when L1-Cache preload-operation error occurs."]
pub type CACHE_PLD_ERR_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CACHE_SYNC_ERR_INT_RAW` reader - The raw bit of the interrupt that occurs only when Cache sync-operation error occurs."]
pub type CACHE_SYNC_ERR_INT_RAW_R = crate::BitReader;
#[doc = "Field `CACHE_SYNC_ERR_INT_RAW` writer - The raw bit of the interrupt that occurs only when Cache sync-operation error occurs."]
pub type CACHE_SYNC_ERR_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The raw bit of the interrupt that occurs only when L1-ICache0 preload-operation is done."]
    #[inline(always)]
    pub fn icache0_pld_done_int_raw(&self) -> ICACHE0_PLD_DONE_INT_RAW_R {
        ICACHE0_PLD_DONE_INT_RAW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The raw bit of the interrupt that occurs only when L1-ICache1 preload-operation is done."]
    #[inline(always)]
    pub fn icache1_pld_done_int_raw(&self) -> ICACHE1_PLD_DONE_INT_RAW_R {
        ICACHE1_PLD_DONE_INT_RAW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn icache2_pld_done_int_raw(&self) -> ICACHE2_PLD_DONE_INT_RAW_R {
        ICACHE2_PLD_DONE_INT_RAW_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn icache3_pld_done_int_raw(&self) -> ICACHE3_PLD_DONE_INT_RAW_R {
        ICACHE3_PLD_DONE_INT_RAW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The raw bit of the interrupt that occurs only when L1-Cache preload-operation is done."]
    #[inline(always)]
    pub fn cache_pld_done_int_raw(&self) -> CACHE_PLD_DONE_INT_RAW_R {
        CACHE_PLD_DONE_INT_RAW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - The raw bit of the interrupt that occurs only when Cache sync-operation is done."]
    #[inline(always)]
    pub fn cache_sync_done_int_raw(&self) -> CACHE_SYNC_DONE_INT_RAW_R {
        CACHE_SYNC_DONE_INT_RAW_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The raw bit of the interrupt that occurs only when L1-ICache0 preload-operation error occurs."]
    #[inline(always)]
    pub fn icache0_pld_err_int_raw(&self) -> ICACHE0_PLD_ERR_INT_RAW_R {
        ICACHE0_PLD_ERR_INT_RAW_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The raw bit of the interrupt that occurs only when L1-ICache1 preload-operation error occurs."]
    #[inline(always)]
    pub fn icache1_pld_err_int_raw(&self) -> ICACHE1_PLD_ERR_INT_RAW_R {
        ICACHE1_PLD_ERR_INT_RAW_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Reserved"]
    #[inline(always)]
    pub fn icache2_pld_err_int_raw(&self) -> ICACHE2_PLD_ERR_INT_RAW_R {
        ICACHE2_PLD_ERR_INT_RAW_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Reserved"]
    #[inline(always)]
    pub fn icache3_pld_err_int_raw(&self) -> ICACHE3_PLD_ERR_INT_RAW_R {
        ICACHE3_PLD_ERR_INT_RAW_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The raw bit of the interrupt that occurs only when L1-Cache preload-operation error occurs."]
    #[inline(always)]
    pub fn cache_pld_err_int_raw(&self) -> CACHE_PLD_ERR_INT_RAW_R {
        CACHE_PLD_ERR_INT_RAW_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - The raw bit of the interrupt that occurs only when Cache sync-operation error occurs."]
    #[inline(always)]
    pub fn cache_sync_err_int_raw(&self) -> CACHE_SYNC_ERR_INT_RAW_R {
        CACHE_SYNC_ERR_INT_RAW_R::new(((self.bits >> 13) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_SYNC_PRELOAD_INT_RAW")
            .field("icache0_pld_done_int_raw", &self.icache0_pld_done_int_raw())
            .field("icache1_pld_done_int_raw", &self.icache1_pld_done_int_raw())
            .field("icache2_pld_done_int_raw", &self.icache2_pld_done_int_raw())
            .field("icache3_pld_done_int_raw", &self.icache3_pld_done_int_raw())
            .field("cache_pld_done_int_raw", &self.cache_pld_done_int_raw())
            .field("cache_sync_done_int_raw", &self.cache_sync_done_int_raw())
            .field("icache0_pld_err_int_raw", &self.icache0_pld_err_int_raw())
            .field("icache1_pld_err_int_raw", &self.icache1_pld_err_int_raw())
            .field("icache2_pld_err_int_raw", &self.icache2_pld_err_int_raw())
            .field("icache3_pld_err_int_raw", &self.icache3_pld_err_int_raw())
            .field("cache_pld_err_int_raw", &self.cache_pld_err_int_raw())
            .field("cache_sync_err_int_raw", &self.cache_sync_err_int_raw())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - The raw bit of the interrupt that occurs only when L1-ICache0 preload-operation is done."]
    #[inline(always)]
    pub fn icache0_pld_done_int_raw(
        &mut self,
    ) -> ICACHE0_PLD_DONE_INT_RAW_W<'_, CACHE_SYNC_PRELOAD_INT_RAW_SPEC> {
        ICACHE0_PLD_DONE_INT_RAW_W::new(self, 0)
    }
    #[doc = "Bit 1 - The raw bit of the interrupt that occurs only when L1-ICache1 preload-operation is done."]
    #[inline(always)]
    pub fn icache1_pld_done_int_raw(
        &mut self,
    ) -> ICACHE1_PLD_DONE_INT_RAW_W<'_, CACHE_SYNC_PRELOAD_INT_RAW_SPEC> {
        ICACHE1_PLD_DONE_INT_RAW_W::new(self, 1)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn icache2_pld_done_int_raw(
        &mut self,
    ) -> ICACHE2_PLD_DONE_INT_RAW_W<'_, CACHE_SYNC_PRELOAD_INT_RAW_SPEC> {
        ICACHE2_PLD_DONE_INT_RAW_W::new(self, 2)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn icache3_pld_done_int_raw(
        &mut self,
    ) -> ICACHE3_PLD_DONE_INT_RAW_W<'_, CACHE_SYNC_PRELOAD_INT_RAW_SPEC> {
        ICACHE3_PLD_DONE_INT_RAW_W::new(self, 3)
    }
    #[doc = "Bit 4 - The raw bit of the interrupt that occurs only when L1-Cache preload-operation is done."]
    #[inline(always)]
    pub fn cache_pld_done_int_raw(
        &mut self,
    ) -> CACHE_PLD_DONE_INT_RAW_W<'_, CACHE_SYNC_PRELOAD_INT_RAW_SPEC> {
        CACHE_PLD_DONE_INT_RAW_W::new(self, 4)
    }
    #[doc = "Bit 6 - The raw bit of the interrupt that occurs only when Cache sync-operation is done."]
    #[inline(always)]
    pub fn cache_sync_done_int_raw(
        &mut self,
    ) -> CACHE_SYNC_DONE_INT_RAW_W<'_, CACHE_SYNC_PRELOAD_INT_RAW_SPEC> {
        CACHE_SYNC_DONE_INT_RAW_W::new(self, 6)
    }
    #[doc = "Bit 7 - The raw bit of the interrupt that occurs only when L1-ICache0 preload-operation error occurs."]
    #[inline(always)]
    pub fn icache0_pld_err_int_raw(
        &mut self,
    ) -> ICACHE0_PLD_ERR_INT_RAW_W<'_, CACHE_SYNC_PRELOAD_INT_RAW_SPEC> {
        ICACHE0_PLD_ERR_INT_RAW_W::new(self, 7)
    }
    #[doc = "Bit 8 - The raw bit of the interrupt that occurs only when L1-ICache1 preload-operation error occurs."]
    #[inline(always)]
    pub fn icache1_pld_err_int_raw(
        &mut self,
    ) -> ICACHE1_PLD_ERR_INT_RAW_W<'_, CACHE_SYNC_PRELOAD_INT_RAW_SPEC> {
        ICACHE1_PLD_ERR_INT_RAW_W::new(self, 8)
    }
    #[doc = "Bit 9 - Reserved"]
    #[inline(always)]
    pub fn icache2_pld_err_int_raw(
        &mut self,
    ) -> ICACHE2_PLD_ERR_INT_RAW_W<'_, CACHE_SYNC_PRELOAD_INT_RAW_SPEC> {
        ICACHE2_PLD_ERR_INT_RAW_W::new(self, 9)
    }
    #[doc = "Bit 10 - Reserved"]
    #[inline(always)]
    pub fn icache3_pld_err_int_raw(
        &mut self,
    ) -> ICACHE3_PLD_ERR_INT_RAW_W<'_, CACHE_SYNC_PRELOAD_INT_RAW_SPEC> {
        ICACHE3_PLD_ERR_INT_RAW_W::new(self, 10)
    }
    #[doc = "Bit 11 - The raw bit of the interrupt that occurs only when L1-Cache preload-operation error occurs."]
    #[inline(always)]
    pub fn cache_pld_err_int_raw(
        &mut self,
    ) -> CACHE_PLD_ERR_INT_RAW_W<'_, CACHE_SYNC_PRELOAD_INT_RAW_SPEC> {
        CACHE_PLD_ERR_INT_RAW_W::new(self, 11)
    }
    #[doc = "Bit 13 - The raw bit of the interrupt that occurs only when Cache sync-operation error occurs."]
    #[inline(always)]
    pub fn cache_sync_err_int_raw(
        &mut self,
    ) -> CACHE_SYNC_ERR_INT_RAW_W<'_, CACHE_SYNC_PRELOAD_INT_RAW_SPEC> {
        CACHE_SYNC_ERR_INT_RAW_W::new(self, 13)
    }
}
#[doc = "Sync Preload operation Interrupt raw register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_sync_preload_int_raw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_sync_preload_int_raw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CACHE_SYNC_PRELOAD_INT_RAW_SPEC;
impl crate::RegisterSpec for CACHE_SYNC_PRELOAD_INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cache_sync_preload_int_raw::R`](R) reader structure"]
impl crate::Readable for CACHE_SYNC_PRELOAD_INT_RAW_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cache_sync_preload_int_raw::W`](W) writer structure"]
impl crate::Writable for CACHE_SYNC_PRELOAD_INT_RAW_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CACHE_SYNC_PRELOAD_INT_RAW to value 0"]
impl crate::Resettable for CACHE_SYNC_PRELOAD_INT_RAW_SPEC {}
