///Register `CPU_PERIOD_CONF` reader
pub type R = crate::R<CPU_PERIOD_CONF_SPEC>;
///Register `CPU_PERIOD_CONF` writer
pub type W = crate::W<CPU_PERIOD_CONF_SPEC>;
///Field `CPUSEL_CONF` reader - CPU sel option
pub type CPUSEL_CONF_R = crate::BitReader;
///Field `CPUSEL_CONF` writer - CPU sel option
pub type CPUSEL_CONF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CPUPERIOD_SEL` reader - CPU clk sel option
pub type CPUPERIOD_SEL_R = crate::FieldReader;
///Field `CPUPERIOD_SEL` writer - CPU clk sel option
pub type CPUPERIOD_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bit 29 - CPU sel option
    #[inline(always)]
    pub fn cpusel_conf(&self) -> CPUSEL_CONF_R {
        CPUSEL_CONF_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bits 30:31 - CPU clk sel option
    #[inline(always)]
    pub fn cpuperiod_sel(&self) -> CPUPERIOD_SEL_R {
        CPUPERIOD_SEL_R::new(((self.bits >> 30) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPU_PERIOD_CONF")
            .field("cpusel_conf", &self.cpusel_conf())
            .field("cpuperiod_sel", &self.cpuperiod_sel())
            .finish()
    }
}
impl W {
    ///Bit 29 - CPU sel option
    #[inline(always)]
    #[must_use]
    pub fn cpusel_conf(&mut self) -> CPUSEL_CONF_W<CPU_PERIOD_CONF_SPEC> {
        CPUSEL_CONF_W::new(self, 29)
    }
    ///Bits 30:31 - CPU clk sel option
    #[inline(always)]
    #[must_use]
    pub fn cpuperiod_sel(&mut self) -> CPUPERIOD_SEL_W<CPU_PERIOD_CONF_SPEC> {
        CPUPERIOD_SEL_W::new(self, 30)
    }
}
/**rtc configure register

You can [`read`](crate::generic::Reg::read) this register and get [`cpu_period_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_period_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CPU_PERIOD_CONF_SPEC;
impl crate::RegisterSpec for CPU_PERIOD_CONF_SPEC {
    type Ux = u32;
}
///`read()` method returns [`cpu_period_conf::R`](R) reader structure
impl crate::Readable for CPU_PERIOD_CONF_SPEC {}
///`write(|w| ..)` method takes [`cpu_period_conf::W`](W) writer structure
impl crate::Writable for CPU_PERIOD_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CPU_PERIOD_CONF to value 0
impl crate::Resettable for CPU_PERIOD_CONF_SPEC {
    const RESET_VALUE: u32 = 0;
}
