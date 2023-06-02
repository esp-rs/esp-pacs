#[doc = "Register `CORE_0_PIF_PMS_CONSTRAIN_5` reader"]
pub struct R(crate::R<CORE_0_PIF_PMS_CONSTRAIN_5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_0_PIF_PMS_CONSTRAIN_5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_0_PIF_PMS_CONSTRAIN_5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_0_PIF_PMS_CONSTRAIN_5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CORE_0_PIF_PMS_CONSTRAIN_5` writer"]
pub struct W(crate::W<CORE_0_PIF_PMS_CONSTRAIN_5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CORE_0_PIF_PMS_CONSTRAIN_5_SPEC>;
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
impl From<crate::W<CORE_0_PIF_PMS_CONSTRAIN_5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CORE_0_PIF_PMS_CONSTRAIN_5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_UART` reader - Core0 access uart permission in world1."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_UART_R = crate::FieldReader;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_UART` writer - Core0 access uart permission in world1."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_UART_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_0_PIF_PMS_CONSTRAIN_5_SPEC, 2, O>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_G0SPI_1` reader - Core0 access g0spi_1 permission in world1."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_G0SPI_1_R = crate::FieldReader;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_G0SPI_1` writer - Core0 access g0spi_1 permission in world1."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_G0SPI_1_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_0_PIF_PMS_CONSTRAIN_5_SPEC, 2, O>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_G0SPI_0` reader - Core0 access g0spi_0 permission in world1."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_G0SPI_0_R = crate::FieldReader;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_G0SPI_0` writer - Core0 access g0spi_0 permission in world1."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_G0SPI_0_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_0_PIF_PMS_CONSTRAIN_5_SPEC, 2, O>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_GPIO` reader - Core0 access gpio permission in world1."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_GPIO_R = crate::FieldReader;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_GPIO` writer - Core0 access gpio permission in world1."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_GPIO_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_0_PIF_PMS_CONSTRAIN_5_SPEC, 2, O>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_FE2` reader - Core0 access fe2 permission in world1."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_FE2_R = crate::FieldReader;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_FE2` writer - Core0 access fe2 permission in world1."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_FE2_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_0_PIF_PMS_CONSTRAIN_5_SPEC, 2, O>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_FE` reader - Core0 access fe permission in world1."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_FE_R = crate::FieldReader;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_FE` writer - Core0 access fe permission in world1."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_FE_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_0_PIF_PMS_CONSTRAIN_5_SPEC, 2, O>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_RTC` reader - Core0 access rtc permission in world1."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_RTC_R = crate::FieldReader;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_RTC` writer - Core0 access rtc permission in world1."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_RTC_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_0_PIF_PMS_CONSTRAIN_5_SPEC, 2, O>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_IO_MUX` reader - Core0 access io_mux permission in world1."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_IO_MUX_R = crate::FieldReader;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_IO_MUX` writer - Core0 access io_mux permission in world1."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_IO_MUX_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_0_PIF_PMS_CONSTRAIN_5_SPEC, 2, O>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_HINF` reader - Core0 access hinf permission in world1."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_HINF_R = crate::FieldReader;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_HINF` writer - Core0 access hinf permission in world1."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_HINF_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_0_PIF_PMS_CONSTRAIN_5_SPEC, 2, O>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_MISC` reader - Core0 access misc permission in world1."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_MISC_R = crate::FieldReader;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_MISC` writer - Core0 access misc permission in world1."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_MISC_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_0_PIF_PMS_CONSTRAIN_5_SPEC, 2, O>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_I2C` reader - Core0 access i2c permission in world1."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_I2C_R = crate::FieldReader;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_I2C` writer - Core0 access i2c permission in world1."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_I2C_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_0_PIF_PMS_CONSTRAIN_5_SPEC, 2, O>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_I2S0` reader - Core0 access i2s0 permission in world1."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_I2S0_R = crate::FieldReader;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_I2S0` writer - Core0 access i2s0 permission in world1."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_I2S0_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_0_PIF_PMS_CONSTRAIN_5_SPEC, 2, O>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_UART1` reader - Core0 access uart1 permission in world1."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_UART1_R = crate::FieldReader;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_UART1` writer - Core0 access uart1 permission in world1."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_UART1_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_0_PIF_PMS_CONSTRAIN_5_SPEC, 2, O>;
impl R {
    #[doc = "Bits 0:1 - Core0 access uart permission in world1."]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_1_uart(&self) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_UART_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_UART_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Core0 access g0spi_1 permission in world1."]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_1_g0spi_1(
        &self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_G0SPI_1_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_G0SPI_1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Core0 access g0spi_0 permission in world1."]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_1_g0spi_0(
        &self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_G0SPI_0_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_G0SPI_0_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Core0 access gpio permission in world1."]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_1_gpio(&self) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_GPIO_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_GPIO_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Core0 access fe2 permission in world1."]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_1_fe2(&self) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_FE2_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_FE2_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Core0 access fe permission in world1."]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_1_fe(&self) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_FE_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_FE_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Core0 access rtc permission in world1."]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_1_rtc(&self) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_RTC_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_RTC_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Core0 access io_mux permission in world1."]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_1_io_mux(
        &self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_IO_MUX_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_IO_MUX_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Core0 access hinf permission in world1."]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_1_hinf(&self) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_HINF_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_HINF_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Core0 access misc permission in world1."]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_1_misc(&self) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_MISC_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_MISC_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Core0 access i2c permission in world1."]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_1_i2c(&self) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_I2C_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_I2C_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Core0 access i2s0 permission in world1."]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_1_i2s0(&self) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_I2S0_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_I2S0_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Core0 access uart1 permission in world1."]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_1_uart1(
        &self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_UART1_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_UART1_R::new(((self.bits >> 30) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_0_PIF_PMS_CONSTRAIN_5")
            .field(
                "core_0_pif_pms_constrain_world_1_uart",
                &format_args!("{}", self.core_0_pif_pms_constrain_world_1_uart().bits()),
            )
            .field(
                "core_0_pif_pms_constrain_world_1_g0spi_1",
                &format_args!("{}", self.core_0_pif_pms_constrain_world_1_g0spi_1().bits()),
            )
            .field(
                "core_0_pif_pms_constrain_world_1_g0spi_0",
                &format_args!("{}", self.core_0_pif_pms_constrain_world_1_g0spi_0().bits()),
            )
            .field(
                "core_0_pif_pms_constrain_world_1_gpio",
                &format_args!("{}", self.core_0_pif_pms_constrain_world_1_gpio().bits()),
            )
            .field(
                "core_0_pif_pms_constrain_world_1_fe2",
                &format_args!("{}", self.core_0_pif_pms_constrain_world_1_fe2().bits()),
            )
            .field(
                "core_0_pif_pms_constrain_world_1_fe",
                &format_args!("{}", self.core_0_pif_pms_constrain_world_1_fe().bits()),
            )
            .field(
                "core_0_pif_pms_constrain_world_1_rtc",
                &format_args!("{}", self.core_0_pif_pms_constrain_world_1_rtc().bits()),
            )
            .field(
                "core_0_pif_pms_constrain_world_1_io_mux",
                &format_args!("{}", self.core_0_pif_pms_constrain_world_1_io_mux().bits()),
            )
            .field(
                "core_0_pif_pms_constrain_world_1_hinf",
                &format_args!("{}", self.core_0_pif_pms_constrain_world_1_hinf().bits()),
            )
            .field(
                "core_0_pif_pms_constrain_world_1_misc",
                &format_args!("{}", self.core_0_pif_pms_constrain_world_1_misc().bits()),
            )
            .field(
                "core_0_pif_pms_constrain_world_1_i2c",
                &format_args!("{}", self.core_0_pif_pms_constrain_world_1_i2c().bits()),
            )
            .field(
                "core_0_pif_pms_constrain_world_1_i2s0",
                &format_args!("{}", self.core_0_pif_pms_constrain_world_1_i2s0().bits()),
            )
            .field(
                "core_0_pif_pms_constrain_world_1_uart1",
                &format_args!("{}", self.core_0_pif_pms_constrain_world_1_uart1().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CORE_0_PIF_PMS_CONSTRAIN_5_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:1 - Core0 access uart permission in world1."]
    #[inline(always)]
    #[must_use]
    pub fn core_0_pif_pms_constrain_world_1_uart(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_UART_W<0> {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_UART_W::new(self)
    }
    #[doc = "Bits 2:3 - Core0 access g0spi_1 permission in world1."]
    #[inline(always)]
    #[must_use]
    pub fn core_0_pif_pms_constrain_world_1_g0spi_1(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_G0SPI_1_W<2> {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_G0SPI_1_W::new(self)
    }
    #[doc = "Bits 4:5 - Core0 access g0spi_0 permission in world1."]
    #[inline(always)]
    #[must_use]
    pub fn core_0_pif_pms_constrain_world_1_g0spi_0(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_G0SPI_0_W<4> {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_G0SPI_0_W::new(self)
    }
    #[doc = "Bits 6:7 - Core0 access gpio permission in world1."]
    #[inline(always)]
    #[must_use]
    pub fn core_0_pif_pms_constrain_world_1_gpio(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_GPIO_W<6> {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_GPIO_W::new(self)
    }
    #[doc = "Bits 8:9 - Core0 access fe2 permission in world1."]
    #[inline(always)]
    #[must_use]
    pub fn core_0_pif_pms_constrain_world_1_fe2(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_FE2_W<8> {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_FE2_W::new(self)
    }
    #[doc = "Bits 10:11 - Core0 access fe permission in world1."]
    #[inline(always)]
    #[must_use]
    pub fn core_0_pif_pms_constrain_world_1_fe(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_FE_W<10> {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_FE_W::new(self)
    }
    #[doc = "Bits 14:15 - Core0 access rtc permission in world1."]
    #[inline(always)]
    #[must_use]
    pub fn core_0_pif_pms_constrain_world_1_rtc(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_RTC_W<14> {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_RTC_W::new(self)
    }
    #[doc = "Bits 16:17 - Core0 access io_mux permission in world1."]
    #[inline(always)]
    #[must_use]
    pub fn core_0_pif_pms_constrain_world_1_io_mux(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_IO_MUX_W<16> {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_IO_MUX_W::new(self)
    }
    #[doc = "Bits 20:21 - Core0 access hinf permission in world1."]
    #[inline(always)]
    #[must_use]
    pub fn core_0_pif_pms_constrain_world_1_hinf(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_HINF_W<20> {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_HINF_W::new(self)
    }
    #[doc = "Bits 24:25 - Core0 access misc permission in world1."]
    #[inline(always)]
    #[must_use]
    pub fn core_0_pif_pms_constrain_world_1_misc(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_MISC_W<24> {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_MISC_W::new(self)
    }
    #[doc = "Bits 26:27 - Core0 access i2c permission in world1."]
    #[inline(always)]
    #[must_use]
    pub fn core_0_pif_pms_constrain_world_1_i2c(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_I2C_W<26> {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_I2C_W::new(self)
    }
    #[doc = "Bits 28:29 - Core0 access i2s0 permission in world1."]
    #[inline(always)]
    #[must_use]
    pub fn core_0_pif_pms_constrain_world_1_i2s0(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_I2S0_W<28> {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_I2S0_W::new(self)
    }
    #[doc = "Bits 30:31 - Core0 access uart1 permission in world1."]
    #[inline(always)]
    #[must_use]
    pub fn core_0_pif_pms_constrain_world_1_uart1(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_UART1_W<30> {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_UART1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Core0 access peripherals permission configuration register 5.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_0_pif_pms_constrain_5](index.html) module"]
pub struct CORE_0_PIF_PMS_CONSTRAIN_5_SPEC;
impl crate::RegisterSpec for CORE_0_PIF_PMS_CONSTRAIN_5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_0_pif_pms_constrain_5::R](R) reader structure"]
impl crate::Readable for CORE_0_PIF_PMS_CONSTRAIN_5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [core_0_pif_pms_constrain_5::W](W) writer structure"]
impl crate::Writable for CORE_0_PIF_PMS_CONSTRAIN_5_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CORE_0_PIF_PMS_CONSTRAIN_5 to value 0xff33_cfff"]
impl crate::Resettable for CORE_0_PIF_PMS_CONSTRAIN_5_SPEC {
    const RESET_VALUE: Self::Ux = 0xff33_cfff;
}
