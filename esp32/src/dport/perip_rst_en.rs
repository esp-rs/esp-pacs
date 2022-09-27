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
#[doc = "Field `SPI01_RST` reader - SPI0 and SPI1 module."]
pub type SPI01_RST_R = crate::BitReader<bool>;
#[doc = "Field `SPI01_RST` writer - SPI0 and SPI1 module."]
pub type SPI01_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERIP_RST_EN_SPEC, bool, O>;
#[doc = "Field `UART_RST` reader - UART0 module."]
pub type UART_RST_R = crate::BitReader<bool>;
#[doc = "Field `UART_RST` writer - UART0 module."]
pub type UART_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERIP_RST_EN_SPEC, bool, O>;
#[doc = "Field `I2S0_RST` reader - I2S0 module."]
pub type I2S0_RST_R = crate::BitReader<bool>;
#[doc = "Field `I2S0_RST` writer - I2S0 module."]
pub type I2S0_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERIP_RST_EN_SPEC, bool, O>;
#[doc = "Field `UART1_RST` reader - UART1 module."]
pub type UART1_RST_R = crate::BitReader<bool>;
#[doc = "Field `UART1_RST` writer - UART1 module."]
pub type UART1_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERIP_RST_EN_SPEC, bool, O>;
#[doc = "Field `SPI2_RST` reader - SPI2 module."]
pub type SPI2_RST_R = crate::BitReader<bool>;
#[doc = "Field `SPI2_RST` writer - SPI2 module."]
pub type SPI2_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERIP_RST_EN_SPEC, bool, O>;
#[doc = "Field `I2C0_EXT0_RST` reader - I2C0 module."]
pub type I2C0_EXT0_RST_R = crate::BitReader<bool>;
#[doc = "Field `I2C0_EXT0_RST` writer - I2C0 module."]
pub type I2C0_EXT0_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERIP_RST_EN_SPEC, bool, O>;
#[doc = "Field `UHCI0_RST` reader - UDMA0 module."]
pub type UHCI0_RST_R = crate::BitReader<bool>;
#[doc = "Field `UHCI0_RST` writer - UDMA0 module."]
pub type UHCI0_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERIP_RST_EN_SPEC, bool, O>;
#[doc = "Field `RMT_RST` reader - RMT module."]
pub type RMT_RST_R = crate::BitReader<bool>;
#[doc = "Field `RMT_RST` writer - RMT module."]
pub type RMT_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERIP_RST_EN_SPEC, bool, O>;
#[doc = "Field `PCNT_RST` reader - PCNT module."]
pub type PCNT_RST_R = crate::BitReader<bool>;
#[doc = "Field `PCNT_RST` writer - PCNT module."]
pub type PCNT_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERIP_RST_EN_SPEC, bool, O>;
#[doc = "Field `LEDC_RST` reader - LEDC module."]
pub type LEDC_RST_R = crate::BitReader<bool>;
#[doc = "Field `LEDC_RST` writer - LEDC module."]
pub type LEDC_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERIP_RST_EN_SPEC, bool, O>;
#[doc = "Field `UHCI1_RST` reader - UDMA1 module."]
pub type UHCI1_RST_R = crate::BitReader<bool>;
#[doc = "Field `UHCI1_RST` writer - UDMA1 module."]
pub type UHCI1_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERIP_RST_EN_SPEC, bool, O>;
#[doc = "Field `TIMERGROUP_RST` reader - TIMG0 module."]
pub type TIMERGROUP_RST_R = crate::BitReader<bool>;
#[doc = "Field `TIMERGROUP_RST` writer - TIMG0 module."]
pub type TIMERGROUP_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERIP_RST_EN_SPEC, bool, O>;
#[doc = "Field `EFUSE_RST` reader - eFuse module."]
pub type EFUSE_RST_R = crate::BitReader<bool>;
#[doc = "Field `EFUSE_RST` writer - eFuse module."]
pub type EFUSE_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERIP_RST_EN_SPEC, bool, O>;
#[doc = "Field `TIMERGROUP1_RST` reader - TIMG1 module."]
pub type TIMERGROUP1_RST_R = crate::BitReader<bool>;
#[doc = "Field `TIMERGROUP1_RST` writer - TIMG1 module."]
pub type TIMERGROUP1_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERIP_RST_EN_SPEC, bool, O>;
#[doc = "Field `SPI3_RST` reader - SPI3 module."]
pub type SPI3_RST_R = crate::BitReader<bool>;
#[doc = "Field `SPI3_RST` writer - SPI3 module."]
pub type SPI3_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERIP_RST_EN_SPEC, bool, O>;
#[doc = "Field `PWM0_RST` reader - PWM0 module."]
pub type PWM0_RST_R = crate::BitReader<bool>;
#[doc = "Field `PWM0_RST` writer - PWM0 module."]
pub type PWM0_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERIP_RST_EN_SPEC, bool, O>;
#[doc = "Field `I2C_EXT1_RST` reader - I2C1 module."]
pub type I2C_EXT1_RST_R = crate::BitReader<bool>;
#[doc = "Field `I2C_EXT1_RST` writer - I2C1 module."]
pub type I2C_EXT1_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERIP_RST_EN_SPEC, bool, O>;
#[doc = "Field `TWAI_RST` reader - TWAI module."]
pub type TWAI_RST_R = crate::BitReader<bool>;
#[doc = "Field `TWAI_RST` writer - TWAI module."]
pub type TWAI_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERIP_RST_EN_SPEC, bool, O>;
#[doc = "Field `PWM1_RST` reader - PWM1 module."]
pub type PWM1_RST_R = crate::BitReader<bool>;
#[doc = "Field `PWM1_RST` writer - PWM1 module."]
pub type PWM1_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERIP_RST_EN_SPEC, bool, O>;
#[doc = "Field `I2S1_RST` reader - I2S1 module."]
pub type I2S1_RST_R = crate::BitReader<bool>;
#[doc = "Field `I2S1_RST` writer - I2S1 module."]
pub type I2S1_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERIP_RST_EN_SPEC, bool, O>;
#[doc = "Field `SPI_DMA_RST` reader - SPI_DMA module."]
pub type SPI_DMA_RST_R = crate::BitReader<bool>;
#[doc = "Field `SPI_DMA_RST` writer - SPI_DMA module."]
pub type SPI_DMA_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERIP_RST_EN_SPEC, bool, O>;
#[doc = "Field `UART2_RST` reader - UART2 module."]
pub type UART2_RST_R = crate::BitReader<bool>;
#[doc = "Field `UART2_RST` writer - UART2 module."]
pub type UART2_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERIP_RST_EN_SPEC, bool, O>;
#[doc = "Field `UART_MEM_RST` reader - Shared memory of UART0 ~ 2."]
pub type UART_MEM_RST_R = crate::BitReader<bool>;
#[doc = "Field `UART_MEM_RST` writer - Shared memory of UART0 ~ 2."]
pub type UART_MEM_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERIP_RST_EN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1 - SPI0 and SPI1 module."]
    #[inline(always)]
    pub fn spi01_rst(&self) -> SPI01_RST_R {
        SPI01_RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - UART0 module."]
    #[inline(always)]
    pub fn uart_rst(&self) -> UART_RST_R {
        UART_RST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - I2S0 module."]
    #[inline(always)]
    pub fn i2s0_rst(&self) -> I2S0_RST_R {
        I2S0_RST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - UART1 module."]
    #[inline(always)]
    pub fn uart1_rst(&self) -> UART1_RST_R {
        UART1_RST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SPI2 module."]
    #[inline(always)]
    pub fn spi2_rst(&self) -> SPI2_RST_R {
        SPI2_RST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - I2C0 module."]
    #[inline(always)]
    pub fn i2c0_ext0_rst(&self) -> I2C0_EXT0_RST_R {
        I2C0_EXT0_RST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - UDMA0 module."]
    #[inline(always)]
    pub fn uhci0_rst(&self) -> UHCI0_RST_R {
        UHCI0_RST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - RMT module."]
    #[inline(always)]
    pub fn rmt_rst(&self) -> RMT_RST_R {
        RMT_RST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PCNT module."]
    #[inline(always)]
    pub fn pcnt_rst(&self) -> PCNT_RST_R {
        PCNT_RST_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - LEDC module."]
    #[inline(always)]
    pub fn ledc_rst(&self) -> LEDC_RST_R {
        LEDC_RST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - UDMA1 module."]
    #[inline(always)]
    pub fn uhci1_rst(&self) -> UHCI1_RST_R {
        UHCI1_RST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TIMG0 module."]
    #[inline(always)]
    pub fn timergroup_rst(&self) -> TIMERGROUP_RST_R {
        TIMERGROUP_RST_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - eFuse module."]
    #[inline(always)]
    pub fn efuse_rst(&self) -> EFUSE_RST_R {
        EFUSE_RST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - TIMG1 module."]
    #[inline(always)]
    pub fn timergroup1_rst(&self) -> TIMERGROUP1_RST_R {
        TIMERGROUP1_RST_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - SPI3 module."]
    #[inline(always)]
    pub fn spi3_rst(&self) -> SPI3_RST_R {
        SPI3_RST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - PWM0 module."]
    #[inline(always)]
    pub fn pwm0_rst(&self) -> PWM0_RST_R {
        PWM0_RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - I2C1 module."]
    #[inline(always)]
    pub fn i2c_ext1_rst(&self) -> I2C_EXT1_RST_R {
        I2C_EXT1_RST_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - TWAI module."]
    #[inline(always)]
    pub fn twai_rst(&self) -> TWAI_RST_R {
        TWAI_RST_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - PWM1 module."]
    #[inline(always)]
    pub fn pwm1_rst(&self) -> PWM1_RST_R {
        PWM1_RST_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - I2S1 module."]
    #[inline(always)]
    pub fn i2s1_rst(&self) -> I2S1_RST_R {
        I2S1_RST_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - SPI_DMA module."]
    #[inline(always)]
    pub fn spi_dma_rst(&self) -> SPI_DMA_RST_R {
        SPI_DMA_RST_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - UART2 module."]
    #[inline(always)]
    pub fn uart2_rst(&self) -> UART2_RST_R {
        UART2_RST_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Shared memory of UART0 ~ 2."]
    #[inline(always)]
    pub fn uart_mem_rst(&self) -> UART_MEM_RST_R {
        UART_MEM_RST_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - SPI0 and SPI1 module."]
    #[inline(always)]
    pub fn spi01_rst(&mut self) -> SPI01_RST_W<1> {
        SPI01_RST_W::new(self)
    }
    #[doc = "Bit 2 - UART0 module."]
    #[inline(always)]
    pub fn uart_rst(&mut self) -> UART_RST_W<2> {
        UART_RST_W::new(self)
    }
    #[doc = "Bit 4 - I2S0 module."]
    #[inline(always)]
    pub fn i2s0_rst(&mut self) -> I2S0_RST_W<4> {
        I2S0_RST_W::new(self)
    }
    #[doc = "Bit 5 - UART1 module."]
    #[inline(always)]
    pub fn uart1_rst(&mut self) -> UART1_RST_W<5> {
        UART1_RST_W::new(self)
    }
    #[doc = "Bit 6 - SPI2 module."]
    #[inline(always)]
    pub fn spi2_rst(&mut self) -> SPI2_RST_W<6> {
        SPI2_RST_W::new(self)
    }
    #[doc = "Bit 7 - I2C0 module."]
    #[inline(always)]
    pub fn i2c0_ext0_rst(&mut self) -> I2C0_EXT0_RST_W<7> {
        I2C0_EXT0_RST_W::new(self)
    }
    #[doc = "Bit 8 - UDMA0 module."]
    #[inline(always)]
    pub fn uhci0_rst(&mut self) -> UHCI0_RST_W<8> {
        UHCI0_RST_W::new(self)
    }
    #[doc = "Bit 9 - RMT module."]
    #[inline(always)]
    pub fn rmt_rst(&mut self) -> RMT_RST_W<9> {
        RMT_RST_W::new(self)
    }
    #[doc = "Bit 10 - PCNT module."]
    #[inline(always)]
    pub fn pcnt_rst(&mut self) -> PCNT_RST_W<10> {
        PCNT_RST_W::new(self)
    }
    #[doc = "Bit 11 - LEDC module."]
    #[inline(always)]
    pub fn ledc_rst(&mut self) -> LEDC_RST_W<11> {
        LEDC_RST_W::new(self)
    }
    #[doc = "Bit 12 - UDMA1 module."]
    #[inline(always)]
    pub fn uhci1_rst(&mut self) -> UHCI1_RST_W<12> {
        UHCI1_RST_W::new(self)
    }
    #[doc = "Bit 13 - TIMG0 module."]
    #[inline(always)]
    pub fn timergroup_rst(&mut self) -> TIMERGROUP_RST_W<13> {
        TIMERGROUP_RST_W::new(self)
    }
    #[doc = "Bit 14 - eFuse module."]
    #[inline(always)]
    pub fn efuse_rst(&mut self) -> EFUSE_RST_W<14> {
        EFUSE_RST_W::new(self)
    }
    #[doc = "Bit 15 - TIMG1 module."]
    #[inline(always)]
    pub fn timergroup1_rst(&mut self) -> TIMERGROUP1_RST_W<15> {
        TIMERGROUP1_RST_W::new(self)
    }
    #[doc = "Bit 16 - SPI3 module."]
    #[inline(always)]
    pub fn spi3_rst(&mut self) -> SPI3_RST_W<16> {
        SPI3_RST_W::new(self)
    }
    #[doc = "Bit 17 - PWM0 module."]
    #[inline(always)]
    pub fn pwm0_rst(&mut self) -> PWM0_RST_W<17> {
        PWM0_RST_W::new(self)
    }
    #[doc = "Bit 18 - I2C1 module."]
    #[inline(always)]
    pub fn i2c_ext1_rst(&mut self) -> I2C_EXT1_RST_W<18> {
        I2C_EXT1_RST_W::new(self)
    }
    #[doc = "Bit 19 - TWAI module."]
    #[inline(always)]
    pub fn twai_rst(&mut self) -> TWAI_RST_W<19> {
        TWAI_RST_W::new(self)
    }
    #[doc = "Bit 20 - PWM1 module."]
    #[inline(always)]
    pub fn pwm1_rst(&mut self) -> PWM1_RST_W<20> {
        PWM1_RST_W::new(self)
    }
    #[doc = "Bit 21 - I2S1 module."]
    #[inline(always)]
    pub fn i2s1_rst(&mut self) -> I2S1_RST_W<21> {
        I2S1_RST_W::new(self)
    }
    #[doc = "Bit 22 - SPI_DMA module."]
    #[inline(always)]
    pub fn spi_dma_rst(&mut self) -> SPI_DMA_RST_W<22> {
        SPI_DMA_RST_W::new(self)
    }
    #[doc = "Bit 23 - UART2 module."]
    #[inline(always)]
    pub fn uart2_rst(&mut self) -> UART2_RST_W<23> {
        UART2_RST_W::new(self)
    }
    #[doc = "Bit 24 - Shared memory of UART0 ~ 2."]
    #[inline(always)]
    pub fn uart_mem_rst(&mut self) -> UART_MEM_RST_W<24> {
        UART_MEM_RST_W::new(self)
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
}
#[doc = "`reset()` method sets PERIP_RST_EN to value 0"]
impl crate::Resettable for PERIP_RST_EN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
