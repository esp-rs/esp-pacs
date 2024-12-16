#[doc = "Register `CONF0` reader"]
pub type R = crate::R<CONF0_SPEC>;
#[doc = "Register `CONF0` writer"]
pub type W = crate::W<CONF0_SPEC>;
#[doc = "Field `IN_RST` reader - Set this bit to reset in DMA FSM."]
pub type IN_RST_R = crate::BitReader;
#[doc = "Field `IN_RST` writer - Set this bit to reset in DMA FSM."]
pub type IN_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_RST` reader - Set this bit to reset out DMA FSM."]
pub type OUT_RST_R = crate::BitReader;
#[doc = "Field `OUT_RST` writer - Set this bit to reset out DMA FSM."]
pub type OUT_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHBM_FIFO_RST` reader - Set this bit to reset AHB interface cmdFIFO of DMA."]
pub type AHBM_FIFO_RST_R = crate::BitReader;
#[doc = "Field `AHBM_FIFO_RST` writer - Set this bit to reset AHB interface cmdFIFO of DMA."]
pub type AHBM_FIFO_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHBM_RST` reader - Set this bit to reset AHB interface of DMA."]
pub type AHBM_RST_R = crate::BitReader;
#[doc = "Field `AHBM_RST` writer - Set this bit to reset AHB interface of DMA."]
pub type AHBM_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_LOOP_TEST` reader - Reserved."]
pub type IN_LOOP_TEST_R = crate::BitReader;
#[doc = "Field `IN_LOOP_TEST` writer - Reserved."]
pub type IN_LOOP_TEST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_LOOP_TEST` reader - Reserved."]
pub type OUT_LOOP_TEST_R = crate::BitReader;
#[doc = "Field `OUT_LOOP_TEST` writer - Reserved."]
pub type OUT_LOOP_TEST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_AUTO_WRBACK` reader - Set this bit to enable automatic outlink writeback when all the data in TX FIFO has been transmitted."]
pub type OUT_AUTO_WRBACK_R = crate::BitReader;
#[doc = "Field `OUT_AUTO_WRBACK` writer - Set this bit to enable automatic outlink writeback when all the data in TX FIFO has been transmitted."]
pub type OUT_AUTO_WRBACK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_NO_RESTART_CLR` reader - Reserved."]
pub type OUT_NO_RESTART_CLR_R = crate::BitReader;
#[doc = "Field `OUT_NO_RESTART_CLR` writer - Reserved."]
pub type OUT_NO_RESTART_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_EOF_MODE` reader - This register is used to specify the generation mode of UHCI_OUT_EOF_INT interrupt. 1: When DMA has popped all data from FIFO. 0: When AHB has pushed all data to FIFO."]
pub type OUT_EOF_MODE_R = crate::BitReader;
#[doc = "Field `OUT_EOF_MODE` writer - This register is used to specify the generation mode of UHCI_OUT_EOF_INT interrupt. 1: When DMA has popped all data from FIFO. 0: When AHB has pushed all data to FIFO."]
pub type OUT_EOF_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART0_CE` reader - Set this bit to link up UHCI and UART0."]
pub type UART0_CE_R = crate::BitReader;
#[doc = "Field `UART0_CE` writer - Set this bit to link up UHCI and UART0."]
pub type UART0_CE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART1_CE` reader - Set this bit to link up UHCI and UART1."]
pub type UART1_CE_R = crate::BitReader;
#[doc = "Field `UART1_CE` writer - Set this bit to link up UHCI and UART1."]
pub type UART1_CE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTDSCR_BURST_EN` reader - This register is used to specify DMA transmit descriptor transfer mode. 1: burst mode. 0: byte mode."]
pub type OUTDSCR_BURST_EN_R = crate::BitReader;
#[doc = "Field `OUTDSCR_BURST_EN` writer - This register is used to specify DMA transmit descriptor transfer mode. 1: burst mode. 0: byte mode."]
pub type OUTDSCR_BURST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INDSCR_BURST_EN` reader - This register is used to specify DMA receive descriptor transfer mode. 1: burst mode. 0: byte mode."]
pub type INDSCR_BURST_EN_R = crate::BitReader;
#[doc = "Field `INDSCR_BURST_EN` writer - This register is used to specify DMA receive descriptor transfer mode. 1: burst mode. 0: byte mode."]
pub type INDSCR_BURST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_TRANS_EN` reader - 1: UHCI transmitted data would be write back into DMA INFIFO."]
pub type MEM_TRANS_EN_R = crate::BitReader;
#[doc = "Field `MEM_TRANS_EN` writer - 1: UHCI transmitted data would be write back into DMA INFIFO."]
pub type MEM_TRANS_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEPER_EN` reader - Set this bit to separate the data frame using a special character."]
pub type SEPER_EN_R = crate::BitReader;
#[doc = "Field `SEPER_EN` writer - Set this bit to separate the data frame using a special character."]
pub type SEPER_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HEAD_EN` reader - Set this bit to encode the data packet with a formatting header."]
pub type HEAD_EN_R = crate::BitReader;
#[doc = "Field `HEAD_EN` writer - Set this bit to encode the data packet with a formatting header."]
pub type HEAD_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRC_REC_EN` reader - Set this bit to enable UHCI to receive the 16 bit CRC."]
pub type CRC_REC_EN_R = crate::BitReader;
#[doc = "Field `CRC_REC_EN` writer - Set this bit to enable UHCI to receive the 16 bit CRC."]
pub type CRC_REC_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART_IDLE_EOF_EN` reader - If this bit is set to 1, UHCI will end the payload receiving process when UART has been in idle state."]
pub type UART_IDLE_EOF_EN_R = crate::BitReader;
#[doc = "Field `UART_IDLE_EOF_EN` writer - If this bit is set to 1, UHCI will end the payload receiving process when UART has been in idle state."]
pub type UART_IDLE_EOF_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEN_EOF_EN` reader - If this bit is set to 1, UHCI decoder stops receiving payload data when the number of received data bytes has reached the specified value. The value is payload length indicated by UCHI packet header when UHCI_HEAD_EN is 1 or the value is a configuration value when UHCI_HEAD_EN is 0. If this bit is set to 0, UHCI decoder stops receiving payload data upon receiving 0xC0."]
pub type LEN_EOF_EN_R = crate::BitReader;
#[doc = "Field `LEN_EOF_EN` writer - If this bit is set to 1, UHCI decoder stops receiving payload data when the number of received data bytes has reached the specified value. The value is payload length indicated by UCHI packet header when UHCI_HEAD_EN is 1 or the value is a configuration value when UHCI_HEAD_EN is 0. If this bit is set to 0, UHCI decoder stops receiving payload data upon receiving 0xC0."]
pub type LEN_EOF_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENCODE_CRC_EN` reader - Set this bit to enable data integrity checking by appending a 16 bit CCITT-CRC to the end of the payload."]
pub type ENCODE_CRC_EN_R = crate::BitReader;
#[doc = "Field `ENCODE_CRC_EN` writer - Set this bit to enable data integrity checking by appending a 16 bit CCITT-CRC to the end of the payload."]
pub type ENCODE_CRC_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_EN` reader - 1: Force clock on for registers. 0: Support clock only when application writes registers."]
pub type CLK_EN_R = crate::BitReader;
#[doc = "Field `CLK_EN` writer - 1: Force clock on for registers. 0: Support clock only when application writes registers."]
pub type CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART_RX_BRK_EOF_EN` reader - If this bit is set to 1, UHCI stops receiving payload data when a NULL frame is received by UART."]
pub type UART_RX_BRK_EOF_EN_R = crate::BitReader;
#[doc = "Field `UART_RX_BRK_EOF_EN` writer - If this bit is set to 1, UHCI stops receiving payload data when a NULL frame is received by UART."]
pub type UART_RX_BRK_EOF_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set this bit to reset in DMA FSM."]
    #[inline(always)]
    pub fn in_rst(&self) -> IN_RST_R {
        IN_RST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set this bit to reset out DMA FSM."]
    #[inline(always)]
    pub fn out_rst(&self) -> OUT_RST_R {
        OUT_RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set this bit to reset AHB interface cmdFIFO of DMA."]
    #[inline(always)]
    pub fn ahbm_fifo_rst(&self) -> AHBM_FIFO_RST_R {
        AHBM_FIFO_RST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set this bit to reset AHB interface of DMA."]
    #[inline(always)]
    pub fn ahbm_rst(&self) -> AHBM_RST_R {
        AHBM_RST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Reserved."]
    #[inline(always)]
    pub fn in_loop_test(&self) -> IN_LOOP_TEST_R {
        IN_LOOP_TEST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Reserved."]
    #[inline(always)]
    pub fn out_loop_test(&self) -> OUT_LOOP_TEST_R {
        OUT_LOOP_TEST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Set this bit to enable automatic outlink writeback when all the data in TX FIFO has been transmitted."]
    #[inline(always)]
    pub fn out_auto_wrback(&self) -> OUT_AUTO_WRBACK_R {
        OUT_AUTO_WRBACK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Reserved."]
    #[inline(always)]
    pub fn out_no_restart_clr(&self) -> OUT_NO_RESTART_CLR_R {
        OUT_NO_RESTART_CLR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - This register is used to specify the generation mode of UHCI_OUT_EOF_INT interrupt. 1: When DMA has popped all data from FIFO. 0: When AHB has pushed all data to FIFO."]
    #[inline(always)]
    pub fn out_eof_mode(&self) -> OUT_EOF_MODE_R {
        OUT_EOF_MODE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Set this bit to link up UHCI and UART0."]
    #[inline(always)]
    pub fn uart0_ce(&self) -> UART0_CE_R {
        UART0_CE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Set this bit to link up UHCI and UART1."]
    #[inline(always)]
    pub fn uart1_ce(&self) -> UART1_CE_R {
        UART1_CE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - This register is used to specify DMA transmit descriptor transfer mode. 1: burst mode. 0: byte mode."]
    #[inline(always)]
    pub fn outdscr_burst_en(&self) -> OUTDSCR_BURST_EN_R {
        OUTDSCR_BURST_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - This register is used to specify DMA receive descriptor transfer mode. 1: burst mode. 0: byte mode."]
    #[inline(always)]
    pub fn indscr_burst_en(&self) -> INDSCR_BURST_EN_R {
        INDSCR_BURST_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - 1: UHCI transmitted data would be write back into DMA INFIFO."]
    #[inline(always)]
    pub fn mem_trans_en(&self) -> MEM_TRANS_EN_R {
        MEM_TRANS_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Set this bit to separate the data frame using a special character."]
    #[inline(always)]
    pub fn seper_en(&self) -> SEPER_EN_R {
        SEPER_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Set this bit to encode the data packet with a formatting header."]
    #[inline(always)]
    pub fn head_en(&self) -> HEAD_EN_R {
        HEAD_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Set this bit to enable UHCI to receive the 16 bit CRC."]
    #[inline(always)]
    pub fn crc_rec_en(&self) -> CRC_REC_EN_R {
        CRC_REC_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - If this bit is set to 1, UHCI will end the payload receiving process when UART has been in idle state."]
    #[inline(always)]
    pub fn uart_idle_eof_en(&self) -> UART_IDLE_EOF_EN_R {
        UART_IDLE_EOF_EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - If this bit is set to 1, UHCI decoder stops receiving payload data when the number of received data bytes has reached the specified value. The value is payload length indicated by UCHI packet header when UHCI_HEAD_EN is 1 or the value is a configuration value when UHCI_HEAD_EN is 0. If this bit is set to 0, UHCI decoder stops receiving payload data upon receiving 0xC0."]
    #[inline(always)]
    pub fn len_eof_en(&self) -> LEN_EOF_EN_R {
        LEN_EOF_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Set this bit to enable data integrity checking by appending a 16 bit CCITT-CRC to the end of the payload."]
    #[inline(always)]
    pub fn encode_crc_en(&self) -> ENCODE_CRC_EN_R {
        ENCODE_CRC_EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 1: Force clock on for registers. 0: Support clock only when application writes registers."]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - If this bit is set to 1, UHCI stops receiving payload data when a NULL frame is received by UART."]
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
            .field("outdscr_burst_en", &self.outdscr_burst_en())
            .field("indscr_burst_en", &self.indscr_burst_en())
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
    #[doc = "Bit 0 - Set this bit to reset in DMA FSM."]
    #[inline(always)]
    pub fn in_rst(&mut self) -> IN_RST_W<CONF0_SPEC> {
        IN_RST_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set this bit to reset out DMA FSM."]
    #[inline(always)]
    pub fn out_rst(&mut self) -> OUT_RST_W<CONF0_SPEC> {
        OUT_RST_W::new(self, 1)
    }
    #[doc = "Bit 2 - Set this bit to reset AHB interface cmdFIFO of DMA."]
    #[inline(always)]
    pub fn ahbm_fifo_rst(&mut self) -> AHBM_FIFO_RST_W<CONF0_SPEC> {
        AHBM_FIFO_RST_W::new(self, 2)
    }
    #[doc = "Bit 3 - Set this bit to reset AHB interface of DMA."]
    #[inline(always)]
    pub fn ahbm_rst(&mut self) -> AHBM_RST_W<CONF0_SPEC> {
        AHBM_RST_W::new(self, 3)
    }
    #[doc = "Bit 4 - Reserved."]
    #[inline(always)]
    pub fn in_loop_test(&mut self) -> IN_LOOP_TEST_W<CONF0_SPEC> {
        IN_LOOP_TEST_W::new(self, 4)
    }
    #[doc = "Bit 5 - Reserved."]
    #[inline(always)]
    pub fn out_loop_test(&mut self) -> OUT_LOOP_TEST_W<CONF0_SPEC> {
        OUT_LOOP_TEST_W::new(self, 5)
    }
    #[doc = "Bit 6 - Set this bit to enable automatic outlink writeback when all the data in TX FIFO has been transmitted."]
    #[inline(always)]
    pub fn out_auto_wrback(&mut self) -> OUT_AUTO_WRBACK_W<CONF0_SPEC> {
        OUT_AUTO_WRBACK_W::new(self, 6)
    }
    #[doc = "Bit 7 - Reserved."]
    #[inline(always)]
    pub fn out_no_restart_clr(&mut self) -> OUT_NO_RESTART_CLR_W<CONF0_SPEC> {
        OUT_NO_RESTART_CLR_W::new(self, 7)
    }
    #[doc = "Bit 8 - This register is used to specify the generation mode of UHCI_OUT_EOF_INT interrupt. 1: When DMA has popped all data from FIFO. 0: When AHB has pushed all data to FIFO."]
    #[inline(always)]
    pub fn out_eof_mode(&mut self) -> OUT_EOF_MODE_W<CONF0_SPEC> {
        OUT_EOF_MODE_W::new(self, 8)
    }
    #[doc = "Bit 9 - Set this bit to link up UHCI and UART0."]
    #[inline(always)]
    pub fn uart0_ce(&mut self) -> UART0_CE_W<CONF0_SPEC> {
        UART0_CE_W::new(self, 9)
    }
    #[doc = "Bit 10 - Set this bit to link up UHCI and UART1."]
    #[inline(always)]
    pub fn uart1_ce(&mut self) -> UART1_CE_W<CONF0_SPEC> {
        UART1_CE_W::new(self, 10)
    }
    #[doc = "Bit 12 - This register is used to specify DMA transmit descriptor transfer mode. 1: burst mode. 0: byte mode."]
    #[inline(always)]
    pub fn outdscr_burst_en(&mut self) -> OUTDSCR_BURST_EN_W<CONF0_SPEC> {
        OUTDSCR_BURST_EN_W::new(self, 12)
    }
    #[doc = "Bit 13 - This register is used to specify DMA receive descriptor transfer mode. 1: burst mode. 0: byte mode."]
    #[inline(always)]
    pub fn indscr_burst_en(&mut self) -> INDSCR_BURST_EN_W<CONF0_SPEC> {
        INDSCR_BURST_EN_W::new(self, 13)
    }
    #[doc = "Bit 15 - 1: UHCI transmitted data would be write back into DMA INFIFO."]
    #[inline(always)]
    pub fn mem_trans_en(&mut self) -> MEM_TRANS_EN_W<CONF0_SPEC> {
        MEM_TRANS_EN_W::new(self, 15)
    }
    #[doc = "Bit 16 - Set this bit to separate the data frame using a special character."]
    #[inline(always)]
    pub fn seper_en(&mut self) -> SEPER_EN_W<CONF0_SPEC> {
        SEPER_EN_W::new(self, 16)
    }
    #[doc = "Bit 17 - Set this bit to encode the data packet with a formatting header."]
    #[inline(always)]
    pub fn head_en(&mut self) -> HEAD_EN_W<CONF0_SPEC> {
        HEAD_EN_W::new(self, 17)
    }
    #[doc = "Bit 18 - Set this bit to enable UHCI to receive the 16 bit CRC."]
    #[inline(always)]
    pub fn crc_rec_en(&mut self) -> CRC_REC_EN_W<CONF0_SPEC> {
        CRC_REC_EN_W::new(self, 18)
    }
    #[doc = "Bit 19 - If this bit is set to 1, UHCI will end the payload receiving process when UART has been in idle state."]
    #[inline(always)]
    pub fn uart_idle_eof_en(&mut self) -> UART_IDLE_EOF_EN_W<CONF0_SPEC> {
        UART_IDLE_EOF_EN_W::new(self, 19)
    }
    #[doc = "Bit 20 - If this bit is set to 1, UHCI decoder stops receiving payload data when the number of received data bytes has reached the specified value. The value is payload length indicated by UCHI packet header when UHCI_HEAD_EN is 1 or the value is a configuration value when UHCI_HEAD_EN is 0. If this bit is set to 0, UHCI decoder stops receiving payload data upon receiving 0xC0."]
    #[inline(always)]
    pub fn len_eof_en(&mut self) -> LEN_EOF_EN_W<CONF0_SPEC> {
        LEN_EOF_EN_W::new(self, 20)
    }
    #[doc = "Bit 21 - Set this bit to enable data integrity checking by appending a 16 bit CCITT-CRC to the end of the payload."]
    #[inline(always)]
    pub fn encode_crc_en(&mut self) -> ENCODE_CRC_EN_W<CONF0_SPEC> {
        ENCODE_CRC_EN_W::new(self, 21)
    }
    #[doc = "Bit 22 - 1: Force clock on for registers. 0: Support clock only when application writes registers."]
    #[inline(always)]
    pub fn clk_en(&mut self) -> CLK_EN_W<CONF0_SPEC> {
        CLK_EN_W::new(self, 22)
    }
    #[doc = "Bit 23 - If this bit is set to 1, UHCI stops receiving payload data when a NULL frame is received by UART."]
    #[inline(always)]
    pub fn uart_rx_brk_eof_en(&mut self) -> UART_RX_BRK_EOF_EN_W<CONF0_SPEC> {
        UART_RX_BRK_EOF_EN_W::new(self, 23)
    }
}
#[doc = "UHCI configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`conf0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`conf0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONF0_SPEC;
impl crate::RegisterSpec for CONF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`conf0::R`](R) reader structure"]
impl crate::Readable for CONF0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`conf0::W`](W) writer structure"]
impl crate::Writable for CONF0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONF0 to value 0x0037_0100"]
impl crate::Resettable for CONF0_SPEC {
    const RESET_VALUE: u32 = 0x0037_0100;
}
