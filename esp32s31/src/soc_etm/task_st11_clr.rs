#[doc = "Register `TASK_ST11_CLR` writer"]
pub type W = crate::W<TASK_ST11_CLR_SPEC>;
#[doc = "Field `MODEM_TASK_G3_ST_CLR` writer - Configures whether or not to clear MODEM_task_g3 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type MODEM_TASK_G3_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORDIC_TASK_CAL_START_ST_CLR` writer - Configures whether or not to clear CORDIC_task_cal_start trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type CORDIC_TASK_CAL_START_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ZERO_DET_TASK_START_ST_CLR` writer - Configures whether or not to clear ZERO_DET_task_start trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type ZERO_DET_TASK_START_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSTIMER_TASK_CNT_UPD0_ST_CLR` writer - Configures whether or not to clear SYSTIMER_task_cnt_upd0 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type SYSTIMER_TASK_CNT_UPD0_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSTIMER_TASK_CNT_UPD1_ST_CLR` writer - Configures whether or not to clear SYSTIMER_task_cnt_upd1 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type SYSTIMER_TASK_CNT_UPD1_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TASK_ST11_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Configures whether or not to clear MODEM_task_g3 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn modem_task_g3_st_clr(&mut self) -> MODEM_TASK_G3_ST_CLR_W<'_, TASK_ST11_CLR_SPEC> {
        MODEM_TASK_G3_ST_CLR_W::new(self, 0)
    }
    #[doc = "Bit 1 - Configures whether or not to clear CORDIC_task_cal_start trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn cordic_task_cal_start_st_clr(
        &mut self,
    ) -> CORDIC_TASK_CAL_START_ST_CLR_W<'_, TASK_ST11_CLR_SPEC> {
        CORDIC_TASK_CAL_START_ST_CLR_W::new(self, 1)
    }
    #[doc = "Bit 2 - Configures whether or not to clear ZERO_DET_task_start trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn zero_det_task_start_st_clr(
        &mut self,
    ) -> ZERO_DET_TASK_START_ST_CLR_W<'_, TASK_ST11_CLR_SPEC> {
        ZERO_DET_TASK_START_ST_CLR_W::new(self, 2)
    }
    #[doc = "Bit 3 - Configures whether or not to clear SYSTIMER_task_cnt_upd0 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn systimer_task_cnt_upd0_st_clr(
        &mut self,
    ) -> SYSTIMER_TASK_CNT_UPD0_ST_CLR_W<'_, TASK_ST11_CLR_SPEC> {
        SYSTIMER_TASK_CNT_UPD0_ST_CLR_W::new(self, 3)
    }
    #[doc = "Bit 4 - Configures whether or not to clear SYSTIMER_task_cnt_upd1 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn systimer_task_cnt_upd1_st_clr(
        &mut self,
    ) -> SYSTIMER_TASK_CNT_UPD1_ST_CLR_W<'_, TASK_ST11_CLR_SPEC> {
        SYSTIMER_TASK_CNT_UPD1_ST_CLR_W::new(self, 4)
    }
}
#[doc = "Tasks trigger status clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`task_st11_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TASK_ST11_CLR_SPEC;
impl crate::RegisterSpec for TASK_ST11_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`task_st11_clr::W`](W) writer structure"]
impl crate::Writable for TASK_ST11_CLR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASK_ST11_CLR to value 0"]
impl crate::Resettable for TASK_ST11_CLR_SPEC {}
