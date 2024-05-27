///Register `CONF0` reader
pub type R = crate::R<CONF0_SPEC>;
///Register `CONF0` writer
pub type W = crate::W<CONF0_SPEC>;
///Field `IN_RST` reader - Set this bit to reset in link operations.
pub type IN_RST_R = crate::BitReader;
///Field `IN_RST` writer - Set this bit to reset in link operations.
pub type IN_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OUT_RST` reader - Set this bit to reset out link operations.
pub type OUT_RST_R = crate::BitReader;
///Field `OUT_RST` writer - Set this bit to reset out link operations.
pub type OUT_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHBM_FIFO_RST` reader - Set this bit to reset dma ahb fifo.
pub type AHBM_FIFO_RST_R = crate::BitReader;
///Field `AHBM_FIFO_RST` writer - Set this bit to reset dma ahb fifo.
pub type AHBM_FIFO_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHBM_RST` reader - Set this bit to reset dma ahb interface.
pub type AHBM_RST_R = crate::BitReader;
///Field `AHBM_RST` writer - Set this bit to reset dma ahb interface.
pub type AHBM_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IN_LOOP_TEST` reader - Set this bit to enable loop test for in links.
pub type IN_LOOP_TEST_R = crate::BitReader;
///Field `IN_LOOP_TEST` writer - Set this bit to enable loop test for in links.
pub type IN_LOOP_TEST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OUT_LOOP_TEST` reader - Set this bit to enable loop test for out links.
pub type OUT_LOOP_TEST_R = crate::BitReader;
///Field `OUT_LOOP_TEST` writer - Set this bit to enable loop test for out links.
pub type OUT_LOOP_TEST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OUT_AUTO_WRBACK` reader - when in link's length is 0 go on to use the next in link automatically.
pub type OUT_AUTO_WRBACK_R = crate::BitReader;
///Field `OUT_AUTO_WRBACK` writer - when in link's length is 0 go on to use the next in link automatically.
pub type OUT_AUTO_WRBACK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OUT_NO_RESTART_CLR` reader - don't use
pub type OUT_NO_RESTART_CLR_R = crate::BitReader;
///Field `OUT_NO_RESTART_CLR` writer - don't use
pub type OUT_NO_RESTART_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OUT_EOF_MODE` reader - Set this bit to produce eof after DMA pops all data clear this bit to produce eof after DMA pushes all data
pub type OUT_EOF_MODE_R = crate::BitReader;
///Field `OUT_EOF_MODE` writer - Set this bit to produce eof after DMA pops all data clear this bit to produce eof after DMA pushes all data
pub type OUT_EOF_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UART0_CE` reader - Set this bit to use UART to transmit or receive data.
pub type UART0_CE_R = crate::BitReader;
///Field `UART0_CE` writer - Set this bit to use UART to transmit or receive data.
pub type UART0_CE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UART1_CE` reader - Set this bit to use UART1 to transmit or receive data.
pub type UART1_CE_R = crate::BitReader;
///Field `UART1_CE` writer - Set this bit to use UART1 to transmit or receive data.
pub type UART1_CE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UART2_CE` reader - Set this bit to use UART2 to transmit or receive data.
pub type UART2_CE_R = crate::BitReader;
///Field `UART2_CE` writer - Set this bit to use UART2 to transmit or receive data.
pub type UART2_CE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OUTDSCR_BURST_EN` reader - Set this bit to enable DMA in links to use burst mode.
pub type OUTDSCR_BURST_EN_R = crate::BitReader;
///Field `OUTDSCR_BURST_EN` writer - Set this bit to enable DMA in links to use burst mode.
pub type OUTDSCR_BURST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `INDSCR_BURST_EN` reader - Set this bit to enable DMA out links to use burst mode.
pub type INDSCR_BURST_EN_R = crate::BitReader;
///Field `INDSCR_BURST_EN` writer - Set this bit to enable DMA out links to use burst mode.
pub type INDSCR_BURST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OUT_DATA_BURST_EN` reader - Set this bit to enable DMA burst MODE
pub type OUT_DATA_BURST_EN_R = crate::BitReader;
///Field `OUT_DATA_BURST_EN` writer - Set this bit to enable DMA burst MODE
pub type OUT_DATA_BURST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MEM_TRANS_EN` reader -
pub type MEM_TRANS_EN_R = crate::BitReader;
///Field `MEM_TRANS_EN` writer -
pub type MEM_TRANS_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEPER_EN` reader - Set this bit to use special char to separate the data frame.
pub type SEPER_EN_R = crate::BitReader;
///Field `SEPER_EN` writer - Set this bit to use special char to separate the data frame.
pub type SEPER_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HEAD_EN` reader - Set this bit to enable to use head packet before the data frame.
pub type HEAD_EN_R = crate::BitReader;
///Field `HEAD_EN` writer - Set this bit to enable to use head packet before the data frame.
pub type HEAD_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRC_REC_EN` reader - Set this bit to enable receiver''s ability of crc calculation when crc_en bit in head packet is 1 then there will be crc bytes after data_frame
pub type CRC_REC_EN_R = crate::BitReader;
///Field `CRC_REC_EN` writer - Set this bit to enable receiver''s ability of crc calculation when crc_en bit in head packet is 1 then there will be crc bytes after data_frame
pub type CRC_REC_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UART_IDLE_EOF_EN` reader - Set this bit to enable to use idle time when the idle time after data frame is satisfied this means the end of a data frame.
pub type UART_IDLE_EOF_EN_R = crate::BitReader;
///Field `UART_IDLE_EOF_EN` writer - Set this bit to enable to use idle time when the idle time after data frame is satisfied this means the end of a data frame.
pub type UART_IDLE_EOF_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LEN_EOF_EN` reader - Set this bit to enable to use packet_len in packet head when the received data is equal to packet_len this means the end of a data frame.
pub type LEN_EOF_EN_R = crate::BitReader;
///Field `LEN_EOF_EN` writer - Set this bit to enable to use packet_len in packet head when the received data is equal to packet_len this means the end of a data frame.
pub type LEN_EOF_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ENCODE_CRC_EN` reader - Set this bit to enable crc calculation for data frame when bit6 in the head packet is 1.
pub type ENCODE_CRC_EN_R = crate::BitReader;
///Field `ENCODE_CRC_EN` writer - Set this bit to enable crc calculation for data frame when bit6 in the head packet is 1.
pub type ENCODE_CRC_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLK_EN` reader - Set this bit to enable clock-gating for read or write registers.
pub type CLK_EN_R = crate::BitReader;
///Field `CLK_EN` writer - Set this bit to enable clock-gating for read or write registers.
pub type CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UART_RX_BRK_EOF_EN` reader - Set this bit to enable to use brk char as the end of a data frame.
pub type UART_RX_BRK_EOF_EN_R = crate::BitReader;
///Field `UART_RX_BRK_EOF_EN` writer - Set this bit to enable to use brk char as the end of a data frame.
pub type UART_RX_BRK_EOF_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Set this bit to reset in link operations.
    #[inline(always)]
    pub fn in_rst(&self) -> IN_RST_R {
        IN_RST_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Set this bit to reset out link operations.
    #[inline(always)]
    pub fn out_rst(&self) -> OUT_RST_R {
        OUT_RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Set this bit to reset dma ahb fifo.
    #[inline(always)]
    pub fn ahbm_fifo_rst(&self) -> AHBM_FIFO_RST_R {
        AHBM_FIFO_RST_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Set this bit to reset dma ahb interface.
    #[inline(always)]
    pub fn ahbm_rst(&self) -> AHBM_RST_R {
        AHBM_RST_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Set this bit to enable loop test for in links.
    #[inline(always)]
    pub fn in_loop_test(&self) -> IN_LOOP_TEST_R {
        IN_LOOP_TEST_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Set this bit to enable loop test for out links.
    #[inline(always)]
    pub fn out_loop_test(&self) -> OUT_LOOP_TEST_R {
        OUT_LOOP_TEST_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - when in link's length is 0 go on to use the next in link automatically.
    #[inline(always)]
    pub fn out_auto_wrback(&self) -> OUT_AUTO_WRBACK_R {
        OUT_AUTO_WRBACK_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - don't use
    #[inline(always)]
    pub fn out_no_restart_clr(&self) -> OUT_NO_RESTART_CLR_R {
        OUT_NO_RESTART_CLR_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Set this bit to produce eof after DMA pops all data clear this bit to produce eof after DMA pushes all data
    #[inline(always)]
    pub fn out_eof_mode(&self) -> OUT_EOF_MODE_R {
        OUT_EOF_MODE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Set this bit to use UART to transmit or receive data.
    #[inline(always)]
    pub fn uart0_ce(&self) -> UART0_CE_R {
        UART0_CE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Set this bit to use UART1 to transmit or receive data.
    #[inline(always)]
    pub fn uart1_ce(&self) -> UART1_CE_R {
        UART1_CE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Set this bit to use UART2 to transmit or receive data.
    #[inline(always)]
    pub fn uart2_ce(&self) -> UART2_CE_R {
        UART2_CE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Set this bit to enable DMA in links to use burst mode.
    #[inline(always)]
    pub fn outdscr_burst_en(&self) -> OUTDSCR_BURST_EN_R {
        OUTDSCR_BURST_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Set this bit to enable DMA out links to use burst mode.
    #[inline(always)]
    pub fn indscr_burst_en(&self) -> INDSCR_BURST_EN_R {
        INDSCR_BURST_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Set this bit to enable DMA burst MODE
    #[inline(always)]
    pub fn out_data_burst_en(&self) -> OUT_DATA_BURST_EN_R {
        OUT_DATA_BURST_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15
    #[inline(always)]
    pub fn mem_trans_en(&self) -> MEM_TRANS_EN_R {
        MEM_TRANS_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Set this bit to use special char to separate the data frame.
    #[inline(always)]
    pub fn seper_en(&self) -> SEPER_EN_R {
        SEPER_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Set this bit to enable to use head packet before the data frame.
    #[inline(always)]
    pub fn head_en(&self) -> HEAD_EN_R {
        HEAD_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Set this bit to enable receiver''s ability of crc calculation when crc_en bit in head packet is 1 then there will be crc bytes after data_frame
    #[inline(always)]
    pub fn crc_rec_en(&self) -> CRC_REC_EN_R {
        CRC_REC_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Set this bit to enable to use idle time when the idle time after data frame is satisfied this means the end of a data frame.
    #[inline(always)]
    pub fn uart_idle_eof_en(&self) -> UART_IDLE_EOF_EN_R {
        UART_IDLE_EOF_EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Set this bit to enable to use packet_len in packet head when the received data is equal to packet_len this means the end of a data frame.
    #[inline(always)]
    pub fn len_eof_en(&self) -> LEN_EOF_EN_R {
        LEN_EOF_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Set this bit to enable crc calculation for data frame when bit6 in the head packet is 1.
    #[inline(always)]
    pub fn encode_crc_en(&self) -> ENCODE_CRC_EN_R {
        ENCODE_CRC_EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Set this bit to enable clock-gating for read or write registers.
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Set this bit to enable to use brk char as the end of a data frame.
    #[inline(always)]
    pub fn uart_rx_brk_eof_en(&self) -> UART_RX_BRK_EOF_EN_R {
        UART_RX_BRK_EOF_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONF0")
            .field("in_rst", &self.in_rst())
            .field("out_rst", &self.out_rst())
            .field("ahbm_fifo_rst", &self.ahbm_fifo_rst())
            .field("ahbm_rst", &self.ahbm_rst())
            .field("in_loop_test", &self.in_loop_test())
            .field("out_loop_test", &self.out_loop_test())
            .field("out_auto_wrback", &self.out_auto_wrback())
            .field("out_no_restart_clr", &self.out_no_restart_clr())
            .field("out_eof_mode", &self.out_eof_mode())
            .field("uart0_ce", &self.uart0_ce())
            .field("uart1_ce", &self.uart1_ce())
            .field("uart2_ce", &self.uart2_ce())
            .field("outdscr_burst_en", &self.outdscr_burst_en())
            .field("indscr_burst_en", &self.indscr_burst_en())
            .field("out_data_burst_en", &self.out_data_burst_en())
            .field("mem_trans_en", &self.mem_trans_en())
            .field("seper_en", &self.seper_en())
            .field("head_en", &self.head_en())
            .field("crc_rec_en", &self.crc_rec_en())
            .field("uart_idle_eof_en", &self.uart_idle_eof_en())
            .field("len_eof_en", &self.len_eof_en())
            .field("encode_crc_en", &self.encode_crc_en())
            .field("clk_en", &self.clk_en())
            .field("uart_rx_brk_eof_en", &self.uart_rx_brk_eof_en())
            .finish()
    }
}
impl W {
    ///Bit 0 - Set this bit to reset in link operations.
    #[inline(always)]
    #[must_use]
    pub fn in_rst(&mut self) -> IN_RST_W<CONF0_SPEC> {
        IN_RST_W::new(self, 0)
    }
    ///Bit 1 - Set this bit to reset out link operations.
    #[inline(always)]
    #[must_use]
    pub fn out_rst(&mut self) -> OUT_RST_W<CONF0_SPEC> {
        OUT_RST_W::new(self, 1)
    }
    ///Bit 2 - Set this bit to reset dma ahb fifo.
    #[inline(always)]
    #[must_use]
    pub fn ahbm_fifo_rst(&mut self) -> AHBM_FIFO_RST_W<CONF0_SPEC> {
        AHBM_FIFO_RST_W::new(self, 2)
    }
    ///Bit 3 - Set this bit to reset dma ahb interface.
    #[inline(always)]
    #[must_use]
    pub fn ahbm_rst(&mut self) -> AHBM_RST_W<CONF0_SPEC> {
        AHBM_RST_W::new(self, 3)
    }
    ///Bit 4 - Set this bit to enable loop test for in links.
    #[inline(always)]
    #[must_use]
    pub fn in_loop_test(&mut self) -> IN_LOOP_TEST_W<CONF0_SPEC> {
        IN_LOOP_TEST_W::new(self, 4)
    }
    ///Bit 5 - Set this bit to enable loop test for out links.
    #[inline(always)]
    #[must_use]
    pub fn out_loop_test(&mut self) -> OUT_LOOP_TEST_W<CONF0_SPEC> {
        OUT_LOOP_TEST_W::new(self, 5)
    }
    ///Bit 6 - when in link's length is 0 go on to use the next in link automatically.
    #[inline(always)]
    #[must_use]
    pub fn out_auto_wrback(&mut self) -> OUT_AUTO_WRBACK_W<CONF0_SPEC> {
        OUT_AUTO_WRBACK_W::new(self, 6)
    }
    ///Bit 7 - don't use
    #[inline(always)]
    #[must_use]
    pub fn out_no_restart_clr(&mut self) -> OUT_NO_RESTART_CLR_W<CONF0_SPEC> {
        OUT_NO_RESTART_CLR_W::new(self, 7)
    }
    ///Bit 8 - Set this bit to produce eof after DMA pops all data clear this bit to produce eof after DMA pushes all data
    #[inline(always)]
    #[must_use]
    pub fn out_eof_mode(&mut self) -> OUT_EOF_MODE_W<CONF0_SPEC> {
        OUT_EOF_MODE_W::new(self, 8)
    }
    ///Bit 9 - Set this bit to use UART to transmit or receive data.
    #[inline(always)]
    #[must_use]
    pub fn uart0_ce(&mut self) -> UART0_CE_W<CONF0_SPEC> {
        UART0_CE_W::new(self, 9)
    }
    ///Bit 10 - Set this bit to use UART1 to transmit or receive data.
    #[inline(always)]
    #[must_use]
    pub fn uart1_ce(&mut self) -> UART1_CE_W<CONF0_SPEC> {
        UART1_CE_W::new(self, 10)
    }
    ///Bit 11 - Set this bit to use UART2 to transmit or receive data.
    #[inline(always)]
    #[must_use]
    pub fn uart2_ce(&mut self) -> UART2_CE_W<CONF0_SPEC> {
        UART2_CE_W::new(self, 11)
    }
    ///Bit 12 - Set this bit to enable DMA in links to use burst mode.
    #[inline(always)]
    #[must_use]
    pub fn outdscr_burst_en(&mut self) -> OUTDSCR_BURST_EN_W<CONF0_SPEC> {
        OUTDSCR_BURST_EN_W::new(self, 12)
    }
    ///Bit 13 - Set this bit to enable DMA out links to use burst mode.
    #[inline(always)]
    #[must_use]
    pub fn indscr_burst_en(&mut self) -> INDSCR_BURST_EN_W<CONF0_SPEC> {
        INDSCR_BURST_EN_W::new(self, 13)
    }
    ///Bit 14 - Set this bit to enable DMA burst MODE
    #[inline(always)]
    #[must_use]
    pub fn out_data_burst_en(&mut self) -> OUT_DATA_BURST_EN_W<CONF0_SPEC> {
        OUT_DATA_BURST_EN_W::new(self, 14)
    }
    ///Bit 15
    #[inline(always)]
    #[must_use]
    pub fn mem_trans_en(&mut self) -> MEM_TRANS_EN_W<CONF0_SPEC> {
        MEM_TRANS_EN_W::new(self, 15)
    }
    ///Bit 16 - Set this bit to use special char to separate the data frame.
    #[inline(always)]
    #[must_use]
    pub fn seper_en(&mut self) -> SEPER_EN_W<CONF0_SPEC> {
        SEPER_EN_W::new(self, 16)
    }
    ///Bit 17 - Set this bit to enable to use head packet before the data frame.
    #[inline(always)]
    #[must_use]
    pub fn head_en(&mut self) -> HEAD_EN_W<CONF0_SPEC> {
        HEAD_EN_W::new(self, 17)
    }
    ///Bit 18 - Set this bit to enable receiver''s ability of crc calculation when crc_en bit in head packet is 1 then there will be crc bytes after data_frame
    #[inline(always)]
    #[must_use]
    pub fn crc_rec_en(&mut self) -> CRC_REC_EN_W<CONF0_SPEC> {
        CRC_REC_EN_W::new(self, 18)
    }
    ///Bit 19 - Set this bit to enable to use idle time when the idle time after data frame is satisfied this means the end of a data frame.
    #[inline(always)]
    #[must_use]
    pub fn uart_idle_eof_en(&mut self) -> UART_IDLE_EOF_EN_W<CONF0_SPEC> {
        UART_IDLE_EOF_EN_W::new(self, 19)
    }
    ///Bit 20 - Set this bit to enable to use packet_len in packet head when the received data is equal to packet_len this means the end of a data frame.
    #[inline(always)]
    #[must_use]
    pub fn len_eof_en(&mut self) -> LEN_EOF_EN_W<CONF0_SPEC> {
        LEN_EOF_EN_W::new(self, 20)
    }
    ///Bit 21 - Set this bit to enable crc calculation for data frame when bit6 in the head packet is 1.
    #[inline(always)]
    #[must_use]
    pub fn encode_crc_en(&mut self) -> ENCODE_CRC_EN_W<CONF0_SPEC> {
        ENCODE_CRC_EN_W::new(self, 21)
    }
    ///Bit 22 - Set this bit to enable clock-gating for read or write registers.
    #[inline(always)]
    #[must_use]
    pub fn clk_en(&mut self) -> CLK_EN_W<CONF0_SPEC> {
        CLK_EN_W::new(self, 22)
    }
    ///Bit 23 - Set this bit to enable to use brk char as the end of a data frame.
    #[inline(always)]
    #[must_use]
    pub fn uart_rx_brk_eof_en(&mut self) -> UART_RX_BRK_EOF_EN_W<CONF0_SPEC> {
        UART_RX_BRK_EOF_EN_W::new(self, 23)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`conf0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conf0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CONF0_SPEC;
impl crate::RegisterSpec for CONF0_SPEC {
    type Ux = u32;
}
///`read()` method returns [`conf0::R`](R) reader structure
impl crate::Readable for CONF0_SPEC {}
///`write(|w| ..)` method takes [`conf0::W`](W) writer structure
impl crate::Writable for CONF0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CONF0 to value 0x0037_0100
impl crate::Resettable for CONF0_SPEC {
    const RESET_VALUE: u32 = 0x0037_0100;
}
