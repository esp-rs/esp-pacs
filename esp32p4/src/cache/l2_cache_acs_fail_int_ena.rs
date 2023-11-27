#[doc = "Register `L2_CACHE_ACS_FAIL_INT_ENA` reader"]
pub type R = crate::R<L2_CACHE_ACS_FAIL_INT_ENA_SPEC>;
#[doc = "Register `L2_CACHE_ACS_FAIL_INT_ENA` writer"]
pub type W = crate::W<L2_CACHE_ACS_FAIL_INT_ENA_SPEC>;
#[doc = "Field `L2_CACHE_FAIL_INT_ENA` reader - The bit is used to enable interrupt of access fail that occurs in L2-Cache due to l1 cache accesses L2-Cache."]
pub type L2_CACHE_FAIL_INT_ENA_R = crate::BitReader;
#[doc = "Field `L2_CACHE_FAIL_INT_ENA` writer - The bit is used to enable interrupt of access fail that occurs in L2-Cache due to l1 cache accesses L2-Cache."]
pub type L2_CACHE_FAIL_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 5 - The bit is used to enable interrupt of access fail that occurs in L2-Cache due to l1 cache accesses L2-Cache."]
    #[inline(always)]
    pub fn l2_cache_fail_int_ena(&self) -> L2_CACHE_FAIL_INT_ENA_R {
        L2_CACHE_FAIL_INT_ENA_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L2_CACHE_ACS_FAIL_INT_ENA")
            .field(
                "l2_cache_fail_int_ena",
                &format_args!("{}", self.l2_cache_fail_int_ena().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<L2_CACHE_ACS_FAIL_INT_ENA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 5 - The bit is used to enable interrupt of access fail that occurs in L2-Cache due to l1 cache accesses L2-Cache."]
    #[inline(always)]
    #[must_use]
    pub fn l2_cache_fail_int_ena(
        &mut self,
    ) -> L2_CACHE_FAIL_INT_ENA_W<L2_CACHE_ACS_FAIL_INT_ENA_SPEC> {
        L2_CACHE_FAIL_INT_ENA_W::new(self, 5)
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
#[doc = "Cache Access Fail Interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l2_cache_acs_fail_int_ena::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2_cache_acs_fail_int_ena::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L2_CACHE_ACS_FAIL_INT_ENA_SPEC;
impl crate::RegisterSpec for L2_CACHE_ACS_FAIL_INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l2_cache_acs_fail_int_ena::R`](R) reader structure"]
impl crate::Readable for L2_CACHE_ACS_FAIL_INT_ENA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`l2_cache_acs_fail_int_ena::W`](W) writer structure"]
impl crate::Writable for L2_CACHE_ACS_FAIL_INT_ENA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets L2_CACHE_ACS_FAIL_INT_ENA to value 0"]
impl crate::Resettable for L2_CACHE_ACS_FAIL_INT_ENA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
