#[doc = "Register `TASK_ST6` reader"]
pub type R = crate::R<TASK_ST6_SPEC>;
#[doc = "Register `TASK_ST6` writer"]
pub type W = crate::W<TASK_ST6_SPEC>;
#[doc = "Field `PDMA_AXI_TASK_IN_START_CH2_ST` reader - Represents PDMA_AXI_task_in_start_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PDMA_AXI_TASK_IN_START_CH2_ST_R = crate::BitReader;
#[doc = "Field `PDMA_AXI_TASK_IN_START_CH2_ST` writer - Represents PDMA_AXI_task_in_start_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PDMA_AXI_TASK_IN_START_CH2_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AXI_TASK_OUT_START_CH0_ST` reader - Represents PDMA_AXI_task_out_start_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PDMA_AXI_TASK_OUT_START_CH0_ST_R = crate::BitReader;
#[doc = "Field `PDMA_AXI_TASK_OUT_START_CH0_ST` writer - Represents PDMA_AXI_task_out_start_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PDMA_AXI_TASK_OUT_START_CH0_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AXI_TASK_OUT_START_CH1_ST` reader - Represents PDMA_AXI_task_out_start_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PDMA_AXI_TASK_OUT_START_CH1_ST_R = crate::BitReader;
#[doc = "Field `PDMA_AXI_TASK_OUT_START_CH1_ST` writer - Represents PDMA_AXI_task_out_start_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PDMA_AXI_TASK_OUT_START_CH1_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AXI_TASK_OUT_START_CH2_ST` reader - Represents PDMA_AXI_task_out_start_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PDMA_AXI_TASK_OUT_START_CH2_ST_R = crate::BitReader;
#[doc = "Field `PDMA_AXI_TASK_OUT_START_CH2_ST` writer - Represents PDMA_AXI_task_out_start_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PDMA_AXI_TASK_OUT_START_CH2_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PMU_TASK_SLEEP_REQ_ST` reader - Represents PMU_task_sleep_req trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PMU_TASK_SLEEP_REQ_ST_R = crate::BitReader;
#[doc = "Field `PMU_TASK_SLEEP_REQ_ST` writer - Represents PMU_task_sleep_req trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PMU_TASK_SLEEP_REQ_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2D_TASK_IN_START_CH0_ST` reader - Represents DMA2D_task_in_start_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type DMA2D_TASK_IN_START_CH0_ST_R = crate::BitReader;
#[doc = "Field `DMA2D_TASK_IN_START_CH0_ST` writer - Represents DMA2D_task_in_start_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type DMA2D_TASK_IN_START_CH0_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2D_TASK_IN_START_CH1_ST` reader - Represents DMA2D_task_in_start_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type DMA2D_TASK_IN_START_CH1_ST_R = crate::BitReader;
#[doc = "Field `DMA2D_TASK_IN_START_CH1_ST` writer - Represents DMA2D_task_in_start_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type DMA2D_TASK_IN_START_CH1_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2D_TASK_IN_DSCR_READY_CH0_ST` reader - Represents DMA2D_task_in_dscr_ready_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type DMA2D_TASK_IN_DSCR_READY_CH0_ST_R = crate::BitReader;
#[doc = "Field `DMA2D_TASK_IN_DSCR_READY_CH0_ST` writer - Represents DMA2D_task_in_dscr_ready_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type DMA2D_TASK_IN_DSCR_READY_CH0_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2D_TASK_IN_DSCR_READY_CH1_ST` reader - Represents DMA2D_task_in_dscr_ready_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type DMA2D_TASK_IN_DSCR_READY_CH1_ST_R = crate::BitReader;
#[doc = "Field `DMA2D_TASK_IN_DSCR_READY_CH1_ST` writer - Represents DMA2D_task_in_dscr_ready_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type DMA2D_TASK_IN_DSCR_READY_CH1_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2D_TASK_OUT_START_CH0_ST` reader - Represents DMA2D_task_out_start_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type DMA2D_TASK_OUT_START_CH0_ST_R = crate::BitReader;
#[doc = "Field `DMA2D_TASK_OUT_START_CH0_ST` writer - Represents DMA2D_task_out_start_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type DMA2D_TASK_OUT_START_CH0_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2D_TASK_OUT_START_CH1_ST` reader - Represents DMA2D_task_out_start_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type DMA2D_TASK_OUT_START_CH1_ST_R = crate::BitReader;
#[doc = "Field `DMA2D_TASK_OUT_START_CH1_ST` writer - Represents DMA2D_task_out_start_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type DMA2D_TASK_OUT_START_CH1_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2D_TASK_OUT_START_CH2_ST` reader - Represents DMA2D_task_out_start_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type DMA2D_TASK_OUT_START_CH2_ST_R = crate::BitReader;
#[doc = "Field `DMA2D_TASK_OUT_START_CH2_ST` writer - Represents DMA2D_task_out_start_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type DMA2D_TASK_OUT_START_CH2_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2D_TASK_OUT_DSCR_READY_CH0_ST` reader - Represents DMA2D_task_out_dscr_ready_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type DMA2D_TASK_OUT_DSCR_READY_CH0_ST_R = crate::BitReader;
#[doc = "Field `DMA2D_TASK_OUT_DSCR_READY_CH0_ST` writer - Represents DMA2D_task_out_dscr_ready_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type DMA2D_TASK_OUT_DSCR_READY_CH0_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2D_TASK_OUT_DSCR_READY_CH1_ST` reader - Represents DMA2D_task_out_dscr_ready_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type DMA2D_TASK_OUT_DSCR_READY_CH1_ST_R = crate::BitReader;
#[doc = "Field `DMA2D_TASK_OUT_DSCR_READY_CH1_ST` writer - Represents DMA2D_task_out_dscr_ready_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type DMA2D_TASK_OUT_DSCR_READY_CH1_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2D_TASK_OUT_DSCR_READY_CH2_ST` reader - Represents DMA2D_task_out_dscr_ready_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type DMA2D_TASK_OUT_DSCR_READY_CH2_ST_R = crate::BitReader;
#[doc = "Field `DMA2D_TASK_OUT_DSCR_READY_CH2_ST` writer - Represents DMA2D_task_out_dscr_ready_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type DMA2D_TASK_OUT_DSCR_READY_CH2_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Represents PDMA_AXI_task_in_start_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_axi_task_in_start_ch2_st(&self) -> PDMA_AXI_TASK_IN_START_CH2_ST_R {
        PDMA_AXI_TASK_IN_START_CH2_ST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Represents PDMA_AXI_task_out_start_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_axi_task_out_start_ch0_st(&self) -> PDMA_AXI_TASK_OUT_START_CH0_ST_R {
        PDMA_AXI_TASK_OUT_START_CH0_ST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Represents PDMA_AXI_task_out_start_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_axi_task_out_start_ch1_st(&self) -> PDMA_AXI_TASK_OUT_START_CH1_ST_R {
        PDMA_AXI_TASK_OUT_START_CH1_ST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Represents PDMA_AXI_task_out_start_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_axi_task_out_start_ch2_st(&self) -> PDMA_AXI_TASK_OUT_START_CH2_ST_R {
        PDMA_AXI_TASK_OUT_START_CH2_ST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Represents PMU_task_sleep_req trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pmu_task_sleep_req_st(&self) -> PMU_TASK_SLEEP_REQ_ST_R {
        PMU_TASK_SLEEP_REQ_ST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Represents DMA2D_task_in_start_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn dma2d_task_in_start_ch0_st(&self) -> DMA2D_TASK_IN_START_CH0_ST_R {
        DMA2D_TASK_IN_START_CH0_ST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Represents DMA2D_task_in_start_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn dma2d_task_in_start_ch1_st(&self) -> DMA2D_TASK_IN_START_CH1_ST_R {
        DMA2D_TASK_IN_START_CH1_ST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Represents DMA2D_task_in_dscr_ready_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn dma2d_task_in_dscr_ready_ch0_st(&self) -> DMA2D_TASK_IN_DSCR_READY_CH0_ST_R {
        DMA2D_TASK_IN_DSCR_READY_CH0_ST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Represents DMA2D_task_in_dscr_ready_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn dma2d_task_in_dscr_ready_ch1_st(&self) -> DMA2D_TASK_IN_DSCR_READY_CH1_ST_R {
        DMA2D_TASK_IN_DSCR_READY_CH1_ST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Represents DMA2D_task_out_start_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn dma2d_task_out_start_ch0_st(&self) -> DMA2D_TASK_OUT_START_CH0_ST_R {
        DMA2D_TASK_OUT_START_CH0_ST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Represents DMA2D_task_out_start_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn dma2d_task_out_start_ch1_st(&self) -> DMA2D_TASK_OUT_START_CH1_ST_R {
        DMA2D_TASK_OUT_START_CH1_ST_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Represents DMA2D_task_out_start_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn dma2d_task_out_start_ch2_st(&self) -> DMA2D_TASK_OUT_START_CH2_ST_R {
        DMA2D_TASK_OUT_START_CH2_ST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Represents DMA2D_task_out_dscr_ready_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn dma2d_task_out_dscr_ready_ch0_st(&self) -> DMA2D_TASK_OUT_DSCR_READY_CH0_ST_R {
        DMA2D_TASK_OUT_DSCR_READY_CH0_ST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Represents DMA2D_task_out_dscr_ready_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn dma2d_task_out_dscr_ready_ch1_st(&self) -> DMA2D_TASK_OUT_DSCR_READY_CH1_ST_R {
        DMA2D_TASK_OUT_DSCR_READY_CH1_ST_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Represents DMA2D_task_out_dscr_ready_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn dma2d_task_out_dscr_ready_ch2_st(&self) -> DMA2D_TASK_OUT_DSCR_READY_CH2_ST_R {
        DMA2D_TASK_OUT_DSCR_READY_CH2_ST_R::new(((self.bits >> 14) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TASK_ST6")
            .field(
                "pdma_axi_task_in_start_ch2_st",
                &self.pdma_axi_task_in_start_ch2_st(),
            )
            .field(
                "pdma_axi_task_out_start_ch0_st",
                &self.pdma_axi_task_out_start_ch0_st(),
            )
            .field(
                "pdma_axi_task_out_start_ch1_st",
                &self.pdma_axi_task_out_start_ch1_st(),
            )
            .field(
                "pdma_axi_task_out_start_ch2_st",
                &self.pdma_axi_task_out_start_ch2_st(),
            )
            .field("pmu_task_sleep_req_st", &self.pmu_task_sleep_req_st())
            .field(
                "dma2d_task_in_start_ch0_st",
                &self.dma2d_task_in_start_ch0_st(),
            )
            .field(
                "dma2d_task_in_start_ch1_st",
                &self.dma2d_task_in_start_ch1_st(),
            )
            .field(
                "dma2d_task_in_dscr_ready_ch0_st",
                &self.dma2d_task_in_dscr_ready_ch0_st(),
            )
            .field(
                "dma2d_task_in_dscr_ready_ch1_st",
                &self.dma2d_task_in_dscr_ready_ch1_st(),
            )
            .field(
                "dma2d_task_out_start_ch0_st",
                &self.dma2d_task_out_start_ch0_st(),
            )
            .field(
                "dma2d_task_out_start_ch1_st",
                &self.dma2d_task_out_start_ch1_st(),
            )
            .field(
                "dma2d_task_out_start_ch2_st",
                &self.dma2d_task_out_start_ch2_st(),
            )
            .field(
                "dma2d_task_out_dscr_ready_ch0_st",
                &self.dma2d_task_out_dscr_ready_ch0_st(),
            )
            .field(
                "dma2d_task_out_dscr_ready_ch1_st",
                &self.dma2d_task_out_dscr_ready_ch1_st(),
            )
            .field(
                "dma2d_task_out_dscr_ready_ch2_st",
                &self.dma2d_task_out_dscr_ready_ch2_st(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Represents PDMA_AXI_task_in_start_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    #[must_use]
    pub fn pdma_axi_task_in_start_ch2_st(
        &mut self,
    ) -> PDMA_AXI_TASK_IN_START_CH2_ST_W<TASK_ST6_SPEC> {
        PDMA_AXI_TASK_IN_START_CH2_ST_W::new(self, 0)
    }
    #[doc = "Bit 1 - Represents PDMA_AXI_task_out_start_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    #[must_use]
    pub fn pdma_axi_task_out_start_ch0_st(
        &mut self,
    ) -> PDMA_AXI_TASK_OUT_START_CH0_ST_W<TASK_ST6_SPEC> {
        PDMA_AXI_TASK_OUT_START_CH0_ST_W::new(self, 1)
    }
    #[doc = "Bit 2 - Represents PDMA_AXI_task_out_start_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    #[must_use]
    pub fn pdma_axi_task_out_start_ch1_st(
        &mut self,
    ) -> PDMA_AXI_TASK_OUT_START_CH1_ST_W<TASK_ST6_SPEC> {
        PDMA_AXI_TASK_OUT_START_CH1_ST_W::new(self, 2)
    }
    #[doc = "Bit 3 - Represents PDMA_AXI_task_out_start_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    #[must_use]
    pub fn pdma_axi_task_out_start_ch2_st(
        &mut self,
    ) -> PDMA_AXI_TASK_OUT_START_CH2_ST_W<TASK_ST6_SPEC> {
        PDMA_AXI_TASK_OUT_START_CH2_ST_W::new(self, 3)
    }
    #[doc = "Bit 4 - Represents PMU_task_sleep_req trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    #[must_use]
    pub fn pmu_task_sleep_req_st(&mut self) -> PMU_TASK_SLEEP_REQ_ST_W<TASK_ST6_SPEC> {
        PMU_TASK_SLEEP_REQ_ST_W::new(self, 4)
    }
    #[doc = "Bit 5 - Represents DMA2D_task_in_start_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    #[must_use]
    pub fn dma2d_task_in_start_ch0_st(&mut self) -> DMA2D_TASK_IN_START_CH0_ST_W<TASK_ST6_SPEC> {
        DMA2D_TASK_IN_START_CH0_ST_W::new(self, 5)
    }
    #[doc = "Bit 6 - Represents DMA2D_task_in_start_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    #[must_use]
    pub fn dma2d_task_in_start_ch1_st(&mut self) -> DMA2D_TASK_IN_START_CH1_ST_W<TASK_ST6_SPEC> {
        DMA2D_TASK_IN_START_CH1_ST_W::new(self, 6)
    }
    #[doc = "Bit 7 - Represents DMA2D_task_in_dscr_ready_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    #[must_use]
    pub fn dma2d_task_in_dscr_ready_ch0_st(
        &mut self,
    ) -> DMA2D_TASK_IN_DSCR_READY_CH0_ST_W<TASK_ST6_SPEC> {
        DMA2D_TASK_IN_DSCR_READY_CH0_ST_W::new(self, 7)
    }
    #[doc = "Bit 8 - Represents DMA2D_task_in_dscr_ready_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    #[must_use]
    pub fn dma2d_task_in_dscr_ready_ch1_st(
        &mut self,
    ) -> DMA2D_TASK_IN_DSCR_READY_CH1_ST_W<TASK_ST6_SPEC> {
        DMA2D_TASK_IN_DSCR_READY_CH1_ST_W::new(self, 8)
    }
    #[doc = "Bit 9 - Represents DMA2D_task_out_start_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    #[must_use]
    pub fn dma2d_task_out_start_ch0_st(&mut self) -> DMA2D_TASK_OUT_START_CH0_ST_W<TASK_ST6_SPEC> {
        DMA2D_TASK_OUT_START_CH0_ST_W::new(self, 9)
    }
    #[doc = "Bit 10 - Represents DMA2D_task_out_start_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    #[must_use]
    pub fn dma2d_task_out_start_ch1_st(&mut self) -> DMA2D_TASK_OUT_START_CH1_ST_W<TASK_ST6_SPEC> {
        DMA2D_TASK_OUT_START_CH1_ST_W::new(self, 10)
    }
    #[doc = "Bit 11 - Represents DMA2D_task_out_start_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    #[must_use]
    pub fn dma2d_task_out_start_ch2_st(&mut self) -> DMA2D_TASK_OUT_START_CH2_ST_W<TASK_ST6_SPEC> {
        DMA2D_TASK_OUT_START_CH2_ST_W::new(self, 11)
    }
    #[doc = "Bit 12 - Represents DMA2D_task_out_dscr_ready_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    #[must_use]
    pub fn dma2d_task_out_dscr_ready_ch0_st(
        &mut self,
    ) -> DMA2D_TASK_OUT_DSCR_READY_CH0_ST_W<TASK_ST6_SPEC> {
        DMA2D_TASK_OUT_DSCR_READY_CH0_ST_W::new(self, 12)
    }
    #[doc = "Bit 13 - Represents DMA2D_task_out_dscr_ready_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    #[must_use]
    pub fn dma2d_task_out_dscr_ready_ch1_st(
        &mut self,
    ) -> DMA2D_TASK_OUT_DSCR_READY_CH1_ST_W<TASK_ST6_SPEC> {
        DMA2D_TASK_OUT_DSCR_READY_CH1_ST_W::new(self, 13)
    }
    #[doc = "Bit 14 - Represents DMA2D_task_out_dscr_ready_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    #[must_use]
    pub fn dma2d_task_out_dscr_ready_ch2_st(
        &mut self,
    ) -> DMA2D_TASK_OUT_DSCR_READY_CH2_ST_W<TASK_ST6_SPEC> {
        DMA2D_TASK_OUT_DSCR_READY_CH2_ST_W::new(self, 14)
    }
}
#[doc = "Tasks trigger status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`task_st6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`task_st6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TASK_ST6_SPEC;
impl crate::RegisterSpec for TASK_ST6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`task_st6::R`](R) reader structure"]
impl crate::Readable for TASK_ST6_SPEC {}
#[doc = "`write(|w| ..)` method takes [`task_st6::W`](W) writer structure"]
impl crate::Writable for TASK_ST6_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TASK_ST6 to value 0"]
impl crate::Resettable for TASK_ST6_SPEC {
    const RESET_VALUE: u32 = 0;
}
