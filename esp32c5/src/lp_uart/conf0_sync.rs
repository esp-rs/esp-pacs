#[doc = "Register `CONF0_SYNC` reader"]
pub type R = crate::R<CONF0_SYNC_SPEC>;
#[doc = "Register `CONF0_SYNC` writer"]
pub type W = crate::W<CONF0_SYNC_SPEC>;
#[doc = "Field `PARITY` reader - Configures the parity check mode.\\\\ 0: Even parity\\\\ 1: Odd parity\\\\"]
pub type PARITY_R = crate::BitReader;
#[doc = "Field `PARITY` writer - Configures the parity check mode.\\\\ 0: Even parity\\\\ 1: Odd parity\\\\"]
pub type PARITY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PARITY_EN` reader - Configures whether or not to enable LP UART parity check.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
pub type PARITY_EN_R = crate::BitReader;
#[doc = "Field `PARITY_EN` writer - Configures whether or not to enable LP UART parity check.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
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
#[doc = "Field `LOOPBACK` reader - Configures whether or not to enable LP UART loopback test.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
pub type LOOPBACK_R = crate::BitReader;
#[doc = "Field `LOOPBACK` writer - Configures whether or not to enable LP UART loopback test.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
pub type LOOPBACK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_FLOW_EN` reader - Configures whether or not to enable flow control for the transmitter.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
pub type TX_FLOW_EN_R = crate::BitReader;
#[doc = "Field `TX_FLOW_EN` writer - Configures whether or not to enable flow control for the transmitter.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
pub type TX_FLOW_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXD_INV` reader - Configures whether or not to invert the level of LP UART RXD signal.\\\\ 0: Not invert\\\\ 1: Invert\\\\"]
pub type RXD_INV_R = crate::BitReader;
#[doc = "Field `RXD_INV` writer - Configures whether or not to invert the level of LP UART RXD signal.\\\\ 0: Not invert\\\\ 1: Invert\\\\"]
pub type RXD_INV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXD_INV` reader - Configures whether or not to invert the level of LP UART TXD signal.\\\\ 0: Not invert\\\\ 1: Invert\\\\"]
pub type TXD_INV_R = crate::BitReader;
#[doc = "Field `TXD_INV` writer - Configures whether or not to invert the level of LP UART TXD signal.\\\\ 0: Not invert\\\\ 1: Invert\\\\"]
pub type TXD_INV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIS_RX_DAT_OVF` reader - Configures whether or not to disable data overflow detection for the LP UART receiver.\\\\ 0: Enable\\\\ 1: Disable\\\\"]
pub type DIS_RX_DAT_OVF_R = crate::BitReader;
#[doc = "Field `DIS_RX_DAT_OVF` writer - Configures whether or not to disable data overflow detection for the LP UART receiver.\\\\ 0: Enable\\\\ 1: Disable\\\\"]
pub type DIS_RX_DAT_OVF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERR_WR_MASK` reader - Configures whether or not to store the received data with errors into FIFO.\\\\ 0: Store\\\\ 1: Not store\\\\"]
pub type ERR_WR_MASK_R = crate::BitReader;
#[doc = "Field `ERR_WR_MASK` writer - Configures whether or not to store the received data with errors into FIFO.\\\\ 0: Store\\\\ 1: Not store\\\\"]
pub type ERR_WR_MASK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_CLK_EN` reader - Configures whether or not to enable clock gating for LP UART memory.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
pub type MEM_CLK_EN_R = crate::BitReader;
#[doc = "Field `MEM_CLK_EN` writer - Configures whether or not to enable clock gating for LP UART memory.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
pub type MEM_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_RTS` reader - Configures the RTS signal used in software flow control.\\\\ 0: The LP UART transmitter is allowed to send data.\\\\ 1: The LP UART transmitted is not allowed to send data.\\\\"]
pub type SW_RTS_R = crate::BitReader;
#[doc = "Field `SW_RTS` writer - Configures the RTS signal used in software flow control.\\\\ 0: The LP UART transmitter is allowed to send data.\\\\ 1: The LP UART transmitted is not allowed to send data.\\\\"]
pub type SW_RTS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFIFO_RST` reader - Configures whether or not to reset the LP UART RX FIFO.\\\\ 0: Not reset\\\\ 1: Reset\\\\"]
pub type RXFIFO_RST_R = crate::BitReader;
#[doc = "Field `RXFIFO_RST` writer - Configures whether or not to reset the LP UART RX FIFO.\\\\ 0: Not reset\\\\ 1: Reset\\\\"]
pub type RXFIFO_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFIFO_RST` reader - Configures whether or not to reset the LP UART TX FIFO.\\\\ 0: Not reset\\\\ 1: Reset\\\\"]
pub type TXFIFO_RST_R = crate::BitReader;
#[doc = "Field `TXFIFO_RST` writer - Configures whether or not to reset the LP UART TX FIFO.\\\\ 0: Not reset\\\\ 1: Reset\\\\"]
pub type TXFIFO_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Configures the parity check mode.\\\\ 0: Even parity\\\\ 1: Odd parity\\\\"]
    #[inline(always)]
    pub fn parity(&self) -> PARITY_R {
        PARITY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Configures whether or not to enable LP UART parity check.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
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
    #[doc = "Bit 12 - Configures whether or not to enable LP UART loopback test.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn loopback(&self) -> LOOPBACK_R {
        LOOPBACK_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Configures whether or not to enable flow control for the transmitter.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn tx_flow_en(&self) -> TX_FLOW_EN_R {
        TX_FLOW_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - Configures whether or not to invert the level of LP UART RXD signal.\\\\ 0: Not invert\\\\ 1: Invert\\\\"]
    #[inline(always)]
    pub fn rxd_inv(&self) -> RXD_INV_R {
        RXD_INV_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Configures whether or not to invert the level of LP UART TXD signal.\\\\ 0: Not invert\\\\ 1: Invert\\\\"]
    #[inline(always)]
    pub fn txd_inv(&self) -> TXD_INV_R {
        TXD_INV_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Configures whether or not to disable data overflow detection for the LP UART receiver.\\\\ 0: Enable\\\\ 1: Disable\\\\"]
    #[inline(always)]
    pub fn dis_rx_dat_ovf(&self) -> DIS_RX_DAT_OVF_R {
        DIS_RX_DAT_OVF_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Configures whether or not to store the received data with errors into FIFO.\\\\ 0: Store\\\\ 1: Not store\\\\"]
    #[inline(always)]
    pub fn err_wr_mask(&self) -> ERR_WR_MASK_R {
        ERR_WR_MASK_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - Configures whether or not to enable clock gating for LP UART memory.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn mem_clk_en(&self) -> MEM_CLK_EN_R {
        MEM_CLK_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Configures the RTS signal used in software flow control.\\\\ 0: The LP UART transmitter is allowed to send data.\\\\ 1: The LP UART transmitted is not allowed to send data.\\\\"]
    #[inline(always)]
    pub fn sw_rts(&self) -> SW_RTS_R {
        SW_RTS_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Configures whether or not to reset the LP UART RX FIFO.\\\\ 0: Not reset\\\\ 1: Reset\\\\"]
    #[inline(always)]
    pub fn rxfifo_rst(&self) -> RXFIFO_RST_R {
        RXFIFO_RST_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Configures whether or not to reset the LP UART TX FIFO.\\\\ 0: Not reset\\\\ 1: Reset\\\\"]
    #[inline(always)]
    pub fn txfifo_rst(&self) -> TXFIFO_RST_R {
        TXFIFO_RST_R::new(((self.bits >> 23) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONF0_SYNC")
            .field("parity", &self.parity())
            .field("parity_en", &self.parity_en())
            .field("bit_num", &self.bit_num())
            .field("stop_bit_num", &self.stop_bit_num())
            .field("txd_brk", &self.txd_brk())
            .field("loopback", &self.loopback())
            .field("tx_flow_en", &self.tx_flow_en())
            .field("rxd_inv", &self.rxd_inv())
            .field("txd_inv", &self.txd_inv())
            .field("dis_rx_dat_ovf", &self.dis_rx_dat_ovf())
            .field("err_wr_mask", &self.err_wr_mask())
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
    pub fn parity(&mut self) -> PARITY_W<CONF0_SYNC_SPEC> {
        PARITY_W::new(self, 0)
    }
    #[doc = "Bit 1 - Configures whether or not to enable LP UART parity check.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn parity_en(&mut self) -> PARITY_EN_W<CONF0_SYNC_SPEC> {
        PARITY_EN_W::new(self, 1)
    }
    #[doc = "Bits 2:3 - Configures the number of data bits.\\\\ 0: 5 bits\\\\ 1: 6 bits\\\\ 2: 7 bits\\\\ 3: 8 bits\\\\"]
    #[inline(always)]
    pub fn bit_num(&mut self) -> BIT_NUM_W<CONF0_SYNC_SPEC> {
        BIT_NUM_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Configures the number of stop bits.\\\\ 0: Invalid. No effect\\\\ 1: 1 bits\\\\ 2: 1.5 bits\\\\ 3: 2 bits\\\\"]
    #[inline(always)]
    pub fn stop_bit_num(&mut self) -> STOP_BIT_NUM_W<CONF0_SYNC_SPEC> {
        STOP_BIT_NUM_W::new(self, 4)
    }
    #[doc = "Bit 6 - Configures whether or not to send NULL characters when finishing data transmission.\\\\ 0: Not send\\\\ 1: Send\\\\"]
    #[inline(always)]
    pub fn txd_brk(&mut self) -> TXD_BRK_W<CONF0_SYNC_SPEC> {
        TXD_BRK_W::new(self, 6)
    }
    #[doc = "Bit 12 - Configures whether or not to enable LP UART loopback test.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn loopback(&mut self) -> LOOPBACK_W<CONF0_SYNC_SPEC> {
        LOOPBACK_W::new(self, 12)
    }
    #[doc = "Bit 13 - Configures whether or not to enable flow control for the transmitter.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn tx_flow_en(&mut self) -> TX_FLOW_EN_W<CONF0_SYNC_SPEC> {
        TX_FLOW_EN_W::new(self, 13)
    }
    #[doc = "Bit 15 - Configures whether or not to invert the level of LP UART RXD signal.\\\\ 0: Not invert\\\\ 1: Invert\\\\"]
    #[inline(always)]
    pub fn rxd_inv(&mut self) -> RXD_INV_W<CONF0_SYNC_SPEC> {
        RXD_INV_W::new(self, 15)
    }
    #[doc = "Bit 16 - Configures whether or not to invert the level of LP UART TXD signal.\\\\ 0: Not invert\\\\ 1: Invert\\\\"]
    #[inline(always)]
    pub fn txd_inv(&mut self) -> TXD_INV_W<CONF0_SYNC_SPEC> {
        TXD_INV_W::new(self, 16)
    }
    #[doc = "Bit 17 - Configures whether or not to disable data overflow detection for the LP UART receiver.\\\\ 0: Enable\\\\ 1: Disable\\\\"]
    #[inline(always)]
    pub fn dis_rx_dat_ovf(&mut self) -> DIS_RX_DAT_OVF_W<CONF0_SYNC_SPEC> {
        DIS_RX_DAT_OVF_W::new(self, 17)
    }
    #[doc = "Bit 18 - Configures whether or not to store the received data with errors into FIFO.\\\\ 0: Store\\\\ 1: Not store\\\\"]
    #[inline(always)]
    pub fn err_wr_mask(&mut self) -> ERR_WR_MASK_W<CONF0_SYNC_SPEC> {
        ERR_WR_MASK_W::new(self, 18)
    }
    #[doc = "Bit 20 - Configures whether or not to enable clock gating for LP UART memory.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn mem_clk_en(&mut self) -> MEM_CLK_EN_W<CONF0_SYNC_SPEC> {
        MEM_CLK_EN_W::new(self, 20)
    }
    #[doc = "Bit 21 - Configures the RTS signal used in software flow control.\\\\ 0: The LP UART transmitter is allowed to send data.\\\\ 1: The LP UART transmitted is not allowed to send data.\\\\"]
    #[inline(always)]
    pub fn sw_rts(&mut self) -> SW_RTS_W<CONF0_SYNC_SPEC> {
        SW_RTS_W::new(self, 21)
    }
    #[doc = "Bit 22 - Configures whether or not to reset the LP UART RX FIFO.\\\\ 0: Not reset\\\\ 1: Reset\\\\"]
    #[inline(always)]
    pub fn rxfifo_rst(&mut self) -> RXFIFO_RST_W<CONF0_SYNC_SPEC> {
        RXFIFO_RST_W::new(self, 22)
    }
    #[doc = "Bit 23 - Configures whether or not to reset the LP UART TX FIFO.\\\\ 0: Not reset\\\\ 1: Reset\\\\"]
    #[inline(always)]
    pub fn txfifo_rst(&mut self) -> TXFIFO_RST_W<CONF0_SYNC_SPEC> {
        TXFIFO_RST_W::new(self, 23)
    }
}
#[doc = "Configuration register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`conf0_sync::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`conf0_sync::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONF0_SYNC_SPEC;
impl crate::RegisterSpec for CONF0_SYNC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`conf0_sync::R`](R) reader structure"]
impl crate::Readable for CONF0_SYNC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`conf0_sync::W`](W) writer structure"]
impl crate::Writable for CONF0_SYNC_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONF0_SYNC to value 0x1c"]
impl crate::Resettable for CONF0_SYNC_SPEC {
    const RESET_VALUE: u32 = 0x1c;
}
