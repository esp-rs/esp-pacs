#[doc = "Register `CORE_1_PIF_PMS_CONSTRAIN_3` reader"]
pub struct R(crate::R<CORE_1_PIF_PMS_CONSTRAIN_3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_1_PIF_PMS_CONSTRAIN_3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_1_PIF_PMS_CONSTRAIN_3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_1_PIF_PMS_CONSTRAIN_3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CORE_1_PIF_PMS_CONSTRAIN_3` writer"]
pub struct W(crate::W<CORE_1_PIF_PMS_CONSTRAIN_3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CORE_1_PIF_PMS_CONSTRAIN_3_SPEC>;
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
impl From<crate::W<CORE_1_PIF_PMS_CONSTRAIN_3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CORE_1_PIF_PMS_CONSTRAIN_3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_SPI_2` reader - Core1 access spi_2 permission in world0."]
pub type CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_SPI_2_R = crate::FieldReader;
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_SPI_2` writer - Core1 access spi_2 permission in world0."]
pub type CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_SPI_2_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_1_PIF_PMS_CONSTRAIN_3_SPEC, 2, O>;
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_SPI_3` reader - Core1 access spi_3 permission in world0."]
pub type CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_SPI_3_R = crate::FieldReader;
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_SPI_3` writer - Core1 access spi_3 permission in world0."]
pub type CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_SPI_3_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_1_PIF_PMS_CONSTRAIN_3_SPEC, 2, O>;
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_APB_CTRL` reader - Core1 access apb_ctrl permission in world0."]
pub type CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_APB_CTRL_R = crate::FieldReader;
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_APB_CTRL` writer - Core1 access apb_ctrl permission in world0."]
pub type CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_APB_CTRL_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_1_PIF_PMS_CONSTRAIN_3_SPEC, 2, O>;
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_I2C_EXT1` reader - Core1 access i2c_ext1 permission in world0."]
pub type CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_I2C_EXT1_R = crate::FieldReader;
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_I2C_EXT1` writer - Core1 access i2c_ext1 permission in world0."]
pub type CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_I2C_EXT1_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_1_PIF_PMS_CONSTRAIN_3_SPEC, 2, O>;
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_SDIO_HOST` reader - Core1 access sdio_host permission in world0."]
pub type CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_SDIO_HOST_R = crate::FieldReader;
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_SDIO_HOST` writer - Core1 access sdio_host permission in world0."]
pub type CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_SDIO_HOST_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_1_PIF_PMS_CONSTRAIN_3_SPEC, 2, O>;
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_CAN` reader - Core1 access can permission in world0."]
pub type CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_CAN_R = crate::FieldReader;
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_CAN` writer - Core1 access can permission in world0."]
pub type CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_CAN_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_1_PIF_PMS_CONSTRAIN_3_SPEC, 2, O>;
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_PWM1` reader - Core1 access pwm1 permission in world0."]
pub type CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_PWM1_R = crate::FieldReader;
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_PWM1` writer - Core1 access pwm1 permission in world0."]
pub type CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_PWM1_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_1_PIF_PMS_CONSTRAIN_3_SPEC, 2, O>;
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_I2S1` reader - Core1 access i2s1 permission in world0."]
pub type CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_I2S1_R = crate::FieldReader;
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_I2S1` writer - Core1 access i2s1 permission in world0."]
pub type CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_I2S1_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_1_PIF_PMS_CONSTRAIN_3_SPEC, 2, O>;
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_UART2` reader - Core1 access uart2 permission in world0."]
pub type CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_UART2_R = crate::FieldReader;
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_UART2` writer - Core1 access uart2 permission in world0."]
pub type CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_UART2_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_1_PIF_PMS_CONSTRAIN_3_SPEC, 2, O>;
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_RWBT` reader - Core1 access rwbt permission in world0."]
pub type CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_RWBT_R = crate::FieldReader;
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_RWBT` writer - Core1 access rwbt permission in world0."]
pub type CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_RWBT_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_1_PIF_PMS_CONSTRAIN_3_SPEC, 2, O>;
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_WIFIMAC` reader - Core1 access wifimac permission in world0."]
pub type CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_WIFIMAC_R = crate::FieldReader;
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_WIFIMAC` writer - Core1 access wifimac permission in world0."]
pub type CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_WIFIMAC_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_1_PIF_PMS_CONSTRAIN_3_SPEC, 2, O>;
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_PWR` reader - Core1 access pwr permission in world0."]
pub type CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_PWR_R = crate::FieldReader;
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_PWR` writer - Core1 access pwr permission in world0."]
pub type CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_PWR_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_1_PIF_PMS_CONSTRAIN_3_SPEC, 2, O>;
impl R {
    #[doc = "Bits 0:1 - Core1 access spi_2 permission in world0."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_world_0_spi_2(
        &self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_SPI_2_R {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_SPI_2_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Core1 access spi_3 permission in world0."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_world_0_spi_3(
        &self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_SPI_3_R {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_SPI_3_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Core1 access apb_ctrl permission in world0."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_world_0_apb_ctrl(
        &self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_APB_CTRL_R {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_APB_CTRL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Core1 access i2c_ext1 permission in world0."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_world_0_i2c_ext1(
        &self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_I2C_EXT1_R {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_I2C_EXT1_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Core1 access sdio_host permission in world0."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_world_0_sdio_host(
        &self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_SDIO_HOST_R {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_SDIO_HOST_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Core1 access can permission in world0."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_world_0_can(&self) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_CAN_R {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_CAN_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Core1 access pwm1 permission in world0."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_world_0_pwm1(&self) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_PWM1_R {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_PWM1_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Core1 access i2s1 permission in world0."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_world_0_i2s1(&self) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_I2S1_R {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_I2S1_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Core1 access uart2 permission in world0."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_world_0_uart2(
        &self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_UART2_R {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_UART2_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Core1 access rwbt permission in world0."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_world_0_rwbt(&self) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_RWBT_R {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_RWBT_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Core1 access wifimac permission in world0."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_world_0_wifimac(
        &self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_WIFIMAC_R {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_WIFIMAC_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Core1 access pwr permission in world0."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_world_0_pwr(&self) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_PWR_R {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_PWR_R::new(((self.bits >> 28) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_1_PIF_PMS_CONSTRAIN_3")
            .field(
                "core_1_pif_pms_constrain_world_0_spi_2",
                &format_args!("{}", self.core_1_pif_pms_constrain_world_0_spi_2().bits()),
            )
            .field(
                "core_1_pif_pms_constrain_world_0_spi_3",
                &format_args!("{}", self.core_1_pif_pms_constrain_world_0_spi_3().bits()),
            )
            .field(
                "core_1_pif_pms_constrain_world_0_apb_ctrl",
                &format_args!(
                    "{}",
                    self.core_1_pif_pms_constrain_world_0_apb_ctrl().bits()
                ),
            )
            .field(
                "core_1_pif_pms_constrain_world_0_i2c_ext1",
                &format_args!(
                    "{}",
                    self.core_1_pif_pms_constrain_world_0_i2c_ext1().bits()
                ),
            )
            .field(
                "core_1_pif_pms_constrain_world_0_sdio_host",
                &format_args!(
                    "{}",
                    self.core_1_pif_pms_constrain_world_0_sdio_host().bits()
                ),
            )
            .field(
                "core_1_pif_pms_constrain_world_0_can",
                &format_args!("{}", self.core_1_pif_pms_constrain_world_0_can().bits()),
            )
            .field(
                "core_1_pif_pms_constrain_world_0_pwm1",
                &format_args!("{}", self.core_1_pif_pms_constrain_world_0_pwm1().bits()),
            )
            .field(
                "core_1_pif_pms_constrain_world_0_i2s1",
                &format_args!("{}", self.core_1_pif_pms_constrain_world_0_i2s1().bits()),
            )
            .field(
                "core_1_pif_pms_constrain_world_0_uart2",
                &format_args!("{}", self.core_1_pif_pms_constrain_world_0_uart2().bits()),
            )
            .field(
                "core_1_pif_pms_constrain_world_0_rwbt",
                &format_args!("{}", self.core_1_pif_pms_constrain_world_0_rwbt().bits()),
            )
            .field(
                "core_1_pif_pms_constrain_world_0_wifimac",
                &format_args!("{}", self.core_1_pif_pms_constrain_world_0_wifimac().bits()),
            )
            .field(
                "core_1_pif_pms_constrain_world_0_pwr",
                &format_args!("{}", self.core_1_pif_pms_constrain_world_0_pwr().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CORE_1_PIF_PMS_CONSTRAIN_3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:1 - Core1 access spi_2 permission in world0."]
    #[inline(always)]
    #[must_use]
    pub fn core_1_pif_pms_constrain_world_0_spi_2(
        &mut self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_SPI_2_W<0> {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_SPI_2_W::new(self)
    }
    #[doc = "Bits 2:3 - Core1 access spi_3 permission in world0."]
    #[inline(always)]
    #[must_use]
    pub fn core_1_pif_pms_constrain_world_0_spi_3(
        &mut self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_SPI_3_W<2> {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_SPI_3_W::new(self)
    }
    #[doc = "Bits 4:5 - Core1 access apb_ctrl permission in world0."]
    #[inline(always)]
    #[must_use]
    pub fn core_1_pif_pms_constrain_world_0_apb_ctrl(
        &mut self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_APB_CTRL_W<4> {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_APB_CTRL_W::new(self)
    }
    #[doc = "Bits 6:7 - Core1 access i2c_ext1 permission in world0."]
    #[inline(always)]
    #[must_use]
    pub fn core_1_pif_pms_constrain_world_0_i2c_ext1(
        &mut self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_I2C_EXT1_W<6> {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_I2C_EXT1_W::new(self)
    }
    #[doc = "Bits 8:9 - Core1 access sdio_host permission in world0."]
    #[inline(always)]
    #[must_use]
    pub fn core_1_pif_pms_constrain_world_0_sdio_host(
        &mut self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_SDIO_HOST_W<8> {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_SDIO_HOST_W::new(self)
    }
    #[doc = "Bits 10:11 - Core1 access can permission in world0."]
    #[inline(always)]
    #[must_use]
    pub fn core_1_pif_pms_constrain_world_0_can(
        &mut self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_CAN_W<10> {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_CAN_W::new(self)
    }
    #[doc = "Bits 12:13 - Core1 access pwm1 permission in world0."]
    #[inline(always)]
    #[must_use]
    pub fn core_1_pif_pms_constrain_world_0_pwm1(
        &mut self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_PWM1_W<12> {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_PWM1_W::new(self)
    }
    #[doc = "Bits 14:15 - Core1 access i2s1 permission in world0."]
    #[inline(always)]
    #[must_use]
    pub fn core_1_pif_pms_constrain_world_0_i2s1(
        &mut self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_I2S1_W<14> {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_I2S1_W::new(self)
    }
    #[doc = "Bits 16:17 - Core1 access uart2 permission in world0."]
    #[inline(always)]
    #[must_use]
    pub fn core_1_pif_pms_constrain_world_0_uart2(
        &mut self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_UART2_W<16> {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_UART2_W::new(self)
    }
    #[doc = "Bits 22:23 - Core1 access rwbt permission in world0."]
    #[inline(always)]
    #[must_use]
    pub fn core_1_pif_pms_constrain_world_0_rwbt(
        &mut self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_RWBT_W<22> {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_RWBT_W::new(self)
    }
    #[doc = "Bits 26:27 - Core1 access wifimac permission in world0."]
    #[inline(always)]
    #[must_use]
    pub fn core_1_pif_pms_constrain_world_0_wifimac(
        &mut self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_WIFIMAC_W<26> {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_WIFIMAC_W::new(self)
    }
    #[doc = "Bits 28:29 - Core1 access pwr permission in world0."]
    #[inline(always)]
    #[must_use]
    pub fn core_1_pif_pms_constrain_world_0_pwr(
        &mut self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_PWR_W<28> {
        CORE_1_PIF_PMS_CONSTRAIN_WORLD_0_PWR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Core1 access peripherals permission configuration register 3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_1_pif_pms_constrain_3](index.html) module"]
pub struct CORE_1_PIF_PMS_CONSTRAIN_3_SPEC;
impl crate::RegisterSpec for CORE_1_PIF_PMS_CONSTRAIN_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_1_pif_pms_constrain_3::R](R) reader structure"]
impl crate::Readable for CORE_1_PIF_PMS_CONSTRAIN_3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [core_1_pif_pms_constrain_3::W](W) writer structure"]
impl crate::Writable for CORE_1_PIF_PMS_CONSTRAIN_3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CORE_1_PIF_PMS_CONSTRAIN_3 to value 0x3cc3_ffff"]
impl crate::Resettable for CORE_1_PIF_PMS_CONSTRAIN_3_SPEC {
    const RESET_VALUE: Self::Ux = 0x3cc3_ffff;
}
