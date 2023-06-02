#[doc = "Register `BACKUP_BUS_PMS_CONSTRAIN_1` reader"]
pub struct R(crate::R<BACKUP_BUS_PMS_CONSTRAIN_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BACKUP_BUS_PMS_CONSTRAIN_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BACKUP_BUS_PMS_CONSTRAIN_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BACKUP_BUS_PMS_CONSTRAIN_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BACKUP_BUS_PMS_CONSTRAIN_1` writer"]
pub struct W(crate::W<BACKUP_BUS_PMS_CONSTRAIN_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BACKUP_BUS_PMS_CONSTRAIN_1_SPEC>;
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
impl From<crate::W<BACKUP_BUS_PMS_CONSTRAIN_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BACKUP_BUS_PMS_CONSTRAIN_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_UART` reader - backup_bus_pms_constrain_uart"]
pub type BACKUP_BUS_PMS_CONSTRAIN_UART_R = crate::FieldReader;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_UART` writer - backup_bus_pms_constrain_uart"]
pub type BACKUP_BUS_PMS_CONSTRAIN_UART_W<'a, const O: u8> =
    crate::FieldWriter<'a, BACKUP_BUS_PMS_CONSTRAIN_1_SPEC, 2, O>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_G0SPI_1` reader - backup_bus_pms_constrain_g0spi_1"]
pub type BACKUP_BUS_PMS_CONSTRAIN_G0SPI_1_R = crate::FieldReader;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_G0SPI_1` writer - backup_bus_pms_constrain_g0spi_1"]
pub type BACKUP_BUS_PMS_CONSTRAIN_G0SPI_1_W<'a, const O: u8> =
    crate::FieldWriter<'a, BACKUP_BUS_PMS_CONSTRAIN_1_SPEC, 2, O>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_G0SPI_0` reader - backup_bus_pms_constrain_g0spi_0"]
pub type BACKUP_BUS_PMS_CONSTRAIN_G0SPI_0_R = crate::FieldReader;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_G0SPI_0` writer - backup_bus_pms_constrain_g0spi_0"]
pub type BACKUP_BUS_PMS_CONSTRAIN_G0SPI_0_W<'a, const O: u8> =
    crate::FieldWriter<'a, BACKUP_BUS_PMS_CONSTRAIN_1_SPEC, 2, O>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_GPIO` reader - backup_bus_pms_constrain_gpio"]
pub type BACKUP_BUS_PMS_CONSTRAIN_GPIO_R = crate::FieldReader;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_GPIO` writer - backup_bus_pms_constrain_gpio"]
pub type BACKUP_BUS_PMS_CONSTRAIN_GPIO_W<'a, const O: u8> =
    crate::FieldWriter<'a, BACKUP_BUS_PMS_CONSTRAIN_1_SPEC, 2, O>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_FE2` reader - backup_bus_pms_constrain_fe2"]
pub type BACKUP_BUS_PMS_CONSTRAIN_FE2_R = crate::FieldReader;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_FE2` writer - backup_bus_pms_constrain_fe2"]
pub type BACKUP_BUS_PMS_CONSTRAIN_FE2_W<'a, const O: u8> =
    crate::FieldWriter<'a, BACKUP_BUS_PMS_CONSTRAIN_1_SPEC, 2, O>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_FE` reader - backup_bus_pms_constrain_fe"]
pub type BACKUP_BUS_PMS_CONSTRAIN_FE_R = crate::FieldReader;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_FE` writer - backup_bus_pms_constrain_fe"]
pub type BACKUP_BUS_PMS_CONSTRAIN_FE_W<'a, const O: u8> =
    crate::FieldWriter<'a, BACKUP_BUS_PMS_CONSTRAIN_1_SPEC, 2, O>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_TIMER` reader - backup_bus_pms_constrain_timer"]
pub type BACKUP_BUS_PMS_CONSTRAIN_TIMER_R = crate::FieldReader;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_TIMER` writer - backup_bus_pms_constrain_timer"]
pub type BACKUP_BUS_PMS_CONSTRAIN_TIMER_W<'a, const O: u8> =
    crate::FieldWriter<'a, BACKUP_BUS_PMS_CONSTRAIN_1_SPEC, 2, O>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_RTC` reader - backup_bus_pms_constrain_rtc"]
pub type BACKUP_BUS_PMS_CONSTRAIN_RTC_R = crate::FieldReader;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_RTC` writer - backup_bus_pms_constrain_rtc"]
pub type BACKUP_BUS_PMS_CONSTRAIN_RTC_W<'a, const O: u8> =
    crate::FieldWriter<'a, BACKUP_BUS_PMS_CONSTRAIN_1_SPEC, 2, O>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_IO_MUX` reader - backup_bus_pms_constrain_io_mux"]
pub type BACKUP_BUS_PMS_CONSTRAIN_IO_MUX_R = crate::FieldReader;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_IO_MUX` writer - backup_bus_pms_constrain_io_mux"]
pub type BACKUP_BUS_PMS_CONSTRAIN_IO_MUX_W<'a, const O: u8> =
    crate::FieldWriter<'a, BACKUP_BUS_PMS_CONSTRAIN_1_SPEC, 2, O>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_WDG` reader - backup_bus_pms_constrain_wdg"]
pub type BACKUP_BUS_PMS_CONSTRAIN_WDG_R = crate::FieldReader;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_WDG` writer - backup_bus_pms_constrain_wdg"]
pub type BACKUP_BUS_PMS_CONSTRAIN_WDG_W<'a, const O: u8> =
    crate::FieldWriter<'a, BACKUP_BUS_PMS_CONSTRAIN_1_SPEC, 2, O>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_MISC` reader - backup_bus_pms_constrain_misc"]
pub type BACKUP_BUS_PMS_CONSTRAIN_MISC_R = crate::FieldReader;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_MISC` writer - backup_bus_pms_constrain_misc"]
pub type BACKUP_BUS_PMS_CONSTRAIN_MISC_W<'a, const O: u8> =
    crate::FieldWriter<'a, BACKUP_BUS_PMS_CONSTRAIN_1_SPEC, 2, O>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_I2C` reader - backup_bus_pms_constrain_i2c"]
pub type BACKUP_BUS_PMS_CONSTRAIN_I2C_R = crate::FieldReader;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_I2C` writer - backup_bus_pms_constrain_i2c"]
pub type BACKUP_BUS_PMS_CONSTRAIN_I2C_W<'a, const O: u8> =
    crate::FieldWriter<'a, BACKUP_BUS_PMS_CONSTRAIN_1_SPEC, 2, O>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_UART1` reader - backup_bus_pms_constrain_uart1"]
pub type BACKUP_BUS_PMS_CONSTRAIN_UART1_R = crate::FieldReader;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_UART1` writer - backup_bus_pms_constrain_uart1"]
pub type BACKUP_BUS_PMS_CONSTRAIN_UART1_W<'a, const O: u8> =
    crate::FieldWriter<'a, BACKUP_BUS_PMS_CONSTRAIN_1_SPEC, 2, O>;
impl R {
    #[doc = "Bits 0:1 - backup_bus_pms_constrain_uart"]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_uart(&self) -> BACKUP_BUS_PMS_CONSTRAIN_UART_R {
        BACKUP_BUS_PMS_CONSTRAIN_UART_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - backup_bus_pms_constrain_g0spi_1"]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_g0spi_1(&self) -> BACKUP_BUS_PMS_CONSTRAIN_G0SPI_1_R {
        BACKUP_BUS_PMS_CONSTRAIN_G0SPI_1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - backup_bus_pms_constrain_g0spi_0"]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_g0spi_0(&self) -> BACKUP_BUS_PMS_CONSTRAIN_G0SPI_0_R {
        BACKUP_BUS_PMS_CONSTRAIN_G0SPI_0_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - backup_bus_pms_constrain_gpio"]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_gpio(&self) -> BACKUP_BUS_PMS_CONSTRAIN_GPIO_R {
        BACKUP_BUS_PMS_CONSTRAIN_GPIO_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - backup_bus_pms_constrain_fe2"]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_fe2(&self) -> BACKUP_BUS_PMS_CONSTRAIN_FE2_R {
        BACKUP_BUS_PMS_CONSTRAIN_FE2_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - backup_bus_pms_constrain_fe"]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_fe(&self) -> BACKUP_BUS_PMS_CONSTRAIN_FE_R {
        BACKUP_BUS_PMS_CONSTRAIN_FE_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - backup_bus_pms_constrain_timer"]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_timer(&self) -> BACKUP_BUS_PMS_CONSTRAIN_TIMER_R {
        BACKUP_BUS_PMS_CONSTRAIN_TIMER_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - backup_bus_pms_constrain_rtc"]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_rtc(&self) -> BACKUP_BUS_PMS_CONSTRAIN_RTC_R {
        BACKUP_BUS_PMS_CONSTRAIN_RTC_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - backup_bus_pms_constrain_io_mux"]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_io_mux(&self) -> BACKUP_BUS_PMS_CONSTRAIN_IO_MUX_R {
        BACKUP_BUS_PMS_CONSTRAIN_IO_MUX_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - backup_bus_pms_constrain_wdg"]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_wdg(&self) -> BACKUP_BUS_PMS_CONSTRAIN_WDG_R {
        BACKUP_BUS_PMS_CONSTRAIN_WDG_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 24:25 - backup_bus_pms_constrain_misc"]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_misc(&self) -> BACKUP_BUS_PMS_CONSTRAIN_MISC_R {
        BACKUP_BUS_PMS_CONSTRAIN_MISC_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - backup_bus_pms_constrain_i2c"]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_i2c(&self) -> BACKUP_BUS_PMS_CONSTRAIN_I2C_R {
        BACKUP_BUS_PMS_CONSTRAIN_I2C_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 30:31 - backup_bus_pms_constrain_uart1"]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_uart1(&self) -> BACKUP_BUS_PMS_CONSTRAIN_UART1_R {
        BACKUP_BUS_PMS_CONSTRAIN_UART1_R::new(((self.bits >> 30) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BACKUP_BUS_PMS_CONSTRAIN_1")
            .field(
                "backup_bus_pms_constrain_uart",
                &format_args!("{}", self.backup_bus_pms_constrain_uart().bits()),
            )
            .field(
                "backup_bus_pms_constrain_g0spi_1",
                &format_args!("{}", self.backup_bus_pms_constrain_g0spi_1().bits()),
            )
            .field(
                "backup_bus_pms_constrain_g0spi_0",
                &format_args!("{}", self.backup_bus_pms_constrain_g0spi_0().bits()),
            )
            .field(
                "backup_bus_pms_constrain_gpio",
                &format_args!("{}", self.backup_bus_pms_constrain_gpio().bits()),
            )
            .field(
                "backup_bus_pms_constrain_fe2",
                &format_args!("{}", self.backup_bus_pms_constrain_fe2().bits()),
            )
            .field(
                "backup_bus_pms_constrain_fe",
                &format_args!("{}", self.backup_bus_pms_constrain_fe().bits()),
            )
            .field(
                "backup_bus_pms_constrain_timer",
                &format_args!("{}", self.backup_bus_pms_constrain_timer().bits()),
            )
            .field(
                "backup_bus_pms_constrain_rtc",
                &format_args!("{}", self.backup_bus_pms_constrain_rtc().bits()),
            )
            .field(
                "backup_bus_pms_constrain_io_mux",
                &format_args!("{}", self.backup_bus_pms_constrain_io_mux().bits()),
            )
            .field(
                "backup_bus_pms_constrain_wdg",
                &format_args!("{}", self.backup_bus_pms_constrain_wdg().bits()),
            )
            .field(
                "backup_bus_pms_constrain_misc",
                &format_args!("{}", self.backup_bus_pms_constrain_misc().bits()),
            )
            .field(
                "backup_bus_pms_constrain_i2c",
                &format_args!("{}", self.backup_bus_pms_constrain_i2c().bits()),
            )
            .field(
                "backup_bus_pms_constrain_uart1",
                &format_args!("{}", self.backup_bus_pms_constrain_uart1().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<BACKUP_BUS_PMS_CONSTRAIN_1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:1 - backup_bus_pms_constrain_uart"]
    #[inline(always)]
    #[must_use]
    pub fn backup_bus_pms_constrain_uart(&mut self) -> BACKUP_BUS_PMS_CONSTRAIN_UART_W<0> {
        BACKUP_BUS_PMS_CONSTRAIN_UART_W::new(self)
    }
    #[doc = "Bits 2:3 - backup_bus_pms_constrain_g0spi_1"]
    #[inline(always)]
    #[must_use]
    pub fn backup_bus_pms_constrain_g0spi_1(&mut self) -> BACKUP_BUS_PMS_CONSTRAIN_G0SPI_1_W<2> {
        BACKUP_BUS_PMS_CONSTRAIN_G0SPI_1_W::new(self)
    }
    #[doc = "Bits 4:5 - backup_bus_pms_constrain_g0spi_0"]
    #[inline(always)]
    #[must_use]
    pub fn backup_bus_pms_constrain_g0spi_0(&mut self) -> BACKUP_BUS_PMS_CONSTRAIN_G0SPI_0_W<4> {
        BACKUP_BUS_PMS_CONSTRAIN_G0SPI_0_W::new(self)
    }
    #[doc = "Bits 6:7 - backup_bus_pms_constrain_gpio"]
    #[inline(always)]
    #[must_use]
    pub fn backup_bus_pms_constrain_gpio(&mut self) -> BACKUP_BUS_PMS_CONSTRAIN_GPIO_W<6> {
        BACKUP_BUS_PMS_CONSTRAIN_GPIO_W::new(self)
    }
    #[doc = "Bits 8:9 - backup_bus_pms_constrain_fe2"]
    #[inline(always)]
    #[must_use]
    pub fn backup_bus_pms_constrain_fe2(&mut self) -> BACKUP_BUS_PMS_CONSTRAIN_FE2_W<8> {
        BACKUP_BUS_PMS_CONSTRAIN_FE2_W::new(self)
    }
    #[doc = "Bits 10:11 - backup_bus_pms_constrain_fe"]
    #[inline(always)]
    #[must_use]
    pub fn backup_bus_pms_constrain_fe(&mut self) -> BACKUP_BUS_PMS_CONSTRAIN_FE_W<10> {
        BACKUP_BUS_PMS_CONSTRAIN_FE_W::new(self)
    }
    #[doc = "Bits 12:13 - backup_bus_pms_constrain_timer"]
    #[inline(always)]
    #[must_use]
    pub fn backup_bus_pms_constrain_timer(&mut self) -> BACKUP_BUS_PMS_CONSTRAIN_TIMER_W<12> {
        BACKUP_BUS_PMS_CONSTRAIN_TIMER_W::new(self)
    }
    #[doc = "Bits 14:15 - backup_bus_pms_constrain_rtc"]
    #[inline(always)]
    #[must_use]
    pub fn backup_bus_pms_constrain_rtc(&mut self) -> BACKUP_BUS_PMS_CONSTRAIN_RTC_W<14> {
        BACKUP_BUS_PMS_CONSTRAIN_RTC_W::new(self)
    }
    #[doc = "Bits 16:17 - backup_bus_pms_constrain_io_mux"]
    #[inline(always)]
    #[must_use]
    pub fn backup_bus_pms_constrain_io_mux(&mut self) -> BACKUP_BUS_PMS_CONSTRAIN_IO_MUX_W<16> {
        BACKUP_BUS_PMS_CONSTRAIN_IO_MUX_W::new(self)
    }
    #[doc = "Bits 18:19 - backup_bus_pms_constrain_wdg"]
    #[inline(always)]
    #[must_use]
    pub fn backup_bus_pms_constrain_wdg(&mut self) -> BACKUP_BUS_PMS_CONSTRAIN_WDG_W<18> {
        BACKUP_BUS_PMS_CONSTRAIN_WDG_W::new(self)
    }
    #[doc = "Bits 24:25 - backup_bus_pms_constrain_misc"]
    #[inline(always)]
    #[must_use]
    pub fn backup_bus_pms_constrain_misc(&mut self) -> BACKUP_BUS_PMS_CONSTRAIN_MISC_W<24> {
        BACKUP_BUS_PMS_CONSTRAIN_MISC_W::new(self)
    }
    #[doc = "Bits 26:27 - backup_bus_pms_constrain_i2c"]
    #[inline(always)]
    #[must_use]
    pub fn backup_bus_pms_constrain_i2c(&mut self) -> BACKUP_BUS_PMS_CONSTRAIN_I2C_W<26> {
        BACKUP_BUS_PMS_CONSTRAIN_I2C_W::new(self)
    }
    #[doc = "Bits 30:31 - backup_bus_pms_constrain_uart1"]
    #[inline(always)]
    #[must_use]
    pub fn backup_bus_pms_constrain_uart1(&mut self) -> BACKUP_BUS_PMS_CONSTRAIN_UART1_W<30> {
        BACKUP_BUS_PMS_CONSTRAIN_UART1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [backup_bus_pms_constrain_1](index.html) module"]
pub struct BACKUP_BUS_PMS_CONSTRAIN_1_SPEC;
impl crate::RegisterSpec for BACKUP_BUS_PMS_CONSTRAIN_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [backup_bus_pms_constrain_1::R](R) reader structure"]
impl crate::Readable for BACKUP_BUS_PMS_CONSTRAIN_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [backup_bus_pms_constrain_1::W](W) writer structure"]
impl crate::Writable for BACKUP_BUS_PMS_CONSTRAIN_1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BACKUP_BUS_PMS_CONSTRAIN_1 to value 0xcf0f_ffff"]
impl crate::Resettable for BACKUP_BUS_PMS_CONSTRAIN_1_SPEC {
    const RESET_VALUE: Self::Ux = 0xcf0f_ffff;
}
