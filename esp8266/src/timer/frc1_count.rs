#[doc = "Register `FRC1_COUNT` reader"]
pub struct R(crate::R<FRC1_COUNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FRC1_COUNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FRC1_COUNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FRC1_COUNT_SPEC>) -> Self {
        R(reader)
    }
}
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
#[doc = "the current value of the counter. It is a decreasingcounter.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [frc1_count](index.html) module"]
pub struct FRC1_COUNT_SPEC;
impl crate::RegisterSpec for FRC1_COUNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [frc1_count::R](R) reader structure"]
impl crate::Readable for FRC1_COUNT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FRC1_COUNT to value 0"]
impl crate::Resettable for FRC1_COUNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
