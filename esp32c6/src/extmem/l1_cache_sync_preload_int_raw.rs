#[doc = "Register `L1_CACHE_SYNC_PRELOAD_INT_RAW` reader"]
pub struct R(crate::R<L1_CACHE_SYNC_PRELOAD_INT_RAW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<L1_CACHE_SYNC_PRELOAD_INT_RAW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<L1_CACHE_SYNC_PRELOAD_INT_RAW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<L1_CACHE_SYNC_PRELOAD_INT_RAW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `L1_ICACHE0_PLD_DONE_INT_RAW` reader - The raw bit of the interrupt that occurs only when L1-ICache0 preload-operation is done."]
pub type L1_ICACHE0_PLD_DONE_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `L1_ICACHE1_PLD_DONE_INT_RAW` reader - The raw bit of the interrupt that occurs only when L1-ICache1 preload-operation is done."]
pub type L1_ICACHE1_PLD_DONE_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `L1_ICACHE2_PLD_DONE_INT_RAW` reader - Reserved"]
pub type L1_ICACHE2_PLD_DONE_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `L1_ICACHE3_PLD_DONE_INT_RAW` reader - Reserved"]
pub type L1_ICACHE3_PLD_DONE_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `L1_CACHE_PLD_DONE_INT_RAW` reader - The raw bit of the interrupt that occurs only when L1-Cache preload-operation is done."]
pub type L1_CACHE_PLD_DONE_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `CACHE_SYNC_DONE_INT_RAW` reader - The raw bit of the interrupt that occurs only when Cache sync-operation is done."]
pub type CACHE_SYNC_DONE_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `L1_ICACHE0_PLD_ERR_INT_RAW` reader - The raw bit of the interrupt that occurs only when L1-ICache0 preload-operation error occurs."]
pub type L1_ICACHE0_PLD_ERR_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `L1_ICACHE1_PLD_ERR_INT_RAW` reader - The raw bit of the interrupt that occurs only when L1-ICache1 preload-operation error occurs."]
pub type L1_ICACHE1_PLD_ERR_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `L1_ICACHE2_PLD_ERR_INT_RAW` reader - Reserved"]
pub type L1_ICACHE2_PLD_ERR_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `L1_ICACHE3_PLD_ERR_INT_RAW` reader - Reserved"]
pub type L1_ICACHE3_PLD_ERR_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `L1_CACHE_PLD_ERR_INT_RAW` reader - The raw bit of the interrupt that occurs only when L1-Cache preload-operation error occurs."]
pub type L1_CACHE_PLD_ERR_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `CACHE_SYNC_ERR_INT_RAW` reader - The raw bit of the interrupt that occurs only when Cache sync-operation error occurs."]
pub type CACHE_SYNC_ERR_INT_RAW_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - The raw bit of the interrupt that occurs only when L1-ICache0 preload-operation is done."]
    #[inline(always)]
    pub fn l1_icache0_pld_done_int_raw(&self) -> L1_ICACHE0_PLD_DONE_INT_RAW_R {
        L1_ICACHE0_PLD_DONE_INT_RAW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The raw bit of the interrupt that occurs only when L1-ICache1 preload-operation is done."]
    #[inline(always)]
    pub fn l1_icache1_pld_done_int_raw(&self) -> L1_ICACHE1_PLD_DONE_INT_RAW_R {
        L1_ICACHE1_PLD_DONE_INT_RAW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn l1_icache2_pld_done_int_raw(&self) -> L1_ICACHE2_PLD_DONE_INT_RAW_R {
        L1_ICACHE2_PLD_DONE_INT_RAW_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn l1_icache3_pld_done_int_raw(&self) -> L1_ICACHE3_PLD_DONE_INT_RAW_R {
        L1_ICACHE3_PLD_DONE_INT_RAW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The raw bit of the interrupt that occurs only when L1-Cache preload-operation is done."]
    #[inline(always)]
    pub fn l1_cache_pld_done_int_raw(&self) -> L1_CACHE_PLD_DONE_INT_RAW_R {
        L1_CACHE_PLD_DONE_INT_RAW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - The raw bit of the interrupt that occurs only when Cache sync-operation is done."]
    #[inline(always)]
    pub fn cache_sync_done_int_raw(&self) -> CACHE_SYNC_DONE_INT_RAW_R {
        CACHE_SYNC_DONE_INT_RAW_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The raw bit of the interrupt that occurs only when L1-ICache0 preload-operation error occurs."]
    #[inline(always)]
    pub fn l1_icache0_pld_err_int_raw(&self) -> L1_ICACHE0_PLD_ERR_INT_RAW_R {
        L1_ICACHE0_PLD_ERR_INT_RAW_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The raw bit of the interrupt that occurs only when L1-ICache1 preload-operation error occurs."]
    #[inline(always)]
    pub fn l1_icache1_pld_err_int_raw(&self) -> L1_ICACHE1_PLD_ERR_INT_RAW_R {
        L1_ICACHE1_PLD_ERR_INT_RAW_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Reserved"]
    #[inline(always)]
    pub fn l1_icache2_pld_err_int_raw(&self) -> L1_ICACHE2_PLD_ERR_INT_RAW_R {
        L1_ICACHE2_PLD_ERR_INT_RAW_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Reserved"]
    #[inline(always)]
    pub fn l1_icache3_pld_err_int_raw(&self) -> L1_ICACHE3_PLD_ERR_INT_RAW_R {
        L1_ICACHE3_PLD_ERR_INT_RAW_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The raw bit of the interrupt that occurs only when L1-Cache preload-operation error occurs."]
    #[inline(always)]
    pub fn l1_cache_pld_err_int_raw(&self) -> L1_CACHE_PLD_ERR_INT_RAW_R {
        L1_CACHE_PLD_ERR_INT_RAW_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - The raw bit of the interrupt that occurs only when Cache sync-operation error occurs."]
    #[inline(always)]
    pub fn cache_sync_err_int_raw(&self) -> CACHE_SYNC_ERR_INT_RAW_R {
        CACHE_SYNC_ERR_INT_RAW_R::new(((self.bits >> 13) & 1) != 0)
    }
}
#[doc = "Sync Preload operation Interrupt raw register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l1_cache_sync_preload_int_raw](index.html) module"]
pub struct L1_CACHE_SYNC_PRELOAD_INT_RAW_SPEC;
impl crate::RegisterSpec for L1_CACHE_SYNC_PRELOAD_INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [l1_cache_sync_preload_int_raw::R](R) reader structure"]
impl crate::Readable for L1_CACHE_SYNC_PRELOAD_INT_RAW_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets L1_CACHE_SYNC_PRELOAD_INT_RAW to value 0"]
impl crate::Resettable for L1_CACHE_SYNC_PRELOAD_INT_RAW_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
