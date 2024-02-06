#[doc = "Register `QUERY_CHECK` reader"]
pub type R = crate::R<QUERY_CHECK_SPEC>;
#[doc = "Field `MD_ERROR` reader - MD checkout result. 1'b0: MD check pass, 1'b1: MD check fail"]
pub type MD_ERROR_R = crate::BitReader;
#[doc = "Field `PADDING_BAD` reader - padding checkout result. 1'b0: a good padding, 1'b1: a bad padding"]
pub type PADDING_BAD_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - MD checkout result. 1'b0: MD check pass, 1'b1: MD check fail"]
    #[inline(always)]
    pub fn md_error(&self) -> MD_ERROR_R {
        MD_ERROR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - padding checkout result. 1'b0: a good padding, 1'b1: a bad padding"]
    #[inline(always)]
    pub fn padding_bad(&self) -> PADDING_BAD_R {
        PADDING_BAD_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("QUERY_CHECK")
            .field("md_error", &format_args!("{}", self.md_error().bit()))
            .field("padding_bad", &format_args!("{}", self.padding_bad().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<QUERY_CHECK_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "DS query check result register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`query_check::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QUERY_CHECK_SPEC;
impl crate::RegisterSpec for QUERY_CHECK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`query_check::R`](R) reader structure"]
impl crate::Readable for QUERY_CHECK_SPEC {}
#[doc = "`reset()` method sets QUERY_CHECK to value 0"]
impl crate::Resettable for QUERY_CHECK_SPEC {
    const RESET_VALUE: u32 = 0;
}
