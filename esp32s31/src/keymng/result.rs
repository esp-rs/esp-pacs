#[doc = "Register `RESULT` reader"]
pub type R = crate::R<RESULT_SPEC>;
#[doc = "Field `PROC_RESULT` reader - The procedure result bit of Key Manager, only valid when Key Manager procedure is done. 1: Key Manager procedure succeeded. 0: Key Manager procedure failed."]
pub type PROC_RESULT_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The procedure result bit of Key Manager, only valid when Key Manager procedure is done. 1: Key Manager procedure succeeded. 0: Key Manager procedure failed."]
    #[inline(always)]
    pub fn proc_result(&self) -> PROC_RESULT_R {
        PROC_RESULT_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RESULT")
            .field("proc_result", &self.proc_result())
            .finish()
    }
}
#[doc = "Key Manager operation result register\n\nYou can [`read`](crate::Reg::read) this register and get [`result::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RESULT_SPEC;
impl crate::RegisterSpec for RESULT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`result::R`](R) reader structure"]
impl crate::Readable for RESULT_SPEC {}
#[doc = "`reset()` method sets RESULT to value 0"]
impl crate::Resettable for RESULT_SPEC {}
