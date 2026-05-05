#[doc = "Register `POWER_CK_WAIT_CNTL` reader"]
pub type R = crate::R<POWER_CK_WAIT_CNTL_SPEC>;
#[doc = "Register `POWER_CK_WAIT_CNTL` writer"]
pub type W = crate::W<POWER_CK_WAIT_CNTL_SPEC>;
#[doc = "Field `PMU_WAIT_XTL_STABLE` reader - PMU_PMU_WAIT_XTL_STABLE"]
pub type PMU_WAIT_XTL_STABLE_R = crate::FieldReader<u16>;
#[doc = "Field `PMU_WAIT_XTL_STABLE` writer - PMU_PMU_WAIT_XTL_STABLE"]
pub type PMU_WAIT_XTL_STABLE_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `PMU_WAIT_PLL_STABLE` reader - PMU_PMU_WAIT_PLL_STABLE"]
pub type PMU_WAIT_PLL_STABLE_R = crate::FieldReader<u16>;
#[doc = "Field `PMU_WAIT_PLL_STABLE` writer - PMU_PMU_WAIT_PLL_STABLE"]
pub type PMU_WAIT_PLL_STABLE_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - PMU_PMU_WAIT_XTL_STABLE"]
    #[inline(always)]
    pub fn pmu_wait_xtl_stable(&self) -> PMU_WAIT_XTL_STABLE_R {
        PMU_WAIT_XTL_STABLE_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - PMU_PMU_WAIT_PLL_STABLE"]
    #[inline(always)]
    pub fn pmu_wait_pll_stable(&self) -> PMU_WAIT_PLL_STABLE_R {
        PMU_WAIT_PLL_STABLE_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("POWER_CK_WAIT_CNTL")
            .field("pmu_wait_xtl_stable", &self.pmu_wait_xtl_stable())
            .field("pmu_wait_pll_stable", &self.pmu_wait_pll_stable())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - PMU_PMU_WAIT_XTL_STABLE"]
    #[inline(always)]
    pub fn pmu_wait_xtl_stable(&mut self) -> PMU_WAIT_XTL_STABLE_W<'_, POWER_CK_WAIT_CNTL_SPEC> {
        PMU_WAIT_XTL_STABLE_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - PMU_PMU_WAIT_PLL_STABLE"]
    #[inline(always)]
    pub fn pmu_wait_pll_stable(&mut self) -> PMU_WAIT_PLL_STABLE_W<'_, POWER_CK_WAIT_CNTL_SPEC> {
        PMU_WAIT_PLL_STABLE_W::new(self, 16)
    }
}
#[doc = "PMU_POWER_CK_WAIT_CNTL\n\nYou can [`read`](crate::Reg::read) this register and get [`power_ck_wait_cntl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`power_ck_wait_cntl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct POWER_CK_WAIT_CNTL_SPEC;
impl crate::RegisterSpec for POWER_CK_WAIT_CNTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`power_ck_wait_cntl::R`](R) reader structure"]
impl crate::Readable for POWER_CK_WAIT_CNTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`power_ck_wait_cntl::W`](W) writer structure"]
impl crate::Writable for POWER_CK_WAIT_CNTL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets POWER_CK_WAIT_CNTL to value 0"]
impl crate::Resettable for POWER_CK_WAIT_CNTL_SPEC {}
