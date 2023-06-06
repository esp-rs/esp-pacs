#[doc = "Register `T0HI` reader"]
pub struct R(crate::R<T0HI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<T0HI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<T0HI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<T0HI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `HI` reader - Register to store timer 0 time-base counter current value higher 32 bits."]
pub type HI_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Register to store timer 0 time-base counter current value higher 32 bits."]
    #[inline(always)]
    pub fn hi(&self) -> HI_R {
        HI_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("T0HI")
            .field("hi", &format_args!("{}", self.hi().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<T0HI_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t0hi](index.html) module"]
pub struct T0HI_SPEC;
impl crate::RegisterSpec for T0HI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [t0hi::R](R) reader structure"]
impl crate::Readable for T0HI_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets T0HI to value 0"]
impl crate::Resettable for T0HI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
