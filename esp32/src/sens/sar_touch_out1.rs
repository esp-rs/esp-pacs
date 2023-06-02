#[doc = "Register `SAR_TOUCH_OUT1` reader"]
pub struct R(crate::R<SAR_TOUCH_OUT1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAR_TOUCH_OUT1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAR_TOUCH_OUT1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAR_TOUCH_OUT1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TOUCH_MEAS_OUT1` reader - the counter for touch pad 1"]
pub type TOUCH_MEAS_OUT1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TOUCH_MEAS_OUT0` reader - the counter for touch pad 0"]
pub type TOUCH_MEAS_OUT0_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - the counter for touch pad 1"]
    #[inline(always)]
    pub fn touch_meas_out1(&self) -> TOUCH_MEAS_OUT1_R {
        TOUCH_MEAS_OUT1_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - the counter for touch pad 0"]
    #[inline(always)]
    pub fn touch_meas_out0(&self) -> TOUCH_MEAS_OUT0_R {
        TOUCH_MEAS_OUT0_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_TOUCH_OUT1")
            .field(
                "touch_meas_out1",
                &format_args!("{}", self.touch_meas_out1().bits()),
            )
            .field(
                "touch_meas_out0",
                &format_args!("{}", self.touch_meas_out0().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SAR_TOUCH_OUT1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar_touch_out1](index.html) module"]
pub struct SAR_TOUCH_OUT1_SPEC;
impl crate::RegisterSpec for SAR_TOUCH_OUT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sar_touch_out1::R](R) reader structure"]
impl crate::Readable for SAR_TOUCH_OUT1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SAR_TOUCH_OUT1 to value 0"]
impl crate::Resettable for SAR_TOUCH_OUT1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
