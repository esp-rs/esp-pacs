///Register `EVT_ST6` reader
pub type R = crate::R<EVT_ST6_SPEC>;
///Register `EVT_ST6` writer
pub type W = crate::W<EVT_ST6_SPEC>;
///Field `PDMA_AHB_EVT_OUT_FIFO_FULL_CH2_ST` reader - Represents PDMA_AHB_evt_out_fifo_full_ch2 trigger status.\\0: Not triggered\\1: Triggered
pub type PDMA_AHB_EVT_OUT_FIFO_FULL_CH2_ST_R = crate::BitReader;
///Field `PDMA_AHB_EVT_OUT_FIFO_FULL_CH2_ST` writer - Represents PDMA_AHB_evt_out_fifo_full_ch2 trigger status.\\0: Not triggered\\1: Triggered
pub type PDMA_AHB_EVT_OUT_FIFO_FULL_CH2_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PDMA_AXI_EVT_IN_DONE_CH0_ST` reader - Represents PDMA_AXI_evt_in_done_ch0 trigger status.\\0: Not triggered\\1: Triggered
pub type PDMA_AXI_EVT_IN_DONE_CH0_ST_R = crate::BitReader;
///Field `PDMA_AXI_EVT_IN_DONE_CH0_ST` writer - Represents PDMA_AXI_evt_in_done_ch0 trigger status.\\0: Not triggered\\1: Triggered
pub type PDMA_AXI_EVT_IN_DONE_CH0_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PDMA_AXI_EVT_IN_DONE_CH1_ST` reader - Represents PDMA_AXI_evt_in_done_ch1 trigger status.\\0: Not triggered\\1: Triggered
pub type PDMA_AXI_EVT_IN_DONE_CH1_ST_R = crate::BitReader;
///Field `PDMA_AXI_EVT_IN_DONE_CH1_ST` writer - Represents PDMA_AXI_evt_in_done_ch1 trigger status.\\0: Not triggered\\1: Triggered
pub type PDMA_AXI_EVT_IN_DONE_CH1_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PDMA_AXI_EVT_IN_DONE_CH2_ST` reader - Represents PDMA_AXI_evt_in_done_ch2 trigger status.\\0: Not triggered\\1: Triggered
pub type PDMA_AXI_EVT_IN_DONE_CH2_ST_R = crate::BitReader;
///Field `PDMA_AXI_EVT_IN_DONE_CH2_ST` writer - Represents PDMA_AXI_evt_in_done_ch2 trigger status.\\0: Not triggered\\1: Triggered
pub type PDMA_AXI_EVT_IN_DONE_CH2_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PDMA_AXI_EVT_IN_SUC_EOF_CH0_ST` reader - Represents PDMA_AXI_evt_in_suc_eof_ch0 trigger status.\\0: Not triggered\\1: Triggered
pub type PDMA_AXI_EVT_IN_SUC_EOF_CH0_ST_R = crate::BitReader;
///Field `PDMA_AXI_EVT_IN_SUC_EOF_CH0_ST` writer - Represents PDMA_AXI_evt_in_suc_eof_ch0 trigger status.\\0: Not triggered\\1: Triggered
pub type PDMA_AXI_EVT_IN_SUC_EOF_CH0_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PDMA_AXI_EVT_IN_SUC_EOF_CH1_ST` reader - Represents PDMA_AXI_evt_in_suc_eof_ch1 trigger status.\\0: Not triggered\\1: Triggered
pub type PDMA_AXI_EVT_IN_SUC_EOF_CH1_ST_R = crate::BitReader;
///Field `PDMA_AXI_EVT_IN_SUC_EOF_CH1_ST` writer - Represents PDMA_AXI_evt_in_suc_eof_ch1 trigger status.\\0: Not triggered\\1: Triggered
pub type PDMA_AXI_EVT_IN_SUC_EOF_CH1_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PDMA_AXI_EVT_IN_SUC_EOF_CH2_ST` reader - Represents PDMA_AXI_evt_in_suc_eof_ch2 trigger status.\\0: Not triggered\\1: Triggered
pub type PDMA_AXI_EVT_IN_SUC_EOF_CH2_ST_R = crate::BitReader;
///Field `PDMA_AXI_EVT_IN_SUC_EOF_CH2_ST` writer - Represents PDMA_AXI_evt_in_suc_eof_ch2 trigger status.\\0: Not triggered\\1: Triggered
pub type PDMA_AXI_EVT_IN_SUC_EOF_CH2_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PDMA_AXI_EVT_IN_FIFO_EMPTY_CH0_ST` reader - Represents PDMA_AXI_evt_in_fifo_empty_ch0 trigger status.\\0: Not triggered\\1: Triggered
pub type PDMA_AXI_EVT_IN_FIFO_EMPTY_CH0_ST_R = crate::BitReader;
///Field `PDMA_AXI_EVT_IN_FIFO_EMPTY_CH0_ST` writer - Represents PDMA_AXI_evt_in_fifo_empty_ch0 trigger status.\\0: Not triggered\\1: Triggered
pub type PDMA_AXI_EVT_IN_FIFO_EMPTY_CH0_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PDMA_AXI_EVT_IN_FIFO_EMPTY_CH1_ST` reader - Represents PDMA_AXI_evt_in_fifo_empty_ch1 trigger status.\\0: Not triggered\\1: Triggered
pub type PDMA_AXI_EVT_IN_FIFO_EMPTY_CH1_ST_R = crate::BitReader;
///Field `PDMA_AXI_EVT_IN_FIFO_EMPTY_CH1_ST` writer - Represents PDMA_AXI_evt_in_fifo_empty_ch1 trigger status.\\0: Not triggered\\1: Triggered
pub type PDMA_AXI_EVT_IN_FIFO_EMPTY_CH1_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PDMA_AXI_EVT_IN_FIFO_EMPTY_CH2_ST` reader - Represents PDMA_AXI_evt_in_fifo_empty_ch2 trigger status.\\0: Not triggered\\1: Triggered
pub type PDMA_AXI_EVT_IN_FIFO_EMPTY_CH2_ST_R = crate::BitReader;
///Field `PDMA_AXI_EVT_IN_FIFO_EMPTY_CH2_ST` writer - Represents PDMA_AXI_evt_in_fifo_empty_ch2 trigger status.\\0: Not triggered\\1: Triggered
pub type PDMA_AXI_EVT_IN_FIFO_EMPTY_CH2_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PDMA_AXI_EVT_IN_FIFO_FULL_CH0_ST` reader - Represents PDMA_AXI_evt_in_fifo_full_ch0 trigger status.\\0: Not triggered\\1: Triggered
pub type PDMA_AXI_EVT_IN_FIFO_FULL_CH0_ST_R = crate::BitReader;
///Field `PDMA_AXI_EVT_IN_FIFO_FULL_CH0_ST` writer - Represents PDMA_AXI_evt_in_fifo_full_ch0 trigger status.\\0: Not triggered\\1: Triggered
pub type PDMA_AXI_EVT_IN_FIFO_FULL_CH0_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PDMA_AXI_EVT_IN_FIFO_FULL_CH1_ST` reader - Represents PDMA_AXI_evt_in_fifo_full_ch1 trigger status.\\0: Not triggered\\1: Triggered
pub type PDMA_AXI_EVT_IN_FIFO_FULL_CH1_ST_R = crate::BitReader;
///Field `PDMA_AXI_EVT_IN_FIFO_FULL_CH1_ST` writer - Represents PDMA_AXI_evt_in_fifo_full_ch1 trigger status.\\0: Not triggered\\1: Triggered
pub type PDMA_AXI_EVT_IN_FIFO_FULL_CH1_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PDMA_AXI_EVT_IN_FIFO_FULL_CH2_ST` reader - Represents PDMA_AXI_evt_in_fifo_full_ch2 trigger status.\\0: Not triggered\\1: Triggered
pub type PDMA_AXI_EVT_IN_FIFO_FULL_CH2_ST_R = crate::BitReader;
///Field `PDMA_AXI_EVT_IN_FIFO_FULL_CH2_ST` writer - Represents PDMA_AXI_evt_in_fifo_full_ch2 trigger status.\\0: Not triggered\\1: Triggered
pub type PDMA_AXI_EVT_IN_FIFO_FULL_CH2_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PDMA_AXI_EVT_OUT_DONE_CH0_ST` reader - Represents PDMA_AXI_evt_out_done_ch0 trigger status.\\0: Not triggered\\1: Triggered
pub type PDMA_AXI_EVT_OUT_DONE_CH0_ST_R = crate::BitReader;
///Field `PDMA_AXI_EVT_OUT_DONE_CH0_ST` writer - Represents PDMA_AXI_evt_out_done_ch0 trigger status.\\0: Not triggered\\1: Triggered
pub type PDMA_AXI_EVT_OUT_DONE_CH0_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PDMA_AXI_EVT_OUT_DONE_CH1_ST` reader - Represents PDMA_AXI_evt_out_done_ch1 trigger status.\\0: Not triggered\\1: Triggered
pub type PDMA_AXI_EVT_OUT_DONE_CH1_ST_R = crate::BitReader;
///Field `PDMA_AXI_EVT_OUT_DONE_CH1_ST` writer - Represents PDMA_AXI_evt_out_done_ch1 trigger status.\\0: Not triggered\\1: Triggered
pub type PDMA_AXI_EVT_OUT_DONE_CH1_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PDMA_AXI_EVT_OUT_DONE_CH2_ST` reader - Represents PDMA_AXI_evt_out_done_ch2 trigger status.\\0: Not triggered\\1: Triggered
pub type PDMA_AXI_EVT_OUT_DONE_CH2_ST_R = crate::BitReader;
///Field `PDMA_AXI_EVT_OUT_DONE_CH2_ST` writer - Represents PDMA_AXI_evt_out_done_ch2 trigger status.\\0: Not triggered\\1: Triggered
pub type PDMA_AXI_EVT_OUT_DONE_CH2_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PDMA_AXI_EVT_OUT_EOF_CH0_ST` reader - Represents PDMA_AXI_evt_out_eof_ch0 trigger status.\\0: Not triggered\\1: Triggered
pub type PDMA_AXI_EVT_OUT_EOF_CH0_ST_R = crate::BitReader;
///Field `PDMA_AXI_EVT_OUT_EOF_CH0_ST` writer - Represents PDMA_AXI_evt_out_eof_ch0 trigger status.\\0: Not triggered\\1: Triggered
pub type PDMA_AXI_EVT_OUT_EOF_CH0_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PDMA_AXI_EVT_OUT_EOF_CH1_ST` reader - Represents PDMA_AXI_evt_out_eof_ch1 trigger status.\\0: Not triggered\\1: Triggered
pub type PDMA_AXI_EVT_OUT_EOF_CH1_ST_R = crate::BitReader;
///Field `PDMA_AXI_EVT_OUT_EOF_CH1_ST` writer - Represents PDMA_AXI_evt_out_eof_ch1 trigger status.\\0: Not triggered\\1: Triggered
pub type PDMA_AXI_EVT_OUT_EOF_CH1_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PDMA_AXI_EVT_OUT_EOF_CH2_ST` reader - Represents PDMA_AXI_evt_out_eof_ch2 trigger status.\\0: Not triggered\\1: Triggered
pub type PDMA_AXI_EVT_OUT_EOF_CH2_ST_R = crate::BitReader;
///Field `PDMA_AXI_EVT_OUT_EOF_CH2_ST` writer - Represents PDMA_AXI_evt_out_eof_ch2 trigger status.\\0: Not triggered\\1: Triggered
pub type PDMA_AXI_EVT_OUT_EOF_CH2_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PDMA_AXI_EVT_OUT_TOTAL_EOF_CH0_ST` reader - Represents PDMA_AXI_evt_out_total_eof_ch0 trigger status.\\0: Not triggered\\1: Triggered
pub type PDMA_AXI_EVT_OUT_TOTAL_EOF_CH0_ST_R = crate::BitReader;
///Field `PDMA_AXI_EVT_OUT_TOTAL_EOF_CH0_ST` writer - Represents PDMA_AXI_evt_out_total_eof_ch0 trigger status.\\0: Not triggered\\1: Triggered
pub type PDMA_AXI_EVT_OUT_TOTAL_EOF_CH0_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PDMA_AXI_EVT_OUT_TOTAL_EOF_CH1_ST` reader - Represents PDMA_AXI_evt_out_total_eof_ch1 trigger status.\\0: Not triggered\\1: Triggered
pub type PDMA_AXI_EVT_OUT_TOTAL_EOF_CH1_ST_R = crate::BitReader;
///Field `PDMA_AXI_EVT_OUT_TOTAL_EOF_CH1_ST` writer - Represents PDMA_AXI_evt_out_total_eof_ch1 trigger status.\\0: Not triggered\\1: Triggered
pub type PDMA_AXI_EVT_OUT_TOTAL_EOF_CH1_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PDMA_AXI_EVT_OUT_TOTAL_EOF_CH2_ST` reader - Represents PDMA_AXI_evt_out_total_eof_ch2 trigger status.\\0: Not triggered\\1: Triggered
pub type PDMA_AXI_EVT_OUT_TOTAL_EOF_CH2_ST_R = crate::BitReader;
///Field `PDMA_AXI_EVT_OUT_TOTAL_EOF_CH2_ST` writer - Represents PDMA_AXI_evt_out_total_eof_ch2 trigger status.\\0: Not triggered\\1: Triggered
pub type PDMA_AXI_EVT_OUT_TOTAL_EOF_CH2_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PDMA_AXI_EVT_OUT_FIFO_EMPTY_CH0_ST` reader - Represents PDMA_AXI_evt_out_fifo_empty_ch0 trigger status.\\0: Not triggered\\1: Triggered
pub type PDMA_AXI_EVT_OUT_FIFO_EMPTY_CH0_ST_R = crate::BitReader;
///Field `PDMA_AXI_EVT_OUT_FIFO_EMPTY_CH0_ST` writer - Represents PDMA_AXI_evt_out_fifo_empty_ch0 trigger status.\\0: Not triggered\\1: Triggered
pub type PDMA_AXI_EVT_OUT_FIFO_EMPTY_CH0_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PDMA_AXI_EVT_OUT_FIFO_EMPTY_CH1_ST` reader - Represents PDMA_AXI_evt_out_fifo_empty_ch1 trigger status.\\0: Not triggered\\1: Triggered
pub type PDMA_AXI_EVT_OUT_FIFO_EMPTY_CH1_ST_R = crate::BitReader;
///Field `PDMA_AXI_EVT_OUT_FIFO_EMPTY_CH1_ST` writer - Represents PDMA_AXI_evt_out_fifo_empty_ch1 trigger status.\\0: Not triggered\\1: Triggered
pub type PDMA_AXI_EVT_OUT_FIFO_EMPTY_CH1_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PDMA_AXI_EVT_OUT_FIFO_EMPTY_CH2_ST` reader - Represents PDMA_AXI_evt_out_fifo_empty_ch2 trigger status.\\0: Not triggered\\1: Triggered
pub type PDMA_AXI_EVT_OUT_FIFO_EMPTY_CH2_ST_R = crate::BitReader;
///Field `PDMA_AXI_EVT_OUT_FIFO_EMPTY_CH2_ST` writer - Represents PDMA_AXI_evt_out_fifo_empty_ch2 trigger status.\\0: Not triggered\\1: Triggered
pub type PDMA_AXI_EVT_OUT_FIFO_EMPTY_CH2_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PDMA_AXI_EVT_OUT_FIFO_FULL_CH0_ST` reader - Represents PDMA_AXI_evt_out_fifo_full_ch0 trigger status.\\0: Not triggered\\1: Triggered
pub type PDMA_AXI_EVT_OUT_FIFO_FULL_CH0_ST_R = crate::BitReader;
///Field `PDMA_AXI_EVT_OUT_FIFO_FULL_CH0_ST` writer - Represents PDMA_AXI_evt_out_fifo_full_ch0 trigger status.\\0: Not triggered\\1: Triggered
pub type PDMA_AXI_EVT_OUT_FIFO_FULL_CH0_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PDMA_AXI_EVT_OUT_FIFO_FULL_CH1_ST` reader - Represents PDMA_AXI_evt_out_fifo_full_ch1 trigger status.\\0: Not triggered\\1: Triggered
pub type PDMA_AXI_EVT_OUT_FIFO_FULL_CH1_ST_R = crate::BitReader;
///Field `PDMA_AXI_EVT_OUT_FIFO_FULL_CH1_ST` writer - Represents PDMA_AXI_evt_out_fifo_full_ch1 trigger status.\\0: Not triggered\\1: Triggered
pub type PDMA_AXI_EVT_OUT_FIFO_FULL_CH1_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PDMA_AXI_EVT_OUT_FIFO_FULL_CH2_ST` reader - Represents PDMA_AXI_evt_out_fifo_full_ch2 trigger status.\\0: Not triggered\\1: Triggered
pub type PDMA_AXI_EVT_OUT_FIFO_FULL_CH2_ST_R = crate::BitReader;
///Field `PDMA_AXI_EVT_OUT_FIFO_FULL_CH2_ST` writer - Represents PDMA_AXI_evt_out_fifo_full_ch2 trigger status.\\0: Not triggered\\1: Triggered
pub type PDMA_AXI_EVT_OUT_FIFO_FULL_CH2_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PMU_EVT_SLEEP_WEEKUP_ST` reader - Represents PMU_evt_sleep_weekup trigger status.\\0: Not triggered\\1: Triggered
pub type PMU_EVT_SLEEP_WEEKUP_ST_R = crate::BitReader;
///Field `PMU_EVT_SLEEP_WEEKUP_ST` writer - Represents PMU_evt_sleep_weekup trigger status.\\0: Not triggered\\1: Triggered
pub type PMU_EVT_SLEEP_WEEKUP_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMA2D_EVT_IN_DONE_CH0_ST` reader - Represents DMA2D_evt_in_done_ch0 trigger status.\\0: Not triggered\\1: Triggered
pub type DMA2D_EVT_IN_DONE_CH0_ST_R = crate::BitReader;
///Field `DMA2D_EVT_IN_DONE_CH0_ST` writer - Represents DMA2D_evt_in_done_ch0 trigger status.\\0: Not triggered\\1: Triggered
pub type DMA2D_EVT_IN_DONE_CH0_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMA2D_EVT_IN_DONE_CH1_ST` reader - Represents DMA2D_evt_in_done_ch1 trigger status.\\0: Not triggered\\1: Triggered
pub type DMA2D_EVT_IN_DONE_CH1_ST_R = crate::BitReader;
///Field `DMA2D_EVT_IN_DONE_CH1_ST` writer - Represents DMA2D_evt_in_done_ch1 trigger status.\\0: Not triggered\\1: Triggered
pub type DMA2D_EVT_IN_DONE_CH1_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMA2D_EVT_IN_SUC_EOF_CH0_ST` reader - Represents DMA2D_evt_in_suc_eof_ch0 trigger status.\\0: Not triggered\\1: Triggered
pub type DMA2D_EVT_IN_SUC_EOF_CH0_ST_R = crate::BitReader;
///Field `DMA2D_EVT_IN_SUC_EOF_CH0_ST` writer - Represents DMA2D_evt_in_suc_eof_ch0 trigger status.\\0: Not triggered\\1: Triggered
pub type DMA2D_EVT_IN_SUC_EOF_CH0_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Represents PDMA_AHB_evt_out_fifo_full_ch2 trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    pub fn pdma_ahb_evt_out_fifo_full_ch2_st(&self) -> PDMA_AHB_EVT_OUT_FIFO_FULL_CH2_ST_R {
        PDMA_AHB_EVT_OUT_FIFO_FULL_CH2_ST_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Represents PDMA_AXI_evt_in_done_ch0 trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    pub fn pdma_axi_evt_in_done_ch0_st(&self) -> PDMA_AXI_EVT_IN_DONE_CH0_ST_R {
        PDMA_AXI_EVT_IN_DONE_CH0_ST_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Represents PDMA_AXI_evt_in_done_ch1 trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    pub fn pdma_axi_evt_in_done_ch1_st(&self) -> PDMA_AXI_EVT_IN_DONE_CH1_ST_R {
        PDMA_AXI_EVT_IN_DONE_CH1_ST_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Represents PDMA_AXI_evt_in_done_ch2 trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    pub fn pdma_axi_evt_in_done_ch2_st(&self) -> PDMA_AXI_EVT_IN_DONE_CH2_ST_R {
        PDMA_AXI_EVT_IN_DONE_CH2_ST_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Represents PDMA_AXI_evt_in_suc_eof_ch0 trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    pub fn pdma_axi_evt_in_suc_eof_ch0_st(&self) -> PDMA_AXI_EVT_IN_SUC_EOF_CH0_ST_R {
        PDMA_AXI_EVT_IN_SUC_EOF_CH0_ST_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Represents PDMA_AXI_evt_in_suc_eof_ch1 trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    pub fn pdma_axi_evt_in_suc_eof_ch1_st(&self) -> PDMA_AXI_EVT_IN_SUC_EOF_CH1_ST_R {
        PDMA_AXI_EVT_IN_SUC_EOF_CH1_ST_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Represents PDMA_AXI_evt_in_suc_eof_ch2 trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    pub fn pdma_axi_evt_in_suc_eof_ch2_st(&self) -> PDMA_AXI_EVT_IN_SUC_EOF_CH2_ST_R {
        PDMA_AXI_EVT_IN_SUC_EOF_CH2_ST_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Represents PDMA_AXI_evt_in_fifo_empty_ch0 trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    pub fn pdma_axi_evt_in_fifo_empty_ch0_st(&self) -> PDMA_AXI_EVT_IN_FIFO_EMPTY_CH0_ST_R {
        PDMA_AXI_EVT_IN_FIFO_EMPTY_CH0_ST_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Represents PDMA_AXI_evt_in_fifo_empty_ch1 trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    pub fn pdma_axi_evt_in_fifo_empty_ch1_st(&self) -> PDMA_AXI_EVT_IN_FIFO_EMPTY_CH1_ST_R {
        PDMA_AXI_EVT_IN_FIFO_EMPTY_CH1_ST_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Represents PDMA_AXI_evt_in_fifo_empty_ch2 trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    pub fn pdma_axi_evt_in_fifo_empty_ch2_st(&self) -> PDMA_AXI_EVT_IN_FIFO_EMPTY_CH2_ST_R {
        PDMA_AXI_EVT_IN_FIFO_EMPTY_CH2_ST_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Represents PDMA_AXI_evt_in_fifo_full_ch0 trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    pub fn pdma_axi_evt_in_fifo_full_ch0_st(&self) -> PDMA_AXI_EVT_IN_FIFO_FULL_CH0_ST_R {
        PDMA_AXI_EVT_IN_FIFO_FULL_CH0_ST_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Represents PDMA_AXI_evt_in_fifo_full_ch1 trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    pub fn pdma_axi_evt_in_fifo_full_ch1_st(&self) -> PDMA_AXI_EVT_IN_FIFO_FULL_CH1_ST_R {
        PDMA_AXI_EVT_IN_FIFO_FULL_CH1_ST_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Represents PDMA_AXI_evt_in_fifo_full_ch2 trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    pub fn pdma_axi_evt_in_fifo_full_ch2_st(&self) -> PDMA_AXI_EVT_IN_FIFO_FULL_CH2_ST_R {
        PDMA_AXI_EVT_IN_FIFO_FULL_CH2_ST_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Represents PDMA_AXI_evt_out_done_ch0 trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    pub fn pdma_axi_evt_out_done_ch0_st(&self) -> PDMA_AXI_EVT_OUT_DONE_CH0_ST_R {
        PDMA_AXI_EVT_OUT_DONE_CH0_ST_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Represents PDMA_AXI_evt_out_done_ch1 trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    pub fn pdma_axi_evt_out_done_ch1_st(&self) -> PDMA_AXI_EVT_OUT_DONE_CH1_ST_R {
        PDMA_AXI_EVT_OUT_DONE_CH1_ST_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Represents PDMA_AXI_evt_out_done_ch2 trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    pub fn pdma_axi_evt_out_done_ch2_st(&self) -> PDMA_AXI_EVT_OUT_DONE_CH2_ST_R {
        PDMA_AXI_EVT_OUT_DONE_CH2_ST_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Represents PDMA_AXI_evt_out_eof_ch0 trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    pub fn pdma_axi_evt_out_eof_ch0_st(&self) -> PDMA_AXI_EVT_OUT_EOF_CH0_ST_R {
        PDMA_AXI_EVT_OUT_EOF_CH0_ST_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Represents PDMA_AXI_evt_out_eof_ch1 trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    pub fn pdma_axi_evt_out_eof_ch1_st(&self) -> PDMA_AXI_EVT_OUT_EOF_CH1_ST_R {
        PDMA_AXI_EVT_OUT_EOF_CH1_ST_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Represents PDMA_AXI_evt_out_eof_ch2 trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    pub fn pdma_axi_evt_out_eof_ch2_st(&self) -> PDMA_AXI_EVT_OUT_EOF_CH2_ST_R {
        PDMA_AXI_EVT_OUT_EOF_CH2_ST_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Represents PDMA_AXI_evt_out_total_eof_ch0 trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    pub fn pdma_axi_evt_out_total_eof_ch0_st(&self) -> PDMA_AXI_EVT_OUT_TOTAL_EOF_CH0_ST_R {
        PDMA_AXI_EVT_OUT_TOTAL_EOF_CH0_ST_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Represents PDMA_AXI_evt_out_total_eof_ch1 trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    pub fn pdma_axi_evt_out_total_eof_ch1_st(&self) -> PDMA_AXI_EVT_OUT_TOTAL_EOF_CH1_ST_R {
        PDMA_AXI_EVT_OUT_TOTAL_EOF_CH1_ST_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Represents PDMA_AXI_evt_out_total_eof_ch2 trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    pub fn pdma_axi_evt_out_total_eof_ch2_st(&self) -> PDMA_AXI_EVT_OUT_TOTAL_EOF_CH2_ST_R {
        PDMA_AXI_EVT_OUT_TOTAL_EOF_CH2_ST_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Represents PDMA_AXI_evt_out_fifo_empty_ch0 trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    pub fn pdma_axi_evt_out_fifo_empty_ch0_st(&self) -> PDMA_AXI_EVT_OUT_FIFO_EMPTY_CH0_ST_R {
        PDMA_AXI_EVT_OUT_FIFO_EMPTY_CH0_ST_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Represents PDMA_AXI_evt_out_fifo_empty_ch1 trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    pub fn pdma_axi_evt_out_fifo_empty_ch1_st(&self) -> PDMA_AXI_EVT_OUT_FIFO_EMPTY_CH1_ST_R {
        PDMA_AXI_EVT_OUT_FIFO_EMPTY_CH1_ST_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Represents PDMA_AXI_evt_out_fifo_empty_ch2 trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    pub fn pdma_axi_evt_out_fifo_empty_ch2_st(&self) -> PDMA_AXI_EVT_OUT_FIFO_EMPTY_CH2_ST_R {
        PDMA_AXI_EVT_OUT_FIFO_EMPTY_CH2_ST_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Represents PDMA_AXI_evt_out_fifo_full_ch0 trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    pub fn pdma_axi_evt_out_fifo_full_ch0_st(&self) -> PDMA_AXI_EVT_OUT_FIFO_FULL_CH0_ST_R {
        PDMA_AXI_EVT_OUT_FIFO_FULL_CH0_ST_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Represents PDMA_AXI_evt_out_fifo_full_ch1 trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    pub fn pdma_axi_evt_out_fifo_full_ch1_st(&self) -> PDMA_AXI_EVT_OUT_FIFO_FULL_CH1_ST_R {
        PDMA_AXI_EVT_OUT_FIFO_FULL_CH1_ST_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Represents PDMA_AXI_evt_out_fifo_full_ch2 trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    pub fn pdma_axi_evt_out_fifo_full_ch2_st(&self) -> PDMA_AXI_EVT_OUT_FIFO_FULL_CH2_ST_R {
        PDMA_AXI_EVT_OUT_FIFO_FULL_CH2_ST_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Represents PMU_evt_sleep_weekup trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    pub fn pmu_evt_sleep_weekup_st(&self) -> PMU_EVT_SLEEP_WEEKUP_ST_R {
        PMU_EVT_SLEEP_WEEKUP_ST_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Represents DMA2D_evt_in_done_ch0 trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    pub fn dma2d_evt_in_done_ch0_st(&self) -> DMA2D_EVT_IN_DONE_CH0_ST_R {
        DMA2D_EVT_IN_DONE_CH0_ST_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Represents DMA2D_evt_in_done_ch1 trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    pub fn dma2d_evt_in_done_ch1_st(&self) -> DMA2D_EVT_IN_DONE_CH1_ST_R {
        DMA2D_EVT_IN_DONE_CH1_ST_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Represents DMA2D_evt_in_suc_eof_ch0 trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    pub fn dma2d_evt_in_suc_eof_ch0_st(&self) -> DMA2D_EVT_IN_SUC_EOF_CH0_ST_R {
        DMA2D_EVT_IN_SUC_EOF_CH0_ST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EVT_ST6")
            .field(
                "pdma_ahb_evt_out_fifo_full_ch2_st",
                &self.pdma_ahb_evt_out_fifo_full_ch2_st(),
            )
            .field(
                "pdma_axi_evt_in_done_ch0_st",
                &self.pdma_axi_evt_in_done_ch0_st(),
            )
            .field(
                "pdma_axi_evt_in_done_ch1_st",
                &self.pdma_axi_evt_in_done_ch1_st(),
            )
            .field(
                "pdma_axi_evt_in_done_ch2_st",
                &self.pdma_axi_evt_in_done_ch2_st(),
            )
            .field(
                "pdma_axi_evt_in_suc_eof_ch0_st",
                &self.pdma_axi_evt_in_suc_eof_ch0_st(),
            )
            .field(
                "pdma_axi_evt_in_suc_eof_ch1_st",
                &self.pdma_axi_evt_in_suc_eof_ch1_st(),
            )
            .field(
                "pdma_axi_evt_in_suc_eof_ch2_st",
                &self.pdma_axi_evt_in_suc_eof_ch2_st(),
            )
            .field(
                "pdma_axi_evt_in_fifo_empty_ch0_st",
                &self.pdma_axi_evt_in_fifo_empty_ch0_st(),
            )
            .field(
                "pdma_axi_evt_in_fifo_empty_ch1_st",
                &self.pdma_axi_evt_in_fifo_empty_ch1_st(),
            )
            .field(
                "pdma_axi_evt_in_fifo_empty_ch2_st",
                &self.pdma_axi_evt_in_fifo_empty_ch2_st(),
            )
            .field(
                "pdma_axi_evt_in_fifo_full_ch0_st",
                &self.pdma_axi_evt_in_fifo_full_ch0_st(),
            )
            .field(
                "pdma_axi_evt_in_fifo_full_ch1_st",
                &self.pdma_axi_evt_in_fifo_full_ch1_st(),
            )
            .field(
                "pdma_axi_evt_in_fifo_full_ch2_st",
                &self.pdma_axi_evt_in_fifo_full_ch2_st(),
            )
            .field(
                "pdma_axi_evt_out_done_ch0_st",
                &self.pdma_axi_evt_out_done_ch0_st(),
            )
            .field(
                "pdma_axi_evt_out_done_ch1_st",
                &self.pdma_axi_evt_out_done_ch1_st(),
            )
            .field(
                "pdma_axi_evt_out_done_ch2_st",
                &self.pdma_axi_evt_out_done_ch2_st(),
            )
            .field(
                "pdma_axi_evt_out_eof_ch0_st",
                &self.pdma_axi_evt_out_eof_ch0_st(),
            )
            .field(
                "pdma_axi_evt_out_eof_ch1_st",
                &self.pdma_axi_evt_out_eof_ch1_st(),
            )
            .field(
                "pdma_axi_evt_out_eof_ch2_st",
                &self.pdma_axi_evt_out_eof_ch2_st(),
            )
            .field(
                "pdma_axi_evt_out_total_eof_ch0_st",
                &self.pdma_axi_evt_out_total_eof_ch0_st(),
            )
            .field(
                "pdma_axi_evt_out_total_eof_ch1_st",
                &self.pdma_axi_evt_out_total_eof_ch1_st(),
            )
            .field(
                "pdma_axi_evt_out_total_eof_ch2_st",
                &self.pdma_axi_evt_out_total_eof_ch2_st(),
            )
            .field(
                "pdma_axi_evt_out_fifo_empty_ch0_st",
                &self.pdma_axi_evt_out_fifo_empty_ch0_st(),
            )
            .field(
                "pdma_axi_evt_out_fifo_empty_ch1_st",
                &self.pdma_axi_evt_out_fifo_empty_ch1_st(),
            )
            .field(
                "pdma_axi_evt_out_fifo_empty_ch2_st",
                &self.pdma_axi_evt_out_fifo_empty_ch2_st(),
            )
            .field(
                "pdma_axi_evt_out_fifo_full_ch0_st",
                &self.pdma_axi_evt_out_fifo_full_ch0_st(),
            )
            .field(
                "pdma_axi_evt_out_fifo_full_ch1_st",
                &self.pdma_axi_evt_out_fifo_full_ch1_st(),
            )
            .field(
                "pdma_axi_evt_out_fifo_full_ch2_st",
                &self.pdma_axi_evt_out_fifo_full_ch2_st(),
            )
            .field("pmu_evt_sleep_weekup_st", &self.pmu_evt_sleep_weekup_st())
            .field("dma2d_evt_in_done_ch0_st", &self.dma2d_evt_in_done_ch0_st())
            .field("dma2d_evt_in_done_ch1_st", &self.dma2d_evt_in_done_ch1_st())
            .field(
                "dma2d_evt_in_suc_eof_ch0_st",
                &self.dma2d_evt_in_suc_eof_ch0_st(),
            )
            .finish()
    }
}
impl W {
    ///Bit 0 - Represents PDMA_AHB_evt_out_fifo_full_ch2 trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    #[must_use]
    pub fn pdma_ahb_evt_out_fifo_full_ch2_st(
        &mut self,
    ) -> PDMA_AHB_EVT_OUT_FIFO_FULL_CH2_ST_W<EVT_ST6_SPEC> {
        PDMA_AHB_EVT_OUT_FIFO_FULL_CH2_ST_W::new(self, 0)
    }
    ///Bit 1 - Represents PDMA_AXI_evt_in_done_ch0 trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    #[must_use]
    pub fn pdma_axi_evt_in_done_ch0_st(&mut self) -> PDMA_AXI_EVT_IN_DONE_CH0_ST_W<EVT_ST6_SPEC> {
        PDMA_AXI_EVT_IN_DONE_CH0_ST_W::new(self, 1)
    }
    ///Bit 2 - Represents PDMA_AXI_evt_in_done_ch1 trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    #[must_use]
    pub fn pdma_axi_evt_in_done_ch1_st(&mut self) -> PDMA_AXI_EVT_IN_DONE_CH1_ST_W<EVT_ST6_SPEC> {
        PDMA_AXI_EVT_IN_DONE_CH1_ST_W::new(self, 2)
    }
    ///Bit 3 - Represents PDMA_AXI_evt_in_done_ch2 trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    #[must_use]
    pub fn pdma_axi_evt_in_done_ch2_st(&mut self) -> PDMA_AXI_EVT_IN_DONE_CH2_ST_W<EVT_ST6_SPEC> {
        PDMA_AXI_EVT_IN_DONE_CH2_ST_W::new(self, 3)
    }
    ///Bit 4 - Represents PDMA_AXI_evt_in_suc_eof_ch0 trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    #[must_use]
    pub fn pdma_axi_evt_in_suc_eof_ch0_st(
        &mut self,
    ) -> PDMA_AXI_EVT_IN_SUC_EOF_CH0_ST_W<EVT_ST6_SPEC> {
        PDMA_AXI_EVT_IN_SUC_EOF_CH0_ST_W::new(self, 4)
    }
    ///Bit 5 - Represents PDMA_AXI_evt_in_suc_eof_ch1 trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    #[must_use]
    pub fn pdma_axi_evt_in_suc_eof_ch1_st(
        &mut self,
    ) -> PDMA_AXI_EVT_IN_SUC_EOF_CH1_ST_W<EVT_ST6_SPEC> {
        PDMA_AXI_EVT_IN_SUC_EOF_CH1_ST_W::new(self, 5)
    }
    ///Bit 6 - Represents PDMA_AXI_evt_in_suc_eof_ch2 trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    #[must_use]
    pub fn pdma_axi_evt_in_suc_eof_ch2_st(
        &mut self,
    ) -> PDMA_AXI_EVT_IN_SUC_EOF_CH2_ST_W<EVT_ST6_SPEC> {
        PDMA_AXI_EVT_IN_SUC_EOF_CH2_ST_W::new(self, 6)
    }
    ///Bit 7 - Represents PDMA_AXI_evt_in_fifo_empty_ch0 trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    #[must_use]
    pub fn pdma_axi_evt_in_fifo_empty_ch0_st(
        &mut self,
    ) -> PDMA_AXI_EVT_IN_FIFO_EMPTY_CH0_ST_W<EVT_ST6_SPEC> {
        PDMA_AXI_EVT_IN_FIFO_EMPTY_CH0_ST_W::new(self, 7)
    }
    ///Bit 8 - Represents PDMA_AXI_evt_in_fifo_empty_ch1 trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    #[must_use]
    pub fn pdma_axi_evt_in_fifo_empty_ch1_st(
        &mut self,
    ) -> PDMA_AXI_EVT_IN_FIFO_EMPTY_CH1_ST_W<EVT_ST6_SPEC> {
        PDMA_AXI_EVT_IN_FIFO_EMPTY_CH1_ST_W::new(self, 8)
    }
    ///Bit 9 - Represents PDMA_AXI_evt_in_fifo_empty_ch2 trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    #[must_use]
    pub fn pdma_axi_evt_in_fifo_empty_ch2_st(
        &mut self,
    ) -> PDMA_AXI_EVT_IN_FIFO_EMPTY_CH2_ST_W<EVT_ST6_SPEC> {
        PDMA_AXI_EVT_IN_FIFO_EMPTY_CH2_ST_W::new(self, 9)
    }
    ///Bit 10 - Represents PDMA_AXI_evt_in_fifo_full_ch0 trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    #[must_use]
    pub fn pdma_axi_evt_in_fifo_full_ch0_st(
        &mut self,
    ) -> PDMA_AXI_EVT_IN_FIFO_FULL_CH0_ST_W<EVT_ST6_SPEC> {
        PDMA_AXI_EVT_IN_FIFO_FULL_CH0_ST_W::new(self, 10)
    }
    ///Bit 11 - Represents PDMA_AXI_evt_in_fifo_full_ch1 trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    #[must_use]
    pub fn pdma_axi_evt_in_fifo_full_ch1_st(
        &mut self,
    ) -> PDMA_AXI_EVT_IN_FIFO_FULL_CH1_ST_W<EVT_ST6_SPEC> {
        PDMA_AXI_EVT_IN_FIFO_FULL_CH1_ST_W::new(self, 11)
    }
    ///Bit 12 - Represents PDMA_AXI_evt_in_fifo_full_ch2 trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    #[must_use]
    pub fn pdma_axi_evt_in_fifo_full_ch2_st(
        &mut self,
    ) -> PDMA_AXI_EVT_IN_FIFO_FULL_CH2_ST_W<EVT_ST6_SPEC> {
        PDMA_AXI_EVT_IN_FIFO_FULL_CH2_ST_W::new(self, 12)
    }
    ///Bit 13 - Represents PDMA_AXI_evt_out_done_ch0 trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    #[must_use]
    pub fn pdma_axi_evt_out_done_ch0_st(&mut self) -> PDMA_AXI_EVT_OUT_DONE_CH0_ST_W<EVT_ST6_SPEC> {
        PDMA_AXI_EVT_OUT_DONE_CH0_ST_W::new(self, 13)
    }
    ///Bit 14 - Represents PDMA_AXI_evt_out_done_ch1 trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    #[must_use]
    pub fn pdma_axi_evt_out_done_ch1_st(&mut self) -> PDMA_AXI_EVT_OUT_DONE_CH1_ST_W<EVT_ST6_SPEC> {
        PDMA_AXI_EVT_OUT_DONE_CH1_ST_W::new(self, 14)
    }
    ///Bit 15 - Represents PDMA_AXI_evt_out_done_ch2 trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    #[must_use]
    pub fn pdma_axi_evt_out_done_ch2_st(&mut self) -> PDMA_AXI_EVT_OUT_DONE_CH2_ST_W<EVT_ST6_SPEC> {
        PDMA_AXI_EVT_OUT_DONE_CH2_ST_W::new(self, 15)
    }
    ///Bit 16 - Represents PDMA_AXI_evt_out_eof_ch0 trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    #[must_use]
    pub fn pdma_axi_evt_out_eof_ch0_st(&mut self) -> PDMA_AXI_EVT_OUT_EOF_CH0_ST_W<EVT_ST6_SPEC> {
        PDMA_AXI_EVT_OUT_EOF_CH0_ST_W::new(self, 16)
    }
    ///Bit 17 - Represents PDMA_AXI_evt_out_eof_ch1 trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    #[must_use]
    pub fn pdma_axi_evt_out_eof_ch1_st(&mut self) -> PDMA_AXI_EVT_OUT_EOF_CH1_ST_W<EVT_ST6_SPEC> {
        PDMA_AXI_EVT_OUT_EOF_CH1_ST_W::new(self, 17)
    }
    ///Bit 18 - Represents PDMA_AXI_evt_out_eof_ch2 trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    #[must_use]
    pub fn pdma_axi_evt_out_eof_ch2_st(&mut self) -> PDMA_AXI_EVT_OUT_EOF_CH2_ST_W<EVT_ST6_SPEC> {
        PDMA_AXI_EVT_OUT_EOF_CH2_ST_W::new(self, 18)
    }
    ///Bit 19 - Represents PDMA_AXI_evt_out_total_eof_ch0 trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    #[must_use]
    pub fn pdma_axi_evt_out_total_eof_ch0_st(
        &mut self,
    ) -> PDMA_AXI_EVT_OUT_TOTAL_EOF_CH0_ST_W<EVT_ST6_SPEC> {
        PDMA_AXI_EVT_OUT_TOTAL_EOF_CH0_ST_W::new(self, 19)
    }
    ///Bit 20 - Represents PDMA_AXI_evt_out_total_eof_ch1 trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    #[must_use]
    pub fn pdma_axi_evt_out_total_eof_ch1_st(
        &mut self,
    ) -> PDMA_AXI_EVT_OUT_TOTAL_EOF_CH1_ST_W<EVT_ST6_SPEC> {
        PDMA_AXI_EVT_OUT_TOTAL_EOF_CH1_ST_W::new(self, 20)
    }
    ///Bit 21 - Represents PDMA_AXI_evt_out_total_eof_ch2 trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    #[must_use]
    pub fn pdma_axi_evt_out_total_eof_ch2_st(
        &mut self,
    ) -> PDMA_AXI_EVT_OUT_TOTAL_EOF_CH2_ST_W<EVT_ST6_SPEC> {
        PDMA_AXI_EVT_OUT_TOTAL_EOF_CH2_ST_W::new(self, 21)
    }
    ///Bit 22 - Represents PDMA_AXI_evt_out_fifo_empty_ch0 trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    #[must_use]
    pub fn pdma_axi_evt_out_fifo_empty_ch0_st(
        &mut self,
    ) -> PDMA_AXI_EVT_OUT_FIFO_EMPTY_CH0_ST_W<EVT_ST6_SPEC> {
        PDMA_AXI_EVT_OUT_FIFO_EMPTY_CH0_ST_W::new(self, 22)
    }
    ///Bit 23 - Represents PDMA_AXI_evt_out_fifo_empty_ch1 trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    #[must_use]
    pub fn pdma_axi_evt_out_fifo_empty_ch1_st(
        &mut self,
    ) -> PDMA_AXI_EVT_OUT_FIFO_EMPTY_CH1_ST_W<EVT_ST6_SPEC> {
        PDMA_AXI_EVT_OUT_FIFO_EMPTY_CH1_ST_W::new(self, 23)
    }
    ///Bit 24 - Represents PDMA_AXI_evt_out_fifo_empty_ch2 trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    #[must_use]
    pub fn pdma_axi_evt_out_fifo_empty_ch2_st(
        &mut self,
    ) -> PDMA_AXI_EVT_OUT_FIFO_EMPTY_CH2_ST_W<EVT_ST6_SPEC> {
        PDMA_AXI_EVT_OUT_FIFO_EMPTY_CH2_ST_W::new(self, 24)
    }
    ///Bit 25 - Represents PDMA_AXI_evt_out_fifo_full_ch0 trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    #[must_use]
    pub fn pdma_axi_evt_out_fifo_full_ch0_st(
        &mut self,
    ) -> PDMA_AXI_EVT_OUT_FIFO_FULL_CH0_ST_W<EVT_ST6_SPEC> {
        PDMA_AXI_EVT_OUT_FIFO_FULL_CH0_ST_W::new(self, 25)
    }
    ///Bit 26 - Represents PDMA_AXI_evt_out_fifo_full_ch1 trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    #[must_use]
    pub fn pdma_axi_evt_out_fifo_full_ch1_st(
        &mut self,
    ) -> PDMA_AXI_EVT_OUT_FIFO_FULL_CH1_ST_W<EVT_ST6_SPEC> {
        PDMA_AXI_EVT_OUT_FIFO_FULL_CH1_ST_W::new(self, 26)
    }
    ///Bit 27 - Represents PDMA_AXI_evt_out_fifo_full_ch2 trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    #[must_use]
    pub fn pdma_axi_evt_out_fifo_full_ch2_st(
        &mut self,
    ) -> PDMA_AXI_EVT_OUT_FIFO_FULL_CH2_ST_W<EVT_ST6_SPEC> {
        PDMA_AXI_EVT_OUT_FIFO_FULL_CH2_ST_W::new(self, 27)
    }
    ///Bit 28 - Represents PMU_evt_sleep_weekup trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    #[must_use]
    pub fn pmu_evt_sleep_weekup_st(&mut self) -> PMU_EVT_SLEEP_WEEKUP_ST_W<EVT_ST6_SPEC> {
        PMU_EVT_SLEEP_WEEKUP_ST_W::new(self, 28)
    }
    ///Bit 29 - Represents DMA2D_evt_in_done_ch0 trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    #[must_use]
    pub fn dma2d_evt_in_done_ch0_st(&mut self) -> DMA2D_EVT_IN_DONE_CH0_ST_W<EVT_ST6_SPEC> {
        DMA2D_EVT_IN_DONE_CH0_ST_W::new(self, 29)
    }
    ///Bit 30 - Represents DMA2D_evt_in_done_ch1 trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    #[must_use]
    pub fn dma2d_evt_in_done_ch1_st(&mut self) -> DMA2D_EVT_IN_DONE_CH1_ST_W<EVT_ST6_SPEC> {
        DMA2D_EVT_IN_DONE_CH1_ST_W::new(self, 30)
    }
    ///Bit 31 - Represents DMA2D_evt_in_suc_eof_ch0 trigger status.\\0: Not triggered\\1: Triggered
    #[inline(always)]
    #[must_use]
    pub fn dma2d_evt_in_suc_eof_ch0_st(&mut self) -> DMA2D_EVT_IN_SUC_EOF_CH0_ST_W<EVT_ST6_SPEC> {
        DMA2D_EVT_IN_SUC_EOF_CH0_ST_W::new(self, 31)
    }
}
/**Events trigger status register

You can [`read`](crate::generic::Reg::read) this register and get [`evt_st6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evt_st6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct EVT_ST6_SPEC;
impl crate::RegisterSpec for EVT_ST6_SPEC {
    type Ux = u32;
}
///`read()` method returns [`evt_st6::R`](R) reader structure
impl crate::Readable for EVT_ST6_SPEC {}
///`write(|w| ..)` method takes [`evt_st6::W`](W) writer structure
impl crate::Writable for EVT_ST6_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets EVT_ST6 to value 0
impl crate::Resettable for EVT_ST6_SPEC {
    const RESET_VALUE: u32 = 0;
}
