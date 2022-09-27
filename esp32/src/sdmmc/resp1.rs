#[doc = "Register `RESP1` reader"]
pub struct R(crate::R<RESP1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESP1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESP1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESP1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RESPONSE1` reader - Bit\\[63:32\\] of long response."]
pub type RESPONSE1_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Bit\\[63:32\\] of long response."]
    #[inline(always)]
    pub fn response1(&self) -> RESPONSE1_R {
        RESPONSE1_R::new(self.bits)
    }
}
#[doc = "Long response data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [resp1](index.html) module"]
pub struct RESP1_SPEC;
impl crate::RegisterSpec for RESP1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [resp1::R](R) reader structure"]
impl crate::Readable for RESP1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RESP1 to value 0"]
impl crate::Resettable for RESP1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
