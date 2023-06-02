#[doc = "Register `CONF0` reader"]
pub struct R(crate::R<CONF0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONF0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONF0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONF0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONF0` writer"]
pub struct W(crate::W<CONF0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONF0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CONF0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONF0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IN_RST` reader - Set this bit to reset in DMA FSM."]
pub type IN_RST_R = crate::BitReader;
#[doc = "Field `IN_RST` writer - Set this bit to reset in DMA FSM."]
pub type IN_RST_W<'a, const O: u8> = crate::BitWriter<'a, CONF0_SPEC, O>;
#[doc = "Field `OUT_RST` reader - Set this bit to reset out DMA FSM."]
pub type OUT_RST_R = crate::BitReader;
#[doc = "Field `OUT_RST` writer - Set this bit to reset out DMA FSM."]
pub type OUT_RST_W<'a, const O: u8> = crate::BitWriter<'a, CONF0_SPEC, O>;
#[doc = "Field `AHBM_FIFO_RST` reader - Set this bit to reset AHB interface cmdFIFO of DMA."]
pub type AHBM_FIFO_RST_R = crate::BitReader;
#[doc = "Field `AHBM_FIFO_RST` writer - Set this bit to reset AHB interface cmdFIFO of DMA."]
pub type AHBM_FIFO_RST_W<'a, const O: u8> = crate::BitWriter<'a, CONF0_SPEC, O>;
#[doc = "Field `AHBM_RST` reader - Set this bit to reset AHB interface of DMA."]
pub type AHBM_RST_R = crate::BitReader;
#[doc = "Field `AHBM_RST` writer - Set this bit to reset AHB interface of DMA."]
pub type AHBM_RST_W<'a, const O: u8> = crate::BitWriter<'a, CONF0_SPEC, O>;
#[doc = "Field `IN_LOOP_TEST` reader - Reserved."]
pub type IN_LOOP_TEST_R = crate::BitReader;
#[doc = "Field `IN_LOOP_TEST` writer - Reserved."]
pub type IN_LOOP_TEST_W<'a, const O: u8> = crate::BitWriter<'a, CONF0_SPEC, O>;
#[doc = "Field `OUT_LOOP_TEST` reader - Reserved."]
pub type OUT_LOOP_TEST_R = crate::BitReader;
#[doc = "Field `OUT_LOOP_TEST` writer - Reserved."]
pub type OUT_LOOP_TEST_W<'a, const O: u8> = crate::BitWriter<'a, CONF0_SPEC, O>;
#[doc = "Field `OUT_AUTO_WRBACK` reader - Set this bit to enable automatic outlink writeback when all the data in TX FIFO has been transmitted."]
pub type OUT_AUTO_WRBACK_R = crate::BitReader;
#[doc = "Field `OUT_AUTO_WRBACK` writer - Set this bit to enable automatic outlink writeback when all the data in TX FIFO has been transmitted."]
pub type OUT_AUTO_WRBACK_W<'a, const O: u8> = crate::BitWriter<'a, CONF0_SPEC, O>;
#[doc = "Field `OUT_NO_RESTART_CLR` reader - Reserved."]
pub type OUT_NO_RESTART_CLR_R = crate::BitReader;
#[doc = "Field `OUT_NO_RESTART_CLR` writer - Reserved."]
pub type OUT_NO_RESTART_CLR_W<'a, const O: u8> = crate::BitWriter<'a, CONF0_SPEC, O>;
#[doc = "Field `OUT_EOF_MODE` reader - This register is used to specify the generation mode of UHCI_OUT_EOF_INT interrupt. 1: When DMA has popped all data from FIFO. 0: When AHB has pushed all data to FIFO."]
pub type OUT_EOF_MODE_R = crate::BitReader;
#[doc = "Field `OUT_EOF_MODE` writer - This register is used to specify the generation mode of UHCI_OUT_EOF_INT interrupt. 1: When DMA has popped all data from FIFO. 0: When AHB has pushed all data to FIFO."]
pub type OUT_EOF_MODE_W<'a, const O: u8> = crate::BitWriter<'a, CONF0_SPEC, O>;
#[doc = "Field `UART0_CE` reader - Set this bit to link up UHCI and UART0."]
pub type UART0_CE_R = crate::BitReader;
#[doc = "Field `UART0_CE` writer - Set this bit to link up UHCI and UART0."]
pub type UART0_CE_W<'a, const O: u8> = crate::BitWriter<'a, CONF0_SPEC, O>;
#[doc = "Field `UART1_CE` reader - Set this bit to link up UHCI and UART1."]
pub type UART1_CE_R = crate::BitReader;
#[doc = "Field `UART1_CE` writer - Set this bit to link up UHCI and UART1."]
pub type UART1_CE_W<'a, const O: u8> = crate::BitWriter<'a, CONF0_SPEC, O>;
#[doc = "Field `OUTDSCR_BURST_EN` reader - This register is used to specify DMA transmit descriptor transfer mode. 1: burst mode. 0: byte mode."]
pub type OUTDSCR_BURST_EN_R = crate::BitReader;
#[doc = "Field `OUTDSCR_BURST_EN` writer - This register is used to specify DMA transmit descriptor transfer mode. 1: burst mode. 0: byte mode."]
pub type OUTDSCR_BURST_EN_W<'a, const O: u8> = crate::BitWriter<'a, CONF0_SPEC, O>;
#[doc = "Field `INDSCR_BURST_EN` reader - This register is used to specify DMA receive descriptor transfer mode. 1: burst mode. 0: byte mode."]
pub type INDSCR_BURST_EN_R = crate::BitReader;
#[doc = "Field `INDSCR_BURST_EN` writer - This register is used to specify DMA receive descriptor transfer mode. 1: burst mode. 0: byte mode."]
pub type INDSCR_BURST_EN_W<'a, const O: u8> = crate::BitWriter<'a, CONF0_SPEC, O>;
#[doc = "Field `MEM_TRANS_EN` reader - 1: UHCI transmitted data would be write back into DMA INFIFO."]
pub type MEM_TRANS_EN_R = crate::BitReader;
#[doc = "Field `MEM_TRANS_EN` writer - 1: UHCI transmitted data would be write back into DMA INFIFO."]
pub type MEM_TRANS_EN_W<'a, const O: u8> = crate::BitWriter<'a, CONF0_SPEC, O>;
#[doc = "Field `SEPER_EN` reader - Set this bit to separate the data frame using a special character."]
pub type SEPER_EN_R = crate::BitReader;
#[doc = "Field `SEPER_EN` writer - Set this bit to separate the data frame using a special character."]
pub type SEPER_EN_W<'a, const O: u8> = crate::BitWriter<'a, CONF0_SPEC, O>;
#[doc = "Field `HEAD_EN` reader - Set this bit to encode the data packet with a formatting header."]
pub type HEAD_EN_R = crate::BitReader;
#[doc = "Field `HEAD_EN` writer - Set this bit to encode the data packet with a formatting header."]
pub type HEAD_EN_W<'a, const O: u8> = crate::BitWriter<'a, CONF0_SPEC, O>;
#[doc = "Field `CRC_REC_EN` reader - Set this bit to enable UHCI to receive the 16 bit CRC."]
pub type CRC_REC_EN_R = crate::BitReader;
#[doc = "Field `CRC_REC_EN` writer - Set this bit to enable UHCI to receive the 16 bit CRC."]
pub type CRC_REC_EN_W<'a, const O: u8> = crate::BitWriter<'a, CONF0_SPEC, O>;
#[doc = "Field `UART_IDLE_EOF_EN` reader - If this bit is set to 1, UHCI will end the payload receiving process when UART has been in idle state."]
pub type UART_IDLE_EOF_EN_R = crate::BitReader;
#[doc = "Field `UART_IDLE_EOF_EN` writer - If this bit is set to 1, UHCI will end the payload receiving process when UART has been in idle state."]
pub type UART_IDLE_EOF_EN_W<'a, const O: u8> = crate::BitWriter<'a, CONF0_SPEC, O>;
#[doc = "Field `LEN_EOF_EN` reader - If this bit is set to 1, UHCI decoder stops receiving payload data when the number of received data bytes has reached the specified value. The value is payload length indicated by UCHI packet header when UHCI_HEAD_EN is 1 or the value is a configuration value when UHCI_HEAD_EN is 0. If this bit is set to 0, UHCI decoder stops receiving payload data upon receiving 0xC0."]
pub type LEN_EOF_EN_R = crate::BitReader;
#[doc = "Field `LEN_EOF_EN` writer - If this bit is set to 1, UHCI decoder stops receiving payload data when the number of received data bytes has reached the specified value. The value is payload length indicated by UCHI packet header when UHCI_HEAD_EN is 1 or the value is a configuration value when UHCI_HEAD_EN is 0. If this bit is set to 0, UHCI decoder stops receiving payload data upon receiving 0xC0."]
pub type LEN_EOF_EN_W<'a, const O: u8> = crate::BitWriter<'a, CONF0_SPEC, O>;
#[doc = "Field `ENCODE_CRC_EN` reader - Set this bit to enable data integrity checking by appending a 16 bit CCITT-CRC to the end of the payload."]
pub type ENCODE_CRC_EN_R = crate::BitReader;
#[doc = "Field `ENCODE_CRC_EN` writer - Set this bit to enable data integrity checking by appending a 16 bit CCITT-CRC to the end of the payload."]
pub type ENCODE_CRC_EN_W<'a, const O: u8> = crate::BitWriter<'a, CONF0_SPEC, O>;
#[doc = "Field `CLK_EN` reader - 1: Force clock on for registers. 0: Support clock only when application writes registers."]
pub type CLK_EN_R = crate::BitReader;
#[doc = "Field `CLK_EN` writer - 1: Force clock on for registers. 0: Support clock only when application writes registers."]
pub type CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, CONF0_SPEC, O>;
#[doc = "Field `UART_RX_BRK_EOF_EN` reader - If this bit is set to 1, UHCI stops receiving payload data when a NULL frame is received by UART."]
pub type UART_RX_BRK_EOF_EN_R = crate::BitReader;
#[doc = "Field `UART_RX_BRK_EOF_EN` writer - If this bit is set to 1, UHCI stops receiving payload data when a NULL frame is received by UART."]
pub type UART_RX_BRK_EOF_EN_W<'a, const O: u8> = crate::BitWriter<'a, CONF0_SPEC, O>;
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
            .field("in_rst", &format_args!("{}", self.in_rst().bit()))
            .field("out_rst", &format_args!("{}", self.out_rst().bit()))
            .field(
                "ahbm_fifo_rst",
                &format_args!("{}", self.ahbm_fifo_rst().bit()),
            )
            .field("ahbm_rst", &format_args!("{}", self.ahbm_rst().bit()))
            .field(
                "in_loop_test",
                &format_args!("{}", self.in_loop_test().bit()),
            )
            .field(
                "out_loop_test",
                &format_args!("{}", self.out_loop_test().bit()),
            )
            .field(
                "out_auto_wrback",
                &format_args!("{}", self.out_auto_wrback().bit()),
            )
            .field(
                "out_no_restart_clr",
                &format_args!("{}", self.out_no_restart_clr().bit()),
            )
            .field(
                "out_eof_mode",
                &format_args!("{}", self.out_eof_mode().bit()),
            )
            .field("uart0_ce", &format_args!("{}", self.uart0_ce().bit()))
            .field("uart1_ce", &format_args!("{}", self.uart1_ce().bit()))
            .field(
                "outdscr_burst_en",
                &format_args!("{}", self.outdscr_burst_en().bit()),
            )
            .field(
                "indscr_burst_en",
                &format_args!("{}", self.indscr_burst_en().bit()),
            )
            .field(
                "mem_trans_en",
                &format_args!("{}", self.mem_trans_en().bit()),
            )
            .field("seper_en", &format_args!("{}", self.seper_en().bit()))
            .field("head_en", &format_args!("{}", self.head_en().bit()))
            .field("crc_rec_en", &format_args!("{}", self.crc_rec_en().bit()))
            .field(
                "uart_idle_eof_en",
                &format_args!("{}", self.uart_idle_eof_en().bit()),
            )
            .field("len_eof_en", &format_args!("{}", self.len_eof_en().bit()))
            .field(
                "encode_crc_en",
                &format_args!("{}", self.encode_crc_en().bit()),
            )
            .field("clk_en", &format_args!("{}", self.clk_en().bit()))
            .field(
                "uart_rx_brk_eof_en",
                &format_args!("{}", self.uart_rx_brk_eof_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CONF0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to reset in DMA FSM."]
    #[inline(always)]
    #[must_use]
    pub fn in_rst(&mut self) -> IN_RST_W<0> {
        IN_RST_W::new(self)
    }
    #[doc = "Bit 1 - Set this bit to reset out DMA FSM."]
    #[inline(always)]
    #[must_use]
    pub fn out_rst(&mut self) -> OUT_RST_W<1> {
        OUT_RST_W::new(self)
    }
    #[doc = "Bit 2 - Set this bit to reset AHB interface cmdFIFO of DMA."]
    #[inline(always)]
    #[must_use]
    pub fn ahbm_fifo_rst(&mut self) -> AHBM_FIFO_RST_W<2> {
        AHBM_FIFO_RST_W::new(self)
    }
    #[doc = "Bit 3 - Set this bit to reset AHB interface of DMA."]
    #[inline(always)]
    #[must_use]
    pub fn ahbm_rst(&mut self) -> AHBM_RST_W<3> {
        AHBM_RST_W::new(self)
    }
    #[doc = "Bit 4 - Reserved."]
    #[inline(always)]
    #[must_use]
    pub fn in_loop_test(&mut self) -> IN_LOOP_TEST_W<4> {
        IN_LOOP_TEST_W::new(self)
    }
    #[doc = "Bit 5 - Reserved."]
    #[inline(always)]
    #[must_use]
    pub fn out_loop_test(&mut self) -> OUT_LOOP_TEST_W<5> {
        OUT_LOOP_TEST_W::new(self)
    }
    #[doc = "Bit 6 - Set this bit to enable automatic outlink writeback when all the data in TX FIFO has been transmitted."]
    #[inline(always)]
    #[must_use]
    pub fn out_auto_wrback(&mut self) -> OUT_AUTO_WRBACK_W<6> {
        OUT_AUTO_WRBACK_W::new(self)
    }
    #[doc = "Bit 7 - Reserved."]
    #[inline(always)]
    #[must_use]
    pub fn out_no_restart_clr(&mut self) -> OUT_NO_RESTART_CLR_W<7> {
        OUT_NO_RESTART_CLR_W::new(self)
    }
    #[doc = "Bit 8 - This register is used to specify the generation mode of UHCI_OUT_EOF_INT interrupt. 1: When DMA has popped all data from FIFO. 0: When AHB has pushed all data to FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn out_eof_mode(&mut self) -> OUT_EOF_MODE_W<8> {
        OUT_EOF_MODE_W::new(self)
    }
    #[doc = "Bit 9 - Set this bit to link up UHCI and UART0."]
    #[inline(always)]
    #[must_use]
    pub fn uart0_ce(&mut self) -> UART0_CE_W<9> {
        UART0_CE_W::new(self)
    }
    #[doc = "Bit 10 - Set this bit to link up UHCI and UART1."]
    #[inline(always)]
    #[must_use]
    pub fn uart1_ce(&mut self) -> UART1_CE_W<10> {
        UART1_CE_W::new(self)
    }
    #[doc = "Bit 12 - This register is used to specify DMA transmit descriptor transfer mode. 1: burst mode. 0: byte mode."]
    #[inline(always)]
    #[must_use]
    pub fn outdscr_burst_en(&mut self) -> OUTDSCR_BURST_EN_W<12> {
        OUTDSCR_BURST_EN_W::new(self)
    }
    #[doc = "Bit 13 - This register is used to specify DMA receive descriptor transfer mode. 1: burst mode. 0: byte mode."]
    #[inline(always)]
    #[must_use]
    pub fn indscr_burst_en(&mut self) -> INDSCR_BURST_EN_W<13> {
        INDSCR_BURST_EN_W::new(self)
    }
    #[doc = "Bit 15 - 1: UHCI transmitted data would be write back into DMA INFIFO."]
    #[inline(always)]
    #[must_use]
    pub fn mem_trans_en(&mut self) -> MEM_TRANS_EN_W<15> {
        MEM_TRANS_EN_W::new(self)
    }
    #[doc = "Bit 16 - Set this bit to separate the data frame using a special character."]
    #[inline(always)]
    #[must_use]
    pub fn seper_en(&mut self) -> SEPER_EN_W<16> {
        SEPER_EN_W::new(self)
    }
    #[doc = "Bit 17 - Set this bit to encode the data packet with a formatting header."]
    #[inline(always)]
    #[must_use]
    pub fn head_en(&mut self) -> HEAD_EN_W<17> {
        HEAD_EN_W::new(self)
    }
    #[doc = "Bit 18 - Set this bit to enable UHCI to receive the 16 bit CRC."]
    #[inline(always)]
    #[must_use]
    pub fn crc_rec_en(&mut self) -> CRC_REC_EN_W<18> {
        CRC_REC_EN_W::new(self)
    }
    #[doc = "Bit 19 - If this bit is set to 1, UHCI will end the payload receiving process when UART has been in idle state."]
    #[inline(always)]
    #[must_use]
    pub fn uart_idle_eof_en(&mut self) -> UART_IDLE_EOF_EN_W<19> {
        UART_IDLE_EOF_EN_W::new(self)
    }
    #[doc = "Bit 20 - If this bit is set to 1, UHCI decoder stops receiving payload data when the number of received data bytes has reached the specified value. The value is payload length indicated by UCHI packet header when UHCI_HEAD_EN is 1 or the value is a configuration value when UHCI_HEAD_EN is 0. If this bit is set to 0, UHCI decoder stops receiving payload data upon receiving 0xC0."]
    #[inline(always)]
    #[must_use]
    pub fn len_eof_en(&mut self) -> LEN_EOF_EN_W<20> {
        LEN_EOF_EN_W::new(self)
    }
    #[doc = "Bit 21 - Set this bit to enable data integrity checking by appending a 16 bit CCITT-CRC to the end of the payload."]
    #[inline(always)]
    #[must_use]
    pub fn encode_crc_en(&mut self) -> ENCODE_CRC_EN_W<21> {
        ENCODE_CRC_EN_W::new(self)
    }
    #[doc = "Bit 22 - 1: Force clock on for registers. 0: Support clock only when application writes registers."]
    #[inline(always)]
    #[must_use]
    pub fn clk_en(&mut self) -> CLK_EN_W<22> {
        CLK_EN_W::new(self)
    }
    #[doc = "Bit 23 - If this bit is set to 1, UHCI stops receiving payload data when a NULL frame is received by UART."]
    #[inline(always)]
    #[must_use]
    pub fn uart_rx_brk_eof_en(&mut self) -> UART_RX_BRK_EOF_EN_W<23> {
        UART_RX_BRK_EOF_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UHCI configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conf0](index.html) module"]
pub struct CONF0_SPEC;
impl crate::RegisterSpec for CONF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [conf0::R](R) reader structure"]
impl crate::Readable for CONF0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [conf0::W](W) writer structure"]
impl crate::Writable for CONF0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONF0 to value 0x0037_0100"]
impl crate::Resettable for CONF0_SPEC {
    const RESET_VALUE: Self::Ux = 0x0037_0100;
}
