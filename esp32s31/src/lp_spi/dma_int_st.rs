#[doc = "Register `DMA_INT_ST` reader"]
pub type R = crate::R<DMA_INT_ST_SPEC>;
#[doc = "Field `SLV_RD_BUF_DONE_INT_ST` reader - "]
pub type SLV_RD_BUF_DONE_INT_ST_R = crate::BitReader;
#[doc = "Field `SLV_WR_BUF_DONE_INT_ST` reader - "]
pub type SLV_WR_BUF_DONE_INT_ST_R = crate::BitReader;
#[doc = "Field `TRANS_DONE_INT_ST` reader - "]
pub type TRANS_DONE_INT_ST_R = crate::BitReader;
#[doc = "Field `SPI_WAKEUP_INT_ST` reader - "]
pub type SPI_WAKEUP_INT_ST_R = crate::BitReader;
#[doc = "Field `SLV_BUF_ADDR_ERR_INT_ST` reader - "]
pub type SLV_BUF_ADDR_ERR_INT_ST_R = crate::BitReader;
#[doc = "Field `SLV_CMD_ERR_INT_ST` reader - "]
pub type SLV_CMD_ERR_INT_ST_R = crate::BitReader;
#[doc = "Field `MST_RX_AFIFO_WFULL_ERR_INT_ST` reader - "]
pub type MST_RX_AFIFO_WFULL_ERR_INT_ST_R = crate::BitReader;
#[doc = "Field `MST_TX_AFIFO_REMPTY_ERR_INT_ST` reader - "]
pub type MST_TX_AFIFO_REMPTY_ERR_INT_ST_R = crate::BitReader;
#[doc = "Field `APP2_INT_ST` reader - "]
pub type APP2_INT_ST_R = crate::BitReader;
#[doc = "Field `APP1_INT_ST` reader - "]
pub type APP1_INT_ST_R = crate::BitReader;
impl R {
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn slv_rd_buf_done_int_st(&self) -> SLV_RD_BUF_DONE_INT_ST_R {
        SLV_RD_BUF_DONE_INT_ST_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn slv_wr_buf_done_int_st(&self) -> SLV_WR_BUF_DONE_INT_ST_R {
        SLV_WR_BUF_DONE_INT_ST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn trans_done_int_st(&self) -> TRANS_DONE_INT_ST_R {
        TRANS_DONE_INT_ST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn spi_wakeup_int_st(&self) -> SPI_WAKEUP_INT_ST_R {
        SPI_WAKEUP_INT_ST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn slv_buf_addr_err_int_st(&self) -> SLV_BUF_ADDR_ERR_INT_ST_R {
        SLV_BUF_ADDR_ERR_INT_ST_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn slv_cmd_err_int_st(&self) -> SLV_CMD_ERR_INT_ST_R {
        SLV_CMD_ERR_INT_ST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn mst_rx_afifo_wfull_err_int_st(&self) -> MST_RX_AFIFO_WFULL_ERR_INT_ST_R {
        MST_RX_AFIFO_WFULL_ERR_INT_ST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn mst_tx_afifo_rempty_err_int_st(&self) -> MST_TX_AFIFO_REMPTY_ERR_INT_ST_R {
        MST_TX_AFIFO_REMPTY_ERR_INT_ST_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn app2_int_st(&self) -> APP2_INT_ST_R {
        APP2_INT_ST_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn app1_int_st(&self) -> APP1_INT_ST_R {
        APP1_INT_ST_R::new(((self.bits >> 20) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_INT_ST")
            .field("slv_rd_buf_done_int_st", &self.slv_rd_buf_done_int_st())
            .field("slv_wr_buf_done_int_st", &self.slv_wr_buf_done_int_st())
            .field("trans_done_int_st", &self.trans_done_int_st())
            .field("spi_wakeup_int_st", &self.spi_wakeup_int_st())
            .field("slv_buf_addr_err_int_st", &self.slv_buf_addr_err_int_st())
            .field("slv_cmd_err_int_st", &self.slv_cmd_err_int_st())
            .field(
                "mst_rx_afifo_wfull_err_int_st",
                &self.mst_rx_afifo_wfull_err_int_st(),
            )
            .field(
                "mst_tx_afifo_rempty_err_int_st",
                &self.mst_tx_afifo_rempty_err_int_st(),
            )
            .field("app2_int_st", &self.app2_int_st())
            .field("app1_int_st", &self.app1_int_st())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_int_st::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMA_INT_ST_SPEC;
impl crate::RegisterSpec for DMA_INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_int_st::R`](R) reader structure"]
impl crate::Readable for DMA_INT_ST_SPEC {}
#[doc = "`reset()` method sets DMA_INT_ST to value 0"]
impl crate::Resettable for DMA_INT_ST_SPEC {}
