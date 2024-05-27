///Register `SAR_TOUCH_OUT2` reader
pub type R = crate::R<SAR_TOUCH_OUT2_SPEC>;
///Field `TOUCH_MEAS_OUT3` reader - the counter for touch pad 3
pub type TOUCH_MEAS_OUT3_R = crate::FieldReader<u16>;
///Field `TOUCH_MEAS_OUT2` reader - the counter for touch pad 2
pub type TOUCH_MEAS_OUT2_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - the counter for touch pad 3
    #[inline(always)]
    pub fn touch_meas_out3(&self) -> TOUCH_MEAS_OUT3_R {
        TOUCH_MEAS_OUT3_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - the counter for touch pad 2
    #[inline(always)]
    pub fn touch_meas_out2(&self) -> TOUCH_MEAS_OUT2_R {
        TOUCH_MEAS_OUT2_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_TOUCH_OUT2")
            .field("touch_meas_out3", &self.touch_meas_out3())
            .field("touch_meas_out2", &self.touch_meas_out2())
            .finish()
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`sar_touch_out2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SAR_TOUCH_OUT2_SPEC;
impl crate::RegisterSpec for SAR_TOUCH_OUT2_SPEC {
    type Ux = u32;
}
///`read()` method returns [`sar_touch_out2::R`](R) reader structure
impl crate::Readable for SAR_TOUCH_OUT2_SPEC {}
///`reset()` method sets SAR_TOUCH_OUT2 to value 0
impl crate::Resettable for SAR_TOUCH_OUT2_SPEC {
    const RESET_VALUE: u32 = 0;
}
