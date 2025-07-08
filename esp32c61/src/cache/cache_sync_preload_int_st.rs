#[doc = "Register `CACHE_SYNC_PRELOAD_INT_ST` reader"]
pub type R = crate::R<CACHE_SYNC_PRELOAD_INT_ST_SPEC>;
#[doc = "Field `ICACHE0_PLD_DONE_INT_ST` reader - The bit indicates the status of the interrupt that occurs only when L1-ICache0 preload-operation is done."]
pub type ICACHE0_PLD_DONE_INT_ST_R = crate::BitReader;
#[doc = "Field `ICACHE1_PLD_DONE_INT_ST` reader - The bit indicates the status of the interrupt that occurs only when L1-ICache1 preload-operation is done."]
pub type ICACHE1_PLD_DONE_INT_ST_R = crate::BitReader;
#[doc = "Field `ICACHE2_PLD_DONE_INT_ST` reader - Reserved"]
pub type ICACHE2_PLD_DONE_INT_ST_R = crate::BitReader;
#[doc = "Field `ICACHE3_PLD_DONE_INT_ST` reader - Reserved"]
pub type ICACHE3_PLD_DONE_INT_ST_R = crate::BitReader;
#[doc = "Field `CACHE_PLD_DONE_INT_ST` reader - The bit indicates the status of the interrupt that occurs only when L1-Cache preload-operation is done."]
pub type CACHE_PLD_DONE_INT_ST_R = crate::BitReader;
#[doc = "Field `CACHE_SYNC_DONE_INT_ST` reader - The bit indicates the status of the interrupt that occurs only when Cache sync-operation is done."]
pub type CACHE_SYNC_DONE_INT_ST_R = crate::BitReader;
#[doc = "Field `ICACHE0_PLD_ERR_INT_ST` reader - The bit indicates the status of the interrupt of L1-ICache0 preload-operation error."]
pub type ICACHE0_PLD_ERR_INT_ST_R = crate::BitReader;
#[doc = "Field `ICACHE1_PLD_ERR_INT_ST` reader - The bit indicates the status of the interrupt of L1-ICache1 preload-operation error."]
pub type ICACHE1_PLD_ERR_INT_ST_R = crate::BitReader;
#[doc = "Field `ICACHE2_PLD_ERR_INT_ST` reader - Reserved"]
pub type ICACHE2_PLD_ERR_INT_ST_R = crate::BitReader;
#[doc = "Field `ICACHE3_PLD_ERR_INT_ST` reader - Reserved"]
pub type ICACHE3_PLD_ERR_INT_ST_R = crate::BitReader;
#[doc = "Field `CACHE_PLD_ERR_INT_ST` reader - The bit indicates the status of the interrupt of L1-Cache preload-operation error."]
pub type CACHE_PLD_ERR_INT_ST_R = crate::BitReader;
#[doc = "Field `CACHE_SYNC_ERR_INT_ST` reader - The bit indicates the status of the interrupt of Cache sync-operation error."]
pub type CACHE_SYNC_ERR_INT_ST_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The bit indicates the status of the interrupt that occurs only when L1-ICache0 preload-operation is done."]
    #[inline(always)]
    pub fn icache0_pld_done_int_st(&self) -> ICACHE0_PLD_DONE_INT_ST_R {
        ICACHE0_PLD_DONE_INT_ST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The bit indicates the status of the interrupt that occurs only when L1-ICache1 preload-operation is done."]
    #[inline(always)]
    pub fn icache1_pld_done_int_st(&self) -> ICACHE1_PLD_DONE_INT_ST_R {
        ICACHE1_PLD_DONE_INT_ST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn icache2_pld_done_int_st(&self) -> ICACHE2_PLD_DONE_INT_ST_R {
        ICACHE2_PLD_DONE_INT_ST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn icache3_pld_done_int_st(&self) -> ICACHE3_PLD_DONE_INT_ST_R {
        ICACHE3_PLD_DONE_INT_ST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The bit indicates the status of the interrupt that occurs only when L1-Cache preload-operation is done."]
    #[inline(always)]
    pub fn cache_pld_done_int_st(&self) -> CACHE_PLD_DONE_INT_ST_R {
        CACHE_PLD_DONE_INT_ST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - The bit indicates the status of the interrupt that occurs only when Cache sync-operation is done."]
    #[inline(always)]
    pub fn cache_sync_done_int_st(&self) -> CACHE_SYNC_DONE_INT_ST_R {
        CACHE_SYNC_DONE_INT_ST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The bit indicates the status of the interrupt of L1-ICache0 preload-operation error."]
    #[inline(always)]
    pub fn icache0_pld_err_int_st(&self) -> ICACHE0_PLD_ERR_INT_ST_R {
        ICACHE0_PLD_ERR_INT_ST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The bit indicates the status of the interrupt of L1-ICache1 preload-operation error."]
    #[inline(always)]
    pub fn icache1_pld_err_int_st(&self) -> ICACHE1_PLD_ERR_INT_ST_R {
        ICACHE1_PLD_ERR_INT_ST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Reserved"]
    #[inline(always)]
    pub fn icache2_pld_err_int_st(&self) -> ICACHE2_PLD_ERR_INT_ST_R {
        ICACHE2_PLD_ERR_INT_ST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Reserved"]
    #[inline(always)]
    pub fn icache3_pld_err_int_st(&self) -> ICACHE3_PLD_ERR_INT_ST_R {
        ICACHE3_PLD_ERR_INT_ST_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The bit indicates the status of the interrupt of L1-Cache preload-operation error."]
    #[inline(always)]
    pub fn cache_pld_err_int_st(&self) -> CACHE_PLD_ERR_INT_ST_R {
        CACHE_PLD_ERR_INT_ST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - The bit indicates the status of the interrupt of Cache sync-operation error."]
    #[inline(always)]
    pub fn cache_sync_err_int_st(&self) -> CACHE_SYNC_ERR_INT_ST_R {
        CACHE_SYNC_ERR_INT_ST_R::new(((self.bits >> 13) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_SYNC_PRELOAD_INT_ST")
            .field("icache0_pld_done_int_st", &self.icache0_pld_done_int_st())
            .field("icache1_pld_done_int_st", &self.icache1_pld_done_int_st())
            .field("icache2_pld_done_int_st", &self.icache2_pld_done_int_st())
            .field("icache3_pld_done_int_st", &self.icache3_pld_done_int_st())
            .field("cache_pld_done_int_st", &self.cache_pld_done_int_st())
            .field("cache_sync_done_int_st", &self.cache_sync_done_int_st())
            .field("icache0_pld_err_int_st", &self.icache0_pld_err_int_st())
            .field("icache1_pld_err_int_st", &self.icache1_pld_err_int_st())
            .field("icache2_pld_err_int_st", &self.icache2_pld_err_int_st())
            .field("icache3_pld_err_int_st", &self.icache3_pld_err_int_st())
            .field("cache_pld_err_int_st", &self.cache_pld_err_int_st())
            .field("cache_sync_err_int_st", &self.cache_sync_err_int_st())
            .finish()
    }
}
#[doc = "L1-Cache Access Fail Interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_sync_preload_int_st::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CACHE_SYNC_PRELOAD_INT_ST_SPEC;
impl crate::RegisterSpec for CACHE_SYNC_PRELOAD_INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cache_sync_preload_int_st::R`](R) reader structure"]
impl crate::Readable for CACHE_SYNC_PRELOAD_INT_ST_SPEC {}
#[doc = "`reset()` method sets CACHE_SYNC_PRELOAD_INT_ST to value 0"]
impl crate::Resettable for CACHE_SYNC_PRELOAD_INT_ST_SPEC {}
