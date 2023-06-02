#[doc = "Register `LC_STATE0` reader"]
pub struct R(crate::R<LC_STATE0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LC_STATE0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LC_STATE0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LC_STATE0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LC_STATE0` reader - "]
pub type LC_STATE0_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn lc_state0(&self) -> LC_STATE0_R {
        LC_STATE0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LC_STATE0")
            .field("lc_state0", &format_args!("{}", self.lc_state0().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LC_STATE0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lc_state0](index.html) module"]
pub struct LC_STATE0_SPEC;
impl crate::RegisterSpec for LC_STATE0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lc_state0::R](R) reader structure"]
impl crate::Readable for LC_STATE0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LC_STATE0 to value 0"]
impl crate::Resettable for LC_STATE0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
