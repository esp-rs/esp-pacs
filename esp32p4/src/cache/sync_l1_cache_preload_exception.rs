///Register `SYNC_L1_CACHE_PRELOAD_EXCEPTION` reader
pub type R = crate::R<SYNC_L1_CACHE_PRELOAD_EXCEPTION_SPEC>;
///Field `L1_ICACHE0_PLD_ERR_CODE` reader - The value 2 is Only available which means preload size is error in L1-ICache0.
pub type L1_ICACHE0_PLD_ERR_CODE_R = crate::FieldReader;
///Field `L1_ICACHE1_PLD_ERR_CODE` reader - The value 2 is Only available which means preload size is error in L1-ICache1.
pub type L1_ICACHE1_PLD_ERR_CODE_R = crate::FieldReader;
///Field `L1_ICACHE2_PLD_ERR_CODE` reader - Reserved
pub type L1_ICACHE2_PLD_ERR_CODE_R = crate::FieldReader;
///Field `L1_ICACHE3_PLD_ERR_CODE` reader - Reserved
pub type L1_ICACHE3_PLD_ERR_CODE_R = crate::FieldReader;
///Field `L1_DCACHE_PLD_ERR_CODE` reader - The value 2 is Only available which means preload size is error in L1-DCache.
pub type L1_DCACHE_PLD_ERR_CODE_R = crate::FieldReader;
///Field `SYNC_ERR_CODE` reader - The values 0-2 are available which means sync map, command conflict and size are error in Cache System.
pub type SYNC_ERR_CODE_R = crate::FieldReader;
impl R {
    ///Bits 0:1 - The value 2 is Only available which means preload size is error in L1-ICache0.
    #[inline(always)]
    pub fn l1_icache0_pld_err_code(&self) -> L1_ICACHE0_PLD_ERR_CODE_R {
        L1_ICACHE0_PLD_ERR_CODE_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - The value 2 is Only available which means preload size is error in L1-ICache1.
    #[inline(always)]
    pub fn l1_icache1_pld_err_code(&self) -> L1_ICACHE1_PLD_ERR_CODE_R {
        L1_ICACHE1_PLD_ERR_CODE_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:5 - Reserved
    #[inline(always)]
    pub fn l1_icache2_pld_err_code(&self) -> L1_ICACHE2_PLD_ERR_CODE_R {
        L1_ICACHE2_PLD_ERR_CODE_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 6:7 - Reserved
    #[inline(always)]
    pub fn l1_icache3_pld_err_code(&self) -> L1_ICACHE3_PLD_ERR_CODE_R {
        L1_ICACHE3_PLD_ERR_CODE_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 8:9 - The value 2 is Only available which means preload size is error in L1-DCache.
    #[inline(always)]
    pub fn l1_dcache_pld_err_code(&self) -> L1_DCACHE_PLD_ERR_CODE_R {
        L1_DCACHE_PLD_ERR_CODE_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 12:13 - The values 0-2 are available which means sync map, command conflict and size are error in Cache System.
    #[inline(always)]
    pub fn sync_err_code(&self) -> SYNC_ERR_CODE_R {
        SYNC_ERR_CODE_R::new(((self.bits >> 12) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYNC_L1_CACHE_PRELOAD_EXCEPTION")
            .field("l1_icache0_pld_err_code", &self.l1_icache0_pld_err_code())
            .field("l1_icache1_pld_err_code", &self.l1_icache1_pld_err_code())
            .field("l1_icache2_pld_err_code", &self.l1_icache2_pld_err_code())
            .field("l1_icache3_pld_err_code", &self.l1_icache3_pld_err_code())
            .field("l1_dcache_pld_err_code", &self.l1_dcache_pld_err_code())
            .field("sync_err_code", &self.sync_err_code())
            .finish()
    }
}
/**Cache Sync/Preload Operation exception register

You can [`read`](crate::generic::Reg::read) this register and get [`sync_l1_cache_preload_exception::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SYNC_L1_CACHE_PRELOAD_EXCEPTION_SPEC;
impl crate::RegisterSpec for SYNC_L1_CACHE_PRELOAD_EXCEPTION_SPEC {
    type Ux = u32;
}
///`read()` method returns [`sync_l1_cache_preload_exception::R`](R) reader structure
impl crate::Readable for SYNC_L1_CACHE_PRELOAD_EXCEPTION_SPEC {}
///`reset()` method sets SYNC_L1_CACHE_PRELOAD_EXCEPTION to value 0
impl crate::Resettable for SYNC_L1_CACHE_PRELOAD_EXCEPTION_SPEC {
    const RESET_VALUE: u32 = 0;
}
