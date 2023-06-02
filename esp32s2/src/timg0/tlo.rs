#[doc = "Register `T%sLO` reader"]
pub struct R(crate::R<TLO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TLO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TLO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TLO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LO` reader - After writing to TIMG_T%sUPDATE_REG, the low 32 bits of the time-base counter of timer %s can be read here."]
pub type LO_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - After writing to TIMG_T%sUPDATE_REG, the low 32 bits of the time-base counter of timer %s can be read here."]
    #[inline(always)]
    pub fn lo(&self) -> LO_R {
        LO_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TLO")
            .field("lo", &format_args!("{}", self.lo().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TLO_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Timer %s current value, low 32 bits\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tlo](index.html) module"]
pub struct TLO_SPEC;
impl crate::RegisterSpec for TLO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tlo::R](R) reader structure"]
impl crate::Readable for TLO_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets T%sLO to value 0"]
impl crate::Resettable for TLO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
