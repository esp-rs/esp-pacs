#[doc = "Register `CPU_WAITI_CONF` reader"]
pub type R = crate::R<CPU_WAITI_CONF_SPEC>;
#[doc = "Register `CPU_WAITI_CONF` writer"]
pub type W = crate::W<CPU_WAITI_CONF_SPEC>;
#[doc = "Field `CPUPERIOD_SEL` reader - Reserved. This filed has been replaced by PCR_CPU_DIV_NUM"]
pub type CPUPERIOD_SEL_R = crate::FieldReader;
#[doc = "Field `PLL_FREQ_SEL` reader - Reserved. This filed has been replaced by PCR_CPU_DIV_NUM"]
pub type PLL_FREQ_SEL_R = crate::BitReader;
#[doc = "Field `CPU_WAIT_MODE_FORCE_ON` reader - Set 1 to force cpu_waiti_clk enable."]
pub type CPU_WAIT_MODE_FORCE_ON_R = crate::BitReader;
#[doc = "Field `CPU_WAIT_MODE_FORCE_ON` writer - Set 1 to force cpu_waiti_clk enable."]
pub type CPU_WAIT_MODE_FORCE_ON_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CPU_WAITI_DELAY_NUM` reader - This field used to set delay cycle when cpu enter waiti mode, after delay waiti_clk will close"]
pub type CPU_WAITI_DELAY_NUM_R = crate::FieldReader;
#[doc = "Field `CPU_WAITI_DELAY_NUM` writer - This field used to set delay cycle when cpu enter waiti mode, after delay waiti_clk will close"]
pub type CPU_WAITI_DELAY_NUM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
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
    #[doc = "Bit 3 - Set 1 to force cpu_waiti_clk enable."]
    #[inline(always)]
    pub fn cpu_wait_mode_force_on(&self) -> CPU_WAIT_MODE_FORCE_ON_R {
        CPU_WAIT_MODE_FORCE_ON_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - This field used to set delay cycle when cpu enter waiti mode, after delay waiti_clk will close"]
    #[inline(always)]
    pub fn cpu_waiti_delay_num(&self) -> CPU_WAITI_DELAY_NUM_R {
        CPU_WAITI_DELAY_NUM_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPU_WAITI_CONF")
            .field(
                "cpuperiod_sel",
                &format_args!("{}", self.cpuperiod_sel().bits()),
            )
            .field(
                "pll_freq_sel",
                &format_args!("{}", self.pll_freq_sel().bit()),
            )
            .field(
                "cpu_wait_mode_force_on",
                &format_args!("{}", self.cpu_wait_mode_force_on().bit()),
            )
            .field(
                "cpu_waiti_delay_num",
                &format_args!("{}", self.cpu_waiti_delay_num().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CPU_WAITI_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 3 - Set 1 to force cpu_waiti_clk enable."]
    #[inline(always)]
    #[must_use]
    pub fn cpu_wait_mode_force_on(&mut self) -> CPU_WAIT_MODE_FORCE_ON_W<CPU_WAITI_CONF_SPEC, 3> {
        CPU_WAIT_MODE_FORCE_ON_W::new(self)
    }
    #[doc = "Bits 4:7 - This field used to set delay cycle when cpu enter waiti mode, after delay waiti_clk will close"]
    #[inline(always)]
    #[must_use]
    pub fn cpu_waiti_delay_num(&mut self) -> CPU_WAITI_DELAY_NUM_W<CPU_WAITI_CONF_SPEC, 4> {
        CPU_WAITI_DELAY_NUM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "CPU_WAITI configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_waiti_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_waiti_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CPU_WAITI_CONF_SPEC;
impl crate::RegisterSpec for CPU_WAITI_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpu_waiti_conf::R`](R) reader structure"]
impl crate::Readable for CPU_WAITI_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cpu_waiti_conf::W`](W) writer structure"]
impl crate::Writable for CPU_WAITI_CONF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CPU_WAITI_CONF to value 0x0d"]
impl crate::Resettable for CPU_WAITI_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x0d;
}
