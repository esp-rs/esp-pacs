#[doc = "Register `CORE_0_PIF_PMS_CONSTRAIN_7` reader"]
pub struct R(crate::R<CORE_0_PIF_PMS_CONSTRAIN_7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_0_PIF_PMS_CONSTRAIN_7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_0_PIF_PMS_CONSTRAIN_7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_0_PIF_PMS_CONSTRAIN_7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CORE_0_PIF_PMS_CONSTRAIN_7` writer"]
pub struct W(crate::W<CORE_0_PIF_PMS_CONSTRAIN_7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CORE_0_PIF_PMS_CONSTRAIN_7_SPEC>;
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
impl From<crate::W<CORE_0_PIF_PMS_CONSTRAIN_7_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CORE_0_PIF_PMS_CONSTRAIN_7_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_SPI_2` reader - core_0_pif_pms_constrain_world_1_spi_2"]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_SPI_2_R = crate::FieldReader;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_SPI_2` writer - core_0_pif_pms_constrain_world_1_spi_2"]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_SPI_2_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_0_PIF_PMS_CONSTRAIN_7_SPEC, 2, O>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_APB_CTRL` reader - core_0_pif_pms_constrain_world_1_apb_ctrl"]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_APB_CTRL_R = crate::FieldReader;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_APB_CTRL` writer - core_0_pif_pms_constrain_world_1_apb_ctrl"]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_APB_CTRL_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_0_PIF_PMS_CONSTRAIN_7_SPEC, 2, O>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_CAN` reader - core_0_pif_pms_constrain_world_1_can"]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_CAN_R = crate::FieldReader;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_CAN` writer - core_0_pif_pms_constrain_world_1_can"]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_CAN_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_0_PIF_PMS_CONSTRAIN_7_SPEC, 2, O>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_I2S1` reader - core_0_pif_pms_constrain_world_1_i2s1"]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_I2S1_R = crate::FieldReader;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_I2S1` writer - core_0_pif_pms_constrain_world_1_i2s1"]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_I2S1_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_0_PIF_PMS_CONSTRAIN_7_SPEC, 2, O>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_RWBT` reader - core_0_pif_pms_constrain_world_1_rwbt"]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_RWBT_R = crate::FieldReader;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_RWBT` writer - core_0_pif_pms_constrain_world_1_rwbt"]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_RWBT_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_0_PIF_PMS_CONSTRAIN_7_SPEC, 2, O>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_WIFIMAC` reader - core_0_pif_pms_constrain_world_1_wifimac"]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_WIFIMAC_R = crate::FieldReader;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_WIFIMAC` writer - core_0_pif_pms_constrain_world_1_wifimac"]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_WIFIMAC_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_0_PIF_PMS_CONSTRAIN_7_SPEC, 2, O>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_PWR` reader - core_0_pif_pms_constrain_world_1_pwr"]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_PWR_R = crate::FieldReader;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_PWR` writer - core_0_pif_pms_constrain_world_1_pwr"]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_PWR_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_0_PIF_PMS_CONSTRAIN_7_SPEC, 2, O>;
impl R {
    #[doc = "Bits 0:1 - core_0_pif_pms_constrain_world_1_spi_2"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_1_spi_2(
        &self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_SPI_2_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_SPI_2_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - core_0_pif_pms_constrain_world_1_apb_ctrl"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_1_apb_ctrl(
        &self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_APB_CTRL_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_APB_CTRL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 10:11 - core_0_pif_pms_constrain_world_1_can"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_1_can(&self) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_CAN_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_CAN_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 14:15 - core_0_pif_pms_constrain_world_1_i2s1"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_1_i2s1(&self) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_I2S1_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_I2S1_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 22:23 - core_0_pif_pms_constrain_world_1_rwbt"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_1_rwbt(&self) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_RWBT_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_RWBT_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 26:27 - core_0_pif_pms_constrain_world_1_wifimac"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_1_wifimac(
        &self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_WIFIMAC_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_WIFIMAC_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - core_0_pif_pms_constrain_world_1_pwr"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_1_pwr(&self) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_PWR_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_PWR_R::new(((self.bits >> 28) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_0_PIF_PMS_CONSTRAIN_7")
            .field(
                "core_0_pif_pms_constrain_world_1_spi_2",
                &format_args!("{}", self.core_0_pif_pms_constrain_world_1_spi_2().bits()),
            )
            .field(
                "core_0_pif_pms_constrain_world_1_apb_ctrl",
                &format_args!(
                    "{}",
                    self.core_0_pif_pms_constrain_world_1_apb_ctrl().bits()
                ),
            )
            .field(
                "core_0_pif_pms_constrain_world_1_can",
                &format_args!("{}", self.core_0_pif_pms_constrain_world_1_can().bits()),
            )
            .field(
                "core_0_pif_pms_constrain_world_1_i2s1",
                &format_args!("{}", self.core_0_pif_pms_constrain_world_1_i2s1().bits()),
            )
            .field(
                "core_0_pif_pms_constrain_world_1_rwbt",
                &format_args!("{}", self.core_0_pif_pms_constrain_world_1_rwbt().bits()),
            )
            .field(
                "core_0_pif_pms_constrain_world_1_wifimac",
                &format_args!("{}", self.core_0_pif_pms_constrain_world_1_wifimac().bits()),
            )
            .field(
                "core_0_pif_pms_constrain_world_1_pwr",
                &format_args!("{}", self.core_0_pif_pms_constrain_world_1_pwr().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CORE_0_PIF_PMS_CONSTRAIN_7_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:1 - core_0_pif_pms_constrain_world_1_spi_2"]
    #[inline(always)]
    #[must_use]
    pub fn core_0_pif_pms_constrain_world_1_spi_2(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_SPI_2_W<0> {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_SPI_2_W::new(self)
    }
    #[doc = "Bits 4:5 - core_0_pif_pms_constrain_world_1_apb_ctrl"]
    #[inline(always)]
    #[must_use]
    pub fn core_0_pif_pms_constrain_world_1_apb_ctrl(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_APB_CTRL_W<4> {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_APB_CTRL_W::new(self)
    }
    #[doc = "Bits 10:11 - core_0_pif_pms_constrain_world_1_can"]
    #[inline(always)]
    #[must_use]
    pub fn core_0_pif_pms_constrain_world_1_can(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_CAN_W<10> {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_CAN_W::new(self)
    }
    #[doc = "Bits 14:15 - core_0_pif_pms_constrain_world_1_i2s1"]
    #[inline(always)]
    #[must_use]
    pub fn core_0_pif_pms_constrain_world_1_i2s1(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_I2S1_W<14> {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_I2S1_W::new(self)
    }
    #[doc = "Bits 22:23 - core_0_pif_pms_constrain_world_1_rwbt"]
    #[inline(always)]
    #[must_use]
    pub fn core_0_pif_pms_constrain_world_1_rwbt(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_RWBT_W<22> {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_RWBT_W::new(self)
    }
    #[doc = "Bits 26:27 - core_0_pif_pms_constrain_world_1_wifimac"]
    #[inline(always)]
    #[must_use]
    pub fn core_0_pif_pms_constrain_world_1_wifimac(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_WIFIMAC_W<26> {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_WIFIMAC_W::new(self)
    }
    #[doc = "Bits 28:29 - core_0_pif_pms_constrain_world_1_pwr"]
    #[inline(always)]
    #[must_use]
    pub fn core_0_pif_pms_constrain_world_1_pwr(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_PWR_W<28> {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_PWR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_7_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_0_pif_pms_constrain_7](index.html) module"]
pub struct CORE_0_PIF_PMS_CONSTRAIN_7_SPEC;
impl crate::RegisterSpec for CORE_0_PIF_PMS_CONSTRAIN_7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_0_pif_pms_constrain_7::R](R) reader structure"]
impl crate::Readable for CORE_0_PIF_PMS_CONSTRAIN_7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [core_0_pif_pms_constrain_7::W](W) writer structure"]
impl crate::Writable for CORE_0_PIF_PMS_CONSTRAIN_7_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CORE_0_PIF_PMS_CONSTRAIN_7 to value 0x3cc0_cc33"]
impl crate::Resettable for CORE_0_PIF_PMS_CONSTRAIN_7_SPEC {
    const RESET_VALUE: Self::Ux = 0x3cc0_cc33;
}
