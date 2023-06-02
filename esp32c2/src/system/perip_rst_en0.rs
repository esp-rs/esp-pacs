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
#[doc = "Field `SPI01_RST` reader - Set 1 to let SPI01 reset"]
pub type SPI01_RST_R = crate::BitReader;
#[doc = "Field `SPI01_RST` writer - Set 1 to let SPI01 reset"]
pub type SPI01_RST_W<'a, const O: u8> = crate::BitWriter<'a, PERIP_RST_EN0_SPEC, O>;
#[doc = "Field `UART_RST` reader - Set 1 to let UART reset"]
pub type UART_RST_R = crate::BitReader;
#[doc = "Field `UART_RST` writer - Set 1 to let UART reset"]
pub type UART_RST_W<'a, const O: u8> = crate::BitWriter<'a, PERIP_RST_EN0_SPEC, O>;
#[doc = "Field `UART1_RST` reader - Set 1 to let UART1 reset"]
pub type UART1_RST_R = crate::BitReader;
#[doc = "Field `UART1_RST` writer - Set 1 to let UART1 reset"]
pub type UART1_RST_W<'a, const O: u8> = crate::BitWriter<'a, PERIP_RST_EN0_SPEC, O>;
#[doc = "Field `SPI2_RST` reader - Set 1 to let SPI2 reset"]
pub type SPI2_RST_R = crate::BitReader;
#[doc = "Field `SPI2_RST` writer - Set 1 to let SPI2 reset"]
pub type SPI2_RST_W<'a, const O: u8> = crate::BitWriter<'a, PERIP_RST_EN0_SPEC, O>;
#[doc = "Field `I2C_EXT0_RST` reader - Set 1 to let I2C_EXT0 reset"]
pub type I2C_EXT0_RST_R = crate::BitReader;
#[doc = "Field `I2C_EXT0_RST` writer - Set 1 to let I2C_EXT0 reset"]
pub type I2C_EXT0_RST_W<'a, const O: u8> = crate::BitWriter<'a, PERIP_RST_EN0_SPEC, O>;
#[doc = "Field `LEDC_RST` reader - Set 1 to let LEDC reset"]
pub type LEDC_RST_R = crate::BitReader;
#[doc = "Field `LEDC_RST` writer - Set 1 to let LEDC reset"]
pub type LEDC_RST_W<'a, const O: u8> = crate::BitWriter<'a, PERIP_RST_EN0_SPEC, O>;
#[doc = "Field `TIMERGROUP_RST` reader - Set 1 to let TIMERGROUP reset"]
pub type TIMERGROUP_RST_R = crate::BitReader;
#[doc = "Field `TIMERGROUP_RST` writer - Set 1 to let TIMERGROUP reset"]
pub type TIMERGROUP_RST_W<'a, const O: u8> = crate::BitWriter<'a, PERIP_RST_EN0_SPEC, O>;
#[doc = "Field `UART_MEM_RST` reader - Set 1 to let UART_MEM reset"]
pub type UART_MEM_RST_R = crate::BitReader;
#[doc = "Field `UART_MEM_RST` writer - Set 1 to let UART_MEM reset"]
pub type UART_MEM_RST_W<'a, const O: u8> = crate::BitWriter<'a, PERIP_RST_EN0_SPEC, O>;
#[doc = "Field `APB_SARADC_RST` reader - Set 1 to let APB_SARADC reset"]
pub type APB_SARADC_RST_R = crate::BitReader;
#[doc = "Field `APB_SARADC_RST` writer - Set 1 to let APB_SARADC reset"]
pub type APB_SARADC_RST_W<'a, const O: u8> = crate::BitWriter<'a, PERIP_RST_EN0_SPEC, O>;
#[doc = "Field `SYSTIMER_RST` reader - Set 1 to let SYSTIMER reset"]
pub type SYSTIMER_RST_R = crate::BitReader;
#[doc = "Field `SYSTIMER_RST` writer - Set 1 to let SYSTIMER reset"]
pub type SYSTIMER_RST_W<'a, const O: u8> = crate::BitWriter<'a, PERIP_RST_EN0_SPEC, O>;
#[doc = "Field `ADC2_ARB_RST` reader - Set 1 to let ADC2_ARB reset"]
pub type ADC2_ARB_RST_R = crate::BitReader;
#[doc = "Field `ADC2_ARB_RST` writer - Set 1 to let ADC2_ARB reset"]
pub type ADC2_ARB_RST_W<'a, const O: u8> = crate::BitWriter<'a, PERIP_RST_EN0_SPEC, O>;
impl R {
    #[doc = "Bit 1 - Set 1 to let SPI01 reset"]
    #[inline(always)]
    pub fn spi01_rst(&self) -> SPI01_RST_R {
        SPI01_RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set 1 to let UART reset"]
    #[inline(always)]
    pub fn uart_rst(&self) -> UART_RST_R {
        UART_RST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - Set 1 to let UART1 reset"]
    #[inline(always)]
    pub fn uart1_rst(&self) -> UART1_RST_R {
        UART1_RST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Set 1 to let SPI2 reset"]
    #[inline(always)]
    pub fn spi2_rst(&self) -> SPI2_RST_R {
        SPI2_RST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Set 1 to let I2C_EXT0 reset"]
    #[inline(always)]
    pub fn i2c_ext0_rst(&self) -> I2C_EXT0_RST_R {
        I2C_EXT0_RST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 11 - Set 1 to let LEDC reset"]
    #[inline(always)]
    pub fn ledc_rst(&self) -> LEDC_RST_R {
        LEDC_RST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - Set 1 to let TIMERGROUP reset"]
    #[inline(always)]
    pub fn timergroup_rst(&self) -> TIMERGROUP_RST_R {
        TIMERGROUP_RST_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 24 - Set 1 to let UART_MEM reset"]
    #[inline(always)]
    pub fn uart_mem_rst(&self) -> UART_MEM_RST_R {
        UART_MEM_RST_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - Set 1 to let APB_SARADC reset"]
    #[inline(always)]
    pub fn apb_saradc_rst(&self) -> APB_SARADC_RST_R {
        APB_SARADC_RST_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Set 1 to let SYSTIMER reset"]
    #[inline(always)]
    pub fn systimer_rst(&self) -> SYSTIMER_RST_R {
        SYSTIMER_RST_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Set 1 to let ADC2_ARB reset"]
    #[inline(always)]
    pub fn adc2_arb_rst(&self) -> ADC2_ARB_RST_R {
        ADC2_ARB_RST_R::new(((self.bits >> 30) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PERIP_RST_EN0")
            .field("spi01_rst", &format_args!("{}", self.spi01_rst().bit()))
            .field("uart_rst", &format_args!("{}", self.uart_rst().bit()))
            .field("uart1_rst", &format_args!("{}", self.uart1_rst().bit()))
            .field("spi2_rst", &format_args!("{}", self.spi2_rst().bit()))
            .field(
                "i2c_ext0_rst",
                &format_args!("{}", self.i2c_ext0_rst().bit()),
            )
            .field("ledc_rst", &format_args!("{}", self.ledc_rst().bit()))
            .field(
                "timergroup_rst",
                &format_args!("{}", self.timergroup_rst().bit()),
            )
            .field(
                "uart_mem_rst",
                &format_args!("{}", self.uart_mem_rst().bit()),
            )
            .field(
                "apb_saradc_rst",
                &format_args!("{}", self.apb_saradc_rst().bit()),
            )
            .field(
                "systimer_rst",
                &format_args!("{}", self.systimer_rst().bit()),
            )
            .field(
                "adc2_arb_rst",
                &format_args!("{}", self.adc2_arb_rst().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PERIP_RST_EN0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 1 - Set 1 to let SPI01 reset"]
    #[inline(always)]
    #[must_use]
    pub fn spi01_rst(&mut self) -> SPI01_RST_W<1> {
        SPI01_RST_W::new(self)
    }
    #[doc = "Bit 2 - Set 1 to let UART reset"]
    #[inline(always)]
    #[must_use]
    pub fn uart_rst(&mut self) -> UART_RST_W<2> {
        UART_RST_W::new(self)
    }
    #[doc = "Bit 5 - Set 1 to let UART1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn uart1_rst(&mut self) -> UART1_RST_W<5> {
        UART1_RST_W::new(self)
    }
    #[doc = "Bit 6 - Set 1 to let SPI2 reset"]
    #[inline(always)]
    #[must_use]
    pub fn spi2_rst(&mut self) -> SPI2_RST_W<6> {
        SPI2_RST_W::new(self)
    }
    #[doc = "Bit 7 - Set 1 to let I2C_EXT0 reset"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_ext0_rst(&mut self) -> I2C_EXT0_RST_W<7> {
        I2C_EXT0_RST_W::new(self)
    }
    #[doc = "Bit 11 - Set 1 to let LEDC reset"]
    #[inline(always)]
    #[must_use]
    pub fn ledc_rst(&mut self) -> LEDC_RST_W<11> {
        LEDC_RST_W::new(self)
    }
    #[doc = "Bit 13 - Set 1 to let TIMERGROUP reset"]
    #[inline(always)]
    #[must_use]
    pub fn timergroup_rst(&mut self) -> TIMERGROUP_RST_W<13> {
        TIMERGROUP_RST_W::new(self)
    }
    #[doc = "Bit 24 - Set 1 to let UART_MEM reset"]
    #[inline(always)]
    #[must_use]
    pub fn uart_mem_rst(&mut self) -> UART_MEM_RST_W<24> {
        UART_MEM_RST_W::new(self)
    }
    #[doc = "Bit 28 - Set 1 to let APB_SARADC reset"]
    #[inline(always)]
    #[must_use]
    pub fn apb_saradc_rst(&mut self) -> APB_SARADC_RST_W<28> {
        APB_SARADC_RST_W::new(self)
    }
    #[doc = "Bit 29 - Set 1 to let SYSTIMER reset"]
    #[inline(always)]
    #[must_use]
    pub fn systimer_rst(&mut self) -> SYSTIMER_RST_W<29> {
        SYSTIMER_RST_W::new(self)
    }
    #[doc = "Bit 30 - Set 1 to let ADC2_ARB reset"]
    #[inline(always)]
    #[must_use]
    pub fn adc2_arb_rst(&mut self) -> ADC2_ARB_RST_W<30> {
        ADC2_ARB_RST_W::new(self)
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
