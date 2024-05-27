#[doc = "Register `CPU_PER_CONF` reader"]
pub type R = crate::R<CPU_PER_CONF_SPEC>;
#[doc = "Register `CPU_PER_CONF` writer"]
pub type W = crate::W<CPU_PER_CONF_SPEC>;
#[doc = "Field `CPUPERIOD_SEL` reader - "]
pub type CPUPERIOD_SEL_R = crate::FieldReader;
#[doc = "Field `CPUPERIOD_SEL` writer - "]
pub type CPUPERIOD_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LOWSPEED_CLK_SEL` reader - "]
pub type LOWSPEED_CLK_SEL_R = crate::BitReader;
#[doc = "Field `LOWSPEED_CLK_SEL` writer - "]
pub type LOWSPEED_CLK_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FAST_CLK_RTC_SEL` reader - "]
pub type FAST_CLK_RTC_SEL_R = crate::BitReader;
#[doc = "Field `FAST_CLK_RTC_SEL` writer - "]
pub type FAST_CLK_RTC_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn cpuperiod_sel(&self) -> CPUPERIOD_SEL_R {
        CPUPERIOD_SEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn lowspeed_clk_sel(&self) -> LOWSPEED_CLK_SEL_R {
        LOWSPEED_CLK_SEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn fast_clk_rtc_sel(&self) -> FAST_CLK_RTC_SEL_R {
        FAST_CLK_RTC_SEL_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPU_PER_CONF")
            .field("cpuperiod_sel", &self.cpuperiod_sel())
            .field("lowspeed_clk_sel", &self.lowspeed_clk_sel())
            .field("fast_clk_rtc_sel", &self.fast_clk_rtc_sel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn cpuperiod_sel(&mut self) -> CPUPERIOD_SEL_W<CPU_PER_CONF_SPEC> {
        CPUPERIOD_SEL_W::new(self, 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn lowspeed_clk_sel(&mut self) -> LOWSPEED_CLK_SEL_W<CPU_PER_CONF_SPEC> {
        LOWSPEED_CLK_SEL_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn fast_clk_rtc_sel(&mut self) -> FAST_CLK_RTC_SEL_W<CPU_PER_CONF_SPEC> {
        FAST_CLK_RTC_SEL_W::new(self, 3)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_per_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_per_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CPU_PER_CONF_SPEC;
impl crate::RegisterSpec for CPU_PER_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpu_per_conf::R`](R) reader structure"]
impl crate::Readable for CPU_PER_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cpu_per_conf::W`](W) writer structure"]
impl crate::Writable for CPU_PER_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPU_PER_CONF to value 0"]
impl crate::Resettable for CPU_PER_CONF_SPEC {
    const RESET_VALUE: u32 = 0;
}
