///Register `TASK_ST6_CLR` writer
pub type W = crate::W<TASK_ST6_CLR_SPEC>;
///Field `PDMA_AXI_TASK_IN_START_CH2_ST_CLR` writer - Configures whether or not to clear PDMA_AXI_task_in_start_ch2 trigger status.\\0: Invalid, No effect\\1: Clear
pub type PDMA_AXI_TASK_IN_START_CH2_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PDMA_AXI_TASK_OUT_START_CH0_ST_CLR` writer - Configures whether or not to clear PDMA_AXI_task_out_start_ch0 trigger status.\\0: Invalid, No effect\\1: Clear
pub type PDMA_AXI_TASK_OUT_START_CH0_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PDMA_AXI_TASK_OUT_START_CH1_ST_CLR` writer - Configures whether or not to clear PDMA_AXI_task_out_start_ch1 trigger status.\\0: Invalid, No effect\\1: Clear
pub type PDMA_AXI_TASK_OUT_START_CH1_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PDMA_AXI_TASK_OUT_START_CH2_ST_CLR` writer - Configures whether or not to clear PDMA_AXI_task_out_start_ch2 trigger status.\\0: Invalid, No effect\\1: Clear
pub type PDMA_AXI_TASK_OUT_START_CH2_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PMU_TASK_SLEEP_REQ_ST_CLR` writer - Configures whether or not to clear PMU_task_sleep_req trigger status.\\0: Invalid, No effect\\1: Clear
pub type PMU_TASK_SLEEP_REQ_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMA2D_TASK_IN_START_CH0_ST_CLR` writer - Configures whether or not to clear DMA2D_task_in_start_ch0 trigger status.\\0: Invalid, No effect\\1: Clear
pub type DMA2D_TASK_IN_START_CH0_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMA2D_TASK_IN_START_CH1_ST_CLR` writer - Configures whether or not to clear DMA2D_task_in_start_ch1 trigger status.\\0: Invalid, No effect\\1: Clear
pub type DMA2D_TASK_IN_START_CH1_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMA2D_TASK_IN_DSCR_READY_CH0_ST_CLR` writer - Configures whether or not to clear DMA2D_task_in_dscr_ready_ch0 trigger status.\\0: Invalid, No effect\\1: Clear
pub type DMA2D_TASK_IN_DSCR_READY_CH0_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMA2D_TASK_IN_DSCR_READY_CH1_ST_CLR` writer - Configures whether or not to clear DMA2D_task_in_dscr_ready_ch1 trigger status.\\0: Invalid, No effect\\1: Clear
pub type DMA2D_TASK_IN_DSCR_READY_CH1_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMA2D_TASK_OUT_START_CH0_ST_CLR` writer - Configures whether or not to clear DMA2D_task_out_start_ch0 trigger status.\\0: Invalid, No effect\\1: Clear
pub type DMA2D_TASK_OUT_START_CH0_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMA2D_TASK_OUT_START_CH1_ST_CLR` writer - Configures whether or not to clear DMA2D_task_out_start_ch1 trigger status.\\0: Invalid, No effect\\1: Clear
pub type DMA2D_TASK_OUT_START_CH1_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMA2D_TASK_OUT_START_CH2_ST_CLR` writer - Configures whether or not to clear DMA2D_task_out_start_ch2 trigger status.\\0: Invalid, No effect\\1: Clear
pub type DMA2D_TASK_OUT_START_CH2_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMA2D_TASK_OUT_DSCR_READY_CH0_ST_CLR` writer - Configures whether or not to clear DMA2D_task_out_dscr_ready_ch0 trigger status.\\0: Invalid, No effect\\1: Clear
pub type DMA2D_TASK_OUT_DSCR_READY_CH0_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMA2D_TASK_OUT_DSCR_READY_CH1_ST_CLR` writer - Configures whether or not to clear DMA2D_task_out_dscr_ready_ch1 trigger status.\\0: Invalid, No effect\\1: Clear
pub type DMA2D_TASK_OUT_DSCR_READY_CH1_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMA2D_TASK_OUT_DSCR_READY_CH2_ST_CLR` writer - Configures whether or not to clear DMA2D_task_out_dscr_ready_ch2 trigger status.\\0: Invalid, No effect\\1: Clear
pub type DMA2D_TASK_OUT_DSCR_READY_CH2_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TASK_ST6_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Configures whether or not to clear PDMA_AXI_task_in_start_ch2 trigger status.\\0: Invalid, No effect\\1: Clear
    #[inline(always)]
    #[must_use]
    pub fn pdma_axi_task_in_start_ch2_st_clr(
        &mut self,
    ) -> PDMA_AXI_TASK_IN_START_CH2_ST_CLR_W<TASK_ST6_CLR_SPEC> {
        PDMA_AXI_TASK_IN_START_CH2_ST_CLR_W::new(self, 0)
    }
    ///Bit 1 - Configures whether or not to clear PDMA_AXI_task_out_start_ch0 trigger status.\\0: Invalid, No effect\\1: Clear
    #[inline(always)]
    #[must_use]
    pub fn pdma_axi_task_out_start_ch0_st_clr(
        &mut self,
    ) -> PDMA_AXI_TASK_OUT_START_CH0_ST_CLR_W<TASK_ST6_CLR_SPEC> {
        PDMA_AXI_TASK_OUT_START_CH0_ST_CLR_W::new(self, 1)
    }
    ///Bit 2 - Configures whether or not to clear PDMA_AXI_task_out_start_ch1 trigger status.\\0: Invalid, No effect\\1: Clear
    #[inline(always)]
    #[must_use]
    pub fn pdma_axi_task_out_start_ch1_st_clr(
        &mut self,
    ) -> PDMA_AXI_TASK_OUT_START_CH1_ST_CLR_W<TASK_ST6_CLR_SPEC> {
        PDMA_AXI_TASK_OUT_START_CH1_ST_CLR_W::new(self, 2)
    }
    ///Bit 3 - Configures whether or not to clear PDMA_AXI_task_out_start_ch2 trigger status.\\0: Invalid, No effect\\1: Clear
    #[inline(always)]
    #[must_use]
    pub fn pdma_axi_task_out_start_ch2_st_clr(
        &mut self,
    ) -> PDMA_AXI_TASK_OUT_START_CH2_ST_CLR_W<TASK_ST6_CLR_SPEC> {
        PDMA_AXI_TASK_OUT_START_CH2_ST_CLR_W::new(self, 3)
    }
    ///Bit 4 - Configures whether or not to clear PMU_task_sleep_req trigger status.\\0: Invalid, No effect\\1: Clear
    #[inline(always)]
    #[must_use]
    pub fn pmu_task_sleep_req_st_clr(&mut self) -> PMU_TASK_SLEEP_REQ_ST_CLR_W<TASK_ST6_CLR_SPEC> {
        PMU_TASK_SLEEP_REQ_ST_CLR_W::new(self, 4)
    }
    ///Bit 5 - Configures whether or not to clear DMA2D_task_in_start_ch0 trigger status.\\0: Invalid, No effect\\1: Clear
    #[inline(always)]
    #[must_use]
    pub fn dma2d_task_in_start_ch0_st_clr(
        &mut self,
    ) -> DMA2D_TASK_IN_START_CH0_ST_CLR_W<TASK_ST6_CLR_SPEC> {
        DMA2D_TASK_IN_START_CH0_ST_CLR_W::new(self, 5)
    }
    ///Bit 6 - Configures whether or not to clear DMA2D_task_in_start_ch1 trigger status.\\0: Invalid, No effect\\1: Clear
    #[inline(always)]
    #[must_use]
    pub fn dma2d_task_in_start_ch1_st_clr(
        &mut self,
    ) -> DMA2D_TASK_IN_START_CH1_ST_CLR_W<TASK_ST6_CLR_SPEC> {
        DMA2D_TASK_IN_START_CH1_ST_CLR_W::new(self, 6)
    }
    ///Bit 7 - Configures whether or not to clear DMA2D_task_in_dscr_ready_ch0 trigger status.\\0: Invalid, No effect\\1: Clear
    #[inline(always)]
    #[must_use]
    pub fn dma2d_task_in_dscr_ready_ch0_st_clr(
        &mut self,
    ) -> DMA2D_TASK_IN_DSCR_READY_CH0_ST_CLR_W<TASK_ST6_CLR_SPEC> {
        DMA2D_TASK_IN_DSCR_READY_CH0_ST_CLR_W::new(self, 7)
    }
    ///Bit 8 - Configures whether or not to clear DMA2D_task_in_dscr_ready_ch1 trigger status.\\0: Invalid, No effect\\1: Clear
    #[inline(always)]
    #[must_use]
    pub fn dma2d_task_in_dscr_ready_ch1_st_clr(
        &mut self,
    ) -> DMA2D_TASK_IN_DSCR_READY_CH1_ST_CLR_W<TASK_ST6_CLR_SPEC> {
        DMA2D_TASK_IN_DSCR_READY_CH1_ST_CLR_W::new(self, 8)
    }
    ///Bit 9 - Configures whether or not to clear DMA2D_task_out_start_ch0 trigger status.\\0: Invalid, No effect\\1: Clear
    #[inline(always)]
    #[must_use]
    pub fn dma2d_task_out_start_ch0_st_clr(
        &mut self,
    ) -> DMA2D_TASK_OUT_START_CH0_ST_CLR_W<TASK_ST6_CLR_SPEC> {
        DMA2D_TASK_OUT_START_CH0_ST_CLR_W::new(self, 9)
    }
    ///Bit 10 - Configures whether or not to clear DMA2D_task_out_start_ch1 trigger status.\\0: Invalid, No effect\\1: Clear
    #[inline(always)]
    #[must_use]
    pub fn dma2d_task_out_start_ch1_st_clr(
        &mut self,
    ) -> DMA2D_TASK_OUT_START_CH1_ST_CLR_W<TASK_ST6_CLR_SPEC> {
        DMA2D_TASK_OUT_START_CH1_ST_CLR_W::new(self, 10)
    }
    ///Bit 11 - Configures whether or not to clear DMA2D_task_out_start_ch2 trigger status.\\0: Invalid, No effect\\1: Clear
    #[inline(always)]
    #[must_use]
    pub fn dma2d_task_out_start_ch2_st_clr(
        &mut self,
    ) -> DMA2D_TASK_OUT_START_CH2_ST_CLR_W<TASK_ST6_CLR_SPEC> {
        DMA2D_TASK_OUT_START_CH2_ST_CLR_W::new(self, 11)
    }
    ///Bit 12 - Configures whether or not to clear DMA2D_task_out_dscr_ready_ch0 trigger status.\\0: Invalid, No effect\\1: Clear
    #[inline(always)]
    #[must_use]
    pub fn dma2d_task_out_dscr_ready_ch0_st_clr(
        &mut self,
    ) -> DMA2D_TASK_OUT_DSCR_READY_CH0_ST_CLR_W<TASK_ST6_CLR_SPEC> {
        DMA2D_TASK_OUT_DSCR_READY_CH0_ST_CLR_W::new(self, 12)
    }
    ///Bit 13 - Configures whether or not to clear DMA2D_task_out_dscr_ready_ch1 trigger status.\\0: Invalid, No effect\\1: Clear
    #[inline(always)]
    #[must_use]
    pub fn dma2d_task_out_dscr_ready_ch1_st_clr(
        &mut self,
    ) -> DMA2D_TASK_OUT_DSCR_READY_CH1_ST_CLR_W<TASK_ST6_CLR_SPEC> {
        DMA2D_TASK_OUT_DSCR_READY_CH1_ST_CLR_W::new(self, 13)
    }
    ///Bit 14 - Configures whether or not to clear DMA2D_task_out_dscr_ready_ch2 trigger status.\\0: Invalid, No effect\\1: Clear
    #[inline(always)]
    #[must_use]
    pub fn dma2d_task_out_dscr_ready_ch2_st_clr(
        &mut self,
    ) -> DMA2D_TASK_OUT_DSCR_READY_CH2_ST_CLR_W<TASK_ST6_CLR_SPEC> {
        DMA2D_TASK_OUT_DSCR_READY_CH2_ST_CLR_W::new(self, 14)
    }
}
/**Tasks trigger status clear register

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`task_st6_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TASK_ST6_CLR_SPEC;
impl crate::RegisterSpec for TASK_ST6_CLR_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`task_st6_clr::W`](W) writer structure
impl crate::Writable for TASK_ST6_CLR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TASK_ST6_CLR to value 0
impl crate::Resettable for TASK_ST6_CLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
