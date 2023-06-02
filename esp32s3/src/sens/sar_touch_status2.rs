#[doc = "Register `SAR_TOUCH_STATUS2` reader"]
pub struct R(crate::R<SAR_TOUCH_STATUS2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAR_TOUCH_STATUS2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAR_TOUCH_STATUS2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAR_TOUCH_STATUS2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SAR_TOUCH_PAD2_DATA` reader - touch data debounce of touch pad 2"]
pub type SAR_TOUCH_PAD2_DATA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SAR_TOUCH_PAD2_DEBOUNCE` reader - touch current debounce of touch pad 2"]
pub type SAR_TOUCH_PAD2_DEBOUNCE_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:21 - touch data debounce of touch pad 2"]
    #[inline(always)]
    pub fn sar_touch_pad2_data(&self) -> SAR_TOUCH_PAD2_DATA_R {
        SAR_TOUCH_PAD2_DATA_R::new(self.bits & 0x003f_ffff)
    }
    #[doc = "Bits 29:31 - touch current debounce of touch pad 2"]
    #[inline(always)]
    pub fn sar_touch_pad2_debounce(&self) -> SAR_TOUCH_PAD2_DEBOUNCE_R {
        SAR_TOUCH_PAD2_DEBOUNCE_R::new(((self.bits >> 29) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_TOUCH_STATUS2")
            .field(
                "sar_touch_pad2_data",
                &format_args!("{}", self.sar_touch_pad2_data().bits()),
            )
            .field(
                "sar_touch_pad2_debounce",
                &format_args!("{}", self.sar_touch_pad2_debounce().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SAR_TOUCH_STATUS2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "touch channel status of touch pad 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar_touch_status2](index.html) module"]
pub struct SAR_TOUCH_STATUS2_SPEC;
impl crate::RegisterSpec for SAR_TOUCH_STATUS2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sar_touch_status2::R](R) reader structure"]
impl crate::Readable for SAR_TOUCH_STATUS2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SAR_TOUCH_STATUS2 to value 0"]
impl crate::Resettable for SAR_TOUCH_STATUS2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
