#[doc = "Register `APP_DCACHE_DBUG3` reader"]
pub type R = crate::R<APP_DCACHE_DBUG3_SPEC>;
#[doc = "Register `APP_DCACHE_DBUG3` writer"]
pub type W = crate::W<APP_DCACHE_DBUG3_SPEC>;
#[doc = "Field `APP_MMU_RDATA` reader - "]
pub type APP_MMU_RDATA_R = crate::FieldReader<u16>;
#[doc = "Field `APP_CPU_DISABLED_CACHE_IA` reader - "]
pub type APP_CPU_DISABLED_CACHE_IA_R = crate::FieldReader;
#[doc = "Field `APP_CPU_DISABLED_CACHE_IA_OPPOSITE` reader - "]
pub type APP_CPU_DISABLED_CACHE_IA_OPPOSITE_R = crate::BitReader;
#[doc = "Field `APP_CPU_DISABLED_CACHE_IA_OPPOSITE` writer - "]
pub type APP_CPU_DISABLED_CACHE_IA_OPPOSITE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APP_CPU_DISABLED_CACHE_IA_DRAM1` reader - "]
pub type APP_CPU_DISABLED_CACHE_IA_DRAM1_R = crate::BitReader;
#[doc = "Field `APP_CPU_DISABLED_CACHE_IA_DRAM1` writer - "]
pub type APP_CPU_DISABLED_CACHE_IA_DRAM1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APP_CPU_DISABLED_CACHE_IA_IROM0` reader - "]
pub type APP_CPU_DISABLED_CACHE_IA_IROM0_R = crate::BitReader;
#[doc = "Field `APP_CPU_DISABLED_CACHE_IA_IROM0` writer - "]
pub type APP_CPU_DISABLED_CACHE_IA_IROM0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APP_CPU_DISABLED_CACHE_IA_IRAM1` reader - "]
pub type APP_CPU_DISABLED_CACHE_IA_IRAM1_R = crate::BitReader;
#[doc = "Field `APP_CPU_DISABLED_CACHE_IA_IRAM1` writer - "]
pub type APP_CPU_DISABLED_CACHE_IA_IRAM1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APP_CPU_DISABLED_CACHE_IA_IRAM0` reader - "]
pub type APP_CPU_DISABLED_CACHE_IA_IRAM0_R = crate::BitReader;
#[doc = "Field `APP_CPU_DISABLED_CACHE_IA_IRAM0` writer - "]
pub type APP_CPU_DISABLED_CACHE_IA_IRAM0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APP_CPU_DISABLED_CACHE_IA_DROM0` reader - "]
pub type APP_CPU_DISABLED_CACHE_IA_DROM0_R = crate::BitReader;
#[doc = "Field `APP_CPU_DISABLED_CACHE_IA_DROM0` writer - "]
pub type APP_CPU_DISABLED_CACHE_IA_DROM0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APP_CACHE_IRAM0_PID_ERROR` reader - "]
pub type APP_CACHE_IRAM0_PID_ERROR_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn app_mmu_rdata(&self) -> APP_MMU_RDATA_R {
        APP_MMU_RDATA_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:14"]
    #[inline(always)]
    pub fn app_cpu_disabled_cache_ia(&self) -> APP_CPU_DISABLED_CACHE_IA_R {
        APP_CPU_DISABLED_CACHE_IA_R::new(((self.bits >> 9) & 0x3f) as u8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn app_cpu_disabled_cache_ia_opposite(&self) -> APP_CPU_DISABLED_CACHE_IA_OPPOSITE_R {
        APP_CPU_DISABLED_CACHE_IA_OPPOSITE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn app_cpu_disabled_cache_ia_dram1(&self) -> APP_CPU_DISABLED_CACHE_IA_DRAM1_R {
        APP_CPU_DISABLED_CACHE_IA_DRAM1_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn app_cpu_disabled_cache_ia_irom0(&self) -> APP_CPU_DISABLED_CACHE_IA_IROM0_R {
        APP_CPU_DISABLED_CACHE_IA_IROM0_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn app_cpu_disabled_cache_ia_iram1(&self) -> APP_CPU_DISABLED_CACHE_IA_IRAM1_R {
        APP_CPU_DISABLED_CACHE_IA_IRAM1_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn app_cpu_disabled_cache_ia_iram0(&self) -> APP_CPU_DISABLED_CACHE_IA_IRAM0_R {
        APP_CPU_DISABLED_CACHE_IA_IRAM0_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn app_cpu_disabled_cache_ia_drom0(&self) -> APP_CPU_DISABLED_CACHE_IA_DROM0_R {
        APP_CPU_DISABLED_CACHE_IA_DROM0_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn app_cache_iram0_pid_error(&self) -> APP_CACHE_IRAM0_PID_ERROR_R {
        APP_CACHE_IRAM0_PID_ERROR_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APP_DCACHE_DBUG3")
            .field("app_mmu_rdata", &self.app_mmu_rdata().bits())
            .field(
                "app_cpu_disabled_cache_ia",
                &self.app_cpu_disabled_cache_ia().bits(),
            )
            .field(
                "app_cpu_disabled_cache_ia_opposite",
                &self.app_cpu_disabled_cache_ia_opposite().bit(),
            )
            .field(
                "app_cpu_disabled_cache_ia_dram1",
                &self.app_cpu_disabled_cache_ia_dram1().bit(),
            )
            .field(
                "app_cpu_disabled_cache_ia_irom0",
                &self.app_cpu_disabled_cache_ia_irom0().bit(),
            )
            .field(
                "app_cpu_disabled_cache_ia_iram1",
                &self.app_cpu_disabled_cache_ia_iram1().bit(),
            )
            .field(
                "app_cpu_disabled_cache_ia_iram0",
                &self.app_cpu_disabled_cache_ia_iram0().bit(),
            )
            .field(
                "app_cpu_disabled_cache_ia_drom0",
                &self.app_cpu_disabled_cache_ia_drom0().bit(),
            )
            .field(
                "app_cache_iram0_pid_error",
                &self.app_cache_iram0_pid_error().bit(),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<APP_DCACHE_DBUG3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn app_cpu_disabled_cache_ia_opposite(
        &mut self,
    ) -> APP_CPU_DISABLED_CACHE_IA_OPPOSITE_W<APP_DCACHE_DBUG3_SPEC> {
        APP_CPU_DISABLED_CACHE_IA_OPPOSITE_W::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn app_cpu_disabled_cache_ia_dram1(
        &mut self,
    ) -> APP_CPU_DISABLED_CACHE_IA_DRAM1_W<APP_DCACHE_DBUG3_SPEC> {
        APP_CPU_DISABLED_CACHE_IA_DRAM1_W::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn app_cpu_disabled_cache_ia_irom0(
        &mut self,
    ) -> APP_CPU_DISABLED_CACHE_IA_IROM0_W<APP_DCACHE_DBUG3_SPEC> {
        APP_CPU_DISABLED_CACHE_IA_IROM0_W::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn app_cpu_disabled_cache_ia_iram1(
        &mut self,
    ) -> APP_CPU_DISABLED_CACHE_IA_IRAM1_W<APP_DCACHE_DBUG3_SPEC> {
        APP_CPU_DISABLED_CACHE_IA_IRAM1_W::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn app_cpu_disabled_cache_ia_iram0(
        &mut self,
    ) -> APP_CPU_DISABLED_CACHE_IA_IRAM0_W<APP_DCACHE_DBUG3_SPEC> {
        APP_CPU_DISABLED_CACHE_IA_IRAM0_W::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn app_cpu_disabled_cache_ia_drom0(
        &mut self,
    ) -> APP_CPU_DISABLED_CACHE_IA_DROM0_W<APP_DCACHE_DBUG3_SPEC> {
        APP_CPU_DISABLED_CACHE_IA_DROM0_W::new(self, 14)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_dcache_dbug3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`app_dcache_dbug3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APP_DCACHE_DBUG3_SPEC;
impl crate::RegisterSpec for APP_DCACHE_DBUG3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`app_dcache_dbug3::R`](R) reader structure"]
impl crate::Readable for APP_DCACHE_DBUG3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`app_dcache_dbug3::W`](W) writer structure"]
impl crate::Writable for APP_DCACHE_DBUG3_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APP_DCACHE_DBUG3 to value 0"]
impl crate::Resettable for APP_DCACHE_DBUG3_SPEC {
    const RESET_VALUE: u32 = 0;
}
