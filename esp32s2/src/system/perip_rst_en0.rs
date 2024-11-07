#[doc = "Register `PERIP_RST_EN0` reader"]
pub type R = crate::R<PERIP_RST_EN0_SPEC>;
#[doc = "Register `PERIP_RST_EN0` writer"]
pub type W = crate::W<PERIP_RST_EN0_SPEC>;
#[doc = "Field `TIMERS_RST` reader - Set this bit to reset timers."]
pub type TIMERS_RST_R = crate::BitReader;
#[doc = "Field `TIMERS_RST` writer - Set this bit to reset timers."]
pub type TIMERS_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI01_RST` reader - Set this bit to reset SPI0 and SPI1."]
pub type SPI01_RST_R = crate::BitReader;
#[doc = "Field `SPI01_RST` writer - Set this bit to reset SPI0 and SPI1."]
pub type SPI01_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART_RST` reader - Set this bit to reset UART0."]
pub type UART_RST_R = crate::BitReader;
#[doc = "Field `UART_RST` writer - Set this bit to reset UART0."]
pub type UART_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDG_RST` reader - Set this bit to reset WDG."]
pub type WDG_RST_R = crate::BitReader;
#[doc = "Field `WDG_RST` writer - Set this bit to reset WDG."]
pub type WDG_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2S0_RST` reader - Set this bit to reset I2S0."]
pub type I2S0_RST_R = crate::BitReader;
#[doc = "Field `I2S0_RST` writer - Set this bit to reset I2S0."]
pub type I2S0_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART1_RST` reader - Set this bit to reset UART1."]
pub type UART1_RST_R = crate::BitReader;
#[doc = "Field `UART1_RST` writer - Set this bit to reset UART1."]
pub type UART1_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI2_RST` reader - Set this bit to reset SPI2."]
pub type SPI2_RST_R = crate::BitReader;
#[doc = "Field `SPI2_RST` writer - Set this bit to reset SPI2."]
pub type SPI2_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_EXT0_RST` reader - Set this bit to reset I2C EXT0."]
pub type I2C_EXT0_RST_R = crate::BitReader;
#[doc = "Field `I2C_EXT0_RST` writer - Set this bit to reset I2C EXT0."]
pub type I2C_EXT0_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UHCI0_RST` reader - Set this bit to reset UHCI0."]
pub type UHCI0_RST_R = crate::BitReader;
#[doc = "Field `UHCI0_RST` writer - Set this bit to reset UHCI0."]
pub type UHCI0_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RMT_RST` reader - Set this bit to reset remote controller."]
pub type RMT_RST_R = crate::BitReader;
#[doc = "Field `RMT_RST` writer - Set this bit to reset remote controller."]
pub type RMT_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCNT_RST` reader - Set this bit to reset pulse count."]
pub type PCNT_RST_R = crate::BitReader;
#[doc = "Field `PCNT_RST` writer - Set this bit to reset pulse count."]
pub type PCNT_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEDC_RST` reader - Set this bit to reset LED PWM."]
pub type LEDC_RST_R = crate::BitReader;
#[doc = "Field `LEDC_RST` writer - Set this bit to reset LED PWM."]
pub type LEDC_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UHCI1_RST` reader - Set this bit to reset UHCI1."]
pub type UHCI1_RST_R = crate::BitReader;
#[doc = "Field `UHCI1_RST` writer - Set this bit to reset UHCI1."]
pub type UHCI1_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMERGROUP_RST` reader - Set this bit to reset timer group0."]
pub type TIMERGROUP_RST_R = crate::BitReader;
#[doc = "Field `TIMERGROUP_RST` writer - Set this bit to reset timer group0."]
pub type TIMERGROUP_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EFUSE_RST` reader - Set this bit to reset eFuse."]
pub type EFUSE_RST_R = crate::BitReader;
#[doc = "Field `EFUSE_RST` writer - Set this bit to reset eFuse."]
pub type EFUSE_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMERGROUP1_RST` reader - Set this bit to reset timer group1."]
pub type TIMERGROUP1_RST_R = crate::BitReader;
#[doc = "Field `TIMERGROUP1_RST` writer - Set this bit to reset timer group1."]
pub type TIMERGROUP1_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI3_RST` reader - Set this bit to reset SPI3."]
pub type SPI3_RST_R = crate::BitReader;
#[doc = "Field `SPI3_RST` writer - Set this bit to reset SPI3."]
pub type SPI3_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM0_RST` reader - Set this bit to reset PWM0."]
pub type PWM0_RST_R = crate::BitReader;
#[doc = "Field `PWM0_RST` writer - Set this bit to reset PWM0."]
pub type PWM0_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_EXT1_RST` reader - Set this bit to reset I2C EXT1."]
pub type I2C_EXT1_RST_R = crate::BitReader;
#[doc = "Field `I2C_EXT1_RST` writer - Set this bit to reset I2C EXT1."]
pub type I2C_EXT1_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TWAI_RST` reader - Set this bit to reset CAN."]
pub type TWAI_RST_R = crate::BitReader;
#[doc = "Field `TWAI_RST` writer - Set this bit to reset CAN."]
pub type TWAI_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM1_RST` reader - Set this bit to reset PWM1."]
pub type PWM1_RST_R = crate::BitReader;
#[doc = "Field `PWM1_RST` writer - Set this bit to reset PWM1."]
pub type PWM1_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2S1_RST` reader - Set this bit to reset I2S1."]
pub type I2S1_RST_R = crate::BitReader;
#[doc = "Field `I2S1_RST` writer - Set this bit to reset I2S1."]
pub type I2S1_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI2_DMA_RST` reader - Set this bit to reset SPI2 DMA."]
pub type SPI2_DMA_RST_R = crate::BitReader;
#[doc = "Field `SPI2_DMA_RST` writer - Set this bit to reset SPI2 DMA."]
pub type SPI2_DMA_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB_RST` reader - Set this bit to reset USB."]
pub type USB_RST_R = crate::BitReader;
#[doc = "Field `USB_RST` writer - Set this bit to reset USB."]
pub type USB_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART_MEM_RST` reader - Set this bit to reset UART memory."]
pub type UART_MEM_RST_R = crate::BitReader;
#[doc = "Field `UART_MEM_RST` writer - Set this bit to reset UART memory."]
pub type UART_MEM_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM2_RST` reader - Set this bit to reset PWM2."]
pub type PWM2_RST_R = crate::BitReader;
#[doc = "Field `PWM2_RST` writer - Set this bit to reset PWM2."]
pub type PWM2_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM3_RST` reader - Set this bit to reset PWM3."]
pub type PWM3_RST_R = crate::BitReader;
#[doc = "Field `PWM3_RST` writer - Set this bit to reset PWM3."]
pub type PWM3_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI3_DMA_RST` reader - Set this bit to reset SPI3 DMA."]
pub type SPI3_DMA_RST_R = crate::BitReader;
#[doc = "Field `SPI3_DMA_RST` writer - Set this bit to reset SPI3 DMA."]
pub type SPI3_DMA_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APB_SARADC_RST` reader - Set this bit to reset SAR ADC."]
pub type APB_SARADC_RST_R = crate::BitReader;
#[doc = "Field `APB_SARADC_RST` writer - Set this bit to reset SAR ADC."]
pub type APB_SARADC_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSTIMER_RST` reader - Set this bit to reset system timer."]
pub type SYSTIMER_RST_R = crate::BitReader;
#[doc = "Field `SYSTIMER_RST` writer - Set this bit to reset system timer."]
pub type SYSTIMER_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC2_ARB_RST` reader - Set this bit to reset aribiter of ADC2."]
pub type ADC2_ARB_RST_R = crate::BitReader;
#[doc = "Field `ADC2_ARB_RST` writer - Set this bit to reset aribiter of ADC2."]
pub type ADC2_ARB_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI4_RST` reader - Set this bit to reset SPI4."]
pub type SPI4_RST_R = crate::BitReader;
#[doc = "Field `SPI4_RST` writer - Set this bit to reset SPI4."]
pub type SPI4_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set this bit to reset timers."]
    #[inline(always)]
    pub fn timers_rst(&self) -> TIMERS_RST_R {
        TIMERS_RST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set this bit to reset SPI0 and SPI1."]
    #[inline(always)]
    pub fn spi01_rst(&self) -> SPI01_RST_R {
        SPI01_RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set this bit to reset UART0."]
    #[inline(always)]
    pub fn uart_rst(&self) -> UART_RST_R {
        UART_RST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set this bit to reset WDG."]
    #[inline(always)]
    pub fn wdg_rst(&self) -> WDG_RST_R {
        WDG_RST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Set this bit to reset I2S0."]
    #[inline(always)]
    pub fn i2s0_rst(&self) -> I2S0_RST_R {
        I2S0_RST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Set this bit to reset UART1."]
    #[inline(always)]
    pub fn uart1_rst(&self) -> UART1_RST_R {
        UART1_RST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Set this bit to reset SPI2."]
    #[inline(always)]
    pub fn spi2_rst(&self) -> SPI2_RST_R {
        SPI2_RST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Set this bit to reset I2C EXT0."]
    #[inline(always)]
    pub fn i2c_ext0_rst(&self) -> I2C_EXT0_RST_R {
        I2C_EXT0_RST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Set this bit to reset UHCI0."]
    #[inline(always)]
    pub fn uhci0_rst(&self) -> UHCI0_RST_R {
        UHCI0_RST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Set this bit to reset remote controller."]
    #[inline(always)]
    pub fn rmt_rst(&self) -> RMT_RST_R {
        RMT_RST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Set this bit to reset pulse count."]
    #[inline(always)]
    pub fn pcnt_rst(&self) -> PCNT_RST_R {
        PCNT_RST_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Set this bit to reset LED PWM."]
    #[inline(always)]
    pub fn ledc_rst(&self) -> LEDC_RST_R {
        LEDC_RST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Set this bit to reset UHCI1."]
    #[inline(always)]
    pub fn uhci1_rst(&self) -> UHCI1_RST_R {
        UHCI1_RST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Set this bit to reset timer group0."]
    #[inline(always)]
    pub fn timergroup_rst(&self) -> TIMERGROUP_RST_R {
        TIMERGROUP_RST_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Set this bit to reset eFuse."]
    #[inline(always)]
    pub fn efuse_rst(&self) -> EFUSE_RST_R {
        EFUSE_RST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Set this bit to reset timer group1."]
    #[inline(always)]
    pub fn timergroup1_rst(&self) -> TIMERGROUP1_RST_R {
        TIMERGROUP1_RST_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Set this bit to reset SPI3."]
    #[inline(always)]
    pub fn spi3_rst(&self) -> SPI3_RST_R {
        SPI3_RST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Set this bit to reset PWM0."]
    #[inline(always)]
    pub fn pwm0_rst(&self) -> PWM0_RST_R {
        PWM0_RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Set this bit to reset I2C EXT1."]
    #[inline(always)]
    pub fn i2c_ext1_rst(&self) -> I2C_EXT1_RST_R {
        I2C_EXT1_RST_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Set this bit to reset CAN."]
    #[inline(always)]
    pub fn twai_rst(&self) -> TWAI_RST_R {
        TWAI_RST_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Set this bit to reset PWM1."]
    #[inline(always)]
    pub fn pwm1_rst(&self) -> PWM1_RST_R {
        PWM1_RST_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Set this bit to reset I2S1."]
    #[inline(always)]
    pub fn i2s1_rst(&self) -> I2S1_RST_R {
        I2S1_RST_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Set this bit to reset SPI2 DMA."]
    #[inline(always)]
    pub fn spi2_dma_rst(&self) -> SPI2_DMA_RST_R {
        SPI2_DMA_RST_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Set this bit to reset USB."]
    #[inline(always)]
    pub fn usb_rst(&self) -> USB_RST_R {
        USB_RST_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Set this bit to reset UART memory."]
    #[inline(always)]
    pub fn uart_mem_rst(&self) -> UART_MEM_RST_R {
        UART_MEM_RST_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Set this bit to reset PWM2."]
    #[inline(always)]
    pub fn pwm2_rst(&self) -> PWM2_RST_R {
        PWM2_RST_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Set this bit to reset PWM3."]
    #[inline(always)]
    pub fn pwm3_rst(&self) -> PWM3_RST_R {
        PWM3_RST_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Set this bit to reset SPI3 DMA."]
    #[inline(always)]
    pub fn spi3_dma_rst(&self) -> SPI3_DMA_RST_R {
        SPI3_DMA_RST_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Set this bit to reset SAR ADC."]
    #[inline(always)]
    pub fn apb_saradc_rst(&self) -> APB_SARADC_RST_R {
        APB_SARADC_RST_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Set this bit to reset system timer."]
    #[inline(always)]
    pub fn systimer_rst(&self) -> SYSTIMER_RST_R {
        SYSTIMER_RST_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Set this bit to reset aribiter of ADC2."]
    #[inline(always)]
    pub fn adc2_arb_rst(&self) -> ADC2_ARB_RST_R {
        ADC2_ARB_RST_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Set this bit to reset SPI4."]
    #[inline(always)]
    pub fn spi4_rst(&self) -> SPI4_RST_R {
        SPI4_RST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PERIP_RST_EN0")
            .field("timers_rst", &self.timers_rst())
            .field("spi01_rst", &self.spi01_rst())
            .field("uart_rst", &self.uart_rst())
            .field("wdg_rst", &self.wdg_rst())
            .field("i2s0_rst", &self.i2s0_rst())
            .field("uart1_rst", &self.uart1_rst())
            .field("spi2_rst", &self.spi2_rst())
            .field("i2c_ext0_rst", &self.i2c_ext0_rst())
            .field("uhci0_rst", &self.uhci0_rst())
            .field("rmt_rst", &self.rmt_rst())
            .field("pcnt_rst", &self.pcnt_rst())
            .field("ledc_rst", &self.ledc_rst())
            .field("uhci1_rst", &self.uhci1_rst())
            .field("timergroup_rst", &self.timergroup_rst())
            .field("efuse_rst", &self.efuse_rst())
            .field("timergroup1_rst", &self.timergroup1_rst())
            .field("spi3_rst", &self.spi3_rst())
            .field("pwm0_rst", &self.pwm0_rst())
            .field("i2c_ext1_rst", &self.i2c_ext1_rst())
            .field("twai_rst", &self.twai_rst())
            .field("pwm1_rst", &self.pwm1_rst())
            .field("i2s1_rst", &self.i2s1_rst())
            .field("spi2_dma_rst", &self.spi2_dma_rst())
            .field("usb_rst", &self.usb_rst())
            .field("uart_mem_rst", &self.uart_mem_rst())
            .field("pwm2_rst", &self.pwm2_rst())
            .field("pwm3_rst", &self.pwm3_rst())
            .field("spi3_dma_rst", &self.spi3_dma_rst())
            .field("apb_saradc_rst", &self.apb_saradc_rst())
            .field("systimer_rst", &self.systimer_rst())
            .field("adc2_arb_rst", &self.adc2_arb_rst())
            .field("spi4_rst", &self.spi4_rst())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to reset timers."]
    #[inline(always)]
    pub fn timers_rst(&mut self) -> TIMERS_RST_W<PERIP_RST_EN0_SPEC> {
        TIMERS_RST_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set this bit to reset SPI0 and SPI1."]
    #[inline(always)]
    pub fn spi01_rst(&mut self) -> SPI01_RST_W<PERIP_RST_EN0_SPEC> {
        SPI01_RST_W::new(self, 1)
    }
    #[doc = "Bit 2 - Set this bit to reset UART0."]
    #[inline(always)]
    pub fn uart_rst(&mut self) -> UART_RST_W<PERIP_RST_EN0_SPEC> {
        UART_RST_W::new(self, 2)
    }
    #[doc = "Bit 3 - Set this bit to reset WDG."]
    #[inline(always)]
    pub fn wdg_rst(&mut self) -> WDG_RST_W<PERIP_RST_EN0_SPEC> {
        WDG_RST_W::new(self, 3)
    }
    #[doc = "Bit 4 - Set this bit to reset I2S0."]
    #[inline(always)]
    pub fn i2s0_rst(&mut self) -> I2S0_RST_W<PERIP_RST_EN0_SPEC> {
        I2S0_RST_W::new(self, 4)
    }
    #[doc = "Bit 5 - Set this bit to reset UART1."]
    #[inline(always)]
    pub fn uart1_rst(&mut self) -> UART1_RST_W<PERIP_RST_EN0_SPEC> {
        UART1_RST_W::new(self, 5)
    }
    #[doc = "Bit 6 - Set this bit to reset SPI2."]
    #[inline(always)]
    pub fn spi2_rst(&mut self) -> SPI2_RST_W<PERIP_RST_EN0_SPEC> {
        SPI2_RST_W::new(self, 6)
    }
    #[doc = "Bit 7 - Set this bit to reset I2C EXT0."]
    #[inline(always)]
    pub fn i2c_ext0_rst(&mut self) -> I2C_EXT0_RST_W<PERIP_RST_EN0_SPEC> {
        I2C_EXT0_RST_W::new(self, 7)
    }
    #[doc = "Bit 8 - Set this bit to reset UHCI0."]
    #[inline(always)]
    pub fn uhci0_rst(&mut self) -> UHCI0_RST_W<PERIP_RST_EN0_SPEC> {
        UHCI0_RST_W::new(self, 8)
    }
    #[doc = "Bit 9 - Set this bit to reset remote controller."]
    #[inline(always)]
    pub fn rmt_rst(&mut self) -> RMT_RST_W<PERIP_RST_EN0_SPEC> {
        RMT_RST_W::new(self, 9)
    }
    #[doc = "Bit 10 - Set this bit to reset pulse count."]
    #[inline(always)]
    pub fn pcnt_rst(&mut self) -> PCNT_RST_W<PERIP_RST_EN0_SPEC> {
        PCNT_RST_W::new(self, 10)
    }
    #[doc = "Bit 11 - Set this bit to reset LED PWM."]
    #[inline(always)]
    pub fn ledc_rst(&mut self) -> LEDC_RST_W<PERIP_RST_EN0_SPEC> {
        LEDC_RST_W::new(self, 11)
    }
    #[doc = "Bit 12 - Set this bit to reset UHCI1."]
    #[inline(always)]
    pub fn uhci1_rst(&mut self) -> UHCI1_RST_W<PERIP_RST_EN0_SPEC> {
        UHCI1_RST_W::new(self, 12)
    }
    #[doc = "Bit 13 - Set this bit to reset timer group0."]
    #[inline(always)]
    pub fn timergroup_rst(&mut self) -> TIMERGROUP_RST_W<PERIP_RST_EN0_SPEC> {
        TIMERGROUP_RST_W::new(self, 13)
    }
    #[doc = "Bit 14 - Set this bit to reset eFuse."]
    #[inline(always)]
    pub fn efuse_rst(&mut self) -> EFUSE_RST_W<PERIP_RST_EN0_SPEC> {
        EFUSE_RST_W::new(self, 14)
    }
    #[doc = "Bit 15 - Set this bit to reset timer group1."]
    #[inline(always)]
    pub fn timergroup1_rst(&mut self) -> TIMERGROUP1_RST_W<PERIP_RST_EN0_SPEC> {
        TIMERGROUP1_RST_W::new(self, 15)
    }
    #[doc = "Bit 16 - Set this bit to reset SPI3."]
    #[inline(always)]
    pub fn spi3_rst(&mut self) -> SPI3_RST_W<PERIP_RST_EN0_SPEC> {
        SPI3_RST_W::new(self, 16)
    }
    #[doc = "Bit 17 - Set this bit to reset PWM0."]
    #[inline(always)]
    pub fn pwm0_rst(&mut self) -> PWM0_RST_W<PERIP_RST_EN0_SPEC> {
        PWM0_RST_W::new(self, 17)
    }
    #[doc = "Bit 18 - Set this bit to reset I2C EXT1."]
    #[inline(always)]
    pub fn i2c_ext1_rst(&mut self) -> I2C_EXT1_RST_W<PERIP_RST_EN0_SPEC> {
        I2C_EXT1_RST_W::new(self, 18)
    }
    #[doc = "Bit 19 - Set this bit to reset CAN."]
    #[inline(always)]
    pub fn twai_rst(&mut self) -> TWAI_RST_W<PERIP_RST_EN0_SPEC> {
        TWAI_RST_W::new(self, 19)
    }
    #[doc = "Bit 20 - Set this bit to reset PWM1."]
    #[inline(always)]
    pub fn pwm1_rst(&mut self) -> PWM1_RST_W<PERIP_RST_EN0_SPEC> {
        PWM1_RST_W::new(self, 20)
    }
    #[doc = "Bit 21 - Set this bit to reset I2S1."]
    #[inline(always)]
    pub fn i2s1_rst(&mut self) -> I2S1_RST_W<PERIP_RST_EN0_SPEC> {
        I2S1_RST_W::new(self, 21)
    }
    #[doc = "Bit 22 - Set this bit to reset SPI2 DMA."]
    #[inline(always)]
    pub fn spi2_dma_rst(&mut self) -> SPI2_DMA_RST_W<PERIP_RST_EN0_SPEC> {
        SPI2_DMA_RST_W::new(self, 22)
    }
    #[doc = "Bit 23 - Set this bit to reset USB."]
    #[inline(always)]
    pub fn usb_rst(&mut self) -> USB_RST_W<PERIP_RST_EN0_SPEC> {
        USB_RST_W::new(self, 23)
    }
    #[doc = "Bit 24 - Set this bit to reset UART memory."]
    #[inline(always)]
    pub fn uart_mem_rst(&mut self) -> UART_MEM_RST_W<PERIP_RST_EN0_SPEC> {
        UART_MEM_RST_W::new(self, 24)
    }
    #[doc = "Bit 25 - Set this bit to reset PWM2."]
    #[inline(always)]
    pub fn pwm2_rst(&mut self) -> PWM2_RST_W<PERIP_RST_EN0_SPEC> {
        PWM2_RST_W::new(self, 25)
    }
    #[doc = "Bit 26 - Set this bit to reset PWM3."]
    #[inline(always)]
    pub fn pwm3_rst(&mut self) -> PWM3_RST_W<PERIP_RST_EN0_SPEC> {
        PWM3_RST_W::new(self, 26)
    }
    #[doc = "Bit 27 - Set this bit to reset SPI3 DMA."]
    #[inline(always)]
    pub fn spi3_dma_rst(&mut self) -> SPI3_DMA_RST_W<PERIP_RST_EN0_SPEC> {
        SPI3_DMA_RST_W::new(self, 27)
    }
    #[doc = "Bit 28 - Set this bit to reset SAR ADC."]
    #[inline(always)]
    pub fn apb_saradc_rst(&mut self) -> APB_SARADC_RST_W<PERIP_RST_EN0_SPEC> {
        APB_SARADC_RST_W::new(self, 28)
    }
    #[doc = "Bit 29 - Set this bit to reset system timer."]
    #[inline(always)]
    pub fn systimer_rst(&mut self) -> SYSTIMER_RST_W<PERIP_RST_EN0_SPEC> {
        SYSTIMER_RST_W::new(self, 29)
    }
    #[doc = "Bit 30 - Set this bit to reset aribiter of ADC2."]
    #[inline(always)]
    pub fn adc2_arb_rst(&mut self) -> ADC2_ARB_RST_W<PERIP_RST_EN0_SPEC> {
        ADC2_ARB_RST_W::new(self, 30)
    }
    #[doc = "Bit 31 - Set this bit to reset SPI4."]
    #[inline(always)]
    pub fn spi4_rst(&mut self) -> SPI4_RST_W<PERIP_RST_EN0_SPEC> {
        SPI4_RST_W::new(self, 31)
    }
}
#[doc = "System peripheral (hardware accelerators) reset register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`perip_rst_en0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`perip_rst_en0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PERIP_RST_EN0_SPEC;
impl crate::RegisterSpec for PERIP_RST_EN0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`perip_rst_en0::R`](R) reader structure"]
impl crate::Readable for PERIP_RST_EN0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`perip_rst_en0::W`](W) writer structure"]
impl crate::Writable for PERIP_RST_EN0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PERIP_RST_EN0 to value 0"]
impl crate::Resettable for PERIP_RST_EN0_SPEC {
    const RESET_VALUE: u32 = 0;
}
