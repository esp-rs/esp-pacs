#[doc = "Register `SAR1_STATUS` reader"]
pub struct R(crate::R<SAR1_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAR1_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAR1_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAR1_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SAR1_STATUS` reader - digital adc1 status"]
pub type SAR1_STATUS_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - digital adc1 status"]
    #[inline(always)]
    pub fn sar1_status(&self) -> SAR1_STATUS_R {
        SAR1_STATUS_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR1_STATUS")
            .field(
                "sar1_status",
                &format_args!("{}", self.sar1_status().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SAR1_STATUS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "digital adc1 status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar1_status](index.html) module"]
pub struct SAR1_STATUS_SPEC;
impl crate::RegisterSpec for SAR1_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sar1_status::R](R) reader structure"]
impl crate::Readable for SAR1_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SAR1_STATUS to value 0"]
impl crate::Resettable for SAR1_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
