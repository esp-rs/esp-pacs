#[doc = "Register `CACHE_L2_CACHE_ACS_FAIL_INT_RAW` reader"]
pub type R = crate::R<CACHE_L2_CACHE_ACS_FAIL_INT_RAW_SPEC>;
#[doc = "Register `CACHE_L2_CACHE_ACS_FAIL_INT_RAW` writer"]
pub type W = crate::W<CACHE_L2_CACHE_ACS_FAIL_INT_RAW_SPEC>;
#[doc = "Field `CACHE_L2_CACHE_FAIL_INT_RAW` reader - The raw bit of the interrupt of access fail that occurs in L2-Cache."]
pub type CACHE_L2_CACHE_FAIL_INT_RAW_R = crate::BitReader;
#[doc = "Field `CACHE_L2_CACHE_FAIL_INT_RAW` writer - The raw bit of the interrupt of access fail that occurs in L2-Cache."]
pub type CACHE_L2_CACHE_FAIL_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 5 - The raw bit of the interrupt of access fail that occurs in L2-Cache."]
    #[inline(always)]
    pub fn cache_l2_cache_fail_int_raw(&self) -> CACHE_L2_CACHE_FAIL_INT_RAW_R {
        CACHE_L2_CACHE_FAIL_INT_RAW_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_L2_CACHE_ACS_FAIL_INT_RAW")
            .field(
                "cache_l2_cache_fail_int_raw",
                &self.cache_l2_cache_fail_int_raw(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 5 - The raw bit of the interrupt of access fail that occurs in L2-Cache."]
    #[inline(always)]
    pub fn cache_l2_cache_fail_int_raw(
        &mut self,
    ) -> CACHE_L2_CACHE_FAIL_INT_RAW_W<CACHE_L2_CACHE_ACS_FAIL_INT_RAW_SPEC> {
        CACHE_L2_CACHE_FAIL_INT_RAW_W::new(self, 5)
    }
}
#[doc = "Cache Access Fail Interrupt raw register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_l2_cache_acs_fail_int_raw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_l2_cache_acs_fail_int_raw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CACHE_L2_CACHE_ACS_FAIL_INT_RAW_SPEC;
impl crate::RegisterSpec for CACHE_L2_CACHE_ACS_FAIL_INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cache_l2_cache_acs_fail_int_raw::R`](R) reader structure"]
impl crate::Readable for CACHE_L2_CACHE_ACS_FAIL_INT_RAW_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cache_l2_cache_acs_fail_int_raw::W`](W) writer structure"]
impl crate::Writable for CACHE_L2_CACHE_ACS_FAIL_INT_RAW_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CACHE_L2_CACHE_ACS_FAIL_INT_RAW to value 0"]
impl crate::Resettable for CACHE_L2_CACHE_ACS_FAIL_INT_RAW_SPEC {}
