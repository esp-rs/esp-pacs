#[doc = "Register `CACHE_ACS_FAIL_INT_ENA` reader"]
pub type R = crate::R<CACHE_ACS_FAIL_INT_ENA_SPEC>;
#[doc = "Register `CACHE_ACS_FAIL_INT_ENA` writer"]
pub type W = crate::W<CACHE_ACS_FAIL_INT_ENA_SPEC>;
#[doc = "Field `ICACHE0_FAIL_INT_ENA` reader - The bit is used to enable interrupt of access fail that occurs in L1-ICache0 due to cpu accesses L1-ICache0."]
pub type ICACHE0_FAIL_INT_ENA_R = crate::BitReader;
#[doc = "Field `ICACHE1_FAIL_INT_ENA` reader - The bit is used to enable interrupt of access fail that occurs in L1-ICache1 due to cpu accesses L1-ICache1."]
pub type ICACHE1_FAIL_INT_ENA_R = crate::BitReader;
#[doc = "Field `ICACHE2_FAIL_INT_ENA` reader - Reserved"]
pub type ICACHE2_FAIL_INT_ENA_R = crate::BitReader;
#[doc = "Field `ICACHE3_FAIL_INT_ENA` reader - Reserved"]
pub type ICACHE3_FAIL_INT_ENA_R = crate::BitReader;
#[doc = "Field `CACHE_FAIL_INT_ENA` reader - The bit is used to enable interrupt of access fail that occurs in L1-DCache due to cpu accesses L1-DCache."]
pub type CACHE_FAIL_INT_ENA_R = crate::BitReader;
#[doc = "Field `CACHE_FAIL_INT_ENA` writer - The bit is used to enable interrupt of access fail that occurs in L1-DCache due to cpu accesses L1-DCache."]
pub type CACHE_FAIL_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The bit is used to enable interrupt of access fail that occurs in L1-ICache0 due to cpu accesses L1-ICache0."]
    #[inline(always)]
    pub fn icache0_fail_int_ena(&self) -> ICACHE0_FAIL_INT_ENA_R {
        ICACHE0_FAIL_INT_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The bit is used to enable interrupt of access fail that occurs in L1-ICache1 due to cpu accesses L1-ICache1."]
    #[inline(always)]
    pub fn icache1_fail_int_ena(&self) -> ICACHE1_FAIL_INT_ENA_R {
        ICACHE1_FAIL_INT_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn icache2_fail_int_ena(&self) -> ICACHE2_FAIL_INT_ENA_R {
        ICACHE2_FAIL_INT_ENA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn icache3_fail_int_ena(&self) -> ICACHE3_FAIL_INT_ENA_R {
        ICACHE3_FAIL_INT_ENA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The bit is used to enable interrupt of access fail that occurs in L1-DCache due to cpu accesses L1-DCache."]
    #[inline(always)]
    pub fn cache_fail_int_ena(&self) -> CACHE_FAIL_INT_ENA_R {
        CACHE_FAIL_INT_ENA_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_ACS_FAIL_INT_ENA")
            .field("icache0_fail_int_ena", &self.icache0_fail_int_ena())
            .field("icache1_fail_int_ena", &self.icache1_fail_int_ena())
            .field("icache2_fail_int_ena", &self.icache2_fail_int_ena())
            .field("icache3_fail_int_ena", &self.icache3_fail_int_ena())
            .field("cache_fail_int_ena", &self.cache_fail_int_ena())
            .finish()
    }
}
impl W {
    #[doc = "Bit 4 - The bit is used to enable interrupt of access fail that occurs in L1-DCache due to cpu accesses L1-DCache."]
    #[inline(always)]
    pub fn cache_fail_int_ena(&mut self) -> CACHE_FAIL_INT_ENA_W<'_, CACHE_ACS_FAIL_INT_ENA_SPEC> {
        CACHE_FAIL_INT_ENA_W::new(self, 4)
    }
}
#[doc = "Cache Access Fail Interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_acs_fail_int_ena::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_acs_fail_int_ena::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CACHE_ACS_FAIL_INT_ENA_SPEC;
impl crate::RegisterSpec for CACHE_ACS_FAIL_INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cache_acs_fail_int_ena::R`](R) reader structure"]
impl crate::Readable for CACHE_ACS_FAIL_INT_ENA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cache_acs_fail_int_ena::W`](W) writer structure"]
impl crate::Writable for CACHE_ACS_FAIL_INT_ENA_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CACHE_ACS_FAIL_INT_ENA to value 0"]
impl crate::Resettable for CACHE_ACS_FAIL_INT_ENA_SPEC {}
