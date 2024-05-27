///Register `SAR_TOUCH_STATUS0` reader
pub type R = crate::R<SAR_TOUCH_STATUS0_SPEC>;
///Field `TOUCH_DENOISE_DATA` reader - Denoise measure value from touch sensor 0.
pub type TOUCH_DENOISE_DATA_R = crate::FieldReader<u32>;
///Field `TOUCH_SCAN_CURR` reader - Current pad in scan status
pub type TOUCH_SCAN_CURR_R = crate::FieldReader;
impl R {
    ///Bits 0:21 - Denoise measure value from touch sensor 0.
    #[inline(always)]
    pub fn touch_denoise_data(&self) -> TOUCH_DENOISE_DATA_R {
        TOUCH_DENOISE_DATA_R::new(self.bits & 0x003f_ffff)
    }
    ///Bits 22:25 - Current pad in scan status
    #[inline(always)]
    pub fn touch_scan_curr(&self) -> TOUCH_SCAN_CURR_R {
        TOUCH_SCAN_CURR_R::new(((self.bits >> 22) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_TOUCH_STATUS0")
            .field("touch_denoise_data", &self.touch_denoise_data())
            .field("touch_scan_curr", &self.touch_scan_curr())
            .finish()
    }
}
/**Status of touch controller

You can [`read`](crate::generic::Reg::read) this register and get [`sar_touch_status0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SAR_TOUCH_STATUS0_SPEC;
impl crate::RegisterSpec for SAR_TOUCH_STATUS0_SPEC {
    type Ux = u32;
}
///`read()` method returns [`sar_touch_status0::R`](R) reader structure
impl crate::Readable for SAR_TOUCH_STATUS0_SPEC {}
///`reset()` method sets SAR_TOUCH_STATUS0 to value 0
impl crate::Resettable for SAR_TOUCH_STATUS0_SPEC {
    const RESET_VALUE: u32 = 0;
}
