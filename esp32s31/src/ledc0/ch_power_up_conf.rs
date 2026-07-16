#[doc = "Register `CH_POWER_UP_CONF` reader"]
pub type R = crate::R<CH_POWER_UP_CONF_SPEC>;
#[doc = "Register `CH_POWER_UP_CONF` writer"]
pub type W = crate::W<CH_POWER_UP_CONF_SPEC>;
#[doc = "Field `CH0_POWER_UP` reader - Configures whether or not to power up ch0.\\\\0: power down\\\\1: power up"]
pub type CH0_POWER_UP_R = crate::BitReader;
#[doc = "Field `CH0_POWER_UP` writer - Configures whether or not to power up ch0.\\\\0: power down\\\\1: power up"]
pub type CH0_POWER_UP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_POWER_UP` reader - Configures whether or not to power up ch1.\\\\0: power down\\\\1: power up"]
pub type CH1_POWER_UP_R = crate::BitReader;
#[doc = "Field `CH1_POWER_UP` writer - Configures whether or not to power up ch1.\\\\0: power down\\\\1: power up"]
pub type CH1_POWER_UP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2_POWER_UP` reader - Configures whether or not to power up ch2.\\\\0: power down\\\\1: power up"]
pub type CH2_POWER_UP_R = crate::BitReader;
#[doc = "Field `CH2_POWER_UP` writer - Configures whether or not to power up ch2.\\\\0: power down\\\\1: power up"]
pub type CH2_POWER_UP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3_POWER_UP` reader - Configures whether or not to power up ch3.\\\\0: power down\\\\1: power up"]
pub type CH3_POWER_UP_R = crate::BitReader;
#[doc = "Field `CH3_POWER_UP` writer - Configures whether or not to power up ch3.\\\\0: power down\\\\1: power up"]
pub type CH3_POWER_UP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4_POWER_UP` reader - Configures whether or not to power up ch4.\\\\0: power down\\\\1: power up"]
pub type CH4_POWER_UP_R = crate::BitReader;
#[doc = "Field `CH4_POWER_UP` writer - Configures whether or not to power up ch4.\\\\0: power down\\\\1: power up"]
pub type CH4_POWER_UP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH5_POWER_UP` reader - Configures whether or not to power up ch5.\\\\0: power down\\\\1: power up"]
pub type CH5_POWER_UP_R = crate::BitReader;
#[doc = "Field `CH5_POWER_UP` writer - Configures whether or not to power up ch5.\\\\0: power down\\\\1: power up"]
pub type CH5_POWER_UP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH6_POWER_UP` reader - Configures whether or not to power up ch6.\\\\0: power down\\\\1: power up"]
pub type CH6_POWER_UP_R = crate::BitReader;
#[doc = "Field `CH6_POWER_UP` writer - Configures whether or not to power up ch6.\\\\0: power down\\\\1: power up"]
pub type CH6_POWER_UP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH7_POWER_UP` reader - Configures whether or not to power up ch7.\\\\0: power down\\\\1: power up"]
pub type CH7_POWER_UP_R = crate::BitReader;
#[doc = "Field `CH7_POWER_UP` writer - Configures whether or not to power up ch7.\\\\0: power down\\\\1: power up"]
pub type CH7_POWER_UP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH0_CLK_GATE_FORCE_ON` reader - Configures whether or not to open ch0 clock gate.\\\\0: Open the clock gate only when ch0 is power up\\\\1: Force open the clock gate for ch0"]
pub type CH0_CLK_GATE_FORCE_ON_R = crate::BitReader;
#[doc = "Field `CH0_CLK_GATE_FORCE_ON` writer - Configures whether or not to open ch0 clock gate.\\\\0: Open the clock gate only when ch0 is power up\\\\1: Force open the clock gate for ch0"]
pub type CH0_CLK_GATE_FORCE_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_CLK_GATE_FORCE_ON` reader - Configures whether or not to open ch1 clock gate.\\\\0: Open the clock gate only when ch1 is power up\\\\1: Force open the clock gate for ch1"]
pub type CH1_CLK_GATE_FORCE_ON_R = crate::BitReader;
#[doc = "Field `CH1_CLK_GATE_FORCE_ON` writer - Configures whether or not to open ch1 clock gate.\\\\0: Open the clock gate only when ch1 is power up\\\\1: Force open the clock gate for ch1"]
pub type CH1_CLK_GATE_FORCE_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2_CLK_GATE_FORCE_ON` reader - Configures whether or not to open ch2 clock gate.\\\\0: Open the clock gate only when ch2 is power up\\\\1: Force open the clock gate for ch2"]
pub type CH2_CLK_GATE_FORCE_ON_R = crate::BitReader;
#[doc = "Field `CH2_CLK_GATE_FORCE_ON` writer - Configures whether or not to open ch2 clock gate.\\\\0: Open the clock gate only when ch2 is power up\\\\1: Force open the clock gate for ch2"]
pub type CH2_CLK_GATE_FORCE_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3_CLK_GATE_FORCE_ON` reader - Configures whether or not to open ch3 clock gate.\\\\0: Open the clock gate only when ch3 is power up\\\\1: Force open the clock gate for ch3"]
pub type CH3_CLK_GATE_FORCE_ON_R = crate::BitReader;
#[doc = "Field `CH3_CLK_GATE_FORCE_ON` writer - Configures whether or not to open ch3 clock gate.\\\\0: Open the clock gate only when ch3 is power up\\\\1: Force open the clock gate for ch3"]
pub type CH3_CLK_GATE_FORCE_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4_CLK_GATE_FORCE_ON` reader - Configures whether or not to open ch4 clock gate.\\\\0: Open the clock gate only when ch4 is power up\\\\1: Force open the clock gate for ch4"]
pub type CH4_CLK_GATE_FORCE_ON_R = crate::BitReader;
#[doc = "Field `CH4_CLK_GATE_FORCE_ON` writer - Configures whether or not to open ch4 clock gate.\\\\0: Open the clock gate only when ch4 is power up\\\\1: Force open the clock gate for ch4"]
pub type CH4_CLK_GATE_FORCE_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH5_CLK_GATE_FORCE_ON` reader - Configures whether or not to open ch5 clock gate.\\\\0: Open the clock gate only when ch5 is power up\\\\1: Force open the clock gate for ch5"]
pub type CH5_CLK_GATE_FORCE_ON_R = crate::BitReader;
#[doc = "Field `CH5_CLK_GATE_FORCE_ON` writer - Configures whether or not to open ch5 clock gate.\\\\0: Open the clock gate only when ch5 is power up\\\\1: Force open the clock gate for ch5"]
pub type CH5_CLK_GATE_FORCE_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH6_CLK_GATE_FORCE_ON` reader - Configures whether or not to open ch6 clock gate.\\\\0: Open the clock gate only when ch6 is power up\\\\1: Force open the clock gate for ch6"]
pub type CH6_CLK_GATE_FORCE_ON_R = crate::BitReader;
#[doc = "Field `CH6_CLK_GATE_FORCE_ON` writer - Configures whether or not to open ch6 clock gate.\\\\0: Open the clock gate only when ch6 is power up\\\\1: Force open the clock gate for ch6"]
pub type CH6_CLK_GATE_FORCE_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH7_CLK_GATE_FORCE_ON` reader - Configures whether or not to open ch7 clock gate.\\\\0: Open the clock gate only when ch7 is power up\\\\1: Force open the clock gate for ch7"]
pub type CH7_CLK_GATE_FORCE_ON_R = crate::BitReader;
#[doc = "Field `CH7_CLK_GATE_FORCE_ON` writer - Configures whether or not to open ch7 clock gate.\\\\0: Open the clock gate only when ch7 is power up\\\\1: Force open the clock gate for ch7"]
pub type CH7_CLK_GATE_FORCE_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Configures whether or not to power up ch0.\\\\0: power down\\\\1: power up"]
    #[inline(always)]
    pub fn ch0_power_up(&self) -> CH0_POWER_UP_R {
        CH0_POWER_UP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Configures whether or not to power up ch1.\\\\0: power down\\\\1: power up"]
    #[inline(always)]
    pub fn ch1_power_up(&self) -> CH1_POWER_UP_R {
        CH1_POWER_UP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Configures whether or not to power up ch2.\\\\0: power down\\\\1: power up"]
    #[inline(always)]
    pub fn ch2_power_up(&self) -> CH2_POWER_UP_R {
        CH2_POWER_UP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Configures whether or not to power up ch3.\\\\0: power down\\\\1: power up"]
    #[inline(always)]
    pub fn ch3_power_up(&self) -> CH3_POWER_UP_R {
        CH3_POWER_UP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Configures whether or not to power up ch4.\\\\0: power down\\\\1: power up"]
    #[inline(always)]
    pub fn ch4_power_up(&self) -> CH4_POWER_UP_R {
        CH4_POWER_UP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Configures whether or not to power up ch5.\\\\0: power down\\\\1: power up"]
    #[inline(always)]
    pub fn ch5_power_up(&self) -> CH5_POWER_UP_R {
        CH5_POWER_UP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Configures whether or not to power up ch6.\\\\0: power down\\\\1: power up"]
    #[inline(always)]
    pub fn ch6_power_up(&self) -> CH6_POWER_UP_R {
        CH6_POWER_UP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Configures whether or not to power up ch7.\\\\0: power down\\\\1: power up"]
    #[inline(always)]
    pub fn ch7_power_up(&self) -> CH7_POWER_UP_R {
        CH7_POWER_UP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Configures whether or not to open ch0 clock gate.\\\\0: Open the clock gate only when ch0 is power up\\\\1: Force open the clock gate for ch0"]
    #[inline(always)]
    pub fn ch0_clk_gate_force_on(&self) -> CH0_CLK_GATE_FORCE_ON_R {
        CH0_CLK_GATE_FORCE_ON_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Configures whether or not to open ch1 clock gate.\\\\0: Open the clock gate only when ch1 is power up\\\\1: Force open the clock gate for ch1"]
    #[inline(always)]
    pub fn ch1_clk_gate_force_on(&self) -> CH1_CLK_GATE_FORCE_ON_R {
        CH1_CLK_GATE_FORCE_ON_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Configures whether or not to open ch2 clock gate.\\\\0: Open the clock gate only when ch2 is power up\\\\1: Force open the clock gate for ch2"]
    #[inline(always)]
    pub fn ch2_clk_gate_force_on(&self) -> CH2_CLK_GATE_FORCE_ON_R {
        CH2_CLK_GATE_FORCE_ON_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Configures whether or not to open ch3 clock gate.\\\\0: Open the clock gate only when ch3 is power up\\\\1: Force open the clock gate for ch3"]
    #[inline(always)]
    pub fn ch3_clk_gate_force_on(&self) -> CH3_CLK_GATE_FORCE_ON_R {
        CH3_CLK_GATE_FORCE_ON_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Configures whether or not to open ch4 clock gate.\\\\0: Open the clock gate only when ch4 is power up\\\\1: Force open the clock gate for ch4"]
    #[inline(always)]
    pub fn ch4_clk_gate_force_on(&self) -> CH4_CLK_GATE_FORCE_ON_R {
        CH4_CLK_GATE_FORCE_ON_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Configures whether or not to open ch5 clock gate.\\\\0: Open the clock gate only when ch5 is power up\\\\1: Force open the clock gate for ch5"]
    #[inline(always)]
    pub fn ch5_clk_gate_force_on(&self) -> CH5_CLK_GATE_FORCE_ON_R {
        CH5_CLK_GATE_FORCE_ON_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Configures whether or not to open ch6 clock gate.\\\\0: Open the clock gate only when ch6 is power up\\\\1: Force open the clock gate for ch6"]
    #[inline(always)]
    pub fn ch6_clk_gate_force_on(&self) -> CH6_CLK_GATE_FORCE_ON_R {
        CH6_CLK_GATE_FORCE_ON_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Configures whether or not to open ch7 clock gate.\\\\0: Open the clock gate only when ch7 is power up\\\\1: Force open the clock gate for ch7"]
    #[inline(always)]
    pub fn ch7_clk_gate_force_on(&self) -> CH7_CLK_GATE_FORCE_ON_R {
        CH7_CLK_GATE_FORCE_ON_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CH_POWER_UP_CONF")
            .field("ch0_power_up", &self.ch0_power_up())
            .field("ch1_power_up", &self.ch1_power_up())
            .field("ch2_power_up", &self.ch2_power_up())
            .field("ch3_power_up", &self.ch3_power_up())
            .field("ch4_power_up", &self.ch4_power_up())
            .field("ch5_power_up", &self.ch5_power_up())
            .field("ch6_power_up", &self.ch6_power_up())
            .field("ch7_power_up", &self.ch7_power_up())
            .field("ch0_clk_gate_force_on", &self.ch0_clk_gate_force_on())
            .field("ch1_clk_gate_force_on", &self.ch1_clk_gate_force_on())
            .field("ch2_clk_gate_force_on", &self.ch2_clk_gate_force_on())
            .field("ch3_clk_gate_force_on", &self.ch3_clk_gate_force_on())
            .field("ch4_clk_gate_force_on", &self.ch4_clk_gate_force_on())
            .field("ch5_clk_gate_force_on", &self.ch5_clk_gate_force_on())
            .field("ch6_clk_gate_force_on", &self.ch6_clk_gate_force_on())
            .field("ch7_clk_gate_force_on", &self.ch7_clk_gate_force_on())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Configures whether or not to power up ch0.\\\\0: power down\\\\1: power up"]
    #[inline(always)]
    pub fn ch0_power_up(&mut self) -> CH0_POWER_UP_W<'_, CH_POWER_UP_CONF_SPEC> {
        CH0_POWER_UP_W::new(self, 0)
    }
    #[doc = "Bit 1 - Configures whether or not to power up ch1.\\\\0: power down\\\\1: power up"]
    #[inline(always)]
    pub fn ch1_power_up(&mut self) -> CH1_POWER_UP_W<'_, CH_POWER_UP_CONF_SPEC> {
        CH1_POWER_UP_W::new(self, 1)
    }
    #[doc = "Bit 2 - Configures whether or not to power up ch2.\\\\0: power down\\\\1: power up"]
    #[inline(always)]
    pub fn ch2_power_up(&mut self) -> CH2_POWER_UP_W<'_, CH_POWER_UP_CONF_SPEC> {
        CH2_POWER_UP_W::new(self, 2)
    }
    #[doc = "Bit 3 - Configures whether or not to power up ch3.\\\\0: power down\\\\1: power up"]
    #[inline(always)]
    pub fn ch3_power_up(&mut self) -> CH3_POWER_UP_W<'_, CH_POWER_UP_CONF_SPEC> {
        CH3_POWER_UP_W::new(self, 3)
    }
    #[doc = "Bit 4 - Configures whether or not to power up ch4.\\\\0: power down\\\\1: power up"]
    #[inline(always)]
    pub fn ch4_power_up(&mut self) -> CH4_POWER_UP_W<'_, CH_POWER_UP_CONF_SPEC> {
        CH4_POWER_UP_W::new(self, 4)
    }
    #[doc = "Bit 5 - Configures whether or not to power up ch5.\\\\0: power down\\\\1: power up"]
    #[inline(always)]
    pub fn ch5_power_up(&mut self) -> CH5_POWER_UP_W<'_, CH_POWER_UP_CONF_SPEC> {
        CH5_POWER_UP_W::new(self, 5)
    }
    #[doc = "Bit 6 - Configures whether or not to power up ch6.\\\\0: power down\\\\1: power up"]
    #[inline(always)]
    pub fn ch6_power_up(&mut self) -> CH6_POWER_UP_W<'_, CH_POWER_UP_CONF_SPEC> {
        CH6_POWER_UP_W::new(self, 6)
    }
    #[doc = "Bit 7 - Configures whether or not to power up ch7.\\\\0: power down\\\\1: power up"]
    #[inline(always)]
    pub fn ch7_power_up(&mut self) -> CH7_POWER_UP_W<'_, CH_POWER_UP_CONF_SPEC> {
        CH7_POWER_UP_W::new(self, 7)
    }
    #[doc = "Bit 8 - Configures whether or not to open ch0 clock gate.\\\\0: Open the clock gate only when ch0 is power up\\\\1: Force open the clock gate for ch0"]
    #[inline(always)]
    pub fn ch0_clk_gate_force_on(&mut self) -> CH0_CLK_GATE_FORCE_ON_W<'_, CH_POWER_UP_CONF_SPEC> {
        CH0_CLK_GATE_FORCE_ON_W::new(self, 8)
    }
    #[doc = "Bit 9 - Configures whether or not to open ch1 clock gate.\\\\0: Open the clock gate only when ch1 is power up\\\\1: Force open the clock gate for ch1"]
    #[inline(always)]
    pub fn ch1_clk_gate_force_on(&mut self) -> CH1_CLK_GATE_FORCE_ON_W<'_, CH_POWER_UP_CONF_SPEC> {
        CH1_CLK_GATE_FORCE_ON_W::new(self, 9)
    }
    #[doc = "Bit 10 - Configures whether or not to open ch2 clock gate.\\\\0: Open the clock gate only when ch2 is power up\\\\1: Force open the clock gate for ch2"]
    #[inline(always)]
    pub fn ch2_clk_gate_force_on(&mut self) -> CH2_CLK_GATE_FORCE_ON_W<'_, CH_POWER_UP_CONF_SPEC> {
        CH2_CLK_GATE_FORCE_ON_W::new(self, 10)
    }
    #[doc = "Bit 11 - Configures whether or not to open ch3 clock gate.\\\\0: Open the clock gate only when ch3 is power up\\\\1: Force open the clock gate for ch3"]
    #[inline(always)]
    pub fn ch3_clk_gate_force_on(&mut self) -> CH3_CLK_GATE_FORCE_ON_W<'_, CH_POWER_UP_CONF_SPEC> {
        CH3_CLK_GATE_FORCE_ON_W::new(self, 11)
    }
    #[doc = "Bit 12 - Configures whether or not to open ch4 clock gate.\\\\0: Open the clock gate only when ch4 is power up\\\\1: Force open the clock gate for ch4"]
    #[inline(always)]
    pub fn ch4_clk_gate_force_on(&mut self) -> CH4_CLK_GATE_FORCE_ON_W<'_, CH_POWER_UP_CONF_SPEC> {
        CH4_CLK_GATE_FORCE_ON_W::new(self, 12)
    }
    #[doc = "Bit 13 - Configures whether or not to open ch5 clock gate.\\\\0: Open the clock gate only when ch5 is power up\\\\1: Force open the clock gate for ch5"]
    #[inline(always)]
    pub fn ch5_clk_gate_force_on(&mut self) -> CH5_CLK_GATE_FORCE_ON_W<'_, CH_POWER_UP_CONF_SPEC> {
        CH5_CLK_GATE_FORCE_ON_W::new(self, 13)
    }
    #[doc = "Bit 14 - Configures whether or not to open ch6 clock gate.\\\\0: Open the clock gate only when ch6 is power up\\\\1: Force open the clock gate for ch6"]
    #[inline(always)]
    pub fn ch6_clk_gate_force_on(&mut self) -> CH6_CLK_GATE_FORCE_ON_W<'_, CH_POWER_UP_CONF_SPEC> {
        CH6_CLK_GATE_FORCE_ON_W::new(self, 14)
    }
    #[doc = "Bit 15 - Configures whether or not to open ch7 clock gate.\\\\0: Open the clock gate only when ch7 is power up\\\\1: Force open the clock gate for ch7"]
    #[inline(always)]
    pub fn ch7_clk_gate_force_on(&mut self) -> CH7_CLK_GATE_FORCE_ON_W<'_, CH_POWER_UP_CONF_SPEC> {
        CH7_CLK_GATE_FORCE_ON_W::new(self, 15)
    }
}
#[doc = "LEDC channel power up configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch_power_up_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch_power_up_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH_POWER_UP_CONF_SPEC;
impl crate::RegisterSpec for CH_POWER_UP_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch_power_up_conf::R`](R) reader structure"]
impl crate::Readable for CH_POWER_UP_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ch_power_up_conf::W`](W) writer structure"]
impl crate::Writable for CH_POWER_UP_CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH_POWER_UP_CONF to value 0"]
impl crate::Resettable for CH_POWER_UP_CONF_SPEC {}
