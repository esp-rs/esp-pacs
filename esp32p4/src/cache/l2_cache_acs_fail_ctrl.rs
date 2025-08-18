#[doc = "Register `L2_CACHE_ACS_FAIL_CTRL` reader"]
pub type R = crate::R<L2_CACHE_ACS_FAIL_CTRL_SPEC>;
#[doc = "Register `L2_CACHE_ACS_FAIL_CTRL` writer"]
pub type W = crate::W<L2_CACHE_ACS_FAIL_CTRL_SPEC>;
#[doc = "Field `L2_CACHE_ACS_FAIL_CHECK_MODE` reader - The bit is used to configure l2 cache access fail check mode. 0: the access fail is not propagated to the request, 1: the access fail is propagated to the request"]
pub type L2_CACHE_ACS_FAIL_CHECK_MODE_R = crate::BitReader;
#[doc = "Field `L2_CACHE_ACS_FAIL_CHECK_MODE` writer - The bit is used to configure l2 cache access fail check mode. 0: the access fail is not propagated to the request, 1: the access fail is propagated to the request"]
pub type L2_CACHE_ACS_FAIL_CHECK_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The bit is used to configure l2 cache access fail check mode. 0: the access fail is not propagated to the request, 1: the access fail is propagated to the request"]
    #[inline(always)]
    pub fn l2_cache_acs_fail_check_mode(&self) -> L2_CACHE_ACS_FAIL_CHECK_MODE_R {
        L2_CACHE_ACS_FAIL_CHECK_MODE_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L2_CACHE_ACS_FAIL_CTRL")
            .field(
                "l2_cache_acs_fail_check_mode",
                &self.l2_cache_acs_fail_check_mode(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - The bit is used to configure l2 cache access fail check mode. 0: the access fail is not propagated to the request, 1: the access fail is propagated to the request"]
    #[inline(always)]
    pub fn l2_cache_acs_fail_check_mode(
        &mut self,
    ) -> L2_CACHE_ACS_FAIL_CHECK_MODE_W<'_, L2_CACHE_ACS_FAIL_CTRL_SPEC> {
        L2_CACHE_ACS_FAIL_CHECK_MODE_W::new(self, 0)
    }
}
#[doc = "Cache Access Fail Configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`l2_cache_acs_fail_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2_cache_acs_fail_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L2_CACHE_ACS_FAIL_CTRL_SPEC;
impl crate::RegisterSpec for L2_CACHE_ACS_FAIL_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l2_cache_acs_fail_ctrl::R`](R) reader structure"]
impl crate::Readable for L2_CACHE_ACS_FAIL_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`l2_cache_acs_fail_ctrl::W`](W) writer structure"]
impl crate::Writable for L2_CACHE_ACS_FAIL_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets L2_CACHE_ACS_FAIL_CTRL to value 0"]
impl crate::Resettable for L2_CACHE_ACS_FAIL_CTRL_SPEC {}
