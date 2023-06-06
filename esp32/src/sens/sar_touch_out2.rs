#[doc = "Register `SAR_TOUCH_OUT2` reader"]
pub struct R(crate::R<SAR_TOUCH_OUT2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAR_TOUCH_OUT2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAR_TOUCH_OUT2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAR_TOUCH_OUT2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TOUCH_MEAS_OUT3` reader - the counter for touch pad 3"]
pub type TOUCH_MEAS_OUT3_R = crate::FieldReader<u16>;
#[doc = "Field `TOUCH_MEAS_OUT2` reader - the counter for touch pad 2"]
pub type TOUCH_MEAS_OUT2_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - the counter for touch pad 3"]
    #[inline(always)]
    pub fn touch_meas_out3(&self) -> TOUCH_MEAS_OUT3_R {
        TOUCH_MEAS_OUT3_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - the counter for touch pad 2"]
    #[inline(always)]
    pub fn touch_meas_out2(&self) -> TOUCH_MEAS_OUT2_R {
        TOUCH_MEAS_OUT2_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_TOUCH_OUT2")
            .field(
                "touch_meas_out3",
                &format_args!("{}", self.touch_meas_out3().bits()),
            )
            .field(
                "touch_meas_out2",
                &format_args!("{}", self.touch_meas_out2().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SAR_TOUCH_OUT2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar_touch_out2](index.html) module"]
pub struct SAR_TOUCH_OUT2_SPEC;
impl crate::RegisterSpec for SAR_TOUCH_OUT2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sar_touch_out2::R](R) reader structure"]
impl crate::Readable for SAR_TOUCH_OUT2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SAR_TOUCH_OUT2 to value 0"]
impl crate::Resettable for SAR_TOUCH_OUT2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
