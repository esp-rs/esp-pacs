#[doc = "Register `REAL_TARGET1_LO` reader"]
pub type R = crate::R<REAL_TARGET1_LO_SPEC>;
#[doc = "Field `TARGET1_LO_RO` reader - Represents the actual target value of COMP1, low 32 bits."]
pub type TARGET1_LO_RO_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Represents the actual target value of COMP1, low 32 bits."]
    #[inline(always)]
    pub fn target1_lo_ro(&self) -> TARGET1_LO_RO_R {
        TARGET1_LO_RO_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REAL_TARGET1_LO")
            .field("target1_lo_ro", &self.target1_lo_ro())
            .finish()
    }
}
#[doc = "Actual target value of COMP1, low 32 bits\n\nYou can [`read`](crate::Reg::read) this register and get [`real_target1_lo::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REAL_TARGET1_LO_SPEC;
impl crate::RegisterSpec for REAL_TARGET1_LO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`real_target1_lo::R`](R) reader structure"]
impl crate::Readable for REAL_TARGET1_LO_SPEC {}
#[doc = "`reset()` method sets REAL_TARGET1_LO to value 0"]
impl crate::Resettable for REAL_TARGET1_LO_SPEC {}
