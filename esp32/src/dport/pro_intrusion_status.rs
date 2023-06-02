#[doc = "Register `PRO_INTRUSION_STATUS` reader"]
pub struct R(crate::R<PRO_INTRUSION_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRO_INTRUSION_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRO_INTRUSION_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRO_INTRUSION_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PRO_INTRUSION_RECORD` reader - "]
pub type PRO_INTRUSION_RECORD_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn pro_intrusion_record(&self) -> PRO_INTRUSION_RECORD_R {
        PRO_INTRUSION_RECORD_R::new((self.bits & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRO_INTRUSION_STATUS")
            .field(
                "pro_intrusion_record",
                &format_args!("{}", self.pro_intrusion_record().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PRO_INTRUSION_STATUS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pro_intrusion_status](index.html) module"]
pub struct PRO_INTRUSION_STATUS_SPEC;
impl crate::RegisterSpec for PRO_INTRUSION_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pro_intrusion_status::R](R) reader structure"]
impl crate::Readable for PRO_INTRUSION_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PRO_INTRUSION_STATUS to value 0"]
impl crate::Resettable for PRO_INTRUSION_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
