///Register `L2_CACHE_ACS_FAIL_CTRL` reader
pub type R = crate::R<L2_CACHE_ACS_FAIL_CTRL_SPEC>;
///Register `L2_CACHE_ACS_FAIL_CTRL` writer
pub type W = crate::W<L2_CACHE_ACS_FAIL_CTRL_SPEC>;
///Field `L2_CACHE_ACS_FAIL_CHECK_MODE` reader - The bit is used to configure l2 cache access fail check mode. 0: the access fail is not propagated to the request, 1: the access fail is propagated to the request
pub type L2_CACHE_ACS_FAIL_CHECK_MODE_R = crate::BitReader;
///Field `L2_CACHE_ACS_FAIL_CHECK_MODE` writer - The bit is used to configure l2 cache access fail check mode. 0: the access fail is not propagated to the request, 1: the access fail is propagated to the request
pub type L2_CACHE_ACS_FAIL_CHECK_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - The bit is used to configure l2 cache access fail check mode. 0: the access fail is not propagated to the request, 1: the access fail is propagated to the request
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
    ///Bit 0 - The bit is used to configure l2 cache access fail check mode. 0: the access fail is not propagated to the request, 1: the access fail is propagated to the request
    #[inline(always)]
    #[must_use]
    pub fn l2_cache_acs_fail_check_mode(
        &mut self,
    ) -> L2_CACHE_ACS_FAIL_CHECK_MODE_W<L2_CACHE_ACS_FAIL_CTRL_SPEC> {
        L2_CACHE_ACS_FAIL_CHECK_MODE_W::new(self, 0)
    }
}
/**Cache Access Fail Configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`l2_cache_acs_fail_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2_cache_acs_fail_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct L2_CACHE_ACS_FAIL_CTRL_SPEC;
impl crate::RegisterSpec for L2_CACHE_ACS_FAIL_CTRL_SPEC {
    type Ux = u32;
}
///`read()` method returns [`l2_cache_acs_fail_ctrl::R`](R) reader structure
impl crate::Readable for L2_CACHE_ACS_FAIL_CTRL_SPEC {}
///`write(|w| ..)` method takes [`l2_cache_acs_fail_ctrl::W`](W) writer structure
impl crate::Writable for L2_CACHE_ACS_FAIL_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets L2_CACHE_ACS_FAIL_CTRL to value 0
impl crate::Resettable for L2_CACHE_ACS_FAIL_CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
