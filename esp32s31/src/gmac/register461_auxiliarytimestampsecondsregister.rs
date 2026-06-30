#[doc = "Register `REGISTER461_AUXILIARYTIMESTAMPSECONDSREGISTER` reader"]
pub type R = crate::R<REGISTER461_AUXILIARYTIMESTAMPSECONDSREGISTER_SPEC>;
#[doc = "Field `AUXTSHI` reader - Contains the lower 32 bits of the Seconds field of the auxiliary timestamp"]
pub type AUXTSHI_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Contains the lower 32 bits of the Seconds field of the auxiliary timestamp"]
    #[inline(always)]
    pub fn auxtshi(&self) -> AUXTSHI_R {
        AUXTSHI_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGISTER461_AUXILIARYTIMESTAMPSECONDSREGISTER")
            .field("auxtshi", &self.auxtshi())
            .finish()
    }
}
#[doc = "Contains the lower 32 bits of the Seconds field of the auxiliary timestamp register\n\nYou can [`read`](crate::Reg::read) this register and get [`register461_auxiliarytimestampsecondsregister::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGISTER461_AUXILIARYTIMESTAMPSECONDSREGISTER_SPEC;
impl crate::RegisterSpec for REGISTER461_AUXILIARYTIMESTAMPSECONDSREGISTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`register461_auxiliarytimestampsecondsregister::R`](R) reader structure"]
impl crate::Readable for REGISTER461_AUXILIARYTIMESTAMPSECONDSREGISTER_SPEC {}
#[doc = "`reset()` method sets REGISTER461_AUXILIARYTIMESTAMPSECONDSREGISTER to value 0"]
impl crate::Resettable for REGISTER461_AUXILIARYTIMESTAMPSECONDSREGISTER_SPEC {}
