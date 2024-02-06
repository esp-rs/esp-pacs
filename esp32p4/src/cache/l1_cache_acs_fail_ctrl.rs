#[doc = "Register `L1_CACHE_ACS_FAIL_CTRL` reader"]
pub type R = crate::R<L1_CACHE_ACS_FAIL_CTRL_SPEC>;
#[doc = "Register `L1_CACHE_ACS_FAIL_CTRL` writer"]
pub type W = crate::W<L1_CACHE_ACS_FAIL_CTRL_SPEC>;
#[doc = "Field `L1_ICACHE0_ACS_FAIL_CHECK_MODE` reader - The bit is used to configure l1 icache0 access fail check mode. 0: the access fail is not propagated to the request, 1: the access fail is propagated to the request"]
pub type L1_ICACHE0_ACS_FAIL_CHECK_MODE_R = crate::BitReader;
#[doc = "Field `L1_ICACHE0_ACS_FAIL_CHECK_MODE` writer - The bit is used to configure l1 icache0 access fail check mode. 0: the access fail is not propagated to the request, 1: the access fail is propagated to the request"]
pub type L1_ICACHE0_ACS_FAIL_CHECK_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1_ICACHE1_ACS_FAIL_CHECK_MODE` reader - The bit is used to configure l1 icache1 access fail check mode. 0: the access fail is not propagated to the request, 1: the access fail is propagated to the request"]
pub type L1_ICACHE1_ACS_FAIL_CHECK_MODE_R = crate::BitReader;
#[doc = "Field `L1_ICACHE1_ACS_FAIL_CHECK_MODE` writer - The bit is used to configure l1 icache1 access fail check mode. 0: the access fail is not propagated to the request, 1: the access fail is propagated to the request"]
pub type L1_ICACHE1_ACS_FAIL_CHECK_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1_ICACHE2_ACS_FAIL_CHECK_MODE` reader - The bit is used to configure l1 icache2 access fail check mode. 0: the access fail is not propagated to the request, 1: the access fail is propagated to the request"]
pub type L1_ICACHE2_ACS_FAIL_CHECK_MODE_R = crate::BitReader;
#[doc = "Field `L1_ICACHE2_ACS_FAIL_CHECK_MODE` writer - The bit is used to configure l1 icache2 access fail check mode. 0: the access fail is not propagated to the request, 1: the access fail is propagated to the request"]
pub type L1_ICACHE2_ACS_FAIL_CHECK_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1_ICACHE3_ACS_FAIL_CHECK_MODE` reader - The bit is used to configure l1 icache3 access fail check mode. 0: the access fail is not propagated to the request, 1: the access fail is propagated to the request"]
pub type L1_ICACHE3_ACS_FAIL_CHECK_MODE_R = crate::BitReader;
#[doc = "Field `L1_ICACHE3_ACS_FAIL_CHECK_MODE` writer - The bit is used to configure l1 icache3 access fail check mode. 0: the access fail is not propagated to the request, 1: the access fail is propagated to the request"]
pub type L1_ICACHE3_ACS_FAIL_CHECK_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1_DCACHE_ACS_FAIL_CHECK_MODE` reader - The bit is used to configure l1 dcache access fail check mode. 0: the access fail is not propagated to the request, 1: the access fail is propagated to the request"]
pub type L1_DCACHE_ACS_FAIL_CHECK_MODE_R = crate::BitReader;
#[doc = "Field `L1_DCACHE_ACS_FAIL_CHECK_MODE` writer - The bit is used to configure l1 dcache access fail check mode. 0: the access fail is not propagated to the request, 1: the access fail is propagated to the request"]
pub type L1_DCACHE_ACS_FAIL_CHECK_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The bit is used to configure l1 icache0 access fail check mode. 0: the access fail is not propagated to the request, 1: the access fail is propagated to the request"]
    #[inline(always)]
    pub fn l1_icache0_acs_fail_check_mode(&self) -> L1_ICACHE0_ACS_FAIL_CHECK_MODE_R {
        L1_ICACHE0_ACS_FAIL_CHECK_MODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The bit is used to configure l1 icache1 access fail check mode. 0: the access fail is not propagated to the request, 1: the access fail is propagated to the request"]
    #[inline(always)]
    pub fn l1_icache1_acs_fail_check_mode(&self) -> L1_ICACHE1_ACS_FAIL_CHECK_MODE_R {
        L1_ICACHE1_ACS_FAIL_CHECK_MODE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The bit is used to configure l1 icache2 access fail check mode. 0: the access fail is not propagated to the request, 1: the access fail is propagated to the request"]
    #[inline(always)]
    pub fn l1_icache2_acs_fail_check_mode(&self) -> L1_ICACHE2_ACS_FAIL_CHECK_MODE_R {
        L1_ICACHE2_ACS_FAIL_CHECK_MODE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The bit is used to configure l1 icache3 access fail check mode. 0: the access fail is not propagated to the request, 1: the access fail is propagated to the request"]
    #[inline(always)]
    pub fn l1_icache3_acs_fail_check_mode(&self) -> L1_ICACHE3_ACS_FAIL_CHECK_MODE_R {
        L1_ICACHE3_ACS_FAIL_CHECK_MODE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The bit is used to configure l1 dcache access fail check mode. 0: the access fail is not propagated to the request, 1: the access fail is propagated to the request"]
    #[inline(always)]
    pub fn l1_dcache_acs_fail_check_mode(&self) -> L1_DCACHE_ACS_FAIL_CHECK_MODE_R {
        L1_DCACHE_ACS_FAIL_CHECK_MODE_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L1_CACHE_ACS_FAIL_CTRL")
            .field(
                "l1_icache0_acs_fail_check_mode",
                &format_args!("{}", self.l1_icache0_acs_fail_check_mode().bit()),
            )
            .field(
                "l1_icache1_acs_fail_check_mode",
                &format_args!("{}", self.l1_icache1_acs_fail_check_mode().bit()),
            )
            .field(
                "l1_icache2_acs_fail_check_mode",
                &format_args!("{}", self.l1_icache2_acs_fail_check_mode().bit()),
            )
            .field(
                "l1_icache3_acs_fail_check_mode",
                &format_args!("{}", self.l1_icache3_acs_fail_check_mode().bit()),
            )
            .field(
                "l1_dcache_acs_fail_check_mode",
                &format_args!("{}", self.l1_dcache_acs_fail_check_mode().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<L1_CACHE_ACS_FAIL_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - The bit is used to configure l1 icache0 access fail check mode. 0: the access fail is not propagated to the request, 1: the access fail is propagated to the request"]
    #[inline(always)]
    #[must_use]
    pub fn l1_icache0_acs_fail_check_mode(
        &mut self,
    ) -> L1_ICACHE0_ACS_FAIL_CHECK_MODE_W<L1_CACHE_ACS_FAIL_CTRL_SPEC> {
        L1_ICACHE0_ACS_FAIL_CHECK_MODE_W::new(self, 0)
    }
    #[doc = "Bit 1 - The bit is used to configure l1 icache1 access fail check mode. 0: the access fail is not propagated to the request, 1: the access fail is propagated to the request"]
    #[inline(always)]
    #[must_use]
    pub fn l1_icache1_acs_fail_check_mode(
        &mut self,
    ) -> L1_ICACHE1_ACS_FAIL_CHECK_MODE_W<L1_CACHE_ACS_FAIL_CTRL_SPEC> {
        L1_ICACHE1_ACS_FAIL_CHECK_MODE_W::new(self, 1)
    }
    #[doc = "Bit 2 - The bit is used to configure l1 icache2 access fail check mode. 0: the access fail is not propagated to the request, 1: the access fail is propagated to the request"]
    #[inline(always)]
    #[must_use]
    pub fn l1_icache2_acs_fail_check_mode(
        &mut self,
    ) -> L1_ICACHE2_ACS_FAIL_CHECK_MODE_W<L1_CACHE_ACS_FAIL_CTRL_SPEC> {
        L1_ICACHE2_ACS_FAIL_CHECK_MODE_W::new(self, 2)
    }
    #[doc = "Bit 3 - The bit is used to configure l1 icache3 access fail check mode. 0: the access fail is not propagated to the request, 1: the access fail is propagated to the request"]
    #[inline(always)]
    #[must_use]
    pub fn l1_icache3_acs_fail_check_mode(
        &mut self,
    ) -> L1_ICACHE3_ACS_FAIL_CHECK_MODE_W<L1_CACHE_ACS_FAIL_CTRL_SPEC> {
        L1_ICACHE3_ACS_FAIL_CHECK_MODE_W::new(self, 3)
    }
    #[doc = "Bit 4 - The bit is used to configure l1 dcache access fail check mode. 0: the access fail is not propagated to the request, 1: the access fail is propagated to the request"]
    #[inline(always)]
    #[must_use]
    pub fn l1_dcache_acs_fail_check_mode(
        &mut self,
    ) -> L1_DCACHE_ACS_FAIL_CHECK_MODE_W<L1_CACHE_ACS_FAIL_CTRL_SPEC> {
        L1_DCACHE_ACS_FAIL_CHECK_MODE_W::new(self, 4)
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
#[doc = "Cache Access Fail Configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l1_cache_acs_fail_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1_cache_acs_fail_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1_CACHE_ACS_FAIL_CTRL_SPEC;
impl crate::RegisterSpec for L1_CACHE_ACS_FAIL_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l1_cache_acs_fail_ctrl::R`](R) reader structure"]
impl crate::Readable for L1_CACHE_ACS_FAIL_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`l1_cache_acs_fail_ctrl::W`](W) writer structure"]
impl crate::Writable for L1_CACHE_ACS_FAIL_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets L1_CACHE_ACS_FAIL_CTRL to value 0"]
impl crate::Resettable for L1_CACHE_ACS_FAIL_CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
