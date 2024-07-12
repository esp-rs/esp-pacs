#[doc = "Register `PRO_CACHE_MMU_POWER_CTRL` reader"]
pub type R = crate::R<PRO_CACHE_MMU_POWER_CTRL_SPEC>;
#[doc = "Register `PRO_CACHE_MMU_POWER_CTRL` writer"]
pub type W = crate::W<PRO_CACHE_MMU_POWER_CTRL_SPEC>;
#[doc = "Field `PRO_CACHE_MMU_MEM_FORCE_ON` reader - The bit is used to enable clock gating to save power when access mmu memory, 0: enable, 1: disable"]
pub type PRO_CACHE_MMU_MEM_FORCE_ON_R = crate::BitReader;
#[doc = "Field `PRO_CACHE_MMU_MEM_FORCE_ON` writer - The bit is used to enable clock gating to save power when access mmu memory, 0: enable, 1: disable"]
pub type PRO_CACHE_MMU_MEM_FORCE_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRO_CACHE_MMU_MEM_FORCE_PD` reader - The bit is used to power mmu memory down, 0: follow_rtc_lslp_pd, 1: power down"]
pub type PRO_CACHE_MMU_MEM_FORCE_PD_R = crate::BitReader;
#[doc = "Field `PRO_CACHE_MMU_MEM_FORCE_PD` writer - The bit is used to power mmu memory down, 0: follow_rtc_lslp_pd, 1: power down"]
pub type PRO_CACHE_MMU_MEM_FORCE_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRO_CACHE_MMU_MEM_FORCE_PU` reader - The bit is used to power mmu memory down, 0: follow_rtc_lslp_pd, 1: power up"]
pub type PRO_CACHE_MMU_MEM_FORCE_PU_R = crate::BitReader;
#[doc = "Field `PRO_CACHE_MMU_MEM_FORCE_PU` writer - The bit is used to power mmu memory down, 0: follow_rtc_lslp_pd, 1: power up"]
pub type PRO_CACHE_MMU_MEM_FORCE_PU_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The bit is used to enable clock gating to save power when access mmu memory, 0: enable, 1: disable"]
    #[inline(always)]
    pub fn pro_cache_mmu_mem_force_on(&self) -> PRO_CACHE_MMU_MEM_FORCE_ON_R {
        PRO_CACHE_MMU_MEM_FORCE_ON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The bit is used to power mmu memory down, 0: follow_rtc_lslp_pd, 1: power down"]
    #[inline(always)]
    pub fn pro_cache_mmu_mem_force_pd(&self) -> PRO_CACHE_MMU_MEM_FORCE_PD_R {
        PRO_CACHE_MMU_MEM_FORCE_PD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The bit is used to power mmu memory down, 0: follow_rtc_lslp_pd, 1: power up"]
    #[inline(always)]
    pub fn pro_cache_mmu_mem_force_pu(&self) -> PRO_CACHE_MMU_MEM_FORCE_PU_R {
        PRO_CACHE_MMU_MEM_FORCE_PU_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRO_CACHE_MMU_POWER_CTRL")
            .field(
                "pro_cache_mmu_mem_force_on",
                &self.pro_cache_mmu_mem_force_on(),
            )
            .field(
                "pro_cache_mmu_mem_force_pd",
                &self.pro_cache_mmu_mem_force_pd(),
            )
            .field(
                "pro_cache_mmu_mem_force_pu",
                &self.pro_cache_mmu_mem_force_pu(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - The bit is used to enable clock gating to save power when access mmu memory, 0: enable, 1: disable"]
    #[inline(always)]
    #[must_use]
    pub fn pro_cache_mmu_mem_force_on(
        &mut self,
    ) -> PRO_CACHE_MMU_MEM_FORCE_ON_W<PRO_CACHE_MMU_POWER_CTRL_SPEC> {
        PRO_CACHE_MMU_MEM_FORCE_ON_W::new(self, 0)
    }
    #[doc = "Bit 1 - The bit is used to power mmu memory down, 0: follow_rtc_lslp_pd, 1: power down"]
    #[inline(always)]
    #[must_use]
    pub fn pro_cache_mmu_mem_force_pd(
        &mut self,
    ) -> PRO_CACHE_MMU_MEM_FORCE_PD_W<PRO_CACHE_MMU_POWER_CTRL_SPEC> {
        PRO_CACHE_MMU_MEM_FORCE_PD_W::new(self, 1)
    }
    #[doc = "Bit 2 - The bit is used to power mmu memory down, 0: follow_rtc_lslp_pd, 1: power up"]
    #[inline(always)]
    #[must_use]
    pub fn pro_cache_mmu_mem_force_pu(
        &mut self,
    ) -> PRO_CACHE_MMU_MEM_FORCE_PU_W<PRO_CACHE_MMU_POWER_CTRL_SPEC> {
        PRO_CACHE_MMU_MEM_FORCE_PU_W::new(self, 2)
    }
}
#[doc = "register description\n\nYou can [`read`](crate::Reg::read) this register and get [`pro_cache_mmu_power_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pro_cache_mmu_power_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRO_CACHE_MMU_POWER_CTRL_SPEC;
impl crate::RegisterSpec for PRO_CACHE_MMU_POWER_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pro_cache_mmu_power_ctrl::R`](R) reader structure"]
impl crate::Readable for PRO_CACHE_MMU_POWER_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pro_cache_mmu_power_ctrl::W`](W) writer structure"]
impl crate::Writable for PRO_CACHE_MMU_POWER_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRO_CACHE_MMU_POWER_CTRL to value 0x05"]
impl crate::Resettable for PRO_CACHE_MMU_POWER_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x05;
}
