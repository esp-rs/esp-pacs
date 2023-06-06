#[doc = "Register `TIME0` reader"]
pub struct R(crate::R<TIME0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIME0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIME0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIME0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TIME_LO` reader - RTC timer low 32 bits"]
pub type TIME_LO_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - RTC timer low 32 bits"]
    #[inline(always)]
    pub fn time_lo(&self) -> TIME_LO_R {
        TIME_LO_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIME0")
            .field("time_lo", &format_args!("{}", self.time_lo().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TIME0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [time0](index.html) module"]
pub struct TIME0_SPEC;
impl crate::RegisterSpec for TIME0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [time0::R](R) reader structure"]
impl crate::Readable for TIME0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TIME0 to value 0"]
impl crate::Resettable for TIME0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
