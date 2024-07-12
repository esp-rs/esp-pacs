#[doc = "Register `RESPONSE_BUF_PORT` reader"]
pub type R = crate::R<RESPONSE_BUF_PORT_SPEC>;
#[doc = "Field `RESPONSE` reader - The Response Buffer can be read through this register. The response status for each Command is written into the Response Buffer by the controller if ROC (Response On Completion) bit is set or if transfer error has occurred. The response buffer can be read through this register."]
pub type RESPONSE_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The Response Buffer can be read through this register. The response status for each Command is written into the Response Buffer by the controller if ROC (Response On Completion) bit is set or if transfer error has occurred. The response buffer can be read through this register."]
    #[inline(always)]
    pub fn response(&self) -> RESPONSE_R {
        RESPONSE_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RESPONSE_BUF_PORT")
            .field("response", &self.response())
            .finish()
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`response_buf_port::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RESPONSE_BUF_PORT_SPEC;
impl crate::RegisterSpec for RESPONSE_BUF_PORT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`response_buf_port::R`](R) reader structure"]
impl crate::Readable for RESPONSE_BUF_PORT_SPEC {}
#[doc = "`reset()` method sets RESPONSE_BUF_PORT to value 0"]
impl crate::Resettable for RESPONSE_BUF_PORT_SPEC {
    const RESET_VALUE: u32 = 0;
}
