#[doc = "Register `RESP2` reader"]
pub struct R(crate::R<RESP2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESP2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESP2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESP2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RESPONSE2` reader - Bit\\[95:64\\] of long response."]
pub type RESPONSE2_R = crate::FieldReader<u32, u32>;
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
            .field("response2", &format_args!("{}", self.response2().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RESP2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Long response data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [resp2](index.html) module"]
pub struct RESP2_SPEC;
impl crate::RegisterSpec for RESP2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [resp2::R](R) reader structure"]
impl crate::Readable for RESP2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RESP2 to value 0"]
impl crate::Resettable for RESP2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
