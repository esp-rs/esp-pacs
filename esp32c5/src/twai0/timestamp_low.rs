#[doc = "Register `TIMESTAMP_LOW` reader"]
pub type R = crate::R<TIMESTAMP_LOW_SPEC>;
#[doc = "Field `TIMESTAMP_LOW` reader - Bits 31:0 of time base."]
pub type TIMESTAMP_LOW_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Bits 31:0 of time base."]
    #[inline(always)]
    pub fn timestamp_low(&self) -> TIMESTAMP_LOW_R {
        TIMESTAMP_LOW_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMESTAMP_LOW")
            .field("timestamp_low", &self.timestamp_low())
            .finish()
    }
}
#[doc = "TWAI FD transmitted frame counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`timestamp_low::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMESTAMP_LOW_SPEC;
impl crate::RegisterSpec for TIMESTAMP_LOW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timestamp_low::R`](R) reader structure"]
impl crate::Readable for TIMESTAMP_LOW_SPEC {}
#[doc = "`reset()` method sets TIMESTAMP_LOW to value 0"]
impl crate::Resettable for TIMESTAMP_LOW_SPEC {}
