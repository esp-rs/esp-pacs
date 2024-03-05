#[doc = "Register `CACHE_MMU_POWER_CTRL` reader"]
pub type R = crate::R<CACHE_MMU_POWER_CTRL_SPEC>;
#[doc = "Register `CACHE_MMU_POWER_CTRL` writer"]
pub type W = crate::W<CACHE_MMU_POWER_CTRL_SPEC>;
#[doc = "Field `CACHE_MMU_MEM_FORCE_ON` reader - The bit is used to enable clock gating to save power when access mmu memory, 0: enable, 1: disable"]
pub type CACHE_MMU_MEM_FORCE_ON_R = crate::BitReader;
#[doc = "Field `CACHE_MMU_MEM_FORCE_ON` writer - The bit is used to enable clock gating to save power when access mmu memory, 0: enable, 1: disable"]
pub type CACHE_MMU_MEM_FORCE_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CACHE_MMU_MEM_FORCE_PD` reader - The bit is used to power mmu memory down, 0: follow_rtc_lslp_pd, 1: power down"]
pub type CACHE_MMU_MEM_FORCE_PD_R = crate::BitReader;
#[doc = "Field `CACHE_MMU_MEM_FORCE_PD` writer - The bit is used to power mmu memory down, 0: follow_rtc_lslp_pd, 1: power down"]
pub type CACHE_MMU_MEM_FORCE_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CACHE_MMU_MEM_FORCE_PU` reader - The bit is used to power mmu memory down, 0: follow_rtc_lslp_pd, 1: power up"]
pub type CACHE_MMU_MEM_FORCE_PU_R = crate::BitReader;
#[doc = "Field `CACHE_MMU_MEM_FORCE_PU` writer - The bit is used to power mmu memory down, 0: follow_rtc_lslp_pd, 1: power up"]
pub type CACHE_MMU_MEM_FORCE_PU_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The bit is used to enable clock gating to save power when access mmu memory, 0: enable, 1: disable"]
    #[inline(always)]
    pub fn cache_mmu_mem_force_on(&self) -> CACHE_MMU_MEM_FORCE_ON_R {
        CACHE_MMU_MEM_FORCE_ON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The bit is used to power mmu memory down, 0: follow_rtc_lslp_pd, 1: power down"]
    #[inline(always)]
    pub fn cache_mmu_mem_force_pd(&self) -> CACHE_MMU_MEM_FORCE_PD_R {
        CACHE_MMU_MEM_FORCE_PD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The bit is used to power mmu memory down, 0: follow_rtc_lslp_pd, 1: power up"]
    #[inline(always)]
    pub fn cache_mmu_mem_force_pu(&self) -> CACHE_MMU_MEM_FORCE_PU_R {
        CACHE_MMU_MEM_FORCE_PU_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_MMU_POWER_CTRL")
            .field(
                "cache_mmu_mem_force_on",
                &format_args!("{}", self.cache_mmu_mem_force_on().bit()),
            )
            .field(
                "cache_mmu_mem_force_pd",
                &format_args!("{}", self.cache_mmu_mem_force_pd().bit()),
            )
            .field(
                "cache_mmu_mem_force_pu",
                &format_args!("{}", self.cache_mmu_mem_force_pu().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CACHE_MMU_POWER_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - The bit is used to enable clock gating to save power when access mmu memory, 0: enable, 1: disable"]
    #[inline(always)]
    #[must_use]
    pub fn cache_mmu_mem_force_on(
        &mut self,
    ) -> CACHE_MMU_MEM_FORCE_ON_W<CACHE_MMU_POWER_CTRL_SPEC> {
        CACHE_MMU_MEM_FORCE_ON_W::new(self, 0)
    }
    #[doc = "Bit 1 - The bit is used to power mmu memory down, 0: follow_rtc_lslp_pd, 1: power down"]
    #[inline(always)]
    #[must_use]
    pub fn cache_mmu_mem_force_pd(
        &mut self,
    ) -> CACHE_MMU_MEM_FORCE_PD_W<CACHE_MMU_POWER_CTRL_SPEC> {
        CACHE_MMU_MEM_FORCE_PD_W::new(self, 1)
    }
    #[doc = "Bit 2 - The bit is used to power mmu memory down, 0: follow_rtc_lslp_pd, 1: power up"]
    #[inline(always)]
    #[must_use]
    pub fn cache_mmu_mem_force_pu(
        &mut self,
    ) -> CACHE_MMU_MEM_FORCE_PU_W<CACHE_MMU_POWER_CTRL_SPEC> {
        CACHE_MMU_MEM_FORCE_PU_W::new(self, 2)
    }
}
#[doc = "This description will be updated in the near future.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cache_mmu_power_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cache_mmu_power_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CACHE_MMU_POWER_CTRL_SPEC;
impl crate::RegisterSpec for CACHE_MMU_POWER_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cache_mmu_power_ctrl::R`](R) reader structure"]
impl crate::Readable for CACHE_MMU_POWER_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cache_mmu_power_ctrl::W`](W) writer structure"]
impl crate::Writable for CACHE_MMU_POWER_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CACHE_MMU_POWER_CTRL to value 0x05"]
impl crate::Resettable for CACHE_MMU_POWER_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x05;
}
