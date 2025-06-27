#[doc = "Register `SAR_TOUCH_OUT5` reader"]
pub type R = crate::R<SAR_TOUCH_OUT5_SPEC>;
#[doc = "Field `TOUCH_MEAS_OUT9` reader - the counter for touch pad 9"]
pub type TOUCH_MEAS_OUT9_R = crate::FieldReader<u16>;
#[doc = "Field `TOUCH_MEAS_OUT8` reader - the counter for touch pad 8"]
pub type TOUCH_MEAS_OUT8_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - the counter for touch pad 9"]
    #[inline(always)]
    pub fn touch_meas_out9(&self) -> TOUCH_MEAS_OUT9_R {
        TOUCH_MEAS_OUT9_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - the counter for touch pad 8"]
    #[inline(always)]
    pub fn touch_meas_out8(&self) -> TOUCH_MEAS_OUT8_R {
        TOUCH_MEAS_OUT8_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_TOUCH_OUT5")
            .field("touch_meas_out9", &self.touch_meas_out9())
            .field("touch_meas_out8", &self.touch_meas_out8())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_touch_out5::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAR_TOUCH_OUT5_SPEC;
impl crate::RegisterSpec for SAR_TOUCH_OUT5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar_touch_out5::R`](R) reader structure"]
impl crate::Readable for SAR_TOUCH_OUT5_SPEC {}
#[doc = "`reset()` method sets SAR_TOUCH_OUT5 to value 0"]
impl crate::Resettable for SAR_TOUCH_OUT5_SPEC {}
