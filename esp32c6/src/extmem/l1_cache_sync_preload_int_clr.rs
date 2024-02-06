#[doc = "Register `L1_CACHE_SYNC_PRELOAD_INT_CLR` reader"]
pub type R = crate::R<L1_CACHE_SYNC_PRELOAD_INT_CLR_SPEC>;
#[doc = "Register `L1_CACHE_SYNC_PRELOAD_INT_CLR` writer"]
pub type W = crate::W<L1_CACHE_SYNC_PRELOAD_INT_CLR_SPEC>;
#[doc = "Field `L1_ICACHE0_PLD_DONE_INT_CLR` reader - The bit is used to clear interrupt that occurs only when L1-ICache0 preload-operation is done."]
pub type L1_ICACHE0_PLD_DONE_INT_CLR_R = crate::BitReader;
#[doc = "Field `L1_ICACHE1_PLD_DONE_INT_CLR` reader - The bit is used to clear interrupt that occurs only when L1-ICache1 preload-operation is done."]
pub type L1_ICACHE1_PLD_DONE_INT_CLR_R = crate::BitReader;
#[doc = "Field `L1_ICACHE2_PLD_DONE_INT_CLR` reader - Reserved"]
pub type L1_ICACHE2_PLD_DONE_INT_CLR_R = crate::BitReader;
#[doc = "Field `L1_ICACHE3_PLD_DONE_INT_CLR` reader - Reserved"]
pub type L1_ICACHE3_PLD_DONE_INT_CLR_R = crate::BitReader;
#[doc = "Field `L1_CACHE_PLD_DONE_INT_CLR` writer - The bit is used to clear interrupt that occurs only when L1-Cache preload-operation is done."]
pub type L1_CACHE_PLD_DONE_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CACHE_SYNC_DONE_INT_CLR` writer - The bit is used to clear interrupt that occurs only when Cache sync-operation is done."]
pub type CACHE_SYNC_DONE_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1_ICACHE0_PLD_ERR_INT_CLR` reader - The bit is used to clear interrupt of L1-ICache0 preload-operation error."]
pub type L1_ICACHE0_PLD_ERR_INT_CLR_R = crate::BitReader;
#[doc = "Field `L1_ICACHE1_PLD_ERR_INT_CLR` reader - The bit is used to clear interrupt of L1-ICache1 preload-operation error."]
pub type L1_ICACHE1_PLD_ERR_INT_CLR_R = crate::BitReader;
#[doc = "Field `L1_ICACHE2_PLD_ERR_INT_CLR` reader - Reserved"]
pub type L1_ICACHE2_PLD_ERR_INT_CLR_R = crate::BitReader;
#[doc = "Field `L1_ICACHE3_PLD_ERR_INT_CLR` reader - Reserved"]
pub type L1_ICACHE3_PLD_ERR_INT_CLR_R = crate::BitReader;
#[doc = "Field `L1_CACHE_PLD_ERR_INT_CLR` writer - The bit is used to clear interrupt of L1-Cache preload-operation error."]
pub type L1_CACHE_PLD_ERR_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CACHE_SYNC_ERR_INT_CLR` writer - The bit is used to clear interrupt of Cache sync-operation error."]
pub type CACHE_SYNC_ERR_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The bit is used to clear interrupt that occurs only when L1-ICache0 preload-operation is done."]
    #[inline(always)]
    pub fn l1_icache0_pld_done_int_clr(&self) -> L1_ICACHE0_PLD_DONE_INT_CLR_R {
        L1_ICACHE0_PLD_DONE_INT_CLR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The bit is used to clear interrupt that occurs only when L1-ICache1 preload-operation is done."]
    #[inline(always)]
    pub fn l1_icache1_pld_done_int_clr(&self) -> L1_ICACHE1_PLD_DONE_INT_CLR_R {
        L1_ICACHE1_PLD_DONE_INT_CLR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn l1_icache2_pld_done_int_clr(&self) -> L1_ICACHE2_PLD_DONE_INT_CLR_R {
        L1_ICACHE2_PLD_DONE_INT_CLR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn l1_icache3_pld_done_int_clr(&self) -> L1_ICACHE3_PLD_DONE_INT_CLR_R {
        L1_ICACHE3_PLD_DONE_INT_CLR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - The bit is used to clear interrupt of L1-ICache0 preload-operation error."]
    #[inline(always)]
    pub fn l1_icache0_pld_err_int_clr(&self) -> L1_ICACHE0_PLD_ERR_INT_CLR_R {
        L1_ICACHE0_PLD_ERR_INT_CLR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The bit is used to clear interrupt of L1-ICache1 preload-operation error."]
    #[inline(always)]
    pub fn l1_icache1_pld_err_int_clr(&self) -> L1_ICACHE1_PLD_ERR_INT_CLR_R {
        L1_ICACHE1_PLD_ERR_INT_CLR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Reserved"]
    #[inline(always)]
    pub fn l1_icache2_pld_err_int_clr(&self) -> L1_ICACHE2_PLD_ERR_INT_CLR_R {
        L1_ICACHE2_PLD_ERR_INT_CLR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Reserved"]
    #[inline(always)]
    pub fn l1_icache3_pld_err_int_clr(&self) -> L1_ICACHE3_PLD_ERR_INT_CLR_R {
        L1_ICACHE3_PLD_ERR_INT_CLR_R::new(((self.bits >> 10) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L1_CACHE_SYNC_PRELOAD_INT_CLR")
            .field(
                "l1_icache0_pld_done_int_clr",
                &format_args!("{}", self.l1_icache0_pld_done_int_clr().bit()),
            )
            .field(
                "l1_icache1_pld_done_int_clr",
                &format_args!("{}", self.l1_icache1_pld_done_int_clr().bit()),
            )
            .field(
                "l1_icache2_pld_done_int_clr",
                &format_args!("{}", self.l1_icache2_pld_done_int_clr().bit()),
            )
            .field(
                "l1_icache3_pld_done_int_clr",
                &format_args!("{}", self.l1_icache3_pld_done_int_clr().bit()),
            )
            .field(
                "l1_icache0_pld_err_int_clr",
                &format_args!("{}", self.l1_icache0_pld_err_int_clr().bit()),
            )
            .field(
                "l1_icache1_pld_err_int_clr",
                &format_args!("{}", self.l1_icache1_pld_err_int_clr().bit()),
            )
            .field(
                "l1_icache2_pld_err_int_clr",
                &format_args!("{}", self.l1_icache2_pld_err_int_clr().bit()),
            )
            .field(
                "l1_icache3_pld_err_int_clr",
                &format_args!("{}", self.l1_icache3_pld_err_int_clr().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<L1_CACHE_SYNC_PRELOAD_INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 4 - The bit is used to clear interrupt that occurs only when L1-Cache preload-operation is done."]
    #[inline(always)]
    #[must_use]
    pub fn l1_cache_pld_done_int_clr(
        &mut self,
    ) -> L1_CACHE_PLD_DONE_INT_CLR_W<L1_CACHE_SYNC_PRELOAD_INT_CLR_SPEC> {
        L1_CACHE_PLD_DONE_INT_CLR_W::new(self, 4)
    }
    #[doc = "Bit 6 - The bit is used to clear interrupt that occurs only when Cache sync-operation is done."]
    #[inline(always)]
    #[must_use]
    pub fn cache_sync_done_int_clr(
        &mut self,
    ) -> CACHE_SYNC_DONE_INT_CLR_W<L1_CACHE_SYNC_PRELOAD_INT_CLR_SPEC> {
        CACHE_SYNC_DONE_INT_CLR_W::new(self, 6)
    }
    #[doc = "Bit 11 - The bit is used to clear interrupt of L1-Cache preload-operation error."]
    #[inline(always)]
    #[must_use]
    pub fn l1_cache_pld_err_int_clr(
        &mut self,
    ) -> L1_CACHE_PLD_ERR_INT_CLR_W<L1_CACHE_SYNC_PRELOAD_INT_CLR_SPEC> {
        L1_CACHE_PLD_ERR_INT_CLR_W::new(self, 11)
    }
    #[doc = "Bit 13 - The bit is used to clear interrupt of Cache sync-operation error."]
    #[inline(always)]
    #[must_use]
    pub fn cache_sync_err_int_clr(
        &mut self,
    ) -> CACHE_SYNC_ERR_INT_CLR_W<L1_CACHE_SYNC_PRELOAD_INT_CLR_SPEC> {
        CACHE_SYNC_ERR_INT_CLR_W::new(self, 13)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Sync Preload operation Interrupt clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l1_cache_sync_preload_int_clr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1_cache_sync_preload_int_clr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1_CACHE_SYNC_PRELOAD_INT_CLR_SPEC;
impl crate::RegisterSpec for L1_CACHE_SYNC_PRELOAD_INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l1_cache_sync_preload_int_clr::R`](R) reader structure"]
impl crate::Readable for L1_CACHE_SYNC_PRELOAD_INT_CLR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`l1_cache_sync_preload_int_clr::W`](W) writer structure"]
impl crate::Writable for L1_CACHE_SYNC_PRELOAD_INT_CLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets L1_CACHE_SYNC_PRELOAD_INT_CLR to value 0"]
impl crate::Resettable for L1_CACHE_SYNC_PRELOAD_INT_CLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
