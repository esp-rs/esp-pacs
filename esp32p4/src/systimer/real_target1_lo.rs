#[doc = "Register `REAL_TARGET1_LO` reader"]
pub type R = crate::R<REAL_TARGET1_LO_SPEC>;
#[doc = "Field `TARGET1_LO_RO` reader - actual target value value low 32bits"]
pub type TARGET1_LO_RO_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - actual target value value low 32bits"]
    #[inline(always)]
    pub fn target1_lo_ro(&self) -> TARGET1_LO_RO_R {
        TARGET1_LO_RO_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REAL_TARGET1_LO")
            .field(
                "target1_lo_ro",
                &format_args!("{}", self.target1_lo_ro().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<REAL_TARGET1_LO_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "system timer comp1 actual target value low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`real_target1_lo::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REAL_TARGET1_LO_SPEC;
impl crate::RegisterSpec for REAL_TARGET1_LO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`real_target1_lo::R`](R) reader structure"]
impl crate::Readable for REAL_TARGET1_LO_SPEC {}
#[doc = "`reset()` method sets REAL_TARGET1_LO to value 0"]
impl crate::Resettable for REAL_TARGET1_LO_SPEC {
    const RESET_VALUE: u32 = 0;
}
