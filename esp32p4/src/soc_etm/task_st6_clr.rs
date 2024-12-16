#[doc = "Register `TASK_ST6_CLR` writer"]
pub type W = crate::W<TASK_ST6_CLR_SPEC>;
#[doc = "Field `PDMA_AXI_TASK_IN_START_CH2_ST_CLR` writer - Configures whether or not to clear PDMA_AXI_task_in_start_ch2 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type PDMA_AXI_TASK_IN_START_CH2_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AXI_TASK_OUT_START_CH0_ST_CLR` writer - Configures whether or not to clear PDMA_AXI_task_out_start_ch0 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type PDMA_AXI_TASK_OUT_START_CH0_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AXI_TASK_OUT_START_CH1_ST_CLR` writer - Configures whether or not to clear PDMA_AXI_task_out_start_ch1 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type PDMA_AXI_TASK_OUT_START_CH1_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AXI_TASK_OUT_START_CH2_ST_CLR` writer - Configures whether or not to clear PDMA_AXI_task_out_start_ch2 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type PDMA_AXI_TASK_OUT_START_CH2_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PMU_TASK_SLEEP_REQ_ST_CLR` writer - Configures whether or not to clear PMU_task_sleep_req trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type PMU_TASK_SLEEP_REQ_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2D_TASK_IN_START_CH0_ST_CLR` writer - Configures whether or not to clear DMA2D_task_in_start_ch0 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type DMA2D_TASK_IN_START_CH0_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2D_TASK_IN_START_CH1_ST_CLR` writer - Configures whether or not to clear DMA2D_task_in_start_ch1 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type DMA2D_TASK_IN_START_CH1_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2D_TASK_IN_DSCR_READY_CH0_ST_CLR` writer - Configures whether or not to clear DMA2D_task_in_dscr_ready_ch0 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type DMA2D_TASK_IN_DSCR_READY_CH0_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2D_TASK_IN_DSCR_READY_CH1_ST_CLR` writer - Configures whether or not to clear DMA2D_task_in_dscr_ready_ch1 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type DMA2D_TASK_IN_DSCR_READY_CH1_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2D_TASK_OUT_START_CH0_ST_CLR` writer - Configures whether or not to clear DMA2D_task_out_start_ch0 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type DMA2D_TASK_OUT_START_CH0_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2D_TASK_OUT_START_CH1_ST_CLR` writer - Configures whether or not to clear DMA2D_task_out_start_ch1 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type DMA2D_TASK_OUT_START_CH1_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2D_TASK_OUT_START_CH2_ST_CLR` writer - Configures whether or not to clear DMA2D_task_out_start_ch2 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type DMA2D_TASK_OUT_START_CH2_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2D_TASK_OUT_DSCR_READY_CH0_ST_CLR` writer - Configures whether or not to clear DMA2D_task_out_dscr_ready_ch0 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type DMA2D_TASK_OUT_DSCR_READY_CH0_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2D_TASK_OUT_DSCR_READY_CH1_ST_CLR` writer - Configures whether or not to clear DMA2D_task_out_dscr_ready_ch1 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type DMA2D_TASK_OUT_DSCR_READY_CH1_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2D_TASK_OUT_DSCR_READY_CH2_ST_CLR` writer - Configures whether or not to clear DMA2D_task_out_dscr_ready_ch2 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type DMA2D_TASK_OUT_DSCR_READY_CH2_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TASK_ST6_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Configures whether or not to clear PDMA_AXI_task_in_start_ch2 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn pdma_axi_task_in_start_ch2_st_clr(
        &mut self,
    ) -> PDMA_AXI_TASK_IN_START_CH2_ST_CLR_W<TASK_ST6_CLR_SPEC> {
        PDMA_AXI_TASK_IN_START_CH2_ST_CLR_W::new(self, 0)
    }
    #[doc = "Bit 1 - Configures whether or not to clear PDMA_AXI_task_out_start_ch0 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn pdma_axi_task_out_start_ch0_st_clr(
        &mut self,
    ) -> PDMA_AXI_TASK_OUT_START_CH0_ST_CLR_W<TASK_ST6_CLR_SPEC> {
        PDMA_AXI_TASK_OUT_START_CH0_ST_CLR_W::new(self, 1)
    }
    #[doc = "Bit 2 - Configures whether or not to clear PDMA_AXI_task_out_start_ch1 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn pdma_axi_task_out_start_ch1_st_clr(
        &mut self,
    ) -> PDMA_AXI_TASK_OUT_START_CH1_ST_CLR_W<TASK_ST6_CLR_SPEC> {
        PDMA_AXI_TASK_OUT_START_CH1_ST_CLR_W::new(self, 2)
    }
    #[doc = "Bit 3 - Configures whether or not to clear PDMA_AXI_task_out_start_ch2 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn pdma_axi_task_out_start_ch2_st_clr(
        &mut self,
    ) -> PDMA_AXI_TASK_OUT_START_CH2_ST_CLR_W<TASK_ST6_CLR_SPEC> {
        PDMA_AXI_TASK_OUT_START_CH2_ST_CLR_W::new(self, 3)
    }
    #[doc = "Bit 4 - Configures whether or not to clear PMU_task_sleep_req trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn pmu_task_sleep_req_st_clr(&mut self) -> PMU_TASK_SLEEP_REQ_ST_CLR_W<TASK_ST6_CLR_SPEC> {
        PMU_TASK_SLEEP_REQ_ST_CLR_W::new(self, 4)
    }
    #[doc = "Bit 5 - Configures whether or not to clear DMA2D_task_in_start_ch0 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn dma2d_task_in_start_ch0_st_clr(
        &mut self,
    ) -> DMA2D_TASK_IN_START_CH0_ST_CLR_W<TASK_ST6_CLR_SPEC> {
        DMA2D_TASK_IN_START_CH0_ST_CLR_W::new(self, 5)
    }
    #[doc = "Bit 6 - Configures whether or not to clear DMA2D_task_in_start_ch1 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn dma2d_task_in_start_ch1_st_clr(
        &mut self,
    ) -> DMA2D_TASK_IN_START_CH1_ST_CLR_W<TASK_ST6_CLR_SPEC> {
        DMA2D_TASK_IN_START_CH1_ST_CLR_W::new(self, 6)
    }
    #[doc = "Bit 7 - Configures whether or not to clear DMA2D_task_in_dscr_ready_ch0 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn dma2d_task_in_dscr_ready_ch0_st_clr(
        &mut self,
    ) -> DMA2D_TASK_IN_DSCR_READY_CH0_ST_CLR_W<TASK_ST6_CLR_SPEC> {
        DMA2D_TASK_IN_DSCR_READY_CH0_ST_CLR_W::new(self, 7)
    }
    #[doc = "Bit 8 - Configures whether or not to clear DMA2D_task_in_dscr_ready_ch1 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn dma2d_task_in_dscr_ready_ch1_st_clr(
        &mut self,
    ) -> DMA2D_TASK_IN_DSCR_READY_CH1_ST_CLR_W<TASK_ST6_CLR_SPEC> {
        DMA2D_TASK_IN_DSCR_READY_CH1_ST_CLR_W::new(self, 8)
    }
    #[doc = "Bit 9 - Configures whether or not to clear DMA2D_task_out_start_ch0 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn dma2d_task_out_start_ch0_st_clr(
        &mut self,
    ) -> DMA2D_TASK_OUT_START_CH0_ST_CLR_W<TASK_ST6_CLR_SPEC> {
        DMA2D_TASK_OUT_START_CH0_ST_CLR_W::new(self, 9)
    }
    #[doc = "Bit 10 - Configures whether or not to clear DMA2D_task_out_start_ch1 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn dma2d_task_out_start_ch1_st_clr(
        &mut self,
    ) -> DMA2D_TASK_OUT_START_CH1_ST_CLR_W<TASK_ST6_CLR_SPEC> {
        DMA2D_TASK_OUT_START_CH1_ST_CLR_W::new(self, 10)
    }
    #[doc = "Bit 11 - Configures whether or not to clear DMA2D_task_out_start_ch2 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn dma2d_task_out_start_ch2_st_clr(
        &mut self,
    ) -> DMA2D_TASK_OUT_START_CH2_ST_CLR_W<TASK_ST6_CLR_SPEC> {
        DMA2D_TASK_OUT_START_CH2_ST_CLR_W::new(self, 11)
    }
    #[doc = "Bit 12 - Configures whether or not to clear DMA2D_task_out_dscr_ready_ch0 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn dma2d_task_out_dscr_ready_ch0_st_clr(
        &mut self,
    ) -> DMA2D_TASK_OUT_DSCR_READY_CH0_ST_CLR_W<TASK_ST6_CLR_SPEC> {
        DMA2D_TASK_OUT_DSCR_READY_CH0_ST_CLR_W::new(self, 12)
    }
    #[doc = "Bit 13 - Configures whether or not to clear DMA2D_task_out_dscr_ready_ch1 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn dma2d_task_out_dscr_ready_ch1_st_clr(
        &mut self,
    ) -> DMA2D_TASK_OUT_DSCR_READY_CH1_ST_CLR_W<TASK_ST6_CLR_SPEC> {
        DMA2D_TASK_OUT_DSCR_READY_CH1_ST_CLR_W::new(self, 13)
    }
    #[doc = "Bit 14 - Configures whether or not to clear DMA2D_task_out_dscr_ready_ch2 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn dma2d_task_out_dscr_ready_ch2_st_clr(
        &mut self,
    ) -> DMA2D_TASK_OUT_DSCR_READY_CH2_ST_CLR_W<TASK_ST6_CLR_SPEC> {
        DMA2D_TASK_OUT_DSCR_READY_CH2_ST_CLR_W::new(self, 14)
    }
}
#[doc = "Tasks trigger status clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`task_st6_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TASK_ST6_CLR_SPEC;
impl crate::RegisterSpec for TASK_ST6_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`task_st6_clr::W`](W) writer structure"]
impl crate::Writable for TASK_ST6_CLR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TASK_ST6_CLR to value 0"]
impl crate::Resettable for TASK_ST6_CLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
