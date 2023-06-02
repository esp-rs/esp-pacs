#[doc = "Register `CPU_PER_CONF` reader"]
pub struct R(crate::R<CPU_PER_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPU_PER_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPU_PER_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPU_PER_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CPU_PER_CONF` writer"]
pub struct W(crate::W<CPU_PER_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPU_PER_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CPU_PER_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CPU_PER_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CPUPERIOD_SEL` reader - reg_cpuperiod_sel"]
pub type CPUPERIOD_SEL_R = crate::FieldReader;
#[doc = "Field `CPUPERIOD_SEL` writer - reg_cpuperiod_sel"]
pub type CPUPERIOD_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, CPU_PER_CONF_SPEC, 2, O>;
#[doc = "Field `PLL_FREQ_SEL` reader - reg_pll_freq_sel"]
pub type PLL_FREQ_SEL_R = crate::BitReader;
#[doc = "Field `PLL_FREQ_SEL` writer - reg_pll_freq_sel"]
pub type PLL_FREQ_SEL_W<'a, const O: u8> = crate::BitWriter<'a, CPU_PER_CONF_SPEC, O>;
#[doc = "Field `CPU_WAIT_MODE_FORCE_ON` reader - reg_cpu_wait_mode_force_on"]
pub type CPU_WAIT_MODE_FORCE_ON_R = crate::BitReader;
#[doc = "Field `CPU_WAIT_MODE_FORCE_ON` writer - reg_cpu_wait_mode_force_on"]
pub type CPU_WAIT_MODE_FORCE_ON_W<'a, const O: u8> = crate::BitWriter<'a, CPU_PER_CONF_SPEC, O>;
#[doc = "Field `CPU_WAITI_DELAY_NUM` reader - reg_cpu_waiti_delay_num"]
pub type CPU_WAITI_DELAY_NUM_R = crate::FieldReader;
#[doc = "Field `CPU_WAITI_DELAY_NUM` writer - reg_cpu_waiti_delay_num"]
pub type CPU_WAITI_DELAY_NUM_W<'a, const O: u8> = crate::FieldWriter<'a, CPU_PER_CONF_SPEC, 4, O>;
impl R {
    #[doc = "Bits 0:1 - reg_cpuperiod_sel"]
    #[inline(always)]
    pub fn cpuperiod_sel(&self) -> CPUPERIOD_SEL_R {
        CPUPERIOD_SEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - reg_pll_freq_sel"]
    #[inline(always)]
    pub fn pll_freq_sel(&self) -> PLL_FREQ_SEL_R {
        PLL_FREQ_SEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reg_cpu_wait_mode_force_on"]
    #[inline(always)]
    pub fn cpu_wait_mode_force_on(&self) -> CPU_WAIT_MODE_FORCE_ON_R {
        CPU_WAIT_MODE_FORCE_ON_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - reg_cpu_waiti_delay_num"]
    #[inline(always)]
    pub fn cpu_waiti_delay_num(&self) -> CPU_WAITI_DELAY_NUM_R {
        CPU_WAITI_DELAY_NUM_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPU_PER_CONF")
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
impl core::fmt::Debug for crate::generic::Reg<CPU_PER_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:1 - reg_cpuperiod_sel"]
    #[inline(always)]
    #[must_use]
    pub fn cpuperiod_sel(&mut self) -> CPUPERIOD_SEL_W<0> {
        CPUPERIOD_SEL_W::new(self)
    }
    #[doc = "Bit 2 - reg_pll_freq_sel"]
    #[inline(always)]
    #[must_use]
    pub fn pll_freq_sel(&mut self) -> PLL_FREQ_SEL_W<2> {
        PLL_FREQ_SEL_W::new(self)
    }
    #[doc = "Bit 3 - reg_cpu_wait_mode_force_on"]
    #[inline(always)]
    #[must_use]
    pub fn cpu_wait_mode_force_on(&mut self) -> CPU_WAIT_MODE_FORCE_ON_W<3> {
        CPU_WAIT_MODE_FORCE_ON_W::new(self)
    }
    #[doc = "Bits 4:7 - reg_cpu_waiti_delay_num"]
    #[inline(always)]
    #[must_use]
    pub fn cpu_waiti_delay_num(&mut self) -> CPU_WAITI_DELAY_NUM_W<4> {
        CPU_WAITI_DELAY_NUM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "cpu clock config register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpu_per_conf](index.html) module"]
pub struct CPU_PER_CONF_SPEC;
impl crate::RegisterSpec for CPU_PER_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpu_per_conf::R](R) reader structure"]
impl crate::Readable for CPU_PER_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cpu_per_conf::W](W) writer structure"]
impl crate::Writable for CPU_PER_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CPU_PER_CONF to value 0x0c"]
impl crate::Resettable for CPU_PER_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x0c;
}
