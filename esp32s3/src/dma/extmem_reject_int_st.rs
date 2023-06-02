#[doc = "Register `EXTMEM_REJECT_INT_ST` reader"]
pub struct R(crate::R<EXTMEM_REJECT_INT_ST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTMEM_REJECT_INT_ST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTMEM_REJECT_INT_ST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTMEM_REJECT_INT_ST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `EXTMEM_REJECT_INT_ST` reader - The raw interrupt status bit for the EXTMEM_REJECT_INT interrupt."]
pub type EXTMEM_REJECT_INT_ST_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The raw interrupt status bit for the EXTMEM_REJECT_INT interrupt."]
    #[inline(always)]
    pub fn extmem_reject_int_st(&self) -> EXTMEM_REJECT_INT_ST_R {
        EXTMEM_REJECT_INT_ST_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXTMEM_REJECT_INT_ST")
            .field(
                "extmem_reject_int_st",
                &format_args!("{}", self.extmem_reject_int_st().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<EXTMEM_REJECT_INT_ST_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Masked interrupt status of external RAM permission\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extmem_reject_int_st](index.html) module"]
pub struct EXTMEM_REJECT_INT_ST_SPEC;
impl crate::RegisterSpec for EXTMEM_REJECT_INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [extmem_reject_int_st::R](R) reader structure"]
impl crate::Readable for EXTMEM_REJECT_INT_ST_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets EXTMEM_REJECT_INT_ST to value 0"]
impl crate::Resettable for EXTMEM_REJECT_INT_ST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
