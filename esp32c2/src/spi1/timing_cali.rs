#[doc = "Register `TIMING_CALI` reader"]
pub struct R(crate::R<TIMING_CALI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMING_CALI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMING_CALI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMING_CALI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TIMING_CALI` reader - The bit is used to enable timing auto-calibration for all reading operations."]
pub type TIMING_CALI_R = crate::BitReader;
#[doc = "Field `EXTRA_DUMMY_CYCLELEN` reader - add extra dummy spi clock cycle length for spi clock calibration."]
pub type EXTRA_DUMMY_CYCLELEN_R = crate::FieldReader;
impl R {
    #[doc = "Bit 1 - The bit is used to enable timing auto-calibration for all reading operations."]
    #[inline(always)]
    pub fn timing_cali(&self) -> TIMING_CALI_R {
        TIMING_CALI_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:4 - add extra dummy spi clock cycle length for spi clock calibration."]
    #[inline(always)]
    pub fn extra_dummy_cyclelen(&self) -> EXTRA_DUMMY_CYCLELEN_R {
        EXTRA_DUMMY_CYCLELEN_R::new(((self.bits >> 2) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMING_CALI")
            .field("timing_cali", &format_args!("{}", self.timing_cali().bit()))
            .field(
                "extra_dummy_cyclelen",
                &format_args!("{}", self.extra_dummy_cyclelen().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TIMING_CALI_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "SPI1 timing control register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timing_cali](index.html) module"]
pub struct TIMING_CALI_SPEC;
impl crate::RegisterSpec for TIMING_CALI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timing_cali::R](R) reader structure"]
impl crate::Readable for TIMING_CALI_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TIMING_CALI to value 0"]
impl crate::Resettable for TIMING_CALI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
