#[doc = "Register `SAR_COCPU_INT_RAW` reader"]
pub struct R(crate::R<SAR_COCPU_INT_RAW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAR_COCPU_INT_RAW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAR_COCPU_INT_RAW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAR_COCPU_INT_RAW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SAR_COCPU_TOUCH_DONE_INT_RAW` reader - int from touch done"]
pub type SAR_COCPU_TOUCH_DONE_INT_RAW_R = crate::BitReader;
#[doc = "Field `SAR_COCPU_TOUCH_INACTIVE_INT_RAW` reader - int from touch inactive"]
pub type SAR_COCPU_TOUCH_INACTIVE_INT_RAW_R = crate::BitReader;
#[doc = "Field `SAR_COCPU_TOUCH_ACTIVE_INT_RAW` reader - int from touch active"]
pub type SAR_COCPU_TOUCH_ACTIVE_INT_RAW_R = crate::BitReader;
#[doc = "Field `SAR_COCPU_SARADC1_INT_RAW` reader - int from saradc1"]
pub type SAR_COCPU_SARADC1_INT_RAW_R = crate::BitReader;
#[doc = "Field `SAR_COCPU_SARADC2_INT_RAW` reader - int from saradc2"]
pub type SAR_COCPU_SARADC2_INT_RAW_R = crate::BitReader;
#[doc = "Field `SAR_COCPU_TSENS_INT_RAW` reader - int from tsens"]
pub type SAR_COCPU_TSENS_INT_RAW_R = crate::BitReader;
#[doc = "Field `SAR_COCPU_START_INT_RAW` reader - int from start"]
pub type SAR_COCPU_START_INT_RAW_R = crate::BitReader;
#[doc = "Field `SAR_COCPU_SW_INT_RAW` reader - int from software"]
pub type SAR_COCPU_SW_INT_RAW_R = crate::BitReader;
#[doc = "Field `SAR_COCPU_SWD_INT_RAW` reader - int from super watch dog"]
pub type SAR_COCPU_SWD_INT_RAW_R = crate::BitReader;
#[doc = "Field `SAR_COCPU_TOUCH_TIMEOUT_INT_RAW` reader - int from timeout done"]
pub type SAR_COCPU_TOUCH_TIMEOUT_INT_RAW_R = crate::BitReader;
#[doc = "Field `SAR_COCPU_TOUCH_APPROACH_LOOP_DONE_INT_RAW` reader - int from approach loop done"]
pub type SAR_COCPU_TOUCH_APPROACH_LOOP_DONE_INT_RAW_R = crate::BitReader;
#[doc = "Field `SAR_COCPU_TOUCH_SCAN_DONE_INT_RAW` reader - int from touch scan done"]
pub type SAR_COCPU_TOUCH_SCAN_DONE_INT_RAW_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - int from touch done"]
    #[inline(always)]
    pub fn sar_cocpu_touch_done_int_raw(&self) -> SAR_COCPU_TOUCH_DONE_INT_RAW_R {
        SAR_COCPU_TOUCH_DONE_INT_RAW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - int from touch inactive"]
    #[inline(always)]
    pub fn sar_cocpu_touch_inactive_int_raw(&self) -> SAR_COCPU_TOUCH_INACTIVE_INT_RAW_R {
        SAR_COCPU_TOUCH_INACTIVE_INT_RAW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - int from touch active"]
    #[inline(always)]
    pub fn sar_cocpu_touch_active_int_raw(&self) -> SAR_COCPU_TOUCH_ACTIVE_INT_RAW_R {
        SAR_COCPU_TOUCH_ACTIVE_INT_RAW_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - int from saradc1"]
    #[inline(always)]
    pub fn sar_cocpu_saradc1_int_raw(&self) -> SAR_COCPU_SARADC1_INT_RAW_R {
        SAR_COCPU_SARADC1_INT_RAW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - int from saradc2"]
    #[inline(always)]
    pub fn sar_cocpu_saradc2_int_raw(&self) -> SAR_COCPU_SARADC2_INT_RAW_R {
        SAR_COCPU_SARADC2_INT_RAW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - int from tsens"]
    #[inline(always)]
    pub fn sar_cocpu_tsens_int_raw(&self) -> SAR_COCPU_TSENS_INT_RAW_R {
        SAR_COCPU_TSENS_INT_RAW_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - int from start"]
    #[inline(always)]
    pub fn sar_cocpu_start_int_raw(&self) -> SAR_COCPU_START_INT_RAW_R {
        SAR_COCPU_START_INT_RAW_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - int from software"]
    #[inline(always)]
    pub fn sar_cocpu_sw_int_raw(&self) -> SAR_COCPU_SW_INT_RAW_R {
        SAR_COCPU_SW_INT_RAW_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - int from super watch dog"]
    #[inline(always)]
    pub fn sar_cocpu_swd_int_raw(&self) -> SAR_COCPU_SWD_INT_RAW_R {
        SAR_COCPU_SWD_INT_RAW_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - int from timeout done"]
    #[inline(always)]
    pub fn sar_cocpu_touch_timeout_int_raw(&self) -> SAR_COCPU_TOUCH_TIMEOUT_INT_RAW_R {
        SAR_COCPU_TOUCH_TIMEOUT_INT_RAW_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - int from approach loop done"]
    #[inline(always)]
    pub fn sar_cocpu_touch_approach_loop_done_int_raw(
        &self,
    ) -> SAR_COCPU_TOUCH_APPROACH_LOOP_DONE_INT_RAW_R {
        SAR_COCPU_TOUCH_APPROACH_LOOP_DONE_INT_RAW_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - int from touch scan done"]
    #[inline(always)]
    pub fn sar_cocpu_touch_scan_done_int_raw(&self) -> SAR_COCPU_TOUCH_SCAN_DONE_INT_RAW_R {
        SAR_COCPU_TOUCH_SCAN_DONE_INT_RAW_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_COCPU_INT_RAW")
            .field(
                "sar_cocpu_touch_done_int_raw",
                &format_args!("{}", self.sar_cocpu_touch_done_int_raw().bit()),
            )
            .field(
                "sar_cocpu_touch_inactive_int_raw",
                &format_args!("{}", self.sar_cocpu_touch_inactive_int_raw().bit()),
            )
            .field(
                "sar_cocpu_touch_active_int_raw",
                &format_args!("{}", self.sar_cocpu_touch_active_int_raw().bit()),
            )
            .field(
                "sar_cocpu_saradc1_int_raw",
                &format_args!("{}", self.sar_cocpu_saradc1_int_raw().bit()),
            )
            .field(
                "sar_cocpu_saradc2_int_raw",
                &format_args!("{}", self.sar_cocpu_saradc2_int_raw().bit()),
            )
            .field(
                "sar_cocpu_tsens_int_raw",
                &format_args!("{}", self.sar_cocpu_tsens_int_raw().bit()),
            )
            .field(
                "sar_cocpu_start_int_raw",
                &format_args!("{}", self.sar_cocpu_start_int_raw().bit()),
            )
            .field(
                "sar_cocpu_sw_int_raw",
                &format_args!("{}", self.sar_cocpu_sw_int_raw().bit()),
            )
            .field(
                "sar_cocpu_swd_int_raw",
                &format_args!("{}", self.sar_cocpu_swd_int_raw().bit()),
            )
            .field(
                "sar_cocpu_touch_timeout_int_raw",
                &format_args!("{}", self.sar_cocpu_touch_timeout_int_raw().bit()),
            )
            .field(
                "sar_cocpu_touch_approach_loop_done_int_raw",
                &format_args!(
                    "{}",
                    self.sar_cocpu_touch_approach_loop_done_int_raw().bit()
                ),
            )
            .field(
                "sar_cocpu_touch_scan_done_int_raw",
                &format_args!("{}", self.sar_cocpu_touch_scan_done_int_raw().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SAR_COCPU_INT_RAW_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "the interrupt raw of ulp\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar_cocpu_int_raw](index.html) module"]
pub struct SAR_COCPU_INT_RAW_SPEC;
impl crate::RegisterSpec for SAR_COCPU_INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sar_cocpu_int_raw::R](R) reader structure"]
impl crate::Readable for SAR_COCPU_INT_RAW_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SAR_COCPU_INT_RAW to value 0"]
impl crate::Resettable for SAR_COCPU_INT_RAW_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
