#[doc = "Register `RD_BLK2_DATA3` reader"]
pub struct R(crate::R<RD_BLK2_DATA3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RD_BLK2_DATA3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RD_BLK2_DATA3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RD_BLK2_DATA3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PVT_HIGH` reader - Store the bit \\[5:14\\] of pvt."]
pub type PVT_HIGH_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ADC_CALIBRATION_0` reader - Store the bit \\[0:21\\] of ADC calibration data."]
pub type ADC_CALIBRATION_0_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:9 - Store the bit \\[5:14\\] of pvt."]
    #[inline(always)]
    pub fn pvt_high(&self) -> PVT_HIGH_R {
        PVT_HIGH_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:31 - Store the bit \\[0:21\\] of ADC calibration data."]
    #[inline(always)]
    pub fn adc_calibration_0(&self) -> ADC_CALIBRATION_0_R {
        ADC_CALIBRATION_0_R::new((self.bits >> 10) & 0x003f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_BLK2_DATA3")
            .field("pvt_high", &format_args!("{}", self.pvt_high().bits()))
            .field(
                "adc_calibration_0",
                &format_args!("{}", self.adc_calibration_0().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RD_BLK2_DATA3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Register 3 of BLOCK2.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_blk2_data3](index.html) module"]
pub struct RD_BLK2_DATA3_SPEC;
impl crate::RegisterSpec for RD_BLK2_DATA3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rd_blk2_data3::R](R) reader structure"]
impl crate::Readable for RD_BLK2_DATA3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RD_BLK2_DATA3 to value 0"]
impl crate::Resettable for RD_BLK2_DATA3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
