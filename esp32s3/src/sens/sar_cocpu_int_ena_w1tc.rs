///Register `SAR_COCPU_INT_ENA_W1TC` writer
pub type W = crate::W<SAR_COCPU_INT_ENA_W1TC_SPEC>;
///Field `SAR_COCPU_TOUCH_DONE_INT_ENA_W1TC` writer - Clear int enable of touch done
pub type SAR_COCPU_TOUCH_DONE_INT_ENA_W1TC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SAR_COCPU_TOUCH_INACTIVE_INT_ENA_W1TC` writer - Clear int enable of from touch inactive
pub type SAR_COCPU_TOUCH_INACTIVE_INT_ENA_W1TC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SAR_COCPU_TOUCH_ACTIVE_INT_ENA_W1TC` writer - Clear int enable of touch active
pub type SAR_COCPU_TOUCH_ACTIVE_INT_ENA_W1TC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SAR_COCPU_SARADC1_INT_ENA_W1TC` writer - Clear int enable of from saradc1
pub type SAR_COCPU_SARADC1_INT_ENA_W1TC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SAR_COCPU_SARADC2_INT_ENA_W1TC` writer - Clear int enable of from saradc2
pub type SAR_COCPU_SARADC2_INT_ENA_W1TC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SAR_COCPU_TSENS_INT_ENA_W1TC` writer - Clear int enable of tsens
pub type SAR_COCPU_TSENS_INT_ENA_W1TC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SAR_COCPU_START_INT_ENA_W1TC` writer - Clear int enable of start
pub type SAR_COCPU_START_INT_ENA_W1TC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SAR_COCPU_SW_INT_ENA_W1TC` writer - Clear int enable of software
pub type SAR_COCPU_SW_INT_ENA_W1TC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SAR_COCPU_SWD_INT_ENA_W1TC` writer - Clear int enable of super watch dog
pub type SAR_COCPU_SWD_INT_ENA_W1TC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SAR_COCPU_TOUCH_TIMEOUT_INT_ENA_W1TC` writer - Clear int enable of timeout done
pub type SAR_COCPU_TOUCH_TIMEOUT_INT_ENA_W1TC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SAR_COCPU_TOUCH_APPROACH_LOOP_DONE_INT_ENA_W1TC` writer - Clear int enable of approach loop done
pub type SAR_COCPU_TOUCH_APPROACH_LOOP_DONE_INT_ENA_W1TC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SAR_COCPU_TOUCH_SCAN_DONE_INT_ENA_W1TC` writer - Clear int enable of touch scan done
pub type SAR_COCPU_TOUCH_SCAN_DONE_INT_ENA_W1TC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SAR_COCPU_INT_ENA_W1TC_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Clear int enable of touch done
    #[inline(always)]
    #[must_use]
    pub fn sar_cocpu_touch_done_int_ena_w1tc(
        &mut self,
    ) -> SAR_COCPU_TOUCH_DONE_INT_ENA_W1TC_W<SAR_COCPU_INT_ENA_W1TC_SPEC> {
        SAR_COCPU_TOUCH_DONE_INT_ENA_W1TC_W::new(self, 0)
    }
    ///Bit 1 - Clear int enable of from touch inactive
    #[inline(always)]
    #[must_use]
    pub fn sar_cocpu_touch_inactive_int_ena_w1tc(
        &mut self,
    ) -> SAR_COCPU_TOUCH_INACTIVE_INT_ENA_W1TC_W<SAR_COCPU_INT_ENA_W1TC_SPEC> {
        SAR_COCPU_TOUCH_INACTIVE_INT_ENA_W1TC_W::new(self, 1)
    }
    ///Bit 2 - Clear int enable of touch active
    #[inline(always)]
    #[must_use]
    pub fn sar_cocpu_touch_active_int_ena_w1tc(
        &mut self,
    ) -> SAR_COCPU_TOUCH_ACTIVE_INT_ENA_W1TC_W<SAR_COCPU_INT_ENA_W1TC_SPEC> {
        SAR_COCPU_TOUCH_ACTIVE_INT_ENA_W1TC_W::new(self, 2)
    }
    ///Bit 3 - Clear int enable of from saradc1
    #[inline(always)]
    #[must_use]
    pub fn sar_cocpu_saradc1_int_ena_w1tc(
        &mut self,
    ) -> SAR_COCPU_SARADC1_INT_ENA_W1TC_W<SAR_COCPU_INT_ENA_W1TC_SPEC> {
        SAR_COCPU_SARADC1_INT_ENA_W1TC_W::new(self, 3)
    }
    ///Bit 4 - Clear int enable of from saradc2
    #[inline(always)]
    #[must_use]
    pub fn sar_cocpu_saradc2_int_ena_w1tc(
        &mut self,
    ) -> SAR_COCPU_SARADC2_INT_ENA_W1TC_W<SAR_COCPU_INT_ENA_W1TC_SPEC> {
        SAR_COCPU_SARADC2_INT_ENA_W1TC_W::new(self, 4)
    }
    ///Bit 5 - Clear int enable of tsens
    #[inline(always)]
    #[must_use]
    pub fn sar_cocpu_tsens_int_ena_w1tc(
        &mut self,
    ) -> SAR_COCPU_TSENS_INT_ENA_W1TC_W<SAR_COCPU_INT_ENA_W1TC_SPEC> {
        SAR_COCPU_TSENS_INT_ENA_W1TC_W::new(self, 5)
    }
    ///Bit 6 - Clear int enable of start
    #[inline(always)]
    #[must_use]
    pub fn sar_cocpu_start_int_ena_w1tc(
        &mut self,
    ) -> SAR_COCPU_START_INT_ENA_W1TC_W<SAR_COCPU_INT_ENA_W1TC_SPEC> {
        SAR_COCPU_START_INT_ENA_W1TC_W::new(self, 6)
    }
    ///Bit 7 - Clear int enable of software
    #[inline(always)]
    #[must_use]
    pub fn sar_cocpu_sw_int_ena_w1tc(
        &mut self,
    ) -> SAR_COCPU_SW_INT_ENA_W1TC_W<SAR_COCPU_INT_ENA_W1TC_SPEC> {
        SAR_COCPU_SW_INT_ENA_W1TC_W::new(self, 7)
    }
    ///Bit 8 - Clear int enable of super watch dog
    #[inline(always)]
    #[must_use]
    pub fn sar_cocpu_swd_int_ena_w1tc(
        &mut self,
    ) -> SAR_COCPU_SWD_INT_ENA_W1TC_W<SAR_COCPU_INT_ENA_W1TC_SPEC> {
        SAR_COCPU_SWD_INT_ENA_W1TC_W::new(self, 8)
    }
    ///Bit 9 - Clear int enable of timeout done
    #[inline(always)]
    #[must_use]
    pub fn sar_cocpu_touch_timeout_int_ena_w1tc(
        &mut self,
    ) -> SAR_COCPU_TOUCH_TIMEOUT_INT_ENA_W1TC_W<SAR_COCPU_INT_ENA_W1TC_SPEC> {
        SAR_COCPU_TOUCH_TIMEOUT_INT_ENA_W1TC_W::new(self, 9)
    }
    ///Bit 10 - Clear int enable of approach loop done
    #[inline(always)]
    #[must_use]
    pub fn sar_cocpu_touch_approach_loop_done_int_ena_w1tc(
        &mut self,
    ) -> SAR_COCPU_TOUCH_APPROACH_LOOP_DONE_INT_ENA_W1TC_W<SAR_COCPU_INT_ENA_W1TC_SPEC> {
        SAR_COCPU_TOUCH_APPROACH_LOOP_DONE_INT_ENA_W1TC_W::new(self, 10)
    }
    ///Bit 11 - Clear int enable of touch scan done
    #[inline(always)]
    #[must_use]
    pub fn sar_cocpu_touch_scan_done_int_ena_w1tc(
        &mut self,
    ) -> SAR_COCPU_TOUCH_SCAN_DONE_INT_ENA_W1TC_W<SAR_COCPU_INT_ENA_W1TC_SPEC> {
        SAR_COCPU_TOUCH_SCAN_DONE_INT_ENA_W1TC_W::new(self, 11)
    }
}
/**the interrupt enable clear of ulp

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_cocpu_int_ena_w1tc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SAR_COCPU_INT_ENA_W1TC_SPEC;
impl crate::RegisterSpec for SAR_COCPU_INT_ENA_W1TC_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`sar_cocpu_int_ena_w1tc::W`](W) writer structure
impl crate::Writable for SAR_COCPU_INT_ENA_W1TC_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SAR_COCPU_INT_ENA_W1TC to value 0
impl crate::Resettable for SAR_COCPU_INT_ENA_W1TC_SPEC {
    const RESET_VALUE: u32 = 0;
}
