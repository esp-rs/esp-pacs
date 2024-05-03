#[doc = "Register `LO` reader"]
pub type R = crate::R<LO_SPEC>;
#[doc = "Field `VALUE_LO` reader - timer read value low 32bit"]
pub type VALUE_LO_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - timer read value low 32bit"]
    #[inline(always)]
    pub fn value_lo(&self) -> VALUE_LO_R {
        VALUE_LO_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LO")
            .field("value_lo", &self.value_lo().bits())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LO_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "SYSTIMER_UNIT0_VALUE_LO.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lo::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LO_SPEC;
impl crate::RegisterSpec for LO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lo::R`](R) reader structure"]
impl crate::Readable for LO_SPEC {}
#[doc = "`reset()` method sets LO to value 0"]
impl crate::Resettable for LO_SPEC {
    const RESET_VALUE: u32 = 0;
}
