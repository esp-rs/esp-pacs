#[doc = "Register `INT_CLR` writer"]
pub type W = crate::W<INT_CLR_SPEC>;
#[doc = "Field `RXFIFO_FULL` writer - Write 1 to clear UART_RXFIFO_FULL_INT."]
pub type RXFIFO_FULL_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `TXFIFO_EMPTY` writer - Write 1 to clear UART_TXFIFO_EMPTY_INT."]
pub type TXFIFO_EMPTY_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `PARITY_ERR` writer - Write 1 to clear UART_PARITY_ERR_INT."]
pub type PARITY_ERR_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `FRM_ERR` writer - Write 1 to clear UART_FRM_ERR_INT."]
pub type FRM_ERR_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `RXFIFO_OVF` writer - Write 1 to clear UART_RXFIFO_OVF_INT."]
pub type RXFIFO_OVF_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `DSR_CHG` writer - Write 1 to clear UART_DSR_CHG_INT."]
pub type DSR_CHG_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CTS_CHG` writer - Write 1 to clear UART_CTS_CHG_INT."]
pub type CTS_CHG_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `BRK_DET` writer - Write 1 to clear UART_BRK_DET_INT."]
pub type BRK_DET_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `RXFIFO_TOUT` writer - Write 1 to clear UART_RXFIFO_TOUT_INT."]
pub type RXFIFO_TOUT_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `SW_XON` writer - Write 1 to clear UART_SW_XON_INT."]
pub type SW_XON_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `SW_XOFF` writer - Write 1 to clear UART_SW_XOFF_INT."]
pub type SW_XOFF_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `GLITCH_DET` writer - Write 1 to clear UART_GLITCH_DET_INT."]
pub type GLITCH_DET_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `TX_BRK_DONE` writer - Write 1 to clear UART_TX_BRK_DONE_INT."]
pub type TX_BRK_DONE_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `TX_BRK_IDLE_DONE` writer - Write 1 to clear UART_TX_BRK_IDLE_DONE_INT."]
pub type TX_BRK_IDLE_DONE_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `TX_DONE` writer - Write 1 to clear UART_TX_DONE_INT."]
pub type TX_DONE_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `RS485_PARITY_ERR` writer - Write 1 to clear UART_RS485_PARITY_ERR_INT."]
pub type RS485_PARITY_ERR_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `RS485_FRM_ERR` writer - Write 1 to clear UART_RS485_FRM_ERR_INT."]
pub type RS485_FRM_ERR_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `RS485_CLASH` writer - Write 1 to clear UART_RS485_CLASH_INT."]
pub type RS485_CLASH_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `AT_CMD_CHAR_DET` writer - Write 1 to clear UART_AT_CMD_CHAR_DET_INT."]
pub type AT_CMD_CHAR_DET_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `WAKEUP` writer - Write 1 to clear UART_WAKEUP_INT."]
pub type WAKEUP_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Write 1 to clear UART_RXFIFO_FULL_INT."]
    #[inline(always)]
    pub fn rxfifo_full(&mut self) -> RXFIFO_FULL_W<INT_CLR_SPEC> {
        RXFIFO_FULL_W::new(self, 0)
    }
    #[doc = "Bit 1 - Write 1 to clear UART_TXFIFO_EMPTY_INT."]
    #[inline(always)]
    pub fn txfifo_empty(&mut self) -> TXFIFO_EMPTY_W<INT_CLR_SPEC> {
        TXFIFO_EMPTY_W::new(self, 1)
    }
    #[doc = "Bit 2 - Write 1 to clear UART_PARITY_ERR_INT."]
    #[inline(always)]
    pub fn parity_err(&mut self) -> PARITY_ERR_W<INT_CLR_SPEC> {
        PARITY_ERR_W::new(self, 2)
    }
    #[doc = "Bit 3 - Write 1 to clear UART_FRM_ERR_INT."]
    #[inline(always)]
    pub fn frm_err(&mut self) -> FRM_ERR_W<INT_CLR_SPEC> {
        FRM_ERR_W::new(self, 3)
    }
    #[doc = "Bit 4 - Write 1 to clear UART_RXFIFO_OVF_INT."]
    #[inline(always)]
    pub fn rxfifo_ovf(&mut self) -> RXFIFO_OVF_W<INT_CLR_SPEC> {
        RXFIFO_OVF_W::new(self, 4)
    }
    #[doc = "Bit 5 - Write 1 to clear UART_DSR_CHG_INT."]
    #[inline(always)]
    pub fn dsr_chg(&mut self) -> DSR_CHG_W<INT_CLR_SPEC> {
        DSR_CHG_W::new(self, 5)
    }
    #[doc = "Bit 6 - Write 1 to clear UART_CTS_CHG_INT."]
    #[inline(always)]
    pub fn cts_chg(&mut self) -> CTS_CHG_W<INT_CLR_SPEC> {
        CTS_CHG_W::new(self, 6)
    }
    #[doc = "Bit 7 - Write 1 to clear UART_BRK_DET_INT."]
    #[inline(always)]
    pub fn brk_det(&mut self) -> BRK_DET_W<INT_CLR_SPEC> {
        BRK_DET_W::new(self, 7)
    }
    #[doc = "Bit 8 - Write 1 to clear UART_RXFIFO_TOUT_INT."]
    #[inline(always)]
    pub fn rxfifo_tout(&mut self) -> RXFIFO_TOUT_W<INT_CLR_SPEC> {
        RXFIFO_TOUT_W::new(self, 8)
    }
    #[doc = "Bit 9 - Write 1 to clear UART_SW_XON_INT."]
    #[inline(always)]
    pub fn sw_xon(&mut self) -> SW_XON_W<INT_CLR_SPEC> {
        SW_XON_W::new(self, 9)
    }
    #[doc = "Bit 10 - Write 1 to clear UART_SW_XOFF_INT."]
    #[inline(always)]
    pub fn sw_xoff(&mut self) -> SW_XOFF_W<INT_CLR_SPEC> {
        SW_XOFF_W::new(self, 10)
    }
    #[doc = "Bit 11 - Write 1 to clear UART_GLITCH_DET_INT."]
    #[inline(always)]
    pub fn glitch_det(&mut self) -> GLITCH_DET_W<INT_CLR_SPEC> {
        GLITCH_DET_W::new(self, 11)
    }
    #[doc = "Bit 12 - Write 1 to clear UART_TX_BRK_DONE_INT."]
    #[inline(always)]
    pub fn tx_brk_done(&mut self) -> TX_BRK_DONE_W<INT_CLR_SPEC> {
        TX_BRK_DONE_W::new(self, 12)
    }
    #[doc = "Bit 13 - Write 1 to clear UART_TX_BRK_IDLE_DONE_INT."]
    #[inline(always)]
    pub fn tx_brk_idle_done(&mut self) -> TX_BRK_IDLE_DONE_W<INT_CLR_SPEC> {
        TX_BRK_IDLE_DONE_W::new(self, 13)
    }
    #[doc = "Bit 14 - Write 1 to clear UART_TX_DONE_INT."]
    #[inline(always)]
    pub fn tx_done(&mut self) -> TX_DONE_W<INT_CLR_SPEC> {
        TX_DONE_W::new(self, 14)
    }
    #[doc = "Bit 15 - Write 1 to clear UART_RS485_PARITY_ERR_INT."]
    #[inline(always)]
    pub fn rs485_parity_err(&mut self) -> RS485_PARITY_ERR_W<INT_CLR_SPEC> {
        RS485_PARITY_ERR_W::new(self, 15)
    }
    #[doc = "Bit 16 - Write 1 to clear UART_RS485_FRM_ERR_INT."]
    #[inline(always)]
    pub fn rs485_frm_err(&mut self) -> RS485_FRM_ERR_W<INT_CLR_SPEC> {
        RS485_FRM_ERR_W::new(self, 16)
    }
    #[doc = "Bit 17 - Write 1 to clear UART_RS485_CLASH_INT."]
    #[inline(always)]
    pub fn rs485_clash(&mut self) -> RS485_CLASH_W<INT_CLR_SPEC> {
        RS485_CLASH_W::new(self, 17)
    }
    #[doc = "Bit 18 - Write 1 to clear UART_AT_CMD_CHAR_DET_INT."]
    #[inline(always)]
    pub fn at_cmd_char_det(&mut self) -> AT_CMD_CHAR_DET_W<INT_CLR_SPEC> {
        AT_CMD_CHAR_DET_W::new(self, 18)
    }
    #[doc = "Bit 19 - Write 1 to clear UART_WAKEUP_INT."]
    #[inline(always)]
    pub fn wakeup(&mut self) -> WAKEUP_W<INT_CLR_SPEC> {
        WAKEUP_W::new(self, 19)
    }
}
#[doc = "Interrupt clear bits\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_CLR_SPEC;
impl crate::RegisterSpec for INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_clr::W`](W) writer structure"]
impl crate::Writable for INT_CLR_SPEC {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x000f_ffff;
}
#[doc = "`reset()` method sets INT_CLR to value 0"]
impl crate::Resettable for INT_CLR_SPEC {}
