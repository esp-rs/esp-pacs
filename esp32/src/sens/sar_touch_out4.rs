///Register `SAR_TOUCH_OUT4` reader
pub type R = crate::R<SAR_TOUCH_OUT4_SPEC>;
///Field `TOUCH_MEAS_OUT7` reader - the counter for touch pad 7
pub type TOUCH_MEAS_OUT7_R = crate::FieldReader<u16>;
///Field `TOUCH_MEAS_OUT6` reader - the counter for touch pad 6
pub type TOUCH_MEAS_OUT6_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - the counter for touch pad 7
    #[inline(always)]
    pub fn touch_meas_out7(&self) -> TOUCH_MEAS_OUT7_R {
        TOUCH_MEAS_OUT7_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - the counter for touch pad 6
    #[inline(always)]
    pub fn touch_meas_out6(&self) -> TOUCH_MEAS_OUT6_R {
        TOUCH_MEAS_OUT6_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_TOUCH_OUT4")
            .field("touch_meas_out7", &self.touch_meas_out7())
            .field("touch_meas_out6", &self.touch_meas_out6())
            .finish()
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`sar_touch_out4::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SAR_TOUCH_OUT4_SPEC;
impl crate::RegisterSpec for SAR_TOUCH_OUT4_SPEC {
    type Ux = u32;
}
///`read()` method returns [`sar_touch_out4::R`](R) reader structure
impl crate::Readable for SAR_TOUCH_OUT4_SPEC {}
///`reset()` method sets SAR_TOUCH_OUT4 to value 0
impl crate::Resettable for SAR_TOUCH_OUT4_SPEC {
    const RESET_VALUE: u32 = 0;
}
