///Register `RD_BLK2_DATA5` reader
pub type R = crate::R<RD_BLK2_DATA5_SPEC>;
///Field `ADC_CALIBRATION_2` reader - Store the bit \[54:85\] of ADC calibration data.
pub type ADC_CALIBRATION_2_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Store the bit \[54:85\] of ADC calibration data.
    #[inline(always)]
    pub fn adc_calibration_2(&self) -> ADC_CALIBRATION_2_R {
        ADC_CALIBRATION_2_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_BLK2_DATA5")
            .field("adc_calibration_2", &self.adc_calibration_2())
            .finish()
    }
}
/**Register 5 of BLOCK2.

You can [`read`](crate::generic::Reg::read) this register and get [`rd_blk2_data5::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RD_BLK2_DATA5_SPEC;
impl crate::RegisterSpec for RD_BLK2_DATA5_SPEC {
    type Ux = u32;
}
///`read()` method returns [`rd_blk2_data5::R`](R) reader structure
impl crate::Readable for RD_BLK2_DATA5_SPEC {}
///`reset()` method sets RD_BLK2_DATA5 to value 0
impl crate::Resettable for RD_BLK2_DATA5_SPEC {
    const RESET_VALUE: u32 = 0;
}
