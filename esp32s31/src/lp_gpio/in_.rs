#[doc = "Register `IN` reader"]
pub type R = crate::R<IN_SPEC>;
#[doc = "Field `DATA_NEXT` reader - Represents the input value of LP_GPIO0 ~ LP_GPIO7. Each bit represents a pin input value:\\\\ 0: Low level\\\\ 1: High level\\\\ Bit0 ~ bit7 are corresponding to LP_GPIO0 ~ LP_GPIO7. Bitxx ~ bitxx is invalid.\\\\"]
pub type DATA_NEXT_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Represents the input value of LP_GPIO0 ~ LP_GPIO7. Each bit represents a pin input value:\\\\ 0: Low level\\\\ 1: High level\\\\ Bit0 ~ bit7 are corresponding to LP_GPIO0 ~ LP_GPIO7. Bitxx ~ bitxx is invalid.\\\\"]
    #[inline(always)]
    pub fn data_next(&self) -> DATA_NEXT_R {
        DATA_NEXT_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IN")
            .field("data_next", &self.data_next())
            .finish()
    }
}
#[doc = "LP_GPIO input register\n\nYou can [`read`](crate::Reg::read) this register and get [`in_::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IN_SPEC;
impl crate::RegisterSpec for IN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in_::R`](R) reader structure"]
impl crate::Readable for IN_SPEC {}
#[doc = "`reset()` method sets IN to value 0"]
impl crate::Resettable for IN_SPEC {}
