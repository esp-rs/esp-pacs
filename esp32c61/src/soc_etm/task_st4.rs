#[doc = "Register `TASK_ST4` reader"]
pub type R = crate::R<TASK_ST4_SPEC>;
#[doc = "Register `TASK_ST4` writer"]
pub type W = crate::W<TASK_ST4_SPEC>;
#[doc = "Field `GDMA_AHB_TASK_OUT_START_CH1_ST` reader - Represents GDMA_AHB_task_out_start_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type GDMA_AHB_TASK_OUT_START_CH1_ST_R = crate::BitReader;
#[doc = "Field `GDMA_AHB_TASK_OUT_START_CH1_ST` writer - Represents GDMA_AHB_task_out_start_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type GDMA_AHB_TASK_OUT_START_CH1_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GDMA_AHB_TASK_OUT_START_CH2_ST` reader - Represents GDMA_AHB_task_out_start_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type GDMA_AHB_TASK_OUT_START_CH2_ST_R = crate::BitReader;
#[doc = "Field `GDMA_AHB_TASK_OUT_START_CH2_ST` writer - Represents GDMA_AHB_task_out_start_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type GDMA_AHB_TASK_OUT_START_CH2_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PMU_TASK_SLEEP_REQ_ST` reader - Represents PMU_task_sleep_req trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PMU_TASK_SLEEP_REQ_ST_R = crate::BitReader;
#[doc = "Field `PMU_TASK_SLEEP_REQ_ST` writer - Represents PMU_task_sleep_req trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PMU_TASK_SLEEP_REQ_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Represents GDMA_AHB_task_out_start_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn gdma_ahb_task_out_start_ch1_st(&self) -> GDMA_AHB_TASK_OUT_START_CH1_ST_R {
        GDMA_AHB_TASK_OUT_START_CH1_ST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Represents GDMA_AHB_task_out_start_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn gdma_ahb_task_out_start_ch2_st(&self) -> GDMA_AHB_TASK_OUT_START_CH2_ST_R {
        GDMA_AHB_TASK_OUT_START_CH2_ST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Represents PMU_task_sleep_req trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pmu_task_sleep_req_st(&self) -> PMU_TASK_SLEEP_REQ_ST_R {
        PMU_TASK_SLEEP_REQ_ST_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TASK_ST4")
            .field(
                "gdma_ahb_task_out_start_ch1_st",
                &self.gdma_ahb_task_out_start_ch1_st(),
            )
            .field(
                "gdma_ahb_task_out_start_ch2_st",
                &self.gdma_ahb_task_out_start_ch2_st(),
            )
            .field("pmu_task_sleep_req_st", &self.pmu_task_sleep_req_st())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Represents GDMA_AHB_task_out_start_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn gdma_ahb_task_out_start_ch1_st(
        &mut self,
    ) -> GDMA_AHB_TASK_OUT_START_CH1_ST_W<'_, TASK_ST4_SPEC> {
        GDMA_AHB_TASK_OUT_START_CH1_ST_W::new(self, 0)
    }
    #[doc = "Bit 1 - Represents GDMA_AHB_task_out_start_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn gdma_ahb_task_out_start_ch2_st(
        &mut self,
    ) -> GDMA_AHB_TASK_OUT_START_CH2_ST_W<'_, TASK_ST4_SPEC> {
        GDMA_AHB_TASK_OUT_START_CH2_ST_W::new(self, 1)
    }
    #[doc = "Bit 2 - Represents PMU_task_sleep_req trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pmu_task_sleep_req_st(&mut self) -> PMU_TASK_SLEEP_REQ_ST_W<'_, TASK_ST4_SPEC> {
        PMU_TASK_SLEEP_REQ_ST_W::new(self, 2)
    }
}
#[doc = "Tasks trigger status register\n\nYou can [`read`](crate::Reg::read) this register and get [`task_st4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`task_st4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TASK_ST4_SPEC;
impl crate::RegisterSpec for TASK_ST4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`task_st4::R`](R) reader structure"]
impl crate::Readable for TASK_ST4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`task_st4::W`](W) writer structure"]
impl crate::Writable for TASK_ST4_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASK_ST4 to value 0"]
impl crate::Resettable for TASK_ST4_SPEC {}
