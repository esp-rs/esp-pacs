#[doc = "Register `RESULT` reader"]
pub type R = crate::R<RESULT_SPEC>;
#[doc = "Field `OPERATION_RESULT` reader - Indicates if the ECDSA operation is successful.\\\\ 0: Not successful\\\\ 1: Successful\\\\ Only valid when the ECDSA operation is done. \\\\"]
pub type OPERATION_RESULT_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Indicates if the ECDSA operation is successful.\\\\ 0: Not successful\\\\ 1: Successful\\\\ Only valid when the ECDSA operation is done. \\\\"]
    #[inline(always)]
    pub fn operation_result(&self) -> OPERATION_RESULT_R {
        OPERATION_RESULT_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RESULT")
            .field("operation_result", &self.operation_result())
            .finish()
    }
}
#[doc = "ECDSA result register\n\nYou can [`read`](crate::Reg::read) this register and get [`result::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RESULT_SPEC;
impl crate::RegisterSpec for RESULT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`result::R`](R) reader structure"]
impl crate::Readable for RESULT_SPEC {}
#[doc = "`reset()` method sets RESULT to value 0"]
impl crate::Resettable for RESULT_SPEC {}
