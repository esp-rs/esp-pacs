#[doc = "Register `QUERY_ERROR` reader"]
pub type R = crate::R<QUERY_ERROR_SPEC>;
#[doc = "Field `QUREY_CHECK` reader - Hmac configuration state. 0: key are agree with purpose. 1: error"]
pub type QUREY_CHECK_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Hmac configuration state. 0: key are agree with purpose. 1: error"]
    #[inline(always)]
    pub fn qurey_check(&self) -> QUREY_CHECK_R {
        QUREY_CHECK_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("QUERY_ERROR")
            .field("qurey_check", &format_args!("{}", self.qurey_check().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<QUERY_ERROR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Error register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`query_error::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QUERY_ERROR_SPEC;
impl crate::RegisterSpec for QUERY_ERROR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`query_error::R`](R) reader structure"]
impl crate::Readable for QUERY_ERROR_SPEC {}
#[doc = "`reset()` method sets QUERY_ERROR to value 0"]
impl crate::Resettable for QUERY_ERROR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
