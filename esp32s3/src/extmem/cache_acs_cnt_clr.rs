#[doc = "Register `CACHE_ACS_CNT_CLR` writer"]
pub type W = crate::W<CACHE_ACS_CNT_CLR_SPEC>;
#[doc = "Field `DCACHE_ACS_CNT_CLR` writer - The bit is used to clear dcache counter."]
pub type DCACHE_ACS_CNT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ICACHE_ACS_CNT_CLR` writer - The bit is used to clear icache counter."]
pub type ICACHE_ACS_CNT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CACHE_ACS_CNT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - The bit is used to clear dcache counter."]
    #[inline(always)]
    #[must_use]
    pub fn dcache_acs_cnt_clr(&mut self) -> DCACHE_ACS_CNT_CLR_W<CACHE_ACS_CNT_CLR_SPEC, 0> {
        DCACHE_ACS_CNT_CLR_W::new(self)
    }
    #[doc = "Bit 1 - The bit is used to clear icache counter."]
    #[inline(always)]
    #[must_use]
    pub fn icache_acs_cnt_clr(&mut self) -> ICACHE_ACS_CNT_CLR_W<CACHE_ACS_CNT_CLR_SPEC, 1> {
        ICACHE_ACS_CNT_CLR_W::new(self)
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
#[doc = "******* Description ***********\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cache_acs_cnt_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CACHE_ACS_CNT_CLR_SPEC;
impl crate::RegisterSpec for CACHE_ACS_CNT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cache_acs_cnt_clr::W`](W) writer structure"]
impl crate::Writable for CACHE_ACS_CNT_CLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CACHE_ACS_CNT_CLR to value 0"]
impl crate::Resettable for CACHE_ACS_CNT_CLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
