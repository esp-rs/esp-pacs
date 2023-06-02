#[doc = "Register `L1_CACHE_SYNC_PRELOAD_INT_CLR` reader"]
pub struct R(crate::R<L1_CACHE_SYNC_PRELOAD_INT_CLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<L1_CACHE_SYNC_PRELOAD_INT_CLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<L1_CACHE_SYNC_PRELOAD_INT_CLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<L1_CACHE_SYNC_PRELOAD_INT_CLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `L1_CACHE_SYNC_PRELOAD_INT_CLR` writer"]
pub struct W(crate::W<L1_CACHE_SYNC_PRELOAD_INT_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<L1_CACHE_SYNC_PRELOAD_INT_CLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<L1_CACHE_SYNC_PRELOAD_INT_CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<L1_CACHE_SYNC_PRELOAD_INT_CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `L1_ICACHE0_PLD_DONE_INT_CLR` reader - The bit is used to clear interrupt that occurs only when L1-ICache0 preload-operation is done."]
pub type L1_ICACHE0_PLD_DONE_INT_CLR_R = crate::BitReader;
#[doc = "Field `L1_ICACHE1_PLD_DONE_INT_CLR` reader - The bit is used to clear interrupt that occurs only when L1-ICache1 preload-operation is done."]
pub type L1_ICACHE1_PLD_DONE_INT_CLR_R = crate::BitReader;
#[doc = "Field `L1_ICACHE2_PLD_DONE_INT_CLR` reader - Reserved"]
pub type L1_ICACHE2_PLD_DONE_INT_CLR_R = crate::BitReader;
#[doc = "Field `L1_ICACHE3_PLD_DONE_INT_CLR` reader - Reserved"]
pub type L1_ICACHE3_PLD_DONE_INT_CLR_R = crate::BitReader;
#[doc = "Field `L1_CACHE_PLD_DONE_INT_CLR` writer - The bit is used to clear interrupt that occurs only when L1-Cache preload-operation is done."]
pub type L1_CACHE_PLD_DONE_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, L1_CACHE_SYNC_PRELOAD_INT_CLR_SPEC, O>;
#[doc = "Field `CACHE_SYNC_DONE_INT_CLR` writer - The bit is used to clear interrupt that occurs only when Cache sync-operation is done."]
pub type CACHE_SYNC_DONE_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, L1_CACHE_SYNC_PRELOAD_INT_CLR_SPEC, O>;
#[doc = "Field `L1_ICACHE0_PLD_ERR_INT_CLR` reader - The bit is used to clear interrupt of L1-ICache0 preload-operation error."]
pub type L1_ICACHE0_PLD_ERR_INT_CLR_R = crate::BitReader;
#[doc = "Field `L1_ICACHE1_PLD_ERR_INT_CLR` reader - The bit is used to clear interrupt of L1-ICache1 preload-operation error."]
pub type L1_ICACHE1_PLD_ERR_INT_CLR_R = crate::BitReader;
#[doc = "Field `L1_ICACHE2_PLD_ERR_INT_CLR` reader - Reserved"]
pub type L1_ICACHE2_PLD_ERR_INT_CLR_R = crate::BitReader;
#[doc = "Field `L1_ICACHE3_PLD_ERR_INT_CLR` reader - Reserved"]
pub type L1_ICACHE3_PLD_ERR_INT_CLR_R = crate::BitReader;
#[doc = "Field `L1_CACHE_PLD_ERR_INT_CLR` writer - The bit is used to clear interrupt of L1-Cache preload-operation error."]
pub type L1_CACHE_PLD_ERR_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, L1_CACHE_SYNC_PRELOAD_INT_CLR_SPEC, O>;
#[doc = "Field `CACHE_SYNC_ERR_INT_CLR` writer - The bit is used to clear interrupt of Cache sync-operation error."]
pub type CACHE_SYNC_ERR_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, L1_CACHE_SYNC_PRELOAD_INT_CLR_SPEC, O>;
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
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 4 - The bit is used to clear interrupt that occurs only when L1-Cache preload-operation is done."]
    #[inline(always)]
    #[must_use]
    pub fn l1_cache_pld_done_int_clr(&mut self) -> L1_CACHE_PLD_DONE_INT_CLR_W<4> {
        L1_CACHE_PLD_DONE_INT_CLR_W::new(self)
    }
    #[doc = "Bit 6 - The bit is used to clear interrupt that occurs only when Cache sync-operation is done."]
    #[inline(always)]
    #[must_use]
    pub fn cache_sync_done_int_clr(&mut self) -> CACHE_SYNC_DONE_INT_CLR_W<6> {
        CACHE_SYNC_DONE_INT_CLR_W::new(self)
    }
    #[doc = "Bit 11 - The bit is used to clear interrupt of L1-Cache preload-operation error."]
    #[inline(always)]
    #[must_use]
    pub fn l1_cache_pld_err_int_clr(&mut self) -> L1_CACHE_PLD_ERR_INT_CLR_W<11> {
        L1_CACHE_PLD_ERR_INT_CLR_W::new(self)
    }
    #[doc = "Bit 13 - The bit is used to clear interrupt of Cache sync-operation error."]
    #[inline(always)]
    #[must_use]
    pub fn cache_sync_err_int_clr(&mut self) -> CACHE_SYNC_ERR_INT_CLR_W<13> {
        CACHE_SYNC_ERR_INT_CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sync Preload operation Interrupt clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l1_cache_sync_preload_int_clr](index.html) module"]
pub struct L1_CACHE_SYNC_PRELOAD_INT_CLR_SPEC;
impl crate::RegisterSpec for L1_CACHE_SYNC_PRELOAD_INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [l1_cache_sync_preload_int_clr::R](R) reader structure"]
impl crate::Readable for L1_CACHE_SYNC_PRELOAD_INT_CLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [l1_cache_sync_preload_int_clr::W](W) writer structure"]
impl crate::Writable for L1_CACHE_SYNC_PRELOAD_INT_CLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets L1_CACHE_SYNC_PRELOAD_INT_CLR to value 0"]
impl crate::Resettable for L1_CACHE_SYNC_PRELOAD_INT_CLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
