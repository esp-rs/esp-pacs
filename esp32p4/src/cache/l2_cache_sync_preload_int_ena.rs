#[doc = "Register `L2_CACHE_SYNC_PRELOAD_INT_ENA` reader"]
pub type R = crate::R<L2_CACHE_SYNC_PRELOAD_INT_ENA_SPEC>;
#[doc = "Register `L2_CACHE_SYNC_PRELOAD_INT_ENA` writer"]
pub type W = crate::W<L2_CACHE_SYNC_PRELOAD_INT_ENA_SPEC>;
#[doc = "Field `L2_CACHE_PLD_DONE_INT_ENA` reader - The bit is used to enable interrupt of L2-Cache preload-operation done."]
pub type L2_CACHE_PLD_DONE_INT_ENA_R = crate::BitReader;
#[doc = "Field `L2_CACHE_PLD_DONE_INT_ENA` writer - The bit is used to enable interrupt of L2-Cache preload-operation done."]
pub type L2_CACHE_PLD_DONE_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L2_CACHE_PLD_ERR_INT_ENA` reader - The bit is used to enable interrupt of L2-Cache preload-operation error."]
pub type L2_CACHE_PLD_ERR_INT_ENA_R = crate::BitReader;
#[doc = "Field `L2_CACHE_PLD_ERR_INT_ENA` writer - The bit is used to enable interrupt of L2-Cache preload-operation error."]
pub type L2_CACHE_PLD_ERR_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 5 - The bit is used to enable interrupt of L2-Cache preload-operation done."]
    #[inline(always)]
    pub fn l2_cache_pld_done_int_ena(&self) -> L2_CACHE_PLD_DONE_INT_ENA_R {
        L2_CACHE_PLD_DONE_INT_ENA_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 12 - The bit is used to enable interrupt of L2-Cache preload-operation error."]
    #[inline(always)]
    pub fn l2_cache_pld_err_int_ena(&self) -> L2_CACHE_PLD_ERR_INT_ENA_R {
        L2_CACHE_PLD_ERR_INT_ENA_R::new(((self.bits >> 12) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L2_CACHE_SYNC_PRELOAD_INT_ENA")
            .field(
                "l2_cache_pld_done_int_ena",
                &self.l2_cache_pld_done_int_ena(),
            )
            .field("l2_cache_pld_err_int_ena", &self.l2_cache_pld_err_int_ena())
            .finish()
    }
}
impl W {
    #[doc = "Bit 5 - The bit is used to enable interrupt of L2-Cache preload-operation done."]
    #[inline(always)]
    pub fn l2_cache_pld_done_int_ena(
        &mut self,
    ) -> L2_CACHE_PLD_DONE_INT_ENA_W<L2_CACHE_SYNC_PRELOAD_INT_ENA_SPEC> {
        L2_CACHE_PLD_DONE_INT_ENA_W::new(self, 5)
    }
    #[doc = "Bit 12 - The bit is used to enable interrupt of L2-Cache preload-operation error."]
    #[inline(always)]
    pub fn l2_cache_pld_err_int_ena(
        &mut self,
    ) -> L2_CACHE_PLD_ERR_INT_ENA_W<L2_CACHE_SYNC_PRELOAD_INT_ENA_SPEC> {
        L2_CACHE_PLD_ERR_INT_ENA_W::new(self, 12)
    }
}
#[doc = "L1-Cache Access Fail Interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`l2_cache_sync_preload_int_ena::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2_cache_sync_preload_int_ena::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L2_CACHE_SYNC_PRELOAD_INT_ENA_SPEC;
impl crate::RegisterSpec for L2_CACHE_SYNC_PRELOAD_INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l2_cache_sync_preload_int_ena::R`](R) reader structure"]
impl crate::Readable for L2_CACHE_SYNC_PRELOAD_INT_ENA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`l2_cache_sync_preload_int_ena::W`](W) writer structure"]
impl crate::Writable for L2_CACHE_SYNC_PRELOAD_INT_ENA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets L2_CACHE_SYNC_PRELOAD_INT_ENA to value 0"]
impl crate::Resettable for L2_CACHE_SYNC_PRELOAD_INT_ENA_SPEC {
    const RESET_VALUE: u32 = 0;
}
