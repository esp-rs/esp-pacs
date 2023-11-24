#[doc = "Register `L1_CACHE_ACS_FAIL_INT_CLR` reader"]
pub type R = crate::R<L1_CACHE_ACS_FAIL_INT_CLR_SPEC>;
#[doc = "Register `L1_CACHE_ACS_FAIL_INT_CLR` writer"]
pub type W = crate::W<L1_CACHE_ACS_FAIL_INT_CLR_SPEC>;
#[doc = "Field `L1_ICACHE0_FAIL_INT_CLR` reader - The bit is used to clear interrupt of access fail that occurs in L1-ICache0 due to cpu accesses L1-ICache0."]
pub type L1_ICACHE0_FAIL_INT_CLR_R = crate::BitReader;
#[doc = "Field `L1_ICACHE1_FAIL_INT_CLR` reader - The bit is used to clear interrupt of access fail that occurs in L1-ICache1 due to cpu accesses L1-ICache1."]
pub type L1_ICACHE1_FAIL_INT_CLR_R = crate::BitReader;
#[doc = "Field `L1_ICACHE2_FAIL_INT_CLR` reader - Reserved"]
pub type L1_ICACHE2_FAIL_INT_CLR_R = crate::BitReader;
#[doc = "Field `L1_ICACHE3_FAIL_INT_CLR` reader - Reserved"]
pub type L1_ICACHE3_FAIL_INT_CLR_R = crate::BitReader;
#[doc = "Field `L1_CACHE_FAIL_INT_CLR` writer - The bit is used to clear interrupt of access fail that occurs in L1-DCache due to cpu accesses L1-DCache."]
pub type L1_CACHE_FAIL_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The bit is used to clear interrupt of access fail that occurs in L1-ICache0 due to cpu accesses L1-ICache0."]
    #[inline(always)]
    pub fn l1_icache0_fail_int_clr(&self) -> L1_ICACHE0_FAIL_INT_CLR_R {
        L1_ICACHE0_FAIL_INT_CLR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The bit is used to clear interrupt of access fail that occurs in L1-ICache1 due to cpu accesses L1-ICache1."]
    #[inline(always)]
    pub fn l1_icache1_fail_int_clr(&self) -> L1_ICACHE1_FAIL_INT_CLR_R {
        L1_ICACHE1_FAIL_INT_CLR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn l1_icache2_fail_int_clr(&self) -> L1_ICACHE2_FAIL_INT_CLR_R {
        L1_ICACHE2_FAIL_INT_CLR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn l1_icache3_fail_int_clr(&self) -> L1_ICACHE3_FAIL_INT_CLR_R {
        L1_ICACHE3_FAIL_INT_CLR_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L1_CACHE_ACS_FAIL_INT_CLR")
            .field(
                "l1_icache0_fail_int_clr",
                &format_args!("{}", self.l1_icache0_fail_int_clr().bit()),
            )
            .field(
                "l1_icache1_fail_int_clr",
                &format_args!("{}", self.l1_icache1_fail_int_clr().bit()),
            )
            .field(
                "l1_icache2_fail_int_clr",
                &format_args!("{}", self.l1_icache2_fail_int_clr().bit()),
            )
            .field(
                "l1_icache3_fail_int_clr",
                &format_args!("{}", self.l1_icache3_fail_int_clr().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<L1_CACHE_ACS_FAIL_INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 4 - The bit is used to clear interrupt of access fail that occurs in L1-DCache due to cpu accesses L1-DCache."]
    #[inline(always)]
    #[must_use]
    pub fn l1_cache_fail_int_clr(
        &mut self,
    ) -> L1_CACHE_FAIL_INT_CLR_W<L1_CACHE_ACS_FAIL_INT_CLR_SPEC> {
        L1_CACHE_FAIL_INT_CLR_W::new(self, 4)
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
#[doc = "L1-Cache Access Fail Interrupt clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l1_cache_acs_fail_int_clr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1_cache_acs_fail_int_clr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1_CACHE_ACS_FAIL_INT_CLR_SPEC;
impl crate::RegisterSpec for L1_CACHE_ACS_FAIL_INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l1_cache_acs_fail_int_clr::R`](R) reader structure"]
impl crate::Readable for L1_CACHE_ACS_FAIL_INT_CLR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`l1_cache_acs_fail_int_clr::W`](W) writer structure"]
impl crate::Writable for L1_CACHE_ACS_FAIL_INT_CLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets L1_CACHE_ACS_FAIL_INT_CLR to value 0"]
impl crate::Resettable for L1_CACHE_ACS_FAIL_INT_CLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
