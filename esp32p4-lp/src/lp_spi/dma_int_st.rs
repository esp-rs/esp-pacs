#[doc = "Register `DMA_INT_ST` reader"]
pub type R = crate::R<DMA_INT_ST_SPEC>;
#[doc = "Field `LP_REG_SLV_RD_BUF_DONE_INT_ST` reader - The status bit for SPI_SLV_RD_BUF_DONE_INT interrupt."]
pub type LP_REG_SLV_RD_BUF_DONE_INT_ST_R = crate::BitReader;
#[doc = "Field `LP_REG_SLV_WR_BUF_DONE_INT_ST` reader - The status bit for SPI_SLV_WR_BUF_DONE_INT interrupt."]
pub type LP_REG_SLV_WR_BUF_DONE_INT_ST_R = crate::BitReader;
#[doc = "Field `LP_REG_TRANS_DONE_INT_ST` reader - The status bit for SPI_TRANS_DONE_INT interrupt."]
pub type LP_REG_TRANS_DONE_INT_ST_R = crate::BitReader;
#[doc = "Field `LP_REG_SPI_WAKEUP_INT_ST` reader - reserved"]
pub type LP_REG_SPI_WAKEUP_INT_ST_R = crate::BitReader;
#[doc = "Field `LP_REG_SLV_BUF_ADDR_ERR_INT_ST` reader - The status bit for SPI_SLV_BUF_ADDR_ERR_INT interrupt."]
pub type LP_REG_SLV_BUF_ADDR_ERR_INT_ST_R = crate::BitReader;
#[doc = "Field `LP_REG_SLV_CMD_ERR_INT_ST` reader - The status bit for SPI_SLV_CMD_ERR_INT interrupt."]
pub type LP_REG_SLV_CMD_ERR_INT_ST_R = crate::BitReader;
#[doc = "Field `LP_REG_MST_RX_AFIFO_WFULL_ERR_INT_ST` reader - The status bit for SPI_MST_RX_AFIFO_WFULL_ERR_INT interrupt."]
pub type LP_REG_MST_RX_AFIFO_WFULL_ERR_INT_ST_R = crate::BitReader;
#[doc = "Field `LP_REG_MST_TX_AFIFO_REMPTY_ERR_INT_ST` reader - The status bit for SPI_MST_TX_AFIFO_REMPTY_ERR_INT interrupt."]
pub type LP_REG_MST_TX_AFIFO_REMPTY_ERR_INT_ST_R = crate::BitReader;
#[doc = "Field `LP_REG_APP2_INT_ST` reader - The status bit for SPI_APP2_INT interrupt."]
pub type LP_REG_APP2_INT_ST_R = crate::BitReader;
#[doc = "Field `LP_REG_APP1_INT_ST` reader - The status bit for SPI_APP1_INT interrupt."]
pub type LP_REG_APP1_INT_ST_R = crate::BitReader;
impl R {
    #[doc = "Bit 10 - The status bit for SPI_SLV_RD_BUF_DONE_INT interrupt."]
    #[inline(always)]
    pub fn lp_reg_slv_rd_buf_done_int_st(&self) -> LP_REG_SLV_RD_BUF_DONE_INT_ST_R {
        LP_REG_SLV_RD_BUF_DONE_INT_ST_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The status bit for SPI_SLV_WR_BUF_DONE_INT interrupt."]
    #[inline(always)]
    pub fn lp_reg_slv_wr_buf_done_int_st(&self) -> LP_REG_SLV_WR_BUF_DONE_INT_ST_R {
        LP_REG_SLV_WR_BUF_DONE_INT_ST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - The status bit for SPI_TRANS_DONE_INT interrupt."]
    #[inline(always)]
    pub fn lp_reg_trans_done_int_st(&self) -> LP_REG_TRANS_DONE_INT_ST_R {
        LP_REG_TRANS_DONE_INT_ST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - reserved"]
    #[inline(always)]
    pub fn lp_reg_spi_wakeup_int_st(&self) -> LP_REG_SPI_WAKEUP_INT_ST_R {
        LP_REG_SPI_WAKEUP_INT_ST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - The status bit for SPI_SLV_BUF_ADDR_ERR_INT interrupt."]
    #[inline(always)]
    pub fn lp_reg_slv_buf_addr_err_int_st(&self) -> LP_REG_SLV_BUF_ADDR_ERR_INT_ST_R {
        LP_REG_SLV_BUF_ADDR_ERR_INT_ST_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - The status bit for SPI_SLV_CMD_ERR_INT interrupt."]
    #[inline(always)]
    pub fn lp_reg_slv_cmd_err_int_st(&self) -> LP_REG_SLV_CMD_ERR_INT_ST_R {
        LP_REG_SLV_CMD_ERR_INT_ST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - The status bit for SPI_MST_RX_AFIFO_WFULL_ERR_INT interrupt."]
    #[inline(always)]
    pub fn lp_reg_mst_rx_afifo_wfull_err_int_st(&self) -> LP_REG_MST_RX_AFIFO_WFULL_ERR_INT_ST_R {
        LP_REG_MST_RX_AFIFO_WFULL_ERR_INT_ST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - The status bit for SPI_MST_TX_AFIFO_REMPTY_ERR_INT interrupt."]
    #[inline(always)]
    pub fn lp_reg_mst_tx_afifo_rempty_err_int_st(&self) -> LP_REG_MST_TX_AFIFO_REMPTY_ERR_INT_ST_R {
        LP_REG_MST_TX_AFIFO_REMPTY_ERR_INT_ST_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - The status bit for SPI_APP2_INT interrupt."]
    #[inline(always)]
    pub fn lp_reg_app2_int_st(&self) -> LP_REG_APP2_INT_ST_R {
        LP_REG_APP2_INT_ST_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - The status bit for SPI_APP1_INT interrupt."]
    #[inline(always)]
    pub fn lp_reg_app1_int_st(&self) -> LP_REG_APP1_INT_ST_R {
        LP_REG_APP1_INT_ST_R::new(((self.bits >> 20) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_INT_ST")
            .field(
                "lp_reg_slv_rd_buf_done_int_st",
                &self.lp_reg_slv_rd_buf_done_int_st(),
            )
            .field(
                "lp_reg_slv_wr_buf_done_int_st",
                &self.lp_reg_slv_wr_buf_done_int_st(),
            )
            .field("lp_reg_trans_done_int_st", &self.lp_reg_trans_done_int_st())
            .field("lp_reg_spi_wakeup_int_st", &self.lp_reg_spi_wakeup_int_st())
            .field(
                "lp_reg_slv_buf_addr_err_int_st",
                &self.lp_reg_slv_buf_addr_err_int_st(),
            )
            .field(
                "lp_reg_slv_cmd_err_int_st",
                &self.lp_reg_slv_cmd_err_int_st(),
            )
            .field(
                "lp_reg_mst_rx_afifo_wfull_err_int_st",
                &self.lp_reg_mst_rx_afifo_wfull_err_int_st(),
            )
            .field(
                "lp_reg_mst_tx_afifo_rempty_err_int_st",
                &self.lp_reg_mst_tx_afifo_rempty_err_int_st(),
            )
            .field("lp_reg_app2_int_st", &self.lp_reg_app2_int_st())
            .field("lp_reg_app1_int_st", &self.lp_reg_app1_int_st())
            .finish()
    }
}
#[doc = "SPI DMA interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_int_st::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMA_INT_ST_SPEC;
impl crate::RegisterSpec for DMA_INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_int_st::R`](R) reader structure"]
impl crate::Readable for DMA_INT_ST_SPEC {}
#[doc = "`reset()` method sets DMA_INT_ST to value 0"]
impl crate::Resettable for DMA_INT_ST_SPEC {}
