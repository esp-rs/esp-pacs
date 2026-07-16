#[doc = "Register `IN2` reader"]
pub type R = crate::R<IN2_SPEC>;
#[doc = "Field `DATA_NEXT` reader - Represents the input value of GPIO64 ~ GPIO66. Each bit represents a pin input value:\\\\ 0: Low level\\\\ 1: High level\\\\ Bit64 ~ bit66 are corresponding to GPIO64 ~ GPIO66. Bitxx ~ bitxx is invalid.\\\\"]
pub type DATA_NEXT_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - Represents the input value of GPIO64 ~ GPIO66. Each bit represents a pin input value:\\\\ 0: Low level\\\\ 1: High level\\\\ Bit64 ~ bit66 are corresponding to GPIO64 ~ GPIO66. Bitxx ~ bitxx is invalid.\\\\"]
    #[inline(always)]
    pub fn data_next(&self) -> DATA_NEXT_R {
        DATA_NEXT_R::new((self.bits & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IN2")
            .field("data_next", &self.data_next())
            .finish()
    }
}
#[doc = "GPIO input register\n\nYou can [`read`](crate::Reg::read) this register and get [`in2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IN2_SPEC;
impl crate::RegisterSpec for IN2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in2::R`](R) reader structure"]
impl crate::Readable for IN2_SPEC {}
#[doc = "`reset()` method sets IN2 to value 0"]
impl crate::Resettable for IN2_SPEC {}
