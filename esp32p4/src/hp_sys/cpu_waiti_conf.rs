#[doc = "Register `CPU_WAITI_CONF` reader"]
pub type R = crate::R<CPU_WAITI_CONF_SPEC>;
#[doc = "Register `CPU_WAITI_CONF` writer"]
pub type W = crate::W<CPU_WAITI_CONF_SPEC>;
#[doc = "Field `CPU_WAIT_MODE_FORCE_ON` reader - Set 1 to force cpu_waiti_clk enable."]
pub type CPU_WAIT_MODE_FORCE_ON_R = crate::BitReader;
#[doc = "Field `CPU_WAIT_MODE_FORCE_ON` writer - Set 1 to force cpu_waiti_clk enable."]
pub type CPU_WAIT_MODE_FORCE_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPU_WAITI_DELAY_NUM` reader - This field used to set delay cycle when cpu enter waiti mode, after delay waiti_clk will close"]
pub type CPU_WAITI_DELAY_NUM_R = crate::FieldReader;
#[doc = "Field `CPU_WAITI_DELAY_NUM` writer - This field used to set delay cycle when cpu enter waiti mode, after delay waiti_clk will close"]
pub type CPU_WAITI_DELAY_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - Set 1 to force cpu_waiti_clk enable."]
    #[inline(always)]
    pub fn cpu_wait_mode_force_on(&self) -> CPU_WAIT_MODE_FORCE_ON_R {
        CPU_WAIT_MODE_FORCE_ON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:4 - This field used to set delay cycle when cpu enter waiti mode, after delay waiti_clk will close"]
    #[inline(always)]
    pub fn cpu_waiti_delay_num(&self) -> CPU_WAITI_DELAY_NUM_R {
        CPU_WAITI_DELAY_NUM_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPU_WAITI_CONF")
            .field("cpu_wait_mode_force_on", &self.cpu_wait_mode_force_on())
            .field("cpu_waiti_delay_num", &self.cpu_waiti_delay_num())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Set 1 to force cpu_waiti_clk enable."]
    #[inline(always)]
    pub fn cpu_wait_mode_force_on(&mut self) -> CPU_WAIT_MODE_FORCE_ON_W<CPU_WAITI_CONF_SPEC> {
        CPU_WAIT_MODE_FORCE_ON_W::new(self, 0)
    }
    #[doc = "Bits 1:4 - This field used to set delay cycle when cpu enter waiti mode, after delay waiti_clk will close"]
    #[inline(always)]
    pub fn cpu_waiti_delay_num(&mut self) -> CPU_WAITI_DELAY_NUM_W<CPU_WAITI_CONF_SPEC> {
        CPU_WAITI_DELAY_NUM_W::new(self, 1)
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
#[doc = "`reset()` method sets CPU_WAITI_CONF to value 0x01"]
impl crate::Resettable for CPU_WAITI_CONF_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
