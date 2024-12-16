#[doc = "Register `DMA_CONF` reader"]
pub type R = crate::R<DMA_CONF_SPEC>;
#[doc = "Register `DMA_CONF` writer"]
pub type W = crate::W<DMA_CONF_SPEC>;
#[doc = "Field `IN_RST` reader - The bit is used to reset in dma fsm and in data fifo pointer."]
pub type IN_RST_R = crate::BitReader;
#[doc = "Field `IN_RST` writer - The bit is used to reset in dma fsm and in data fifo pointer."]
pub type IN_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_RST` reader - The bit is used to reset out dma fsm and out data fifo pointer."]
pub type OUT_RST_R = crate::BitReader;
#[doc = "Field `OUT_RST` writer - The bit is used to reset out dma fsm and out data fifo pointer."]
pub type OUT_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHBM_FIFO_RST` reader - reset spi dma ahb master fifo pointer."]
pub type AHBM_FIFO_RST_R = crate::BitReader;
#[doc = "Field `AHBM_FIFO_RST` writer - reset spi dma ahb master fifo pointer."]
pub type AHBM_FIFO_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHBM_RST` reader - reset spi dma ahb master."]
pub type AHBM_RST_R = crate::BitReader;
#[doc = "Field `AHBM_RST` writer - reset spi dma ahb master."]
pub type AHBM_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_LOOP_TEST` reader - Set bit to test in link."]
pub type IN_LOOP_TEST_R = crate::BitReader;
#[doc = "Field `IN_LOOP_TEST` writer - Set bit to test in link."]
pub type IN_LOOP_TEST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_LOOP_TEST` reader - Set bit to test out link."]
pub type OUT_LOOP_TEST_R = crate::BitReader;
#[doc = "Field `OUT_LOOP_TEST` writer - Set bit to test out link."]
pub type OUT_LOOP_TEST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_AUTO_WRBACK` reader - when the link is empty jump to next automatically."]
pub type OUT_AUTO_WRBACK_R = crate::BitReader;
#[doc = "Field `OUT_AUTO_WRBACK` writer - when the link is empty jump to next automatically."]
pub type OUT_AUTO_WRBACK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_EOF_MODE` reader - out eof flag generation mode . 1: when dma pop all data from fifo 0:when ahb push all data to fifo."]
pub type OUT_EOF_MODE_R = crate::BitReader;
#[doc = "Field `OUT_EOF_MODE` writer - out eof flag generation mode . 1: when dma pop all data from fifo 0:when ahb push all data to fifo."]
pub type OUT_EOF_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTDSCR_BURST_EN` reader - read descriptor use burst mode when read data for memory."]
pub type OUTDSCR_BURST_EN_R = crate::BitReader;
#[doc = "Field `OUTDSCR_BURST_EN` writer - read descriptor use burst mode when read data for memory."]
pub type OUTDSCR_BURST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INDSCR_BURST_EN` reader - read descriptor use burst mode when write data to memory."]
pub type INDSCR_BURST_EN_R = crate::BitReader;
#[doc = "Field `INDSCR_BURST_EN` writer - read descriptor use burst mode when write data to memory."]
pub type INDSCR_BURST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_DATA_BURST_EN` reader - spi dma read data from memory in burst mode."]
pub type OUT_DATA_BURST_EN_R = crate::BitReader;
#[doc = "Field `OUT_DATA_BURST_EN` writer - spi dma read data from memory in burst mode."]
pub type OUT_DATA_BURST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_RX_STOP` reader - spi dma read data stop when in continue tx/rx mode."]
pub type DMA_RX_STOP_R = crate::BitReader;
#[doc = "Field `DMA_RX_STOP` writer - spi dma read data stop when in continue tx/rx mode."]
pub type DMA_RX_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_TX_STOP` reader - spi dma write data stop when in continue tx/rx mode."]
pub type DMA_TX_STOP_R = crate::BitReader;
#[doc = "Field `DMA_TX_STOP` writer - spi dma write data stop when in continue tx/rx mode."]
pub type DMA_TX_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_CONTINUE` reader - spi dma continue tx/rx data."]
pub type DMA_CONTINUE_R = crate::BitReader;
#[doc = "Field `DMA_CONTINUE` writer - spi dma continue tx/rx data."]
pub type DMA_CONTINUE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - The bit is used to reset in dma fsm and in data fifo pointer."]
    #[inline(always)]
    pub fn in_rst(&self) -> IN_RST_R {
        IN_RST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The bit is used to reset out dma fsm and out data fifo pointer."]
    #[inline(always)]
    pub fn out_rst(&self) -> OUT_RST_R {
        OUT_RST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - reset spi dma ahb master fifo pointer."]
    #[inline(always)]
    pub fn ahbm_fifo_rst(&self) -> AHBM_FIFO_RST_R {
        AHBM_FIFO_RST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - reset spi dma ahb master."]
    #[inline(always)]
    pub fn ahbm_rst(&self) -> AHBM_RST_R {
        AHBM_RST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Set bit to test in link."]
    #[inline(always)]
    pub fn in_loop_test(&self) -> IN_LOOP_TEST_R {
        IN_LOOP_TEST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Set bit to test out link."]
    #[inline(always)]
    pub fn out_loop_test(&self) -> OUT_LOOP_TEST_R {
        OUT_LOOP_TEST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - when the link is empty jump to next automatically."]
    #[inline(always)]
    pub fn out_auto_wrback(&self) -> OUT_AUTO_WRBACK_R {
        OUT_AUTO_WRBACK_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - out eof flag generation mode . 1: when dma pop all data from fifo 0:when ahb push all data to fifo."]
    #[inline(always)]
    pub fn out_eof_mode(&self) -> OUT_EOF_MODE_R {
        OUT_EOF_MODE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - read descriptor use burst mode when read data for memory."]
    #[inline(always)]
    pub fn outdscr_burst_en(&self) -> OUTDSCR_BURST_EN_R {
        OUTDSCR_BURST_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - read descriptor use burst mode when write data to memory."]
    #[inline(always)]
    pub fn indscr_burst_en(&self) -> INDSCR_BURST_EN_R {
        INDSCR_BURST_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - spi dma read data from memory in burst mode."]
    #[inline(always)]
    pub fn out_data_burst_en(&self) -> OUT_DATA_BURST_EN_R {
        OUT_DATA_BURST_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - spi dma read data stop when in continue tx/rx mode."]
    #[inline(always)]
    pub fn dma_rx_stop(&self) -> DMA_RX_STOP_R {
        DMA_RX_STOP_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - spi dma write data stop when in continue tx/rx mode."]
    #[inline(always)]
    pub fn dma_tx_stop(&self) -> DMA_TX_STOP_R {
        DMA_TX_STOP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - spi dma continue tx/rx data."]
    #[inline(always)]
    pub fn dma_continue(&self) -> DMA_CONTINUE_R {
        DMA_CONTINUE_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_CONF")
            .field("in_rst", &self.in_rst())
            .field("out_rst", &self.out_rst())
            .field("ahbm_fifo_rst", &self.ahbm_fifo_rst())
            .field("ahbm_rst", &self.ahbm_rst())
            .field("in_loop_test", &self.in_loop_test())
            .field("out_loop_test", &self.out_loop_test())
            .field("out_auto_wrback", &self.out_auto_wrback())
            .field("out_eof_mode", &self.out_eof_mode())
            .field("outdscr_burst_en", &self.outdscr_burst_en())
            .field("indscr_burst_en", &self.indscr_burst_en())
            .field("out_data_burst_en", &self.out_data_burst_en())
            .field("dma_rx_stop", &self.dma_rx_stop())
            .field("dma_tx_stop", &self.dma_tx_stop())
            .field("dma_continue", &self.dma_continue())
            .finish()
    }
}
impl W {
    #[doc = "Bit 2 - The bit is used to reset in dma fsm and in data fifo pointer."]
    #[inline(always)]
    pub fn in_rst(&mut self) -> IN_RST_W<DMA_CONF_SPEC> {
        IN_RST_W::new(self, 2)
    }
    #[doc = "Bit 3 - The bit is used to reset out dma fsm and out data fifo pointer."]
    #[inline(always)]
    pub fn out_rst(&mut self) -> OUT_RST_W<DMA_CONF_SPEC> {
        OUT_RST_W::new(self, 3)
    }
    #[doc = "Bit 4 - reset spi dma ahb master fifo pointer."]
    #[inline(always)]
    pub fn ahbm_fifo_rst(&mut self) -> AHBM_FIFO_RST_W<DMA_CONF_SPEC> {
        AHBM_FIFO_RST_W::new(self, 4)
    }
    #[doc = "Bit 5 - reset spi dma ahb master."]
    #[inline(always)]
    pub fn ahbm_rst(&mut self) -> AHBM_RST_W<DMA_CONF_SPEC> {
        AHBM_RST_W::new(self, 5)
    }
    #[doc = "Bit 6 - Set bit to test in link."]
    #[inline(always)]
    pub fn in_loop_test(&mut self) -> IN_LOOP_TEST_W<DMA_CONF_SPEC> {
        IN_LOOP_TEST_W::new(self, 6)
    }
    #[doc = "Bit 7 - Set bit to test out link."]
    #[inline(always)]
    pub fn out_loop_test(&mut self) -> OUT_LOOP_TEST_W<DMA_CONF_SPEC> {
        OUT_LOOP_TEST_W::new(self, 7)
    }
    #[doc = "Bit 8 - when the link is empty jump to next automatically."]
    #[inline(always)]
    pub fn out_auto_wrback(&mut self) -> OUT_AUTO_WRBACK_W<DMA_CONF_SPEC> {
        OUT_AUTO_WRBACK_W::new(self, 8)
    }
    #[doc = "Bit 9 - out eof flag generation mode . 1: when dma pop all data from fifo 0:when ahb push all data to fifo."]
    #[inline(always)]
    pub fn out_eof_mode(&mut self) -> OUT_EOF_MODE_W<DMA_CONF_SPEC> {
        OUT_EOF_MODE_W::new(self, 9)
    }
    #[doc = "Bit 10 - read descriptor use burst mode when read data for memory."]
    #[inline(always)]
    pub fn outdscr_burst_en(&mut self) -> OUTDSCR_BURST_EN_W<DMA_CONF_SPEC> {
        OUTDSCR_BURST_EN_W::new(self, 10)
    }
    #[doc = "Bit 11 - read descriptor use burst mode when write data to memory."]
    #[inline(always)]
    pub fn indscr_burst_en(&mut self) -> INDSCR_BURST_EN_W<DMA_CONF_SPEC> {
        INDSCR_BURST_EN_W::new(self, 11)
    }
    #[doc = "Bit 12 - spi dma read data from memory in burst mode."]
    #[inline(always)]
    pub fn out_data_burst_en(&mut self) -> OUT_DATA_BURST_EN_W<DMA_CONF_SPEC> {
        OUT_DATA_BURST_EN_W::new(self, 12)
    }
    #[doc = "Bit 14 - spi dma read data stop when in continue tx/rx mode."]
    #[inline(always)]
    pub fn dma_rx_stop(&mut self) -> DMA_RX_STOP_W<DMA_CONF_SPEC> {
        DMA_RX_STOP_W::new(self, 14)
    }
    #[doc = "Bit 15 - spi dma write data stop when in continue tx/rx mode."]
    #[inline(always)]
    pub fn dma_tx_stop(&mut self) -> DMA_TX_STOP_W<DMA_CONF_SPEC> {
        DMA_TX_STOP_W::new(self, 15)
    }
    #[doc = "Bit 16 - spi dma continue tx/rx data."]
    #[inline(always)]
    pub fn dma_continue(&mut self) -> DMA_CONTINUE_W<DMA_CONF_SPEC> {
        DMA_CONTINUE_W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMA_CONF_SPEC;
impl crate::RegisterSpec for DMA_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_conf::R`](R) reader structure"]
impl crate::Readable for DMA_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dma_conf::W`](W) writer structure"]
impl crate::Writable for DMA_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMA_CONF to value 0x0200"]
impl crate::Resettable for DMA_CONF_SPEC {
    const RESET_VALUE: u32 = 0x0200;
}
