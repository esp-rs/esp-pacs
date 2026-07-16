#[doc = "Register `DMA_INT_RAW` reader"]
pub type R = crate::R<DMA_INT_RAW_SPEC>;
#[doc = "Register `DMA_INT_RAW` writer"]
pub type W = crate::W<DMA_INT_RAW_SPEC>;
#[doc = "Field `SLV_RD_BUF_DONE_INT_RAW` reader - "]
pub type SLV_RD_BUF_DONE_INT_RAW_R = crate::BitReader;
#[doc = "Field `SLV_RD_BUF_DONE_INT_RAW` writer - "]
pub type SLV_RD_BUF_DONE_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLV_WR_BUF_DONE_INT_RAW` reader - "]
pub type SLV_WR_BUF_DONE_INT_RAW_R = crate::BitReader;
#[doc = "Field `SLV_WR_BUF_DONE_INT_RAW` writer - "]
pub type SLV_WR_BUF_DONE_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRANS_DONE_INT_RAW` reader - "]
pub type TRANS_DONE_INT_RAW_R = crate::BitReader;
#[doc = "Field `TRANS_DONE_INT_RAW` writer - "]
pub type TRANS_DONE_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_WAKEUP_INT_RAW` reader - "]
pub type SPI_WAKEUP_INT_RAW_R = crate::BitReader;
#[doc = "Field `SPI_WAKEUP_INT_RAW` writer - "]
pub type SPI_WAKEUP_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLV_BUF_ADDR_ERR_INT_RAW` reader - "]
pub type SLV_BUF_ADDR_ERR_INT_RAW_R = crate::BitReader;
#[doc = "Field `SLV_BUF_ADDR_ERR_INT_RAW` writer - "]
pub type SLV_BUF_ADDR_ERR_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLV_CMD_ERR_INT_RAW` reader - "]
pub type SLV_CMD_ERR_INT_RAW_R = crate::BitReader;
#[doc = "Field `SLV_CMD_ERR_INT_RAW` writer - "]
pub type SLV_CMD_ERR_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MST_RX_AFIFO_WFULL_ERR_INT_RAW` reader - "]
pub type MST_RX_AFIFO_WFULL_ERR_INT_RAW_R = crate::BitReader;
#[doc = "Field `MST_RX_AFIFO_WFULL_ERR_INT_RAW` writer - "]
pub type MST_RX_AFIFO_WFULL_ERR_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MST_TX_AFIFO_REMPTY_ERR_INT_RAW` reader - "]
pub type MST_TX_AFIFO_REMPTY_ERR_INT_RAW_R = crate::BitReader;
#[doc = "Field `MST_TX_AFIFO_REMPTY_ERR_INT_RAW` writer - "]
pub type MST_TX_AFIFO_REMPTY_ERR_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APP2_INT_RAW` reader - "]
pub type APP2_INT_RAW_R = crate::BitReader;
#[doc = "Field `APP2_INT_RAW` writer - "]
pub type APP2_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APP1_INT_RAW` reader - "]
pub type APP1_INT_RAW_R = crate::BitReader;
#[doc = "Field `APP1_INT_RAW` writer - "]
pub type APP1_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn slv_rd_buf_done_int_raw(&self) -> SLV_RD_BUF_DONE_INT_RAW_R {
        SLV_RD_BUF_DONE_INT_RAW_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn slv_wr_buf_done_int_raw(&self) -> SLV_WR_BUF_DONE_INT_RAW_R {
        SLV_WR_BUF_DONE_INT_RAW_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn trans_done_int_raw(&self) -> TRANS_DONE_INT_RAW_R {
        TRANS_DONE_INT_RAW_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn spi_wakeup_int_raw(&self) -> SPI_WAKEUP_INT_RAW_R {
        SPI_WAKEUP_INT_RAW_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn slv_buf_addr_err_int_raw(&self) -> SLV_BUF_ADDR_ERR_INT_RAW_R {
        SLV_BUF_ADDR_ERR_INT_RAW_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn slv_cmd_err_int_raw(&self) -> SLV_CMD_ERR_INT_RAW_R {
        SLV_CMD_ERR_INT_RAW_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn mst_rx_afifo_wfull_err_int_raw(&self) -> MST_RX_AFIFO_WFULL_ERR_INT_RAW_R {
        MST_RX_AFIFO_WFULL_ERR_INT_RAW_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn mst_tx_afifo_rempty_err_int_raw(&self) -> MST_TX_AFIFO_REMPTY_ERR_INT_RAW_R {
        MST_TX_AFIFO_REMPTY_ERR_INT_RAW_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn app2_int_raw(&self) -> APP2_INT_RAW_R {
        APP2_INT_RAW_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn app1_int_raw(&self) -> APP1_INT_RAW_R {
        APP1_INT_RAW_R::new(((self.bits >> 20) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_INT_RAW")
            .field("slv_rd_buf_done_int_raw", &self.slv_rd_buf_done_int_raw())
            .field("slv_wr_buf_done_int_raw", &self.slv_wr_buf_done_int_raw())
            .field("trans_done_int_raw", &self.trans_done_int_raw())
            .field("spi_wakeup_int_raw", &self.spi_wakeup_int_raw())
            .field("slv_buf_addr_err_int_raw", &self.slv_buf_addr_err_int_raw())
            .field("slv_cmd_err_int_raw", &self.slv_cmd_err_int_raw())
            .field(
                "mst_rx_afifo_wfull_err_int_raw",
                &self.mst_rx_afifo_wfull_err_int_raw(),
            )
            .field(
                "mst_tx_afifo_rempty_err_int_raw",
                &self.mst_tx_afifo_rempty_err_int_raw(),
            )
            .field("app2_int_raw", &self.app2_int_raw())
            .field("app1_int_raw", &self.app1_int_raw())
            .finish()
    }
}
impl W {
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn slv_rd_buf_done_int_raw(&mut self) -> SLV_RD_BUF_DONE_INT_RAW_W<'_, DMA_INT_RAW_SPEC> {
        SLV_RD_BUF_DONE_INT_RAW_W::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn slv_wr_buf_done_int_raw(&mut self) -> SLV_WR_BUF_DONE_INT_RAW_W<'_, DMA_INT_RAW_SPEC> {
        SLV_WR_BUF_DONE_INT_RAW_W::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn trans_done_int_raw(&mut self) -> TRANS_DONE_INT_RAW_W<'_, DMA_INT_RAW_SPEC> {
        TRANS_DONE_INT_RAW_W::new(self, 12)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn spi_wakeup_int_raw(&mut self) -> SPI_WAKEUP_INT_RAW_W<'_, DMA_INT_RAW_SPEC> {
        SPI_WAKEUP_INT_RAW_W::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn slv_buf_addr_err_int_raw(&mut self) -> SLV_BUF_ADDR_ERR_INT_RAW_W<'_, DMA_INT_RAW_SPEC> {
        SLV_BUF_ADDR_ERR_INT_RAW_W::new(self, 15)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn slv_cmd_err_int_raw(&mut self) -> SLV_CMD_ERR_INT_RAW_W<'_, DMA_INT_RAW_SPEC> {
        SLV_CMD_ERR_INT_RAW_W::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn mst_rx_afifo_wfull_err_int_raw(
        &mut self,
    ) -> MST_RX_AFIFO_WFULL_ERR_INT_RAW_W<'_, DMA_INT_RAW_SPEC> {
        MST_RX_AFIFO_WFULL_ERR_INT_RAW_W::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn mst_tx_afifo_rempty_err_int_raw(
        &mut self,
    ) -> MST_TX_AFIFO_REMPTY_ERR_INT_RAW_W<'_, DMA_INT_RAW_SPEC> {
        MST_TX_AFIFO_REMPTY_ERR_INT_RAW_W::new(self, 18)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn app2_int_raw(&mut self) -> APP2_INT_RAW_W<'_, DMA_INT_RAW_SPEC> {
        APP2_INT_RAW_W::new(self, 19)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn app1_int_raw(&mut self) -> APP1_INT_RAW_W<'_, DMA_INT_RAW_SPEC> {
        APP1_INT_RAW_W::new(self, 20)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_int_raw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_int_raw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
