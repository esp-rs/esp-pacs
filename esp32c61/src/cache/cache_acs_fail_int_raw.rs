#[doc = "Register `CACHE_ACS_FAIL_INT_RAW` reader"]
pub type R = crate::R<CACHE_ACS_FAIL_INT_RAW_SPEC>;
#[doc = "Register `CACHE_ACS_FAIL_INT_RAW` writer"]
pub type W = crate::W<CACHE_ACS_FAIL_INT_RAW_SPEC>;
#[doc = "Field `ICACHE0_FAIL_INT_RAW` reader - The raw bit of the interrupt of access fail that occurs in L1-ICache0."]
pub type ICACHE0_FAIL_INT_RAW_R = crate::BitReader;
#[doc = "Field `ICACHE0_FAIL_INT_RAW` writer - The raw bit of the interrupt of access fail that occurs in L1-ICache0."]
pub type ICACHE0_FAIL_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICACHE1_FAIL_INT_RAW` reader - The raw bit of the interrupt of access fail that occurs in L1-ICache1."]
pub type ICACHE1_FAIL_INT_RAW_R = crate::BitReader;
#[doc = "Field `ICACHE1_FAIL_INT_RAW` writer - The raw bit of the interrupt of access fail that occurs in L1-ICache1."]
pub type ICACHE1_FAIL_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICACHE2_FAIL_INT_RAW` reader - The raw bit of the interrupt of access fail that occurs in L1-ICache2."]
pub type ICACHE2_FAIL_INT_RAW_R = crate::BitReader;
#[doc = "Field `ICACHE2_FAIL_INT_RAW` writer - The raw bit of the interrupt of access fail that occurs in L1-ICache2."]
pub type ICACHE2_FAIL_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICACHE3_FAIL_INT_RAW` reader - The raw bit of the interrupt of access fail that occurs in L1-ICache3."]
pub type ICACHE3_FAIL_INT_RAW_R = crate::BitReader;
#[doc = "Field `ICACHE3_FAIL_INT_RAW` writer - The raw bit of the interrupt of access fail that occurs in L1-ICache3."]
pub type ICACHE3_FAIL_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CACHE_FAIL_INT_RAW` reader - The raw bit of the interrupt of access fail that occurs in L1-DCache."]
pub type CACHE_FAIL_INT_RAW_R = crate::BitReader;
#[doc = "Field `CACHE_FAIL_INT_RAW` writer - The raw bit of the interrupt of access fail that occurs in L1-DCache."]
pub type CACHE_FAIL_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The raw bit of the interrupt of access fail that occurs in L1-ICache0."]
    #[inline(always)]
    pub fn icache0_fail_int_raw(&self) -> ICACHE0_FAIL_INT_RAW_R {
        ICACHE0_FAIL_INT_RAW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The raw bit of the interrupt of access fail that occurs in L1-ICache1."]
    #[inline(always)]
    pub fn icache1_fail_int_raw(&self) -> ICACHE1_FAIL_INT_RAW_R {
        ICACHE1_FAIL_INT_RAW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The raw bit of the interrupt of access fail that occurs in L1-ICache2."]
    #[inline(always)]
    pub fn icache2_fail_int_raw(&self) -> ICACHE2_FAIL_INT_RAW_R {
        ICACHE2_FAIL_INT_RAW_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The raw bit of the interrupt of access fail that occurs in L1-ICache3."]
    #[inline(always)]
    pub fn icache3_fail_int_raw(&self) -> ICACHE3_FAIL_INT_RAW_R {
        ICACHE3_FAIL_INT_RAW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The raw bit of the interrupt of access fail that occurs in L1-DCache."]
    #[inline(always)]
    pub fn cache_fail_int_raw(&self) -> CACHE_FAIL_INT_RAW_R {
        CACHE_FAIL_INT_RAW_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_ACS_FAIL_INT_RAW")
            .field("icache0_fail_int_raw", &self.icache0_fail_int_raw())
            .field("icache1_fail_int_raw", &self.icache1_fail_int_raw())
            .field("icache2_fail_int_raw", &self.icache2_fail_int_raw())
            .field("icache3_fail_int_raw", &self.icache3_fail_int_raw())
            .field("cache_fail_int_raw", &self.cache_fail_int_raw())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - The raw bit of the interrupt of access fail that occurs in L1-ICache0."]
    #[inline(always)]
    pub fn icache0_fail_int_raw(
        &mut self,
    ) -> ICACHE0_FAIL_INT_RAW_W<'_, CACHE_ACS_FAIL_INT_RAW_SPEC> {
        ICACHE0_FAIL_INT_RAW_W::new(self, 0)
    }
    #[doc = "Bit 1 - The raw bit of the interrupt of access fail that occurs in L1-ICache1."]
    #[inline(always)]
    pub fn icache1_fail_int_raw(
        &mut self,
    ) -> ICACHE1_FAIL_INT_RAW_W<'_, CACHE_ACS_FAIL_INT_RAW_SPEC> {
        ICACHE1_FAIL_INT_RAW_W::new(self, 1)
    }
    #[doc = "Bit 2 - The raw bit of the interrupt of access fail that occurs in L1-ICache2."]
    #[inline(always)]
    pub fn icache2_fail_int_raw(
        &mut self,
    ) -> ICACHE2_FAIL_INT_RAW_W<'_, CACHE_ACS_FAIL_INT_RAW_SPEC> {
        ICACHE2_FAIL_INT_RAW_W::new(self, 2)
    }
    #[doc = "Bit 3 - The raw bit of the interrupt of access fail that occurs in L1-ICache3."]
    #[inline(always)]
    pub fn icache3_fail_int_raw(
        &mut self,
    ) -> ICACHE3_FAIL_INT_RAW_W<'_, CACHE_ACS_FAIL_INT_RAW_SPEC> {
        ICACHE3_FAIL_INT_RAW_W::new(self, 3)
    }
    #[doc = "Bit 4 - The raw bit of the interrupt of access fail that occurs in L1-DCache."]
    #[inline(always)]
    pub fn cache_fail_int_raw(&mut self) -> CACHE_FAIL_INT_RAW_W<'_, CACHE_ACS_FAIL_INT_RAW_SPEC> {
        CACHE_FAIL_INT_RAW_W::new(self, 4)
    }
}
#[doc = "Cache Access Fail Interrupt raw register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_acs_fail_int_raw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_acs_fail_int_raw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CACHE_ACS_FAIL_INT_RAW_SPEC;
impl crate::RegisterSpec for CACHE_ACS_FAIL_INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cache_acs_fail_int_raw::R`](R) reader structure"]
impl crate::Readable for CACHE_ACS_FAIL_INT_RAW_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cache_acs_fail_int_raw::W`](W) writer structure"]
impl crate::Writable for CACHE_ACS_FAIL_INT_RAW_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CACHE_ACS_FAIL_INT_RAW to value 0"]
impl crate::Resettable for CACHE_ACS_FAIL_INT_RAW_SPEC {}
