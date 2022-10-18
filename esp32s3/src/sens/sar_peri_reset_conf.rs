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
pub type SAR_COCPU_RESET_R = crate::BitReader<bool>;
#[doc = "Field `SAR_COCPU_RESET` writer - enable ulp-riscv reset"]
pub type SAR_COCPU_RESET_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SAR_PERI_RESET_CONF_SPEC, bool, O>;
#[doc = "Field `SAR_RTC_I2C_RESET` reader - Reserved."]
pub type SAR_RTC_I2C_RESET_R = crate::BitReader<bool>;
#[doc = "Field `SAR_RTC_I2C_RESET` writer - Reserved."]
pub type SAR_RTC_I2C_RESET_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SAR_PERI_RESET_CONF_SPEC, bool, O>;
#[doc = "Field `SAR_TSENS_RESET` reader - enbale saradc reset"]
pub type SAR_TSENS_RESET_R = crate::BitReader<bool>;
#[doc = "Field `SAR_TSENS_RESET` writer - enbale saradc reset"]
pub type SAR_TSENS_RESET_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SAR_PERI_RESET_CONF_SPEC, bool, O>;
#[doc = "Field `SAR_SARADC_RESET` reader - enable io_mux reset"]
pub type SAR_SARADC_RESET_R = crate::BitReader<bool>;
#[doc = "Field `SAR_SARADC_RESET` writer - enable io_mux reset"]
pub type SAR_SARADC_RESET_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SAR_PERI_RESET_CONF_SPEC, bool, O>;
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
impl W {
    #[doc = "Bit 25 - enable ulp-riscv reset"]
    #[inline(always)]
    pub fn sar_cocpu_reset(&mut self) -> SAR_COCPU_RESET_W<25> {
        SAR_COCPU_RESET_W::new(self)
    }
    #[doc = "Bit 27 - Reserved."]
    #[inline(always)]
    pub fn sar_rtc_i2c_reset(&mut self) -> SAR_RTC_I2C_RESET_W<27> {
        SAR_RTC_I2C_RESET_W::new(self)
    }
    #[doc = "Bit 29 - enbale saradc reset"]
    #[inline(always)]
    pub fn sar_tsens_reset(&mut self) -> SAR_TSENS_RESET_W<29> {
        SAR_TSENS_RESET_W::new(self)
    }
    #[doc = "Bit 30 - enable io_mux reset"]
    #[inline(always)]
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
}
#[doc = "`reset()` method sets SAR_PERI_RESET_CONF to value 0"]
impl crate::Resettable for SAR_PERI_RESET_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
