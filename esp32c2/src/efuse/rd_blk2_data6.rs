#[doc = "Register `RD_BLK2_DATA6` reader"]
pub struct R(crate::R<RD_BLK2_DATA6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RD_BLK2_DATA6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RD_BLK2_DATA6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RD_BLK2_DATA6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ADC_CALIBRATION_3` reader - Store the bit \\[86:96\\] of ADC calibration data."]
pub type ADC_CALIBRATION_3_R = crate::FieldReader<u16, u16>;
#[doc = "Field `BLK2_RESERVED_DATA_0` reader - Store the bit \\[0:20\\] of block2 reserved data."]
pub type BLK2_RESERVED_DATA_0_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:10 - Store the bit \\[86:96\\] of ADC calibration data."]
    #[inline(always)]
    pub fn adc_calibration_3(&self) -> ADC_CALIBRATION_3_R {
        ADC_CALIBRATION_3_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:31 - Store the bit \\[0:20\\] of block2 reserved data."]
    #[inline(always)]
    pub fn blk2_reserved_data_0(&self) -> BLK2_RESERVED_DATA_0_R {
        BLK2_RESERVED_DATA_0_R::new((self.bits >> 11) & 0x001f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_BLK2_DATA6")
            .field(
                "adc_calibration_3",
                &format_args!("{}", self.adc_calibration_3().bits()),
            )
            .field(
                "blk2_reserved_data_0",
                &format_args!("{}", self.blk2_reserved_data_0().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RD_BLK2_DATA6_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Register 6 of BLOCK2.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_blk2_data6](index.html) module"]
pub struct RD_BLK2_DATA6_SPEC;
impl crate::RegisterSpec for RD_BLK2_DATA6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rd_blk2_data6::R](R) reader structure"]
impl crate::Readable for RD_BLK2_DATA6_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RD_BLK2_DATA6 to value 0"]
impl crate::Resettable for RD_BLK2_DATA6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
