#[doc = "Register `CPU_WAITI_CONF` reader"]
pub type R = crate::R<CPU_WAITI_CONF_SPEC>;
#[doc = "Register `CPU_WAITI_CONF` writer"]
pub type W = crate::W<CPU_WAITI_CONF_SPEC>;
#[doc = "Field `CPUPERIOD_SEL` reader - Reserved. This filed has been replaced by PCR_CPU_DIV_NUM"]
pub type CPUPERIOD_SEL_R = crate::FieldReader;
#[doc = "Field `PLL_FREQ_SEL` reader - Reserved. This filed has been replaced by PCR_CPU_DIV_NUM"]
pub type PLL_FREQ_SEL_R = crate::BitReader;
#[doc = "Field `CPU1_WAIT_MODE_FORCE_ON` reader - Set 1 to force cpu1_waiti_clk enable."]
pub type CPU1_WAIT_MODE_FORCE_ON_R = crate::BitReader;
#[doc = "Field `CPU1_WAIT_MODE_FORCE_ON` writer - Set 1 to force cpu1_waiti_clk enable."]
pub type CPU1_WAIT_MODE_FORCE_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPU0_WAIT_MODE_FORCE_ON` reader - Set 1 to force cpu0_waiti_clk enable."]
pub type CPU0_WAIT_MODE_FORCE_ON_R = crate::BitReader;
#[doc = "Field `CPU0_WAIT_MODE_FORCE_ON` writer - Set 1 to force cpu0_waiti_clk enable."]
pub type CPU0_WAIT_MODE_FORCE_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPU_WAITI_DELAY_NUM` reader - This field used to set delay cycle when cpu enter waiti mode, after delay waiti_clk will close"]
pub type CPU_WAITI_DELAY_NUM_R = crate::FieldReader;
#[doc = "Field `CPU_WAITI_DELAY_NUM` writer - This field used to set delay cycle when cpu enter waiti mode, after delay waiti_clk will close"]
pub type CPU_WAITI_DELAY_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CPU_WFI_DECREASE_EN` reader - Set 1 to enable cpu freq decrease to ahb freq when cpu enter waiti mode"]
pub type CPU_WFI_DECREASE_EN_R = crate::BitReader;
#[doc = "Field `CPU_WFI_DECREASE_EN` writer - Set 1 to enable cpu freq decrease to ahb freq when cpu enter waiti mode"]
pub type CPU_WFI_DECREASE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Reserved. This filed has been replaced by PCR_CPU_DIV_NUM"]
    #[inline(always)]
    pub fn cpuperiod_sel(&self) -> CPUPERIOD_SEL_R {
        CPUPERIOD_SEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Reserved. This filed has been replaced by PCR_CPU_DIV_NUM"]
    #[inline(always)]
    pub fn pll_freq_sel(&self) -> PLL_FREQ_SEL_R {
        PLL_FREQ_SEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set 1 to force cpu1_waiti_clk enable."]
    #[inline(always)]
    pub fn cpu1_wait_mode_force_on(&self) -> CPU1_WAIT_MODE_FORCE_ON_R {
        CPU1_WAIT_MODE_FORCE_ON_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Set 1 to force cpu0_waiti_clk enable."]
    #[inline(always)]
    pub fn cpu0_wait_mode_force_on(&self) -> CPU0_WAIT_MODE_FORCE_ON_R {
        CPU0_WAIT_MODE_FORCE_ON_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:8 - This field used to set delay cycle when cpu enter waiti mode, after delay waiti_clk will close"]
    #[inline(always)]
    pub fn cpu_waiti_delay_num(&self) -> CPU_WAITI_DELAY_NUM_R {
        CPU_WAITI_DELAY_NUM_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    #[doc = "Bit 9 - Set 1 to enable cpu freq decrease to ahb freq when cpu enter waiti mode"]
    #[inline(always)]
    pub fn cpu_wfi_decrease_en(&self) -> CPU_WFI_DECREASE_EN_R {
        CPU_WFI_DECREASE_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPU_WAITI_CONF")
            .field("cpuperiod_sel", &self.cpuperiod_sel())
            .field("pll_freq_sel", &self.pll_freq_sel())
            .field("cpu1_wait_mode_force_on", &self.cpu1_wait_mode_force_on())
            .field("cpu0_wait_mode_force_on", &self.cpu0_wait_mode_force_on())
            .field("cpu_waiti_delay_num", &self.cpu_waiti_delay_num())
            .field("cpu_wfi_decrease_en", &self.cpu_wfi_decrease_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 3 - Set 1 to force cpu1_waiti_clk enable."]
    #[inline(always)]
    pub fn cpu1_wait_mode_force_on(
        &mut self,
    ) -> CPU1_WAIT_MODE_FORCE_ON_W<'_, CPU_WAITI_CONF_SPEC> {
        CPU1_WAIT_MODE_FORCE_ON_W::new(self, 3)
    }
    #[doc = "Bit 4 - Set 1 to force cpu0_waiti_clk enable."]
    #[inline(always)]
    pub fn cpu0_wait_mode_force_on(
        &mut self,
    ) -> CPU0_WAIT_MODE_FORCE_ON_W<'_, CPU_WAITI_CONF_SPEC> {
        CPU0_WAIT_MODE_FORCE_ON_W::new(self, 4)
    }
    #[doc = "Bits 5:8 - This field used to set delay cycle when cpu enter waiti mode, after delay waiti_clk will close"]
    #[inline(always)]
    pub fn cpu_waiti_delay_num(&mut self) -> CPU_WAITI_DELAY_NUM_W<'_, CPU_WAITI_CONF_SPEC> {
        CPU_WAITI_DELAY_NUM_W::new(self, 5)
    }
    #[doc = "Bit 9 - Set 1 to enable cpu freq decrease to ahb freq when cpu enter waiti mode"]
    #[inline(always)]
    pub fn cpu_wfi_decrease_en(&mut self) -> CPU_WFI_DECREASE_EN_W<'_, CPU_WAITI_CONF_SPEC> {
        CPU_WFI_DECREASE_EN_W::new(self, 9)
    }
}
#[doc = "CPU_WAITI configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_waiti_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_waiti_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CPU_WAITI_CONF_SPEC;
impl crate::RegisterSpec for CPU_WAITI_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpu_waiti_conf::R`](R) reader structure"]
impl crate::Readable for CPU_WAITI_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cpu_waiti_conf::W`](W) writer structure"]
impl crate::Writable for CPU_WAITI_CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CPU_WAITI_CONF to value 0x1d"]
impl crate::Resettable for CPU_WAITI_CONF_SPEC {
    const RESET_VALUE: u32 = 0x1d;
}
