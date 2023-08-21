#[doc = "Register `FRC1_COUNT` reader"]
pub type R = crate::R<FRC1_COUNT_SPEC>;
#[doc = "Field `frc1_count` reader - the current value of the counter. It is a decreasingcounter."]
pub type FRC1_COUNT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:22 - the current value of the counter. It is a decreasingcounter."]
    #[inline(always)]
    pub fn frc1_count(&self) -> FRC1_COUNT_R {
        FRC1_COUNT_R::new(self.bits & 0x007f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FRC1_COUNT")
            .field("frc1_count", &format_args!("{}", self.frc1_count().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<FRC1_COUNT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "the current value of the counter. It is a decreasingcounter.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`frc1_count::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FRC1_COUNT_SPEC;
impl crate::RegisterSpec for FRC1_COUNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`frc1_count::R`](R) reader structure"]
impl crate::Readable for FRC1_COUNT_SPEC {}
#[doc = "`reset()` method sets FRC1_COUNT to value 0"]
impl crate::Resettable for FRC1_COUNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
