#[doc = "Register `CPUCORE_CFG` reader"]
pub type R = crate::R<CPUCORE_CFG_SPEC>;
#[doc = "Register `CPUCORE_CFG` writer"]
pub type W = crate::W<CPUCORE_CFG_SPEC>;
#[doc = "Field `CPU_CORE0_SW_STALL` reader - enable cpu 0 entry stall status 0x86: entry stall status Others : no operation"]
pub type CPU_CORE0_SW_STALL_R = crate::FieldReader;
#[doc = "Field `CPU_CORE0_SW_STALL` writer - enable cpu 0 entry stall status 0x86: entry stall status Others : no operation"]
pub type CPU_CORE0_SW_STALL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CPU_CORE0_SW_RESET` writer - enable core 0 reset by software 1: reset 0: no operation"]
pub type CPU_CORE0_SW_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPU_CORE0_OCD_HALT_ON_RESET` reader - reserved"]
pub type CPU_CORE0_OCD_HALT_ON_RESET_R = crate::BitReader;
#[doc = "Field `CPU_CORE0_OCD_HALT_ON_RESET` writer - reserved"]
pub type CPU_CORE0_OCD_HALT_ON_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPU_CORE0_STAT_VECTOR_SEL` reader - configure core 0 boot address 1: ROM 0: lp memory"]
pub type CPU_CORE0_STAT_VECTOR_SEL_R = crate::BitReader;
#[doc = "Field `CPU_CORE0_STAT_VECTOR_SEL` writer - configure core 0 boot address 1: ROM 0: lp memory"]
pub type CPU_CORE0_STAT_VECTOR_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPU_CORE1_SW_STALL` reader - enable core 1 entry stall status 0x86: entry stall status Others : no operation"]
pub type CPU_CORE1_SW_STALL_R = crate::FieldReader;
#[doc = "Field `CPU_CORE1_SW_STALL` writer - enable core 1 entry stall status 0x86: entry stall status Others : no operation"]
pub type CPU_CORE1_SW_STALL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CPU_CORE1_SW_RESET` writer - enable core1 reset by software 1: reset 0: no operation"]
pub type CPU_CORE1_SW_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPU_CORE1_OCD_HALT_ON_RESET` reader - reserved"]
pub type CPU_CORE1_OCD_HALT_ON_RESET_R = crate::BitReader;
#[doc = "Field `CPU_CORE1_OCD_HALT_ON_RESET` writer - reserved"]
pub type CPU_CORE1_OCD_HALT_ON_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPU_CORE1_STAT_VECTOR_SEL` reader - configure core1 boot address 1: ROM 0: lp memory"]
pub type CPU_CORE1_STAT_VECTOR_SEL_R = crate::BitReader;
#[doc = "Field `CPU_CORE1_STAT_VECTOR_SEL` writer - configure core1 boot address 1: ROM 0: lp memory"]
pub type CPU_CORE1_STAT_VECTOR_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSTIMER_STALL_SEL` reader - selete which core run_stall to lp_timer 1: core1 0: core0"]
pub type SYSTIMER_STALL_SEL_R = crate::BitReader;
#[doc = "Field `SYSTIMER_STALL_SEL` writer - selete which core run_stall to lp_timer 1: core1 0: core0"]
pub type SYSTIMER_STALL_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - enable cpu 0 entry stall status 0x86: entry stall status Others : no operation"]
    #[inline(always)]
    pub fn cpu_core0_sw_stall(&self) -> CPU_CORE0_SW_STALL_R {
        CPU_CORE0_SW_STALL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 9 - reserved"]
    #[inline(always)]
    pub fn cpu_core0_ocd_halt_on_reset(&self) -> CPU_CORE0_OCD_HALT_ON_RESET_R {
        CPU_CORE0_OCD_HALT_ON_RESET_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - configure core 0 boot address 1: ROM 0: lp memory"]
    #[inline(always)]
    pub fn cpu_core0_stat_vector_sel(&self) -> CPU_CORE0_STAT_VECTOR_SEL_R {
        CPU_CORE0_STAT_VECTOR_SEL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 16:23 - enable core 1 entry stall status 0x86: entry stall status Others : no operation"]
    #[inline(always)]
    pub fn cpu_core1_sw_stall(&self) -> CPU_CORE1_SW_STALL_R {
        CPU_CORE1_SW_STALL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 25 - reserved"]
    #[inline(always)]
    pub fn cpu_core1_ocd_halt_on_reset(&self) -> CPU_CORE1_OCD_HALT_ON_RESET_R {
        CPU_CORE1_OCD_HALT_ON_RESET_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - configure core1 boot address 1: ROM 0: lp memory"]
    #[inline(always)]
    pub fn cpu_core1_stat_vector_sel(&self) -> CPU_CORE1_STAT_VECTOR_SEL_R {
        CPU_CORE1_STAT_VECTOR_SEL_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 31 - selete which core run_stall to lp_timer 1: core1 0: core0"]
    #[inline(always)]
    pub fn systimer_stall_sel(&self) -> SYSTIMER_STALL_SEL_R {
        SYSTIMER_STALL_SEL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPUCORE_CFG")
            .field("cpu_core0_sw_stall", &self.cpu_core0_sw_stall())
            .field(
                "cpu_core0_ocd_halt_on_reset",
                &self.cpu_core0_ocd_halt_on_reset(),
            )
            .field(
                "cpu_core0_stat_vector_sel",
                &self.cpu_core0_stat_vector_sel(),
            )
            .field("cpu_core1_sw_stall", &self.cpu_core1_sw_stall())
            .field(
                "cpu_core1_ocd_halt_on_reset",
                &self.cpu_core1_ocd_halt_on_reset(),
            )
            .field(
                "cpu_core1_stat_vector_sel",
                &self.cpu_core1_stat_vector_sel(),
            )
            .field("systimer_stall_sel", &self.systimer_stall_sel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - enable cpu 0 entry stall status 0x86: entry stall status Others : no operation"]
    #[inline(always)]
    pub fn cpu_core0_sw_stall(&mut self) -> CPU_CORE0_SW_STALL_W<'_, CPUCORE_CFG_SPEC> {
        CPU_CORE0_SW_STALL_W::new(self, 0)
    }
    #[doc = "Bit 8 - enable core 0 reset by software 1: reset 0: no operation"]
    #[inline(always)]
    pub fn cpu_core0_sw_reset(&mut self) -> CPU_CORE0_SW_RESET_W<'_, CPUCORE_CFG_SPEC> {
        CPU_CORE0_SW_RESET_W::new(self, 8)
    }
    #[doc = "Bit 9 - reserved"]
    #[inline(always)]
    pub fn cpu_core0_ocd_halt_on_reset(
        &mut self,
    ) -> CPU_CORE0_OCD_HALT_ON_RESET_W<'_, CPUCORE_CFG_SPEC> {
        CPU_CORE0_OCD_HALT_ON_RESET_W::new(self, 9)
    }
    #[doc = "Bit 10 - configure core 0 boot address 1: ROM 0: lp memory"]
    #[inline(always)]
    pub fn cpu_core0_stat_vector_sel(
        &mut self,
    ) -> CPU_CORE0_STAT_VECTOR_SEL_W<'_, CPUCORE_CFG_SPEC> {
        CPU_CORE0_STAT_VECTOR_SEL_W::new(self, 10)
    }
    #[doc = "Bits 16:23 - enable core 1 entry stall status 0x86: entry stall status Others : no operation"]
    #[inline(always)]
    pub fn cpu_core1_sw_stall(&mut self) -> CPU_CORE1_SW_STALL_W<'_, CPUCORE_CFG_SPEC> {
        CPU_CORE1_SW_STALL_W::new(self, 16)
    }
    #[doc = "Bit 24 - enable core1 reset by software 1: reset 0: no operation"]
    #[inline(always)]
    pub fn cpu_core1_sw_reset(&mut self) -> CPU_CORE1_SW_RESET_W<'_, CPUCORE_CFG_SPEC> {
        CPU_CORE1_SW_RESET_W::new(self, 24)
    }
    #[doc = "Bit 25 - reserved"]
    #[inline(always)]
    pub fn cpu_core1_ocd_halt_on_reset(
        &mut self,
    ) -> CPU_CORE1_OCD_HALT_ON_RESET_W<'_, CPUCORE_CFG_SPEC> {
        CPU_CORE1_OCD_HALT_ON_RESET_W::new(self, 25)
    }
    #[doc = "Bit 26 - configure core1 boot address 1: ROM 0: lp memory"]
    #[inline(always)]
    pub fn cpu_core1_stat_vector_sel(
        &mut self,
    ) -> CPU_CORE1_STAT_VECTOR_SEL_W<'_, CPUCORE_CFG_SPEC> {
        CPU_CORE1_STAT_VECTOR_SEL_W::new(self, 26)
    }
    #[doc = "Bit 31 - selete which core run_stall to lp_timer 1: core1 0: core0"]
    #[inline(always)]
    pub fn systimer_stall_sel(&mut self) -> SYSTIMER_STALL_SEL_W<'_, CPUCORE_CFG_SPEC> {
        SYSTIMER_STALL_SEL_W::new(self, 31)
    }
}
#[doc = "configure core reset register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpucore_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpucore_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CPUCORE_CFG_SPEC;
impl crate::RegisterSpec for CPUCORE_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpucore_cfg::R`](R) reader structure"]
impl crate::Readable for CPUCORE_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cpucore_cfg::W`](W) writer structure"]
impl crate::Writable for CPUCORE_CFG_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CPUCORE_CFG to value 0x0400_0400"]
impl crate::Resettable for CPUCORE_CFG_SPEC {
    const RESET_VALUE: u32 = 0x0400_0400;
}
