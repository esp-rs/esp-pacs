#[doc = "Register `CONF` reader"]
pub type R = crate::R<CONF_SPEC>;
#[doc = "Register `CONF` writer"]
pub type W = crate::W<CONF_SPEC>;
#[doc = "Field `APB_CLK_SEL` reader - Configures the clock source for the four timers.\\\\0: APB_CLK\\\\1: RC_FAST_CLK\\\\2: XTAL_CLK\\\\3: Invalid. No clock"]
pub type APB_CLK_SEL_R = crate::FieldReader;
#[doc = "Field `APB_CLK_SEL` writer - Configures the clock source for the four timers.\\\\0: APB_CLK\\\\1: RC_FAST_CLK\\\\2: XTAL_CLK\\\\3: Invalid. No clock"]
pub type APB_CLK_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `GAMMA_RAM_CLK_EN_CH0` reader - Configures whether or not to open LEDC ch0 gamma ram clock gate.\\\\0: Open the clock gate only when application writes or reads LEDC ch0 gamma ram\\\\1: Force open the clock gate for LEDC ch0 gamma ram"]
pub type GAMMA_RAM_CLK_EN_CH0_R = crate::BitReader;
#[doc = "Field `GAMMA_RAM_CLK_EN_CH0` writer - Configures whether or not to open LEDC ch0 gamma ram clock gate.\\\\0: Open the clock gate only when application writes or reads LEDC ch0 gamma ram\\\\1: Force open the clock gate for LEDC ch0 gamma ram"]
pub type GAMMA_RAM_CLK_EN_CH0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GAMMA_RAM_CLK_EN_CH1` reader - Configures whether or not to open LEDC ch1 gamma ram clock gate.\\\\0: Open the clock gate only when application writes or reads LEDC ch1 gamma ram\\\\1: Force open the clock gate for LEDC ch1 gamma ram"]
pub type GAMMA_RAM_CLK_EN_CH1_R = crate::BitReader;
#[doc = "Field `GAMMA_RAM_CLK_EN_CH1` writer - Configures whether or not to open LEDC ch1 gamma ram clock gate.\\\\0: Open the clock gate only when application writes or reads LEDC ch1 gamma ram\\\\1: Force open the clock gate for LEDC ch1 gamma ram"]
pub type GAMMA_RAM_CLK_EN_CH1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GAMMA_RAM_CLK_EN_CH2` reader - Configures whether or not to open LEDC ch2 gamma ram clock gate.\\\\0: Open the clock gate only when application writes or reads LEDC ch2 gamma ram\\\\1: Force open the clock gate for LEDC ch2 gamma ram"]
pub type GAMMA_RAM_CLK_EN_CH2_R = crate::BitReader;
#[doc = "Field `GAMMA_RAM_CLK_EN_CH2` writer - Configures whether or not to open LEDC ch2 gamma ram clock gate.\\\\0: Open the clock gate only when application writes or reads LEDC ch2 gamma ram\\\\1: Force open the clock gate for LEDC ch2 gamma ram"]
pub type GAMMA_RAM_CLK_EN_CH2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GAMMA_RAM_CLK_EN_CH3` reader - Configures whether or not to open LEDC ch3 gamma ram clock gate.\\\\0: Open the clock gate only when application writes or reads LEDC ch3 gamma ram\\\\1: Force open the clock gate for LEDC ch3 gamma ram"]
pub type GAMMA_RAM_CLK_EN_CH3_R = crate::BitReader;
#[doc = "Field `GAMMA_RAM_CLK_EN_CH3` writer - Configures whether or not to open LEDC ch3 gamma ram clock gate.\\\\0: Open the clock gate only when application writes or reads LEDC ch3 gamma ram\\\\1: Force open the clock gate for LEDC ch3 gamma ram"]
pub type GAMMA_RAM_CLK_EN_CH3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GAMMA_RAM_CLK_EN_CH4` reader - Configures whether or not to open LEDC ch4 gamma ram clock gate.\\\\0: Open the clock gate only when application writes or reads LEDC ch4 gamma ram\\\\1: Force open the clock gate for LEDC ch4 gamma ram"]
pub type GAMMA_RAM_CLK_EN_CH4_R = crate::BitReader;
#[doc = "Field `GAMMA_RAM_CLK_EN_CH4` writer - Configures whether or not to open LEDC ch4 gamma ram clock gate.\\\\0: Open the clock gate only when application writes or reads LEDC ch4 gamma ram\\\\1: Force open the clock gate for LEDC ch4 gamma ram"]
pub type GAMMA_RAM_CLK_EN_CH4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GAMMA_RAM_CLK_EN_CH5` reader - Configures whether or not to open LEDC ch5 gamma ram clock gate.\\\\0: Open the clock gate only when application writes or reads LEDC ch5 gamma ram\\\\1: Force open the clock gate for LEDC ch5 gamma ram"]
pub type GAMMA_RAM_CLK_EN_CH5_R = crate::BitReader;
#[doc = "Field `GAMMA_RAM_CLK_EN_CH5` writer - Configures whether or not to open LEDC ch5 gamma ram clock gate.\\\\0: Open the clock gate only when application writes or reads LEDC ch5 gamma ram\\\\1: Force open the clock gate for LEDC ch5 gamma ram"]
pub type GAMMA_RAM_CLK_EN_CH5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GAMMA_RAM_CLK_EN_CH6` reader - Configures whether or not to open LEDC ch6 gamma ram clock gate.\\\\0: Open the clock gate only when application writes or reads LEDC ch6 gamma ram\\\\1: Force open the clock gate for LEDC ch6 gamma ram"]
pub type GAMMA_RAM_CLK_EN_CH6_R = crate::BitReader;
#[doc = "Field `GAMMA_RAM_CLK_EN_CH6` writer - Configures whether or not to open LEDC ch6 gamma ram clock gate.\\\\0: Open the clock gate only when application writes or reads LEDC ch6 gamma ram\\\\1: Force open the clock gate for LEDC ch6 gamma ram"]
pub type GAMMA_RAM_CLK_EN_CH6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GAMMA_RAM_CLK_EN_CH7` reader - Configures whether or not to open LEDC ch7 gamma ram clock gate.\\\\0: Open the clock gate only when application writes or reads LEDC ch7 gamma ram\\\\1: Force open the clock gate for LEDC ch7 gamma ram"]
pub type GAMMA_RAM_CLK_EN_CH7_R = crate::BitReader;
#[doc = "Field `GAMMA_RAM_CLK_EN_CH7` writer - Configures whether or not to open LEDC ch7 gamma ram clock gate.\\\\0: Open the clock gate only when application writes or reads LEDC ch7 gamma ram\\\\1: Force open the clock gate for LEDC ch7 gamma ram"]
pub type GAMMA_RAM_CLK_EN_CH7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_EN` reader - Configures whether or not to open register clock gate.\\\\0: Open the clock gate only when application writes registers\\\\1: Force open the clock gate for register"]
pub type CLK_EN_R = crate::BitReader;
#[doc = "Field `CLK_EN` writer - Configures whether or not to open register clock gate.\\\\0: Open the clock gate only when application writes registers\\\\1: Force open the clock gate for register"]
pub type CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Configures the clock source for the four timers.\\\\0: APB_CLK\\\\1: RC_FAST_CLK\\\\2: XTAL_CLK\\\\3: Invalid. No clock"]
    #[inline(always)]
    pub fn apb_clk_sel(&self) -> APB_CLK_SEL_R {
        APB_CLK_SEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Configures whether or not to open LEDC ch0 gamma ram clock gate.\\\\0: Open the clock gate only when application writes or reads LEDC ch0 gamma ram\\\\1: Force open the clock gate for LEDC ch0 gamma ram"]
    #[inline(always)]
    pub fn gamma_ram_clk_en_ch0(&self) -> GAMMA_RAM_CLK_EN_CH0_R {
        GAMMA_RAM_CLK_EN_CH0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Configures whether or not to open LEDC ch1 gamma ram clock gate.\\\\0: Open the clock gate only when application writes or reads LEDC ch1 gamma ram\\\\1: Force open the clock gate for LEDC ch1 gamma ram"]
    #[inline(always)]
    pub fn gamma_ram_clk_en_ch1(&self) -> GAMMA_RAM_CLK_EN_CH1_R {
        GAMMA_RAM_CLK_EN_CH1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Configures whether or not to open LEDC ch2 gamma ram clock gate.\\\\0: Open the clock gate only when application writes or reads LEDC ch2 gamma ram\\\\1: Force open the clock gate for LEDC ch2 gamma ram"]
    #[inline(always)]
    pub fn gamma_ram_clk_en_ch2(&self) -> GAMMA_RAM_CLK_EN_CH2_R {
        GAMMA_RAM_CLK_EN_CH2_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Configures whether or not to open LEDC ch3 gamma ram clock gate.\\\\0: Open the clock gate only when application writes or reads LEDC ch3 gamma ram\\\\1: Force open the clock gate for LEDC ch3 gamma ram"]
    #[inline(always)]
    pub fn gamma_ram_clk_en_ch3(&self) -> GAMMA_RAM_CLK_EN_CH3_R {
        GAMMA_RAM_CLK_EN_CH3_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Configures whether or not to open LEDC ch4 gamma ram clock gate.\\\\0: Open the clock gate only when application writes or reads LEDC ch4 gamma ram\\\\1: Force open the clock gate for LEDC ch4 gamma ram"]
    #[inline(always)]
    pub fn gamma_ram_clk_en_ch4(&self) -> GAMMA_RAM_CLK_EN_CH4_R {
        GAMMA_RAM_CLK_EN_CH4_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Configures whether or not to open LEDC ch5 gamma ram clock gate.\\\\0: Open the clock gate only when application writes or reads LEDC ch5 gamma ram\\\\1: Force open the clock gate for LEDC ch5 gamma ram"]
    #[inline(always)]
    pub fn gamma_ram_clk_en_ch5(&self) -> GAMMA_RAM_CLK_EN_CH5_R {
        GAMMA_RAM_CLK_EN_CH5_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Configures whether or not to open LEDC ch6 gamma ram clock gate.\\\\0: Open the clock gate only when application writes or reads LEDC ch6 gamma ram\\\\1: Force open the clock gate for LEDC ch6 gamma ram"]
    #[inline(always)]
    pub fn gamma_ram_clk_en_ch6(&self) -> GAMMA_RAM_CLK_EN_CH6_R {
        GAMMA_RAM_CLK_EN_CH6_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Configures whether or not to open LEDC ch7 gamma ram clock gate.\\\\0: Open the clock gate only when application writes or reads LEDC ch7 gamma ram\\\\1: Force open the clock gate for LEDC ch7 gamma ram"]
    #[inline(always)]
    pub fn gamma_ram_clk_en_ch7(&self) -> GAMMA_RAM_CLK_EN_CH7_R {
        GAMMA_RAM_CLK_EN_CH7_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 31 - Configures whether or not to open register clock gate.\\\\0: Open the clock gate only when application writes registers\\\\1: Force open the clock gate for register"]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONF")
            .field("apb_clk_sel", &self.apb_clk_sel())
            .field("gamma_ram_clk_en_ch0", &self.gamma_ram_clk_en_ch0())
            .field("gamma_ram_clk_en_ch1", &self.gamma_ram_clk_en_ch1())
            .field("gamma_ram_clk_en_ch2", &self.gamma_ram_clk_en_ch2())
            .field("gamma_ram_clk_en_ch3", &self.gamma_ram_clk_en_ch3())
            .field("gamma_ram_clk_en_ch4", &self.gamma_ram_clk_en_ch4())
            .field("gamma_ram_clk_en_ch5", &self.gamma_ram_clk_en_ch5())
            .field("gamma_ram_clk_en_ch6", &self.gamma_ram_clk_en_ch6())
            .field("gamma_ram_clk_en_ch7", &self.gamma_ram_clk_en_ch7())
            .field("clk_en", &self.clk_en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Configures the clock source for the four timers.\\\\0: APB_CLK\\\\1: RC_FAST_CLK\\\\2: XTAL_CLK\\\\3: Invalid. No clock"]
    #[inline(always)]
    pub fn apb_clk_sel(&mut self) -> APB_CLK_SEL_W<CONF_SPEC> {
        APB_CLK_SEL_W::new(self, 0)
    }
    #[doc = "Bit 2 - Configures whether or not to open LEDC ch0 gamma ram clock gate.\\\\0: Open the clock gate only when application writes or reads LEDC ch0 gamma ram\\\\1: Force open the clock gate for LEDC ch0 gamma ram"]
    #[inline(always)]
    pub fn gamma_ram_clk_en_ch0(&mut self) -> GAMMA_RAM_CLK_EN_CH0_W<CONF_SPEC> {
        GAMMA_RAM_CLK_EN_CH0_W::new(self, 2)
    }
    #[doc = "Bit 3 - Configures whether or not to open LEDC ch1 gamma ram clock gate.\\\\0: Open the clock gate only when application writes or reads LEDC ch1 gamma ram\\\\1: Force open the clock gate for LEDC ch1 gamma ram"]
    #[inline(always)]
    pub fn gamma_ram_clk_en_ch1(&mut self) -> GAMMA_RAM_CLK_EN_CH1_W<CONF_SPEC> {
        GAMMA_RAM_CLK_EN_CH1_W::new(self, 3)
    }
    #[doc = "Bit 4 - Configures whether or not to open LEDC ch2 gamma ram clock gate.\\\\0: Open the clock gate only when application writes or reads LEDC ch2 gamma ram\\\\1: Force open the clock gate for LEDC ch2 gamma ram"]
    #[inline(always)]
    pub fn gamma_ram_clk_en_ch2(&mut self) -> GAMMA_RAM_CLK_EN_CH2_W<CONF_SPEC> {
        GAMMA_RAM_CLK_EN_CH2_W::new(self, 4)
    }
    #[doc = "Bit 5 - Configures whether or not to open LEDC ch3 gamma ram clock gate.\\\\0: Open the clock gate only when application writes or reads LEDC ch3 gamma ram\\\\1: Force open the clock gate for LEDC ch3 gamma ram"]
    #[inline(always)]
    pub fn gamma_ram_clk_en_ch3(&mut self) -> GAMMA_RAM_CLK_EN_CH3_W<CONF_SPEC> {
        GAMMA_RAM_CLK_EN_CH3_W::new(self, 5)
    }
    #[doc = "Bit 6 - Configures whether or not to open LEDC ch4 gamma ram clock gate.\\\\0: Open the clock gate only when application writes or reads LEDC ch4 gamma ram\\\\1: Force open the clock gate for LEDC ch4 gamma ram"]
    #[inline(always)]
    pub fn gamma_ram_clk_en_ch4(&mut self) -> GAMMA_RAM_CLK_EN_CH4_W<CONF_SPEC> {
        GAMMA_RAM_CLK_EN_CH4_W::new(self, 6)
    }
    #[doc = "Bit 7 - Configures whether or not to open LEDC ch5 gamma ram clock gate.\\\\0: Open the clock gate only when application writes or reads LEDC ch5 gamma ram\\\\1: Force open the clock gate for LEDC ch5 gamma ram"]
    #[inline(always)]
    pub fn gamma_ram_clk_en_ch5(&mut self) -> GAMMA_RAM_CLK_EN_CH5_W<CONF_SPEC> {
        GAMMA_RAM_CLK_EN_CH5_W::new(self, 7)
    }
    #[doc = "Bit 8 - Configures whether or not to open LEDC ch6 gamma ram clock gate.\\\\0: Open the clock gate only when application writes or reads LEDC ch6 gamma ram\\\\1: Force open the clock gate for LEDC ch6 gamma ram"]
    #[inline(always)]
    pub fn gamma_ram_clk_en_ch6(&mut self) -> GAMMA_RAM_CLK_EN_CH6_W<CONF_SPEC> {
        GAMMA_RAM_CLK_EN_CH6_W::new(self, 8)
    }
    #[doc = "Bit 9 - Configures whether or not to open LEDC ch7 gamma ram clock gate.\\\\0: Open the clock gate only when application writes or reads LEDC ch7 gamma ram\\\\1: Force open the clock gate for LEDC ch7 gamma ram"]
    #[inline(always)]
    pub fn gamma_ram_clk_en_ch7(&mut self) -> GAMMA_RAM_CLK_EN_CH7_W<CONF_SPEC> {
        GAMMA_RAM_CLK_EN_CH7_W::new(self, 9)
    }
    #[doc = "Bit 31 - Configures whether or not to open register clock gate.\\\\0: Open the clock gate only when application writes registers\\\\1: Force open the clock gate for register"]
    #[inline(always)]
    pub fn clk_en(&mut self) -> CLK_EN_W<CONF_SPEC> {
        CLK_EN_W::new(self, 31)
    }
}
#[doc = "LEDC global configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONF_SPEC;
impl crate::RegisterSpec for CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`conf::R`](R) reader structure"]
impl crate::Readable for CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`conf::W`](W) writer structure"]
impl crate::Writable for CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONF to value 0"]
impl crate::Resettable for CONF_SPEC {
    const RESET_VALUE: u32 = 0;
}
