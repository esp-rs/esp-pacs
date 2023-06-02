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
#[doc = "Field `TIMERS_CLK_EN` reader - reg_timers_clk_en"]
pub type TIMERS_CLK_EN_R = crate::BitReader;
#[doc = "Field `TIMERS_CLK_EN` writer - reg_timers_clk_en"]
pub type TIMERS_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, PERIP_CLK_EN0_SPEC, O>;
#[doc = "Field `SPI01_CLK_EN` reader - reg_spi01_clk_en"]
pub type SPI01_CLK_EN_R = crate::BitReader;
#[doc = "Field `SPI01_CLK_EN` writer - reg_spi01_clk_en"]
pub type SPI01_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, PERIP_CLK_EN0_SPEC, O>;
#[doc = "Field `UART_CLK_EN` reader - reg_uart_clk_en"]
pub type UART_CLK_EN_R = crate::BitReader;
#[doc = "Field `UART_CLK_EN` writer - reg_uart_clk_en"]
pub type UART_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, PERIP_CLK_EN0_SPEC, O>;
#[doc = "Field `WDG_CLK_EN` reader - reg_wdg_clk_en"]
pub type WDG_CLK_EN_R = crate::BitReader;
#[doc = "Field `WDG_CLK_EN` writer - reg_wdg_clk_en"]
pub type WDG_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, PERIP_CLK_EN0_SPEC, O>;
#[doc = "Field `I2S0_CLK_EN` reader - reg_i2s0_clk_en"]
pub type I2S0_CLK_EN_R = crate::BitReader;
#[doc = "Field `I2S0_CLK_EN` writer - reg_i2s0_clk_en"]
pub type I2S0_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, PERIP_CLK_EN0_SPEC, O>;
#[doc = "Field `UART1_CLK_EN` reader - reg_uart1_clk_en"]
pub type UART1_CLK_EN_R = crate::BitReader;
#[doc = "Field `UART1_CLK_EN` writer - reg_uart1_clk_en"]
pub type UART1_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, PERIP_CLK_EN0_SPEC, O>;
#[doc = "Field `SPI2_CLK_EN` reader - reg_spi2_clk_en"]
pub type SPI2_CLK_EN_R = crate::BitReader;
#[doc = "Field `SPI2_CLK_EN` writer - reg_spi2_clk_en"]
pub type SPI2_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, PERIP_CLK_EN0_SPEC, O>;
#[doc = "Field `I2C_EXT0_CLK_EN` reader - reg_ext0_clk_en"]
pub type I2C_EXT0_CLK_EN_R = crate::BitReader;
#[doc = "Field `I2C_EXT0_CLK_EN` writer - reg_ext0_clk_en"]
pub type I2C_EXT0_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, PERIP_CLK_EN0_SPEC, O>;
#[doc = "Field `UHCI0_CLK_EN` reader - reg_uhci0_clk_en"]
pub type UHCI0_CLK_EN_R = crate::BitReader;
#[doc = "Field `UHCI0_CLK_EN` writer - reg_uhci0_clk_en"]
pub type UHCI0_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, PERIP_CLK_EN0_SPEC, O>;
#[doc = "Field `RMT_CLK_EN` reader - reg_rmt_clk_en"]
pub type RMT_CLK_EN_R = crate::BitReader;
#[doc = "Field `RMT_CLK_EN` writer - reg_rmt_clk_en"]
pub type RMT_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, PERIP_CLK_EN0_SPEC, O>;
#[doc = "Field `PCNT_CLK_EN` reader - reg_pcnt_clk_en"]
pub type PCNT_CLK_EN_R = crate::BitReader;
#[doc = "Field `PCNT_CLK_EN` writer - reg_pcnt_clk_en"]
pub type PCNT_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, PERIP_CLK_EN0_SPEC, O>;
#[doc = "Field `LEDC_CLK_EN` reader - reg_ledc_clk_en"]
pub type LEDC_CLK_EN_R = crate::BitReader;
#[doc = "Field `LEDC_CLK_EN` writer - reg_ledc_clk_en"]
pub type LEDC_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, PERIP_CLK_EN0_SPEC, O>;
#[doc = "Field `UHCI1_CLK_EN` reader - reg_uhci1_clk_en"]
pub type UHCI1_CLK_EN_R = crate::BitReader;
#[doc = "Field `UHCI1_CLK_EN` writer - reg_uhci1_clk_en"]
pub type UHCI1_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, PERIP_CLK_EN0_SPEC, O>;
#[doc = "Field `TIMERGROUP_CLK_EN` reader - reg_timergroup_clk_en"]
pub type TIMERGROUP_CLK_EN_R = crate::BitReader;
#[doc = "Field `TIMERGROUP_CLK_EN` writer - reg_timergroup_clk_en"]
pub type TIMERGROUP_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, PERIP_CLK_EN0_SPEC, O>;
#[doc = "Field `EFUSE_CLK_EN` reader - reg_efuse_clk_en"]
pub type EFUSE_CLK_EN_R = crate::BitReader;
#[doc = "Field `EFUSE_CLK_EN` writer - reg_efuse_clk_en"]
pub type EFUSE_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, PERIP_CLK_EN0_SPEC, O>;
#[doc = "Field `TIMERGROUP1_CLK_EN` reader - reg_timergroup1_clk_en"]
pub type TIMERGROUP1_CLK_EN_R = crate::BitReader;
#[doc = "Field `TIMERGROUP1_CLK_EN` writer - reg_timergroup1_clk_en"]
pub type TIMERGROUP1_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, PERIP_CLK_EN0_SPEC, O>;
#[doc = "Field `SPI3_CLK_EN` reader - reg_spi3_clk_en"]
pub type SPI3_CLK_EN_R = crate::BitReader;
#[doc = "Field `SPI3_CLK_EN` writer - reg_spi3_clk_en"]
pub type SPI3_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, PERIP_CLK_EN0_SPEC, O>;
#[doc = "Field `PWM0_CLK_EN` reader - reg_pwm0_clk_en"]
pub type PWM0_CLK_EN_R = crate::BitReader;
#[doc = "Field `PWM0_CLK_EN` writer - reg_pwm0_clk_en"]
pub type PWM0_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, PERIP_CLK_EN0_SPEC, O>;
#[doc = "Field `EXT1_CLK_EN` reader - reg_ext1_clk_en"]
pub type EXT1_CLK_EN_R = crate::BitReader;
#[doc = "Field `EXT1_CLK_EN` writer - reg_ext1_clk_en"]
pub type EXT1_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, PERIP_CLK_EN0_SPEC, O>;
#[doc = "Field `TWAI_CLK_EN` reader - reg_can_clk_en"]
pub type TWAI_CLK_EN_R = crate::BitReader;
#[doc = "Field `TWAI_CLK_EN` writer - reg_can_clk_en"]
pub type TWAI_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, PERIP_CLK_EN0_SPEC, O>;
#[doc = "Field `PWM1_CLK_EN` reader - reg_pwm1_clk_en"]
pub type PWM1_CLK_EN_R = crate::BitReader;
#[doc = "Field `PWM1_CLK_EN` writer - reg_pwm1_clk_en"]
pub type PWM1_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, PERIP_CLK_EN0_SPEC, O>;
#[doc = "Field `I2S1_CLK_EN` reader - reg_i2s1_clk_en"]
pub type I2S1_CLK_EN_R = crate::BitReader;
#[doc = "Field `I2S1_CLK_EN` writer - reg_i2s1_clk_en"]
pub type I2S1_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, PERIP_CLK_EN0_SPEC, O>;
#[doc = "Field `SPI2_DMA_CLK_EN` reader - reg_spi2_dma_clk_en"]
pub type SPI2_DMA_CLK_EN_R = crate::BitReader;
#[doc = "Field `SPI2_DMA_CLK_EN` writer - reg_spi2_dma_clk_en"]
pub type SPI2_DMA_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, PERIP_CLK_EN0_SPEC, O>;
#[doc = "Field `USB_DEVICE_CLK_EN` reader - reg_usb_device_clk_en"]
pub type USB_DEVICE_CLK_EN_R = crate::BitReader;
#[doc = "Field `USB_DEVICE_CLK_EN` writer - reg_usb_device_clk_en"]
pub type USB_DEVICE_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, PERIP_CLK_EN0_SPEC, O>;
#[doc = "Field `UART_MEM_CLK_EN` reader - reg_uart_mem_clk_en"]
pub type UART_MEM_CLK_EN_R = crate::BitReader;
#[doc = "Field `UART_MEM_CLK_EN` writer - reg_uart_mem_clk_en"]
pub type UART_MEM_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, PERIP_CLK_EN0_SPEC, O>;
#[doc = "Field `PWM2_CLK_EN` reader - reg_pwm2_clk_en"]
pub type PWM2_CLK_EN_R = crate::BitReader;
#[doc = "Field `PWM2_CLK_EN` writer - reg_pwm2_clk_en"]
pub type PWM2_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, PERIP_CLK_EN0_SPEC, O>;
#[doc = "Field `PWM3_CLK_EN` reader - reg_pwm3_clk_en"]
pub type PWM3_CLK_EN_R = crate::BitReader;
#[doc = "Field `PWM3_CLK_EN` writer - reg_pwm3_clk_en"]
pub type PWM3_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, PERIP_CLK_EN0_SPEC, O>;
#[doc = "Field `SPI3_DMA_CLK_EN` reader - reg_spi3_dma_clk_en"]
pub type SPI3_DMA_CLK_EN_R = crate::BitReader;
#[doc = "Field `SPI3_DMA_CLK_EN` writer - reg_spi3_dma_clk_en"]
pub type SPI3_DMA_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, PERIP_CLK_EN0_SPEC, O>;
#[doc = "Field `APB_SARADC_CLK_EN` reader - reg_apb_saradc_clk_en"]
pub type APB_SARADC_CLK_EN_R = crate::BitReader;
#[doc = "Field `APB_SARADC_CLK_EN` writer - reg_apb_saradc_clk_en"]
pub type APB_SARADC_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, PERIP_CLK_EN0_SPEC, O>;
#[doc = "Field `SYSTIMER_CLK_EN` reader - reg_systimer_clk_en"]
pub type SYSTIMER_CLK_EN_R = crate::BitReader;
#[doc = "Field `SYSTIMER_CLK_EN` writer - reg_systimer_clk_en"]
pub type SYSTIMER_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, PERIP_CLK_EN0_SPEC, O>;
#[doc = "Field `ADC2_ARB_CLK_EN` reader - reg_adc2_arb_clk_en"]
pub type ADC2_ARB_CLK_EN_R = crate::BitReader;
#[doc = "Field `ADC2_ARB_CLK_EN` writer - reg_adc2_arb_clk_en"]
pub type ADC2_ARB_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, PERIP_CLK_EN0_SPEC, O>;
#[doc = "Field `SPI4_CLK_EN` reader - reg_spi4_clk_en"]
pub type SPI4_CLK_EN_R = crate::BitReader;
#[doc = "Field `SPI4_CLK_EN` writer - reg_spi4_clk_en"]
pub type SPI4_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, PERIP_CLK_EN0_SPEC, O>;
impl R {
    #[doc = "Bit 0 - reg_timers_clk_en"]
    #[inline(always)]
    pub fn timers_clk_en(&self) -> TIMERS_CLK_EN_R {
        TIMERS_CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reg_spi01_clk_en"]
    #[inline(always)]
    pub fn spi01_clk_en(&self) -> SPI01_CLK_EN_R {
        SPI01_CLK_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - reg_uart_clk_en"]
    #[inline(always)]
    pub fn uart_clk_en(&self) -> UART_CLK_EN_R {
        UART_CLK_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reg_wdg_clk_en"]
    #[inline(always)]
    pub fn wdg_clk_en(&self) -> WDG_CLK_EN_R {
        WDG_CLK_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - reg_i2s0_clk_en"]
    #[inline(always)]
    pub fn i2s0_clk_en(&self) -> I2S0_CLK_EN_R {
        I2S0_CLK_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - reg_uart1_clk_en"]
    #[inline(always)]
    pub fn uart1_clk_en(&self) -> UART1_CLK_EN_R {
        UART1_CLK_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - reg_spi2_clk_en"]
    #[inline(always)]
    pub fn spi2_clk_en(&self) -> SPI2_CLK_EN_R {
        SPI2_CLK_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - reg_ext0_clk_en"]
    #[inline(always)]
    pub fn i2c_ext0_clk_en(&self) -> I2C_EXT0_CLK_EN_R {
        I2C_EXT0_CLK_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - reg_uhci0_clk_en"]
    #[inline(always)]
    pub fn uhci0_clk_en(&self) -> UHCI0_CLK_EN_R {
        UHCI0_CLK_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - reg_rmt_clk_en"]
    #[inline(always)]
    pub fn rmt_clk_en(&self) -> RMT_CLK_EN_R {
        RMT_CLK_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - reg_pcnt_clk_en"]
    #[inline(always)]
    pub fn pcnt_clk_en(&self) -> PCNT_CLK_EN_R {
        PCNT_CLK_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - reg_ledc_clk_en"]
    #[inline(always)]
    pub fn ledc_clk_en(&self) -> LEDC_CLK_EN_R {
        LEDC_CLK_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - reg_uhci1_clk_en"]
    #[inline(always)]
    pub fn uhci1_clk_en(&self) -> UHCI1_CLK_EN_R {
        UHCI1_CLK_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - reg_timergroup_clk_en"]
    #[inline(always)]
    pub fn timergroup_clk_en(&self) -> TIMERGROUP_CLK_EN_R {
        TIMERGROUP_CLK_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - reg_efuse_clk_en"]
    #[inline(always)]
    pub fn efuse_clk_en(&self) -> EFUSE_CLK_EN_R {
        EFUSE_CLK_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - reg_timergroup1_clk_en"]
    #[inline(always)]
    pub fn timergroup1_clk_en(&self) -> TIMERGROUP1_CLK_EN_R {
        TIMERGROUP1_CLK_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - reg_spi3_clk_en"]
    #[inline(always)]
    pub fn spi3_clk_en(&self) -> SPI3_CLK_EN_R {
        SPI3_CLK_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - reg_pwm0_clk_en"]
    #[inline(always)]
    pub fn pwm0_clk_en(&self) -> PWM0_CLK_EN_R {
        PWM0_CLK_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - reg_ext1_clk_en"]
    #[inline(always)]
    pub fn ext1_clk_en(&self) -> EXT1_CLK_EN_R {
        EXT1_CLK_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - reg_can_clk_en"]
    #[inline(always)]
    pub fn twai_clk_en(&self) -> TWAI_CLK_EN_R {
        TWAI_CLK_EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - reg_pwm1_clk_en"]
    #[inline(always)]
    pub fn pwm1_clk_en(&self) -> PWM1_CLK_EN_R {
        PWM1_CLK_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - reg_i2s1_clk_en"]
    #[inline(always)]
    pub fn i2s1_clk_en(&self) -> I2S1_CLK_EN_R {
        I2S1_CLK_EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - reg_spi2_dma_clk_en"]
    #[inline(always)]
    pub fn spi2_dma_clk_en(&self) -> SPI2_DMA_CLK_EN_R {
        SPI2_DMA_CLK_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - reg_usb_device_clk_en"]
    #[inline(always)]
    pub fn usb_device_clk_en(&self) -> USB_DEVICE_CLK_EN_R {
        USB_DEVICE_CLK_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - reg_uart_mem_clk_en"]
    #[inline(always)]
    pub fn uart_mem_clk_en(&self) -> UART_MEM_CLK_EN_R {
        UART_MEM_CLK_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - reg_pwm2_clk_en"]
    #[inline(always)]
    pub fn pwm2_clk_en(&self) -> PWM2_CLK_EN_R {
        PWM2_CLK_EN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - reg_pwm3_clk_en"]
    #[inline(always)]
    pub fn pwm3_clk_en(&self) -> PWM3_CLK_EN_R {
        PWM3_CLK_EN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - reg_spi3_dma_clk_en"]
    #[inline(always)]
    pub fn spi3_dma_clk_en(&self) -> SPI3_DMA_CLK_EN_R {
        SPI3_DMA_CLK_EN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - reg_apb_saradc_clk_en"]
    #[inline(always)]
    pub fn apb_saradc_clk_en(&self) -> APB_SARADC_CLK_EN_R {
        APB_SARADC_CLK_EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - reg_systimer_clk_en"]
    #[inline(always)]
    pub fn systimer_clk_en(&self) -> SYSTIMER_CLK_EN_R {
        SYSTIMER_CLK_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - reg_adc2_arb_clk_en"]
    #[inline(always)]
    pub fn adc2_arb_clk_en(&self) -> ADC2_ARB_CLK_EN_R {
        ADC2_ARB_CLK_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - reg_spi4_clk_en"]
    #[inline(always)]
    pub fn spi4_clk_en(&self) -> SPI4_CLK_EN_R {
        SPI4_CLK_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PERIP_CLK_EN0")
            .field(
                "timers_clk_en",
                &format_args!("{}", self.timers_clk_en().bit()),
            )
            .field(
                "spi01_clk_en",
                &format_args!("{}", self.spi01_clk_en().bit()),
            )
            .field("uart_clk_en", &format_args!("{}", self.uart_clk_en().bit()))
            .field("wdg_clk_en", &format_args!("{}", self.wdg_clk_en().bit()))
            .field("i2s0_clk_en", &format_args!("{}", self.i2s0_clk_en().bit()))
            .field(
                "uart1_clk_en",
                &format_args!("{}", self.uart1_clk_en().bit()),
            )
            .field("spi2_clk_en", &format_args!("{}", self.spi2_clk_en().bit()))
            .field(
                "i2c_ext0_clk_en",
                &format_args!("{}", self.i2c_ext0_clk_en().bit()),
            )
            .field(
                "uhci0_clk_en",
                &format_args!("{}", self.uhci0_clk_en().bit()),
            )
            .field("rmt_clk_en", &format_args!("{}", self.rmt_clk_en().bit()))
            .field("pcnt_clk_en", &format_args!("{}", self.pcnt_clk_en().bit()))
            .field("ledc_clk_en", &format_args!("{}", self.ledc_clk_en().bit()))
            .field(
                "uhci1_clk_en",
                &format_args!("{}", self.uhci1_clk_en().bit()),
            )
            .field(
                "timergroup_clk_en",
                &format_args!("{}", self.timergroup_clk_en().bit()),
            )
            .field(
                "efuse_clk_en",
                &format_args!("{}", self.efuse_clk_en().bit()),
            )
            .field(
                "timergroup1_clk_en",
                &format_args!("{}", self.timergroup1_clk_en().bit()),
            )
            .field("spi3_clk_en", &format_args!("{}", self.spi3_clk_en().bit()))
            .field("pwm0_clk_en", &format_args!("{}", self.pwm0_clk_en().bit()))
            .field("ext1_clk_en", &format_args!("{}", self.ext1_clk_en().bit()))
            .field("twai_clk_en", &format_args!("{}", self.twai_clk_en().bit()))
            .field("pwm1_clk_en", &format_args!("{}", self.pwm1_clk_en().bit()))
            .field("i2s1_clk_en", &format_args!("{}", self.i2s1_clk_en().bit()))
            .field(
                "spi2_dma_clk_en",
                &format_args!("{}", self.spi2_dma_clk_en().bit()),
            )
            .field(
                "usb_device_clk_en",
                &format_args!("{}", self.usb_device_clk_en().bit()),
            )
            .field(
                "uart_mem_clk_en",
                &format_args!("{}", self.uart_mem_clk_en().bit()),
            )
            .field("pwm2_clk_en", &format_args!("{}", self.pwm2_clk_en().bit()))
            .field("pwm3_clk_en", &format_args!("{}", self.pwm3_clk_en().bit()))
            .field(
                "spi3_dma_clk_en",
                &format_args!("{}", self.spi3_dma_clk_en().bit()),
            )
            .field(
                "apb_saradc_clk_en",
                &format_args!("{}", self.apb_saradc_clk_en().bit()),
            )
            .field(
                "systimer_clk_en",
                &format_args!("{}", self.systimer_clk_en().bit()),
            )
            .field(
                "adc2_arb_clk_en",
                &format_args!("{}", self.adc2_arb_clk_en().bit()),
            )
            .field("spi4_clk_en", &format_args!("{}", self.spi4_clk_en().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PERIP_CLK_EN0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - reg_timers_clk_en"]
    #[inline(always)]
    #[must_use]
    pub fn timers_clk_en(&mut self) -> TIMERS_CLK_EN_W<0> {
        TIMERS_CLK_EN_W::new(self)
    }
    #[doc = "Bit 1 - reg_spi01_clk_en"]
    #[inline(always)]
    #[must_use]
    pub fn spi01_clk_en(&mut self) -> SPI01_CLK_EN_W<1> {
        SPI01_CLK_EN_W::new(self)
    }
    #[doc = "Bit 2 - reg_uart_clk_en"]
    #[inline(always)]
    #[must_use]
    pub fn uart_clk_en(&mut self) -> UART_CLK_EN_W<2> {
        UART_CLK_EN_W::new(self)
    }
    #[doc = "Bit 3 - reg_wdg_clk_en"]
    #[inline(always)]
    #[must_use]
    pub fn wdg_clk_en(&mut self) -> WDG_CLK_EN_W<3> {
        WDG_CLK_EN_W::new(self)
    }
    #[doc = "Bit 4 - reg_i2s0_clk_en"]
    #[inline(always)]
    #[must_use]
    pub fn i2s0_clk_en(&mut self) -> I2S0_CLK_EN_W<4> {
        I2S0_CLK_EN_W::new(self)
    }
    #[doc = "Bit 5 - reg_uart1_clk_en"]
    #[inline(always)]
    #[must_use]
    pub fn uart1_clk_en(&mut self) -> UART1_CLK_EN_W<5> {
        UART1_CLK_EN_W::new(self)
    }
    #[doc = "Bit 6 - reg_spi2_clk_en"]
    #[inline(always)]
    #[must_use]
    pub fn spi2_clk_en(&mut self) -> SPI2_CLK_EN_W<6> {
        SPI2_CLK_EN_W::new(self)
    }
    #[doc = "Bit 7 - reg_ext0_clk_en"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_ext0_clk_en(&mut self) -> I2C_EXT0_CLK_EN_W<7> {
        I2C_EXT0_CLK_EN_W::new(self)
    }
    #[doc = "Bit 8 - reg_uhci0_clk_en"]
    #[inline(always)]
    #[must_use]
    pub fn uhci0_clk_en(&mut self) -> UHCI0_CLK_EN_W<8> {
        UHCI0_CLK_EN_W::new(self)
    }
    #[doc = "Bit 9 - reg_rmt_clk_en"]
    #[inline(always)]
    #[must_use]
    pub fn rmt_clk_en(&mut self) -> RMT_CLK_EN_W<9> {
        RMT_CLK_EN_W::new(self)
    }
    #[doc = "Bit 10 - reg_pcnt_clk_en"]
    #[inline(always)]
    #[must_use]
    pub fn pcnt_clk_en(&mut self) -> PCNT_CLK_EN_W<10> {
        PCNT_CLK_EN_W::new(self)
    }
    #[doc = "Bit 11 - reg_ledc_clk_en"]
    #[inline(always)]
    #[must_use]
    pub fn ledc_clk_en(&mut self) -> LEDC_CLK_EN_W<11> {
        LEDC_CLK_EN_W::new(self)
    }
    #[doc = "Bit 12 - reg_uhci1_clk_en"]
    #[inline(always)]
    #[must_use]
    pub fn uhci1_clk_en(&mut self) -> UHCI1_CLK_EN_W<12> {
        UHCI1_CLK_EN_W::new(self)
    }
    #[doc = "Bit 13 - reg_timergroup_clk_en"]
    #[inline(always)]
    #[must_use]
    pub fn timergroup_clk_en(&mut self) -> TIMERGROUP_CLK_EN_W<13> {
        TIMERGROUP_CLK_EN_W::new(self)
    }
    #[doc = "Bit 14 - reg_efuse_clk_en"]
    #[inline(always)]
    #[must_use]
    pub fn efuse_clk_en(&mut self) -> EFUSE_CLK_EN_W<14> {
        EFUSE_CLK_EN_W::new(self)
    }
    #[doc = "Bit 15 - reg_timergroup1_clk_en"]
    #[inline(always)]
    #[must_use]
    pub fn timergroup1_clk_en(&mut self) -> TIMERGROUP1_CLK_EN_W<15> {
        TIMERGROUP1_CLK_EN_W::new(self)
    }
    #[doc = "Bit 16 - reg_spi3_clk_en"]
    #[inline(always)]
    #[must_use]
    pub fn spi3_clk_en(&mut self) -> SPI3_CLK_EN_W<16> {
        SPI3_CLK_EN_W::new(self)
    }
    #[doc = "Bit 17 - reg_pwm0_clk_en"]
    #[inline(always)]
    #[must_use]
    pub fn pwm0_clk_en(&mut self) -> PWM0_CLK_EN_W<17> {
        PWM0_CLK_EN_W::new(self)
    }
    #[doc = "Bit 18 - reg_ext1_clk_en"]
    #[inline(always)]
    #[must_use]
    pub fn ext1_clk_en(&mut self) -> EXT1_CLK_EN_W<18> {
        EXT1_CLK_EN_W::new(self)
    }
    #[doc = "Bit 19 - reg_can_clk_en"]
    #[inline(always)]
    #[must_use]
    pub fn twai_clk_en(&mut self) -> TWAI_CLK_EN_W<19> {
        TWAI_CLK_EN_W::new(self)
    }
    #[doc = "Bit 20 - reg_pwm1_clk_en"]
    #[inline(always)]
    #[must_use]
    pub fn pwm1_clk_en(&mut self) -> PWM1_CLK_EN_W<20> {
        PWM1_CLK_EN_W::new(self)
    }
    #[doc = "Bit 21 - reg_i2s1_clk_en"]
    #[inline(always)]
    #[must_use]
    pub fn i2s1_clk_en(&mut self) -> I2S1_CLK_EN_W<21> {
        I2S1_CLK_EN_W::new(self)
    }
    #[doc = "Bit 22 - reg_spi2_dma_clk_en"]
    #[inline(always)]
    #[must_use]
    pub fn spi2_dma_clk_en(&mut self) -> SPI2_DMA_CLK_EN_W<22> {
        SPI2_DMA_CLK_EN_W::new(self)
    }
    #[doc = "Bit 23 - reg_usb_device_clk_en"]
    #[inline(always)]
    #[must_use]
    pub fn usb_device_clk_en(&mut self) -> USB_DEVICE_CLK_EN_W<23> {
        USB_DEVICE_CLK_EN_W::new(self)
    }
    #[doc = "Bit 24 - reg_uart_mem_clk_en"]
    #[inline(always)]
    #[must_use]
    pub fn uart_mem_clk_en(&mut self) -> UART_MEM_CLK_EN_W<24> {
        UART_MEM_CLK_EN_W::new(self)
    }
    #[doc = "Bit 25 - reg_pwm2_clk_en"]
    #[inline(always)]
    #[must_use]
    pub fn pwm2_clk_en(&mut self) -> PWM2_CLK_EN_W<25> {
        PWM2_CLK_EN_W::new(self)
    }
    #[doc = "Bit 26 - reg_pwm3_clk_en"]
    #[inline(always)]
    #[must_use]
    pub fn pwm3_clk_en(&mut self) -> PWM3_CLK_EN_W<26> {
        PWM3_CLK_EN_W::new(self)
    }
    #[doc = "Bit 27 - reg_spi3_dma_clk_en"]
    #[inline(always)]
    #[must_use]
    pub fn spi3_dma_clk_en(&mut self) -> SPI3_DMA_CLK_EN_W<27> {
        SPI3_DMA_CLK_EN_W::new(self)
    }
    #[doc = "Bit 28 - reg_apb_saradc_clk_en"]
    #[inline(always)]
    #[must_use]
    pub fn apb_saradc_clk_en(&mut self) -> APB_SARADC_CLK_EN_W<28> {
        APB_SARADC_CLK_EN_W::new(self)
    }
    #[doc = "Bit 29 - reg_systimer_clk_en"]
    #[inline(always)]
    #[must_use]
    pub fn systimer_clk_en(&mut self) -> SYSTIMER_CLK_EN_W<29> {
        SYSTIMER_CLK_EN_W::new(self)
    }
    #[doc = "Bit 30 - reg_adc2_arb_clk_en"]
    #[inline(always)]
    #[must_use]
    pub fn adc2_arb_clk_en(&mut self) -> ADC2_ARB_CLK_EN_W<30> {
        ADC2_ARB_CLK_EN_W::new(self)
    }
    #[doc = "Bit 31 - reg_spi4_clk_en"]
    #[inline(always)]
    #[must_use]
    pub fn spi4_clk_en(&mut self) -> SPI4_CLK_EN_W<31> {
        SPI4_CLK_EN_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PERIP_CLK_EN0 to value 0xf9c1_e06f"]
impl crate::Resettable for PERIP_CLK_EN0_SPEC {
    const RESET_VALUE: Self::Ux = 0xf9c1_e06f;
}
