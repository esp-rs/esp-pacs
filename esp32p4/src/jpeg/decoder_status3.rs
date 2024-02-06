#[doc = "Register `DECODER_STATUS3` reader"]
pub type R = crate::R<DECODER_STATUS3_SPEC>;
#[doc = "Field `LOOKUP_DATA` reader - Reserved"]
pub type LOOKUP_DATA_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Reserved"]
    #[inline(always)]
    pub fn lookup_data(&self) -> LOOKUP_DATA_R {
        LOOKUP_DATA_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DECODER_STATUS3")
            .field(
                "lookup_data",
                &format_args!("{}", self.lookup_data().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DECODER_STATUS3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Trace and Debug registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`decoder_status3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DECODER_STATUS3_SPEC;
impl crate::RegisterSpec for DECODER_STATUS3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`decoder_status3::R`](R) reader structure"]
impl crate::Readable for DECODER_STATUS3_SPEC {}
#[doc = "`reset()` method sets DECODER_STATUS3 to value 0"]
impl crate::Resettable for DECODER_STATUS3_SPEC {
    const RESET_VALUE: u32 = 0;
}
