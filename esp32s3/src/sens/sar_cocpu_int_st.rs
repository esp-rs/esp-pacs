#[doc = "Register `SAR_COCPU_INT_ST` reader"]
pub type R = crate::R<SAR_COCPU_INT_ST_SPEC>;
#[doc = "Field `SAR_COCPU_TOUCH_DONE_INT_ST` reader - int state of touch done"]
pub type SAR_COCPU_TOUCH_DONE_INT_ST_R = crate::BitReader;
#[doc = "Field `SAR_COCPU_TOUCH_INACTIVE_INT_ST` reader - int state of from touch inactive"]
pub type SAR_COCPU_TOUCH_INACTIVE_INT_ST_R = crate::BitReader;
#[doc = "Field `SAR_COCPU_TOUCH_ACTIVE_INT_ST` reader - int state of touch active"]
pub type SAR_COCPU_TOUCH_ACTIVE_INT_ST_R = crate::BitReader;
#[doc = "Field `SAR_COCPU_SARADC1_INT_ST` reader - int state of from saradc1"]
pub type SAR_COCPU_SARADC1_INT_ST_R = crate::BitReader;
#[doc = "Field `SAR_COCPU_SARADC2_INT_ST` reader - int state of from saradc2"]
pub type SAR_COCPU_SARADC2_INT_ST_R = crate::BitReader;
#[doc = "Field `SAR_COCPU_TSENS_INT_ST` reader - int state of tsens"]
pub type SAR_COCPU_TSENS_INT_ST_R = crate::BitReader;
#[doc = "Field `SAR_COCPU_START_INT_ST` reader - int state of start"]
pub type SAR_COCPU_START_INT_ST_R = crate::BitReader;
#[doc = "Field `SAR_COCPU_SW_INT_ST` reader - int state of software"]
pub type SAR_COCPU_SW_INT_ST_R = crate::BitReader;
#[doc = "Field `SAR_COCPU_SWD_INT_ST` reader - int state of super watch dog"]
pub type SAR_COCPU_SWD_INT_ST_R = crate::BitReader;
#[doc = "Field `SAR_COCPU_TOUCH_TIMEOUT_INT_ST` reader - int state of timeout done"]
pub type SAR_COCPU_TOUCH_TIMEOUT_INT_ST_R = crate::BitReader;
#[doc = "Field `SAR_COCPU_TOUCH_APPROACH_LOOP_DONE_INT_ST` reader - int state of approach loop done"]
pub type SAR_COCPU_TOUCH_APPROACH_LOOP_DONE_INT_ST_R = crate::BitReader;
#[doc = "Field `SAR_COCPU_TOUCH_SCAN_DONE_INT_ST` reader - int state of touch scan done"]
pub type SAR_COCPU_TOUCH_SCAN_DONE_INT_ST_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - int state of touch done"]
    #[inline(always)]
    pub fn sar_cocpu_touch_done_int_st(&self) -> SAR_COCPU_TOUCH_DONE_INT_ST_R {
        SAR_COCPU_TOUCH_DONE_INT_ST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - int state of from touch inactive"]
    #[inline(always)]
    pub fn sar_cocpu_touch_inactive_int_st(&self) -> SAR_COCPU_TOUCH_INACTIVE_INT_ST_R {
        SAR_COCPU_TOUCH_INACTIVE_INT_ST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - int state of touch active"]
    #[inline(always)]
    pub fn sar_cocpu_touch_active_int_st(&self) -> SAR_COCPU_TOUCH_ACTIVE_INT_ST_R {
        SAR_COCPU_TOUCH_ACTIVE_INT_ST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - int state of from saradc1"]
    #[inline(always)]
    pub fn sar_cocpu_saradc1_int_st(&self) -> SAR_COCPU_SARADC1_INT_ST_R {
        SAR_COCPU_SARADC1_INT_ST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - int state of from saradc2"]
    #[inline(always)]
    pub fn sar_cocpu_saradc2_int_st(&self) -> SAR_COCPU_SARADC2_INT_ST_R {
        SAR_COCPU_SARADC2_INT_ST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - int state of tsens"]
    #[inline(always)]
    pub fn sar_cocpu_tsens_int_st(&self) -> SAR_COCPU_TSENS_INT_ST_R {
        SAR_COCPU_TSENS_INT_ST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - int state of start"]
    #[inline(always)]
    pub fn sar_cocpu_start_int_st(&self) -> SAR_COCPU_START_INT_ST_R {
        SAR_COCPU_START_INT_ST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - int state of software"]
    #[inline(always)]
    pub fn sar_cocpu_sw_int_st(&self) -> SAR_COCPU_SW_INT_ST_R {
        SAR_COCPU_SW_INT_ST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - int state of super watch dog"]
    #[inline(always)]
    pub fn sar_cocpu_swd_int_st(&self) -> SAR_COCPU_SWD_INT_ST_R {
        SAR_COCPU_SWD_INT_ST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - int state of timeout done"]
    #[inline(always)]
    pub fn sar_cocpu_touch_timeout_int_st(&self) -> SAR_COCPU_TOUCH_TIMEOUT_INT_ST_R {
        SAR_COCPU_TOUCH_TIMEOUT_INT_ST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - int state of approach loop done"]
    #[inline(always)]
    pub fn sar_cocpu_touch_approach_loop_done_int_st(
        &self,
    ) -> SAR_COCPU_TOUCH_APPROACH_LOOP_DONE_INT_ST_R {
        SAR_COCPU_TOUCH_APPROACH_LOOP_DONE_INT_ST_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - int state of touch scan done"]
    #[inline(always)]
    pub fn sar_cocpu_touch_scan_done_int_st(&self) -> SAR_COCPU_TOUCH_SCAN_DONE_INT_ST_R {
        SAR_COCPU_TOUCH_SCAN_DONE_INT_ST_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_COCPU_INT_ST")
            .field(
                "sar_cocpu_touch_done_int_st",
                &self.sar_cocpu_touch_done_int_st(),
            )
            .field(
                "sar_cocpu_touch_inactive_int_st",
                &self.sar_cocpu_touch_inactive_int_st(),
            )
            .field(
                "sar_cocpu_touch_active_int_st",
                &self.sar_cocpu_touch_active_int_st(),
            )
            .field("sar_cocpu_saradc1_int_st", &self.sar_cocpu_saradc1_int_st())
            .field("sar_cocpu_saradc2_int_st", &self.sar_cocpu_saradc2_int_st())
            .field("sar_cocpu_tsens_int_st", &self.sar_cocpu_tsens_int_st())
            .field("sar_cocpu_start_int_st", &self.sar_cocpu_start_int_st())
            .field("sar_cocpu_sw_int_st", &self.sar_cocpu_sw_int_st())
            .field("sar_cocpu_swd_int_st", &self.sar_cocpu_swd_int_st())
            .field(
                "sar_cocpu_touch_timeout_int_st",
                &self.sar_cocpu_touch_timeout_int_st(),
            )
            .field(
                "sar_cocpu_touch_approach_loop_done_int_st",
                &self.sar_cocpu_touch_approach_loop_done_int_st(),
            )
            .field(
                "sar_cocpu_touch_scan_done_int_st",
                &self.sar_cocpu_touch_scan_done_int_st(),
            )
            .finish()
    }
}
#[doc = "the interrupt state of ulp\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_cocpu_int_st::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAR_COCPU_INT_ST_SPEC;
impl crate::RegisterSpec for SAR_COCPU_INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar_cocpu_int_st::R`](R) reader structure"]
impl crate::Readable for SAR_COCPU_INT_ST_SPEC {}
#[doc = "`reset()` method sets SAR_COCPU_INT_ST to value 0"]
impl crate::Resettable for SAR_COCPU_INT_ST_SPEC {}
