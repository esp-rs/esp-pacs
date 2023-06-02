#[doc = "Register `MULT_INT_ST` reader"]
pub struct R(crate::R<MULT_INT_ST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MULT_INT_ST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MULT_INT_ST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MULT_INT_ST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CALC_DONE_INT_ST` reader - The masked interrupt status bit for the ecc_calc_done_int interrupt"]
pub type CALC_DONE_INT_ST_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The masked interrupt status bit for the ecc_calc_done_int interrupt"]
    #[inline(always)]
    pub fn calc_done_int_st(&self) -> CALC_DONE_INT_ST_R {
        CALC_DONE_INT_ST_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MULT_INT_ST")
            .field(
                "calc_done_int_st",
                &format_args!("{}", self.calc_done_int_st().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<MULT_INT_ST_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "ECC interrupt status register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mult_int_st](index.html) module"]
pub struct MULT_INT_ST_SPEC;
impl crate::RegisterSpec for MULT_INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mult_int_st::R](R) reader structure"]
impl crate::Readable for MULT_INT_ST_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MULT_INT_ST to value 0"]
impl crate::Resettable for MULT_INT_ST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
