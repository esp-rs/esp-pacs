#[doc = "Register `RESP1` reader"]
pub type R = crate::R<RESP1_SPEC>;
#[doc = "Field `RESPONSE1` reader - Bit\\[63:32\\] of long response."]
pub type RESPONSE1_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Bit\\[63:32\\] of long response."]
    #[inline(always)]
    pub fn response1(&self) -> RESPONSE1_R {
        RESPONSE1_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RESP1")
            .field("response1", &self.response1())
            .finish()
    }
}
#[doc = "Long response data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`resp1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RESP1_SPEC;
impl crate::RegisterSpec for RESP1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`resp1::R`](R) reader structure"]
impl crate::Readable for RESP1_SPEC {}
#[doc = "`reset()` method sets RESP1 to value 0"]
impl crate::Resettable for RESP1_SPEC {
    const RESET_VALUE: u32 = 0;
}
