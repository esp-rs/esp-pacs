#[doc = "Register `CONF0` reader"]
pub type R = crate::R<CONF0_SPEC>;
#[doc = "Register `CONF0` writer"]
pub type W = crate::W<CONF0_SPEC>;
#[doc = "Field `PARITY` reader - Configures the parity check mode.\\\\ 0: Even parity\\\\ 1: Odd parity\\\\"]
pub type PARITY_R = crate::BitReader;
#[doc = "Field `PARITY` writer - Configures the parity check mode.\\\\ 0: Even parity\\\\ 1: Odd parity\\\\"]
pub type PARITY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PARITY_EN` reader - Configures whether or not to enable UART parity check.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
pub type PARITY_EN_R = crate::BitReader;
#[doc = "Field `PARITY_EN` writer - Configures whether or not to enable UART parity check.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
pub type PARITY_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_NUM` reader - Configures the number of data bits.\\\\ 0: 5 bits\\\\ 1: 6 bits\\\\ 2: 7 bits\\\\ 3: 8 bits\\\\"]
pub type BIT_NUM_R = crate::FieldReader;
#[doc = "Field `BIT_NUM` writer - Configures the number of data bits.\\\\ 0: 5 bits\\\\ 1: 6 bits\\\\ 2: 7 bits\\\\ 3: 8 bits\\\\"]
pub type BIT_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `STOP_BIT_NUM` reader - Configures the number of stop bits.\\\\ 0: Invalid. No effect\\\\ 1: 1 bits\\\\ 2: 1.5 bits\\\\ 3: 2 bits\\\\"]
pub type STOP_BIT_NUM_R = crate::FieldReader;
#[doc = "Field `STOP_BIT_NUM` writer - Configures the number of stop bits.\\\\ 0: Invalid. No effect\\\\ 1: 1 bits\\\\ 2: 1.5 bits\\\\ 3: 2 bits\\\\"]
pub type STOP_BIT_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TXD_BRK` reader - Configures whether or not to send NULL characters when finishing data transmission.\\\\ 0: Not send\\\\ 1: Send\\\\"]
pub type TXD_BRK_R = crate::BitReader;
#[doc = "Field `TXD_BRK` writer - Configures whether or not to send NULL characters when finishing data transmission.\\\\ 0: Not send\\\\ 1: Send\\\\"]
pub type TXD_BRK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRDA_DPLX` reader - Configures whether or not to enable IrDA loopback test.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
pub type IRDA_DPLX_R = crate::BitReader;
#[doc = "Field `IRDA_DPLX` writer - Configures whether or not to enable IrDA loopback test.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
pub type IRDA_DPLX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRDA_TX_EN` reader - Configures whether or not to enable the IrDA transmitter.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
pub type IRDA_TX_EN_R = crate::BitReader;
#[doc = "Field `IRDA_TX_EN` writer - Configures whether or not to enable the IrDA transmitter.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
pub type IRDA_TX_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRDA_WCTL` reader - Configures the 11th bit of the IrDA transmitter.\\\\ 0: This bit is 0.\\\\ 1: This bit is the same as the 10th bit.\\\\"]
pub type IRDA_WCTL_R = crate::BitReader;
#[doc = "Field `IRDA_WCTL` writer - Configures the 11th bit of the IrDA transmitter.\\\\ 0: This bit is 0.\\\\ 1: This bit is the same as the 10th bit.\\\\"]
pub type IRDA_WCTL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRDA_TX_INV` reader - Configures whether or not to invert the level of the IrDA transmitter.\\\\ 0: Not invert\\\\ 1: Invert\\\\"]
pub type IRDA_TX_INV_R = crate::BitReader;
#[doc = "Field `IRDA_TX_INV` writer - Configures whether or not to invert the level of the IrDA transmitter.\\\\ 0: Not invert\\\\ 1: Invert\\\\"]
pub type IRDA_TX_INV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRDA_RX_INV` reader - Configures whether or not to invert the level of the IrDA receiver.\\\\ 0: Not invert\\\\ 1: Invert\\\\"]
pub type IRDA_RX_INV_R = crate::BitReader;
#[doc = "Field `IRDA_RX_INV` writer - Configures whether or not to invert the level of the IrDA receiver.\\\\ 0: Not invert\\\\ 1: Invert\\\\"]
pub type IRDA_RX_INV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOOPBACK` reader - Configures whether or not to enable UART loopback test.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
pub type LOOPBACK_R = crate::BitReader;
#[doc = "Field `LOOPBACK` writer - Configures whether or not to enable UART loopback test.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
pub type LOOPBACK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_FLOW_EN` reader - Configures whether or not to enable flow control for the transmitter.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
pub type TX_FLOW_EN_R = crate::BitReader;
#[doc = "Field `TX_FLOW_EN` writer - Configures whether or not to enable flow control for the transmitter.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
pub type TX_FLOW_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRDA_EN` reader - Configures whether or not to enable IrDA protocol.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
pub type IRDA_EN_R = crate::BitReader;
#[doc = "Field `IRDA_EN` writer - Configures whether or not to enable IrDA protocol.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
pub type IRDA_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXD_INV` reader - Configures whether or not to invert the level of UART RXD signal.\\\\ 0: Not invert\\\\ 1: Invert\\\\"]
pub type RXD_INV_R = crate::BitReader;
#[doc = "Field `RXD_INV` writer - Configures whether or not to invert the level of UART RXD signal.\\\\ 0: Not invert\\\\ 1: Invert\\\\"]
pub type RXD_INV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXD_INV` reader - Configures whether or not to invert the level of UART TXD signal.\\\\ 0: Not invert\\\\ 1: Invert\\\\"]
pub type TXD_INV_R = crate::BitReader;
#[doc = "Field `TXD_INV` writer - Configures whether or not to invert the level of UART TXD signal.\\\\ 0: Not invert\\\\ 1: Invert\\\\"]
pub type TXD_INV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIS_RX_DAT_OVF` reader - Configures whether or not to disable data overflow detection for the UART receiver.\\\\ 0: Enable\\\\ 1: Disable\\\\"]
pub type DIS_RX_DAT_OVF_R = crate::BitReader;
#[doc = "Field `DIS_RX_DAT_OVF` writer - Configures whether or not to disable data overflow detection for the UART receiver.\\\\ 0: Enable\\\\ 1: Disable\\\\"]
pub type DIS_RX_DAT_OVF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERR_WR_MASK` reader - Configures whether or not to store the received data with errors into FIFO.\\\\ 0: Store\\\\ 1: Not store\\\\"]
pub type ERR_WR_MASK_R = crate::BitReader;
#[doc = "Field `ERR_WR_MASK` writer - Configures whether or not to store the received data with errors into FIFO.\\\\ 0: Store\\\\ 1: Not store\\\\"]
pub type ERR_WR_MASK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTOBAUD_EN` reader - Configures whether or not to enable baud rate detection.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
pub type AUTOBAUD_EN_R = crate::BitReader;
#[doc = "Field `AUTOBAUD_EN` writer - Configures whether or not to enable baud rate detection.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
pub type AUTOBAUD_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_CLK_EN` reader - Configures whether or not to enable clock gating for UART memory.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
pub type MEM_CLK_EN_R = crate::BitReader;
#[doc = "Field `MEM_CLK_EN` writer - Configures whether or not to enable clock gating for UART memory.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
pub type MEM_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_RTS` reader - Configures the RTS signal used in software flow control.\\\\ 0: The UART transmitter is allowed to send data.\\\\ 1: The UART transmitted is not allowed to send data.\\\\"]
pub type SW_RTS_R = crate::BitReader;
#[doc = "Field `SW_RTS` writer - Configures the RTS signal used in software flow control.\\\\ 0: The UART transmitter is allowed to send data.\\\\ 1: The UART transmitted is not allowed to send data.\\\\"]
pub type SW_RTS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFIFO_RST` reader - Configures whether or not to reset the UART RX FIFO.\\\\ 0: Not reset\\\\ 1: Reset\\\\"]
pub type RXFIFO_RST_R = crate::BitReader;
#[doc = "Field `RXFIFO_RST` writer - Configures whether or not to reset the UART RX FIFO.\\\\ 0: Not reset\\\\ 1: Reset\\\\"]
pub type RXFIFO_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFIFO_RST` reader - Configures whether or not to reset the UART TX FIFO.\\\\ 0: Not reset\\\\ 1: Reset\\\\"]
pub type TXFIFO_RST_R = crate::BitReader;
#[doc = "Field `TXFIFO_RST` writer - Configures whether or not to reset the UART TX FIFO.\\\\ 0: Not reset\\\\ 1: Reset\\\\"]
pub type TXFIFO_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Configures the parity check mode.\\\\ 0: Even parity\\\\ 1: Odd parity\\\\"]
    #[inline(always)]
    pub fn parity(&self) -> PARITY_R {
        PARITY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Configures whether or not to enable UART parity check.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn parity_en(&self) -> PARITY_EN_R {
        PARITY_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Configures the number of data bits.\\\\ 0: 5 bits\\\\ 1: 6 bits\\\\ 2: 7 bits\\\\ 3: 8 bits\\\\"]
    #[inline(always)]
    pub fn bit_num(&self) -> BIT_NUM_R {
        BIT_NUM_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Configures the number of stop bits.\\\\ 0: Invalid. No effect\\\\ 1: 1 bits\\\\ 2: 1.5 bits\\\\ 3: 2 bits\\\\"]
    #[inline(always)]
    pub fn stop_bit_num(&self) -> STOP_BIT_NUM_R {
        STOP_BIT_NUM_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Configures whether or not to send NULL characters when finishing data transmission.\\\\ 0: Not send\\\\ 1: Send\\\\"]
    #[inline(always)]
    pub fn txd_brk(&self) -> TXD_BRK_R {
        TXD_BRK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Configures whether or not to enable IrDA loopback test.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn irda_dplx(&self) -> IRDA_DPLX_R {
        IRDA_DPLX_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Configures whether or not to enable the IrDA transmitter.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn irda_tx_en(&self) -> IRDA_TX_EN_R {
        IRDA_TX_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Configures the 11th bit of the IrDA transmitter.\\\\ 0: This bit is 0.\\\\ 1: This bit is the same as the 10th bit.\\\\"]
    #[inline(always)]
    pub fn irda_wctl(&self) -> IRDA_WCTL_R {
        IRDA_WCTL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Configures whether or not to invert the level of the IrDA transmitter.\\\\ 0: Not invert\\\\ 1: Invert\\\\"]
    #[inline(always)]
    pub fn irda_tx_inv(&self) -> IRDA_TX_INV_R {
        IRDA_TX_INV_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Configures whether or not to invert the level of the IrDA receiver.\\\\ 0: Not invert\\\\ 1: Invert\\\\"]
    #[inline(always)]
    pub fn irda_rx_inv(&self) -> IRDA_RX_INV_R {
        IRDA_RX_INV_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Configures whether or not to enable UART loopback test.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn loopback(&self) -> LOOPBACK_R {
        LOOPBACK_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Configures whether or not to enable flow control for the transmitter.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn tx_flow_en(&self) -> TX_FLOW_EN_R {
        TX_FLOW_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Configures whether or not to enable IrDA protocol.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn irda_en(&self) -> IRDA_EN_R {
        IRDA_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Configures whether or not to invert the level of UART RXD signal.\\\\ 0: Not invert\\\\ 1: Invert\\\\"]
    #[inline(always)]
    pub fn rxd_inv(&self) -> RXD_INV_R {
        RXD_INV_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Configures whether or not to invert the level of UART TXD signal.\\\\ 0: Not invert\\\\ 1: Invert\\\\"]
    #[inline(always)]
    pub fn txd_inv(&self) -> TXD_INV_R {
        TXD_INV_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Configures whether or not to disable data overflow detection for the UART receiver.\\\\ 0: Enable\\\\ 1: Disable\\\\"]
    #[inline(always)]
    pub fn dis_rx_dat_ovf(&self) -> DIS_RX_DAT_OVF_R {
        DIS_RX_DAT_OVF_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Configures whether or not to store the received data with errors into FIFO.\\\\ 0: Store\\\\ 1: Not store\\\\"]
    #[inline(always)]
    pub fn err_wr_mask(&self) -> ERR_WR_MASK_R {
        ERR_WR_MASK_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Configures whether or not to enable baud rate detection.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn autobaud_en(&self) -> AUTOBAUD_EN_R {
        AUTOBAUD_EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Configures whether or not to enable clock gating for UART memory.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn mem_clk_en(&self) -> MEM_CLK_EN_R {
        MEM_CLK_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Configures the RTS signal used in software flow control.\\\\ 0: The UART transmitter is allowed to send data.\\\\ 1: The UART transmitted is not allowed to send data.\\\\"]
    #[inline(always)]
    pub fn sw_rts(&self) -> SW_RTS_R {
        SW_RTS_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Configures whether or not to reset the UART RX FIFO.\\\\ 0: Not reset\\\\ 1: Reset\\\\"]
    #[inline(always)]
    pub fn rxfifo_rst(&self) -> RXFIFO_RST_R {
        RXFIFO_RST_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Configures whether or not to reset the UART TX FIFO.\\\\ 0: Not reset\\\\ 1: Reset\\\\"]
    #[inline(always)]
    pub fn txfifo_rst(&self) -> TXFIFO_RST_R {
        TXFIFO_RST_R::new(((self.bits >> 23) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONF0")
            .field("parity", &self.parity())
            .field("parity_en", &self.parity_en())
            .field("bit_num", &self.bit_num())
            .field("stop_bit_num", &self.stop_bit_num())
            .field("txd_brk", &self.txd_brk())
            .field("irda_dplx", &self.irda_dplx())
            .field("irda_tx_en", &self.irda_tx_en())
            .field("irda_wctl", &self.irda_wctl())
            .field("irda_tx_inv", &self.irda_tx_inv())
            .field("irda_rx_inv", &self.irda_rx_inv())
            .field("loopback", &self.loopback())
            .field("tx_flow_en", &self.tx_flow_en())
            .field("irda_en", &self.irda_en())
            .field("rxd_inv", &self.rxd_inv())
            .field("txd_inv", &self.txd_inv())
            .field("dis_rx_dat_ovf", &self.dis_rx_dat_ovf())
            .field("err_wr_mask", &self.err_wr_mask())
            .field("autobaud_en", &self.autobaud_en())
            .field("mem_clk_en", &self.mem_clk_en())
            .field("sw_rts", &self.sw_rts())
            .field("rxfifo_rst", &self.rxfifo_rst())
            .field("txfifo_rst", &self.txfifo_rst())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Configures the parity check mode.\\\\ 0: Even parity\\\\ 1: Odd parity\\\\"]
    #[inline(always)]
    pub fn parity(&mut self) -> PARITY_W<CONF0_SPEC> {
        PARITY_W::new(self, 0)
    }
    #[doc = "Bit 1 - Configures whether or not to enable UART parity check.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn parity_en(&mut self) -> PARITY_EN_W<CONF0_SPEC> {
        PARITY_EN_W::new(self, 1)
    }
    #[doc = "Bits 2:3 - Configures the number of data bits.\\\\ 0: 5 bits\\\\ 1: 6 bits\\\\ 2: 7 bits\\\\ 3: 8 bits\\\\"]
    #[inline(always)]
    pub fn bit_num(&mut self) -> BIT_NUM_W<CONF0_SPEC> {
        BIT_NUM_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Configures the number of stop bits.\\\\ 0: Invalid. No effect\\\\ 1: 1 bits\\\\ 2: 1.5 bits\\\\ 3: 2 bits\\\\"]
    #[inline(always)]
    pub fn stop_bit_num(&mut self) -> STOP_BIT_NUM_W<CONF0_SPEC> {
        STOP_BIT_NUM_W::new(self, 4)
    }
    #[doc = "Bit 6 - Configures whether or not to send NULL characters when finishing data transmission.\\\\ 0: Not send\\\\ 1: Send\\\\"]
    #[inline(always)]
    pub fn txd_brk(&mut self) -> TXD_BRK_W<CONF0_SPEC> {
        TXD_BRK_W::new(self, 6)
    }
    #[doc = "Bit 7 - Configures whether or not to enable IrDA loopback test.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn irda_dplx(&mut self) -> IRDA_DPLX_W<CONF0_SPEC> {
        IRDA_DPLX_W::new(self, 7)
    }
    #[doc = "Bit 8 - Configures whether or not to enable the IrDA transmitter.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn irda_tx_en(&mut self) -> IRDA_TX_EN_W<CONF0_SPEC> {
        IRDA_TX_EN_W::new(self, 8)
    }
    #[doc = "Bit 9 - Configures the 11th bit of the IrDA transmitter.\\\\ 0: This bit is 0.\\\\ 1: This bit is the same as the 10th bit.\\\\"]
    #[inline(always)]
    pub fn irda_wctl(&mut self) -> IRDA_WCTL_W<CONF0_SPEC> {
        IRDA_WCTL_W::new(self, 9)
    }
    #[doc = "Bit 10 - Configures whether or not to invert the level of the IrDA transmitter.\\\\ 0: Not invert\\\\ 1: Invert\\\\"]
    #[inline(always)]
    pub fn irda_tx_inv(&mut self) -> IRDA_TX_INV_W<CONF0_SPEC> {
        IRDA_TX_INV_W::new(self, 10)
    }
    #[doc = "Bit 11 - Configures whether or not to invert the level of the IrDA receiver.\\\\ 0: Not invert\\\\ 1: Invert\\\\"]
    #[inline(always)]
    pub fn irda_rx_inv(&mut self) -> IRDA_RX_INV_W<CONF0_SPEC> {
        IRDA_RX_INV_W::new(self, 11)
    }
    #[doc = "Bit 12 - Configures whether or not to enable UART loopback test.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn loopback(&mut self) -> LOOPBACK_W<CONF0_SPEC> {
        LOOPBACK_W::new(self, 12)
    }
    #[doc = "Bit 13 - Configures whether or not to enable flow control for the transmitter.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn tx_flow_en(&mut self) -> TX_FLOW_EN_W<CONF0_SPEC> {
        TX_FLOW_EN_W::new(self, 13)
    }
    #[doc = "Bit 14 - Configures whether or not to enable IrDA protocol.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn irda_en(&mut self) -> IRDA_EN_W<CONF0_SPEC> {
        IRDA_EN_W::new(self, 14)
    }
    #[doc = "Bit 15 - Configures whether or not to invert the level of UART RXD signal.\\\\ 0: Not invert\\\\ 1: Invert\\\\"]
    #[inline(always)]
    pub fn rxd_inv(&mut self) -> RXD_INV_W<CONF0_SPEC> {
        RXD_INV_W::new(self, 15)
    }
    #[doc = "Bit 16 - Configures whether or not to invert the level of UART TXD signal.\\\\ 0: Not invert\\\\ 1: Invert\\\\"]
    #[inline(always)]
    pub fn txd_inv(&mut self) -> TXD_INV_W<CONF0_SPEC> {
        TXD_INV_W::new(self, 16)
    }
    #[doc = "Bit 17 - Configures whether or not to disable data overflow detection for the UART receiver.\\\\ 0: Enable\\\\ 1: Disable\\\\"]
    #[inline(always)]
    pub fn dis_rx_dat_ovf(&mut self) -> DIS_RX_DAT_OVF_W<CONF0_SPEC> {
        DIS_RX_DAT_OVF_W::new(self, 17)
    }
    #[doc = "Bit 18 - Configures whether or not to store the received data with errors into FIFO.\\\\ 0: Store\\\\ 1: Not store\\\\"]
    #[inline(always)]
    pub fn err_wr_mask(&mut self) -> ERR_WR_MASK_W<CONF0_SPEC> {
        ERR_WR_MASK_W::new(self, 18)
    }
    #[doc = "Bit 19 - Configures whether or not to enable baud rate detection.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn autobaud_en(&mut self) -> AUTOBAUD_EN_W<CONF0_SPEC> {
        AUTOBAUD_EN_W::new(self, 19)
    }
    #[doc = "Bit 20 - Configures whether or not to enable clock gating for UART memory.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn mem_clk_en(&mut self) -> MEM_CLK_EN_W<CONF0_SPEC> {
        MEM_CLK_EN_W::new(self, 20)
    }
    #[doc = "Bit 21 - Configures the RTS signal used in software flow control.\\\\ 0: The UART transmitter is allowed to send data.\\\\ 1: The UART transmitted is not allowed to send data.\\\\"]
    #[inline(always)]
    pub fn sw_rts(&mut self) -> SW_RTS_W<CONF0_SPEC> {
        SW_RTS_W::new(self, 21)
    }
    #[doc = "Bit 22 - Configures whether or not to reset the UART RX FIFO.\\\\ 0: Not reset\\\\ 1: Reset\\\\"]
    #[inline(always)]
    pub fn rxfifo_rst(&mut self) -> RXFIFO_RST_W<CONF0_SPEC> {
        RXFIFO_RST_W::new(self, 22)
    }
    #[doc = "Bit 23 - Configures whether or not to reset the UART TX FIFO.\\\\ 0: Not reset\\\\ 1: Reset\\\\"]
    #[inline(always)]
    pub fn txfifo_rst(&mut self) -> TXFIFO_RST_W<CONF0_SPEC> {
        TXFIFO_RST_W::new(self, 23)
    }
}
#[doc = "Configuration register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`conf0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`conf0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets CONF0 to value 0x1c"]
impl crate::Resettable for CONF0_SPEC {
    const RESET_VALUE: u32 = 0x1c;
}
