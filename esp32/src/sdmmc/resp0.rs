#[doc = "Register `RESP0` reader"]
pub struct R(crate::R<RESP0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESP0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESP0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESP0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RESPONSE0` reader - Bit\\[31:0\\] of response."]
pub type RESPONSE0_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Bit\\[31:0\\] of response."]
    #[inline(always)]
    pub fn response0(&self) -> RESPONSE0_R {
        RESPONSE0_R::new(self.bits)
    }
}
#[doc = "Response data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [resp0](index.html) module"]
pub struct RESP0_SPEC;
impl crate::RegisterSpec for RESP0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [resp0::R](R) reader structure"]
impl crate::Readable for RESP0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RESP0 to value 0"]
impl crate::Resettable for RESP0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
