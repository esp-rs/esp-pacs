#[doc = "Register `SAR_TOUCH_STATUS0` reader"]
pub struct R(crate::R<SAR_TOUCH_STATUS0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAR_TOUCH_STATUS0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAR_TOUCH_STATUS0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAR_TOUCH_STATUS0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TOUCH_DENOISE_DATA` reader - Denoise measure value from touch sensor 0."]
pub type TOUCH_DENOISE_DATA_R = crate::FieldReader<u32>;
#[doc = "Field `TOUCH_SCAN_CURR` reader - Current pad in scan status"]
pub type TOUCH_SCAN_CURR_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:21 - Denoise measure value from touch sensor 0."]
    #[inline(always)]
    pub fn touch_denoise_data(&self) -> TOUCH_DENOISE_DATA_R {
        TOUCH_DENOISE_DATA_R::new(self.bits & 0x003f_ffff)
    }
    #[doc = "Bits 22:25 - Current pad in scan status"]
    #[inline(always)]
    pub fn touch_scan_curr(&self) -> TOUCH_SCAN_CURR_R {
        TOUCH_SCAN_CURR_R::new(((self.bits >> 22) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_TOUCH_STATUS0")
            .field(
                "touch_denoise_data",
                &format_args!("{}", self.touch_denoise_data().bits()),
            )
            .field(
                "touch_scan_curr",
                &format_args!("{}", self.touch_scan_curr().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SAR_TOUCH_STATUS0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Status of touch controller\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar_touch_status0](index.html) module"]
pub struct SAR_TOUCH_STATUS0_SPEC;
impl crate::RegisterSpec for SAR_TOUCH_STATUS0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sar_touch_status0::R](R) reader structure"]
impl crate::Readable for SAR_TOUCH_STATUS0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SAR_TOUCH_STATUS0 to value 0"]
impl crate::Resettable for SAR_TOUCH_STATUS0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
