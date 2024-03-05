#[doc = "Register `POWER_CK_WAIT_CNTL` reader"]
pub type R = crate::R<POWER_CK_WAIT_CNTL_SPEC>;
#[doc = "Register `POWER_CK_WAIT_CNTL` writer"]
pub type W = crate::W<POWER_CK_WAIT_CNTL_SPEC>;
#[doc = "Field `PMU_WAIT_XTL_STABLE` reader - need_des"]
pub type PMU_WAIT_XTL_STABLE_R = crate::FieldReader<u16>;
#[doc = "Field `PMU_WAIT_XTL_STABLE` writer - need_des"]
pub type PMU_WAIT_XTL_STABLE_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `PMU_WAIT_PLL_STABLE` reader - need_des"]
pub type PMU_WAIT_PLL_STABLE_R = crate::FieldReader<u16>;
#[doc = "Field `PMU_WAIT_PLL_STABLE` writer - need_des"]
pub type PMU_WAIT_PLL_STABLE_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - need_des"]
    #[inline(always)]
    pub fn pmu_wait_xtl_stable(&self) -> PMU_WAIT_XTL_STABLE_R {
        PMU_WAIT_XTL_STABLE_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - need_des"]
    #[inline(always)]
    pub fn pmu_wait_pll_stable(&self) -> PMU_WAIT_PLL_STABLE_R {
        PMU_WAIT_PLL_STABLE_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("POWER_CK_WAIT_CNTL")
            .field(
                "pmu_wait_xtl_stable",
                &format_args!("{}", self.pmu_wait_xtl_stable().bits()),
            )
            .field(
                "pmu_wait_pll_stable",
                &format_args!("{}", self.pmu_wait_pll_stable().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<POWER_CK_WAIT_CNTL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:15 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn pmu_wait_xtl_stable(&mut self) -> PMU_WAIT_XTL_STABLE_W<POWER_CK_WAIT_CNTL_SPEC> {
        PMU_WAIT_XTL_STABLE_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn pmu_wait_pll_stable(&mut self) -> PMU_WAIT_PLL_STABLE_W<POWER_CK_WAIT_CNTL_SPEC> {
        PMU_WAIT_PLL_STABLE_W::new(self, 16)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`power_ck_wait_cntl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`power_ck_wait_cntl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct POWER_CK_WAIT_CNTL_SPEC;
impl crate::RegisterSpec for POWER_CK_WAIT_CNTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`power_ck_wait_cntl::R`](R) reader structure"]
impl crate::Readable for POWER_CK_WAIT_CNTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`power_ck_wait_cntl::W`](W) writer structure"]
impl crate::Writable for POWER_CK_WAIT_CNTL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets POWER_CK_WAIT_CNTL to value 0x0100_0100"]
impl crate::Resettable for POWER_CK_WAIT_CNTL_SPEC {
    const RESET_VALUE: u32 = 0x0100_0100;
}
