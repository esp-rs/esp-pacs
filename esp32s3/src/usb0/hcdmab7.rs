#[doc = "Register `HCDMAB7` reader"]
pub struct R(crate::R<HCDMAB7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCDMAB7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HCDMAB7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HCDMAB7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `H_HCDMAB7` reader - "]
pub type H_HCDMAB7_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn h_hcdmab7(&self) -> H_HCDMAB7_R {
        H_HCDMAB7_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCDMAB7")
            .field("h_hcdmab7", &format_args!("{}", self.h_hcdmab7().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HCDMAB7_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcdmab7](index.html) module"]
pub struct HCDMAB7_SPEC;
impl crate::RegisterSpec for HCDMAB7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hcdmab7::R](R) reader structure"]
impl crate::Readable for HCDMAB7_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HCDMAB7 to value 0"]
impl crate::Resettable for HCDMAB7_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
