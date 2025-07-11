#[doc = "Register `SPI_DMA_CONF` reader"]
pub type R = crate::R<SPI_DMA_CONF_SPEC>;
#[doc = "Register `SPI_DMA_CONF` writer"]
pub type W = crate::W<SPI_DMA_CONF_SPEC>;
#[doc = "Field `SPI_DMA_OUTFIFO_EMPTY` reader - Represents whether or not the DMA TX FIFO is ready for sending data.\\\\ 0: Ready\\\\ 1: Not ready\\\\"]
pub type SPI_DMA_OUTFIFO_EMPTY_R = crate::BitReader;
#[doc = "Field `SPI_DMA_INFIFO_FULL` reader - Represents whether or not the DMA RX FIFO is ready for receiving data.\\\\ 0: Ready\\\\ 1: Not ready\\\\"]
pub type SPI_DMA_INFIFO_FULL_R = crate::BitReader;
#[doc = "Field `SPI_DMA_SLV_SEG_TRANS_EN` reader - Configures whether or not to enable DMA-controlled segmented transfer in slave half-duplex communication.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
pub type SPI_DMA_SLV_SEG_TRANS_EN_R = crate::BitReader;
#[doc = "Field `SPI_DMA_SLV_SEG_TRANS_EN` writer - Configures whether or not to enable DMA-controlled segmented transfer in slave half-duplex communication.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
pub type SPI_DMA_SLV_SEG_TRANS_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_SLV_RX_SEG_TRANS_CLR_EN` reader - In slave segmented transfer, if the size of the DMA RX buffer is smaller than the size of the received data, \\\\1: the data in all the following Wr_DMA transactions will not be received\\\\ 0: the data in this Wr_DMA transaction will not be received, but in the following transactions,\\\\ - if the size of DMA RX buffer is not 0, the data in following Wr_DMA transactions will be received. - if the size of DMA RX buffer is 0, the data in following Wr_DMA transactions will not be received."]
pub type SPI_SLV_RX_SEG_TRANS_CLR_EN_R = crate::BitReader;
#[doc = "Field `SPI_SLV_RX_SEG_TRANS_CLR_EN` writer - In slave segmented transfer, if the size of the DMA RX buffer is smaller than the size of the received data, \\\\1: the data in all the following Wr_DMA transactions will not be received\\\\ 0: the data in this Wr_DMA transaction will not be received, but in the following transactions,\\\\ - if the size of DMA RX buffer is not 0, the data in following Wr_DMA transactions will be received. - if the size of DMA RX buffer is 0, the data in following Wr_DMA transactions will not be received."]
pub type SPI_SLV_RX_SEG_TRANS_CLR_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_SLV_TX_SEG_TRANS_CLR_EN` reader - In slave segmented transfer, if the size of the DMA TX buffer is smaller than the size of the transmitted data,\\\\ 1: the data in the following transactions will not be updated, i.e. the old data is transmitted repeatedly.\\\\ 0: the data in this transaction will not be updated. But in the following transactions,\\\\ - if new data is filled in DMA TX FIFO, new data will be transmitted. - if no new data is filled in DMA TX FIFO, no new data will be transmitted."]
pub type SPI_SLV_TX_SEG_TRANS_CLR_EN_R = crate::BitReader;
#[doc = "Field `SPI_SLV_TX_SEG_TRANS_CLR_EN` writer - In slave segmented transfer, if the size of the DMA TX buffer is smaller than the size of the transmitted data,\\\\ 1: the data in the following transactions will not be updated, i.e. the old data is transmitted repeatedly.\\\\ 0: the data in this transaction will not be updated. But in the following transactions,\\\\ - if new data is filled in DMA TX FIFO, new data will be transmitted. - if no new data is filled in DMA TX FIFO, no new data will be transmitted."]
pub type SPI_SLV_TX_SEG_TRANS_CLR_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_RX_EOF_EN` reader - 1: In a DAM-controlled transfer, if the bit number of transferred data is equal to (SPI_MS_DATA_BITLEN + 1), then GDMA_IN_SUC_EOF_CH\\it{n}_INT_RAW will be set by hardware. 0: GDMA_IN_SUC_EOF_CH\\it{n}_INT_RAW is set by SPI_TRANS_DONE_INT event in a single transfer, or by an SPI_DMA_SEG_TRANS_DONE_INT event in a segmented transfer."]
pub type SPI_RX_EOF_EN_R = crate::BitReader;
#[doc = "Field `SPI_RX_EOF_EN` writer - 1: In a DAM-controlled transfer, if the bit number of transferred data is equal to (SPI_MS_DATA_BITLEN + 1), then GDMA_IN_SUC_EOF_CH\\it{n}_INT_RAW will be set by hardware. 0: GDMA_IN_SUC_EOF_CH\\it{n}_INT_RAW is set by SPI_TRANS_DONE_INT event in a single transfer, or by an SPI_DMA_SEG_TRANS_DONE_INT event in a segmented transfer."]
pub type SPI_RX_EOF_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_DMA_RX_ENA` reader - Configures whether or not to enable DMA-controlled receive data transfer.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
pub type SPI_DMA_RX_ENA_R = crate::BitReader;
#[doc = "Field `SPI_DMA_RX_ENA` writer - Configures whether or not to enable DMA-controlled receive data transfer.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
pub type SPI_DMA_RX_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_DMA_TX_ENA` reader - Configures whether or not to enable DMA-controlled send data transfer.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
pub type SPI_DMA_TX_ENA_R = crate::BitReader;
#[doc = "Field `SPI_DMA_TX_ENA` writer - Configures whether or not to enable DMA-controlled send data transfer.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
pub type SPI_DMA_TX_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_RX_AFIFO_RST` writer - Configures whether or not to reset spi_rx_afifo as shown in Figure <a href='fig:spi-master-data-flow-control'>link</a> and in Figure <a href='fig:spi-slave-data-flow-control'>link</a>.\\\\ 0: Not reset\\\\ 1: Reset\\\\ spi_rx_afifo is used to receive data in SPI master and slave transfer."]
pub type SPI_RX_AFIFO_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_BUF_AFIFO_RST` writer - Configures whether or not to reset buf_tx_afifo as shown in Figure <a href='fig:spi-master-data-flow-control'>link</a> and in Figure <a href='fig:spi-slave-data-flow-control'>link</a>.\\\\ 0: Not reset\\\\ 1: Reset\\\\ buf_tx_afifo is used to send data out in CPU-controlled master and slave transfer."]
pub type SPI_BUF_AFIFO_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_DMA_AFIFO_RST` writer - Configures whether or not to reset dma_tx_afifo as shown in Figure <a href='fig:spi-master-data-flow-control'>link</a> and in Figure <a href='fig:spi-slave-data-flow-control'>link</a>.\\\\ 0: Not reset\\\\ 1: Reset\\\\ dma_tx_afifo is used to send data out in DMA-controlled slave transfer."]
pub type SPI_DMA_AFIFO_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Represents whether or not the DMA TX FIFO is ready for sending data.\\\\ 0: Ready\\\\ 1: Not ready\\\\"]
    #[inline(always)]
    pub fn spi_dma_outfifo_empty(&self) -> SPI_DMA_OUTFIFO_EMPTY_R {
        SPI_DMA_OUTFIFO_EMPTY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Represents whether or not the DMA RX FIFO is ready for receiving data.\\\\ 0: Ready\\\\ 1: Not ready\\\\"]
    #[inline(always)]
    pub fn spi_dma_infifo_full(&self) -> SPI_DMA_INFIFO_FULL_R {
        SPI_DMA_INFIFO_FULL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 18 - Configures whether or not to enable DMA-controlled segmented transfer in slave half-duplex communication.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn spi_dma_slv_seg_trans_en(&self) -> SPI_DMA_SLV_SEG_TRANS_EN_R {
        SPI_DMA_SLV_SEG_TRANS_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - In slave segmented transfer, if the size of the DMA RX buffer is smaller than the size of the received data, \\\\1: the data in all the following Wr_DMA transactions will not be received\\\\ 0: the data in this Wr_DMA transaction will not be received, but in the following transactions,\\\\ - if the size of DMA RX buffer is not 0, the data in following Wr_DMA transactions will be received. - if the size of DMA RX buffer is 0, the data in following Wr_DMA transactions will not be received."]
    #[inline(always)]
    pub fn spi_slv_rx_seg_trans_clr_en(&self) -> SPI_SLV_RX_SEG_TRANS_CLR_EN_R {
        SPI_SLV_RX_SEG_TRANS_CLR_EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - In slave segmented transfer, if the size of the DMA TX buffer is smaller than the size of the transmitted data,\\\\ 1: the data in the following transactions will not be updated, i.e. the old data is transmitted repeatedly.\\\\ 0: the data in this transaction will not be updated. But in the following transactions,\\\\ - if new data is filled in DMA TX FIFO, new data will be transmitted. - if no new data is filled in DMA TX FIFO, no new data will be transmitted."]
    #[inline(always)]
    pub fn spi_slv_tx_seg_trans_clr_en(&self) -> SPI_SLV_TX_SEG_TRANS_CLR_EN_R {
        SPI_SLV_TX_SEG_TRANS_CLR_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 1: In a DAM-controlled transfer, if the bit number of transferred data is equal to (SPI_MS_DATA_BITLEN + 1), then GDMA_IN_SUC_EOF_CH\\it{n}_INT_RAW will be set by hardware. 0: GDMA_IN_SUC_EOF_CH\\it{n}_INT_RAW is set by SPI_TRANS_DONE_INT event in a single transfer, or by an SPI_DMA_SEG_TRANS_DONE_INT event in a segmented transfer."]
    #[inline(always)]
    pub fn spi_rx_eof_en(&self) -> SPI_RX_EOF_EN_R {
        SPI_RX_EOF_EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 27 - Configures whether or not to enable DMA-controlled receive data transfer.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn spi_dma_rx_ena(&self) -> SPI_DMA_RX_ENA_R {
        SPI_DMA_RX_ENA_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Configures whether or not to enable DMA-controlled send data transfer.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn spi_dma_tx_ena(&self) -> SPI_DMA_TX_ENA_R {
        SPI_DMA_TX_ENA_R::new(((self.bits >> 28) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_DMA_CONF")
            .field("spi_dma_outfifo_empty", &self.spi_dma_outfifo_empty())
            .field("spi_dma_infifo_full", &self.spi_dma_infifo_full())
            .field("spi_dma_slv_seg_trans_en", &self.spi_dma_slv_seg_trans_en())
            .field(
                "spi_slv_rx_seg_trans_clr_en",
                &self.spi_slv_rx_seg_trans_clr_en(),
            )
            .field(
                "spi_slv_tx_seg_trans_clr_en",
                &self.spi_slv_tx_seg_trans_clr_en(),
            )
            .field("spi_rx_eof_en", &self.spi_rx_eof_en())
            .field("spi_dma_rx_ena", &self.spi_dma_rx_ena())
            .field("spi_dma_tx_ena", &self.spi_dma_tx_ena())
            .finish()
    }
}
impl W {
    #[doc = "Bit 18 - Configures whether or not to enable DMA-controlled segmented transfer in slave half-duplex communication.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn spi_dma_slv_seg_trans_en(&mut self) -> SPI_DMA_SLV_SEG_TRANS_EN_W<SPI_DMA_CONF_SPEC> {
        SPI_DMA_SLV_SEG_TRANS_EN_W::new(self, 18)
    }
    #[doc = "Bit 19 - In slave segmented transfer, if the size of the DMA RX buffer is smaller than the size of the received data, \\\\1: the data in all the following Wr_DMA transactions will not be received\\\\ 0: the data in this Wr_DMA transaction will not be received, but in the following transactions,\\\\ - if the size of DMA RX buffer is not 0, the data in following Wr_DMA transactions will be received. - if the size of DMA RX buffer is 0, the data in following Wr_DMA transactions will not be received."]
    #[inline(always)]
    pub fn spi_slv_rx_seg_trans_clr_en(
        &mut self,
    ) -> SPI_SLV_RX_SEG_TRANS_CLR_EN_W<SPI_DMA_CONF_SPEC> {
        SPI_SLV_RX_SEG_TRANS_CLR_EN_W::new(self, 19)
    }
    #[doc = "Bit 20 - In slave segmented transfer, if the size of the DMA TX buffer is smaller than the size of the transmitted data,\\\\ 1: the data in the following transactions will not be updated, i.e. the old data is transmitted repeatedly.\\\\ 0: the data in this transaction will not be updated. But in the following transactions,\\\\ - if new data is filled in DMA TX FIFO, new data will be transmitted. - if no new data is filled in DMA TX FIFO, no new data will be transmitted."]
    #[inline(always)]
    pub fn spi_slv_tx_seg_trans_clr_en(
        &mut self,
    ) -> SPI_SLV_TX_SEG_TRANS_CLR_EN_W<SPI_DMA_CONF_SPEC> {
        SPI_SLV_TX_SEG_TRANS_CLR_EN_W::new(self, 20)
    }
    #[doc = "Bit 21 - 1: In a DAM-controlled transfer, if the bit number of transferred data is equal to (SPI_MS_DATA_BITLEN + 1), then GDMA_IN_SUC_EOF_CH\\it{n}_INT_RAW will be set by hardware. 0: GDMA_IN_SUC_EOF_CH\\it{n}_INT_RAW is set by SPI_TRANS_DONE_INT event in a single transfer, or by an SPI_DMA_SEG_TRANS_DONE_INT event in a segmented transfer."]
    #[inline(always)]
    pub fn spi_rx_eof_en(&mut self) -> SPI_RX_EOF_EN_W<SPI_DMA_CONF_SPEC> {
        SPI_RX_EOF_EN_W::new(self, 21)
    }
    #[doc = "Bit 27 - Configures whether or not to enable DMA-controlled receive data transfer.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn spi_dma_rx_ena(&mut self) -> SPI_DMA_RX_ENA_W<SPI_DMA_CONF_SPEC> {
        SPI_DMA_RX_ENA_W::new(self, 27)
    }
    #[doc = "Bit 28 - Configures whether or not to enable DMA-controlled send data transfer.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn spi_dma_tx_ena(&mut self) -> SPI_DMA_TX_ENA_W<SPI_DMA_CONF_SPEC> {
        SPI_DMA_TX_ENA_W::new(self, 28)
    }
    #[doc = "Bit 29 - Configures whether or not to reset spi_rx_afifo as shown in Figure <a href='fig:spi-master-data-flow-control'>link</a> and in Figure <a href='fig:spi-slave-data-flow-control'>link</a>.\\\\ 0: Not reset\\\\ 1: Reset\\\\ spi_rx_afifo is used to receive data in SPI master and slave transfer."]
    #[inline(always)]
    pub fn spi_rx_afifo_rst(&mut self) -> SPI_RX_AFIFO_RST_W<SPI_DMA_CONF_SPEC> {
        SPI_RX_AFIFO_RST_W::new(self, 29)
    }
    #[doc = "Bit 30 - Configures whether or not to reset buf_tx_afifo as shown in Figure <a href='fig:spi-master-data-flow-control'>link</a> and in Figure <a href='fig:spi-slave-data-flow-control'>link</a>.\\\\ 0: Not reset\\\\ 1: Reset\\\\ buf_tx_afifo is used to send data out in CPU-controlled master and slave transfer."]
    #[inline(always)]
    pub fn spi_buf_afifo_rst(&mut self) -> SPI_BUF_AFIFO_RST_W<SPI_DMA_CONF_SPEC> {
        SPI_BUF_AFIFO_RST_W::new(self, 30)
    }
    #[doc = "Bit 31 - Configures whether or not to reset dma_tx_afifo as shown in Figure <a href='fig:spi-master-data-flow-control'>link</a> and in Figure <a href='fig:spi-slave-data-flow-control'>link</a>.\\\\ 0: Not reset\\\\ 1: Reset\\\\ dma_tx_afifo is used to send data out in DMA-controlled slave transfer."]
    #[inline(always)]
    pub fn spi_dma_afifo_rst(&mut self) -> SPI_DMA_AFIFO_RST_W<SPI_DMA_CONF_SPEC> {
        SPI_DMA_AFIFO_RST_W::new(self, 31)
    }
}
#[doc = "SPI DMA control register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_dma_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_dma_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_DMA_CONF_SPEC;
impl crate::RegisterSpec for SPI_DMA_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_dma_conf::R`](R) reader structure"]
impl crate::Readable for SPI_DMA_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_dma_conf::W`](W) writer structure"]
impl crate::Writable for SPI_DMA_CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI_DMA_CONF to value 0x03"]
impl crate::Resettable for SPI_DMA_CONF_SPEC {
    const RESET_VALUE: u32 = 0x03;
}
