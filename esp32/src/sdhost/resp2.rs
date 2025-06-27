#[doc = "Register `RESP2` reader"]
pub type R = crate::R<RESP2_SPEC>;
#[doc = "Field `RESPONSE2` reader - Bit\\[95:64\\] of long response."]
pub type RESPONSE2_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Bit\\[95:64\\] of long response."]
    #[inline(always)]
    pub fn response2(&self) -> RESPONSE2_R {
        RESPONSE2_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RESP2")
            .field("response2", &self.response2())
            .finish()
    }
}
#[doc = "Long response data register\n\nYou can [`read`](crate::Reg::read) this register and get [`resp2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RESP2_SPEC;
impl crate::RegisterSpec for RESP2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`resp2::R`](R) reader structure"]
impl crate::Readable for RESP2_SPEC {}
#[doc = "`reset()` method sets RESP2 to value 0"]
impl crate::Resettable for RESP2_SPEC {}
