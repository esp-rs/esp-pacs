#[doc = "Register `INT_CLR` writer"]
pub struct W(crate::W<INT_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_CLR_SPEC>;
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
impl From<crate::W<INT_CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXFIFO_FULL_INT_CLR` writer - Set this bit to clear the rxfifo_full_int_raw interrupt."]
pub type RXFIFO_FULL_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 0>;
#[doc = "Field `TXFIFO_EMPTY_INT_CLR` writer - Set this bit to clear txfifo_empty_int_raw interrupt."]
pub type TXFIFO_EMPTY_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 1>;
#[doc = "Field `PARITY_ERR_INT_CLR` writer - Set this bit to clear parity_err_int_raw interrupt."]
pub type PARITY_ERR_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 2>;
#[doc = "Field `FRM_ERR_INT_CLR` writer - Set this bit to clear frm_err_int_raw interrupt."]
pub type FRM_ERR_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 3>;
#[doc = "Field `RXFIFO_OVF_INT_CLR` writer - Set this bit to clear rxfifo_ovf_int_raw interrupt."]
pub type RXFIFO_OVF_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 4>;
#[doc = "Field `DSR_CHG_INT_CLR` writer - Set this bit to clear the dsr_chg_int_raw interrupt."]
pub type DSR_CHG_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 5>;
#[doc = "Field `CTS_CHG_INT_CLR` writer - Set this bit to clear the cts_chg_int_raw interrupt."]
pub type CTS_CHG_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 6>;
#[doc = "Field `BRK_DET_INT_CLR` writer - Set this bit to clear the brk_det_int_raw interrupt."]
pub type BRK_DET_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 7>;
#[doc = "Field `RXFIFO_TOUT_INT_CLR` writer - Set this bit to clear the rxfifo_tout_int_raw interrupt."]
pub type RXFIFO_TOUT_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 8>;
#[doc = "Field `SW_XON_INT_CLR` writer - Set this bit to clear the sw_xon_int_raw interrupt."]
pub type SW_XON_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 9>;
#[doc = "Field `SW_XOFF_INT_CLR` writer - Set this bit to clear the sw_xoff_int_raw interrupt."]
pub type SW_XOFF_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 10>;
#[doc = "Field `GLITCH_DET_INT_CLR` writer - Set this bit to clear the glitch_det_int_raw interrupt."]
pub type GLITCH_DET_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 11>;
#[doc = "Field `TX_BRK_DONE_INT_CLR` writer - Set this bit to clear the tx_brk_done_int_raw interrupt.."]
pub type TX_BRK_DONE_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 12>;
#[doc = "Field `TX_BRK_IDLE_DONE_INT_CLR` writer - Set this bit to clear the tx_brk_idle_done_int_raw interrupt."]
pub type TX_BRK_IDLE_DONE_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 13>;
#[doc = "Field `TX_DONE_INT_CLR` writer - Set this bit to clear the tx_done_int_raw interrupt."]
pub type TX_DONE_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 14>;
#[doc = "Field `RS485_PARITY_ERR_INT_CLR` writer - Set this bit to clear the rs485_parity_err_int_raw interrupt."]
pub type RS485_PARITY_ERR_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 15>;
#[doc = "Field `RS485_FRM_ERR_INT_CLR` writer - Set this bit to clear the rs485_frm_err_int_raw interrupt."]
pub type RS485_FRM_ERR_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 16>;
#[doc = "Field `RS485_CLASH_INT_CLR` writer - Set this bit to clear the rs485_clash_int_raw interrupt."]
pub type RS485_CLASH_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 17>;
#[doc = "Field `AT_CMD_CHAR_DET_INT_CLR` writer - Set this bit to clear the at_cmd_char_det_int_raw interrupt."]
pub type AT_CMD_CHAR_DET_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 18>;
#[doc = "Field `WAKEUP_INT_CLR` writer - Set this bit to clear the uart_wakeup_int_raw interrupt."]
pub type WAKEUP_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 19>;
impl W {
    #[doc = "Bit 0 - Set this bit to clear the rxfifo_full_int_raw interrupt."]
    #[inline(always)]
    pub fn rxfifo_full_int_clr(&mut self) -> RXFIFO_FULL_INT_CLR_W {
        RXFIFO_FULL_INT_CLR_W::new(self)
    }
    #[doc = "Bit 1 - Set this bit to clear txfifo_empty_int_raw interrupt."]
    #[inline(always)]
    pub fn txfifo_empty_int_clr(&mut self) -> TXFIFO_EMPTY_INT_CLR_W {
        TXFIFO_EMPTY_INT_CLR_W::new(self)
    }
    #[doc = "Bit 2 - Set this bit to clear parity_err_int_raw interrupt."]
    #[inline(always)]
    pub fn parity_err_int_clr(&mut self) -> PARITY_ERR_INT_CLR_W {
        PARITY_ERR_INT_CLR_W::new(self)
    }
    #[doc = "Bit 3 - Set this bit to clear frm_err_int_raw interrupt."]
    #[inline(always)]
    pub fn frm_err_int_clr(&mut self) -> FRM_ERR_INT_CLR_W {
        FRM_ERR_INT_CLR_W::new(self)
    }
    #[doc = "Bit 4 - Set this bit to clear rxfifo_ovf_int_raw interrupt."]
    #[inline(always)]
    pub fn rxfifo_ovf_int_clr(&mut self) -> RXFIFO_OVF_INT_CLR_W {
        RXFIFO_OVF_INT_CLR_W::new(self)
    }
    #[doc = "Bit 5 - Set this bit to clear the dsr_chg_int_raw interrupt."]
    #[inline(always)]
    pub fn dsr_chg_int_clr(&mut self) -> DSR_CHG_INT_CLR_W {
        DSR_CHG_INT_CLR_W::new(self)
    }
    #[doc = "Bit 6 - Set this bit to clear the cts_chg_int_raw interrupt."]
    #[inline(always)]
    pub fn cts_chg_int_clr(&mut self) -> CTS_CHG_INT_CLR_W {
        CTS_CHG_INT_CLR_W::new(self)
    }
    #[doc = "Bit 7 - Set this bit to clear the brk_det_int_raw interrupt."]
    #[inline(always)]
    pub fn brk_det_int_clr(&mut self) -> BRK_DET_INT_CLR_W {
        BRK_DET_INT_CLR_W::new(self)
    }
    #[doc = "Bit 8 - Set this bit to clear the rxfifo_tout_int_raw interrupt."]
    #[inline(always)]
    pub fn rxfifo_tout_int_clr(&mut self) -> RXFIFO_TOUT_INT_CLR_W {
        RXFIFO_TOUT_INT_CLR_W::new(self)
    }
    #[doc = "Bit 9 - Set this bit to clear the sw_xon_int_raw interrupt."]
    #[inline(always)]
    pub fn sw_xon_int_clr(&mut self) -> SW_XON_INT_CLR_W {
        SW_XON_INT_CLR_W::new(self)
    }
    #[doc = "Bit 10 - Set this bit to clear the sw_xoff_int_raw interrupt."]
    #[inline(always)]
    pub fn sw_xoff_int_clr(&mut self) -> SW_XOFF_INT_CLR_W {
        SW_XOFF_INT_CLR_W::new(self)
    }
    #[doc = "Bit 11 - Set this bit to clear the glitch_det_int_raw interrupt."]
    #[inline(always)]
    pub fn glitch_det_int_clr(&mut self) -> GLITCH_DET_INT_CLR_W {
        GLITCH_DET_INT_CLR_W::new(self)
    }
    #[doc = "Bit 12 - Set this bit to clear the tx_brk_done_int_raw interrupt.."]
    #[inline(always)]
    pub fn tx_brk_done_int_clr(&mut self) -> TX_BRK_DONE_INT_CLR_W {
        TX_BRK_DONE_INT_CLR_W::new(self)
    }
    #[doc = "Bit 13 - Set this bit to clear the tx_brk_idle_done_int_raw interrupt."]
    #[inline(always)]
    pub fn tx_brk_idle_done_int_clr(&mut self) -> TX_BRK_IDLE_DONE_INT_CLR_W {
        TX_BRK_IDLE_DONE_INT_CLR_W::new(self)
    }
    #[doc = "Bit 14 - Set this bit to clear the tx_done_int_raw interrupt."]
    #[inline(always)]
    pub fn tx_done_int_clr(&mut self) -> TX_DONE_INT_CLR_W {
        TX_DONE_INT_CLR_W::new(self)
    }
    #[doc = "Bit 15 - Set this bit to clear the rs485_parity_err_int_raw interrupt."]
    #[inline(always)]
    pub fn rs485_parity_err_int_clr(&mut self) -> RS485_PARITY_ERR_INT_CLR_W {
        RS485_PARITY_ERR_INT_CLR_W::new(self)
    }
    #[doc = "Bit 16 - Set this bit to clear the rs485_frm_err_int_raw interrupt."]
    #[inline(always)]
    pub fn rs485_frm_err_int_clr(&mut self) -> RS485_FRM_ERR_INT_CLR_W {
        RS485_FRM_ERR_INT_CLR_W::new(self)
    }
    #[doc = "Bit 17 - Set this bit to clear the rs485_clash_int_raw interrupt."]
    #[inline(always)]
    pub fn rs485_clash_int_clr(&mut self) -> RS485_CLASH_INT_CLR_W {
        RS485_CLASH_INT_CLR_W::new(self)
    }
    #[doc = "Bit 18 - Set this bit to clear the at_cmd_char_det_int_raw interrupt."]
    #[inline(always)]
    pub fn at_cmd_char_det_int_clr(&mut self) -> AT_CMD_CHAR_DET_INT_CLR_W {
        AT_CMD_CHAR_DET_INT_CLR_W::new(self)
    }
    #[doc = "Bit 19 - Set this bit to clear the uart_wakeup_int_raw interrupt."]
    #[inline(always)]
    pub fn wakeup_int_clr(&mut self) -> WAKEUP_INT_CLR_W {
        WAKEUP_INT_CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt clear bits\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_clr](index.html) module"]
pub struct INT_CLR_SPEC;
impl crate::RegisterSpec for INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [int_clr::W](W) writer structure"]
impl crate::Writable for INT_CLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INT_CLR to value 0"]
impl crate::Resettable for INT_CLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
