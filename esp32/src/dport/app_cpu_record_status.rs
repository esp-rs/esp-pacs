#[doc = "Register `APP_CPU_RECORD_STATUS` reader"]
pub type R = crate::R<APP_CPU_RECORD_STATUS_SPEC>;
#[doc = "Field `APP_CPU_RECORDING` reader - "]
pub type APP_CPU_RECORDING_R = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn app_cpu_recording(&self) -> APP_CPU_RECORDING_R {
        APP_CPU_RECORDING_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APP_CPU_RECORD_STATUS")
            .field("app_cpu_recording", &self.app_cpu_recording().bit())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<APP_CPU_RECORD_STATUS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_cpu_record_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APP_CPU_RECORD_STATUS_SPEC;
impl crate::RegisterSpec for APP_CPU_RECORD_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`app_cpu_record_status::R`](R) reader structure"]
impl crate::Readable for APP_CPU_RECORD_STATUS_SPEC {}
#[doc = "`reset()` method sets APP_CPU_RECORD_STATUS to value 0"]
impl crate::Resettable for APP_CPU_RECORD_STATUS_SPEC {
    const RESET_VALUE: u32 = 0;
}
