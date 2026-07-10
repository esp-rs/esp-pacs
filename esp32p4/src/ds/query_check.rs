#[doc = "Register `QUERY_CHECK` reader"]
pub type R = crate::R<QUERY_CHECK_SPEC>;
#[doc = "Field `MD_ERROR` reader - Represents whether or not the MD check passes.\\\\ 0: The MD check passes\\\\ 1: The MD check fails\\\\"]
pub type MD_ERROR_R = crate::BitReader;
#[doc = "Field `PADDING_BAD` reader - Represents whether or not the padding check passes.\\\\ 0: The padding check passes\\\\ 1: The padding check fails\\\\"]
pub type PADDING_BAD_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Represents whether or not the MD check passes.\\\\ 0: The MD check passes\\\\ 1: The MD check fails\\\\"]
    #[inline(always)]
    pub fn md_error(&self) -> MD_ERROR_R {
        MD_ERROR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Represents whether or not the padding check passes.\\\\ 0: The padding check passes\\\\ 1: The padding check fails\\\\"]
    #[inline(always)]
    pub fn padding_bad(&self) -> PADDING_BAD_R {
        PADDING_BAD_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("QUERY_CHECK")
            .field("md_error", &self.md_error())
            .field("padding_bad", &self.padding_bad())
            .finish()
    }
}
#[doc = "Queries DS check result\n\nYou can [`read`](crate::Reg::read) this register and get [`query_check::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QUERY_CHECK_SPEC;
impl crate::RegisterSpec for QUERY_CHECK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`query_check::R`](R) reader structure"]
impl crate::Readable for QUERY_CHECK_SPEC {}
#[doc = "`reset()` method sets QUERY_CHECK to value 0"]
impl crate::Resettable for QUERY_CHECK_SPEC {}
