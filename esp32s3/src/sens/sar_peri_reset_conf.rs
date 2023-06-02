#[doc = "Register `SAR_PERI_RESET_CONF` reader"]
pub struct R(crate::R<SAR_PERI_RESET_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAR_PERI_RESET_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAR_PERI_RESET_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAR_PERI_RESET_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAR_PERI_RESET_CONF` writer"]
pub struct W(crate::W<SAR_PERI_RESET_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAR_PERI_RESET_CONF_SPEC>;
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
impl From<crate::W<SAR_PERI_RESET_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAR_PERI_RESET_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SAR_COCPU_RESET` reader - enable ulp-riscv reset"]
pub type SAR_COCPU_RESET_R = crate::BitReader;
#[doc = "Field `SAR_COCPU_RESET` writer - enable ulp-riscv reset"]
pub type SAR_COCPU_RESET_W<'a, const O: u8> = crate::BitWriter<'a, SAR_PERI_RESET_CONF_SPEC, O>;
#[doc = "Field `SAR_RTC_I2C_RESET` reader - Reserved."]
pub type SAR_RTC_I2C_RESET_R = crate::BitReader;
#[doc = "Field `SAR_RTC_I2C_RESET` writer - Reserved."]
pub type SAR_RTC_I2C_RESET_W<'a, const O: u8> = crate::BitWriter<'a, SAR_PERI_RESET_CONF_SPEC, O>;
#[doc = "Field `SAR_TSENS_RESET` reader - enbale saradc reset"]
pub type SAR_TSENS_RESET_R = crate::BitReader;
#[doc = "Field `SAR_TSENS_RESET` writer - enbale saradc reset"]
pub type SAR_TSENS_RESET_W<'a, const O: u8> = crate::BitWriter<'a, SAR_PERI_RESET_CONF_SPEC, O>;
#[doc = "Field `SAR_SARADC_RESET` reader - enable io_mux reset"]
pub type SAR_SARADC_RESET_R = crate::BitReader;
#[doc = "Field `SAR_SARADC_RESET` writer - enable io_mux reset"]
pub type SAR_SARADC_RESET_W<'a, const O: u8> = crate::BitWriter<'a, SAR_PERI_RESET_CONF_SPEC, O>;
impl R {
    #[doc = "Bit 25 - enable ulp-riscv reset"]
    #[inline(always)]
    pub fn sar_cocpu_reset(&self) -> SAR_COCPU_RESET_R {
        SAR_COCPU_RESET_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 27 - Reserved."]
    #[inline(always)]
    pub fn sar_rtc_i2c_reset(&self) -> SAR_RTC_I2C_RESET_R {
        SAR_RTC_I2C_RESET_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 29 - enbale saradc reset"]
    #[inline(always)]
    pub fn sar_tsens_reset(&self) -> SAR_TSENS_RESET_R {
        SAR_TSENS_RESET_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - enable io_mux reset"]
    #[inline(always)]
    pub fn sar_saradc_reset(&self) -> SAR_SARADC_RESET_R {
        SAR_SARADC_RESET_R::new(((self.bits >> 30) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_PERI_RESET_CONF")
            .field(
                "sar_cocpu_reset",
                &format_args!("{}", self.sar_cocpu_reset().bit()),
            )
            .field(
                "sar_rtc_i2c_reset",
                &format_args!("{}", self.sar_rtc_i2c_reset().bit()),
            )
            .field(
                "sar_tsens_reset",
                &format_args!("{}", self.sar_tsens_reset().bit()),
            )
            .field(
                "sar_saradc_reset",
                &format_args!("{}", self.sar_saradc_reset().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SAR_PERI_RESET_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 25 - enable ulp-riscv reset"]
    #[inline(always)]
    #[must_use]
    pub fn sar_cocpu_reset(&mut self) -> SAR_COCPU_RESET_W<25> {
        SAR_COCPU_RESET_W::new(self)
    }
    #[doc = "Bit 27 - Reserved."]
    #[inline(always)]
    #[must_use]
    pub fn sar_rtc_i2c_reset(&mut self) -> SAR_RTC_I2C_RESET_W<27> {
        SAR_RTC_I2C_RESET_W::new(self)
    }
    #[doc = "Bit 29 - enbale saradc reset"]
    #[inline(always)]
    #[must_use]
    pub fn sar_tsens_reset(&mut self) -> SAR_TSENS_RESET_W<29> {
        SAR_TSENS_RESET_W::new(self)
    }
    #[doc = "Bit 30 - enable io_mux reset"]
    #[inline(always)]
    #[must_use]
    pub fn sar_saradc_reset(&mut self) -> SAR_SARADC_RESET_W<30> {
        SAR_SARADC_RESET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "the peri reset of rtc peri\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar_peri_reset_conf](index.html) module"]
pub struct SAR_PERI_RESET_CONF_SPEC;
impl crate::RegisterSpec for SAR_PERI_RESET_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sar_peri_reset_conf::R](R) reader structure"]
impl crate::Readable for SAR_PERI_RESET_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sar_peri_reset_conf::W](W) writer structure"]
impl crate::Writable for SAR_PERI_RESET_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SAR_PERI_RESET_CONF to value 0"]
impl crate::Resettable for SAR_PERI_RESET_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
