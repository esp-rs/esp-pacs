#[doc = "Register `DMA_INT_SET` writer"]
pub type W = crate::W<DMA_INT_SET_SPEC>;
#[doc = "Field `LP_SPI_SLV_RD_BUF_DONE_INT_SET` writer - "]
pub type LP_SPI_SLV_RD_BUF_DONE_INT_SET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_SPI_SLV_WR_BUF_DONE_INT_SET` writer - "]
pub type LP_SPI_SLV_WR_BUF_DONE_INT_SET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_SPI_TRANS_DONE_INT_SET` writer - "]
pub type LP_SPI_TRANS_DONE_INT_SET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_SPI_SLV_BUF_ADDR_ERR_INT_SET` writer - "]
pub type LP_SPI_SLV_BUF_ADDR_ERR_INT_SET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_SPI_SLV_CMD_ERR_INT_SET` writer - "]
pub type LP_SPI_SLV_CMD_ERR_INT_SET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_SPI_MST_RX_AFIFO_WFULL_ERR_INT_SET` writer - "]
pub type LP_SPI_MST_RX_AFIFO_WFULL_ERR_INT_SET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_SPI_MST_TX_AFIFO_REMPTY_ERR_INT_SET` writer - "]
pub type LP_SPI_MST_TX_AFIFO_REMPTY_ERR_INT_SET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_SPI_APP2_INT_SET` writer - "]
pub type LP_SPI_APP2_INT_SET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_SPI_APP1_INT_SET` writer - "]
pub type LP_SPI_APP1_INT_SET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DMA_INT_SET_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn lp_spi_slv_rd_buf_done_int_set(
        &mut self,
    ) -> LP_SPI_SLV_RD_BUF_DONE_INT_SET_W<'_, DMA_INT_SET_SPEC> {
        LP_SPI_SLV_RD_BUF_DONE_INT_SET_W::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn lp_spi_slv_wr_buf_done_int_set(
        &mut self,
    ) -> LP_SPI_SLV_WR_BUF_DONE_INT_SET_W<'_, DMA_INT_SET_SPEC> {
        LP_SPI_SLV_WR_BUF_DONE_INT_SET_W::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn lp_spi_trans_done_int_set(
        &mut self,
    ) -> LP_SPI_TRANS_DONE_INT_SET_W<'_, DMA_INT_SET_SPEC> {
        LP_SPI_TRANS_DONE_INT_SET_W::new(self, 12)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn lp_spi_slv_buf_addr_err_int_set(
        &mut self,
    ) -> LP_SPI_SLV_BUF_ADDR_ERR_INT_SET_W<'_, DMA_INT_SET_SPEC> {
        LP_SPI_SLV_BUF_ADDR_ERR_INT_SET_W::new(self, 15)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn lp_spi_slv_cmd_err_int_set(
        &mut self,
    ) -> LP_SPI_SLV_CMD_ERR_INT_SET_W<'_, DMA_INT_SET_SPEC> {
        LP_SPI_SLV_CMD_ERR_INT_SET_W::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn lp_spi_mst_rx_afifo_wfull_err_int_set(
        &mut self,
    ) -> LP_SPI_MST_RX_AFIFO_WFULL_ERR_INT_SET_W<'_, DMA_INT_SET_SPEC> {
        LP_SPI_MST_RX_AFIFO_WFULL_ERR_INT_SET_W::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn lp_spi_mst_tx_afifo_rempty_err_int_set(
        &mut self,
    ) -> LP_SPI_MST_TX_AFIFO_REMPTY_ERR_INT_SET_W<'_, DMA_INT_SET_SPEC> {
        LP_SPI_MST_TX_AFIFO_REMPTY_ERR_INT_SET_W::new(self, 18)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn lp_spi_app2_int_set(&mut self) -> LP_SPI_APP2_INT_SET_W<'_, DMA_INT_SET_SPEC> {
        LP_SPI_APP2_INT_SET_W::new(self, 19)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn lp_spi_app1_int_set(&mut self) -> LP_SPI_APP1_INT_SET_W<'_, DMA_INT_SET_SPEC> {
        LP_SPI_APP1_INT_SET_W::new(self, 20)
    }
}
#[doc = "\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_int_set::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMA_INT_SET_SPEC;
impl crate::RegisterSpec for DMA_INT_SET_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dma_int_set::W`](W) writer structure"]
impl crate::Writable for DMA_INT_SET_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA_INT_SET to value 0"]
impl crate::Resettable for DMA_INT_SET_SPEC {}
