#[doc = "Register `RD_BLK2_DATA4` reader"]
pub struct R(crate::R<RD_BLK2_DATA4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RD_BLK2_DATA4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RD_BLK2_DATA4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RD_BLK2_DATA4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ADC_CALIBRATION_1` reader - Store the bit \\[22:53\\] of ADC calibration data."]
pub type ADC_CALIBRATION_1_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Store the bit \\[22:53\\] of ADC calibration data."]
    #[inline(always)]
    pub fn adc_calibration_1(&self) -> ADC_CALIBRATION_1_R {
        ADC_CALIBRATION_1_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_BLK2_DATA4")
            .field(
                "adc_calibration_1",
                &format_args!("{}", self.adc_calibration_1().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RD_BLK2_DATA4_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Register 4 of BLOCK2.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_blk2_data4](index.html) module"]
pub struct RD_BLK2_DATA4_SPEC;
impl crate::RegisterSpec for RD_BLK2_DATA4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rd_blk2_data4::R](R) reader structure"]
impl crate::Readable for RD_BLK2_DATA4_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RD_BLK2_DATA4 to value 0"]
impl crate::Resettable for RD_BLK2_DATA4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
