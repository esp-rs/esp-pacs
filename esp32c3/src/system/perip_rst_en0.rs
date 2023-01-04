#[doc = "Register `PERIP_RST_EN0` reader"]
pub struct R(crate::R<PERIP_RST_EN0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PERIP_RST_EN0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PERIP_RST_EN0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PERIP_RST_EN0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PERIP_RST_EN0` writer"]
pub struct W(crate::W<PERIP_RST_EN0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PERIP_RST_EN0_SPEC>;
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
impl From<crate::W<PERIP_RST_EN0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PERIP_RST_EN0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMERS_RST` reader - reg_timers_rst"]
pub type TIMERS_RST_R = crate::BitReader<bool>;
#[doc = "Field `TIMERS_RST` writer - reg_timers_rst"]
pub type TIMERS_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERIP_RST_EN0_SPEC, bool, O>;
#[doc = "Field `SPI01_RST` reader - reg_spi01_rst"]
pub type SPI01_RST_R = crate::BitReader<bool>;
#[doc = "Field `SPI01_RST` writer - reg_spi01_rst"]
pub type SPI01_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERIP_RST_EN0_SPEC, bool, O>;
#[doc = "Field `UART_RST` reader - reg_uart_rst"]
pub type UART_RST_R = crate::BitReader<bool>;
#[doc = "Field `UART_RST` writer - reg_uart_rst"]
pub type UART_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERIP_RST_EN0_SPEC, bool, O>;
#[doc = "Field `WDG_RST` reader - reg_wdg_rst"]
pub type WDG_RST_R = crate::BitReader<bool>;
#[doc = "Field `WDG_RST` writer - reg_wdg_rst"]
pub type WDG_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERIP_RST_EN0_SPEC, bool, O>;
#[doc = "Field `I2S0_RST` reader - reg_i2s0_rst"]
pub type I2S0_RST_R = crate::BitReader<bool>;
#[doc = "Field `I2S0_RST` writer - reg_i2s0_rst"]
pub type I2S0_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERIP_RST_EN0_SPEC, bool, O>;
#[doc = "Field `UART1_RST` reader - reg_uart1_rst"]
pub type UART1_RST_R = crate::BitReader<bool>;
#[doc = "Field `UART1_RST` writer - reg_uart1_rst"]
pub type UART1_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERIP_RST_EN0_SPEC, bool, O>;
#[doc = "Field `SPI2_RST` reader - reg_spi2_rst"]
pub type SPI2_RST_R = crate::BitReader<bool>;
#[doc = "Field `SPI2_RST` writer - reg_spi2_rst"]
pub type SPI2_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERIP_RST_EN0_SPEC, bool, O>;
#[doc = "Field `I2C_EXT0_RST` reader - reg_ext0_rst"]
pub type I2C_EXT0_RST_R = crate::BitReader<bool>;
#[doc = "Field `I2C_EXT0_RST` writer - reg_ext0_rst"]
pub type I2C_EXT0_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERIP_RST_EN0_SPEC, bool, O>;
#[doc = "Field `UHCI0_RST` reader - reg_uhci0_rst"]
pub type UHCI0_RST_R = crate::BitReader<bool>;
#[doc = "Field `UHCI0_RST` writer - reg_uhci0_rst"]
pub type UHCI0_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERIP_RST_EN0_SPEC, bool, O>;
#[doc = "Field `RMT_RST` reader - reg_rmt_rst"]
pub type RMT_RST_R = crate::BitReader<bool>;
#[doc = "Field `RMT_RST` writer - reg_rmt_rst"]
pub type RMT_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERIP_RST_EN0_SPEC, bool, O>;
#[doc = "Field `PCNT_RST` reader - reg_pcnt_rst"]
pub type PCNT_RST_R = crate::BitReader<bool>;
#[doc = "Field `PCNT_RST` writer - reg_pcnt_rst"]
pub type PCNT_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERIP_RST_EN0_SPEC, bool, O>;
#[doc = "Field `LEDC_RST` reader - reg_ledc_rst"]
pub type LEDC_RST_R = crate::BitReader<bool>;
#[doc = "Field `LEDC_RST` writer - reg_ledc_rst"]
pub type LEDC_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERIP_RST_EN0_SPEC, bool, O>;
#[doc = "Field `UHCI1_RST` reader - reg_uhci1_rst"]
pub type UHCI1_RST_R = crate::BitReader<bool>;
#[doc = "Field `UHCI1_RST` writer - reg_uhci1_rst"]
pub type UHCI1_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERIP_RST_EN0_SPEC, bool, O>;
#[doc = "Field `TIMERGROUP_RST` reader - reg_timergroup_rst"]
pub type TIMERGROUP_RST_R = crate::BitReader<bool>;
#[doc = "Field `TIMERGROUP_RST` writer - reg_timergroup_rst"]
pub type TIMERGROUP_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERIP_RST_EN0_SPEC, bool, O>;
#[doc = "Field `EFUSE_RST` reader - reg_efuse_rst"]
pub type EFUSE_RST_R = crate::BitReader<bool>;
#[doc = "Field `EFUSE_RST` writer - reg_efuse_rst"]
pub type EFUSE_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERIP_RST_EN0_SPEC, bool, O>;
#[doc = "Field `TIMERGROUP1_RST` reader - reg_timergroup1_rst"]
pub type TIMERGROUP1_RST_R = crate::BitReader<bool>;
#[doc = "Field `TIMERGROUP1_RST` writer - reg_timergroup1_rst"]
pub type TIMERGROUP1_RST_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PERIP_RST_EN0_SPEC, bool, O>;
#[doc = "Field `SPI3_RST` reader - reg_spi3_rst"]
pub type SPI3_RST_R = crate::BitReader<bool>;
#[doc = "Field `SPI3_RST` writer - reg_spi3_rst"]
pub type SPI3_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERIP_RST_EN0_SPEC, bool, O>;
#[doc = "Field `PWM0_RST` reader - reg_pwm0_rst"]
pub type PWM0_RST_R = crate::BitReader<bool>;
#[doc = "Field `PWM0_RST` writer - reg_pwm0_rst"]
pub type PWM0_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERIP_RST_EN0_SPEC, bool, O>;
#[doc = "Field `EXT1_RST` reader - reg_ext1_rst"]
pub type EXT1_RST_R = crate::BitReader<bool>;
#[doc = "Field `EXT1_RST` writer - reg_ext1_rst"]
pub type EXT1_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERIP_RST_EN0_SPEC, bool, O>;
#[doc = "Field `TWAI_RST` reader - reg_can_rst"]
pub type TWAI_RST_R = crate::BitReader<bool>;
#[doc = "Field `TWAI_RST` writer - reg_can_rst"]
pub type TWAI_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERIP_RST_EN0_SPEC, bool, O>;
#[doc = "Field `PWM1_RST` reader - reg_pwm1_rst"]
pub type PWM1_RST_R = crate::BitReader<bool>;
#[doc = "Field `PWM1_RST` writer - reg_pwm1_rst"]
pub type PWM1_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERIP_RST_EN0_SPEC, bool, O>;
#[doc = "Field `I2S1_RST` reader - reg_i2s1_rst"]
pub type I2S1_RST_R = crate::BitReader<bool>;
#[doc = "Field `I2S1_RST` writer - reg_i2s1_rst"]
pub type I2S1_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERIP_RST_EN0_SPEC, bool, O>;
#[doc = "Field `SPI2_DMA_RST` reader - reg_spi2_dma_rst"]
pub type SPI2_DMA_RST_R = crate::BitReader<bool>;
#[doc = "Field `SPI2_DMA_RST` writer - reg_spi2_dma_rst"]
pub type SPI2_DMA_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERIP_RST_EN0_SPEC, bool, O>;
#[doc = "Field `USB_DEVICE_RST` reader - reg_usb_device_rst"]
pub type USB_DEVICE_RST_R = crate::BitReader<bool>;
#[doc = "Field `USB_DEVICE_RST` writer - reg_usb_device_rst"]
pub type USB_DEVICE_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERIP_RST_EN0_SPEC, bool, O>;
#[doc = "Field `UART_MEM_RST` reader - reg_uart_mem_rst"]
pub type UART_MEM_RST_R = crate::BitReader<bool>;
#[doc = "Field `UART_MEM_RST` writer - reg_uart_mem_rst"]
pub type UART_MEM_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERIP_RST_EN0_SPEC, bool, O>;
#[doc = "Field `PWM2_RST` reader - reg_pwm2_rst"]
pub type PWM2_RST_R = crate::BitReader<bool>;
#[doc = "Field `PWM2_RST` writer - reg_pwm2_rst"]
pub type PWM2_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERIP_RST_EN0_SPEC, bool, O>;
#[doc = "Field `PWM3_RST` reader - reg_pwm3_rst"]
pub type PWM3_RST_R = crate::BitReader<bool>;
#[doc = "Field `PWM3_RST` writer - reg_pwm3_rst"]
pub type PWM3_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERIP_RST_EN0_SPEC, bool, O>;
#[doc = "Field `SPI3_DMA_RST` reader - reg_spi3_dma_rst"]
pub type SPI3_DMA_RST_R = crate::BitReader<bool>;
#[doc = "Field `SPI3_DMA_RST` writer - reg_spi3_dma_rst"]
pub type SPI3_DMA_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERIP_RST_EN0_SPEC, bool, O>;
#[doc = "Field `APB_SARADC_RST` reader - reg_apb_saradc_rst"]
pub type APB_SARADC_RST_R = crate::BitReader<bool>;
#[doc = "Field `APB_SARADC_RST` writer - reg_apb_saradc_rst"]
pub type APB_SARADC_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERIP_RST_EN0_SPEC, bool, O>;
#[doc = "Field `SYSTIMER_RST` reader - reg_systimer_rst"]
pub type SYSTIMER_RST_R = crate::BitReader<bool>;
#[doc = "Field `SYSTIMER_RST` writer - reg_systimer_rst"]
pub type SYSTIMER_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERIP_RST_EN0_SPEC, bool, O>;
#[doc = "Field `ADC2_ARB_RST` reader - reg_adc2_arb_rst"]
pub type ADC2_ARB_RST_R = crate::BitReader<bool>;
#[doc = "Field `ADC2_ARB_RST` writer - reg_adc2_arb_rst"]
pub type ADC2_ARB_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERIP_RST_EN0_SPEC, bool, O>;
#[doc = "Field `SPI4_RST` reader - reg_spi4_rst"]
pub type SPI4_RST_R = crate::BitReader<bool>;
#[doc = "Field `SPI4_RST` writer - reg_spi4_rst"]
pub type SPI4_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERIP_RST_EN0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - reg_timers_rst"]
    #[inline(always)]
    pub fn timers_rst(&self) -> TIMERS_RST_R {
        TIMERS_RST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reg_spi01_rst"]
    #[inline(always)]
    pub fn spi01_rst(&self) -> SPI01_RST_R {
        SPI01_RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - reg_uart_rst"]
    #[inline(always)]
    pub fn uart_rst(&self) -> UART_RST_R {
        UART_RST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reg_wdg_rst"]
    #[inline(always)]
    pub fn wdg_rst(&self) -> WDG_RST_R {
        WDG_RST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - reg_i2s0_rst"]
    #[inline(always)]
    pub fn i2s0_rst(&self) -> I2S0_RST_R {
        I2S0_RST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - reg_uart1_rst"]
    #[inline(always)]
    pub fn uart1_rst(&self) -> UART1_RST_R {
        UART1_RST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - reg_spi2_rst"]
    #[inline(always)]
    pub fn spi2_rst(&self) -> SPI2_RST_R {
        SPI2_RST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - reg_ext0_rst"]
    #[inline(always)]
    pub fn i2c_ext0_rst(&self) -> I2C_EXT0_RST_R {
        I2C_EXT0_RST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - reg_uhci0_rst"]
    #[inline(always)]
    pub fn uhci0_rst(&self) -> UHCI0_RST_R {
        UHCI0_RST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - reg_rmt_rst"]
    #[inline(always)]
    pub fn rmt_rst(&self) -> RMT_RST_R {
        RMT_RST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - reg_pcnt_rst"]
    #[inline(always)]
    pub fn pcnt_rst(&self) -> PCNT_RST_R {
        PCNT_RST_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - reg_ledc_rst"]
    #[inline(always)]
    pub fn ledc_rst(&self) -> LEDC_RST_R {
        LEDC_RST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - reg_uhci1_rst"]
    #[inline(always)]
    pub fn uhci1_rst(&self) -> UHCI1_RST_R {
        UHCI1_RST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - reg_timergroup_rst"]
    #[inline(always)]
    pub fn timergroup_rst(&self) -> TIMERGROUP_RST_R {
        TIMERGROUP_RST_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - reg_efuse_rst"]
    #[inline(always)]
    pub fn efuse_rst(&self) -> EFUSE_RST_R {
        EFUSE_RST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - reg_timergroup1_rst"]
    #[inline(always)]
    pub fn timergroup1_rst(&self) -> TIMERGROUP1_RST_R {
        TIMERGROUP1_RST_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - reg_spi3_rst"]
    #[inline(always)]
    pub fn spi3_rst(&self) -> SPI3_RST_R {
        SPI3_RST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - reg_pwm0_rst"]
    #[inline(always)]
    pub fn pwm0_rst(&self) -> PWM0_RST_R {
        PWM0_RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - reg_ext1_rst"]
    #[inline(always)]
    pub fn ext1_rst(&self) -> EXT1_RST_R {
        EXT1_RST_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - reg_can_rst"]
    #[inline(always)]
    pub fn twai_rst(&self) -> TWAI_RST_R {
        TWAI_RST_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - reg_pwm1_rst"]
    #[inline(always)]
    pub fn pwm1_rst(&self) -> PWM1_RST_R {
        PWM1_RST_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - reg_i2s1_rst"]
    #[inline(always)]
    pub fn i2s1_rst(&self) -> I2S1_RST_R {
        I2S1_RST_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - reg_spi2_dma_rst"]
    #[inline(always)]
    pub fn spi2_dma_rst(&self) -> SPI2_DMA_RST_R {
        SPI2_DMA_RST_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - reg_usb_device_rst"]
    #[inline(always)]
    pub fn usb_device_rst(&self) -> USB_DEVICE_RST_R {
        USB_DEVICE_RST_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - reg_uart_mem_rst"]
    #[inline(always)]
    pub fn uart_mem_rst(&self) -> UART_MEM_RST_R {
        UART_MEM_RST_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - reg_pwm2_rst"]
    #[inline(always)]
    pub fn pwm2_rst(&self) -> PWM2_RST_R {
        PWM2_RST_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - reg_pwm3_rst"]
    #[inline(always)]
    pub fn pwm3_rst(&self) -> PWM3_RST_R {
        PWM3_RST_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - reg_spi3_dma_rst"]
    #[inline(always)]
    pub fn spi3_dma_rst(&self) -> SPI3_DMA_RST_R {
        SPI3_DMA_RST_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - reg_apb_saradc_rst"]
    #[inline(always)]
    pub fn apb_saradc_rst(&self) -> APB_SARADC_RST_R {
        APB_SARADC_RST_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - reg_systimer_rst"]
    #[inline(always)]
    pub fn systimer_rst(&self) -> SYSTIMER_RST_R {
        SYSTIMER_RST_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - reg_adc2_arb_rst"]
    #[inline(always)]
    pub fn adc2_arb_rst(&self) -> ADC2_ARB_RST_R {
        ADC2_ARB_RST_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - reg_spi4_rst"]
    #[inline(always)]
    pub fn spi4_rst(&self) -> SPI4_RST_R {
        SPI4_RST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - reg_timers_rst"]
    #[inline(always)]
    #[must_use]
    pub fn timers_rst(&mut self) -> TIMERS_RST_W<0> {
        TIMERS_RST_W::new(self)
    }
    #[doc = "Bit 1 - reg_spi01_rst"]
    #[inline(always)]
    #[must_use]
    pub fn spi01_rst(&mut self) -> SPI01_RST_W<1> {
        SPI01_RST_W::new(self)
    }
    #[doc = "Bit 2 - reg_uart_rst"]
    #[inline(always)]
    #[must_use]
    pub fn uart_rst(&mut self) -> UART_RST_W<2> {
        UART_RST_W::new(self)
    }
    #[doc = "Bit 3 - reg_wdg_rst"]
    #[inline(always)]
    #[must_use]
    pub fn wdg_rst(&mut self) -> WDG_RST_W<3> {
        WDG_RST_W::new(self)
    }
    #[doc = "Bit 4 - reg_i2s0_rst"]
    #[inline(always)]
    #[must_use]
    pub fn i2s0_rst(&mut self) -> I2S0_RST_W<4> {
        I2S0_RST_W::new(self)
    }
    #[doc = "Bit 5 - reg_uart1_rst"]
    #[inline(always)]
    #[must_use]
    pub fn uart1_rst(&mut self) -> UART1_RST_W<5> {
        UART1_RST_W::new(self)
    }
    #[doc = "Bit 6 - reg_spi2_rst"]
    #[inline(always)]
    #[must_use]
    pub fn spi2_rst(&mut self) -> SPI2_RST_W<6> {
        SPI2_RST_W::new(self)
    }
    #[doc = "Bit 7 - reg_ext0_rst"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_ext0_rst(&mut self) -> I2C_EXT0_RST_W<7> {
        I2C_EXT0_RST_W::new(self)
    }
    #[doc = "Bit 8 - reg_uhci0_rst"]
    #[inline(always)]
    #[must_use]
    pub fn uhci0_rst(&mut self) -> UHCI0_RST_W<8> {
        UHCI0_RST_W::new(self)
    }
    #[doc = "Bit 9 - reg_rmt_rst"]
    #[inline(always)]
    #[must_use]
    pub fn rmt_rst(&mut self) -> RMT_RST_W<9> {
        RMT_RST_W::new(self)
    }
    #[doc = "Bit 10 - reg_pcnt_rst"]
    #[inline(always)]
    #[must_use]
    pub fn pcnt_rst(&mut self) -> PCNT_RST_W<10> {
        PCNT_RST_W::new(self)
    }
    #[doc = "Bit 11 - reg_ledc_rst"]
    #[inline(always)]
    #[must_use]
    pub fn ledc_rst(&mut self) -> LEDC_RST_W<11> {
        LEDC_RST_W::new(self)
    }
    #[doc = "Bit 12 - reg_uhci1_rst"]
    #[inline(always)]
    #[must_use]
    pub fn uhci1_rst(&mut self) -> UHCI1_RST_W<12> {
        UHCI1_RST_W::new(self)
    }
    #[doc = "Bit 13 - reg_timergroup_rst"]
    #[inline(always)]
    #[must_use]
    pub fn timergroup_rst(&mut self) -> TIMERGROUP_RST_W<13> {
        TIMERGROUP_RST_W::new(self)
    }
    #[doc = "Bit 14 - reg_efuse_rst"]
    #[inline(always)]
    #[must_use]
    pub fn efuse_rst(&mut self) -> EFUSE_RST_W<14> {
        EFUSE_RST_W::new(self)
    }
    #[doc = "Bit 15 - reg_timergroup1_rst"]
    #[inline(always)]
    #[must_use]
    pub fn timergroup1_rst(&mut self) -> TIMERGROUP1_RST_W<15> {
        TIMERGROUP1_RST_W::new(self)
    }
    #[doc = "Bit 16 - reg_spi3_rst"]
    #[inline(always)]
    #[must_use]
    pub fn spi3_rst(&mut self) -> SPI3_RST_W<16> {
        SPI3_RST_W::new(self)
    }
    #[doc = "Bit 17 - reg_pwm0_rst"]
    #[inline(always)]
    #[must_use]
    pub fn pwm0_rst(&mut self) -> PWM0_RST_W<17> {
        PWM0_RST_W::new(self)
    }
    #[doc = "Bit 18 - reg_ext1_rst"]
    #[inline(always)]
    #[must_use]
    pub fn ext1_rst(&mut self) -> EXT1_RST_W<18> {
        EXT1_RST_W::new(self)
    }
    #[doc = "Bit 19 - reg_can_rst"]
    #[inline(always)]
    #[must_use]
    pub fn twai_rst(&mut self) -> TWAI_RST_W<19> {
        TWAI_RST_W::new(self)
    }
    #[doc = "Bit 20 - reg_pwm1_rst"]
    #[inline(always)]
    #[must_use]
    pub fn pwm1_rst(&mut self) -> PWM1_RST_W<20> {
        PWM1_RST_W::new(self)
    }
    #[doc = "Bit 21 - reg_i2s1_rst"]
    #[inline(always)]
    #[must_use]
    pub fn i2s1_rst(&mut self) -> I2S1_RST_W<21> {
        I2S1_RST_W::new(self)
    }
    #[doc = "Bit 22 - reg_spi2_dma_rst"]
    #[inline(always)]
    #[must_use]
    pub fn spi2_dma_rst(&mut self) -> SPI2_DMA_RST_W<22> {
        SPI2_DMA_RST_W::new(self)
    }
    #[doc = "Bit 23 - reg_usb_device_rst"]
    #[inline(always)]
    #[must_use]
    pub fn usb_device_rst(&mut self) -> USB_DEVICE_RST_W<23> {
        USB_DEVICE_RST_W::new(self)
    }
    #[doc = "Bit 24 - reg_uart_mem_rst"]
    #[inline(always)]
    #[must_use]
    pub fn uart_mem_rst(&mut self) -> UART_MEM_RST_W<24> {
        UART_MEM_RST_W::new(self)
    }
    #[doc = "Bit 25 - reg_pwm2_rst"]
    #[inline(always)]
    #[must_use]
    pub fn pwm2_rst(&mut self) -> PWM2_RST_W<25> {
        PWM2_RST_W::new(self)
    }
    #[doc = "Bit 26 - reg_pwm3_rst"]
    #[inline(always)]
    #[must_use]
    pub fn pwm3_rst(&mut self) -> PWM3_RST_W<26> {
        PWM3_RST_W::new(self)
    }
    #[doc = "Bit 27 - reg_spi3_dma_rst"]
    #[inline(always)]
    #[must_use]
    pub fn spi3_dma_rst(&mut self) -> SPI3_DMA_RST_W<27> {
        SPI3_DMA_RST_W::new(self)
    }
    #[doc = "Bit 28 - reg_apb_saradc_rst"]
    #[inline(always)]
    #[must_use]
    pub fn apb_saradc_rst(&mut self) -> APB_SARADC_RST_W<28> {
        APB_SARADC_RST_W::new(self)
    }
    #[doc = "Bit 29 - reg_systimer_rst"]
    #[inline(always)]
    #[must_use]
    pub fn systimer_rst(&mut self) -> SYSTIMER_RST_W<29> {
        SYSTIMER_RST_W::new(self)
    }
    #[doc = "Bit 30 - reg_adc2_arb_rst"]
    #[inline(always)]
    #[must_use]
    pub fn adc2_arb_rst(&mut self) -> ADC2_ARB_RST_W<30> {
        ADC2_ARB_RST_W::new(self)
    }
    #[doc = "Bit 31 - reg_spi4_rst"]
    #[inline(always)]
    #[must_use]
    pub fn spi4_rst(&mut self) -> SPI4_RST_W<31> {
        SPI4_RST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "reserved\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [perip_rst_en0](index.html) module"]
pub struct PERIP_RST_EN0_SPEC;
impl crate::RegisterSpec for PERIP_RST_EN0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [perip_rst_en0::R](R) reader structure"]
impl crate::Readable for PERIP_RST_EN0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [perip_rst_en0::W](W) writer structure"]
impl crate::Writable for PERIP_RST_EN0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PERIP_RST_EN0 to value 0"]
impl crate::Resettable for PERIP_RST_EN0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
