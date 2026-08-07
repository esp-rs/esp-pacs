#[doc = "Register `INT_ST` reader"]
pub type R = crate::R<INT_ST_SPEC>;
#[doc = "Field `RXFIFO_FULL` reader - The masked interrupt status of UART_RXFIFO_FULL_INT."]
pub type RXFIFO_FULL_R = crate::BitReader;
#[doc = "Field `TXFIFO_EMPTY` reader - The masked interrupt status of UART_TXFIFO_EMPTY_INT."]
pub type TXFIFO_EMPTY_R = crate::BitReader;
#[doc = "Field `PARITY_ERR` reader - The masked interrupt status of UART_PARITY_ERR_INT."]
pub type PARITY_ERR_R = crate::BitReader;
#[doc = "Field `FRM_ERR` reader - The masked interrupt status of UART_FRM_ERR_INT."]
pub type FRM_ERR_R = crate::BitReader;
#[doc = "Field `RXFIFO_OVF` reader - The masked interrupt status of UART_RXFIFO_OVF_INT."]
pub type RXFIFO_OVF_R = crate::BitReader;
#[doc = "Field `DSR_CHG` reader - The masked interrupt status of UART_DSR_CHG_INT."]
pub type DSR_CHG_R = crate::BitReader;
#[doc = "Field `CTS_CHG` reader - The masked interrupt status of UART_CTS_CHG_INT."]
pub type CTS_CHG_R = crate::BitReader;
#[doc = "Field `BRK_DET` reader - The masked interrupt status of UART_BRK_DET_INT."]
pub type BRK_DET_R = crate::BitReader;
#[doc = "Field `RXFIFO_TOUT` reader - The masked interrupt status of UART_RXFIFO_TOUT_INT."]
pub type RXFIFO_TOUT_R = crate::BitReader;
#[doc = "Field `SW_XON` reader - The masked interrupt status of UART_SW_XON_INT."]
pub type SW_XON_R = crate::BitReader;
#[doc = "Field `SW_XOFF` reader - The masked interrupt status of UART_SW_XOFF_INT."]
pub type SW_XOFF_R = crate::BitReader;
#[doc = "Field `GLITCH_DET` reader - The masked interrupt status of UART_GLITCH_DET_INT."]
pub type GLITCH_DET_R = crate::BitReader;
#[doc = "Field `TX_BRK_DONE` reader - The masked interrupt status of UART_TX_BRK_DONE_INT."]
pub type TX_BRK_DONE_R = crate::BitReader;
#[doc = "Field `TX_BRK_IDLE_DONE` reader - The masked interrupt status of UART_TX_BRK_IDLE_DONE_INT."]
pub type TX_BRK_IDLE_DONE_R = crate::BitReader;
#[doc = "Field `TX_DONE` reader - The masked interrupt status of UART_TX_DONE_INT."]
pub type TX_DONE_R = crate::BitReader;
#[doc = "Field `RS485_PARITY_ERR` reader - The masked interrupt status of UART_RS485_PARITY_ERR_INT."]
pub type RS485_PARITY_ERR_R = crate::BitReader;
#[doc = "Field `RS485_FRM_ERR` reader - The masked interrupt status of UART_RS485_FRM_ERR_INT."]
pub type RS485_FRM_ERR_R = crate::BitReader;
#[doc = "Field `RS485_CLASH` reader - The masked interrupt status of UART_RS485_CLASH_INT."]
pub type RS485_CLASH_R = crate::BitReader;
#[doc = "Field `AT_CMD_CHAR_DET` reader - The masked interrupt status of UART_AT_CMD_CHAR_DET_INT."]
pub type AT_CMD_CHAR_DET_R = crate::BitReader;
#[doc = "Field `WAKEUP` reader - The masked interrupt status of UART_WAKEUP_INT."]
pub type WAKEUP_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The masked interrupt status of UART_RXFIFO_FULL_INT."]
    #[inline(always)]
    pub fn rxfifo_full(&self) -> RXFIFO_FULL_R {
        RXFIFO_FULL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The masked interrupt status of UART_TXFIFO_EMPTY_INT."]
    #[inline(always)]
    pub fn txfifo_empty(&self) -> TXFIFO_EMPTY_R {
        TXFIFO_EMPTY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The masked interrupt status of UART_PARITY_ERR_INT."]
    #[inline(always)]
    pub fn parity_err(&self) -> PARITY_ERR_R {
        PARITY_ERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The masked interrupt status of UART_FRM_ERR_INT."]
    #[inline(always)]
    pub fn frm_err(&self) -> FRM_ERR_R {
        FRM_ERR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The masked interrupt status of UART_RXFIFO_OVF_INT."]
    #[inline(always)]
    pub fn rxfifo_ovf(&self) -> RXFIFO_OVF_R {
        RXFIFO_OVF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The masked interrupt status of UART_DSR_CHG_INT."]
    #[inline(always)]
    pub fn dsr_chg(&self) -> DSR_CHG_R {
        DSR_CHG_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The masked interrupt status of UART_CTS_CHG_INT."]
    #[inline(always)]
    pub fn cts_chg(&self) -> CTS_CHG_R {
        CTS_CHG_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The masked interrupt status of UART_BRK_DET_INT."]
    #[inline(always)]
    pub fn brk_det(&self) -> BRK_DET_R {
        BRK_DET_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The masked interrupt status of UART_RXFIFO_TOUT_INT."]
    #[inline(always)]
    pub fn rxfifo_tout(&self) -> RXFIFO_TOUT_R {
        RXFIFO_TOUT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The masked interrupt status of UART_SW_XON_INT."]
    #[inline(always)]
    pub fn sw_xon(&self) -> SW_XON_R {
        SW_XON_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - The masked interrupt status of UART_SW_XOFF_INT."]
    #[inline(always)]
    pub fn sw_xoff(&self) -> SW_XOFF_R {
        SW_XOFF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The masked interrupt status of UART_GLITCH_DET_INT."]
    #[inline(always)]
    pub fn glitch_det(&self) -> GLITCH_DET_R {
        GLITCH_DET_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - The masked interrupt status of UART_TX_BRK_DONE_INT."]
    #[inline(always)]
    pub fn tx_brk_done(&self) -> TX_BRK_DONE_R {
        TX_BRK_DONE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - The masked interrupt status of UART_TX_BRK_IDLE_DONE_INT."]
    #[inline(always)]
    pub fn tx_brk_idle_done(&self) -> TX_BRK_IDLE_DONE_R {
        TX_BRK_IDLE_DONE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - The masked interrupt status of UART_TX_DONE_INT."]
    #[inline(always)]
    pub fn tx_done(&self) -> TX_DONE_R {
        TX_DONE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - The masked interrupt status of UART_RS485_PARITY_ERR_INT."]
    #[inline(always)]
    pub fn rs485_parity_err(&self) -> RS485_PARITY_ERR_R {
        RS485_PARITY_ERR_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - The masked interrupt status of UART_RS485_FRM_ERR_INT."]
    #[inline(always)]
    pub fn rs485_frm_err(&self) -> RS485_FRM_ERR_R {
        RS485_FRM_ERR_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - The masked interrupt status of UART_RS485_CLASH_INT."]
    #[inline(always)]
    pub fn rs485_clash(&self) -> RS485_CLASH_R {
        RS485_CLASH_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - The masked interrupt status of UART_AT_CMD_CHAR_DET_INT."]
    #[inline(always)]
    pub fn at_cmd_char_det(&self) -> AT_CMD_CHAR_DET_R {
        AT_CMD_CHAR_DET_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - The masked interrupt status of UART_WAKEUP_INT."]
    #[inline(always)]
    pub fn wakeup(&self) -> WAKEUP_R {
        WAKEUP_R::new(((self.bits >> 19) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ST")
            .field("rxfifo_full", &self.rxfifo_full())
            .field("txfifo_empty", &self.txfifo_empty())
            .field("parity_err", &self.parity_err())
            .field("frm_err", &self.frm_err())
            .field("rxfifo_ovf", &self.rxfifo_ovf())
            .field("dsr_chg", &self.dsr_chg())
            .field("cts_chg", &self.cts_chg())
            .field("brk_det", &self.brk_det())
            .field("rxfifo_tout", &self.rxfifo_tout())
            .field("sw_xon", &self.sw_xon())
            .field("sw_xoff", &self.sw_xoff())
            .field("glitch_det", &self.glitch_det())
            .field("tx_brk_done", &self.tx_brk_done())
            .field("tx_brk_idle_done", &self.tx_brk_idle_done())
            .field("tx_done", &self.tx_done())
            .field("rs485_parity_err", &self.rs485_parity_err())
            .field("rs485_frm_err", &self.rs485_frm_err())
            .field("rs485_clash", &self.rs485_clash())
            .field("at_cmd_char_det", &self.at_cmd_char_det())
            .field("wakeup", &self.wakeup())
            .finish()
    }
}
#[doc = "Masked interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`int_st::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ST_SPEC;
impl crate::RegisterSpec for INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_st::R`](R) reader structure"]
impl crate::Readable for INT_ST_SPEC {}
#[doc = "`reset()` method sets INT_ST to value 0"]
impl crate::Resettable for INT_ST_SPEC {}
