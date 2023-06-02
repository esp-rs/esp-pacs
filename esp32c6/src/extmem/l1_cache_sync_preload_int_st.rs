#[doc = "Register `L1_CACHE_SYNC_PRELOAD_INT_ST` reader"]
pub struct R(crate::R<L1_CACHE_SYNC_PRELOAD_INT_ST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<L1_CACHE_SYNC_PRELOAD_INT_ST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<L1_CACHE_SYNC_PRELOAD_INT_ST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<L1_CACHE_SYNC_PRELOAD_INT_ST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `L1_ICACHE0_PLD_DONE_INT_ST` reader - The bit indicates the status of the interrupt that occurs only when L1-ICache0 preload-operation is done."]
pub type L1_ICACHE0_PLD_DONE_INT_ST_R = crate::BitReader;
#[doc = "Field `L1_ICACHE1_PLD_DONE_INT_ST` reader - The bit indicates the status of the interrupt that occurs only when L1-ICache1 preload-operation is done."]
pub type L1_ICACHE1_PLD_DONE_INT_ST_R = crate::BitReader;
#[doc = "Field `L1_ICACHE2_PLD_DONE_INT_ST` reader - Reserved"]
pub type L1_ICACHE2_PLD_DONE_INT_ST_R = crate::BitReader;
#[doc = "Field `L1_ICACHE3_PLD_DONE_INT_ST` reader - Reserved"]
pub type L1_ICACHE3_PLD_DONE_INT_ST_R = crate::BitReader;
#[doc = "Field `L1_CACHE_PLD_DONE_INT_ST` reader - The bit indicates the status of the interrupt that occurs only when L1-Cache preload-operation is done."]
pub type L1_CACHE_PLD_DONE_INT_ST_R = crate::BitReader;
#[doc = "Field `CACHE_SYNC_DONE_INT_ST` reader - The bit indicates the status of the interrupt that occurs only when Cache sync-operation is done."]
pub type CACHE_SYNC_DONE_INT_ST_R = crate::BitReader;
#[doc = "Field `L1_ICACHE0_PLD_ERR_INT_ST` reader - The bit indicates the status of the interrupt of L1-ICache0 preload-operation error."]
pub type L1_ICACHE0_PLD_ERR_INT_ST_R = crate::BitReader;
#[doc = "Field `L1_ICACHE1_PLD_ERR_INT_ST` reader - The bit indicates the status of the interrupt of L1-ICache1 preload-operation error."]
pub type L1_ICACHE1_PLD_ERR_INT_ST_R = crate::BitReader;
#[doc = "Field `L1_ICACHE2_PLD_ERR_INT_ST` reader - Reserved"]
pub type L1_ICACHE2_PLD_ERR_INT_ST_R = crate::BitReader;
#[doc = "Field `L1_ICACHE3_PLD_ERR_INT_ST` reader - Reserved"]
pub type L1_ICACHE3_PLD_ERR_INT_ST_R = crate::BitReader;
#[doc = "Field `L1_CACHE_PLD_ERR_INT_ST` reader - The bit indicates the status of the interrupt of L1-Cache preload-operation error."]
pub type L1_CACHE_PLD_ERR_INT_ST_R = crate::BitReader;
#[doc = "Field `CACHE_SYNC_ERR_INT_ST` reader - The bit indicates the status of the interrupt of Cache sync-operation error."]
pub type CACHE_SYNC_ERR_INT_ST_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The bit indicates the status of the interrupt that occurs only when L1-ICache0 preload-operation is done."]
    #[inline(always)]
    pub fn l1_icache0_pld_done_int_st(&self) -> L1_ICACHE0_PLD_DONE_INT_ST_R {
        L1_ICACHE0_PLD_DONE_INT_ST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The bit indicates the status of the interrupt that occurs only when L1-ICache1 preload-operation is done."]
    #[inline(always)]
    pub fn l1_icache1_pld_done_int_st(&self) -> L1_ICACHE1_PLD_DONE_INT_ST_R {
        L1_ICACHE1_PLD_DONE_INT_ST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn l1_icache2_pld_done_int_st(&self) -> L1_ICACHE2_PLD_DONE_INT_ST_R {
        L1_ICACHE2_PLD_DONE_INT_ST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn l1_icache3_pld_done_int_st(&self) -> L1_ICACHE3_PLD_DONE_INT_ST_R {
        L1_ICACHE3_PLD_DONE_INT_ST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The bit indicates the status of the interrupt that occurs only when L1-Cache preload-operation is done."]
    #[inline(always)]
    pub fn l1_cache_pld_done_int_st(&self) -> L1_CACHE_PLD_DONE_INT_ST_R {
        L1_CACHE_PLD_DONE_INT_ST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - The bit indicates the status of the interrupt that occurs only when Cache sync-operation is done."]
    #[inline(always)]
    pub fn cache_sync_done_int_st(&self) -> CACHE_SYNC_DONE_INT_ST_R {
        CACHE_SYNC_DONE_INT_ST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The bit indicates the status of the interrupt of L1-ICache0 preload-operation error."]
    #[inline(always)]
    pub fn l1_icache0_pld_err_int_st(&self) -> L1_ICACHE0_PLD_ERR_INT_ST_R {
        L1_ICACHE0_PLD_ERR_INT_ST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The bit indicates the status of the interrupt of L1-ICache1 preload-operation error."]
    #[inline(always)]
    pub fn l1_icache1_pld_err_int_st(&self) -> L1_ICACHE1_PLD_ERR_INT_ST_R {
        L1_ICACHE1_PLD_ERR_INT_ST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Reserved"]
    #[inline(always)]
    pub fn l1_icache2_pld_err_int_st(&self) -> L1_ICACHE2_PLD_ERR_INT_ST_R {
        L1_ICACHE2_PLD_ERR_INT_ST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Reserved"]
    #[inline(always)]
    pub fn l1_icache3_pld_err_int_st(&self) -> L1_ICACHE3_PLD_ERR_INT_ST_R {
        L1_ICACHE3_PLD_ERR_INT_ST_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The bit indicates the status of the interrupt of L1-Cache preload-operation error."]
    #[inline(always)]
    pub fn l1_cache_pld_err_int_st(&self) -> L1_CACHE_PLD_ERR_INT_ST_R {
        L1_CACHE_PLD_ERR_INT_ST_R::new(((self.bits >> 11) & 1) != 0)
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
        f.debug_struct("L1_CACHE_SYNC_PRELOAD_INT_ST")
            .field(
                "l1_icache0_pld_done_int_st",
                &format_args!("{}", self.l1_icache0_pld_done_int_st().bit()),
            )
            .field(
                "l1_icache1_pld_done_int_st",
                &format_args!("{}", self.l1_icache1_pld_done_int_st().bit()),
            )
            .field(
                "l1_icache2_pld_done_int_st",
                &format_args!("{}", self.l1_icache2_pld_done_int_st().bit()),
            )
            .field(
                "l1_icache3_pld_done_int_st",
                &format_args!("{}", self.l1_icache3_pld_done_int_st().bit()),
            )
            .field(
                "l1_cache_pld_done_int_st",
                &format_args!("{}", self.l1_cache_pld_done_int_st().bit()),
            )
            .field(
                "cache_sync_done_int_st",
                &format_args!("{}", self.cache_sync_done_int_st().bit()),
            )
            .field(
                "l1_icache0_pld_err_int_st",
                &format_args!("{}", self.l1_icache0_pld_err_int_st().bit()),
            )
            .field(
                "l1_icache1_pld_err_int_st",
                &format_args!("{}", self.l1_icache1_pld_err_int_st().bit()),
            )
            .field(
                "l1_icache2_pld_err_int_st",
                &format_args!("{}", self.l1_icache2_pld_err_int_st().bit()),
            )
            .field(
                "l1_icache3_pld_err_int_st",
                &format_args!("{}", self.l1_icache3_pld_err_int_st().bit()),
            )
            .field(
                "l1_cache_pld_err_int_st",
                &format_args!("{}", self.l1_cache_pld_err_int_st().bit()),
            )
            .field(
                "cache_sync_err_int_st",
                &format_args!("{}", self.cache_sync_err_int_st().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<L1_CACHE_SYNC_PRELOAD_INT_ST_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "L1-Cache Access Fail Interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l1_cache_sync_preload_int_st](index.html) module"]
pub struct L1_CACHE_SYNC_PRELOAD_INT_ST_SPEC;
impl crate::RegisterSpec for L1_CACHE_SYNC_PRELOAD_INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [l1_cache_sync_preload_int_st::R](R) reader structure"]
impl crate::Readable for L1_CACHE_SYNC_PRELOAD_INT_ST_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets L1_CACHE_SYNC_PRELOAD_INT_ST to value 0"]
impl crate::Resettable for L1_CACHE_SYNC_PRELOAD_INT_ST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
