#[doc = "Register `TASK_ST4_CLR` writer"]
pub type W = crate::W<TASK_ST4_CLR_SPEC>;
#[doc = "Field `GDMA_AHB_TASK_OUT_START_CH1_ST_CLR` writer - Configures whether or not to clear GDMA_AHB_task_out_start_ch1 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type GDMA_AHB_TASK_OUT_START_CH1_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GDMA_AHB_TASK_OUT_START_CH2_ST_CLR` writer - Configures whether or not to clear GDMA_AHB_task_out_start_ch2 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type GDMA_AHB_TASK_OUT_START_CH2_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PMU_TASK_SLEEP_REQ_ST_CLR` writer - Configures whether or not to clear PMU_task_sleep_req trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type PMU_TASK_SLEEP_REQ_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TASK_ST4_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Configures whether or not to clear GDMA_AHB_task_out_start_ch1 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn gdma_ahb_task_out_start_ch1_st_clr(
        &mut self,
    ) -> GDMA_AHB_TASK_OUT_START_CH1_ST_CLR_W<TASK_ST4_CLR_SPEC> {
        GDMA_AHB_TASK_OUT_START_CH1_ST_CLR_W::new(self, 0)
    }
    #[doc = "Bit 1 - Configures whether or not to clear GDMA_AHB_task_out_start_ch2 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn gdma_ahb_task_out_start_ch2_st_clr(
        &mut self,
    ) -> GDMA_AHB_TASK_OUT_START_CH2_ST_CLR_W<TASK_ST4_CLR_SPEC> {
        GDMA_AHB_TASK_OUT_START_CH2_ST_CLR_W::new(self, 1)
    }
    #[doc = "Bit 2 - Configures whether or not to clear PMU_task_sleep_req trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn pmu_task_sleep_req_st_clr(&mut self) -> PMU_TASK_SLEEP_REQ_ST_CLR_W<TASK_ST4_CLR_SPEC> {
        PMU_TASK_SLEEP_REQ_ST_CLR_W::new(self, 2)
    }
}
#[doc = "Tasks trigger status clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`task_st4_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TASK_ST4_CLR_SPEC;
impl crate::RegisterSpec for TASK_ST4_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`task_st4_clr::W`](W) writer structure"]
impl crate::Writable for TASK_ST4_CLR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASK_ST4_CLR to value 0"]
impl crate::Resettable for TASK_ST4_CLR_SPEC {}
