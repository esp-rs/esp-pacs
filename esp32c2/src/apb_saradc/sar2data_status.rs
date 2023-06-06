#[doc = "Register `SAR2DATA_STATUS` reader"]
pub struct R(crate::R<SAR2DATA_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAR2DATA_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAR2DATA_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAR2DATA_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `APB_SARADC2_DATA` reader - Need add description"]
pub type APB_SARADC2_DATA_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:16 - Need add description"]
    #[inline(always)]
    pub fn apb_saradc2_data(&self) -> APB_SARADC2_DATA_R {
        APB_SARADC2_DATA_R::new(self.bits & 0x0001_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR2DATA_STATUS")
            .field(
                "apb_saradc2_data",
                &format_args!("{}", self.apb_saradc2_data().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SAR2DATA_STATUS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar2data_status](index.html) module"]
pub struct SAR2DATA_STATUS_SPEC;
impl crate::RegisterSpec for SAR2DATA_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sar2data_status::R](R) reader structure"]
impl crate::Readable for SAR2DATA_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SAR2DATA_STATUS to value 0"]
impl crate::Resettable for SAR2DATA_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
