#[doc = "Register `RD_BLK2_DATA4` reader"]
pub type R = crate::R<RD_BLK2_DATA4_SPEC>;
#[doc = "Field `ADC_CALIBRATION_1` reader - Store the bit \\[22:53\\] of ADC calibration data."]
pub type ADC_CALIBRATION_1_R = crate::FieldReader<u32>;
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
            .field("adc_calibration_1", &self.adc_calibration_1())
            .finish()
    }
}
#[doc = "Register 4 of BLOCK2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd_blk2_data4::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RD_BLK2_DATA4_SPEC;
impl crate::RegisterSpec for RD_BLK2_DATA4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_blk2_data4::R`](R) reader structure"]
impl crate::Readable for RD_BLK2_DATA4_SPEC {}
#[doc = "`reset()` method sets RD_BLK2_DATA4 to value 0"]
impl crate::Resettable for RD_BLK2_DATA4_SPEC {
    const RESET_VALUE: u32 = 0;
}
