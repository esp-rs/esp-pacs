#[doc = "Register `MULT_INT_RAW` reader"]
pub struct R(crate::R<MULT_INT_RAW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MULT_INT_RAW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MULT_INT_RAW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MULT_INT_RAW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CALC_DONE_INT_RAW` reader - The raw interrupt status bit for the i2s_tx_hung_int interrupt"]
pub type CALC_DONE_INT_RAW_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The raw interrupt status bit for the i2s_tx_hung_int interrupt"]
    #[inline(always)]
    pub fn calc_done_int_raw(&self) -> CALC_DONE_INT_RAW_R {
        CALC_DONE_INT_RAW_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MULT_INT_RAW")
            .field(
                "calc_done_int_raw",
                &format_args!("{}", self.calc_done_int_raw().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<MULT_INT_RAW_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "I2S interrupt raw register, valid in level.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mult_int_raw](index.html) module"]
pub struct MULT_INT_RAW_SPEC;
impl crate::RegisterSpec for MULT_INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mult_int_raw::R](R) reader structure"]
impl crate::Readable for MULT_INT_RAW_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MULT_INT_RAW to value 0"]
impl crate::Resettable for MULT_INT_RAW_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
