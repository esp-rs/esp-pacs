#[doc = "Register `LP_AONCLKRST_RESET_CAUSE` reader"]
pub type R = crate::R<LP_AONCLKRST_RESET_CAUSE_SPEC>;
#[doc = "Register `LP_AONCLKRST_RESET_CAUSE` writer"]
pub type W = crate::W<LP_AONCLKRST_RESET_CAUSE_SPEC>;
#[doc = "Field `LP_AONCLKRST_LPCORE_RESET_CAUSE` reader - 6'h1: POR reset 6'h9: PMU LP PERI power down reset 6'ha: PMU LP CPU reset 6'hf: brown out reset 6'h10: LP watchdog chip reset 6'h12: super watch dog reset 6'h13: glitch reset 6'h14: software reset"]
pub type LP_AONCLKRST_LPCORE_RESET_CAUSE_R = crate::FieldReader;
#[doc = "Field `LP_AONCLKRST_LPCORE_RESET_FLAG` reader - need_des"]
pub type LP_AONCLKRST_LPCORE_RESET_FLAG_R = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_HPCORE0_RESET_CAUSE` reader - 6'h1: POR reset 6'h3: digital system software reset 6'h5: PMU HP system power down reset 6'h7: HP system reset from HP watchdog 6'h9: HP system reset from LP watchdog 6'hb: HP core reset from HP watchdog 6'hc: HP core software reset 6'hd: HP core reset from LP watchdog 6'hf: brown out reset 6'h10: LP watchdog chip reset 6'h12: super watch dog reset 6'h13: glitch reset 6'h14: efuse crc error reset 6'h16: HP usb jtag chip reset 6'h17: HP usb uart chip reset 6'h18: HP jtag reset 6'h1a: HP core lockup"]
pub type LP_AONCLKRST_HPCORE0_RESET_CAUSE_R = crate::FieldReader;
#[doc = "Field `LP_AONCLKRST_HPCORE0_RESET_FLAG` reader - need_des"]
pub type LP_AONCLKRST_HPCORE0_RESET_FLAG_R = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_HPCORE1_RESET_CAUSE` reader - 6'h1: POR reset 6'h3: digital system software reset 6'h5: PMU HP system power down reset 6'h7: HP system reset from HP watchdog 6'h9: HP system reset from LP watchdog 6'hb: HP core reset from HP watchdog 6'hc: HP core software reset 6'hd: HP core reset from LP watchdog 6'hf: brown out reset 6'h10: LP watchdog chip reset 6'h12: super watch dog reset 6'h13: glitch reset 6'h14: efuse crc error reset 6'h16: HP usb jtag chip reset 6'h17: HP usb uart chip reset 6'h18: HP jtag reset 6'h1a: HP core lockup"]
pub type LP_AONCLKRST_HPCORE1_RESET_CAUSE_R = crate::FieldReader;
#[doc = "Field `LP_AONCLKRST_HPCORE1_RESET_FLAG` reader - need_des"]
pub type LP_AONCLKRST_HPCORE1_RESET_FLAG_R = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_LPCORE_RESET_CAUSE_PMU_LP_CPU_MASK` reader - 1'b0: enable lpcore pmu_lp_cpu_reset reset_cause, 1'b1: disable lpcore pmu_lp_cpu_reset reset_cause"]
pub type LP_AONCLKRST_LPCORE_RESET_CAUSE_PMU_LP_CPU_MASK_R = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_LPCORE_RESET_CAUSE_PMU_LP_CPU_MASK` writer - 1'b0: enable lpcore pmu_lp_cpu_reset reset_cause, 1'b1: disable lpcore pmu_lp_cpu_reset reset_cause"]
pub type LP_AONCLKRST_LPCORE_RESET_CAUSE_PMU_LP_CPU_MASK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_AONCLKRST_LPCORE_RESET_CAUSE_CLR` writer - need_des"]
pub type LP_AONCLKRST_LPCORE_RESET_CAUSE_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_AONCLKRST_LPCORE_RESET_FLAG_CLR` writer - need_des"]
pub type LP_AONCLKRST_LPCORE_RESET_FLAG_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_AONCLKRST_HPCORE0_RESET_CAUSE_CLR` writer - need_des"]
pub type LP_AONCLKRST_HPCORE0_RESET_CAUSE_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_AONCLKRST_HPCORE0_RESET_FLAG_CLR` writer - need_des"]
pub type LP_AONCLKRST_HPCORE0_RESET_FLAG_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_AONCLKRST_HPCORE1_RESET_CAUSE_CLR` writer - need_des"]
pub type LP_AONCLKRST_HPCORE1_RESET_CAUSE_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_AONCLKRST_HPCORE1_RESET_FLAG_CLR` writer - need_des"]
pub type LP_AONCLKRST_HPCORE1_RESET_FLAG_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - 6'h1: POR reset 6'h9: PMU LP PERI power down reset 6'ha: PMU LP CPU reset 6'hf: brown out reset 6'h10: LP watchdog chip reset 6'h12: super watch dog reset 6'h13: glitch reset 6'h14: software reset"]
    #[inline(always)]
    pub fn lp_aonclkrst_lpcore_reset_cause(&self) -> LP_AONCLKRST_LPCORE_RESET_CAUSE_R {
        LP_AONCLKRST_LPCORE_RESET_CAUSE_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_lpcore_reset_flag(&self) -> LP_AONCLKRST_LPCORE_RESET_FLAG_R {
        LP_AONCLKRST_LPCORE_RESET_FLAG_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:12 - 6'h1: POR reset 6'h3: digital system software reset 6'h5: PMU HP system power down reset 6'h7: HP system reset from HP watchdog 6'h9: HP system reset from LP watchdog 6'hb: HP core reset from HP watchdog 6'hc: HP core software reset 6'hd: HP core reset from LP watchdog 6'hf: brown out reset 6'h10: LP watchdog chip reset 6'h12: super watch dog reset 6'h13: glitch reset 6'h14: efuse crc error reset 6'h16: HP usb jtag chip reset 6'h17: HP usb uart chip reset 6'h18: HP jtag reset 6'h1a: HP core lockup"]
    #[inline(always)]
    pub fn lp_aonclkrst_hpcore0_reset_cause(&self) -> LP_AONCLKRST_HPCORE0_RESET_CAUSE_R {
        LP_AONCLKRST_HPCORE0_RESET_CAUSE_R::new(((self.bits >> 7) & 0x3f) as u8)
    }
    #[doc = "Bit 13 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_hpcore0_reset_flag(&self) -> LP_AONCLKRST_HPCORE0_RESET_FLAG_R {
        LP_AONCLKRST_HPCORE0_RESET_FLAG_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:19 - 6'h1: POR reset 6'h3: digital system software reset 6'h5: PMU HP system power down reset 6'h7: HP system reset from HP watchdog 6'h9: HP system reset from LP watchdog 6'hb: HP core reset from HP watchdog 6'hc: HP core software reset 6'hd: HP core reset from LP watchdog 6'hf: brown out reset 6'h10: LP watchdog chip reset 6'h12: super watch dog reset 6'h13: glitch reset 6'h14: efuse crc error reset 6'h16: HP usb jtag chip reset 6'h17: HP usb uart chip reset 6'h18: HP jtag reset 6'h1a: HP core lockup"]
    #[inline(always)]
    pub fn lp_aonclkrst_hpcore1_reset_cause(&self) -> LP_AONCLKRST_HPCORE1_RESET_CAUSE_R {
        LP_AONCLKRST_HPCORE1_RESET_CAUSE_R::new(((self.bits >> 14) & 0x3f) as u8)
    }
    #[doc = "Bit 20 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_hpcore1_reset_flag(&self) -> LP_AONCLKRST_HPCORE1_RESET_FLAG_R {
        LP_AONCLKRST_HPCORE1_RESET_FLAG_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 25 - 1'b0: enable lpcore pmu_lp_cpu_reset reset_cause, 1'b1: disable lpcore pmu_lp_cpu_reset reset_cause"]
    #[inline(always)]
    pub fn lp_aonclkrst_lpcore_reset_cause_pmu_lp_cpu_mask(
        &self,
    ) -> LP_AONCLKRST_LPCORE_RESET_CAUSE_PMU_LP_CPU_MASK_R {
        LP_AONCLKRST_LPCORE_RESET_CAUSE_PMU_LP_CPU_MASK_R::new(((self.bits >> 25) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_AONCLKRST_RESET_CAUSE")
            .field(
                "lp_aonclkrst_lpcore_reset_cause",
                &self.lp_aonclkrst_lpcore_reset_cause(),
            )
            .field(
                "lp_aonclkrst_lpcore_reset_flag",
                &self.lp_aonclkrst_lpcore_reset_flag(),
            )
            .field(
                "lp_aonclkrst_hpcore0_reset_cause",
                &self.lp_aonclkrst_hpcore0_reset_cause(),
            )
            .field(
                "lp_aonclkrst_hpcore0_reset_flag",
                &self.lp_aonclkrst_hpcore0_reset_flag(),
            )
            .field(
                "lp_aonclkrst_hpcore1_reset_cause",
                &self.lp_aonclkrst_hpcore1_reset_cause(),
            )
            .field(
                "lp_aonclkrst_hpcore1_reset_flag",
                &self.lp_aonclkrst_hpcore1_reset_flag(),
            )
            .field(
                "lp_aonclkrst_lpcore_reset_cause_pmu_lp_cpu_mask",
                &self.lp_aonclkrst_lpcore_reset_cause_pmu_lp_cpu_mask(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 25 - 1'b0: enable lpcore pmu_lp_cpu_reset reset_cause, 1'b1: disable lpcore pmu_lp_cpu_reset reset_cause"]
    #[inline(always)]
    pub fn lp_aonclkrst_lpcore_reset_cause_pmu_lp_cpu_mask(
        &mut self,
    ) -> LP_AONCLKRST_LPCORE_RESET_CAUSE_PMU_LP_CPU_MASK_W<LP_AONCLKRST_RESET_CAUSE_SPEC> {
        LP_AONCLKRST_LPCORE_RESET_CAUSE_PMU_LP_CPU_MASK_W::new(self, 25)
    }
    #[doc = "Bit 26 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_lpcore_reset_cause_clr(
        &mut self,
    ) -> LP_AONCLKRST_LPCORE_RESET_CAUSE_CLR_W<LP_AONCLKRST_RESET_CAUSE_SPEC> {
        LP_AONCLKRST_LPCORE_RESET_CAUSE_CLR_W::new(self, 26)
    }
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_lpcore_reset_flag_clr(
        &mut self,
    ) -> LP_AONCLKRST_LPCORE_RESET_FLAG_CLR_W<LP_AONCLKRST_RESET_CAUSE_SPEC> {
        LP_AONCLKRST_LPCORE_RESET_FLAG_CLR_W::new(self, 27)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_hpcore0_reset_cause_clr(
        &mut self,
    ) -> LP_AONCLKRST_HPCORE0_RESET_CAUSE_CLR_W<LP_AONCLKRST_RESET_CAUSE_SPEC> {
        LP_AONCLKRST_HPCORE0_RESET_CAUSE_CLR_W::new(self, 28)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_hpcore0_reset_flag_clr(
        &mut self,
    ) -> LP_AONCLKRST_HPCORE0_RESET_FLAG_CLR_W<LP_AONCLKRST_RESET_CAUSE_SPEC> {
        LP_AONCLKRST_HPCORE0_RESET_FLAG_CLR_W::new(self, 29)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_hpcore1_reset_cause_clr(
        &mut self,
    ) -> LP_AONCLKRST_HPCORE1_RESET_CAUSE_CLR_W<LP_AONCLKRST_RESET_CAUSE_SPEC> {
        LP_AONCLKRST_HPCORE1_RESET_CAUSE_CLR_W::new(self, 30)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_hpcore1_reset_flag_clr(
        &mut self,
    ) -> LP_AONCLKRST_HPCORE1_RESET_FLAG_CLR_W<LP_AONCLKRST_RESET_CAUSE_SPEC> {
        LP_AONCLKRST_HPCORE1_RESET_FLAG_CLR_W::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_aonclkrst_reset_cause::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_aonclkrst_reset_cause::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LP_AONCLKRST_RESET_CAUSE_SPEC;
impl crate::RegisterSpec for LP_AONCLKRST_RESET_CAUSE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_aonclkrst_reset_cause::R`](R) reader structure"]
impl crate::Readable for LP_AONCLKRST_RESET_CAUSE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lp_aonclkrst_reset_cause::W`](W) writer structure"]
impl crate::Writable for LP_AONCLKRST_RESET_CAUSE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LP_AONCLKRST_RESET_CAUSE to value 0x0200_0000"]
impl crate::Resettable for LP_AONCLKRST_RESET_CAUSE_SPEC {
    const RESET_VALUE: u32 = 0x0200_0000;
}
