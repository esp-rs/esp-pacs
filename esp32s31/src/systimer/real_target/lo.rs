#[doc = "Register `LO` reader"]
pub type R = crate::R<LO_SPEC>;
#[doc = "Field `LO_RO` reader - Represents the actual target value of COMP0, low 32 bits."]
pub type LO_RO_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Represents the actual target value of COMP0, low 32 bits."]
    #[inline(always)]
    pub fn lo_ro(&self) -> LO_RO_R {
        LO_RO_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LO").field("lo_ro", &self.lo_ro()).finish()
    }
}
#[doc = "Actual target value of COMP0, low 32 bits\n\nYou can [`read`](crate::Reg::read) this register and get [`lo::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LO_SPEC;
impl crate::RegisterSpec for LO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lo::R`](R) reader structure"]
impl crate::Readable for LO_SPEC {}
#[doc = "`reset()` method sets LO to value 0"]
impl crate::Resettable for LO_SPEC {}
