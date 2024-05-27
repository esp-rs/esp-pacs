///Register `RD_BLK2_DATA6` reader
pub type R = crate::R<RD_BLK2_DATA6_SPEC>;
///Field `ADC_CALIBRATION_3` reader - Store the bit \[86:96\] of ADC calibration data.
pub type ADC_CALIBRATION_3_R = crate::FieldReader<u16>;
///Field `BLK2_RESERVED_DATA_0` reader - Store the bit \[0:20\] of block2 reserved data.
pub type BLK2_RESERVED_DATA_0_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:10 - Store the bit \[86:96\] of ADC calibration data.
    #[inline(always)]
    pub fn adc_calibration_3(&self) -> ADC_CALIBRATION_3_R {
        ADC_CALIBRATION_3_R::new((self.bits & 0x07ff) as u16)
    }
    ///Bits 11:31 - Store the bit \[0:20\] of block2 reserved data.
    #[inline(always)]
    pub fn blk2_reserved_data_0(&self) -> BLK2_RESERVED_DATA_0_R {
        BLK2_RESERVED_DATA_0_R::new((self.bits >> 11) & 0x001f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_BLK2_DATA6")
            .field("adc_calibration_3", &self.adc_calibration_3())
            .field("blk2_reserved_data_0", &self.blk2_reserved_data_0())
            .finish()
    }
}
/**Register 6 of BLOCK2.

You can [`read`](crate::generic::Reg::read) this register and get [`rd_blk2_data6::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RD_BLK2_DATA6_SPEC;
impl crate::RegisterSpec for RD_BLK2_DATA6_SPEC {
    type Ux = u32;
}
///`read()` method returns [`rd_blk2_data6::R`](R) reader structure
impl crate::Readable for RD_BLK2_DATA6_SPEC {}
///`reset()` method sets RD_BLK2_DATA6 to value 0
impl crate::Resettable for RD_BLK2_DATA6_SPEC {
    const RESET_VALUE: u32 = 0;
}
