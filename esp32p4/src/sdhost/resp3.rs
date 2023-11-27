#[doc = "Register `RESP3` reader"]
pub type R = crate::R<RESP3_SPEC>;
#[doc = "Field `RESPONSE3` reader - Bit\\[127:96\\] of long response."]
pub type RESPONSE3_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Bit\\[127:96\\] of long response."]
    #[inline(always)]
    pub fn response3(&self) -> RESPONSE3_R {
        RESPONSE3_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RESP3")
            .field("response3", &format_args!("{}", self.response3().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RESP3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Long response data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`resp3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RESP3_SPEC;
impl crate::RegisterSpec for RESP3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`resp3::R`](R) reader structure"]
impl crate::Readable for RESP3_SPEC {}
#[doc = "`reset()` method sets RESP3 to value 0"]
impl crate::Resettable for RESP3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
