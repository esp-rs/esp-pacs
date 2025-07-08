#[doc = "Register `CONF0` reader"]
pub type R = crate::R<CONF0_SPEC>;
#[doc = "Register `CONF0` writer"]
pub type W = crate::W<CONF0_SPEC>;
#[doc = "Field `TX_RST` reader - Write 1 and then write 0 to reset the decoder state machine."]
pub type TX_RST_R = crate::BitReader;
#[doc = "Field `TX_RST` writer - Write 1 and then write 0 to reset the decoder state machine."]
pub type TX_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_RST` reader - Write 1 and then write 0 to reset the encoder state machine."]
pub type RX_RST_R = crate::BitReader;
#[doc = "Field `RX_RST` writer - Write 1 and then write 0 to reset the encoder state machine."]
pub type RX_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART_SEL` reader - Configures to select which uart to connect with UHCI.\\\\ 0: UART0\\\\ 1: UART1\\\\"]
pub type UART_SEL_R = crate::FieldReader;
#[doc = "Field `UART_SEL` writer - Configures to select which uart to connect with UHCI.\\\\ 0: UART0\\\\ 1: UART1\\\\"]
pub type UART_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SEPER_EN` reader - Configures whether or not to separate the data frame with a special character.\\\\ 0: Not separate\\\\ 1: Separate\\\\"]
pub type SEPER_EN_R = crate::BitReader;
#[doc = "Field `SEPER_EN` writer - Configures whether or not to separate the data frame with a special character.\\\\ 0: Not separate\\\\ 1: Separate\\\\"]
pub type SEPER_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HEAD_EN` reader - Configures whether or not to encode the data packet with a formatting header.\\\\ 0: Not use formatting header\\\\ 1: Use formatting header\\\\"]
pub type HEAD_EN_R = crate::BitReader;
#[doc = "Field `HEAD_EN` writer - Configures whether or not to encode the data packet with a formatting header.\\\\ 0: Not use formatting header\\\\ 1: Use formatting header\\\\"]
pub type HEAD_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRC_REC_EN` reader - Configures whether or not to enable the reception of the 16-bit CRC.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
pub type CRC_REC_EN_R = crate::BitReader;
#[doc = "Field `CRC_REC_EN` writer - Configures whether or not to enable the reception of the 16-bit CRC.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
pub type CRC_REC_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART_IDLE_EOF_EN` reader - Configures whether or not to stop receiving data when UART is idle.\\\\ 0: Not stop\\\\ 1: Stop\\\\"]
pub type UART_IDLE_EOF_EN_R = crate::BitReader;
#[doc = "Field `UART_IDLE_EOF_EN` writer - Configures whether or not to stop receiving data when UART is idle.\\\\ 0: Not stop\\\\ 1: Stop\\\\"]
pub type UART_IDLE_EOF_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEN_EOF_EN` reader - Configures when the UHCI decoder stops receiving data.\\\\ 0: Stops after receiving 0xC0\\\\ 1: Stops when the number of received data bytes reach the specified value. When UHCI_HEAD_EN is 1, the specified value is the data length indicated by the UHCI packet header. when UHCI_HEAD_EN is 0, the specified value is the configured value.\\\\"]
pub type LEN_EOF_EN_R = crate::BitReader;
#[doc = "Field `LEN_EOF_EN` writer - Configures when the UHCI decoder stops receiving data.\\\\ 0: Stops after receiving 0xC0\\\\ 1: Stops when the number of received data bytes reach the specified value. When UHCI_HEAD_EN is 1, the specified value is the data length indicated by the UHCI packet header. when UHCI_HEAD_EN is 0, the specified value is the configured value.\\\\"]
pub type LEN_EOF_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENCODE_CRC_EN` reader - Configures whether or not to enable data integrity check by appending a 16 bit CCITT-CRC to the end of the data.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
pub type ENCODE_CRC_EN_R = crate::BitReader;
#[doc = "Field `ENCODE_CRC_EN` writer - Configures whether or not to enable data integrity check by appending a 16 bit CCITT-CRC to the end of the data.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
pub type ENCODE_CRC_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_EN` reader - Configures clock gating.\\\\ 0: Support clock only when the application writes registers.\\\\ 1: Always force the clock on for registers.\\\\"]
pub type CLK_EN_R = crate::BitReader;
#[doc = "Field `CLK_EN` writer - Configures clock gating.\\\\ 0: Support clock only when the application writes registers.\\\\ 1: Always force the clock on for registers.\\\\"]
pub type CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART_RX_BRK_EOF_EN` reader - Configures whether or not to stop UHCI from receiving data after UART has received a NULL frame.\\\\ 0: Not stop\\\\ 1: Stop\\\\"]
pub type UART_RX_BRK_EOF_EN_R = crate::BitReader;
#[doc = "Field `UART_RX_BRK_EOF_EN` writer - Configures whether or not to stop UHCI from receiving data after UART has received a NULL frame.\\\\ 0: Not stop\\\\ 1: Stop\\\\"]
pub type UART_RX_BRK_EOF_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Write 1 and then write 0 to reset the decoder state machine."]
    #[inline(always)]
    pub fn tx_rst(&self) -> TX_RST_R {
        TX_RST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write 1 and then write 0 to reset the encoder state machine."]
    #[inline(always)]
    pub fn rx_rst(&self) -> RX_RST_R {
        RX_RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:4 - Configures to select which uart to connect with UHCI.\\\\ 0: UART0\\\\ 1: UART1\\\\"]
    #[inline(always)]
    pub fn uart_sel(&self) -> UART_SEL_R {
        UART_SEL_R::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bit 5 - Configures whether or not to separate the data frame with a special character.\\\\ 0: Not separate\\\\ 1: Separate\\\\"]
    #[inline(always)]
    pub fn seper_en(&self) -> SEPER_EN_R {
        SEPER_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Configures whether or not to encode the data packet with a formatting header.\\\\ 0: Not use formatting header\\\\ 1: Use formatting header\\\\"]
    #[inline(always)]
    pub fn head_en(&self) -> HEAD_EN_R {
        HEAD_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Configures whether or not to enable the reception of the 16-bit CRC.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn crc_rec_en(&self) -> CRC_REC_EN_R {
        CRC_REC_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Configures whether or not to stop receiving data when UART is idle.\\\\ 0: Not stop\\\\ 1: Stop\\\\"]
    #[inline(always)]
    pub fn uart_idle_eof_en(&self) -> UART_IDLE_EOF_EN_R {
        UART_IDLE_EOF_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Configures when the UHCI decoder stops receiving data.\\\\ 0: Stops after receiving 0xC0\\\\ 1: Stops when the number of received data bytes reach the specified value. When UHCI_HEAD_EN is 1, the specified value is the data length indicated by the UHCI packet header. when UHCI_HEAD_EN is 0, the specified value is the configured value.\\\\"]
    #[inline(always)]
    pub fn len_eof_en(&self) -> LEN_EOF_EN_R {
        LEN_EOF_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Configures whether or not to enable data integrity check by appending a 16 bit CCITT-CRC to the end of the data.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn encode_crc_en(&self) -> ENCODE_CRC_EN_R {
        ENCODE_CRC_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Configures clock gating.\\\\ 0: Support clock only when the application writes registers.\\\\ 1: Always force the clock on for registers.\\\\"]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Configures whether or not to stop UHCI from receiving data after UART has received a NULL frame.\\\\ 0: Not stop\\\\ 1: Stop\\\\"]
    #[inline(always)]
    pub fn uart_rx_brk_eof_en(&self) -> UART_RX_BRK_EOF_EN_R {
        UART_RX_BRK_EOF_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONF0")
            .field("tx_rst", &self.tx_rst())
            .field("rx_rst", &self.rx_rst())
            .field("uart_sel", &self.uart_sel())
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
    #[doc = "Bit 0 - Write 1 and then write 0 to reset the decoder state machine."]
    #[inline(always)]
    pub fn tx_rst(&mut self) -> TX_RST_W<CONF0_SPEC> {
        TX_RST_W::new(self, 0)
    }
    #[doc = "Bit 1 - Write 1 and then write 0 to reset the encoder state machine."]
    #[inline(always)]
    pub fn rx_rst(&mut self) -> RX_RST_W<CONF0_SPEC> {
        RX_RST_W::new(self, 1)
    }
    #[doc = "Bits 2:4 - Configures to select which uart to connect with UHCI.\\\\ 0: UART0\\\\ 1: UART1\\\\"]
    #[inline(always)]
    pub fn uart_sel(&mut self) -> UART_SEL_W<CONF0_SPEC> {
        UART_SEL_W::new(self, 2)
    }
    #[doc = "Bit 5 - Configures whether or not to separate the data frame with a special character.\\\\ 0: Not separate\\\\ 1: Separate\\\\"]
    #[inline(always)]
    pub fn seper_en(&mut self) -> SEPER_EN_W<CONF0_SPEC> {
        SEPER_EN_W::new(self, 5)
    }
    #[doc = "Bit 6 - Configures whether or not to encode the data packet with a formatting header.\\\\ 0: Not use formatting header\\\\ 1: Use formatting header\\\\"]
    #[inline(always)]
    pub fn head_en(&mut self) -> HEAD_EN_W<CONF0_SPEC> {
        HEAD_EN_W::new(self, 6)
    }
    #[doc = "Bit 7 - Configures whether or not to enable the reception of the 16-bit CRC.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn crc_rec_en(&mut self) -> CRC_REC_EN_W<CONF0_SPEC> {
        CRC_REC_EN_W::new(self, 7)
    }
    #[doc = "Bit 8 - Configures whether or not to stop receiving data when UART is idle.\\\\ 0: Not stop\\\\ 1: Stop\\\\"]
    #[inline(always)]
    pub fn uart_idle_eof_en(&mut self) -> UART_IDLE_EOF_EN_W<CONF0_SPEC> {
        UART_IDLE_EOF_EN_W::new(self, 8)
    }
    #[doc = "Bit 9 - Configures when the UHCI decoder stops receiving data.\\\\ 0: Stops after receiving 0xC0\\\\ 1: Stops when the number of received data bytes reach the specified value. When UHCI_HEAD_EN is 1, the specified value is the data length indicated by the UHCI packet header. when UHCI_HEAD_EN is 0, the specified value is the configured value.\\\\"]
    #[inline(always)]
    pub fn len_eof_en(&mut self) -> LEN_EOF_EN_W<CONF0_SPEC> {
        LEN_EOF_EN_W::new(self, 9)
    }
    #[doc = "Bit 10 - Configures whether or not to enable data integrity check by appending a 16 bit CCITT-CRC to the end of the data.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn encode_crc_en(&mut self) -> ENCODE_CRC_EN_W<CONF0_SPEC> {
        ENCODE_CRC_EN_W::new(self, 10)
    }
    #[doc = "Bit 11 - Configures clock gating.\\\\ 0: Support clock only when the application writes registers.\\\\ 1: Always force the clock on for registers.\\\\"]
    #[inline(always)]
    pub fn clk_en(&mut self) -> CLK_EN_W<CONF0_SPEC> {
        CLK_EN_W::new(self, 11)
    }
    #[doc = "Bit 12 - Configures whether or not to stop UHCI from receiving data after UART has received a NULL frame.\\\\ 0: Not stop\\\\ 1: Stop\\\\"]
    #[inline(always)]
    pub fn uart_rx_brk_eof_en(&mut self) -> UART_RX_BRK_EOF_EN_W<CONF0_SPEC> {
        UART_RX_BRK_EOF_EN_W::new(self, 12)
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
}
#[doc = "`reset()` method sets CONF0 to value 0x06fc"]
impl crate::Resettable for CONF0_SPEC {
    const RESET_VALUE: u32 = 0x06fc;
}
