#[doc = "Register `SAR_COCPU_INT_ENA_W1TS` writer"]
pub type W = crate::W<SAR_COCPU_INT_ENA_W1TS_SPEC>;
#[doc = "Field `SAR_COCPU_TOUCH_DONE_INT_ENA_W1TS` writer - int enable of touch done"]
pub type SAR_COCPU_TOUCH_DONE_INT_ENA_W1TS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAR_COCPU_TOUCH_INACTIVE_INT_ENA_W1TS` writer - int enable of from touch inactive"]
pub type SAR_COCPU_TOUCH_INACTIVE_INT_ENA_W1TS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAR_COCPU_TOUCH_ACTIVE_INT_ENA_W1TS` writer - int enable of touch active"]
pub type SAR_COCPU_TOUCH_ACTIVE_INT_ENA_W1TS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAR_COCPU_SARADC1_INT_ENA_W1TS` writer - int enable of from saradc1"]
pub type SAR_COCPU_SARADC1_INT_ENA_W1TS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAR_COCPU_SARADC2_INT_ENA_W1TS` writer - int enable of from saradc2"]
pub type SAR_COCPU_SARADC2_INT_ENA_W1TS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAR_COCPU_TSENS_INT_ENA_W1TS` writer - int enable of tsens"]
pub type SAR_COCPU_TSENS_INT_ENA_W1TS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAR_COCPU_START_INT_ENA_W1TS` writer - int enable of start"]
pub type SAR_COCPU_START_INT_ENA_W1TS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAR_COCPU_SW_INT_ENA_W1TS` writer - int enable of software"]
pub type SAR_COCPU_SW_INT_ENA_W1TS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAR_COCPU_SWD_INT_ENA_W1TS` writer - int enable of super watch dog"]
pub type SAR_COCPU_SWD_INT_ENA_W1TS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAR_COCPU_TOUCH_TIMEOUT_INT_ENA_W1TS` writer - int enable of timeout done"]
pub type SAR_COCPU_TOUCH_TIMEOUT_INT_ENA_W1TS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAR_COCPU_TOUCH_APPROACH_LOOP_DONE_INT_ENA_W1TS` writer - int enable of approach loop done"]
pub type SAR_COCPU_TOUCH_APPROACH_LOOP_DONE_INT_ENA_W1TS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAR_COCPU_TOUCH_SCAN_DONE_INT_ENA_W1TS` writer - int enable of touch scan done"]
pub type SAR_COCPU_TOUCH_SCAN_DONE_INT_ENA_W1TS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SAR_COCPU_INT_ENA_W1TS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - int enable of touch done"]
    #[inline(always)]
    pub fn sar_cocpu_touch_done_int_ena_w1ts(
        &mut self,
    ) -> SAR_COCPU_TOUCH_DONE_INT_ENA_W1TS_W<'_, SAR_COCPU_INT_ENA_W1TS_SPEC> {
        SAR_COCPU_TOUCH_DONE_INT_ENA_W1TS_W::new(self, 0)
    }
    #[doc = "Bit 1 - int enable of from touch inactive"]
    #[inline(always)]
    pub fn sar_cocpu_touch_inactive_int_ena_w1ts(
        &mut self,
    ) -> SAR_COCPU_TOUCH_INACTIVE_INT_ENA_W1TS_W<'_, SAR_COCPU_INT_ENA_W1TS_SPEC> {
        SAR_COCPU_TOUCH_INACTIVE_INT_ENA_W1TS_W::new(self, 1)
    }
    #[doc = "Bit 2 - int enable of touch active"]
    #[inline(always)]
    pub fn sar_cocpu_touch_active_int_ena_w1ts(
        &mut self,
    ) -> SAR_COCPU_TOUCH_ACTIVE_INT_ENA_W1TS_W<'_, SAR_COCPU_INT_ENA_W1TS_SPEC> {
        SAR_COCPU_TOUCH_ACTIVE_INT_ENA_W1TS_W::new(self, 2)
    }
    #[doc = "Bit 3 - int enable of from saradc1"]
    #[inline(always)]
    pub fn sar_cocpu_saradc1_int_ena_w1ts(
        &mut self,
    ) -> SAR_COCPU_SARADC1_INT_ENA_W1TS_W<'_, SAR_COCPU_INT_ENA_W1TS_SPEC> {
        SAR_COCPU_SARADC1_INT_ENA_W1TS_W::new(self, 3)
    }
    #[doc = "Bit 4 - int enable of from saradc2"]
    #[inline(always)]
    pub fn sar_cocpu_saradc2_int_ena_w1ts(
        &mut self,
    ) -> SAR_COCPU_SARADC2_INT_ENA_W1TS_W<'_, SAR_COCPU_INT_ENA_W1TS_SPEC> {
        SAR_COCPU_SARADC2_INT_ENA_W1TS_W::new(self, 4)
    }
    #[doc = "Bit 5 - int enable of tsens"]
    #[inline(always)]
    pub fn sar_cocpu_tsens_int_ena_w1ts(
        &mut self,
    ) -> SAR_COCPU_TSENS_INT_ENA_W1TS_W<'_, SAR_COCPU_INT_ENA_W1TS_SPEC> {
        SAR_COCPU_TSENS_INT_ENA_W1TS_W::new(self, 5)
    }
    #[doc = "Bit 6 - int enable of start"]
    #[inline(always)]
    pub fn sar_cocpu_start_int_ena_w1ts(
        &mut self,
    ) -> SAR_COCPU_START_INT_ENA_W1TS_W<'_, SAR_COCPU_INT_ENA_W1TS_SPEC> {
        SAR_COCPU_START_INT_ENA_W1TS_W::new(self, 6)
    }
    #[doc = "Bit 7 - int enable of software"]
    #[inline(always)]
    pub fn sar_cocpu_sw_int_ena_w1ts(
        &mut self,
    ) -> SAR_COCPU_SW_INT_ENA_W1TS_W<'_, SAR_COCPU_INT_ENA_W1TS_SPEC> {
        SAR_COCPU_SW_INT_ENA_W1TS_W::new(self, 7)
    }
    #[doc = "Bit 8 - int enable of super watch dog"]
    #[inline(always)]
    pub fn sar_cocpu_swd_int_ena_w1ts(
        &mut self,
    ) -> SAR_COCPU_SWD_INT_ENA_W1TS_W<'_, SAR_COCPU_INT_ENA_W1TS_SPEC> {
        SAR_COCPU_SWD_INT_ENA_W1TS_W::new(self, 8)
    }
    #[doc = "Bit 9 - int enable of timeout done"]
    #[inline(always)]
    pub fn sar_cocpu_touch_timeout_int_ena_w1ts(
        &mut self,
    ) -> SAR_COCPU_TOUCH_TIMEOUT_INT_ENA_W1TS_W<'_, SAR_COCPU_INT_ENA_W1TS_SPEC> {
        SAR_COCPU_TOUCH_TIMEOUT_INT_ENA_W1TS_W::new(self, 9)
    }
    #[doc = "Bit 10 - int enable of approach loop done"]
    #[inline(always)]
    pub fn sar_cocpu_touch_approach_loop_done_int_ena_w1ts(
        &mut self,
    ) -> SAR_COCPU_TOUCH_APPROACH_LOOP_DONE_INT_ENA_W1TS_W<'_, SAR_COCPU_INT_ENA_W1TS_SPEC> {
        SAR_COCPU_TOUCH_APPROACH_LOOP_DONE_INT_ENA_W1TS_W::new(self, 10)
    }
    #[doc = "Bit 11 - int enable of touch scan done"]
    #[inline(always)]
    pub fn sar_cocpu_touch_scan_done_int_ena_w1ts(
        &mut self,
    ) -> SAR_COCPU_TOUCH_SCAN_DONE_INT_ENA_W1TS_W<'_, SAR_COCPU_INT_ENA_W1TS_SPEC> {
        SAR_COCPU_TOUCH_SCAN_DONE_INT_ENA_W1TS_W::new(self, 11)
    }
}
#[doc = "the interrupt enable of ulp\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_cocpu_int_ena_w1ts::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAR_COCPU_INT_ENA_W1TS_SPEC;
impl crate::RegisterSpec for SAR_COCPU_INT_ENA_W1TS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`sar_cocpu_int_ena_w1ts::W`](W) writer structure"]
impl crate::Writable for SAR_COCPU_INT_ENA_W1TS_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SAR_COCPU_INT_ENA_W1TS to value 0"]
impl crate::Resettable for SAR_COCPU_INT_ENA_W1TS_SPEC {}
