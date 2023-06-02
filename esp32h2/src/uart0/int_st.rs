#[doc = "Register `INT_ST` reader"]
pub struct R(crate::R<INT_ST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_ST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_ST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_ST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXFIFO_FULL_INT_ST` reader - This is the status bit for rxfifo_full_int_raw when rxfifo_full_int_ena is set to 1."]
pub type RXFIFO_FULL_INT_ST_R = crate::BitReader;
#[doc = "Field `TXFIFO_EMPTY_INT_ST` reader - This is the status bit for txfifo_empty_int_raw when txfifo_empty_int_ena is set to 1."]
pub type TXFIFO_EMPTY_INT_ST_R = crate::BitReader;
#[doc = "Field `PARITY_ERR_INT_ST` reader - This is the status bit for parity_err_int_raw when parity_err_int_ena is set to 1."]
pub type PARITY_ERR_INT_ST_R = crate::BitReader;
#[doc = "Field `FRM_ERR_INT_ST` reader - This is the status bit for frm_err_int_raw when frm_err_int_ena is set to 1."]
pub type FRM_ERR_INT_ST_R = crate::BitReader;
#[doc = "Field `RXFIFO_OVF_INT_ST` reader - This is the status bit for rxfifo_ovf_int_raw when rxfifo_ovf_int_ena is set to 1."]
pub type RXFIFO_OVF_INT_ST_R = crate::BitReader;
#[doc = "Field `DSR_CHG_INT_ST` reader - This is the status bit for dsr_chg_int_raw when dsr_chg_int_ena is set to 1."]
pub type DSR_CHG_INT_ST_R = crate::BitReader;
#[doc = "Field `CTS_CHG_INT_ST` reader - This is the status bit for cts_chg_int_raw when cts_chg_int_ena is set to 1."]
pub type CTS_CHG_INT_ST_R = crate::BitReader;
#[doc = "Field `BRK_DET_INT_ST` reader - This is the status bit for brk_det_int_raw when brk_det_int_ena is set to 1."]
pub type BRK_DET_INT_ST_R = crate::BitReader;
#[doc = "Field `RXFIFO_TOUT_INT_ST` reader - This is the status bit for rxfifo_tout_int_raw when rxfifo_tout_int_ena is set to 1."]
pub type RXFIFO_TOUT_INT_ST_R = crate::BitReader;
#[doc = "Field `SW_XON_INT_ST` reader - This is the status bit for sw_xon_int_raw when sw_xon_int_ena is set to 1."]
pub type SW_XON_INT_ST_R = crate::BitReader;
#[doc = "Field `SW_XOFF_INT_ST` reader - This is the status bit for sw_xoff_int_raw when sw_xoff_int_ena is set to 1."]
pub type SW_XOFF_INT_ST_R = crate::BitReader;
#[doc = "Field `GLITCH_DET_INT_ST` reader - This is the status bit for glitch_det_int_raw when glitch_det_int_ena is set to 1."]
pub type GLITCH_DET_INT_ST_R = crate::BitReader;
#[doc = "Field `TX_BRK_DONE_INT_ST` reader - This is the status bit for tx_brk_done_int_raw when tx_brk_done_int_ena is set to 1."]
pub type TX_BRK_DONE_INT_ST_R = crate::BitReader;
#[doc = "Field `TX_BRK_IDLE_DONE_INT_ST` reader - This is the stauts bit for tx_brk_idle_done_int_raw when tx_brk_idle_done_int_ena is set to 1."]
pub type TX_BRK_IDLE_DONE_INT_ST_R = crate::BitReader;
#[doc = "Field `TX_DONE_INT_ST` reader - This is the status bit for tx_done_int_raw when tx_done_int_ena is set to 1."]
pub type TX_DONE_INT_ST_R = crate::BitReader;
#[doc = "Field `RS485_PARITY_ERR_INT_ST` reader - This is the status bit for rs485_parity_err_int_raw when rs485_parity_int_ena is set to 1."]
pub type RS485_PARITY_ERR_INT_ST_R = crate::BitReader;
#[doc = "Field `RS485_FRM_ERR_INT_ST` reader - This is the status bit for rs485_frm_err_int_raw when rs485_fm_err_int_ena is set to 1."]
pub type RS485_FRM_ERR_INT_ST_R = crate::BitReader;
#[doc = "Field `RS485_CLASH_INT_ST` reader - This is the status bit for rs485_clash_int_raw when rs485_clash_int_ena is set to 1."]
pub type RS485_CLASH_INT_ST_R = crate::BitReader;
#[doc = "Field `AT_CMD_CHAR_DET_INT_ST` reader - This is the status bit for at_cmd_det_int_raw when at_cmd_char_det_int_ena is set to 1."]
pub type AT_CMD_CHAR_DET_INT_ST_R = crate::BitReader;
#[doc = "Field `WAKEUP_INT_ST` reader - This is the status bit for uart_wakeup_int_raw when uart_wakeup_int_ena is set to 1."]
pub type WAKEUP_INT_ST_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - This is the status bit for rxfifo_full_int_raw when rxfifo_full_int_ena is set to 1."]
    #[inline(always)]
    pub fn rxfifo_full_int_st(&self) -> RXFIFO_FULL_INT_ST_R {
        RXFIFO_FULL_INT_ST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This is the status bit for txfifo_empty_int_raw when txfifo_empty_int_ena is set to 1."]
    #[inline(always)]
    pub fn txfifo_empty_int_st(&self) -> TXFIFO_EMPTY_INT_ST_R {
        TXFIFO_EMPTY_INT_ST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This is the status bit for parity_err_int_raw when parity_err_int_ena is set to 1."]
    #[inline(always)]
    pub fn parity_err_int_st(&self) -> PARITY_ERR_INT_ST_R {
        PARITY_ERR_INT_ST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This is the status bit for frm_err_int_raw when frm_err_int_ena is set to 1."]
    #[inline(always)]
    pub fn frm_err_int_st(&self) -> FRM_ERR_INT_ST_R {
        FRM_ERR_INT_ST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This is the status bit for rxfifo_ovf_int_raw when rxfifo_ovf_int_ena is set to 1."]
    #[inline(always)]
    pub fn rxfifo_ovf_int_st(&self) -> RXFIFO_OVF_INT_ST_R {
        RXFIFO_OVF_INT_ST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This is the status bit for dsr_chg_int_raw when dsr_chg_int_ena is set to 1."]
    #[inline(always)]
    pub fn dsr_chg_int_st(&self) -> DSR_CHG_INT_ST_R {
        DSR_CHG_INT_ST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - This is the status bit for cts_chg_int_raw when cts_chg_int_ena is set to 1."]
    #[inline(always)]
    pub fn cts_chg_int_st(&self) -> CTS_CHG_INT_ST_R {
        CTS_CHG_INT_ST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - This is the status bit for brk_det_int_raw when brk_det_int_ena is set to 1."]
    #[inline(always)]
    pub fn brk_det_int_st(&self) -> BRK_DET_INT_ST_R {
        BRK_DET_INT_ST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - This is the status bit for rxfifo_tout_int_raw when rxfifo_tout_int_ena is set to 1."]
    #[inline(always)]
    pub fn rxfifo_tout_int_st(&self) -> RXFIFO_TOUT_INT_ST_R {
        RXFIFO_TOUT_INT_ST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - This is the status bit for sw_xon_int_raw when sw_xon_int_ena is set to 1."]
    #[inline(always)]
    pub fn sw_xon_int_st(&self) -> SW_XON_INT_ST_R {
        SW_XON_INT_ST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - This is the status bit for sw_xoff_int_raw when sw_xoff_int_ena is set to 1."]
    #[inline(always)]
    pub fn sw_xoff_int_st(&self) -> SW_XOFF_INT_ST_R {
        SW_XOFF_INT_ST_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - This is the status bit for glitch_det_int_raw when glitch_det_int_ena is set to 1."]
    #[inline(always)]
    pub fn glitch_det_int_st(&self) -> GLITCH_DET_INT_ST_R {
        GLITCH_DET_INT_ST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - This is the status bit for tx_brk_done_int_raw when tx_brk_done_int_ena is set to 1."]
    #[inline(always)]
    pub fn tx_brk_done_int_st(&self) -> TX_BRK_DONE_INT_ST_R {
        TX_BRK_DONE_INT_ST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - This is the stauts bit for tx_brk_idle_done_int_raw when tx_brk_idle_done_int_ena is set to 1."]
    #[inline(always)]
    pub fn tx_brk_idle_done_int_st(&self) -> TX_BRK_IDLE_DONE_INT_ST_R {
        TX_BRK_IDLE_DONE_INT_ST_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - This is the status bit for tx_done_int_raw when tx_done_int_ena is set to 1."]
    #[inline(always)]
    pub fn tx_done_int_st(&self) -> TX_DONE_INT_ST_R {
        TX_DONE_INT_ST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - This is the status bit for rs485_parity_err_int_raw when rs485_parity_int_ena is set to 1."]
    #[inline(always)]
    pub fn rs485_parity_err_int_st(&self) -> RS485_PARITY_ERR_INT_ST_R {
        RS485_PARITY_ERR_INT_ST_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - This is the status bit for rs485_frm_err_int_raw when rs485_fm_err_int_ena is set to 1."]
    #[inline(always)]
    pub fn rs485_frm_err_int_st(&self) -> RS485_FRM_ERR_INT_ST_R {
        RS485_FRM_ERR_INT_ST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - This is the status bit for rs485_clash_int_raw when rs485_clash_int_ena is set to 1."]
    #[inline(always)]
    pub fn rs485_clash_int_st(&self) -> RS485_CLASH_INT_ST_R {
        RS485_CLASH_INT_ST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - This is the status bit for at_cmd_det_int_raw when at_cmd_char_det_int_ena is set to 1."]
    #[inline(always)]
    pub fn at_cmd_char_det_int_st(&self) -> AT_CMD_CHAR_DET_INT_ST_R {
        AT_CMD_CHAR_DET_INT_ST_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - This is the status bit for uart_wakeup_int_raw when uart_wakeup_int_ena is set to 1."]
    #[inline(always)]
    pub fn wakeup_int_st(&self) -> WAKEUP_INT_ST_R {
        WAKEUP_INT_ST_R::new(((self.bits >> 19) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ST")
            .field(
                "rxfifo_full_int_st",
                &format_args!("{}", self.rxfifo_full_int_st().bit()),
            )
            .field(
                "txfifo_empty_int_st",
                &format_args!("{}", self.txfifo_empty_int_st().bit()),
            )
            .field(
                "parity_err_int_st",
                &format_args!("{}", self.parity_err_int_st().bit()),
            )
            .field(
                "frm_err_int_st",
                &format_args!("{}", self.frm_err_int_st().bit()),
            )
            .field(
                "rxfifo_ovf_int_st",
                &format_args!("{}", self.rxfifo_ovf_int_st().bit()),
            )
            .field(
                "dsr_chg_int_st",
                &format_args!("{}", self.dsr_chg_int_st().bit()),
            )
            .field(
                "cts_chg_int_st",
                &format_args!("{}", self.cts_chg_int_st().bit()),
            )
            .field(
                "brk_det_int_st",
                &format_args!("{}", self.brk_det_int_st().bit()),
            )
            .field(
                "rxfifo_tout_int_st",
                &format_args!("{}", self.rxfifo_tout_int_st().bit()),
            )
            .field(
                "sw_xon_int_st",
                &format_args!("{}", self.sw_xon_int_st().bit()),
            )
            .field(
                "sw_xoff_int_st",
                &format_args!("{}", self.sw_xoff_int_st().bit()),
            )
            .field(
                "glitch_det_int_st",
                &format_args!("{}", self.glitch_det_int_st().bit()),
            )
            .field(
                "tx_brk_done_int_st",
                &format_args!("{}", self.tx_brk_done_int_st().bit()),
            )
            .field(
                "tx_brk_idle_done_int_st",
                &format_args!("{}", self.tx_brk_idle_done_int_st().bit()),
            )
            .field(
                "tx_done_int_st",
                &format_args!("{}", self.tx_done_int_st().bit()),
            )
            .field(
                "rs485_parity_err_int_st",
                &format_args!("{}", self.rs485_parity_err_int_st().bit()),
            )
            .field(
                "rs485_frm_err_int_st",
                &format_args!("{}", self.rs485_frm_err_int_st().bit()),
            )
            .field(
                "rs485_clash_int_st",
                &format_args!("{}", self.rs485_clash_int_st().bit()),
            )
            .field(
                "at_cmd_char_det_int_st",
                &format_args!("{}", self.at_cmd_char_det_int_st().bit()),
            )
            .field(
                "wakeup_int_st",
                &format_args!("{}", self.wakeup_int_st().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_ST_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Masked interrupt status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_st](index.html) module"]
pub struct INT_ST_SPEC;
impl crate::RegisterSpec for INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_st::R](R) reader structure"]
impl crate::Readable for INT_ST_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INT_ST to value 0"]
impl crate::Resettable for INT_ST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
