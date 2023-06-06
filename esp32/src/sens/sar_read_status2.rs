#[doc = "Register `SAR_READ_STATUS2` reader"]
pub struct R(crate::R<SAR_READ_STATUS2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAR_READ_STATUS2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAR_READ_STATUS2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAR_READ_STATUS2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SAR2_READER_STATUS` reader - "]
pub type SAR2_READER_STATUS_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sar2_reader_status(&self) -> SAR2_READER_STATUS_R {
        SAR2_READER_STATUS_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_READ_STATUS2")
            .field(
                "sar2_reader_status",
                &format_args!("{}", self.sar2_reader_status().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SAR_READ_STATUS2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar_read_status2](index.html) module"]
pub struct SAR_READ_STATUS2_SPEC;
impl crate::RegisterSpec for SAR_READ_STATUS2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sar_read_status2::R](R) reader structure"]
impl crate::Readable for SAR_READ_STATUS2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SAR_READ_STATUS2 to value 0"]
impl crate::Resettable for SAR_READ_STATUS2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
