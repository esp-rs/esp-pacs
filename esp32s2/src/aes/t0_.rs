#[doc = "Register `T0_%s` reader"]
pub struct R(crate::R<T0__SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<T0__SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<T0__SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<T0__SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `T0_0` reader - This register stores the %sth 32-bit piece of 128-bit T0"]
pub type T0_0_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This register stores the %sth 32-bit piece of 128-bit T0"]
    #[inline(always)]
    pub fn t0_0(&self) -> T0_0_R {
        T0_0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("T0_")
            .field("t0_0", &format_args!("{}", self.t0_0().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<T0__SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "T0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t0_](index.html) module"]
pub struct T0__SPEC;
impl crate::RegisterSpec for T0__SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [t0_::R](R) reader structure"]
impl crate::Readable for T0__SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets T0_%s to value 0"]
impl crate::Resettable for T0__SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
