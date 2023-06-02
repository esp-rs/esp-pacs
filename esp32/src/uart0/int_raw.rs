#[doc = "Register `INT_RAW` reader"]
pub struct R(crate::R<INT_RAW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_RAW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_RAW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_RAW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXFIFO_FULL_INT_RAW` reader - This interrupt raw bit turns to high level when receiver receives more data than (rx_flow_thrhd_h3 rx_flow_thrhd)."]
pub type RXFIFO_FULL_INT_RAW_R = crate::BitReader;
#[doc = "Field `TXFIFO_EMPTY_INT_RAW` reader - This interrupt raw bit turns to high level when the amount of data in transmitter's fifo is less than ((tx_mem_cnttxfifo_cnt) ."]
pub type TXFIFO_EMPTY_INT_RAW_R = crate::BitReader;
#[doc = "Field `PARITY_ERR_INT_RAW` reader - This interrupt raw bit turns to high level when receiver detects the parity error of data."]
pub type PARITY_ERR_INT_RAW_R = crate::BitReader;
#[doc = "Field `FRM_ERR_INT_RAW` reader - This interrupt raw bit turns to high level when receiver detects data's frame error ."]
pub type FRM_ERR_INT_RAW_R = crate::BitReader;
#[doc = "Field `RXFIFO_OVF_INT_RAW` reader - This interrupt raw bit turns to high level when receiver receives more data than the fifo can store."]
pub type RXFIFO_OVF_INT_RAW_R = crate::BitReader;
#[doc = "Field `DSR_CHG_INT_RAW` reader - This interrupt raw bit turns to high level when receiver detects the edge change of dsrn signal."]
pub type DSR_CHG_INT_RAW_R = crate::BitReader;
#[doc = "Field `CTS_CHG_INT_RAW` reader - This interrupt raw bit turns to high level when receiver detects the edge change of ctsn signal."]
pub type CTS_CHG_INT_RAW_R = crate::BitReader;
#[doc = "Field `BRK_DET_INT_RAW` reader - This interrupt raw bit turns to high level when receiver detects the 0 after the stop bit."]
pub type BRK_DET_INT_RAW_R = crate::BitReader;
#[doc = "Field `RXFIFO_TOUT_INT_RAW` reader - This interrupt raw bit turns to high level when receiver takes more time than rx_tout_thrhd to receive a byte."]
pub type RXFIFO_TOUT_INT_RAW_R = crate::BitReader;
#[doc = "Field `SW_XON_INT_RAW` reader - This interrupt raw bit turns to high level when receiver receives xoff char with uart_sw_flow_con_en is set to 1."]
pub type SW_XON_INT_RAW_R = crate::BitReader;
#[doc = "Field `SW_XOFF_INT_RAW` reader - This interrupt raw bit turns to high level when receiver receives xon char with uart_sw_flow_con_en is set to 1."]
pub type SW_XOFF_INT_RAW_R = crate::BitReader;
#[doc = "Field `GLITCH_DET_INT_RAW` reader - This interrupt raw bit turns to high level when receiver detects the start bit."]
pub type GLITCH_DET_INT_RAW_R = crate::BitReader;
#[doc = "Field `TX_BRK_DONE_INT_RAW` reader - This interrupt raw bit turns to high level when transmitter completes sendding 0 after all the datas in transmitter's fifo are send."]
pub type TX_BRK_DONE_INT_RAW_R = crate::BitReader;
#[doc = "Field `TX_BRK_IDLE_DONE_INT_RAW` reader - This interrupt raw bit turns to high level when transmitter has kept the shortest duration after the last data has been send."]
pub type TX_BRK_IDLE_DONE_INT_RAW_R = crate::BitReader;
#[doc = "Field `TX_DONE_INT_RAW` reader - This interrupt raw bit turns to high level when transmitter has send all the data in fifo."]
pub type TX_DONE_INT_RAW_R = crate::BitReader;
#[doc = "Field `RS485_PARITY_ERR_INT_RAW` reader - This interrupt raw bit turns to high level when rs485 detects the parity error."]
pub type RS485_PARITY_ERR_INT_RAW_R = crate::BitReader;
#[doc = "Field `RS485_FRM_ERR_INT_RAW` reader - This interrupt raw bit turns to high level when rs485 detects the data frame error."]
pub type RS485_FRM_ERR_INT_RAW_R = crate::BitReader;
#[doc = "Field `RS485_CLASH_INT_RAW` reader - This interrupt raw bit turns to high level when rs485 detects the clash between transmitter and receiver."]
pub type RS485_CLASH_INT_RAW_R = crate::BitReader;
#[doc = "Field `AT_CMD_CHAR_DET_INT_RAW` reader - This interrupt raw bit turns to high level when receiver detects the configured at_cmd chars."]
pub type AT_CMD_CHAR_DET_INT_RAW_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - This interrupt raw bit turns to high level when receiver receives more data than (rx_flow_thrhd_h3 rx_flow_thrhd)."]
    #[inline(always)]
    pub fn rxfifo_full_int_raw(&self) -> RXFIFO_FULL_INT_RAW_R {
        RXFIFO_FULL_INT_RAW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This interrupt raw bit turns to high level when the amount of data in transmitter's fifo is less than ((tx_mem_cnttxfifo_cnt) ."]
    #[inline(always)]
    pub fn txfifo_empty_int_raw(&self) -> TXFIFO_EMPTY_INT_RAW_R {
        TXFIFO_EMPTY_INT_RAW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This interrupt raw bit turns to high level when receiver detects the parity error of data."]
    #[inline(always)]
    pub fn parity_err_int_raw(&self) -> PARITY_ERR_INT_RAW_R {
        PARITY_ERR_INT_RAW_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This interrupt raw bit turns to high level when receiver detects data's frame error ."]
    #[inline(always)]
    pub fn frm_err_int_raw(&self) -> FRM_ERR_INT_RAW_R {
        FRM_ERR_INT_RAW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This interrupt raw bit turns to high level when receiver receives more data than the fifo can store."]
    #[inline(always)]
    pub fn rxfifo_ovf_int_raw(&self) -> RXFIFO_OVF_INT_RAW_R {
        RXFIFO_OVF_INT_RAW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This interrupt raw bit turns to high level when receiver detects the edge change of dsrn signal."]
    #[inline(always)]
    pub fn dsr_chg_int_raw(&self) -> DSR_CHG_INT_RAW_R {
        DSR_CHG_INT_RAW_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - This interrupt raw bit turns to high level when receiver detects the edge change of ctsn signal."]
    #[inline(always)]
    pub fn cts_chg_int_raw(&self) -> CTS_CHG_INT_RAW_R {
        CTS_CHG_INT_RAW_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - This interrupt raw bit turns to high level when receiver detects the 0 after the stop bit."]
    #[inline(always)]
    pub fn brk_det_int_raw(&self) -> BRK_DET_INT_RAW_R {
        BRK_DET_INT_RAW_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - This interrupt raw bit turns to high level when receiver takes more time than rx_tout_thrhd to receive a byte."]
    #[inline(always)]
    pub fn rxfifo_tout_int_raw(&self) -> RXFIFO_TOUT_INT_RAW_R {
        RXFIFO_TOUT_INT_RAW_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - This interrupt raw bit turns to high level when receiver receives xoff char with uart_sw_flow_con_en is set to 1."]
    #[inline(always)]
    pub fn sw_xon_int_raw(&self) -> SW_XON_INT_RAW_R {
        SW_XON_INT_RAW_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - This interrupt raw bit turns to high level when receiver receives xon char with uart_sw_flow_con_en is set to 1."]
    #[inline(always)]
    pub fn sw_xoff_int_raw(&self) -> SW_XOFF_INT_RAW_R {
        SW_XOFF_INT_RAW_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - This interrupt raw bit turns to high level when receiver detects the start bit."]
    #[inline(always)]
    pub fn glitch_det_int_raw(&self) -> GLITCH_DET_INT_RAW_R {
        GLITCH_DET_INT_RAW_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - This interrupt raw bit turns to high level when transmitter completes sendding 0 after all the datas in transmitter's fifo are send."]
    #[inline(always)]
    pub fn tx_brk_done_int_raw(&self) -> TX_BRK_DONE_INT_RAW_R {
        TX_BRK_DONE_INT_RAW_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - This interrupt raw bit turns to high level when transmitter has kept the shortest duration after the last data has been send."]
    #[inline(always)]
    pub fn tx_brk_idle_done_int_raw(&self) -> TX_BRK_IDLE_DONE_INT_RAW_R {
        TX_BRK_IDLE_DONE_INT_RAW_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - This interrupt raw bit turns to high level when transmitter has send all the data in fifo."]
    #[inline(always)]
    pub fn tx_done_int_raw(&self) -> TX_DONE_INT_RAW_R {
        TX_DONE_INT_RAW_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - This interrupt raw bit turns to high level when rs485 detects the parity error."]
    #[inline(always)]
    pub fn rs485_parity_err_int_raw(&self) -> RS485_PARITY_ERR_INT_RAW_R {
        RS485_PARITY_ERR_INT_RAW_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - This interrupt raw bit turns to high level when rs485 detects the data frame error."]
    #[inline(always)]
    pub fn rs485_frm_err_int_raw(&self) -> RS485_FRM_ERR_INT_RAW_R {
        RS485_FRM_ERR_INT_RAW_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - This interrupt raw bit turns to high level when rs485 detects the clash between transmitter and receiver."]
    #[inline(always)]
    pub fn rs485_clash_int_raw(&self) -> RS485_CLASH_INT_RAW_R {
        RS485_CLASH_INT_RAW_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - This interrupt raw bit turns to high level when receiver detects the configured at_cmd chars."]
    #[inline(always)]
    pub fn at_cmd_char_det_int_raw(&self) -> AT_CMD_CHAR_DET_INT_RAW_R {
        AT_CMD_CHAR_DET_INT_RAW_R::new(((self.bits >> 18) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_RAW")
            .field(
                "rxfifo_full_int_raw",
                &format_args!("{}", self.rxfifo_full_int_raw().bit()),
            )
            .field(
                "txfifo_empty_int_raw",
                &format_args!("{}", self.txfifo_empty_int_raw().bit()),
            )
            .field(
                "parity_err_int_raw",
                &format_args!("{}", self.parity_err_int_raw().bit()),
            )
            .field(
                "frm_err_int_raw",
                &format_args!("{}", self.frm_err_int_raw().bit()),
            )
            .field(
                "rxfifo_ovf_int_raw",
                &format_args!("{}", self.rxfifo_ovf_int_raw().bit()),
            )
            .field(
                "dsr_chg_int_raw",
                &format_args!("{}", self.dsr_chg_int_raw().bit()),
            )
            .field(
                "cts_chg_int_raw",
                &format_args!("{}", self.cts_chg_int_raw().bit()),
            )
            .field(
                "brk_det_int_raw",
                &format_args!("{}", self.brk_det_int_raw().bit()),
            )
            .field(
                "rxfifo_tout_int_raw",
                &format_args!("{}", self.rxfifo_tout_int_raw().bit()),
            )
            .field(
                "sw_xon_int_raw",
                &format_args!("{}", self.sw_xon_int_raw().bit()),
            )
            .field(
                "sw_xoff_int_raw",
                &format_args!("{}", self.sw_xoff_int_raw().bit()),
            )
            .field(
                "glitch_det_int_raw",
                &format_args!("{}", self.glitch_det_int_raw().bit()),
            )
            .field(
                "tx_brk_done_int_raw",
                &format_args!("{}", self.tx_brk_done_int_raw().bit()),
            )
            .field(
                "tx_brk_idle_done_int_raw",
                &format_args!("{}", self.tx_brk_idle_done_int_raw().bit()),
            )
            .field(
                "tx_done_int_raw",
                &format_args!("{}", self.tx_done_int_raw().bit()),
            )
            .field(
                "rs485_parity_err_int_raw",
                &format_args!("{}", self.rs485_parity_err_int_raw().bit()),
            )
            .field(
                "rs485_frm_err_int_raw",
                &format_args!("{}", self.rs485_frm_err_int_raw().bit()),
            )
            .field(
                "rs485_clash_int_raw",
                &format_args!("{}", self.rs485_clash_int_raw().bit()),
            )
            .field(
                "at_cmd_char_det_int_raw",
                &format_args!("{}", self.at_cmd_char_det_int_raw().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_RAW_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_raw](index.html) module"]
pub struct INT_RAW_SPEC;
impl crate::RegisterSpec for INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_raw::R](R) reader structure"]
impl crate::Readable for INT_RAW_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INT_RAW to value 0"]
impl crate::Resettable for INT_RAW_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
