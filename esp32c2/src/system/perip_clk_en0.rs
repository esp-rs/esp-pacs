#[doc = "Register `PERIP_CLK_EN0` reader"]
pub struct R(crate::R<PERIP_CLK_EN0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PERIP_CLK_EN0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PERIP_CLK_EN0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PERIP_CLK_EN0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PERIP_CLK_EN0` writer"]
pub struct W(crate::W<PERIP_CLK_EN0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PERIP_CLK_EN0_SPEC>;
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
impl From<crate::W<PERIP_CLK_EN0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PERIP_CLK_EN0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI01_CLK_EN` reader - Set 1 to enable SPI01 clock"]
pub type SPI01_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `SPI01_CLK_EN` writer - Set 1 to enable SPI01 clock"]
pub type SPI01_CLK_EN_W<'a> = crate::BitWriter<'a, u32, PERIP_CLK_EN0_SPEC, bool, 1>;
#[doc = "Field `UART_CLK_EN` reader - Set 1 to enable UART clock"]
pub type UART_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `UART_CLK_EN` writer - Set 1 to enable UART clock"]
pub type UART_CLK_EN_W<'a> = crate::BitWriter<'a, u32, PERIP_CLK_EN0_SPEC, bool, 2>;
#[doc = "Field `UART1_CLK_EN` reader - Set 1 to enable UART1 clock"]
pub type UART1_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `UART1_CLK_EN` writer - Set 1 to enable UART1 clock"]
pub type UART1_CLK_EN_W<'a> = crate::BitWriter<'a, u32, PERIP_CLK_EN0_SPEC, bool, 5>;
#[doc = "Field `SPI2_CLK_EN` reader - Set 1 to enable SPI2 clock"]
pub type SPI2_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `SPI2_CLK_EN` writer - Set 1 to enable SPI2 clock"]
pub type SPI2_CLK_EN_W<'a> = crate::BitWriter<'a, u32, PERIP_CLK_EN0_SPEC, bool, 6>;
#[doc = "Field `I2C_EXT0_CLK_EN` reader - Set 1 to enable I2C_EXT0 clock"]
pub type I2C_EXT0_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `I2C_EXT0_CLK_EN` writer - Set 1 to enable I2C_EXT0 clock"]
pub type I2C_EXT0_CLK_EN_W<'a> = crate::BitWriter<'a, u32, PERIP_CLK_EN0_SPEC, bool, 7>;
#[doc = "Field `LEDC_CLK_EN` reader - Set 1 to enable LEDC clock"]
pub type LEDC_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `LEDC_CLK_EN` writer - Set 1 to enable LEDC clock"]
pub type LEDC_CLK_EN_W<'a> = crate::BitWriter<'a, u32, PERIP_CLK_EN0_SPEC, bool, 11>;
#[doc = "Field `TIMERGROUP_CLK_EN` reader - Set 1 to enable TIMERGROUP clock"]
pub type TIMERGROUP_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `TIMERGROUP_CLK_EN` writer - Set 1 to enable TIMERGROUP clock"]
pub type TIMERGROUP_CLK_EN_W<'a> = crate::BitWriter<'a, u32, PERIP_CLK_EN0_SPEC, bool, 13>;
#[doc = "Field `UART_MEM_CLK_EN` reader - Set 1 to enable UART_MEM clock"]
pub type UART_MEM_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `UART_MEM_CLK_EN` writer - Set 1 to enable UART_MEM clock"]
pub type UART_MEM_CLK_EN_W<'a> = crate::BitWriter<'a, u32, PERIP_CLK_EN0_SPEC, bool, 24>;
#[doc = "Field `APB_SARADC_CLK_EN` reader - Set 1 to enable APB_SARADC clock"]
pub type APB_SARADC_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `APB_SARADC_CLK_EN` writer - Set 1 to enable APB_SARADC clock"]
pub type APB_SARADC_CLK_EN_W<'a> = crate::BitWriter<'a, u32, PERIP_CLK_EN0_SPEC, bool, 28>;
#[doc = "Field `SYSTIMER_CLK_EN` reader - Set 1 to enable SYSTEMTIMER clock"]
pub type SYSTIMER_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `SYSTIMER_CLK_EN` writer - Set 1 to enable SYSTEMTIMER clock"]
pub type SYSTIMER_CLK_EN_W<'a> = crate::BitWriter<'a, u32, PERIP_CLK_EN0_SPEC, bool, 29>;
#[doc = "Field `ADC2_ARB_CLK_EN` reader - Set 1 to enable ADC2_ARB clock"]
pub type ADC2_ARB_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `ADC2_ARB_CLK_EN` writer - Set 1 to enable ADC2_ARB clock"]
pub type ADC2_ARB_CLK_EN_W<'a> = crate::BitWriter<'a, u32, PERIP_CLK_EN0_SPEC, bool, 30>;
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
impl W {
    #[doc = "Bit 1 - Set 1 to enable SPI01 clock"]
    #[inline(always)]
    pub fn spi01_clk_en(&mut self) -> SPI01_CLK_EN_W {
        SPI01_CLK_EN_W::new(self)
    }
    #[doc = "Bit 2 - Set 1 to enable UART clock"]
    #[inline(always)]
    pub fn uart_clk_en(&mut self) -> UART_CLK_EN_W {
        UART_CLK_EN_W::new(self)
    }
    #[doc = "Bit 5 - Set 1 to enable UART1 clock"]
    #[inline(always)]
    pub fn uart1_clk_en(&mut self) -> UART1_CLK_EN_W {
        UART1_CLK_EN_W::new(self)
    }
    #[doc = "Bit 6 - Set 1 to enable SPI2 clock"]
    #[inline(always)]
    pub fn spi2_clk_en(&mut self) -> SPI2_CLK_EN_W {
        SPI2_CLK_EN_W::new(self)
    }
    #[doc = "Bit 7 - Set 1 to enable I2C_EXT0 clock"]
    #[inline(always)]
    pub fn i2c_ext0_clk_en(&mut self) -> I2C_EXT0_CLK_EN_W {
        I2C_EXT0_CLK_EN_W::new(self)
    }
    #[doc = "Bit 11 - Set 1 to enable LEDC clock"]
    #[inline(always)]
    pub fn ledc_clk_en(&mut self) -> LEDC_CLK_EN_W {
        LEDC_CLK_EN_W::new(self)
    }
    #[doc = "Bit 13 - Set 1 to enable TIMERGROUP clock"]
    #[inline(always)]
    pub fn timergroup_clk_en(&mut self) -> TIMERGROUP_CLK_EN_W {
        TIMERGROUP_CLK_EN_W::new(self)
    }
    #[doc = "Bit 24 - Set 1 to enable UART_MEM clock"]
    #[inline(always)]
    pub fn uart_mem_clk_en(&mut self) -> UART_MEM_CLK_EN_W {
        UART_MEM_CLK_EN_W::new(self)
    }
    #[doc = "Bit 28 - Set 1 to enable APB_SARADC clock"]
    #[inline(always)]
    pub fn apb_saradc_clk_en(&mut self) -> APB_SARADC_CLK_EN_W {
        APB_SARADC_CLK_EN_W::new(self)
    }
    #[doc = "Bit 29 - Set 1 to enable SYSTEMTIMER clock"]
    #[inline(always)]
    pub fn systimer_clk_en(&mut self) -> SYSTIMER_CLK_EN_W {
        SYSTIMER_CLK_EN_W::new(self)
    }
    #[doc = "Bit 30 - Set 1 to enable ADC2_ARB clock"]
    #[inline(always)]
    pub fn adc2_arb_clk_en(&mut self) -> ADC2_ARB_CLK_EN_W {
        ADC2_ARB_CLK_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "peripheral clock gating register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [perip_clk_en0](index.html) module"]
pub struct PERIP_CLK_EN0_SPEC;
impl crate::RegisterSpec for PERIP_CLK_EN0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [perip_clk_en0::R](R) reader structure"]
impl crate::Readable for PERIP_CLK_EN0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [perip_clk_en0::W](W) writer structure"]
impl crate::Writable for PERIP_CLK_EN0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PERIP_CLK_EN0 to value 0x7100_2066"]
impl crate::Resettable for PERIP_CLK_EN0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x7100_2066
    }
}
