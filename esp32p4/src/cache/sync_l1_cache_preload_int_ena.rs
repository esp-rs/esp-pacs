#[doc = "Register `SYNC_L1_CACHE_PRELOAD_INT_ENA` reader"]
pub type R = crate::R<SYNC_L1_CACHE_PRELOAD_INT_ENA_SPEC>;
#[doc = "Register `SYNC_L1_CACHE_PRELOAD_INT_ENA` writer"]
pub type W = crate::W<SYNC_L1_CACHE_PRELOAD_INT_ENA_SPEC>;
#[doc = "Field `L1_ICACHE0_PLD_DONE_INT_ENA` reader - The bit is used to enable interrupt of L1-ICache0 preload-operation. If preload operation is done, interrupt occurs."]
pub type L1_ICACHE0_PLD_DONE_INT_ENA_R = crate::BitReader;
#[doc = "Field `L1_ICACHE0_PLD_DONE_INT_ENA` writer - The bit is used to enable interrupt of L1-ICache0 preload-operation. If preload operation is done, interrupt occurs."]
pub type L1_ICACHE0_PLD_DONE_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1_ICACHE1_PLD_DONE_INT_ENA` reader - The bit is used to enable interrupt of L1-ICache1 preload-operation. If preload operation is done, interrupt occurs."]
pub type L1_ICACHE1_PLD_DONE_INT_ENA_R = crate::BitReader;
#[doc = "Field `L1_ICACHE1_PLD_DONE_INT_ENA` writer - The bit is used to enable interrupt of L1-ICache1 preload-operation. If preload operation is done, interrupt occurs."]
pub type L1_ICACHE1_PLD_DONE_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1_ICACHE2_PLD_DONE_INT_ENA` reader - Reserved"]
pub type L1_ICACHE2_PLD_DONE_INT_ENA_R = crate::BitReader;
#[doc = "Field `L1_ICACHE3_PLD_DONE_INT_ENA` reader - Reserved"]
pub type L1_ICACHE3_PLD_DONE_INT_ENA_R = crate::BitReader;
#[doc = "Field `L1_DCACHE_PLD_DONE_INT_ENA` reader - The bit is used to enable interrupt of L1-DCache preload-operation. If preload operation is done, interrupt occurs."]
pub type L1_DCACHE_PLD_DONE_INT_ENA_R = crate::BitReader;
#[doc = "Field `L1_DCACHE_PLD_DONE_INT_ENA` writer - The bit is used to enable interrupt of L1-DCache preload-operation. If preload operation is done, interrupt occurs."]
pub type L1_DCACHE_PLD_DONE_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNC_DONE_INT_ENA` reader - The bit is used to enable interrupt of Cache sync-operation done."]
pub type SYNC_DONE_INT_ENA_R = crate::BitReader;
#[doc = "Field `SYNC_DONE_INT_ENA` writer - The bit is used to enable interrupt of Cache sync-operation done."]
pub type SYNC_DONE_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1_ICACHE0_PLD_ERR_INT_ENA` reader - The bit is used to enable interrupt of L1-ICache0 preload-operation error."]
pub type L1_ICACHE0_PLD_ERR_INT_ENA_R = crate::BitReader;
#[doc = "Field `L1_ICACHE0_PLD_ERR_INT_ENA` writer - The bit is used to enable interrupt of L1-ICache0 preload-operation error."]
pub type L1_ICACHE0_PLD_ERR_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1_ICACHE1_PLD_ERR_INT_ENA` reader - The bit is used to enable interrupt of L1-ICache1 preload-operation error."]
pub type L1_ICACHE1_PLD_ERR_INT_ENA_R = crate::BitReader;
#[doc = "Field `L1_ICACHE1_PLD_ERR_INT_ENA` writer - The bit is used to enable interrupt of L1-ICache1 preload-operation error."]
pub type L1_ICACHE1_PLD_ERR_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1_ICACHE2_PLD_ERR_INT_ENA` reader - Reserved"]
pub type L1_ICACHE2_PLD_ERR_INT_ENA_R = crate::BitReader;
#[doc = "Field `L1_ICACHE3_PLD_ERR_INT_ENA` reader - Reserved"]
pub type L1_ICACHE3_PLD_ERR_INT_ENA_R = crate::BitReader;
#[doc = "Field `L1_DCACHE_PLD_ERR_INT_ENA` reader - The bit is used to enable interrupt of L1-DCache preload-operation error."]
pub type L1_DCACHE_PLD_ERR_INT_ENA_R = crate::BitReader;
#[doc = "Field `L1_DCACHE_PLD_ERR_INT_ENA` writer - The bit is used to enable interrupt of L1-DCache preload-operation error."]
pub type L1_DCACHE_PLD_ERR_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNC_ERR_INT_ENA` reader - The bit is used to enable interrupt of Cache sync-operation error."]
pub type SYNC_ERR_INT_ENA_R = crate::BitReader;
#[doc = "Field `SYNC_ERR_INT_ENA` writer - The bit is used to enable interrupt of Cache sync-operation error."]
pub type SYNC_ERR_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The bit is used to enable interrupt of L1-ICache0 preload-operation. If preload operation is done, interrupt occurs."]
    #[inline(always)]
    pub fn l1_icache0_pld_done_int_ena(&self) -> L1_ICACHE0_PLD_DONE_INT_ENA_R {
        L1_ICACHE0_PLD_DONE_INT_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The bit is used to enable interrupt of L1-ICache1 preload-operation. If preload operation is done, interrupt occurs."]
    #[inline(always)]
    pub fn l1_icache1_pld_done_int_ena(&self) -> L1_ICACHE1_PLD_DONE_INT_ENA_R {
        L1_ICACHE1_PLD_DONE_INT_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn l1_icache2_pld_done_int_ena(&self) -> L1_ICACHE2_PLD_DONE_INT_ENA_R {
        L1_ICACHE2_PLD_DONE_INT_ENA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn l1_icache3_pld_done_int_ena(&self) -> L1_ICACHE3_PLD_DONE_INT_ENA_R {
        L1_ICACHE3_PLD_DONE_INT_ENA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The bit is used to enable interrupt of L1-DCache preload-operation. If preload operation is done, interrupt occurs."]
    #[inline(always)]
    pub fn l1_dcache_pld_done_int_ena(&self) -> L1_DCACHE_PLD_DONE_INT_ENA_R {
        L1_DCACHE_PLD_DONE_INT_ENA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - The bit is used to enable interrupt of Cache sync-operation done."]
    #[inline(always)]
    pub fn sync_done_int_ena(&self) -> SYNC_DONE_INT_ENA_R {
        SYNC_DONE_INT_ENA_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The bit is used to enable interrupt of L1-ICache0 preload-operation error."]
    #[inline(always)]
    pub fn l1_icache0_pld_err_int_ena(&self) -> L1_ICACHE0_PLD_ERR_INT_ENA_R {
        L1_ICACHE0_PLD_ERR_INT_ENA_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The bit is used to enable interrupt of L1-ICache1 preload-operation error."]
    #[inline(always)]
    pub fn l1_icache1_pld_err_int_ena(&self) -> L1_ICACHE1_PLD_ERR_INT_ENA_R {
        L1_ICACHE1_PLD_ERR_INT_ENA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Reserved"]
    #[inline(always)]
    pub fn l1_icache2_pld_err_int_ena(&self) -> L1_ICACHE2_PLD_ERR_INT_ENA_R {
        L1_ICACHE2_PLD_ERR_INT_ENA_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Reserved"]
    #[inline(always)]
    pub fn l1_icache3_pld_err_int_ena(&self) -> L1_ICACHE3_PLD_ERR_INT_ENA_R {
        L1_ICACHE3_PLD_ERR_INT_ENA_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The bit is used to enable interrupt of L1-DCache preload-operation error."]
    #[inline(always)]
    pub fn l1_dcache_pld_err_int_ena(&self) -> L1_DCACHE_PLD_ERR_INT_ENA_R {
        L1_DCACHE_PLD_ERR_INT_ENA_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - The bit is used to enable interrupt of Cache sync-operation error."]
    #[inline(always)]
    pub fn sync_err_int_ena(&self) -> SYNC_ERR_INT_ENA_R {
        SYNC_ERR_INT_ENA_R::new(((self.bits >> 13) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYNC_L1_CACHE_PRELOAD_INT_ENA")
            .field(
                "l1_icache0_pld_done_int_ena",
                &self.l1_icache0_pld_done_int_ena().bit(),
            )
            .field(
                "l1_icache1_pld_done_int_ena",
                &self.l1_icache1_pld_done_int_ena().bit(),
            )
            .field(
                "l1_icache2_pld_done_int_ena",
                &self.l1_icache2_pld_done_int_ena().bit(),
            )
            .field(
                "l1_icache3_pld_done_int_ena",
                &self.l1_icache3_pld_done_int_ena().bit(),
            )
            .field(
                "l1_dcache_pld_done_int_ena",
                &self.l1_dcache_pld_done_int_ena().bit(),
            )
            .field("sync_done_int_ena", &self.sync_done_int_ena().bit())
            .field(
                "l1_icache0_pld_err_int_ena",
                &self.l1_icache0_pld_err_int_ena().bit(),
            )
            .field(
                "l1_icache1_pld_err_int_ena",
                &self.l1_icache1_pld_err_int_ena().bit(),
            )
            .field(
                "l1_icache2_pld_err_int_ena",
                &self.l1_icache2_pld_err_int_ena().bit(),
            )
            .field(
                "l1_icache3_pld_err_int_ena",
                &self.l1_icache3_pld_err_int_ena().bit(),
            )
            .field(
                "l1_dcache_pld_err_int_ena",
                &self.l1_dcache_pld_err_int_ena().bit(),
            )
            .field("sync_err_int_ena", &self.sync_err_int_ena().bit())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SYNC_L1_CACHE_PRELOAD_INT_ENA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - The bit is used to enable interrupt of L1-ICache0 preload-operation. If preload operation is done, interrupt occurs."]
    #[inline(always)]
    #[must_use]
    pub fn l1_icache0_pld_done_int_ena(
        &mut self,
    ) -> L1_ICACHE0_PLD_DONE_INT_ENA_W<SYNC_L1_CACHE_PRELOAD_INT_ENA_SPEC> {
        L1_ICACHE0_PLD_DONE_INT_ENA_W::new(self, 0)
    }
    #[doc = "Bit 1 - The bit is used to enable interrupt of L1-ICache1 preload-operation. If preload operation is done, interrupt occurs."]
    #[inline(always)]
    #[must_use]
    pub fn l1_icache1_pld_done_int_ena(
        &mut self,
    ) -> L1_ICACHE1_PLD_DONE_INT_ENA_W<SYNC_L1_CACHE_PRELOAD_INT_ENA_SPEC> {
        L1_ICACHE1_PLD_DONE_INT_ENA_W::new(self, 1)
    }
    #[doc = "Bit 4 - The bit is used to enable interrupt of L1-DCache preload-operation. If preload operation is done, interrupt occurs."]
    #[inline(always)]
    #[must_use]
    pub fn l1_dcache_pld_done_int_ena(
        &mut self,
    ) -> L1_DCACHE_PLD_DONE_INT_ENA_W<SYNC_L1_CACHE_PRELOAD_INT_ENA_SPEC> {
        L1_DCACHE_PLD_DONE_INT_ENA_W::new(self, 4)
    }
    #[doc = "Bit 6 - The bit is used to enable interrupt of Cache sync-operation done."]
    #[inline(always)]
    #[must_use]
    pub fn sync_done_int_ena(&mut self) -> SYNC_DONE_INT_ENA_W<SYNC_L1_CACHE_PRELOAD_INT_ENA_SPEC> {
        SYNC_DONE_INT_ENA_W::new(self, 6)
    }
    #[doc = "Bit 7 - The bit is used to enable interrupt of L1-ICache0 preload-operation error."]
    #[inline(always)]
    #[must_use]
    pub fn l1_icache0_pld_err_int_ena(
        &mut self,
    ) -> L1_ICACHE0_PLD_ERR_INT_ENA_W<SYNC_L1_CACHE_PRELOAD_INT_ENA_SPEC> {
        L1_ICACHE0_PLD_ERR_INT_ENA_W::new(self, 7)
    }
    #[doc = "Bit 8 - The bit is used to enable interrupt of L1-ICache1 preload-operation error."]
    #[inline(always)]
    #[must_use]
    pub fn l1_icache1_pld_err_int_ena(
        &mut self,
    ) -> L1_ICACHE1_PLD_ERR_INT_ENA_W<SYNC_L1_CACHE_PRELOAD_INT_ENA_SPEC> {
        L1_ICACHE1_PLD_ERR_INT_ENA_W::new(self, 8)
    }
    #[doc = "Bit 11 - The bit is used to enable interrupt of L1-DCache preload-operation error."]
    #[inline(always)]
    #[must_use]
    pub fn l1_dcache_pld_err_int_ena(
        &mut self,
    ) -> L1_DCACHE_PLD_ERR_INT_ENA_W<SYNC_L1_CACHE_PRELOAD_INT_ENA_SPEC> {
        L1_DCACHE_PLD_ERR_INT_ENA_W::new(self, 11)
    }
    #[doc = "Bit 13 - The bit is used to enable interrupt of Cache sync-operation error."]
    #[inline(always)]
    #[must_use]
    pub fn sync_err_int_ena(&mut self) -> SYNC_ERR_INT_ENA_W<SYNC_L1_CACHE_PRELOAD_INT_ENA_SPEC> {
        SYNC_ERR_INT_ENA_W::new(self, 13)
    }
}
#[doc = "L1-Cache Access Fail Interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sync_l1_cache_preload_int_ena::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sync_l1_cache_preload_int_ena::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYNC_L1_CACHE_PRELOAD_INT_ENA_SPEC;
impl crate::RegisterSpec for SYNC_L1_CACHE_PRELOAD_INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sync_l1_cache_preload_int_ena::R`](R) reader structure"]
impl crate::Readable for SYNC_L1_CACHE_PRELOAD_INT_ENA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sync_l1_cache_preload_int_ena::W`](W) writer structure"]
impl crate::Writable for SYNC_L1_CACHE_PRELOAD_INT_ENA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYNC_L1_CACHE_PRELOAD_INT_ENA to value 0"]
impl crate::Resettable for SYNC_L1_CACHE_PRELOAD_INT_ENA_SPEC {
    const RESET_VALUE: u32 = 0;
}
