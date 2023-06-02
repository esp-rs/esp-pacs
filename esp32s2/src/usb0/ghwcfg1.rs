#[doc = "Register `GHWCFG1` reader"]
pub struct R(crate::R<GHWCFG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GHWCFG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GHWCFG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GHWCFG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `EPDIR` reader - "]
pub type EPDIR_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn epdir(&self) -> EPDIR_R {
        EPDIR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GHWCFG1")
            .field("epdir", &format_args!("{}", self.epdir().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<GHWCFG1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ghwcfg1](index.html) module"]
pub struct GHWCFG1_SPEC;
impl crate::RegisterSpec for GHWCFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ghwcfg1::R](R) reader structure"]
impl crate::Readable for GHWCFG1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GHWCFG1 to value 0"]
impl crate::Resettable for GHWCFG1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
