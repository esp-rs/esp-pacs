#[doc = "Register `CONTINUE` reader"]
pub type R = crate::R<CONTINUE_SPEC>;
#[doc = "Field `CONTINUE` reader - Reserved."]
pub type CONTINUE_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 1:31 - Reserved."]
    #[inline(always)]
    pub fn continue_(&self) -> CONTINUE_R {
        CONTINUE_R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONTINUE")
            .field("continue_", &self.continue_())
            .finish()
    }
}
#[doc = "Typical SHA configuration register 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`continue_::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONTINUE_SPEC;
impl crate::RegisterSpec for CONTINUE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`continue_::R`](R) reader structure"]
impl crate::Readable for CONTINUE_SPEC {}
#[doc = "`reset()` method sets CONTINUE to value 0"]
impl crate::Resettable for CONTINUE_SPEC {}
