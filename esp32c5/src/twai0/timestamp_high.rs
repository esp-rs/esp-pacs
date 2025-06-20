#[doc = "Register `TIMESTAMP_HIGH` reader"]
pub type R = crate::R<TIMESTAMP_HIGH_SPEC>;
#[doc = "Field `TIMESTAMP_HIGH` reader - Bits 63:32 of time base."]
pub type TIMESTAMP_HIGH_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Bits 63:32 of time base."]
    #[inline(always)]
    pub fn timestamp_high(&self) -> TIMESTAMP_HIGH_R {
        TIMESTAMP_HIGH_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMESTAMP_HIGH")
            .field("timestamp_high", &self.timestamp_high())
            .finish()
    }
}
#[doc = "TWAI FD transmitted frame counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`timestamp_high::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMESTAMP_HIGH_SPEC;
impl crate::RegisterSpec for TIMESTAMP_HIGH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timestamp_high::R`](R) reader structure"]
impl crate::Readable for TIMESTAMP_HIGH_SPEC {}
#[doc = "`reset()` method sets TIMESTAMP_HIGH to value 0"]
impl crate::Resettable for TIMESTAMP_HIGH_SPEC {
    const RESET_VALUE: u32 = 0;
}
