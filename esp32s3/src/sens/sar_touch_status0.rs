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
#[doc = "Field `SAR_TOUCH_SCAN_CURR` reader - current sample channel"]
pub type SAR_TOUCH_SCAN_CURR_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 22:25 - current sample channel"]
    #[inline(always)]
    pub fn sar_touch_scan_curr(&self) -> SAR_TOUCH_SCAN_CURR_R {
        SAR_TOUCH_SCAN_CURR_R::new(((self.bits >> 22) & 0x0f) as u8)
    }
}
#[doc = "get touch scan status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar_touch_status0](index.html) module"]
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
