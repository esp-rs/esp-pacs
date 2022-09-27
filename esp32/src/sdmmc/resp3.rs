#[doc = "Register `RESP3` reader"]
pub struct R(crate::R<RESP3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESP3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESP3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESP3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RESPONSE3` reader - Bit\\[127:96\\] of long response."]
pub type RESPONSE3_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Bit\\[127:96\\] of long response."]
    #[inline(always)]
    pub fn response3(&self) -> RESPONSE3_R {
        RESPONSE3_R::new(self.bits)
    }
}
#[doc = "Long response data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [resp3](index.html) module"]
pub struct RESP3_SPEC;
impl crate::RegisterSpec for RESP3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [resp3::R](R) reader structure"]
impl crate::Readable for RESP3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RESP3 to value 0"]
impl crate::Resettable for RESP3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
