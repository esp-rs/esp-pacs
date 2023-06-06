#[doc = "Register `HCDMAB0` reader"]
pub struct R(crate::R<HCDMAB0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCDMAB0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HCDMAB0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HCDMAB0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `H_HCDMAB0` reader - "]
pub type H_HCDMAB0_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn h_hcdmab0(&self) -> H_HCDMAB0_R {
        H_HCDMAB0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCDMAB0")
            .field("h_hcdmab0", &format_args!("{}", self.h_hcdmab0().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HCDMAB0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcdmab0](index.html) module"]
pub struct HCDMAB0_SPEC;
impl crate::RegisterSpec for HCDMAB0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hcdmab0::R](R) reader structure"]
impl crate::Readable for HCDMAB0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HCDMAB0 to value 0"]
impl crate::Resettable for HCDMAB0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
