#[doc = "Register `APP_INTRUSION_STATUS` reader"]
pub type R = crate::R<APP_INTRUSION_STATUS_SPEC>;
#[doc = "Field `APP_INTRUSION_RECORD` reader - "]
pub type APP_INTRUSION_RECORD_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn app_intrusion_record(&self) -> APP_INTRUSION_RECORD_R {
        APP_INTRUSION_RECORD_R::new((self.bits & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APP_INTRUSION_STATUS")
            .field("app_intrusion_record", &self.app_intrusion_record())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`app_intrusion_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APP_INTRUSION_STATUS_SPEC;
impl crate::RegisterSpec for APP_INTRUSION_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`app_intrusion_status::R`](R) reader structure"]
impl crate::Readable for APP_INTRUSION_STATUS_SPEC {}
#[doc = "`reset()` method sets APP_INTRUSION_STATUS to value 0"]
impl crate::Resettable for APP_INTRUSION_STATUS_SPEC {
    const RESET_VALUE: u32 = 0;
}
