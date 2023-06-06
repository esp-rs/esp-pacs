#[doc = "Register `START` reader"]
pub struct R(crate::R<START_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<START_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<START_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<START_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `START` reader - reserved."]
pub type START_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 1:31 - reserved."]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("START")
            .field("start", &format_args!("{}", self.start().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<START_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Typical SHA configuration register 0.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [start](index.html) module"]
pub struct START_SPEC;
impl crate::RegisterSpec for START_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [start::R](R) reader structure"]
impl crate::Readable for START_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets START to value 0"]
impl crate::Resettable for START_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
