#[doc = "Register `PERIP_CLK_EN0` reader"]
pub type R = crate::R<PERIP_CLK_EN0_SPEC>;
#[doc = "Register `PERIP_CLK_EN0` writer"]
pub type W = crate::W<PERIP_CLK_EN0_SPEC>;
#[doc = "Field `SPI01_CLK_EN` reader - Set 1 to enable SPI01 clock"]
pub type SPI01_CLK_EN_R = crate::BitReader;
#[doc = "Field `SPI01_CLK_EN` writer - Set 1 to enable SPI01 clock"]
pub type SPI01_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART_CLK_EN` reader - Set 1 to enable UART clock"]
pub type UART_CLK_EN_R = crate::BitReader;
#[doc = "Field `UART_CLK_EN` writer - Set 1 to enable UART clock"]
pub type UART_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART1_CLK_EN` reader - Set 1 to enable UART1 clock"]
pub type UART1_CLK_EN_R = crate::BitReader;
#[doc = "Field `UART1_CLK_EN` writer - Set 1 to enable UART1 clock"]
pub type UART1_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI2_CLK_EN` reader - Set 1 to enable SPI2 clock"]
pub type SPI2_CLK_EN_R = crate::BitReader;
#[doc = "Field `SPI2_CLK_EN` writer - Set 1 to enable SPI2 clock"]
pub type SPI2_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_EXT0_CLK_EN` reader - Set 1 to enable I2C_EXT0 clock"]
pub type I2C_EXT0_CLK_EN_R = crate::BitReader;
#[doc = "Field `I2C_EXT0_CLK_EN` writer - Set 1 to enable I2C_EXT0 clock"]
pub type I2C_EXT0_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEDC_CLK_EN` reader - Set 1 to enable LEDC clock"]
pub type LEDC_CLK_EN_R = crate::BitReader;
#[doc = "Field `LEDC_CLK_EN` writer - Set 1 to enable LEDC clock"]
pub type LEDC_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMERGROUP_CLK_EN` reader - Set 1 to enable TIMERGROUP clock"]
pub type TIMERGROUP_CLK_EN_R = crate::BitReader;
#[doc = "Field `TIMERGROUP_CLK_EN` writer - Set 1 to enable TIMERGROUP clock"]
pub type TIMERGROUP_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART_MEM_CLK_EN` reader - Set 1 to enable UART_MEM clock"]
pub type UART_MEM_CLK_EN_R = crate::BitReader;
#[doc = "Field `UART_MEM_CLK_EN` writer - Set 1 to enable UART_MEM clock"]
pub type UART_MEM_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APB_SARADC_CLK_EN` reader - Set 1 to enable APB_SARADC clock"]
pub type APB_SARADC_CLK_EN_R = crate::BitReader;
#[doc = "Field `APB_SARADC_CLK_EN` writer - Set 1 to enable APB_SARADC clock"]
pub type APB_SARADC_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSTIMER_CLK_EN` reader - Set 1 to enable SYSTEMTIMER clock"]
pub type SYSTIMER_CLK_EN_R = crate::BitReader;
#[doc = "Field `SYSTIMER_CLK_EN` writer - Set 1 to enable SYSTEMTIMER clock"]
pub type SYSTIMER_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC2_ARB_CLK_EN` reader - Set 1 to enable ADC2_ARB clock"]
pub type ADC2_ARB_CLK_EN_R = crate::BitReader;
#[doc = "Field `ADC2_ARB_CLK_EN` writer - Set 1 to enable ADC2_ARB clock"]
pub type ADC2_ARB_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Set 1 to enable SPI01 clock"]
    #[inline(always)]
    pub fn spi01_clk_en(&self) -> SPI01_CLK_EN_R {
        SPI01_CLK_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set 1 to enable UART clock"]
    #[inline(always)]
    pub fn uart_clk_en(&self) -> UART_CLK_EN_R {
        UART_CLK_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - Set 1 to enable UART1 clock"]
    #[inline(always)]
    pub fn uart1_clk_en(&self) -> UART1_CLK_EN_R {
        UART1_CLK_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Set 1 to enable SPI2 clock"]
    #[inline(always)]
    pub fn spi2_clk_en(&self) -> SPI2_CLK_EN_R {
        SPI2_CLK_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Set 1 to enable I2C_EXT0 clock"]
    #[inline(always)]
    pub fn i2c_ext0_clk_en(&self) -> I2C_EXT0_CLK_EN_R {
        I2C_EXT0_CLK_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 11 - Set 1 to enable LEDC clock"]
    #[inline(always)]
    pub fn ledc_clk_en(&self) -> LEDC_CLK_EN_R {
        LEDC_CLK_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - Set 1 to enable TIMERGROUP clock"]
    #[inline(always)]
    pub fn timergroup_clk_en(&self) -> TIMERGROUP_CLK_EN_R {
        TIMERGROUP_CLK_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 24 - Set 1 to enable UART_MEM clock"]
    #[inline(always)]
    pub fn uart_mem_clk_en(&self) -> UART_MEM_CLK_EN_R {
        UART_MEM_CLK_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - Set 1 to enable APB_SARADC clock"]
    #[inline(always)]
    pub fn apb_saradc_clk_en(&self) -> APB_SARADC_CLK_EN_R {
        APB_SARADC_CLK_EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Set 1 to enable SYSTEMTIMER clock"]
    #[inline(always)]
    pub fn systimer_clk_en(&self) -> SYSTIMER_CLK_EN_R {
        SYSTIMER_CLK_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Set 1 to enable ADC2_ARB clock"]
    #[inline(always)]
    pub fn adc2_arb_clk_en(&self) -> ADC2_ARB_CLK_EN_R {
        ADC2_ARB_CLK_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PERIP_CLK_EN0")
            .field("spi01_clk_en", &self.spi01_clk_en())
            .field("uart_clk_en", &self.uart_clk_en())
            .field("uart1_clk_en", &self.uart1_clk_en())
            .field("spi2_clk_en", &self.spi2_clk_en())
            .field("i2c_ext0_clk_en", &self.i2c_ext0_clk_en())
            .field("ledc_clk_en", &self.ledc_clk_en())
            .field("timergroup_clk_en", &self.timergroup_clk_en())
            .field("uart_mem_clk_en", &self.uart_mem_clk_en())
            .field("apb_saradc_clk_en", &self.apb_saradc_clk_en())
            .field("systimer_clk_en", &self.systimer_clk_en())
            .field("adc2_arb_clk_en", &self.adc2_arb_clk_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - Set 1 to enable SPI01 clock"]
    #[inline(always)]
    #[must_use]
    pub fn spi01_clk_en(&mut self) -> SPI01_CLK_EN_W<PERIP_CLK_EN0_SPEC> {
        SPI01_CLK_EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - Set 1 to enable UART clock"]
    #[inline(always)]
    #[must_use]
    pub fn uart_clk_en(&mut self) -> UART_CLK_EN_W<PERIP_CLK_EN0_SPEC> {
        UART_CLK_EN_W::new(self, 2)
    }
    #[doc = "Bit 5 - Set 1 to enable UART1 clock"]
    #[inline(always)]
    #[must_use]
    pub fn uart1_clk_en(&mut self) -> UART1_CLK_EN_W<PERIP_CLK_EN0_SPEC> {
        UART1_CLK_EN_W::new(self, 5)
    }
    #[doc = "Bit 6 - Set 1 to enable SPI2 clock"]
    #[inline(always)]
    #[must_use]
    pub fn spi2_clk_en(&mut self) -> SPI2_CLK_EN_W<PERIP_CLK_EN0_SPEC> {
        SPI2_CLK_EN_W::new(self, 6)
    }
    #[doc = "Bit 7 - Set 1 to enable I2C_EXT0 clock"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_ext0_clk_en(&mut self) -> I2C_EXT0_CLK_EN_W<PERIP_CLK_EN0_SPEC> {
        I2C_EXT0_CLK_EN_W::new(self, 7)
    }
    #[doc = "Bit 11 - Set 1 to enable LEDC clock"]
    #[inline(always)]
    #[must_use]
    pub fn ledc_clk_en(&mut self) -> LEDC_CLK_EN_W<PERIP_CLK_EN0_SPEC> {
        LEDC_CLK_EN_W::new(self, 11)
    }
    #[doc = "Bit 13 - Set 1 to enable TIMERGROUP clock"]
    #[inline(always)]
    #[must_use]
    pub fn timergroup_clk_en(&mut self) -> TIMERGROUP_CLK_EN_W<PERIP_CLK_EN0_SPEC> {
        TIMERGROUP_CLK_EN_W::new(self, 13)
    }
    #[doc = "Bit 24 - Set 1 to enable UART_MEM clock"]
    #[inline(always)]
    #[must_use]
    pub fn uart_mem_clk_en(&mut self) -> UART_MEM_CLK_EN_W<PERIP_CLK_EN0_SPEC> {
        UART_MEM_CLK_EN_W::new(self, 24)
    }
    #[doc = "Bit 28 - Set 1 to enable APB_SARADC clock"]
    #[inline(always)]
    #[must_use]
    pub fn apb_saradc_clk_en(&mut self) -> APB_SARADC_CLK_EN_W<PERIP_CLK_EN0_SPEC> {
        APB_SARADC_CLK_EN_W::new(self, 28)
    }
    #[doc = "Bit 29 - Set 1 to enable SYSTEMTIMER clock"]
    #[inline(always)]
    #[must_use]
    pub fn systimer_clk_en(&mut self) -> SYSTIMER_CLK_EN_W<PERIP_CLK_EN0_SPEC> {
        SYSTIMER_CLK_EN_W::new(self, 29)
    }
    #[doc = "Bit 30 - Set 1 to enable ADC2_ARB clock"]
    #[inline(always)]
    #[must_use]
    pub fn adc2_arb_clk_en(&mut self) -> ADC2_ARB_CLK_EN_W<PERIP_CLK_EN0_SPEC> {
        ADC2_ARB_CLK_EN_W::new(self, 30)
    }
}
#[doc = "peripheral clock gating register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`perip_clk_en0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`perip_clk_en0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PERIP_CLK_EN0_SPEC;
impl crate::RegisterSpec for PERIP_CLK_EN0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`perip_clk_en0::R`](R) reader structure"]
impl crate::Readable for PERIP_CLK_EN0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`perip_clk_en0::W`](W) writer structure"]
impl crate::Writable for PERIP_CLK_EN0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PERIP_CLK_EN0 to value 0x7100_2066"]
impl crate::Resettable for PERIP_CLK_EN0_SPEC {
    const RESET_VALUE: u32 = 0x7100_2066;
}
