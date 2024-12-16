#[doc = "Register `PERIP_CLK_EN0` reader"]
pub type R = crate::R<PERIP_CLK_EN0_SPEC>;
#[doc = "Register `PERIP_CLK_EN0` writer"]
pub type W = crate::W<PERIP_CLK_EN0_SPEC>;
#[doc = "Field `TIMERS_CLK_EN` reader - Set this bit to enable clock of timers."]
pub type TIMERS_CLK_EN_R = crate::BitReader;
#[doc = "Field `TIMERS_CLK_EN` writer - Set this bit to enable clock of timers."]
pub type TIMERS_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI01_CLK_EN` reader - Set this bit to enable clock of SPI0 and SPI1."]
pub type SPI01_CLK_EN_R = crate::BitReader;
#[doc = "Field `SPI01_CLK_EN` writer - Set this bit to enable clock of SPI0 and SPI1."]
pub type SPI01_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART_CLK_EN` reader - Set this bit to enable clock of UART0."]
pub type UART_CLK_EN_R = crate::BitReader;
#[doc = "Field `UART_CLK_EN` writer - Set this bit to enable clock of UART0."]
pub type UART_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDG_CLK_EN` reader - Set this bit to enable clock of WDG."]
pub type WDG_CLK_EN_R = crate::BitReader;
#[doc = "Field `WDG_CLK_EN` writer - Set this bit to enable clock of WDG."]
pub type WDG_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2S0_CLK_EN` reader - Set this bit to enable clock of I2S0."]
pub type I2S0_CLK_EN_R = crate::BitReader;
#[doc = "Field `I2S0_CLK_EN` writer - Set this bit to enable clock of I2S0."]
pub type I2S0_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART1_CLK_EN` reader - Set this bit to enable clock of UART1."]
pub type UART1_CLK_EN_R = crate::BitReader;
#[doc = "Field `UART1_CLK_EN` writer - Set this bit to enable clock of UART1."]
pub type UART1_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI2_CLK_EN` reader - Set this bit to enable clock of SPI2."]
pub type SPI2_CLK_EN_R = crate::BitReader;
#[doc = "Field `SPI2_CLK_EN` writer - Set this bit to enable clock of SPI2."]
pub type SPI2_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_EXT0_CLK_EN` reader - Set this bit to enable clock of I2C EXT0."]
pub type I2C_EXT0_CLK_EN_R = crate::BitReader;
#[doc = "Field `I2C_EXT0_CLK_EN` writer - Set this bit to enable clock of I2C EXT0."]
pub type I2C_EXT0_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UHCI0_CLK_EN` reader - Set this bit to enable clock of UHCI0."]
pub type UHCI0_CLK_EN_R = crate::BitReader;
#[doc = "Field `UHCI0_CLK_EN` writer - Set this bit to enable clock of UHCI0."]
pub type UHCI0_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RMT_CLK_EN` reader - Set this bit to enable clock of remote controller."]
pub type RMT_CLK_EN_R = crate::BitReader;
#[doc = "Field `RMT_CLK_EN` writer - Set this bit to enable clock of remote controller."]
pub type RMT_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCNT_CLK_EN` reader - Set this bit to enable clock of pulse count."]
pub type PCNT_CLK_EN_R = crate::BitReader;
#[doc = "Field `PCNT_CLK_EN` writer - Set this bit to enable clock of pulse count."]
pub type PCNT_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEDC_CLK_EN` reader - Set this bit to enable clock of LED PWM."]
pub type LEDC_CLK_EN_R = crate::BitReader;
#[doc = "Field `LEDC_CLK_EN` writer - Set this bit to enable clock of LED PWM."]
pub type LEDC_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UHCI1_CLK_EN` reader - Set this bit to enable clock of UHCI1."]
pub type UHCI1_CLK_EN_R = crate::BitReader;
#[doc = "Field `UHCI1_CLK_EN` writer - Set this bit to enable clock of UHCI1."]
pub type UHCI1_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMERGROUP_CLK_EN` reader - Set this bit to enable clock of timer group0."]
pub type TIMERGROUP_CLK_EN_R = crate::BitReader;
#[doc = "Field `TIMERGROUP_CLK_EN` writer - Set this bit to enable clock of timer group0."]
pub type TIMERGROUP_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EFUSE_CLK_EN` reader - Set this bit to enable clock of eFuse."]
pub type EFUSE_CLK_EN_R = crate::BitReader;
#[doc = "Field `EFUSE_CLK_EN` writer - Set this bit to enable clock of eFuse."]
pub type EFUSE_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMERGROUP1_CLK_EN` reader - Set this bit to enable clock of timer group1."]
pub type TIMERGROUP1_CLK_EN_R = crate::BitReader;
#[doc = "Field `TIMERGROUP1_CLK_EN` writer - Set this bit to enable clock of timer group1."]
pub type TIMERGROUP1_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI3_CLK_EN` reader - Set this bit to enable clock of SPI3."]
pub type SPI3_CLK_EN_R = crate::BitReader;
#[doc = "Field `SPI3_CLK_EN` writer - Set this bit to enable clock of SPI3."]
pub type SPI3_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM0_CLK_EN` reader - Set this bit to enable clock of PWM0."]
pub type PWM0_CLK_EN_R = crate::BitReader;
#[doc = "Field `PWM0_CLK_EN` writer - Set this bit to enable clock of PWM0."]
pub type PWM0_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_EXT1_CLK_EN` reader - Set this bit to enable clock of I2C EXT1."]
pub type I2C_EXT1_CLK_EN_R = crate::BitReader;
#[doc = "Field `I2C_EXT1_CLK_EN` writer - Set this bit to enable clock of I2C EXT1."]
pub type I2C_EXT1_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TWAI_CLK_EN` reader - Set this bit to enable clock of CAN."]
pub type TWAI_CLK_EN_R = crate::BitReader;
#[doc = "Field `TWAI_CLK_EN` writer - Set this bit to enable clock of CAN."]
pub type TWAI_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM1_CLK_EN` reader - Set this bit to enable clock of PWM1."]
pub type PWM1_CLK_EN_R = crate::BitReader;
#[doc = "Field `PWM1_CLK_EN` writer - Set this bit to enable clock of PWM1."]
pub type PWM1_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2S1_CLK_EN` reader - Set this bit to enable clock of I2S1."]
pub type I2S1_CLK_EN_R = crate::BitReader;
#[doc = "Field `I2S1_CLK_EN` writer - Set this bit to enable clock of I2S1."]
pub type I2S1_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI2_DMA_CLK_EN` reader - Set this bit to enable clock of SPI2 DMA."]
pub type SPI2_DMA_CLK_EN_R = crate::BitReader;
#[doc = "Field `SPI2_DMA_CLK_EN` writer - Set this bit to enable clock of SPI2 DMA."]
pub type SPI2_DMA_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB_CLK_EN` reader - Set this bit to enable clock of USB."]
pub type USB_CLK_EN_R = crate::BitReader;
#[doc = "Field `USB_CLK_EN` writer - Set this bit to enable clock of USB."]
pub type USB_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART_MEM_CLK_EN` reader - Set this bit to enable clock of UART memory."]
pub type UART_MEM_CLK_EN_R = crate::BitReader;
#[doc = "Field `UART_MEM_CLK_EN` writer - Set this bit to enable clock of UART memory."]
pub type UART_MEM_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM2_CLK_EN` reader - Set this bit to enable clock of PWM2."]
pub type PWM2_CLK_EN_R = crate::BitReader;
#[doc = "Field `PWM2_CLK_EN` writer - Set this bit to enable clock of PWM2."]
pub type PWM2_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM3_CLK_EN` reader - Set this bit to enable clock of PWM3."]
pub type PWM3_CLK_EN_R = crate::BitReader;
#[doc = "Field `PWM3_CLK_EN` writer - Set this bit to enable clock of PWM3."]
pub type PWM3_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI3_DMA_CLK_EN` reader - Set this bit to enable clock of SPI3 DMA."]
pub type SPI3_DMA_CLK_EN_R = crate::BitReader;
#[doc = "Field `SPI3_DMA_CLK_EN` writer - Set this bit to enable clock of SPI3 DMA."]
pub type SPI3_DMA_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APB_SARADC_CLK_EN` reader - Set this bit to enable clock of SAR ADC."]
pub type APB_SARADC_CLK_EN_R = crate::BitReader;
#[doc = "Field `APB_SARADC_CLK_EN` writer - Set this bit to enable clock of SAR ADC."]
pub type APB_SARADC_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSTIMER_CLK_EN` reader - Set this bit to enable clock of system timer."]
pub type SYSTIMER_CLK_EN_R = crate::BitReader;
#[doc = "Field `SYSTIMER_CLK_EN` writer - Set this bit to enable clock of system timer."]
pub type SYSTIMER_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC2_ARB_CLK_EN` reader - Set this bit to enable clock of aribiter of ADC2."]
pub type ADC2_ARB_CLK_EN_R = crate::BitReader;
#[doc = "Field `ADC2_ARB_CLK_EN` writer - Set this bit to enable clock of aribiter of ADC2."]
pub type ADC2_ARB_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI4_CLK_EN` reader - Set this bit to enable clock of SPI4."]
pub type SPI4_CLK_EN_R = crate::BitReader;
#[doc = "Field `SPI4_CLK_EN` writer - Set this bit to enable clock of SPI4."]
pub type SPI4_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set this bit to enable clock of timers."]
    #[inline(always)]
    pub fn timers_clk_en(&self) -> TIMERS_CLK_EN_R {
        TIMERS_CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set this bit to enable clock of SPI0 and SPI1."]
    #[inline(always)]
    pub fn spi01_clk_en(&self) -> SPI01_CLK_EN_R {
        SPI01_CLK_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set this bit to enable clock of UART0."]
    #[inline(always)]
    pub fn uart_clk_en(&self) -> UART_CLK_EN_R {
        UART_CLK_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set this bit to enable clock of WDG."]
    #[inline(always)]
    pub fn wdg_clk_en(&self) -> WDG_CLK_EN_R {
        WDG_CLK_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Set this bit to enable clock of I2S0."]
    #[inline(always)]
    pub fn i2s0_clk_en(&self) -> I2S0_CLK_EN_R {
        I2S0_CLK_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Set this bit to enable clock of UART1."]
    #[inline(always)]
    pub fn uart1_clk_en(&self) -> UART1_CLK_EN_R {
        UART1_CLK_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Set this bit to enable clock of SPI2."]
    #[inline(always)]
    pub fn spi2_clk_en(&self) -> SPI2_CLK_EN_R {
        SPI2_CLK_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Set this bit to enable clock of I2C EXT0."]
    #[inline(always)]
    pub fn i2c_ext0_clk_en(&self) -> I2C_EXT0_CLK_EN_R {
        I2C_EXT0_CLK_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Set this bit to enable clock of UHCI0."]
    #[inline(always)]
    pub fn uhci0_clk_en(&self) -> UHCI0_CLK_EN_R {
        UHCI0_CLK_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Set this bit to enable clock of remote controller."]
    #[inline(always)]
    pub fn rmt_clk_en(&self) -> RMT_CLK_EN_R {
        RMT_CLK_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Set this bit to enable clock of pulse count."]
    #[inline(always)]
    pub fn pcnt_clk_en(&self) -> PCNT_CLK_EN_R {
        PCNT_CLK_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Set this bit to enable clock of LED PWM."]
    #[inline(always)]
    pub fn ledc_clk_en(&self) -> LEDC_CLK_EN_R {
        LEDC_CLK_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Set this bit to enable clock of UHCI1."]
    #[inline(always)]
    pub fn uhci1_clk_en(&self) -> UHCI1_CLK_EN_R {
        UHCI1_CLK_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Set this bit to enable clock of timer group0."]
    #[inline(always)]
    pub fn timergroup_clk_en(&self) -> TIMERGROUP_CLK_EN_R {
        TIMERGROUP_CLK_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Set this bit to enable clock of eFuse."]
    #[inline(always)]
    pub fn efuse_clk_en(&self) -> EFUSE_CLK_EN_R {
        EFUSE_CLK_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Set this bit to enable clock of timer group1."]
    #[inline(always)]
    pub fn timergroup1_clk_en(&self) -> TIMERGROUP1_CLK_EN_R {
        TIMERGROUP1_CLK_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Set this bit to enable clock of SPI3."]
    #[inline(always)]
    pub fn spi3_clk_en(&self) -> SPI3_CLK_EN_R {
        SPI3_CLK_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Set this bit to enable clock of PWM0."]
    #[inline(always)]
    pub fn pwm0_clk_en(&self) -> PWM0_CLK_EN_R {
        PWM0_CLK_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Set this bit to enable clock of I2C EXT1."]
    #[inline(always)]
    pub fn i2c_ext1_clk_en(&self) -> I2C_EXT1_CLK_EN_R {
        I2C_EXT1_CLK_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Set this bit to enable clock of CAN."]
    #[inline(always)]
    pub fn twai_clk_en(&self) -> TWAI_CLK_EN_R {
        TWAI_CLK_EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Set this bit to enable clock of PWM1."]
    #[inline(always)]
    pub fn pwm1_clk_en(&self) -> PWM1_CLK_EN_R {
        PWM1_CLK_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Set this bit to enable clock of I2S1."]
    #[inline(always)]
    pub fn i2s1_clk_en(&self) -> I2S1_CLK_EN_R {
        I2S1_CLK_EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Set this bit to enable clock of SPI2 DMA."]
    #[inline(always)]
    pub fn spi2_dma_clk_en(&self) -> SPI2_DMA_CLK_EN_R {
        SPI2_DMA_CLK_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Set this bit to enable clock of USB."]
    #[inline(always)]
    pub fn usb_clk_en(&self) -> USB_CLK_EN_R {
        USB_CLK_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Set this bit to enable clock of UART memory."]
    #[inline(always)]
    pub fn uart_mem_clk_en(&self) -> UART_MEM_CLK_EN_R {
        UART_MEM_CLK_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Set this bit to enable clock of PWM2."]
    #[inline(always)]
    pub fn pwm2_clk_en(&self) -> PWM2_CLK_EN_R {
        PWM2_CLK_EN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Set this bit to enable clock of PWM3."]
    #[inline(always)]
    pub fn pwm3_clk_en(&self) -> PWM3_CLK_EN_R {
        PWM3_CLK_EN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Set this bit to enable clock of SPI3 DMA."]
    #[inline(always)]
    pub fn spi3_dma_clk_en(&self) -> SPI3_DMA_CLK_EN_R {
        SPI3_DMA_CLK_EN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Set this bit to enable clock of SAR ADC."]
    #[inline(always)]
    pub fn apb_saradc_clk_en(&self) -> APB_SARADC_CLK_EN_R {
        APB_SARADC_CLK_EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Set this bit to enable clock of system timer."]
    #[inline(always)]
    pub fn systimer_clk_en(&self) -> SYSTIMER_CLK_EN_R {
        SYSTIMER_CLK_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Set this bit to enable clock of aribiter of ADC2."]
    #[inline(always)]
    pub fn adc2_arb_clk_en(&self) -> ADC2_ARB_CLK_EN_R {
        ADC2_ARB_CLK_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Set this bit to enable clock of SPI4."]
    #[inline(always)]
    pub fn spi4_clk_en(&self) -> SPI4_CLK_EN_R {
        SPI4_CLK_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PERIP_CLK_EN0")
            .field("timers_clk_en", &self.timers_clk_en())
            .field("spi01_clk_en", &self.spi01_clk_en())
            .field("uart_clk_en", &self.uart_clk_en())
            .field("wdg_clk_en", &self.wdg_clk_en())
            .field("i2s0_clk_en", &self.i2s0_clk_en())
            .field("uart1_clk_en", &self.uart1_clk_en())
            .field("spi2_clk_en", &self.spi2_clk_en())
            .field("i2c_ext0_clk_en", &self.i2c_ext0_clk_en())
            .field("uhci0_clk_en", &self.uhci0_clk_en())
            .field("rmt_clk_en", &self.rmt_clk_en())
            .field("pcnt_clk_en", &self.pcnt_clk_en())
            .field("ledc_clk_en", &self.ledc_clk_en())
            .field("uhci1_clk_en", &self.uhci1_clk_en())
            .field("timergroup_clk_en", &self.timergroup_clk_en())
            .field("efuse_clk_en", &self.efuse_clk_en())
            .field("timergroup1_clk_en", &self.timergroup1_clk_en())
            .field("spi3_clk_en", &self.spi3_clk_en())
            .field("pwm0_clk_en", &self.pwm0_clk_en())
            .field("i2c_ext1_clk_en", &self.i2c_ext1_clk_en())
            .field("twai_clk_en", &self.twai_clk_en())
            .field("pwm1_clk_en", &self.pwm1_clk_en())
            .field("i2s1_clk_en", &self.i2s1_clk_en())
            .field("spi2_dma_clk_en", &self.spi2_dma_clk_en())
            .field("usb_clk_en", &self.usb_clk_en())
            .field("uart_mem_clk_en", &self.uart_mem_clk_en())
            .field("pwm2_clk_en", &self.pwm2_clk_en())
            .field("pwm3_clk_en", &self.pwm3_clk_en())
            .field("spi3_dma_clk_en", &self.spi3_dma_clk_en())
            .field("apb_saradc_clk_en", &self.apb_saradc_clk_en())
            .field("systimer_clk_en", &self.systimer_clk_en())
            .field("adc2_arb_clk_en", &self.adc2_arb_clk_en())
            .field("spi4_clk_en", &self.spi4_clk_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to enable clock of timers."]
    #[inline(always)]
    pub fn timers_clk_en(&mut self) -> TIMERS_CLK_EN_W<PERIP_CLK_EN0_SPEC> {
        TIMERS_CLK_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set this bit to enable clock of SPI0 and SPI1."]
    #[inline(always)]
    pub fn spi01_clk_en(&mut self) -> SPI01_CLK_EN_W<PERIP_CLK_EN0_SPEC> {
        SPI01_CLK_EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - Set this bit to enable clock of UART0."]
    #[inline(always)]
    pub fn uart_clk_en(&mut self) -> UART_CLK_EN_W<PERIP_CLK_EN0_SPEC> {
        UART_CLK_EN_W::new(self, 2)
    }
    #[doc = "Bit 3 - Set this bit to enable clock of WDG."]
    #[inline(always)]
    pub fn wdg_clk_en(&mut self) -> WDG_CLK_EN_W<PERIP_CLK_EN0_SPEC> {
        WDG_CLK_EN_W::new(self, 3)
    }
    #[doc = "Bit 4 - Set this bit to enable clock of I2S0."]
    #[inline(always)]
    pub fn i2s0_clk_en(&mut self) -> I2S0_CLK_EN_W<PERIP_CLK_EN0_SPEC> {
        I2S0_CLK_EN_W::new(self, 4)
    }
    #[doc = "Bit 5 - Set this bit to enable clock of UART1."]
    #[inline(always)]
    pub fn uart1_clk_en(&mut self) -> UART1_CLK_EN_W<PERIP_CLK_EN0_SPEC> {
        UART1_CLK_EN_W::new(self, 5)
    }
    #[doc = "Bit 6 - Set this bit to enable clock of SPI2."]
    #[inline(always)]
    pub fn spi2_clk_en(&mut self) -> SPI2_CLK_EN_W<PERIP_CLK_EN0_SPEC> {
        SPI2_CLK_EN_W::new(self, 6)
    }
    #[doc = "Bit 7 - Set this bit to enable clock of I2C EXT0."]
    #[inline(always)]
    pub fn i2c_ext0_clk_en(&mut self) -> I2C_EXT0_CLK_EN_W<PERIP_CLK_EN0_SPEC> {
        I2C_EXT0_CLK_EN_W::new(self, 7)
    }
    #[doc = "Bit 8 - Set this bit to enable clock of UHCI0."]
    #[inline(always)]
    pub fn uhci0_clk_en(&mut self) -> UHCI0_CLK_EN_W<PERIP_CLK_EN0_SPEC> {
        UHCI0_CLK_EN_W::new(self, 8)
    }
    #[doc = "Bit 9 - Set this bit to enable clock of remote controller."]
    #[inline(always)]
    pub fn rmt_clk_en(&mut self) -> RMT_CLK_EN_W<PERIP_CLK_EN0_SPEC> {
        RMT_CLK_EN_W::new(self, 9)
    }
    #[doc = "Bit 10 - Set this bit to enable clock of pulse count."]
    #[inline(always)]
    pub fn pcnt_clk_en(&mut self) -> PCNT_CLK_EN_W<PERIP_CLK_EN0_SPEC> {
        PCNT_CLK_EN_W::new(self, 10)
    }
    #[doc = "Bit 11 - Set this bit to enable clock of LED PWM."]
    #[inline(always)]
    pub fn ledc_clk_en(&mut self) -> LEDC_CLK_EN_W<PERIP_CLK_EN0_SPEC> {
        LEDC_CLK_EN_W::new(self, 11)
    }
    #[doc = "Bit 12 - Set this bit to enable clock of UHCI1."]
    #[inline(always)]
    pub fn uhci1_clk_en(&mut self) -> UHCI1_CLK_EN_W<PERIP_CLK_EN0_SPEC> {
        UHCI1_CLK_EN_W::new(self, 12)
    }
    #[doc = "Bit 13 - Set this bit to enable clock of timer group0."]
    #[inline(always)]
    pub fn timergroup_clk_en(&mut self) -> TIMERGROUP_CLK_EN_W<PERIP_CLK_EN0_SPEC> {
        TIMERGROUP_CLK_EN_W::new(self, 13)
    }
    #[doc = "Bit 14 - Set this bit to enable clock of eFuse."]
    #[inline(always)]
    pub fn efuse_clk_en(&mut self) -> EFUSE_CLK_EN_W<PERIP_CLK_EN0_SPEC> {
        EFUSE_CLK_EN_W::new(self, 14)
    }
    #[doc = "Bit 15 - Set this bit to enable clock of timer group1."]
    #[inline(always)]
    pub fn timergroup1_clk_en(&mut self) -> TIMERGROUP1_CLK_EN_W<PERIP_CLK_EN0_SPEC> {
        TIMERGROUP1_CLK_EN_W::new(self, 15)
    }
    #[doc = "Bit 16 - Set this bit to enable clock of SPI3."]
    #[inline(always)]
    pub fn spi3_clk_en(&mut self) -> SPI3_CLK_EN_W<PERIP_CLK_EN0_SPEC> {
        SPI3_CLK_EN_W::new(self, 16)
    }
    #[doc = "Bit 17 - Set this bit to enable clock of PWM0."]
    #[inline(always)]
    pub fn pwm0_clk_en(&mut self) -> PWM0_CLK_EN_W<PERIP_CLK_EN0_SPEC> {
        PWM0_CLK_EN_W::new(self, 17)
    }
    #[doc = "Bit 18 - Set this bit to enable clock of I2C EXT1."]
    #[inline(always)]
    pub fn i2c_ext1_clk_en(&mut self) -> I2C_EXT1_CLK_EN_W<PERIP_CLK_EN0_SPEC> {
        I2C_EXT1_CLK_EN_W::new(self, 18)
    }
    #[doc = "Bit 19 - Set this bit to enable clock of CAN."]
    #[inline(always)]
    pub fn twai_clk_en(&mut self) -> TWAI_CLK_EN_W<PERIP_CLK_EN0_SPEC> {
        TWAI_CLK_EN_W::new(self, 19)
    }
    #[doc = "Bit 20 - Set this bit to enable clock of PWM1."]
    #[inline(always)]
    pub fn pwm1_clk_en(&mut self) -> PWM1_CLK_EN_W<PERIP_CLK_EN0_SPEC> {
        PWM1_CLK_EN_W::new(self, 20)
    }
    #[doc = "Bit 21 - Set this bit to enable clock of I2S1."]
    #[inline(always)]
    pub fn i2s1_clk_en(&mut self) -> I2S1_CLK_EN_W<PERIP_CLK_EN0_SPEC> {
        I2S1_CLK_EN_W::new(self, 21)
    }
    #[doc = "Bit 22 - Set this bit to enable clock of SPI2 DMA."]
    #[inline(always)]
    pub fn spi2_dma_clk_en(&mut self) -> SPI2_DMA_CLK_EN_W<PERIP_CLK_EN0_SPEC> {
        SPI2_DMA_CLK_EN_W::new(self, 22)
    }
    #[doc = "Bit 23 - Set this bit to enable clock of USB."]
    #[inline(always)]
    pub fn usb_clk_en(&mut self) -> USB_CLK_EN_W<PERIP_CLK_EN0_SPEC> {
        USB_CLK_EN_W::new(self, 23)
    }
    #[doc = "Bit 24 - Set this bit to enable clock of UART memory."]
    #[inline(always)]
    pub fn uart_mem_clk_en(&mut self) -> UART_MEM_CLK_EN_W<PERIP_CLK_EN0_SPEC> {
        UART_MEM_CLK_EN_W::new(self, 24)
    }
    #[doc = "Bit 25 - Set this bit to enable clock of PWM2."]
    #[inline(always)]
    pub fn pwm2_clk_en(&mut self) -> PWM2_CLK_EN_W<PERIP_CLK_EN0_SPEC> {
        PWM2_CLK_EN_W::new(self, 25)
    }
    #[doc = "Bit 26 - Set this bit to enable clock of PWM3."]
    #[inline(always)]
    pub fn pwm3_clk_en(&mut self) -> PWM3_CLK_EN_W<PERIP_CLK_EN0_SPEC> {
        PWM3_CLK_EN_W::new(self, 26)
    }
    #[doc = "Bit 27 - Set this bit to enable clock of SPI3 DMA."]
    #[inline(always)]
    pub fn spi3_dma_clk_en(&mut self) -> SPI3_DMA_CLK_EN_W<PERIP_CLK_EN0_SPEC> {
        SPI3_DMA_CLK_EN_W::new(self, 27)
    }
    #[doc = "Bit 28 - Set this bit to enable clock of SAR ADC."]
    #[inline(always)]
    pub fn apb_saradc_clk_en(&mut self) -> APB_SARADC_CLK_EN_W<PERIP_CLK_EN0_SPEC> {
        APB_SARADC_CLK_EN_W::new(self, 28)
    }
    #[doc = "Bit 29 - Set this bit to enable clock of system timer."]
    #[inline(always)]
    pub fn systimer_clk_en(&mut self) -> SYSTIMER_CLK_EN_W<PERIP_CLK_EN0_SPEC> {
        SYSTIMER_CLK_EN_W::new(self, 29)
    }
    #[doc = "Bit 30 - Set this bit to enable clock of aribiter of ADC2."]
    #[inline(always)]
    pub fn adc2_arb_clk_en(&mut self) -> ADC2_ARB_CLK_EN_W<PERIP_CLK_EN0_SPEC> {
        ADC2_ARB_CLK_EN_W::new(self, 30)
    }
    #[doc = "Bit 31 - Set this bit to enable clock of SPI4."]
    #[inline(always)]
    pub fn spi4_clk_en(&mut self) -> SPI4_CLK_EN_W<PERIP_CLK_EN0_SPEC> {
        SPI4_CLK_EN_W::new(self, 31)
    }
}
#[doc = "System peripheral clock (for hardware accelerators) enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`perip_clk_en0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`perip_clk_en0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets PERIP_CLK_EN0 to value 0xf9c1_e06f"]
impl crate::Resettable for PERIP_CLK_EN0_SPEC {
    const RESET_VALUE: u32 = 0xf9c1_e06f;
}
