#[doc = "Register `INT_RAW` reader"]
pub type R = crate::R<INT_RAW_SPEC>;
#[doc = "Field `RXFIFO_FULL` reader - This interrupt raw bit turns to high level when receiver receives more data than (rx_flow_thrhd_h3 rx_flow_thrhd)."]
pub type RXFIFO_FULL_R = crate::BitReader;
#[doc = "Field `TXFIFO_EMPTY` reader - This interrupt raw bit turns to high level when the amount of data in transmitter's fifo is less than ((tx_mem_cnttxfifo_cnt) ."]
pub type TXFIFO_EMPTY_R = crate::BitReader;
#[doc = "Field `PARITY_ERR` reader - This interrupt raw bit turns to high level when receiver detects the parity error of data."]
pub type PARITY_ERR_R = crate::BitReader;
#[doc = "Field `FRM_ERR` reader - This interrupt raw bit turns to high level when receiver detects data's frame error ."]
pub type FRM_ERR_R = crate::BitReader;
#[doc = "Field `RXFIFO_OVF` reader - This interrupt raw bit turns to high level when receiver receives more data than the fifo can store."]
pub type RXFIFO_OVF_R = crate::BitReader;
#[doc = "Field `DSR_CHG` reader - This interrupt raw bit turns to high level when receiver detects the edge change of dsrn signal."]
pub type DSR_CHG_R = crate::BitReader;
#[doc = "Field `CTS_CHG` reader - This interrupt raw bit turns to high level when receiver detects the edge change of ctsn signal."]
pub type CTS_CHG_R = crate::BitReader;
#[doc = "Field `BRK_DET` reader - This interrupt raw bit turns to high level when receiver detects the 0 after the stop bit."]
pub type BRK_DET_R = crate::BitReader;
#[doc = "Field `RXFIFO_TOUT` reader - This interrupt raw bit turns to high level when receiver takes more time than rx_tout_thrhd to receive a byte."]
pub type RXFIFO_TOUT_R = crate::BitReader;
#[doc = "Field `SW_XON` reader - This interrupt raw bit turns to high level when receiver receives xoff char with uart_sw_flow_con_en is set to 1."]
pub type SW_XON_R = crate::BitReader;
#[doc = "Field `SW_XOFF` reader - This interrupt raw bit turns to high level when receiver receives xon char with uart_sw_flow_con_en is set to 1."]
pub type SW_XOFF_R = crate::BitReader;
#[doc = "Field `GLITCH_DET` reader - This interrupt raw bit turns to high level when receiver detects the start bit."]
pub type GLITCH_DET_R = crate::BitReader;
#[doc = "Field `TX_BRK_DONE` reader - This interrupt raw bit turns to high level when transmitter completes sendding 0 after all the datas in transmitter's fifo are send."]
pub type TX_BRK_DONE_R = crate::BitReader;
#[doc = "Field `TX_BRK_IDLE_DONE` reader - This interrupt raw bit turns to high level when transmitter has kept the shortest duration after the last data has been send."]
pub type TX_BRK_IDLE_DONE_R = crate::BitReader;
#[doc = "Field `TX_DONE` reader - This interrupt raw bit turns to high level when transmitter has send all the data in fifo."]
pub type TX_DONE_R = crate::BitReader;
#[doc = "Field `RS485_PARITY_ERR` reader - This interrupt raw bit turns to high level when rs485 detects the parity error."]
pub type RS485_PARITY_ERR_R = crate::BitReader;
#[doc = "Field `RS485_FRM_ERR` reader - This interrupt raw bit turns to high level when rs485 detects the data frame error."]
pub type RS485_FRM_ERR_R = crate::BitReader;
#[doc = "Field `RS485_CLASH` reader - This interrupt raw bit turns to high level when rs485 detects the clash between transmitter and receiver."]
pub type RS485_CLASH_R = crate::BitReader;
#[doc = "Field `AT_CMD_CHAR_DET` reader - This interrupt raw bit turns to high level when receiver detects the configured at_cmd chars."]
pub type AT_CMD_CHAR_DET_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - This interrupt raw bit turns to high level when receiver receives more data than (rx_flow_thrhd_h3 rx_flow_thrhd)."]
    #[inline(always)]
    pub fn rxfifo_full(&self) -> RXFIFO_FULL_R {
        RXFIFO_FULL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This interrupt raw bit turns to high level when the amount of data in transmitter's fifo is less than ((tx_mem_cnttxfifo_cnt) ."]
    #[inline(always)]
    pub fn txfifo_empty(&self) -> TXFIFO_EMPTY_R {
        TXFIFO_EMPTY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This interrupt raw bit turns to high level when receiver detects the parity error of data."]
    #[inline(always)]
    pub fn parity_err(&self) -> PARITY_ERR_R {
        PARITY_ERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This interrupt raw bit turns to high level when receiver detects data's frame error ."]
    #[inline(always)]
    pub fn frm_err(&self) -> FRM_ERR_R {
        FRM_ERR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This interrupt raw bit turns to high level when receiver receives more data than the fifo can store."]
    #[inline(always)]
    pub fn rxfifo_ovf(&self) -> RXFIFO_OVF_R {
        RXFIFO_OVF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This interrupt raw bit turns to high level when receiver detects the edge change of dsrn signal."]
    #[inline(always)]
    pub fn dsr_chg(&self) -> DSR_CHG_R {
        DSR_CHG_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - This interrupt raw bit turns to high level when receiver detects the edge change of ctsn signal."]
    #[inline(always)]
    pub fn cts_chg(&self) -> CTS_CHG_R {
        CTS_CHG_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - This interrupt raw bit turns to high level when receiver detects the 0 after the stop bit."]
    #[inline(always)]
    pub fn brk_det(&self) -> BRK_DET_R {
        BRK_DET_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - This interrupt raw bit turns to high level when receiver takes more time than rx_tout_thrhd to receive a byte."]
    #[inline(always)]
    pub fn rxfifo_tout(&self) -> RXFIFO_TOUT_R {
        RXFIFO_TOUT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - This interrupt raw bit turns to high level when receiver receives xoff char with uart_sw_flow_con_en is set to 1."]
    #[inline(always)]
    pub fn sw_xon(&self) -> SW_XON_R {
        SW_XON_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - This interrupt raw bit turns to high level when receiver receives xon char with uart_sw_flow_con_en is set to 1."]
    #[inline(always)]
    pub fn sw_xoff(&self) -> SW_XOFF_R {
        SW_XOFF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - This interrupt raw bit turns to high level when receiver detects the start bit."]
    #[inline(always)]
    pub fn glitch_det(&self) -> GLITCH_DET_R {
        GLITCH_DET_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - This interrupt raw bit turns to high level when transmitter completes sendding 0 after all the datas in transmitter's fifo are send."]
    #[inline(always)]
    pub fn tx_brk_done(&self) -> TX_BRK_DONE_R {
        TX_BRK_DONE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - This interrupt raw bit turns to high level when transmitter has kept the shortest duration after the last data has been send."]
    #[inline(always)]
    pub fn tx_brk_idle_done(&self) -> TX_BRK_IDLE_DONE_R {
        TX_BRK_IDLE_DONE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - This interrupt raw bit turns to high level when transmitter has send all the data in fifo."]
    #[inline(always)]
    pub fn tx_done(&self) -> TX_DONE_R {
        TX_DONE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - This interrupt raw bit turns to high level when rs485 detects the parity error."]
    #[inline(always)]
    pub fn rs485_parity_err(&self) -> RS485_PARITY_ERR_R {
        RS485_PARITY_ERR_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - This interrupt raw bit turns to high level when rs485 detects the data frame error."]
    #[inline(always)]
    pub fn rs485_frm_err(&self) -> RS485_FRM_ERR_R {
        RS485_FRM_ERR_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - This interrupt raw bit turns to high level when rs485 detects the clash between transmitter and receiver."]
    #[inline(always)]
    pub fn rs485_clash(&self) -> RS485_CLASH_R {
        RS485_CLASH_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - This interrupt raw bit turns to high level when receiver detects the configured at_cmd chars."]
    #[inline(always)]
    pub fn at_cmd_char_det(&self) -> AT_CMD_CHAR_DET_R {
        AT_CMD_CHAR_DET_R::new(((self.bits >> 18) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_RAW")
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
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`int_raw::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_RAW_SPEC;
impl crate::RegisterSpec for INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_raw::R`](R) reader structure"]
impl crate::Readable for INT_RAW_SPEC {}
#[doc = "`reset()` method sets INT_RAW to value 0"]
impl crate::Resettable for INT_RAW_SPEC {}
