#[doc = "Register `_0_LENGTH` reader"]
pub struct R(crate::R<_0_LENGTH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<_0_LENGTH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<_0_LENGTH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<_0_LENGTH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SLC0_LEN` reader - "]
pub type SLC0_LEN_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn slc0_len(&self) -> SLC0_LEN_R {
        SLC0_LEN_R::new(self.bits & 0x000f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("_0_LENGTH")
            .field("slc0_len", &format_args!("{}", self.slc0_len().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<_0_LENGTH_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [_0_length](index.html) module"]
pub struct _0_LENGTH_SPEC;
impl crate::RegisterSpec for _0_LENGTH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [_0_length::R](R) reader structure"]
impl crate::Readable for _0_LENGTH_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets _0_LENGTH to value 0"]
impl crate::Resettable for _0_LENGTH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
