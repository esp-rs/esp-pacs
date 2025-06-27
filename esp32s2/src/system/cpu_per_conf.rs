#[doc = "Register `CPU_PER_CONF` reader"]
pub type R = crate::R<CPU_PER_CONF_SPEC>;
#[doc = "Register `CPU_PER_CONF` writer"]
pub type W = crate::W<CPU_PER_CONF_SPEC>;
#[doc = "Field `CPUPERIOD_SEL` reader - This field is used to select the clock frequency of CPU or CPU period."]
pub type CPUPERIOD_SEL_R = crate::FieldReader;
#[doc = "Field `CPUPERIOD_SEL` writer - This field is used to select the clock frequency of CPU or CPU period."]
pub type CPUPERIOD_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PLL_FREQ_SEL` reader - This field is used to select the PLL clock frequency based on CPU period."]
pub type PLL_FREQ_SEL_R = crate::BitReader;
#[doc = "Field `PLL_FREQ_SEL` writer - This field is used to select the PLL clock frequency based on CPU period."]
pub type PLL_FREQ_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPU_WAIT_MODE_FORCE_ON` reader - Set this bit to force on CPU wait mode. In this mode, the clock gate of CPU is turned off until any interrupts happen. This mode could also be force on via WAITI instruction."]
pub type CPU_WAIT_MODE_FORCE_ON_R = crate::BitReader;
#[doc = "Field `CPU_WAIT_MODE_FORCE_ON` writer - Set this bit to force on CPU wait mode. In this mode, the clock gate of CPU is turned off until any interrupts happen. This mode could also be force on via WAITI instruction."]
pub type CPU_WAIT_MODE_FORCE_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPU_WAITI_DELAY_NUM` reader - Sets the number of delay cycles to enter CPU wait mode after a WAITI instruction."]
pub type CPU_WAITI_DELAY_NUM_R = crate::FieldReader;
#[doc = "Field `CPU_WAITI_DELAY_NUM` writer - Sets the number of delay cycles to enter CPU wait mode after a WAITI instruction."]
pub type CPU_WAITI_DELAY_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - This field is used to select the clock frequency of CPU or CPU period."]
    #[inline(always)]
    pub fn cpuperiod_sel(&self) -> CPUPERIOD_SEL_R {
        CPUPERIOD_SEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - This field is used to select the PLL clock frequency based on CPU period."]
    #[inline(always)]
    pub fn pll_freq_sel(&self) -> PLL_FREQ_SEL_R {
        PLL_FREQ_SEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set this bit to force on CPU wait mode. In this mode, the clock gate of CPU is turned off until any interrupts happen. This mode could also be force on via WAITI instruction."]
    #[inline(always)]
    pub fn cpu_wait_mode_force_on(&self) -> CPU_WAIT_MODE_FORCE_ON_R {
        CPU_WAIT_MODE_FORCE_ON_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Sets the number of delay cycles to enter CPU wait mode after a WAITI instruction."]
    #[inline(always)]
    pub fn cpu_waiti_delay_num(&self) -> CPU_WAITI_DELAY_NUM_R {
        CPU_WAITI_DELAY_NUM_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPU_PER_CONF")
            .field("cpuperiod_sel", &self.cpuperiod_sel())
            .field("pll_freq_sel", &self.pll_freq_sel())
            .field("cpu_wait_mode_force_on", &self.cpu_wait_mode_force_on())
            .field("cpu_waiti_delay_num", &self.cpu_waiti_delay_num())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - This field is used to select the clock frequency of CPU or CPU period."]
    #[inline(always)]
    pub fn cpuperiod_sel(&mut self) -> CPUPERIOD_SEL_W<CPU_PER_CONF_SPEC> {
        CPUPERIOD_SEL_W::new(self, 0)
    }
    #[doc = "Bit 2 - This field is used to select the PLL clock frequency based on CPU period."]
    #[inline(always)]
    pub fn pll_freq_sel(&mut self) -> PLL_FREQ_SEL_W<CPU_PER_CONF_SPEC> {
        PLL_FREQ_SEL_W::new(self, 2)
    }
    #[doc = "Bit 3 - Set this bit to force on CPU wait mode. In this mode, the clock gate of CPU is turned off until any interrupts happen. This mode could also be force on via WAITI instruction."]
    #[inline(always)]
    pub fn cpu_wait_mode_force_on(&mut self) -> CPU_WAIT_MODE_FORCE_ON_W<CPU_PER_CONF_SPEC> {
        CPU_WAIT_MODE_FORCE_ON_W::new(self, 3)
    }
    #[doc = "Bits 4:7 - Sets the number of delay cycles to enter CPU wait mode after a WAITI instruction."]
    #[inline(always)]
    pub fn cpu_waiti_delay_num(&mut self) -> CPU_WAITI_DELAY_NUM_W<CPU_PER_CONF_SPEC> {
        CPU_WAITI_DELAY_NUM_W::new(self, 4)
    }
}
#[doc = "CPU peripheral clock configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_per_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_per_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CPU_PER_CONF_SPEC;
impl crate::RegisterSpec for CPU_PER_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpu_per_conf::R`](R) reader structure"]
impl crate::Readable for CPU_PER_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cpu_per_conf::W`](W) writer structure"]
impl crate::Writable for CPU_PER_CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CPU_PER_CONF to value 0x0c"]
impl crate::Resettable for CPU_PER_CONF_SPEC {
    const RESET_VALUE: u32 = 0x0c;
}
