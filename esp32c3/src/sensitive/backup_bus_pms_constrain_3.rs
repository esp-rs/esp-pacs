#[doc = "Register `BACKUP_BUS_PMS_CONSTRAIN_3` reader"]
pub struct R(crate::R<BACKUP_BUS_PMS_CONSTRAIN_3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BACKUP_BUS_PMS_CONSTRAIN_3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BACKUP_BUS_PMS_CONSTRAIN_3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BACKUP_BUS_PMS_CONSTRAIN_3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BACKUP_BUS_PMS_CONSTRAIN_3` writer"]
pub struct W(crate::W<BACKUP_BUS_PMS_CONSTRAIN_3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BACKUP_BUS_PMS_CONSTRAIN_3_SPEC>;
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
impl From<crate::W<BACKUP_BUS_PMS_CONSTRAIN_3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BACKUP_BUS_PMS_CONSTRAIN_3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_SPI_2` reader - backup_bus_pms_constrain_spi_2"]
pub type BACKUP_BUS_PMS_CONSTRAIN_SPI_2_R = crate::FieldReader;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_SPI_2` writer - backup_bus_pms_constrain_spi_2"]
pub type BACKUP_BUS_PMS_CONSTRAIN_SPI_2_W<'a, const O: u8> =
    crate::FieldWriter<'a, BACKUP_BUS_PMS_CONSTRAIN_3_SPEC, 2, O>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_APB_CTRL` reader - backup_bus_pms_constrain_apb_ctrl"]
pub type BACKUP_BUS_PMS_CONSTRAIN_APB_CTRL_R = crate::FieldReader;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_APB_CTRL` writer - backup_bus_pms_constrain_apb_ctrl"]
pub type BACKUP_BUS_PMS_CONSTRAIN_APB_CTRL_W<'a, const O: u8> =
    crate::FieldWriter<'a, BACKUP_BUS_PMS_CONSTRAIN_3_SPEC, 2, O>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_CAN` reader - backup_bus_pms_constrain_can"]
pub type BACKUP_BUS_PMS_CONSTRAIN_CAN_R = crate::FieldReader;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_CAN` writer - backup_bus_pms_constrain_can"]
pub type BACKUP_BUS_PMS_CONSTRAIN_CAN_W<'a, const O: u8> =
    crate::FieldWriter<'a, BACKUP_BUS_PMS_CONSTRAIN_3_SPEC, 2, O>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_I2S1` reader - backup_bus_pms_constrain_i2s1"]
pub type BACKUP_BUS_PMS_CONSTRAIN_I2S1_R = crate::FieldReader;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_I2S1` writer - backup_bus_pms_constrain_i2s1"]
pub type BACKUP_BUS_PMS_CONSTRAIN_I2S1_W<'a, const O: u8> =
    crate::FieldWriter<'a, BACKUP_BUS_PMS_CONSTRAIN_3_SPEC, 2, O>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_RWBT` reader - backup_bus_pms_constrain_rwbt"]
pub type BACKUP_BUS_PMS_CONSTRAIN_RWBT_R = crate::FieldReader;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_RWBT` writer - backup_bus_pms_constrain_rwbt"]
pub type BACKUP_BUS_PMS_CONSTRAIN_RWBT_W<'a, const O: u8> =
    crate::FieldWriter<'a, BACKUP_BUS_PMS_CONSTRAIN_3_SPEC, 2, O>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_WIFIMAC` reader - backup_bus_pms_constrain_wifimac"]
pub type BACKUP_BUS_PMS_CONSTRAIN_WIFIMAC_R = crate::FieldReader;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_WIFIMAC` writer - backup_bus_pms_constrain_wifimac"]
pub type BACKUP_BUS_PMS_CONSTRAIN_WIFIMAC_W<'a, const O: u8> =
    crate::FieldWriter<'a, BACKUP_BUS_PMS_CONSTRAIN_3_SPEC, 2, O>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_PWR` reader - backup_bus_pms_constrain_pwr"]
pub type BACKUP_BUS_PMS_CONSTRAIN_PWR_R = crate::FieldReader;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_PWR` writer - backup_bus_pms_constrain_pwr"]
pub type BACKUP_BUS_PMS_CONSTRAIN_PWR_W<'a, const O: u8> =
    crate::FieldWriter<'a, BACKUP_BUS_PMS_CONSTRAIN_3_SPEC, 2, O>;
impl R {
    #[doc = "Bits 0:1 - backup_bus_pms_constrain_spi_2"]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_spi_2(&self) -> BACKUP_BUS_PMS_CONSTRAIN_SPI_2_R {
        BACKUP_BUS_PMS_CONSTRAIN_SPI_2_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - backup_bus_pms_constrain_apb_ctrl"]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_apb_ctrl(&self) -> BACKUP_BUS_PMS_CONSTRAIN_APB_CTRL_R {
        BACKUP_BUS_PMS_CONSTRAIN_APB_CTRL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 10:11 - backup_bus_pms_constrain_can"]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_can(&self) -> BACKUP_BUS_PMS_CONSTRAIN_CAN_R {
        BACKUP_BUS_PMS_CONSTRAIN_CAN_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 14:15 - backup_bus_pms_constrain_i2s1"]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_i2s1(&self) -> BACKUP_BUS_PMS_CONSTRAIN_I2S1_R {
        BACKUP_BUS_PMS_CONSTRAIN_I2S1_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 22:23 - backup_bus_pms_constrain_rwbt"]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_rwbt(&self) -> BACKUP_BUS_PMS_CONSTRAIN_RWBT_R {
        BACKUP_BUS_PMS_CONSTRAIN_RWBT_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 26:27 - backup_bus_pms_constrain_wifimac"]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_wifimac(&self) -> BACKUP_BUS_PMS_CONSTRAIN_WIFIMAC_R {
        BACKUP_BUS_PMS_CONSTRAIN_WIFIMAC_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - backup_bus_pms_constrain_pwr"]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_pwr(&self) -> BACKUP_BUS_PMS_CONSTRAIN_PWR_R {
        BACKUP_BUS_PMS_CONSTRAIN_PWR_R::new(((self.bits >> 28) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BACKUP_BUS_PMS_CONSTRAIN_3")
            .field(
                "backup_bus_pms_constrain_spi_2",
                &format_args!("{}", self.backup_bus_pms_constrain_spi_2().bits()),
            )
            .field(
                "backup_bus_pms_constrain_apb_ctrl",
                &format_args!("{}", self.backup_bus_pms_constrain_apb_ctrl().bits()),
            )
            .field(
                "backup_bus_pms_constrain_can",
                &format_args!("{}", self.backup_bus_pms_constrain_can().bits()),
            )
            .field(
                "backup_bus_pms_constrain_i2s1",
                &format_args!("{}", self.backup_bus_pms_constrain_i2s1().bits()),
            )
            .field(
                "backup_bus_pms_constrain_rwbt",
                &format_args!("{}", self.backup_bus_pms_constrain_rwbt().bits()),
            )
            .field(
                "backup_bus_pms_constrain_wifimac",
                &format_args!("{}", self.backup_bus_pms_constrain_wifimac().bits()),
            )
            .field(
                "backup_bus_pms_constrain_pwr",
                &format_args!("{}", self.backup_bus_pms_constrain_pwr().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<BACKUP_BUS_PMS_CONSTRAIN_3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:1 - backup_bus_pms_constrain_spi_2"]
    #[inline(always)]
    #[must_use]
    pub fn backup_bus_pms_constrain_spi_2(&mut self) -> BACKUP_BUS_PMS_CONSTRAIN_SPI_2_W<0> {
        BACKUP_BUS_PMS_CONSTRAIN_SPI_2_W::new(self)
    }
    #[doc = "Bits 4:5 - backup_bus_pms_constrain_apb_ctrl"]
    #[inline(always)]
    #[must_use]
    pub fn backup_bus_pms_constrain_apb_ctrl(&mut self) -> BACKUP_BUS_PMS_CONSTRAIN_APB_CTRL_W<4> {
        BACKUP_BUS_PMS_CONSTRAIN_APB_CTRL_W::new(self)
    }
    #[doc = "Bits 10:11 - backup_bus_pms_constrain_can"]
    #[inline(always)]
    #[must_use]
    pub fn backup_bus_pms_constrain_can(&mut self) -> BACKUP_BUS_PMS_CONSTRAIN_CAN_W<10> {
        BACKUP_BUS_PMS_CONSTRAIN_CAN_W::new(self)
    }
    #[doc = "Bits 14:15 - backup_bus_pms_constrain_i2s1"]
    #[inline(always)]
    #[must_use]
    pub fn backup_bus_pms_constrain_i2s1(&mut self) -> BACKUP_BUS_PMS_CONSTRAIN_I2S1_W<14> {
        BACKUP_BUS_PMS_CONSTRAIN_I2S1_W::new(self)
    }
    #[doc = "Bits 22:23 - backup_bus_pms_constrain_rwbt"]
    #[inline(always)]
    #[must_use]
    pub fn backup_bus_pms_constrain_rwbt(&mut self) -> BACKUP_BUS_PMS_CONSTRAIN_RWBT_W<22> {
        BACKUP_BUS_PMS_CONSTRAIN_RWBT_W::new(self)
    }
    #[doc = "Bits 26:27 - backup_bus_pms_constrain_wifimac"]
    #[inline(always)]
    #[must_use]
    pub fn backup_bus_pms_constrain_wifimac(&mut self) -> BACKUP_BUS_PMS_CONSTRAIN_WIFIMAC_W<26> {
        BACKUP_BUS_PMS_CONSTRAIN_WIFIMAC_W::new(self)
    }
    #[doc = "Bits 28:29 - backup_bus_pms_constrain_pwr"]
    #[inline(always)]
    #[must_use]
    pub fn backup_bus_pms_constrain_pwr(&mut self) -> BACKUP_BUS_PMS_CONSTRAIN_PWR_W<28> {
        BACKUP_BUS_PMS_CONSTRAIN_PWR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_3_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [backup_bus_pms_constrain_3](index.html) module"]
pub struct BACKUP_BUS_PMS_CONSTRAIN_3_SPEC;
impl crate::RegisterSpec for BACKUP_BUS_PMS_CONSTRAIN_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [backup_bus_pms_constrain_3::R](R) reader structure"]
impl crate::Readable for BACKUP_BUS_PMS_CONSTRAIN_3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [backup_bus_pms_constrain_3::W](W) writer structure"]
impl crate::Writable for BACKUP_BUS_PMS_CONSTRAIN_3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BACKUP_BUS_PMS_CONSTRAIN_3 to value 0x3cc0_cc33"]
impl crate::Resettable for BACKUP_BUS_PMS_CONSTRAIN_3_SPEC {
    const RESET_VALUE: Self::Ux = 0x3cc0_cc33;
}
