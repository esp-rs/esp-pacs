#[doc = "Register `H_%s` reader"]
pub struct R(crate::R<H__SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<H__SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<H__SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<H__SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `H_0` reader - GCM hash subkey"]
pub type H_0_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - GCM hash subkey"]
    #[inline(always)]
    pub fn h_0(&self) -> H_0_R {
        H_0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("H_")
            .field("h_0", &format_args!("{}", self.h_0().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<H__SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "GCM hash subkey\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [h_](index.html) module"]
pub struct H__SPEC;
impl crate::RegisterSpec for H__SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [h_::R](R) reader structure"]
impl crate::Readable for H__SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets H_%s to value 0"]
impl crate::Resettable for H__SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
