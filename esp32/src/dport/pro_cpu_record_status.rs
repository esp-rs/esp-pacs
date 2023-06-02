#[doc = "Register `PRO_CPU_RECORD_STATUS` reader"]
pub struct R(crate::R<PRO_CPU_RECORD_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRO_CPU_RECORD_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRO_CPU_RECORD_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRO_CPU_RECORD_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PRO_CPU_RECORDING` reader - "]
pub type PRO_CPU_RECORDING_R = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pro_cpu_recording(&self) -> PRO_CPU_RECORDING_R {
        PRO_CPU_RECORDING_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRO_CPU_RECORD_STATUS")
            .field(
                "pro_cpu_recording",
                &format_args!("{}", self.pro_cpu_recording().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PRO_CPU_RECORD_STATUS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pro_cpu_record_status](index.html) module"]
pub struct PRO_CPU_RECORD_STATUS_SPEC;
impl crate::RegisterSpec for PRO_CPU_RECORD_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pro_cpu_record_status::R](R) reader structure"]
impl crate::Readable for PRO_CPU_RECORD_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PRO_CPU_RECORD_STATUS to value 0"]
impl crate::Resettable for PRO_CPU_RECORD_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
