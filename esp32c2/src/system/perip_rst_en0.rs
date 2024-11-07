#[doc = "Register `PERIP_RST_EN0` reader"]
pub type R = crate::R<PERIP_RST_EN0_SPEC>;
#[doc = "Register `PERIP_RST_EN0` writer"]
pub type W = crate::W<PERIP_RST_EN0_SPEC>;
#[doc = "Field `SPI01_RST` reader - Set 1 to let SPI01 reset"]
pub type SPI01_RST_R = crate::BitReader;
#[doc = "Field `SPI01_RST` writer - Set 1 to let SPI01 reset"]
pub type SPI01_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART_RST` reader - Set 1 to let UART reset"]
pub type UART_RST_R = crate::BitReader;
#[doc = "Field `UART_RST` writer - Set 1 to let UART reset"]
pub type UART_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART1_RST` reader - Set 1 to let UART1 reset"]
pub type UART1_RST_R = crate::BitReader;
#[doc = "Field `UART1_RST` writer - Set 1 to let UART1 reset"]
pub type UART1_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI2_RST` reader - Set 1 to let SPI2 reset"]
pub type SPI2_RST_R = crate::BitReader;
#[doc = "Field `SPI2_RST` writer - Set 1 to let SPI2 reset"]
pub type SPI2_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_EXT0_RST` reader - Set 1 to let I2C_EXT0 reset"]
pub type I2C_EXT0_RST_R = crate::BitReader;
#[doc = "Field `I2C_EXT0_RST` writer - Set 1 to let I2C_EXT0 reset"]
pub type I2C_EXT0_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEDC_RST` reader - Set 1 to let LEDC reset"]
pub type LEDC_RST_R = crate::BitReader;
#[doc = "Field `LEDC_RST` writer - Set 1 to let LEDC reset"]
pub type LEDC_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMERGROUP_RST` reader - Set 1 to let TIMERGROUP reset"]
pub type TIMERGROUP_RST_R = crate::BitReader;
#[doc = "Field `TIMERGROUP_RST` writer - Set 1 to let TIMERGROUP reset"]
pub type TIMERGROUP_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART_MEM_RST` reader - Set 1 to let UART_MEM reset"]
pub type UART_MEM_RST_R = crate::BitReader;
#[doc = "Field `UART_MEM_RST` writer - Set 1 to let UART_MEM reset"]
pub type UART_MEM_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APB_SARADC_RST` reader - Set 1 to let APB_SARADC reset"]
pub type APB_SARADC_RST_R = crate::BitReader;
#[doc = "Field `APB_SARADC_RST` writer - Set 1 to let APB_SARADC reset"]
pub type APB_SARADC_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSTIMER_RST` reader - Set 1 to let SYSTIMER reset"]
pub type SYSTIMER_RST_R = crate::BitReader;
#[doc = "Field `SYSTIMER_RST` writer - Set 1 to let SYSTIMER reset"]
pub type SYSTIMER_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC2_ARB_RST` reader - Set 1 to let ADC2_ARB reset"]
pub type ADC2_ARB_RST_R = crate::BitReader;
#[doc = "Field `ADC2_ARB_RST` writer - Set 1 to let ADC2_ARB reset"]
pub type ADC2_ARB_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
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
            .field("spi01_rst", &self.spi01_rst())
            .field("uart_rst", &self.uart_rst())
            .field("uart1_rst", &self.uart1_rst())
            .field("spi2_rst", &self.spi2_rst())
            .field("i2c_ext0_rst", &self.i2c_ext0_rst())
            .field("ledc_rst", &self.ledc_rst())
            .field("timergroup_rst", &self.timergroup_rst())
            .field("uart_mem_rst", &self.uart_mem_rst())
            .field("apb_saradc_rst", &self.apb_saradc_rst())
            .field("systimer_rst", &self.systimer_rst())
            .field("adc2_arb_rst", &self.adc2_arb_rst())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - Set 1 to let SPI01 reset"]
    #[inline(always)]
    pub fn spi01_rst(&mut self) -> SPI01_RST_W<PERIP_RST_EN0_SPEC> {
        SPI01_RST_W::new(self, 1)
    }
    #[doc = "Bit 2 - Set 1 to let UART reset"]
    #[inline(always)]
    pub fn uart_rst(&mut self) -> UART_RST_W<PERIP_RST_EN0_SPEC> {
        UART_RST_W::new(self, 2)
    }
    #[doc = "Bit 5 - Set 1 to let UART1 reset"]
    #[inline(always)]
    pub fn uart1_rst(&mut self) -> UART1_RST_W<PERIP_RST_EN0_SPEC> {
        UART1_RST_W::new(self, 5)
    }
    #[doc = "Bit 6 - Set 1 to let SPI2 reset"]
    #[inline(always)]
    pub fn spi2_rst(&mut self) -> SPI2_RST_W<PERIP_RST_EN0_SPEC> {
        SPI2_RST_W::new(self, 6)
    }
    #[doc = "Bit 7 - Set 1 to let I2C_EXT0 reset"]
    #[inline(always)]
    pub fn i2c_ext0_rst(&mut self) -> I2C_EXT0_RST_W<PERIP_RST_EN0_SPEC> {
        I2C_EXT0_RST_W::new(self, 7)
    }
    #[doc = "Bit 11 - Set 1 to let LEDC reset"]
    #[inline(always)]
    pub fn ledc_rst(&mut self) -> LEDC_RST_W<PERIP_RST_EN0_SPEC> {
        LEDC_RST_W::new(self, 11)
    }
    #[doc = "Bit 13 - Set 1 to let TIMERGROUP reset"]
    #[inline(always)]
    pub fn timergroup_rst(&mut self) -> TIMERGROUP_RST_W<PERIP_RST_EN0_SPEC> {
        TIMERGROUP_RST_W::new(self, 13)
    }
    #[doc = "Bit 24 - Set 1 to let UART_MEM reset"]
    #[inline(always)]
    pub fn uart_mem_rst(&mut self) -> UART_MEM_RST_W<PERIP_RST_EN0_SPEC> {
        UART_MEM_RST_W::new(self, 24)
    }
    #[doc = "Bit 28 - Set 1 to let APB_SARADC reset"]
    #[inline(always)]
    pub fn apb_saradc_rst(&mut self) -> APB_SARADC_RST_W<PERIP_RST_EN0_SPEC> {
        APB_SARADC_RST_W::new(self, 28)
    }
    #[doc = "Bit 29 - Set 1 to let SYSTIMER reset"]
    #[inline(always)]
    pub fn systimer_rst(&mut self) -> SYSTIMER_RST_W<PERIP_RST_EN0_SPEC> {
        SYSTIMER_RST_W::new(self, 29)
    }
    #[doc = "Bit 30 - Set 1 to let ADC2_ARB reset"]
    #[inline(always)]
    pub fn adc2_arb_rst(&mut self) -> ADC2_ARB_RST_W<PERIP_RST_EN0_SPEC> {
        ADC2_ARB_RST_W::new(self, 30)
    }
}
#[doc = "reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`perip_rst_en0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`perip_rst_en0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
