#[doc = "Register `HCDMAB1` reader"]
pub struct R(crate::R<HCDMAB1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCDMAB1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HCDMAB1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HCDMAB1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `H_HCDMAB1` reader - "]
pub type H_HCDMAB1_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn h_hcdmab1(&self) -> H_HCDMAB1_R {
        H_HCDMAB1_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCDMAB1")
            .field("h_hcdmab1", &format_args!("{}", self.h_hcdmab1().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HCDMAB1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcdmab1](index.html) module"]
pub struct HCDMAB1_SPEC;
impl crate::RegisterSpec for HCDMAB1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hcdmab1::R](R) reader structure"]
impl crate::Readable for HCDMAB1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HCDMAB1 to value 0"]
impl crate::Resettable for HCDMAB1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
