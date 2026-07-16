#[doc = "Register `LPCORE_RESET_CAUSE` reader"]
pub type R = crate::R<LPCORE_RESET_CAUSE_SPEC>;
#[doc = "Register `LPCORE_RESET_CAUSE` writer"]
pub type W = crate::W<LPCORE_RESET_CAUSE_SPEC>;
#[doc = "Field `LPCORE_RESET_FLAG` reader - need_des"]
pub type LPCORE_RESET_FLAG_R = crate::BitReader;
#[doc = "Field `LPCORE_RESET_CAUSE` reader - 6'h1: POR reset 6'h9: PMU LP PERI power down reset 6'ha: PMU LP CPU reset 6'hf: brown out reset 6'h10: LP watchdog chip reset 6'h12: super watch dog reset 6'h13: glitch reset 6'h14: software reset"]
pub type LPCORE_RESET_CAUSE_R = crate::FieldReader;
#[doc = "Field `PMU_LP_CPU_MASK` reader - 1'b0: enable lpcore pmu_lp_cpu_reset reset_cause, 1'b1: disable lpcore pmu_lp_cpu_reset reset_cause"]
pub type PMU_LP_CPU_MASK_R = crate::BitReader;
#[doc = "Field `PMU_LP_CPU_MASK` writer - 1'b0: enable lpcore pmu_lp_cpu_reset reset_cause, 1'b1: disable lpcore pmu_lp_cpu_reset reset_cause"]
pub type PMU_LP_CPU_MASK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLR` writer - need_des"]
pub type CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPCORE_RESET_FLAG_CLR` writer - need_des"]
pub type LPCORE_RESET_FLAG_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn lpcore_reset_flag(&self) -> LPCORE_RESET_FLAG_R {
        LPCORE_RESET_FLAG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:6 - 6'h1: POR reset 6'h9: PMU LP PERI power down reset 6'ha: PMU LP CPU reset 6'hf: brown out reset 6'h10: LP watchdog chip reset 6'h12: super watch dog reset 6'h13: glitch reset 6'h14: software reset"]
    #[inline(always)]
    pub fn lpcore_reset_cause(&self) -> LPCORE_RESET_CAUSE_R {
        LPCORE_RESET_CAUSE_R::new(((self.bits >> 1) & 0x3f) as u8)
    }
    #[doc = "Bit 29 - 1'b0: enable lpcore pmu_lp_cpu_reset reset_cause, 1'b1: disable lpcore pmu_lp_cpu_reset reset_cause"]
    #[inline(always)]
    pub fn pmu_lp_cpu_mask(&self) -> PMU_LP_CPU_MASK_R {
        PMU_LP_CPU_MASK_R::new(((self.bits >> 29) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPCORE_RESET_CAUSE")
            .field("lpcore_reset_flag", &self.lpcore_reset_flag())
            .field("lpcore_reset_cause", &self.lpcore_reset_cause())
            .field("pmu_lp_cpu_mask", &self.pmu_lp_cpu_mask())
            .finish()
    }
}
impl W {
    #[doc = "Bit 29 - 1'b0: enable lpcore pmu_lp_cpu_reset reset_cause, 1'b1: disable lpcore pmu_lp_cpu_reset reset_cause"]
    #[inline(always)]
    pub fn pmu_lp_cpu_mask(&mut self) -> PMU_LP_CPU_MASK_W<'_, LPCORE_RESET_CAUSE_SPEC> {
        PMU_LP_CPU_MASK_W::new(self, 29)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn clr(&mut self) -> CLR_W<'_, LPCORE_RESET_CAUSE_SPEC> {
        CLR_W::new(self, 30)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn lpcore_reset_flag_clr(
        &mut self,
    ) -> LPCORE_RESET_FLAG_CLR_W<'_, LPCORE_RESET_CAUSE_SPEC> {
        LPCORE_RESET_FLAG_CLR_W::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lpcore_reset_cause::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpcore_reset_cause::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LPCORE_RESET_CAUSE_SPEC;
impl crate::RegisterSpec for LPCORE_RESET_CAUSE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lpcore_reset_cause::R`](R) reader structure"]
impl crate::Readable for LPCORE_RESET_CAUSE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lpcore_reset_cause::W`](W) writer structure"]
impl crate::Writable for LPCORE_RESET_CAUSE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LPCORE_RESET_CAUSE to value 0x2000_0000"]
impl crate::Resettable for LPCORE_RESET_CAUSE_SPEC {
    const RESET_VALUE: u32 = 0x2000_0000;
}
