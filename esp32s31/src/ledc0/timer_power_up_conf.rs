#[doc = "Register `TIMER_POWER_UP_CONF` reader"]
pub type R = crate::R<TIMER_POWER_UP_CONF_SPEC>;
#[doc = "Register `TIMER_POWER_UP_CONF` writer"]
pub type W = crate::W<TIMER_POWER_UP_CONF_SPEC>;
#[doc = "Field `TIMER0_POWER_UP` reader - Configures whether or not to power up timer0.\\\\0: power down\\\\1: power up"]
pub type TIMER0_POWER_UP_R = crate::BitReader;
#[doc = "Field `TIMER0_POWER_UP` writer - Configures whether or not to power up timer0.\\\\0: power down\\\\1: power up"]
pub type TIMER0_POWER_UP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER1_POWER_UP` reader - Configures whether or not to power up timer1.\\\\0: power down\\\\1: power up"]
pub type TIMER1_POWER_UP_R = crate::BitReader;
#[doc = "Field `TIMER1_POWER_UP` writer - Configures whether or not to power up timer1.\\\\0: power down\\\\1: power up"]
pub type TIMER1_POWER_UP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER2_POWER_UP` reader - Configures whether or not to power up timer2.\\\\0: power down\\\\1: power up"]
pub type TIMER2_POWER_UP_R = crate::BitReader;
#[doc = "Field `TIMER2_POWER_UP` writer - Configures whether or not to power up timer2.\\\\0: power down\\\\1: power up"]
pub type TIMER2_POWER_UP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER3_POWER_UP` reader - Configures whether or not to power up timer3.\\\\0: power down\\\\1: power up"]
pub type TIMER3_POWER_UP_R = crate::BitReader;
#[doc = "Field `TIMER3_POWER_UP` writer - Configures whether or not to power up timer3.\\\\0: power down\\\\1: power up"]
pub type TIMER3_POWER_UP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER0_CLK_GATE_FORCE_ON` reader - Configures whether or not to open timer0 clock gate.\\\\0: Open the clock gate only when timer0 is power up\\\\1: Force open the clock gate for timer0"]
pub type TIMER0_CLK_GATE_FORCE_ON_R = crate::BitReader;
#[doc = "Field `TIMER0_CLK_GATE_FORCE_ON` writer - Configures whether or not to open timer0 clock gate.\\\\0: Open the clock gate only when timer0 is power up\\\\1: Force open the clock gate for timer0"]
pub type TIMER0_CLK_GATE_FORCE_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER1_CLK_GATE_FORCE_ON` reader - Configures whether or not to open timer1 clock gate.\\\\0: Open the clock gate only when timer1 is power up\\\\1: Force open the clock gate for timer1"]
pub type TIMER1_CLK_GATE_FORCE_ON_R = crate::BitReader;
#[doc = "Field `TIMER1_CLK_GATE_FORCE_ON` writer - Configures whether or not to open timer1 clock gate.\\\\0: Open the clock gate only when timer1 is power up\\\\1: Force open the clock gate for timer1"]
pub type TIMER1_CLK_GATE_FORCE_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER2_CLK_GATE_FORCE_ON` reader - Configures whether or not to open timer2 clock gate.\\\\0: Open the clock gate only when timer2 is power up\\\\1: Force open the clock gate for timer2"]
pub type TIMER2_CLK_GATE_FORCE_ON_R = crate::BitReader;
#[doc = "Field `TIMER2_CLK_GATE_FORCE_ON` writer - Configures whether or not to open timer2 clock gate.\\\\0: Open the clock gate only when timer2 is power up\\\\1: Force open the clock gate for timer2"]
pub type TIMER2_CLK_GATE_FORCE_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER3_CLK_GATE_FORCE_ON` reader - Configures whether or not to open timer3 clock gate.\\\\0: Open the clock gate only when timer3 is power up\\\\1: Force open the clock gate for timer3"]
pub type TIMER3_CLK_GATE_FORCE_ON_R = crate::BitReader;
#[doc = "Field `TIMER3_CLK_GATE_FORCE_ON` writer - Configures whether or not to open timer3 clock gate.\\\\0: Open the clock gate only when timer3 is power up\\\\1: Force open the clock gate for timer3"]
pub type TIMER3_CLK_GATE_FORCE_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Configures whether or not to power up timer0.\\\\0: power down\\\\1: power up"]
    #[inline(always)]
    pub fn timer0_power_up(&self) -> TIMER0_POWER_UP_R {
        TIMER0_POWER_UP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Configures whether or not to power up timer1.\\\\0: power down\\\\1: power up"]
    #[inline(always)]
    pub fn timer1_power_up(&self) -> TIMER1_POWER_UP_R {
        TIMER1_POWER_UP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Configures whether or not to power up timer2.\\\\0: power down\\\\1: power up"]
    #[inline(always)]
    pub fn timer2_power_up(&self) -> TIMER2_POWER_UP_R {
        TIMER2_POWER_UP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Configures whether or not to power up timer3.\\\\0: power down\\\\1: power up"]
    #[inline(always)]
    pub fn timer3_power_up(&self) -> TIMER3_POWER_UP_R {
        TIMER3_POWER_UP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Configures whether or not to open timer0 clock gate.\\\\0: Open the clock gate only when timer0 is power up\\\\1: Force open the clock gate for timer0"]
    #[inline(always)]
    pub fn timer0_clk_gate_force_on(&self) -> TIMER0_CLK_GATE_FORCE_ON_R {
        TIMER0_CLK_GATE_FORCE_ON_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Configures whether or not to open timer1 clock gate.\\\\0: Open the clock gate only when timer1 is power up\\\\1: Force open the clock gate for timer1"]
    #[inline(always)]
    pub fn timer1_clk_gate_force_on(&self) -> TIMER1_CLK_GATE_FORCE_ON_R {
        TIMER1_CLK_GATE_FORCE_ON_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Configures whether or not to open timer2 clock gate.\\\\0: Open the clock gate only when timer2 is power up\\\\1: Force open the clock gate for timer2"]
    #[inline(always)]
    pub fn timer2_clk_gate_force_on(&self) -> TIMER2_CLK_GATE_FORCE_ON_R {
        TIMER2_CLK_GATE_FORCE_ON_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Configures whether or not to open timer3 clock gate.\\\\0: Open the clock gate only when timer3 is power up\\\\1: Force open the clock gate for timer3"]
    #[inline(always)]
    pub fn timer3_clk_gate_force_on(&self) -> TIMER3_CLK_GATE_FORCE_ON_R {
        TIMER3_CLK_GATE_FORCE_ON_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMER_POWER_UP_CONF")
            .field("timer0_power_up", &self.timer0_power_up())
            .field("timer1_power_up", &self.timer1_power_up())
            .field("timer2_power_up", &self.timer2_power_up())
            .field("timer3_power_up", &self.timer3_power_up())
            .field("timer0_clk_gate_force_on", &self.timer0_clk_gate_force_on())
            .field("timer1_clk_gate_force_on", &self.timer1_clk_gate_force_on())
            .field("timer2_clk_gate_force_on", &self.timer2_clk_gate_force_on())
            .field("timer3_clk_gate_force_on", &self.timer3_clk_gate_force_on())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Configures whether or not to power up timer0.\\\\0: power down\\\\1: power up"]
    #[inline(always)]
    pub fn timer0_power_up(&mut self) -> TIMER0_POWER_UP_W<'_, TIMER_POWER_UP_CONF_SPEC> {
        TIMER0_POWER_UP_W::new(self, 0)
    }
    #[doc = "Bit 1 - Configures whether or not to power up timer1.\\\\0: power down\\\\1: power up"]
    #[inline(always)]
    pub fn timer1_power_up(&mut self) -> TIMER1_POWER_UP_W<'_, TIMER_POWER_UP_CONF_SPEC> {
        TIMER1_POWER_UP_W::new(self, 1)
    }
    #[doc = "Bit 2 - Configures whether or not to power up timer2.\\\\0: power down\\\\1: power up"]
    #[inline(always)]
    pub fn timer2_power_up(&mut self) -> TIMER2_POWER_UP_W<'_, TIMER_POWER_UP_CONF_SPEC> {
        TIMER2_POWER_UP_W::new(self, 2)
    }
    #[doc = "Bit 3 - Configures whether or not to power up timer3.\\\\0: power down\\\\1: power up"]
    #[inline(always)]
    pub fn timer3_power_up(&mut self) -> TIMER3_POWER_UP_W<'_, TIMER_POWER_UP_CONF_SPEC> {
        TIMER3_POWER_UP_W::new(self, 3)
    }
    #[doc = "Bit 4 - Configures whether or not to open timer0 clock gate.\\\\0: Open the clock gate only when timer0 is power up\\\\1: Force open the clock gate for timer0"]
    #[inline(always)]
    pub fn timer0_clk_gate_force_on(
        &mut self,
    ) -> TIMER0_CLK_GATE_FORCE_ON_W<'_, TIMER_POWER_UP_CONF_SPEC> {
        TIMER0_CLK_GATE_FORCE_ON_W::new(self, 4)
    }
    #[doc = "Bit 5 - Configures whether or not to open timer1 clock gate.\\\\0: Open the clock gate only when timer1 is power up\\\\1: Force open the clock gate for timer1"]
    #[inline(always)]
    pub fn timer1_clk_gate_force_on(
        &mut self,
    ) -> TIMER1_CLK_GATE_FORCE_ON_W<'_, TIMER_POWER_UP_CONF_SPEC> {
        TIMER1_CLK_GATE_FORCE_ON_W::new(self, 5)
    }
    #[doc = "Bit 6 - Configures whether or not to open timer2 clock gate.\\\\0: Open the clock gate only when timer2 is power up\\\\1: Force open the clock gate for timer2"]
    #[inline(always)]
    pub fn timer2_clk_gate_force_on(
        &mut self,
    ) -> TIMER2_CLK_GATE_FORCE_ON_W<'_, TIMER_POWER_UP_CONF_SPEC> {
        TIMER2_CLK_GATE_FORCE_ON_W::new(self, 6)
    }
    #[doc = "Bit 7 - Configures whether or not to open timer3 clock gate.\\\\0: Open the clock gate only when timer3 is power up\\\\1: Force open the clock gate for timer3"]
    #[inline(always)]
    pub fn timer3_clk_gate_force_on(
        &mut self,
    ) -> TIMER3_CLK_GATE_FORCE_ON_W<'_, TIMER_POWER_UP_CONF_SPEC> {
        TIMER3_CLK_GATE_FORCE_ON_W::new(self, 7)
    }
}
#[doc = "LEDC timer power up configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`timer_power_up_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer_power_up_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMER_POWER_UP_CONF_SPEC;
impl crate::RegisterSpec for TIMER_POWER_UP_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer_power_up_conf::R`](R) reader structure"]
impl crate::Readable for TIMER_POWER_UP_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`timer_power_up_conf::W`](W) writer structure"]
impl crate::Writable for TIMER_POWER_UP_CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIMER_POWER_UP_CONF to value 0"]
impl crate::Resettable for TIMER_POWER_UP_CONF_SPEC {}
