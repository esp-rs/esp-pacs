#[doc = "Register `INT_CLR` writer"]
pub type W = crate::W<INT_CLR_SPEC>;
#[doc = "Field `RXFIFO_FULL` writer - Set this bit to clear UART_THE RXFIFO_FULL_INT interrupt."]
pub type RXFIFO_FULL_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `TXFIFO_EMPTY` writer - Set this bit to clear UART_TXFIFO_EMPTY_INT interrupt."]
pub type TXFIFO_EMPTY_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `PARITY_ERR` writer - Set this bit to clear UART_PARITY_ERR_INT interrupt."]
pub type PARITY_ERR_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `FRM_ERR` writer - Set this bit to clear UART_FRM_ERR_INT interrupt."]
pub type FRM_ERR_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `RXFIFO_OVF` writer - Set this bit to clear UART_UART_RXFIFO_OVF_INT interrupt."]
pub type RXFIFO_OVF_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `DSR_CHG` writer - Set this bit to clear UART_DSR_CHG_INT interrupt."]
pub type DSR_CHG_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CTS_CHG` writer - Set this bit to clear UART_CTS_CHG_INT interrupt."]
pub type CTS_CHG_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `BRK_DET` writer - Set this bit to clear UART_BRK_DET_INT interrupt."]
pub type BRK_DET_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `RXFIFO_TOUT` writer - Set this bit to clear UART_RXFIFO_TOUT_INT interrupt."]
pub type RXFIFO_TOUT_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `SW_XON` writer - Set this bit to clear UART_SW_XON_INT interrupt."]
pub type SW_XON_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `SW_XOFF` writer - Set this bit to clear UART_SW_XOFF_INT interrupt."]
pub type SW_XOFF_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `GLITCH_DET` writer - Set this bit to clear UART_GLITCH_DET_INT interrupt."]
pub type GLITCH_DET_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `TX_BRK_DONE` writer - Set this bit to clear UART_TX_BRK_DONE_INT interrupt."]
pub type TX_BRK_DONE_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `TX_BRK_IDLE_DONE` writer - Set this bit to clear UART_TX_BRK_IDLE_DONE_INT interrupt."]
pub type TX_BRK_IDLE_DONE_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `TX_DONE` writer - Set this bit to clear UART_TX_DONE_INT interrupt."]
pub type TX_DONE_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `RS485_PARITY_ERR` writer - Set this bit to clear UART_RS485_PARITY_ERR_INT interrupt."]
pub type RS485_PARITY_ERR_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `RS485_FRM_ERR` writer - Set this bit to clear UART_RS485_FRM_ERR_INT interrupt."]
pub type RS485_FRM_ERR_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `RS485_CLASH` writer - Set this bit to clear UART_RS485_CLASH_INT interrupt."]
pub type RS485_CLASH_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `AT_CMD_CHAR_DET` writer - Set this bit to clear UART_AT_CMD_CHAR_DET_INT interrupt."]
pub type AT_CMD_CHAR_DET_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `WAKEUP` writer - Set this bit to clear UART_WAKEUP_INT interrupt."]
pub type WAKEUP_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to clear UART_THE RXFIFO_FULL_INT interrupt."]
    #[inline(always)]
    pub fn rxfifo_full(&mut self) -> RXFIFO_FULL_W<INT_CLR_SPEC> {
        RXFIFO_FULL_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set this bit to clear UART_TXFIFO_EMPTY_INT interrupt."]
    #[inline(always)]
    pub fn txfifo_empty(&mut self) -> TXFIFO_EMPTY_W<INT_CLR_SPEC> {
        TXFIFO_EMPTY_W::new(self, 1)
    }
    #[doc = "Bit 2 - Set this bit to clear UART_PARITY_ERR_INT interrupt."]
    #[inline(always)]
    pub fn parity_err(&mut self) -> PARITY_ERR_W<INT_CLR_SPEC> {
        PARITY_ERR_W::new(self, 2)
    }
    #[doc = "Bit 3 - Set this bit to clear UART_FRM_ERR_INT interrupt."]
    #[inline(always)]
    pub fn frm_err(&mut self) -> FRM_ERR_W<INT_CLR_SPEC> {
        FRM_ERR_W::new(self, 3)
    }
    #[doc = "Bit 4 - Set this bit to clear UART_UART_RXFIFO_OVF_INT interrupt."]
    #[inline(always)]
    pub fn rxfifo_ovf(&mut self) -> RXFIFO_OVF_W<INT_CLR_SPEC> {
        RXFIFO_OVF_W::new(self, 4)
    }
    #[doc = "Bit 5 - Set this bit to clear UART_DSR_CHG_INT interrupt."]
    #[inline(always)]
    pub fn dsr_chg(&mut self) -> DSR_CHG_W<INT_CLR_SPEC> {
        DSR_CHG_W::new(self, 5)
    }
    #[doc = "Bit 6 - Set this bit to clear UART_CTS_CHG_INT interrupt."]
    #[inline(always)]
    pub fn cts_chg(&mut self) -> CTS_CHG_W<INT_CLR_SPEC> {
        CTS_CHG_W::new(self, 6)
    }
    #[doc = "Bit 7 - Set this bit to clear UART_BRK_DET_INT interrupt."]
    #[inline(always)]
    pub fn brk_det(&mut self) -> BRK_DET_W<INT_CLR_SPEC> {
        BRK_DET_W::new(self, 7)
    }
    #[doc = "Bit 8 - Set this bit to clear UART_RXFIFO_TOUT_INT interrupt."]
    #[inline(always)]
    pub fn rxfifo_tout(&mut self) -> RXFIFO_TOUT_W<INT_CLR_SPEC> {
        RXFIFO_TOUT_W::new(self, 8)
    }
    #[doc = "Bit 9 - Set this bit to clear UART_SW_XON_INT interrupt."]
    #[inline(always)]
    pub fn sw_xon(&mut self) -> SW_XON_W<INT_CLR_SPEC> {
        SW_XON_W::new(self, 9)
    }
    #[doc = "Bit 10 - Set this bit to clear UART_SW_XOFF_INT interrupt."]
    #[inline(always)]
    pub fn sw_xoff(&mut self) -> SW_XOFF_W<INT_CLR_SPEC> {
        SW_XOFF_W::new(self, 10)
    }
    #[doc = "Bit 11 - Set this bit to clear UART_GLITCH_DET_INT interrupt."]
    #[inline(always)]
    pub fn glitch_det(&mut self) -> GLITCH_DET_W<INT_CLR_SPEC> {
        GLITCH_DET_W::new(self, 11)
    }
    #[doc = "Bit 12 - Set this bit to clear UART_TX_BRK_DONE_INT interrupt."]
    #[inline(always)]
    pub fn tx_brk_done(&mut self) -> TX_BRK_DONE_W<INT_CLR_SPEC> {
        TX_BRK_DONE_W::new(self, 12)
    }
    #[doc = "Bit 13 - Set this bit to clear UART_TX_BRK_IDLE_DONE_INT interrupt."]
    #[inline(always)]
    pub fn tx_brk_idle_done(&mut self) -> TX_BRK_IDLE_DONE_W<INT_CLR_SPEC> {
        TX_BRK_IDLE_DONE_W::new(self, 13)
    }
    #[doc = "Bit 14 - Set this bit to clear UART_TX_DONE_INT interrupt."]
    #[inline(always)]
    pub fn tx_done(&mut self) -> TX_DONE_W<INT_CLR_SPEC> {
        TX_DONE_W::new(self, 14)
    }
    #[doc = "Bit 15 - Set this bit to clear UART_RS485_PARITY_ERR_INT interrupt."]
    #[inline(always)]
    pub fn rs485_parity_err(&mut self) -> RS485_PARITY_ERR_W<INT_CLR_SPEC> {
        RS485_PARITY_ERR_W::new(self, 15)
    }
    #[doc = "Bit 16 - Set this bit to clear UART_RS485_FRM_ERR_INT interrupt."]
    #[inline(always)]
    pub fn rs485_frm_err(&mut self) -> RS485_FRM_ERR_W<INT_CLR_SPEC> {
        RS485_FRM_ERR_W::new(self, 16)
    }
    #[doc = "Bit 17 - Set this bit to clear UART_RS485_CLASH_INT interrupt."]
    #[inline(always)]
    pub fn rs485_clash(&mut self) -> RS485_CLASH_W<INT_CLR_SPEC> {
        RS485_CLASH_W::new(self, 17)
    }
    #[doc = "Bit 18 - Set this bit to clear UART_AT_CMD_CHAR_DET_INT interrupt."]
    #[inline(always)]
    pub fn at_cmd_char_det(&mut self) -> AT_CMD_CHAR_DET_W<INT_CLR_SPEC> {
        AT_CMD_CHAR_DET_W::new(self, 18)
    }
    #[doc = "Bit 19 - Set this bit to clear UART_WAKEUP_INT interrupt."]
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x000f_ffff;
}
#[doc = "`reset()` method sets INT_CLR to value 0"]
impl crate::Resettable for INT_CLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
