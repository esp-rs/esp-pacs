#[doc = "Register `SAR_MEAS2_MUX` reader"]
pub struct R(crate::R<SAR_MEAS2_MUX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAR_MEAS2_MUX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAR_MEAS2_MUX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAR_MEAS2_MUX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAR_MEAS2_MUX` writer"]
pub struct W(crate::W<SAR_MEAS2_MUX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAR_MEAS2_MUX_SPEC>;
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
impl From<crate::W<SAR_MEAS2_MUX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAR_MEAS2_MUX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SAR2_PWDET_CCT` reader - SAR2_PWDET_CCT"]
pub type SAR2_PWDET_CCT_R = crate::FieldReader;
#[doc = "Field `SAR2_PWDET_CCT` writer - SAR2_PWDET_CCT"]
pub type SAR2_PWDET_CCT_W<'a, const O: u8> = crate::FieldWriter<'a, SAR_MEAS2_MUX_SPEC, 3, O>;
#[doc = "Field `SAR2_RTC_FORCE` reader - in sleep, force to use rtc to control ADC"]
pub type SAR2_RTC_FORCE_R = crate::BitReader;
#[doc = "Field `SAR2_RTC_FORCE` writer - in sleep, force to use rtc to control ADC"]
pub type SAR2_RTC_FORCE_W<'a, const O: u8> = crate::BitWriter<'a, SAR_MEAS2_MUX_SPEC, O>;
impl R {
    #[doc = "Bits 28:30 - SAR2_PWDET_CCT"]
    #[inline(always)]
    pub fn sar2_pwdet_cct(&self) -> SAR2_PWDET_CCT_R {
        SAR2_PWDET_CCT_R::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31 - in sleep, force to use rtc to control ADC"]
    #[inline(always)]
    pub fn sar2_rtc_force(&self) -> SAR2_RTC_FORCE_R {
        SAR2_RTC_FORCE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_MEAS2_MUX")
            .field(
                "sar2_pwdet_cct",
                &format_args!("{}", self.sar2_pwdet_cct().bits()),
            )
            .field(
                "sar2_rtc_force",
                &format_args!("{}", self.sar2_rtc_force().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SAR_MEAS2_MUX_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 28:30 - SAR2_PWDET_CCT"]
    #[inline(always)]
    #[must_use]
    pub fn sar2_pwdet_cct(&mut self) -> SAR2_PWDET_CCT_W<28> {
        SAR2_PWDET_CCT_W::new(self)
    }
    #[doc = "Bit 31 - in sleep, force to use rtc to control ADC"]
    #[inline(always)]
    #[must_use]
    pub fn sar2_rtc_force(&mut self) -> SAR2_RTC_FORCE_W<31> {
        SAR2_RTC_FORCE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "configure saradc2 controller\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar_meas2_mux](index.html) module"]
pub struct SAR_MEAS2_MUX_SPEC;
impl crate::RegisterSpec for SAR_MEAS2_MUX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sar_meas2_mux::R](R) reader structure"]
impl crate::Readable for SAR_MEAS2_MUX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sar_meas2_mux::W](W) writer structure"]
impl crate::Writable for SAR_MEAS2_MUX_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SAR_MEAS2_MUX to value 0"]
impl crate::Resettable for SAR_MEAS2_MUX_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
