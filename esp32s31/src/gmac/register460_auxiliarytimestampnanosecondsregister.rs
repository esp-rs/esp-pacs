#[doc = "Register `REGISTER460_AUXILIARYTIMESTAMPNANOSECONDSREGISTER` reader"]
pub type R = crate::R<REGISTER460_AUXILIARYTIMESTAMPNANOSECONDSREGISTER_SPEC>;
#[doc = "Field `AUXTSLO` reader - Contains the lower 31 bits _nanoseconds field_ of the auxiliary timestamp"]
pub type AUXTSLO_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:30 - Contains the lower 31 bits _nanoseconds field_ of the auxiliary timestamp"]
    #[inline(always)]
    pub fn auxtslo(&self) -> AUXTSLO_R {
        AUXTSLO_R::new(self.bits & 0x7fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGISTER460_AUXILIARYTIMESTAMPNANOSECONDSREGISTER")
            .field("auxtslo", &self.auxtslo())
            .finish()
    }
}
#[doc = "Contains the lower 32 bits _nanoseconds field_ of the auxiliary timestamp register\n\nYou can [`read`](crate::Reg::read) this register and get [`register460_auxiliarytimestampnanosecondsregister::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGISTER460_AUXILIARYTIMESTAMPNANOSECONDSREGISTER_SPEC;
impl crate::RegisterSpec for REGISTER460_AUXILIARYTIMESTAMPNANOSECONDSREGISTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`register460_auxiliarytimestampnanosecondsregister::R`](R) reader structure"]
impl crate::Readable for REGISTER460_AUXILIARYTIMESTAMPNANOSECONDSREGISTER_SPEC {}
#[doc = "`reset()` method sets REGISTER460_AUXILIARYTIMESTAMPNANOSECONDSREGISTER to value 0"]
impl crate::Resettable for REGISTER460_AUXILIARYTIMESTAMPNANOSECONDSREGISTER_SPEC {}
