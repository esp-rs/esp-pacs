#[doc = "Register `HCDMAB2` reader"]
pub struct R(crate::R<HCDMAB2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCDMAB2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HCDMAB2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HCDMAB2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `H_HCDMAB2` reader - "]
pub type H_HCDMAB2_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn h_hcdmab2(&self) -> H_HCDMAB2_R {
        H_HCDMAB2_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCDMAB2")
            .field("h_hcdmab2", &format_args!("{}", self.h_hcdmab2().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HCDMAB2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcdmab2](index.html) module"]
pub struct HCDMAB2_SPEC;
impl crate::RegisterSpec for HCDMAB2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hcdmab2::R](R) reader structure"]
impl crate::Readable for HCDMAB2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HCDMAB2 to value 0"]
impl crate::Resettable for HCDMAB2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
