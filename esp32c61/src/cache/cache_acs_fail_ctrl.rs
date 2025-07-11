#[doc = "Register `CACHE_ACS_FAIL_CTRL` reader"]
pub type R = crate::R<CACHE_ACS_FAIL_CTRL_SPEC>;
#[doc = "Register `CACHE_ACS_FAIL_CTRL` writer"]
pub type W = crate::W<CACHE_ACS_FAIL_CTRL_SPEC>;
#[doc = "Field `ICACHE0_ACS_FAIL_CHECK_MODE` reader - The bit is used to configure l1 icache0 access fail check mode. 0: the access fail is not propagated to the request, 1: the access fail is propagated to the request"]
pub type ICACHE0_ACS_FAIL_CHECK_MODE_R = crate::BitReader;
#[doc = "Field `ICACHE1_ACS_FAIL_CHECK_MODE` reader - The bit is used to configure l1 icache1 access fail check mode. 0: the access fail is not propagated to the request, 1: the access fail is propagated to the request"]
pub type ICACHE1_ACS_FAIL_CHECK_MODE_R = crate::BitReader;
#[doc = "Field `ICACHE2_ACS_FAIL_CHECK_MODE` reader - The bit is used to configure l1 icache2 access fail check mode. 0: the access fail is not propagated to the request, 1: the access fail is propagated to the request"]
pub type ICACHE2_ACS_FAIL_CHECK_MODE_R = crate::BitReader;
#[doc = "Field `ICACHE3_ACS_FAIL_CHECK_MODE` reader - The bit is used to configure l1 icache3 access fail check mode. 0: the access fail is not propagated to the request, 1: the access fail is propagated to the request"]
pub type ICACHE3_ACS_FAIL_CHECK_MODE_R = crate::BitReader;
#[doc = "Field `CACHE_ACS_FAIL_CHECK_MODE` reader - The bit is used to configure l1 cache access fail check mode. 0: the access fail is not propagated to the request, 1: the access fail is propagated to the request"]
pub type CACHE_ACS_FAIL_CHECK_MODE_R = crate::BitReader;
#[doc = "Field `CACHE_ACS_FAIL_CHECK_MODE` writer - The bit is used to configure l1 cache access fail check mode. 0: the access fail is not propagated to the request, 1: the access fail is propagated to the request"]
pub type CACHE_ACS_FAIL_CHECK_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The bit is used to configure l1 icache0 access fail check mode. 0: the access fail is not propagated to the request, 1: the access fail is propagated to the request"]
    #[inline(always)]
    pub fn icache0_acs_fail_check_mode(&self) -> ICACHE0_ACS_FAIL_CHECK_MODE_R {
        ICACHE0_ACS_FAIL_CHECK_MODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The bit is used to configure l1 icache1 access fail check mode. 0: the access fail is not propagated to the request, 1: the access fail is propagated to the request"]
    #[inline(always)]
    pub fn icache1_acs_fail_check_mode(&self) -> ICACHE1_ACS_FAIL_CHECK_MODE_R {
        ICACHE1_ACS_FAIL_CHECK_MODE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The bit is used to configure l1 icache2 access fail check mode. 0: the access fail is not propagated to the request, 1: the access fail is propagated to the request"]
    #[inline(always)]
    pub fn icache2_acs_fail_check_mode(&self) -> ICACHE2_ACS_FAIL_CHECK_MODE_R {
        ICACHE2_ACS_FAIL_CHECK_MODE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The bit is used to configure l1 icache3 access fail check mode. 0: the access fail is not propagated to the request, 1: the access fail is propagated to the request"]
    #[inline(always)]
    pub fn icache3_acs_fail_check_mode(&self) -> ICACHE3_ACS_FAIL_CHECK_MODE_R {
        ICACHE3_ACS_FAIL_CHECK_MODE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The bit is used to configure l1 cache access fail check mode. 0: the access fail is not propagated to the request, 1: the access fail is propagated to the request"]
    #[inline(always)]
    pub fn cache_acs_fail_check_mode(&self) -> CACHE_ACS_FAIL_CHECK_MODE_R {
        CACHE_ACS_FAIL_CHECK_MODE_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_ACS_FAIL_CTRL")
            .field(
                "icache0_acs_fail_check_mode",
                &self.icache0_acs_fail_check_mode(),
            )
            .field(
                "icache1_acs_fail_check_mode",
                &self.icache1_acs_fail_check_mode(),
            )
            .field(
                "icache2_acs_fail_check_mode",
                &self.icache2_acs_fail_check_mode(),
            )
            .field(
                "icache3_acs_fail_check_mode",
                &self.icache3_acs_fail_check_mode(),
            )
            .field(
                "cache_acs_fail_check_mode",
                &self.cache_acs_fail_check_mode(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 4 - The bit is used to configure l1 cache access fail check mode. 0: the access fail is not propagated to the request, 1: the access fail is propagated to the request"]
    #[inline(always)]
    pub fn cache_acs_fail_check_mode(
        &mut self,
    ) -> CACHE_ACS_FAIL_CHECK_MODE_W<CACHE_ACS_FAIL_CTRL_SPEC> {
        CACHE_ACS_FAIL_CHECK_MODE_W::new(self, 4)
    }
}
#[doc = "Cache Access Fail Configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_acs_fail_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_acs_fail_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CACHE_ACS_FAIL_CTRL_SPEC;
impl crate::RegisterSpec for CACHE_ACS_FAIL_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cache_acs_fail_ctrl::R`](R) reader structure"]
impl crate::Readable for CACHE_ACS_FAIL_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cache_acs_fail_ctrl::W`](W) writer structure"]
impl crate::Writable for CACHE_ACS_FAIL_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CACHE_ACS_FAIL_CTRL to value 0"]
impl crate::Resettable for CACHE_ACS_FAIL_CTRL_SPEC {}
