#[doc = "Register `DMA_INT_RAW` reader"]
pub type R = crate::R<DMA_INT_RAW_SPEC>;
#[doc = "Register `DMA_INT_RAW` writer"]
pub type W = crate::W<DMA_INT_RAW_SPEC>;
#[doc = "Field `LP_REG_SLV_RD_BUF_DONE_INT_RAW` reader - The raw bit for SPI_SLV_RD_BUF_DONE_INT interrupt. 1: SPI slave mode Rd_BUF transmission is ended. 0: Others."]
pub type LP_REG_SLV_RD_BUF_DONE_INT_RAW_R = crate::BitReader;
#[doc = "Field `LP_REG_SLV_RD_BUF_DONE_INT_RAW` writer - The raw bit for SPI_SLV_RD_BUF_DONE_INT interrupt. 1: SPI slave mode Rd_BUF transmission is ended. 0: Others."]
pub type LP_REG_SLV_RD_BUF_DONE_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_REG_SLV_WR_BUF_DONE_INT_RAW` reader - The raw bit for SPI_SLV_WR_BUF_DONE_INT interrupt. 1: SPI slave mode Wr_BUF transmission is ended. 0: Others."]
pub type LP_REG_SLV_WR_BUF_DONE_INT_RAW_R = crate::BitReader;
#[doc = "Field `LP_REG_SLV_WR_BUF_DONE_INT_RAW` writer - The raw bit for SPI_SLV_WR_BUF_DONE_INT interrupt. 1: SPI slave mode Wr_BUF transmission is ended. 0: Others."]
pub type LP_REG_SLV_WR_BUF_DONE_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_REG_TRANS_DONE_INT_RAW` reader - The raw bit for SPI_TRANS_DONE_INT interrupt. 1: SPI master mode transmission is ended. 0: others."]
pub type LP_REG_TRANS_DONE_INT_RAW_R = crate::BitReader;
#[doc = "Field `LP_REG_TRANS_DONE_INT_RAW` writer - The raw bit for SPI_TRANS_DONE_INT interrupt. 1: SPI master mode transmission is ended. 0: others."]
pub type LP_REG_TRANS_DONE_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_REG_SPI_WAKEUP_INT_RAW` reader - The raw bit for SPI_SPI_WAKEUP_INT interrupt. 1: There is a wake up signal when low power mode. 0: Others."]
pub type LP_REG_SPI_WAKEUP_INT_RAW_R = crate::BitReader;
#[doc = "Field `LP_REG_SPI_WAKEUP_INT_RAW` writer - The raw bit for SPI_SPI_WAKEUP_INT interrupt. 1: There is a wake up signal when low power mode. 0: Others."]
pub type LP_REG_SPI_WAKEUP_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_REG_SLV_BUF_ADDR_ERR_INT_RAW` reader - The raw bit for SPI_SLV_BUF_ADDR_ERR_INT interrupt. 1: The accessing data address of the current SPI slave mode CPU controlled FD, Wr_BUF or Rd_BUF transmission is bigger than 63. 0: Others."]
pub type LP_REG_SLV_BUF_ADDR_ERR_INT_RAW_R = crate::BitReader;
#[doc = "Field `LP_REG_SLV_BUF_ADDR_ERR_INT_RAW` writer - The raw bit for SPI_SLV_BUF_ADDR_ERR_INT interrupt. 1: The accessing data address of the current SPI slave mode CPU controlled FD, Wr_BUF or Rd_BUF transmission is bigger than 63. 0: Others."]
pub type LP_REG_SLV_BUF_ADDR_ERR_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_REG_SLV_CMD_ERR_INT_RAW` reader - The raw bit for SPI_SLV_CMD_ERR_INT interrupt. 1: The slave command value in the current SPI slave HD mode transmission is not supported. 0: Others."]
pub type LP_REG_SLV_CMD_ERR_INT_RAW_R = crate::BitReader;
#[doc = "Field `LP_REG_SLV_CMD_ERR_INT_RAW` writer - The raw bit for SPI_SLV_CMD_ERR_INT interrupt. 1: The slave command value in the current SPI slave HD mode transmission is not supported. 0: Others."]
pub type LP_REG_SLV_CMD_ERR_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_REG_MST_RX_AFIFO_WFULL_ERR_INT_RAW` reader - The raw bit for SPI_MST_RX_AFIFO_WFULL_ERR_INT interrupt. 1: There is a RX AFIFO write-full error when SPI inputs data in master mode. 0: Others."]
pub type LP_REG_MST_RX_AFIFO_WFULL_ERR_INT_RAW_R = crate::BitReader;
#[doc = "Field `LP_REG_MST_RX_AFIFO_WFULL_ERR_INT_RAW` writer - The raw bit for SPI_MST_RX_AFIFO_WFULL_ERR_INT interrupt. 1: There is a RX AFIFO write-full error when SPI inputs data in master mode. 0: Others."]
pub type LP_REG_MST_RX_AFIFO_WFULL_ERR_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_REG_MST_TX_AFIFO_REMPTY_ERR_INT_RAW` reader - The raw bit for SPI_MST_TX_AFIFO_REMPTY_ERR_INT interrupt. 1: There is a TX BUF AFIFO read-empty error when SPI outputs data in master mode. 0: Others."]
pub type LP_REG_MST_TX_AFIFO_REMPTY_ERR_INT_RAW_R = crate::BitReader;
#[doc = "Field `LP_REG_MST_TX_AFIFO_REMPTY_ERR_INT_RAW` writer - The raw bit for SPI_MST_TX_AFIFO_REMPTY_ERR_INT interrupt. 1: There is a TX BUF AFIFO read-empty error when SPI outputs data in master mode. 0: Others."]
pub type LP_REG_MST_TX_AFIFO_REMPTY_ERR_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_REG_APP2_INT_RAW` reader - The raw bit for SPI_APP2_INT interrupt. The value is only controlled by application."]
pub type LP_REG_APP2_INT_RAW_R = crate::BitReader;
#[doc = "Field `LP_REG_APP2_INT_RAW` writer - The raw bit for SPI_APP2_INT interrupt. The value is only controlled by application."]
pub type LP_REG_APP2_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_REG_APP1_INT_RAW` reader - The raw bit for SPI_APP1_INT interrupt. The value is only controlled by application."]
pub type LP_REG_APP1_INT_RAW_R = crate::BitReader;
#[doc = "Field `LP_REG_APP1_INT_RAW` writer - The raw bit for SPI_APP1_INT interrupt. The value is only controlled by application."]
pub type LP_REG_APP1_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 10 - The raw bit for SPI_SLV_RD_BUF_DONE_INT interrupt. 1: SPI slave mode Rd_BUF transmission is ended. 0: Others."]
    #[inline(always)]
    pub fn lp_reg_slv_rd_buf_done_int_raw(&self) -> LP_REG_SLV_RD_BUF_DONE_INT_RAW_R {
        LP_REG_SLV_RD_BUF_DONE_INT_RAW_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The raw bit for SPI_SLV_WR_BUF_DONE_INT interrupt. 1: SPI slave mode Wr_BUF transmission is ended. 0: Others."]
    #[inline(always)]
    pub fn lp_reg_slv_wr_buf_done_int_raw(&self) -> LP_REG_SLV_WR_BUF_DONE_INT_RAW_R {
        LP_REG_SLV_WR_BUF_DONE_INT_RAW_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - The raw bit for SPI_TRANS_DONE_INT interrupt. 1: SPI master mode transmission is ended. 0: others."]
    #[inline(always)]
    pub fn lp_reg_trans_done_int_raw(&self) -> LP_REG_TRANS_DONE_INT_RAW_R {
        LP_REG_TRANS_DONE_INT_RAW_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - The raw bit for SPI_SPI_WAKEUP_INT interrupt. 1: There is a wake up signal when low power mode. 0: Others."]
    #[inline(always)]
    pub fn lp_reg_spi_wakeup_int_raw(&self) -> LP_REG_SPI_WAKEUP_INT_RAW_R {
        LP_REG_SPI_WAKEUP_INT_RAW_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - The raw bit for SPI_SLV_BUF_ADDR_ERR_INT interrupt. 1: The accessing data address of the current SPI slave mode CPU controlled FD, Wr_BUF or Rd_BUF transmission is bigger than 63. 0: Others."]
    #[inline(always)]
    pub fn lp_reg_slv_buf_addr_err_int_raw(&self) -> LP_REG_SLV_BUF_ADDR_ERR_INT_RAW_R {
        LP_REG_SLV_BUF_ADDR_ERR_INT_RAW_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - The raw bit for SPI_SLV_CMD_ERR_INT interrupt. 1: The slave command value in the current SPI slave HD mode transmission is not supported. 0: Others."]
    #[inline(always)]
    pub fn lp_reg_slv_cmd_err_int_raw(&self) -> LP_REG_SLV_CMD_ERR_INT_RAW_R {
        LP_REG_SLV_CMD_ERR_INT_RAW_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - The raw bit for SPI_MST_RX_AFIFO_WFULL_ERR_INT interrupt. 1: There is a RX AFIFO write-full error when SPI inputs data in master mode. 0: Others."]
    #[inline(always)]
    pub fn lp_reg_mst_rx_afifo_wfull_err_int_raw(&self) -> LP_REG_MST_RX_AFIFO_WFULL_ERR_INT_RAW_R {
        LP_REG_MST_RX_AFIFO_WFULL_ERR_INT_RAW_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - The raw bit for SPI_MST_TX_AFIFO_REMPTY_ERR_INT interrupt. 1: There is a TX BUF AFIFO read-empty error when SPI outputs data in master mode. 0: Others."]
    #[inline(always)]
    pub fn lp_reg_mst_tx_afifo_rempty_err_int_raw(
        &self,
    ) -> LP_REG_MST_TX_AFIFO_REMPTY_ERR_INT_RAW_R {
        LP_REG_MST_TX_AFIFO_REMPTY_ERR_INT_RAW_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - The raw bit for SPI_APP2_INT interrupt. The value is only controlled by application."]
    #[inline(always)]
    pub fn lp_reg_app2_int_raw(&self) -> LP_REG_APP2_INT_RAW_R {
        LP_REG_APP2_INT_RAW_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - The raw bit for SPI_APP1_INT interrupt. The value is only controlled by application."]
    #[inline(always)]
    pub fn lp_reg_app1_int_raw(&self) -> LP_REG_APP1_INT_RAW_R {
        LP_REG_APP1_INT_RAW_R::new(((self.bits >> 20) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_INT_RAW")
            .field(
                "lp_reg_slv_rd_buf_done_int_raw",
                &self.lp_reg_slv_rd_buf_done_int_raw(),
            )
            .field(
                "lp_reg_slv_wr_buf_done_int_raw",
                &self.lp_reg_slv_wr_buf_done_int_raw(),
            )
            .field(
                "lp_reg_trans_done_int_raw",
                &self.lp_reg_trans_done_int_raw(),
            )
            .field(
                "lp_reg_spi_wakeup_int_raw",
                &self.lp_reg_spi_wakeup_int_raw(),
            )
            .field(
                "lp_reg_slv_buf_addr_err_int_raw",
                &self.lp_reg_slv_buf_addr_err_int_raw(),
            )
            .field(
                "lp_reg_slv_cmd_err_int_raw",
                &self.lp_reg_slv_cmd_err_int_raw(),
            )
            .field(
                "lp_reg_mst_rx_afifo_wfull_err_int_raw",
                &self.lp_reg_mst_rx_afifo_wfull_err_int_raw(),
            )
            .field(
                "lp_reg_mst_tx_afifo_rempty_err_int_raw",
                &self.lp_reg_mst_tx_afifo_rempty_err_int_raw(),
            )
            .field("lp_reg_app2_int_raw", &self.lp_reg_app2_int_raw())
            .field("lp_reg_app1_int_raw", &self.lp_reg_app1_int_raw())
            .finish()
    }
}
impl W {
    #[doc = "Bit 10 - The raw bit for SPI_SLV_RD_BUF_DONE_INT interrupt. 1: SPI slave mode Rd_BUF transmission is ended. 0: Others."]
    #[inline(always)]
    pub fn lp_reg_slv_rd_buf_done_int_raw(
        &mut self,
    ) -> LP_REG_SLV_RD_BUF_DONE_INT_RAW_W<'_, DMA_INT_RAW_SPEC> {
        LP_REG_SLV_RD_BUF_DONE_INT_RAW_W::new(self, 10)
    }
    #[doc = "Bit 11 - The raw bit for SPI_SLV_WR_BUF_DONE_INT interrupt. 1: SPI slave mode Wr_BUF transmission is ended. 0: Others."]
    #[inline(always)]
    pub fn lp_reg_slv_wr_buf_done_int_raw(
        &mut self,
    ) -> LP_REG_SLV_WR_BUF_DONE_INT_RAW_W<'_, DMA_INT_RAW_SPEC> {
        LP_REG_SLV_WR_BUF_DONE_INT_RAW_W::new(self, 11)
    }
    #[doc = "Bit 12 - The raw bit for SPI_TRANS_DONE_INT interrupt. 1: SPI master mode transmission is ended. 0: others."]
    #[inline(always)]
    pub fn lp_reg_trans_done_int_raw(
        &mut self,
    ) -> LP_REG_TRANS_DONE_INT_RAW_W<'_, DMA_INT_RAW_SPEC> {
        LP_REG_TRANS_DONE_INT_RAW_W::new(self, 12)
    }
    #[doc = "Bit 14 - The raw bit for SPI_SPI_WAKEUP_INT interrupt. 1: There is a wake up signal when low power mode. 0: Others."]
    #[inline(always)]
    pub fn lp_reg_spi_wakeup_int_raw(
        &mut self,
    ) -> LP_REG_SPI_WAKEUP_INT_RAW_W<'_, DMA_INT_RAW_SPEC> {
        LP_REG_SPI_WAKEUP_INT_RAW_W::new(self, 14)
    }
    #[doc = "Bit 15 - The raw bit for SPI_SLV_BUF_ADDR_ERR_INT interrupt. 1: The accessing data address of the current SPI slave mode CPU controlled FD, Wr_BUF or Rd_BUF transmission is bigger than 63. 0: Others."]
    #[inline(always)]
    pub fn lp_reg_slv_buf_addr_err_int_raw(
        &mut self,
    ) -> LP_REG_SLV_BUF_ADDR_ERR_INT_RAW_W<'_, DMA_INT_RAW_SPEC> {
        LP_REG_SLV_BUF_ADDR_ERR_INT_RAW_W::new(self, 15)
    }
    #[doc = "Bit 16 - The raw bit for SPI_SLV_CMD_ERR_INT interrupt. 1: The slave command value in the current SPI slave HD mode transmission is not supported. 0: Others."]
    #[inline(always)]
    pub fn lp_reg_slv_cmd_err_int_raw(
        &mut self,
    ) -> LP_REG_SLV_CMD_ERR_INT_RAW_W<'_, DMA_INT_RAW_SPEC> {
        LP_REG_SLV_CMD_ERR_INT_RAW_W::new(self, 16)
    }
    #[doc = "Bit 17 - The raw bit for SPI_MST_RX_AFIFO_WFULL_ERR_INT interrupt. 1: There is a RX AFIFO write-full error when SPI inputs data in master mode. 0: Others."]
    #[inline(always)]
    pub fn lp_reg_mst_rx_afifo_wfull_err_int_raw(
        &mut self,
    ) -> LP_REG_MST_RX_AFIFO_WFULL_ERR_INT_RAW_W<'_, DMA_INT_RAW_SPEC> {
        LP_REG_MST_RX_AFIFO_WFULL_ERR_INT_RAW_W::new(self, 17)
    }
    #[doc = "Bit 18 - The raw bit for SPI_MST_TX_AFIFO_REMPTY_ERR_INT interrupt. 1: There is a TX BUF AFIFO read-empty error when SPI outputs data in master mode. 0: Others."]
    #[inline(always)]
    pub fn lp_reg_mst_tx_afifo_rempty_err_int_raw(
        &mut self,
    ) -> LP_REG_MST_TX_AFIFO_REMPTY_ERR_INT_RAW_W<'_, DMA_INT_RAW_SPEC> {
        LP_REG_MST_TX_AFIFO_REMPTY_ERR_INT_RAW_W::new(self, 18)
    }
    #[doc = "Bit 19 - The raw bit for SPI_APP2_INT interrupt. The value is only controlled by application."]
    #[inline(always)]
    pub fn lp_reg_app2_int_raw(&mut self) -> LP_REG_APP2_INT_RAW_W<'_, DMA_INT_RAW_SPEC> {
        LP_REG_APP2_INT_RAW_W::new(self, 19)
    }
    #[doc = "Bit 20 - The raw bit for SPI_APP1_INT interrupt. The value is only controlled by application."]
    #[inline(always)]
    pub fn lp_reg_app1_int_raw(&mut self) -> LP_REG_APP1_INT_RAW_W<'_, DMA_INT_RAW_SPEC> {
        LP_REG_APP1_INT_RAW_W::new(self, 20)
    }
}
#[doc = "SPI DMA interrupt raw register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_int_raw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_int_raw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMA_INT_RAW_SPEC;
impl crate::RegisterSpec for DMA_INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_int_raw::R`](R) reader structure"]
impl crate::Readable for DMA_INT_RAW_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dma_int_raw::W`](W) writer structure"]
impl crate::Writable for DMA_INT_RAW_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA_INT_RAW to value 0"]
impl crate::Resettable for DMA_INT_RAW_SPEC {}
