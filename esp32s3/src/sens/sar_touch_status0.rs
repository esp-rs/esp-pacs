#[doc = "Register `SAR_TOUCH_STATUS0` reader"]
pub type R = crate::R<SAR_TOUCH_STATUS0_SPEC>;
#[doc = "Field `SAR_TOUCH_SCAN_CURR` reader - current sample channel"]
pub type SAR_TOUCH_SCAN_CURR_R = crate::FieldReader;
impl R {
    #[doc = "Bits 22:25 - current sample channel"]
    #[inline(always)]
    pub fn sar_touch_scan_curr(&self) -> SAR_TOUCH_SCAN_CURR_R {
        SAR_TOUCH_SCAN_CURR_R::new(((self.bits >> 22) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_TOUCH_STATUS0")
            .field(
                "sar_touch_scan_curr",
                &format_args!("{}", self.sar_touch_scan_curr().bits()),
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
#[doc = "get touch scan status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_touch_status0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAR_TOUCH_STATUS0_SPEC;
impl crate::RegisterSpec for SAR_TOUCH_STATUS0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar_touch_status0::R`](R) reader structure"]
impl crate::Readable for SAR_TOUCH_STATUS0_SPEC {}
#[doc = "`reset()` method sets SAR_TOUCH_STATUS0 to value 0"]
impl crate::Resettable for SAR_TOUCH_STATUS0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
