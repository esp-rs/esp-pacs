#[doc = "Register `PERIP_RST_EN` reader"]
pub struct R(crate::R<PERIP_RST_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PERIP_RST_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PERIP_RST_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PERIP_RST_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PERIP_RST_EN` writer"]
pub struct W(crate::W<PERIP_RST_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PERIP_RST_EN_SPEC>;
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
impl From<crate::W<PERIP_RST_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PERIP_RST_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMERS_RST` reader - "]
pub type TIMERS_RST_R = crate::BitReader;
#[doc = "Field `TIMERS_RST` writer - "]
pub type TIMERS_RST_W<'a, const O: u8> = crate::BitWriter<'a, PERIP_RST_EN_SPEC, O>;
#[doc = "Field `SPI01_RST` reader - "]
pub type SPI01_RST_R = crate::BitReader;
#[doc = "Field `SPI01_RST` writer - "]
pub type SPI01_RST_W<'a, const O: u8> = crate::BitWriter<'a, PERIP_RST_EN_SPEC, O>;
#[doc = "Field `UART_RST` reader - "]
pub type UART_RST_R = crate::BitReader;
#[doc = "Field `UART_RST` writer - "]
pub type UART_RST_W<'a, const O: u8> = crate::BitWriter<'a, PERIP_RST_EN_SPEC, O>;
#[doc = "Field `WDG_RST` reader - "]
pub type WDG_RST_R = crate::BitReader;
#[doc = "Field `WDG_RST` writer - "]
pub type WDG_RST_W<'a, const O: u8> = crate::BitWriter<'a, PERIP_RST_EN_SPEC, O>;
#[doc = "Field `I2S0_RST` reader - "]
pub type I2S0_RST_R = crate::BitReader;
#[doc = "Field `I2S0_RST` writer - "]
pub type I2S0_RST_W<'a, const O: u8> = crate::BitWriter<'a, PERIP_RST_EN_SPEC, O>;
#[doc = "Field `UART1_RST` reader - "]
pub type UART1_RST_R = crate::BitReader;
#[doc = "Field `UART1_RST` writer - "]
pub type UART1_RST_W<'a, const O: u8> = crate::BitWriter<'a, PERIP_RST_EN_SPEC, O>;
#[doc = "Field `SPI2_RST` reader - "]
pub type SPI2_RST_R = crate::BitReader;
#[doc = "Field `SPI2_RST` writer - "]
pub type SPI2_RST_W<'a, const O: u8> = crate::BitWriter<'a, PERIP_RST_EN_SPEC, O>;
#[doc = "Field `I2C0_EXT0_RST` reader - "]
pub type I2C0_EXT0_RST_R = crate::BitReader;
#[doc = "Field `I2C0_EXT0_RST` writer - "]
pub type I2C0_EXT0_RST_W<'a, const O: u8> = crate::BitWriter<'a, PERIP_RST_EN_SPEC, O>;
#[doc = "Field `UHCI0_RST` reader - "]
pub type UHCI0_RST_R = crate::BitReader;
#[doc = "Field `UHCI0_RST` writer - "]
pub type UHCI0_RST_W<'a, const O: u8> = crate::BitWriter<'a, PERIP_RST_EN_SPEC, O>;
#[doc = "Field `RMT_RST` reader - "]
pub type RMT_RST_R = crate::BitReader;
#[doc = "Field `RMT_RST` writer - "]
pub type RMT_RST_W<'a, const O: u8> = crate::BitWriter<'a, PERIP_RST_EN_SPEC, O>;
#[doc = "Field `PCNT_RST` reader - "]
pub type PCNT_RST_R = crate::BitReader;
#[doc = "Field `PCNT_RST` writer - "]
pub type PCNT_RST_W<'a, const O: u8> = crate::BitWriter<'a, PERIP_RST_EN_SPEC, O>;
#[doc = "Field `LEDC_RST` reader - "]
pub type LEDC_RST_R = crate::BitReader;
#[doc = "Field `LEDC_RST` writer - "]
pub type LEDC_RST_W<'a, const O: u8> = crate::BitWriter<'a, PERIP_RST_EN_SPEC, O>;
#[doc = "Field `UHCI1_RST` reader - "]
pub type UHCI1_RST_R = crate::BitReader;
#[doc = "Field `UHCI1_RST` writer - "]
pub type UHCI1_RST_W<'a, const O: u8> = crate::BitWriter<'a, PERIP_RST_EN_SPEC, O>;
#[doc = "Field `TIMERGROUP_RST` reader - "]
pub type TIMERGROUP_RST_R = crate::BitReader;
#[doc = "Field `TIMERGROUP_RST` writer - "]
pub type TIMERGROUP_RST_W<'a, const O: u8> = crate::BitWriter<'a, PERIP_RST_EN_SPEC, O>;
#[doc = "Field `EFUSE_RST` reader - "]
pub type EFUSE_RST_R = crate::BitReader;
#[doc = "Field `EFUSE_RST` writer - "]
pub type EFUSE_RST_W<'a, const O: u8> = crate::BitWriter<'a, PERIP_RST_EN_SPEC, O>;
#[doc = "Field `TIMERGROUP1_RST` reader - "]
pub type TIMERGROUP1_RST_R = crate::BitReader;
#[doc = "Field `TIMERGROUP1_RST` writer - "]
pub type TIMERGROUP1_RST_W<'a, const O: u8> = crate::BitWriter<'a, PERIP_RST_EN_SPEC, O>;
#[doc = "Field `SPI3_RST` reader - "]
pub type SPI3_RST_R = crate::BitReader;
#[doc = "Field `SPI3_RST` writer - "]
pub type SPI3_RST_W<'a, const O: u8> = crate::BitWriter<'a, PERIP_RST_EN_SPEC, O>;
#[doc = "Field `PWM0_RST` reader - "]
pub type PWM0_RST_R = crate::BitReader;
#[doc = "Field `PWM0_RST` writer - "]
pub type PWM0_RST_W<'a, const O: u8> = crate::BitWriter<'a, PERIP_RST_EN_SPEC, O>;
#[doc = "Field `I2C_EXT1_RST` reader - "]
pub type I2C_EXT1_RST_R = crate::BitReader;
#[doc = "Field `I2C_EXT1_RST` writer - "]
pub type I2C_EXT1_RST_W<'a, const O: u8> = crate::BitWriter<'a, PERIP_RST_EN_SPEC, O>;
#[doc = "Field `TWAI_RST` reader - "]
pub type TWAI_RST_R = crate::BitReader;
#[doc = "Field `TWAI_RST` writer - "]
pub type TWAI_RST_W<'a, const O: u8> = crate::BitWriter<'a, PERIP_RST_EN_SPEC, O>;
#[doc = "Field `PWM1_RST` reader - "]
pub type PWM1_RST_R = crate::BitReader;
#[doc = "Field `PWM1_RST` writer - "]
pub type PWM1_RST_W<'a, const O: u8> = crate::BitWriter<'a, PERIP_RST_EN_SPEC, O>;
#[doc = "Field `I2S1_RST` reader - "]
pub type I2S1_RST_R = crate::BitReader;
#[doc = "Field `I2S1_RST` writer - "]
pub type I2S1_RST_W<'a, const O: u8> = crate::BitWriter<'a, PERIP_RST_EN_SPEC, O>;
#[doc = "Field `SPI_DMA_RST` reader - "]
pub type SPI_DMA_RST_R = crate::BitReader;
#[doc = "Field `SPI_DMA_RST` writer - "]
pub type SPI_DMA_RST_W<'a, const O: u8> = crate::BitWriter<'a, PERIP_RST_EN_SPEC, O>;
#[doc = "Field `UART2_RST` reader - "]
pub type UART2_RST_R = crate::BitReader;
#[doc = "Field `UART2_RST` writer - "]
pub type UART2_RST_W<'a, const O: u8> = crate::BitWriter<'a, PERIP_RST_EN_SPEC, O>;
#[doc = "Field `UART_MEM_RST` reader - "]
pub type UART_MEM_RST_R = crate::BitReader;
#[doc = "Field `UART_MEM_RST` writer - "]
pub type UART_MEM_RST_W<'a, const O: u8> = crate::BitWriter<'a, PERIP_RST_EN_SPEC, O>;
#[doc = "Field `PWM2_RST` reader - "]
pub type PWM2_RST_R = crate::BitReader;
#[doc = "Field `PWM2_RST` writer - "]
pub type PWM2_RST_W<'a, const O: u8> = crate::BitWriter<'a, PERIP_RST_EN_SPEC, O>;
#[doc = "Field `PWM3_RST` reader - "]
pub type PWM3_RST_R = crate::BitReader;
#[doc = "Field `PWM3_RST` writer - "]
pub type PWM3_RST_W<'a, const O: u8> = crate::BitWriter<'a, PERIP_RST_EN_SPEC, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn timers_rst(&self) -> TIMERS_RST_R {
        TIMERS_RST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn spi01_rst(&self) -> SPI01_RST_R {
        SPI01_RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn uart_rst(&self) -> UART_RST_R {
        UART_RST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn wdg_rst(&self) -> WDG_RST_R {
        WDG_RST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn i2s0_rst(&self) -> I2S0_RST_R {
        I2S0_RST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn uart1_rst(&self) -> UART1_RST_R {
        UART1_RST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn spi2_rst(&self) -> SPI2_RST_R {
        SPI2_RST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn i2c0_ext0_rst(&self) -> I2C0_EXT0_RST_R {
        I2C0_EXT0_RST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn uhci0_rst(&self) -> UHCI0_RST_R {
        UHCI0_RST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn rmt_rst(&self) -> RMT_RST_R {
        RMT_RST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn pcnt_rst(&self) -> PCNT_RST_R {
        PCNT_RST_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ledc_rst(&self) -> LEDC_RST_R {
        LEDC_RST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn uhci1_rst(&self) -> UHCI1_RST_R {
        UHCI1_RST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn timergroup_rst(&self) -> TIMERGROUP_RST_R {
        TIMERGROUP_RST_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn efuse_rst(&self) -> EFUSE_RST_R {
        EFUSE_RST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn timergroup1_rst(&self) -> TIMERGROUP1_RST_R {
        TIMERGROUP1_RST_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn spi3_rst(&self) -> SPI3_RST_R {
        SPI3_RST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn pwm0_rst(&self) -> PWM0_RST_R {
        PWM0_RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn i2c_ext1_rst(&self) -> I2C_EXT1_RST_R {
        I2C_EXT1_RST_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn twai_rst(&self) -> TWAI_RST_R {
        TWAI_RST_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn pwm1_rst(&self) -> PWM1_RST_R {
        PWM1_RST_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn i2s1_rst(&self) -> I2S1_RST_R {
        I2S1_RST_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn spi_dma_rst(&self) -> SPI_DMA_RST_R {
        SPI_DMA_RST_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn uart2_rst(&self) -> UART2_RST_R {
        UART2_RST_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn uart_mem_rst(&self) -> UART_MEM_RST_R {
        UART_MEM_RST_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn pwm2_rst(&self) -> PWM2_RST_R {
        PWM2_RST_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn pwm3_rst(&self) -> PWM3_RST_R {
        PWM3_RST_R::new(((self.bits >> 26) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PERIP_RST_EN")
            .field("timers_rst", &format_args!("{}", self.timers_rst().bit()))
            .field("spi01_rst", &format_args!("{}", self.spi01_rst().bit()))
            .field("uart_rst", &format_args!("{}", self.uart_rst().bit()))
            .field("wdg_rst", &format_args!("{}", self.wdg_rst().bit()))
            .field("i2s0_rst", &format_args!("{}", self.i2s0_rst().bit()))
            .field("uart1_rst", &format_args!("{}", self.uart1_rst().bit()))
            .field("spi2_rst", &format_args!("{}", self.spi2_rst().bit()))
            .field(
                "i2c0_ext0_rst",
                &format_args!("{}", self.i2c0_ext0_rst().bit()),
            )
            .field("uhci0_rst", &format_args!("{}", self.uhci0_rst().bit()))
            .field("rmt_rst", &format_args!("{}", self.rmt_rst().bit()))
            .field("pcnt_rst", &format_args!("{}", self.pcnt_rst().bit()))
            .field("ledc_rst", &format_args!("{}", self.ledc_rst().bit()))
            .field("uhci1_rst", &format_args!("{}", self.uhci1_rst().bit()))
            .field(
                "timergroup_rst",
                &format_args!("{}", self.timergroup_rst().bit()),
            )
            .field("efuse_rst", &format_args!("{}", self.efuse_rst().bit()))
            .field(
                "timergroup1_rst",
                &format_args!("{}", self.timergroup1_rst().bit()),
            )
            .field("spi3_rst", &format_args!("{}", self.spi3_rst().bit()))
            .field("pwm0_rst", &format_args!("{}", self.pwm0_rst().bit()))
            .field(
                "i2c_ext1_rst",
                &format_args!("{}", self.i2c_ext1_rst().bit()),
            )
            .field("twai_rst", &format_args!("{}", self.twai_rst().bit()))
            .field("pwm1_rst", &format_args!("{}", self.pwm1_rst().bit()))
            .field("i2s1_rst", &format_args!("{}", self.i2s1_rst().bit()))
            .field("spi_dma_rst", &format_args!("{}", self.spi_dma_rst().bit()))
            .field("uart2_rst", &format_args!("{}", self.uart2_rst().bit()))
            .field(
                "uart_mem_rst",
                &format_args!("{}", self.uart_mem_rst().bit()),
            )
            .field("pwm2_rst", &format_args!("{}", self.pwm2_rst().bit()))
            .field("pwm3_rst", &format_args!("{}", self.pwm3_rst().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PERIP_RST_EN_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn timers_rst(&mut self) -> TIMERS_RST_W<0> {
        TIMERS_RST_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn spi01_rst(&mut self) -> SPI01_RST_W<1> {
        SPI01_RST_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn uart_rst(&mut self) -> UART_RST_W<2> {
        UART_RST_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn wdg_rst(&mut self) -> WDG_RST_W<3> {
        WDG_RST_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn i2s0_rst(&mut self) -> I2S0_RST_W<4> {
        I2S0_RST_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn uart1_rst(&mut self) -> UART1_RST_W<5> {
        UART1_RST_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn spi2_rst(&mut self) -> SPI2_RST_W<6> {
        SPI2_RST_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn i2c0_ext0_rst(&mut self) -> I2C0_EXT0_RST_W<7> {
        I2C0_EXT0_RST_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn uhci0_rst(&mut self) -> UHCI0_RST_W<8> {
        UHCI0_RST_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn rmt_rst(&mut self) -> RMT_RST_W<9> {
        RMT_RST_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn pcnt_rst(&mut self) -> PCNT_RST_W<10> {
        PCNT_RST_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn ledc_rst(&mut self) -> LEDC_RST_W<11> {
        LEDC_RST_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn uhci1_rst(&mut self) -> UHCI1_RST_W<12> {
        UHCI1_RST_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn timergroup_rst(&mut self) -> TIMERGROUP_RST_W<13> {
        TIMERGROUP_RST_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn efuse_rst(&mut self) -> EFUSE_RST_W<14> {
        EFUSE_RST_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn timergroup1_rst(&mut self) -> TIMERGROUP1_RST_W<15> {
        TIMERGROUP1_RST_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn spi3_rst(&mut self) -> SPI3_RST_W<16> {
        SPI3_RST_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn pwm0_rst(&mut self) -> PWM0_RST_W<17> {
        PWM0_RST_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_ext1_rst(&mut self) -> I2C_EXT1_RST_W<18> {
        I2C_EXT1_RST_W::new(self)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn twai_rst(&mut self) -> TWAI_RST_W<19> {
        TWAI_RST_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn pwm1_rst(&mut self) -> PWM1_RST_W<20> {
        PWM1_RST_W::new(self)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn i2s1_rst(&mut self) -> I2S1_RST_W<21> {
        I2S1_RST_W::new(self)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn spi_dma_rst(&mut self) -> SPI_DMA_RST_W<22> {
        SPI_DMA_RST_W::new(self)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn uart2_rst(&mut self) -> UART2_RST_W<23> {
        UART2_RST_W::new(self)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn uart_mem_rst(&mut self) -> UART_MEM_RST_W<24> {
        UART_MEM_RST_W::new(self)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn pwm2_rst(&mut self) -> PWM2_RST_W<25> {
        PWM2_RST_W::new(self)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn pwm3_rst(&mut self) -> PWM3_RST_W<26> {
        PWM3_RST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [perip_rst_en](index.html) module"]
pub struct PERIP_RST_EN_SPEC;
impl crate::RegisterSpec for PERIP_RST_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [perip_rst_en::R](R) reader structure"]
impl crate::Readable for PERIP_RST_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [perip_rst_en::W](W) writer structure"]
impl crate::Writable for PERIP_RST_EN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PERIP_RST_EN to value 0"]
impl crate::Resettable for PERIP_RST_EN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
