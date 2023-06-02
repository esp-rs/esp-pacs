#[doc = "Register `M3_EXCEPTION_INFO1` reader"]
pub struct R(crate::R<M3_EXCEPTION_INFO1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<M3_EXCEPTION_INFO1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<M3_EXCEPTION_INFO1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<M3_EXCEPTION_INFO1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `M3_EXCEPTION_ADDR` reader - Exception addr"]
pub type M3_EXCEPTION_ADDR_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Exception addr"]
    #[inline(always)]
    pub fn m3_exception_addr(&self) -> M3_EXCEPTION_ADDR_R {
        M3_EXCEPTION_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("M3_EXCEPTION_INFO1")
            .field(
                "m3_exception_addr",
                &format_args!("{}", self.m3_exception_addr().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<M3_EXCEPTION_INFO1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "M3 exception_info1 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m3_exception_info1](index.html) module"]
pub struct M3_EXCEPTION_INFO1_SPEC;
impl crate::RegisterSpec for M3_EXCEPTION_INFO1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [m3_exception_info1::R](R) reader structure"]
impl crate::Readable for M3_EXCEPTION_INFO1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets M3_EXCEPTION_INFO1 to value 0"]
impl crate::Resettable for M3_EXCEPTION_INFO1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
