#[doc = "Register `IN` reader"]
pub type R = crate::R<IN_SPEC>;
#[doc = "Field `DATA_NEXT` reader - Represents the input value of GPIO0 ~ GPIO7.\\\\ 0: Low level input\\\\ 1: High level input\\\\ bit0 ~ bit7 are corresponding to GPIO0 ~ GPIO7.\\\\"]
pub type DATA_NEXT_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Represents the input value of GPIO0 ~ GPIO7.\\\\ 0: Low level input\\\\ 1: High level input\\\\ bit0 ~ bit7 are corresponding to GPIO0 ~ GPIO7.\\\\"]
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
#[doc = "LP GPIO input register\n\nYou can [`read`](crate::Reg::read) this register and get [`in_::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IN_SPEC;
impl crate::RegisterSpec for IN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in_::R`](R) reader structure"]
impl crate::Readable for IN_SPEC {}
#[doc = "`reset()` method sets IN to value 0"]
impl crate::Resettable for IN_SPEC {
    const RESET_VALUE: u32 = 0;
}
