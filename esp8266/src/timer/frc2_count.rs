#[doc = "Register `FRC2_COUNT` reader"]
pub type R = crate::R<FRC2_COUNT_SPEC>;
#[doc = "Field `frc2_count` reader - the current value of the counter. It is a increasingcounter."]
pub type FRC2_COUNT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - the current value of the counter. It is a increasingcounter."]
    #[inline(always)]
    pub fn frc2_count(&self) -> FRC2_COUNT_R {
        FRC2_COUNT_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FRC2_COUNT")
            .field("frc2_count", &format_args!("{}", self.frc2_count().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<FRC2_COUNT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "the current value of the counter. It is a increasingcounter.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`frc2_count::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FRC2_COUNT_SPEC;
impl crate::RegisterSpec for FRC2_COUNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`frc2_count::R`](R) reader structure"]
impl crate::Readable for FRC2_COUNT_SPEC {}
#[doc = "`reset()` method sets FRC2_COUNT to value 0"]
impl crate::Resettable for FRC2_COUNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
