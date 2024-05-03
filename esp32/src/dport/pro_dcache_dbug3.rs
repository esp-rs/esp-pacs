#[doc = "Register `PRO_DCACHE_DBUG3` reader"]
pub type R = crate::R<PRO_DCACHE_DBUG3_SPEC>;
#[doc = "Register `PRO_DCACHE_DBUG3` writer"]
pub type W = crate::W<PRO_DCACHE_DBUG3_SPEC>;
#[doc = "Field `PRO_MMU_RDATA` reader - "]
pub type PRO_MMU_RDATA_R = crate::FieldReader<u16>;
#[doc = "Field `PRO_CPU_DISABLED_CACHE_IA` reader - "]
pub type PRO_CPU_DISABLED_CACHE_IA_R = crate::FieldReader;
#[doc = "Field `PRO_CPU_DISABLED_CACHE_IA_OPPOSITE` reader - "]
pub type PRO_CPU_DISABLED_CACHE_IA_OPPOSITE_R = crate::BitReader;
#[doc = "Field `PRO_CPU_DISABLED_CACHE_IA_OPPOSITE` writer - "]
pub type PRO_CPU_DISABLED_CACHE_IA_OPPOSITE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRO_CPU_DISABLED_CACHE_IA_DRAM1` reader - "]
pub type PRO_CPU_DISABLED_CACHE_IA_DRAM1_R = crate::BitReader;
#[doc = "Field `PRO_CPU_DISABLED_CACHE_IA_DRAM1` writer - "]
pub type PRO_CPU_DISABLED_CACHE_IA_DRAM1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRO_CPU_DISABLED_CACHE_IA_IROM0` reader - "]
pub type PRO_CPU_DISABLED_CACHE_IA_IROM0_R = crate::BitReader;
#[doc = "Field `PRO_CPU_DISABLED_CACHE_IA_IROM0` writer - "]
pub type PRO_CPU_DISABLED_CACHE_IA_IROM0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRO_CPU_DISABLED_CACHE_IA_IRAM1` reader - "]
pub type PRO_CPU_DISABLED_CACHE_IA_IRAM1_R = crate::BitReader;
#[doc = "Field `PRO_CPU_DISABLED_CACHE_IA_IRAM1` writer - "]
pub type PRO_CPU_DISABLED_CACHE_IA_IRAM1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRO_CPU_DISABLED_CACHE_IA_IRAM0` reader - "]
pub type PRO_CPU_DISABLED_CACHE_IA_IRAM0_R = crate::BitReader;
#[doc = "Field `PRO_CPU_DISABLED_CACHE_IA_IRAM0` writer - "]
pub type PRO_CPU_DISABLED_CACHE_IA_IRAM0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRO_CPU_DISABLED_CACHE_IA_DROM0` reader - "]
pub type PRO_CPU_DISABLED_CACHE_IA_DROM0_R = crate::BitReader;
#[doc = "Field `PRO_CPU_DISABLED_CACHE_IA_DROM0` writer - "]
pub type PRO_CPU_DISABLED_CACHE_IA_DROM0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRO_CACHE_IRAM0_PID_ERROR` reader - "]
pub type PRO_CACHE_IRAM0_PID_ERROR_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn pro_mmu_rdata(&self) -> PRO_MMU_RDATA_R {
        PRO_MMU_RDATA_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:14"]
    #[inline(always)]
    pub fn pro_cpu_disabled_cache_ia(&self) -> PRO_CPU_DISABLED_CACHE_IA_R {
        PRO_CPU_DISABLED_CACHE_IA_R::new(((self.bits >> 9) & 0x3f) as u8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn pro_cpu_disabled_cache_ia_opposite(&self) -> PRO_CPU_DISABLED_CACHE_IA_OPPOSITE_R {
        PRO_CPU_DISABLED_CACHE_IA_OPPOSITE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn pro_cpu_disabled_cache_ia_dram1(&self) -> PRO_CPU_DISABLED_CACHE_IA_DRAM1_R {
        PRO_CPU_DISABLED_CACHE_IA_DRAM1_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn pro_cpu_disabled_cache_ia_irom0(&self) -> PRO_CPU_DISABLED_CACHE_IA_IROM0_R {
        PRO_CPU_DISABLED_CACHE_IA_IROM0_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn pro_cpu_disabled_cache_ia_iram1(&self) -> PRO_CPU_DISABLED_CACHE_IA_IRAM1_R {
        PRO_CPU_DISABLED_CACHE_IA_IRAM1_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn pro_cpu_disabled_cache_ia_iram0(&self) -> PRO_CPU_DISABLED_CACHE_IA_IRAM0_R {
        PRO_CPU_DISABLED_CACHE_IA_IRAM0_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn pro_cpu_disabled_cache_ia_drom0(&self) -> PRO_CPU_DISABLED_CACHE_IA_DROM0_R {
        PRO_CPU_DISABLED_CACHE_IA_DROM0_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn pro_cache_iram0_pid_error(&self) -> PRO_CACHE_IRAM0_PID_ERROR_R {
        PRO_CACHE_IRAM0_PID_ERROR_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRO_DCACHE_DBUG3")
            .field("pro_mmu_rdata", &self.pro_mmu_rdata().bits())
            .field(
                "pro_cpu_disabled_cache_ia",
                &self.pro_cpu_disabled_cache_ia().bits(),
            )
            .field(
                "pro_cpu_disabled_cache_ia_opposite",
                &self.pro_cpu_disabled_cache_ia_opposite().bit(),
            )
            .field(
                "pro_cpu_disabled_cache_ia_dram1",
                &self.pro_cpu_disabled_cache_ia_dram1().bit(),
            )
            .field(
                "pro_cpu_disabled_cache_ia_irom0",
                &self.pro_cpu_disabled_cache_ia_irom0().bit(),
            )
            .field(
                "pro_cpu_disabled_cache_ia_iram1",
                &self.pro_cpu_disabled_cache_ia_iram1().bit(),
            )
            .field(
                "pro_cpu_disabled_cache_ia_iram0",
                &self.pro_cpu_disabled_cache_ia_iram0().bit(),
            )
            .field(
                "pro_cpu_disabled_cache_ia_drom0",
                &self.pro_cpu_disabled_cache_ia_drom0().bit(),
            )
            .field(
                "pro_cache_iram0_pid_error",
                &self.pro_cache_iram0_pid_error().bit(),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PRO_DCACHE_DBUG3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn pro_cpu_disabled_cache_ia_opposite(
        &mut self,
    ) -> PRO_CPU_DISABLED_CACHE_IA_OPPOSITE_W<PRO_DCACHE_DBUG3_SPEC> {
        PRO_CPU_DISABLED_CACHE_IA_OPPOSITE_W::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn pro_cpu_disabled_cache_ia_dram1(
        &mut self,
    ) -> PRO_CPU_DISABLED_CACHE_IA_DRAM1_W<PRO_DCACHE_DBUG3_SPEC> {
        PRO_CPU_DISABLED_CACHE_IA_DRAM1_W::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn pro_cpu_disabled_cache_ia_irom0(
        &mut self,
    ) -> PRO_CPU_DISABLED_CACHE_IA_IROM0_W<PRO_DCACHE_DBUG3_SPEC> {
        PRO_CPU_DISABLED_CACHE_IA_IROM0_W::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn pro_cpu_disabled_cache_ia_iram1(
        &mut self,
    ) -> PRO_CPU_DISABLED_CACHE_IA_IRAM1_W<PRO_DCACHE_DBUG3_SPEC> {
        PRO_CPU_DISABLED_CACHE_IA_IRAM1_W::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn pro_cpu_disabled_cache_ia_iram0(
        &mut self,
    ) -> PRO_CPU_DISABLED_CACHE_IA_IRAM0_W<PRO_DCACHE_DBUG3_SPEC> {
        PRO_CPU_DISABLED_CACHE_IA_IRAM0_W::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn pro_cpu_disabled_cache_ia_drom0(
        &mut self,
    ) -> PRO_CPU_DISABLED_CACHE_IA_DROM0_W<PRO_DCACHE_DBUG3_SPEC> {
        PRO_CPU_DISABLED_CACHE_IA_DROM0_W::new(self, 14)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_dcache_dbug3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_dcache_dbug3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRO_DCACHE_DBUG3_SPEC;
impl crate::RegisterSpec for PRO_DCACHE_DBUG3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pro_dcache_dbug3::R`](R) reader structure"]
impl crate::Readable for PRO_DCACHE_DBUG3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pro_dcache_dbug3::W`](W) writer structure"]
impl crate::Writable for PRO_DCACHE_DBUG3_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRO_DCACHE_DBUG3 to value 0"]
impl crate::Resettable for PRO_DCACHE_DBUG3_SPEC {
    const RESET_VALUE: u32 = 0;
}
