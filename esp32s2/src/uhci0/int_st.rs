#[doc = "Register `INT_ST` reader"]
pub type R = crate::R<INT_ST_SPEC>;
#[doc = "Field `RX_START` reader - This is the masked interrupt bit for UHCI_RX_START_INT interrupt when UHCI_RX_START_INT_ENA is set to 1."]
pub type RX_START_R = crate::BitReader;
#[doc = "Field `TX_START` reader - This is the masked interrupt bit for UHCI_TX_START_INT interrupt when UHCI_TX_START_INT_ENA is set to 1."]
pub type TX_START_R = crate::BitReader;
#[doc = "Field `RX_HUNG` reader - This is the masked interrupt bit for UHCI_RX_HUNG_INT interrupt when UHCI_RX_HUNG_INT_ENA is set to 1."]
pub type RX_HUNG_R = crate::BitReader;
#[doc = "Field `TX_HUNG` reader - This is the masked interrupt bit for UHCI_TX_HUNG_INT interrupt when UHCI_TX_HUNG_INT_ENA is set to 1."]
pub type TX_HUNG_R = crate::BitReader;
#[doc = "Field `IN_DONE` reader - This is the masked interrupt bit for UHCI_IN_DONE_INT interrupt when UHCI_IN_DONE_INT_ENA is set to 1."]
pub type IN_DONE_R = crate::BitReader;
#[doc = "Field `IN_SUC_EOF` reader - This is the masked interrupt bit for UHCI_IN_SUC_EOF_INT interrupt when UHCI_IN_SUC_EOF_INT_ENA is set to 1."]
pub type IN_SUC_EOF_R = crate::BitReader;
#[doc = "Field `IN_ERR_EOF` reader - This is the masked interrupt bit for UHCI_IN_ERR_EOF_INT interrupt when UHCI_IN_ERR_EOF_INT_ENA is set to 1."]
pub type IN_ERR_EOF_R = crate::BitReader;
#[doc = "Field `OUT_DONE` reader - This is the masked interrupt bit for UHCI_OUT_DONE_INT interrupt when UHCI_OUT_DONE_INT_ENA is set to 1."]
pub type OUT_DONE_R = crate::BitReader;
#[doc = "Field `OUT_EOF` reader - This is the masked interrupt bit for UHCI_OUT_EOF_INT interrupt when UHCI_OUT_EOF_INT_ENA is set to 1."]
pub type OUT_EOF_R = crate::BitReader;
#[doc = "Field `IN_DSCR_ERR` reader - This is the masked interrupt bit for UHCI_IN_DSCR_ERR_INT interrupt when UHCI_IN_DSCR_ERR_INT is set to 1."]
pub type IN_DSCR_ERR_R = crate::BitReader;
#[doc = "Field `OUT_DSCR_ERR` reader - This is the masked interrupt bit for UHCI_OUT_DSCR_ERR_INT interrupt when UHCI_OUT_DSCR_ERR_INT_ENA is set to 1."]
pub type OUT_DSCR_ERR_R = crate::BitReader;
#[doc = "Field `IN_DSCR_EMPTY` reader - This is the masked interrupt bit for UHCI_IN_DSCR_EMPTY_INT interrupt when UHCI_IN_DSCR_EMPTY_INT_ENA is set to 1."]
pub type IN_DSCR_EMPTY_R = crate::BitReader;
#[doc = "Field `OUTLINK_EOF_ERR` reader - This is the masked interrupt bit for UHCI_OUTLINK_EOF_ERR_INT interrupt when UHCI_OUTLINK_EOF_ERR_INT_ENA is set to 1."]
pub type OUTLINK_EOF_ERR_R = crate::BitReader;
#[doc = "Field `OUT_TOTAL_EOF` reader - This is the masked interrupt bit for UHCI_OUT_TOTAL_EOF_INT interrupt when UHCI_OUT_TOTAL_EOF_INT_ENA is set to 1."]
pub type OUT_TOTAL_EOF_R = crate::BitReader;
#[doc = "Field `SEND_S_REG_Q` reader - This is the masked interrupt bit for UHCI_SEND_S_REG_Q_INT interrupt when UHCI_SEND_S_REG_Q_INT_ENA is set to 1."]
pub type SEND_S_REG_Q_R = crate::BitReader;
#[doc = "Field `SEND_A_REG_Q` reader - This is the masked interrupt bit for UHCI_SEND_A_REG_Q_INT interrupt when UHCI_SEND_A_REG_Q_INT_ENA is set to 1."]
pub type SEND_A_REG_Q_R = crate::BitReader;
#[doc = "Field `DMA_INFIFO_FULL_WM` reader - This is the masked interrupt bit for UHCI_DMA_INFIFO_FULL_WM_INT INTERRUPT when UHCI_DMA_INFIFO_FULL_WM_INT_ENA is set to 1."]
pub type DMA_INFIFO_FULL_WM_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - This is the masked interrupt bit for UHCI_RX_START_INT interrupt when UHCI_RX_START_INT_ENA is set to 1."]
    #[inline(always)]
    pub fn rx_start(&self) -> RX_START_R {
        RX_START_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This is the masked interrupt bit for UHCI_TX_START_INT interrupt when UHCI_TX_START_INT_ENA is set to 1."]
    #[inline(always)]
    pub fn tx_start(&self) -> TX_START_R {
        TX_START_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This is the masked interrupt bit for UHCI_RX_HUNG_INT interrupt when UHCI_RX_HUNG_INT_ENA is set to 1."]
    #[inline(always)]
    pub fn rx_hung(&self) -> RX_HUNG_R {
        RX_HUNG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This is the masked interrupt bit for UHCI_TX_HUNG_INT interrupt when UHCI_TX_HUNG_INT_ENA is set to 1."]
    #[inline(always)]
    pub fn tx_hung(&self) -> TX_HUNG_R {
        TX_HUNG_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This is the masked interrupt bit for UHCI_IN_DONE_INT interrupt when UHCI_IN_DONE_INT_ENA is set to 1."]
    #[inline(always)]
    pub fn in_done(&self) -> IN_DONE_R {
        IN_DONE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This is the masked interrupt bit for UHCI_IN_SUC_EOF_INT interrupt when UHCI_IN_SUC_EOF_INT_ENA is set to 1."]
    #[inline(always)]
    pub fn in_suc_eof(&self) -> IN_SUC_EOF_R {
        IN_SUC_EOF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - This is the masked interrupt bit for UHCI_IN_ERR_EOF_INT interrupt when UHCI_IN_ERR_EOF_INT_ENA is set to 1."]
    #[inline(always)]
    pub fn in_err_eof(&self) -> IN_ERR_EOF_R {
        IN_ERR_EOF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - This is the masked interrupt bit for UHCI_OUT_DONE_INT interrupt when UHCI_OUT_DONE_INT_ENA is set to 1."]
    #[inline(always)]
    pub fn out_done(&self) -> OUT_DONE_R {
        OUT_DONE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - This is the masked interrupt bit for UHCI_OUT_EOF_INT interrupt when UHCI_OUT_EOF_INT_ENA is set to 1."]
    #[inline(always)]
    pub fn out_eof(&self) -> OUT_EOF_R {
        OUT_EOF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - This is the masked interrupt bit for UHCI_IN_DSCR_ERR_INT interrupt when UHCI_IN_DSCR_ERR_INT is set to 1."]
    #[inline(always)]
    pub fn in_dscr_err(&self) -> IN_DSCR_ERR_R {
        IN_DSCR_ERR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - This is the masked interrupt bit for UHCI_OUT_DSCR_ERR_INT interrupt when UHCI_OUT_DSCR_ERR_INT_ENA is set to 1."]
    #[inline(always)]
    pub fn out_dscr_err(&self) -> OUT_DSCR_ERR_R {
        OUT_DSCR_ERR_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - This is the masked interrupt bit for UHCI_IN_DSCR_EMPTY_INT interrupt when UHCI_IN_DSCR_EMPTY_INT_ENA is set to 1."]
    #[inline(always)]
    pub fn in_dscr_empty(&self) -> IN_DSCR_EMPTY_R {
        IN_DSCR_EMPTY_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - This is the masked interrupt bit for UHCI_OUTLINK_EOF_ERR_INT interrupt when UHCI_OUTLINK_EOF_ERR_INT_ENA is set to 1."]
    #[inline(always)]
    pub fn outlink_eof_err(&self) -> OUTLINK_EOF_ERR_R {
        OUTLINK_EOF_ERR_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - This is the masked interrupt bit for UHCI_OUT_TOTAL_EOF_INT interrupt when UHCI_OUT_TOTAL_EOF_INT_ENA is set to 1."]
    #[inline(always)]
    pub fn out_total_eof(&self) -> OUT_TOTAL_EOF_R {
        OUT_TOTAL_EOF_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - This is the masked interrupt bit for UHCI_SEND_S_REG_Q_INT interrupt when UHCI_SEND_S_REG_Q_INT_ENA is set to 1."]
    #[inline(always)]
    pub fn send_s_reg_q(&self) -> SEND_S_REG_Q_R {
        SEND_S_REG_Q_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - This is the masked interrupt bit for UHCI_SEND_A_REG_Q_INT interrupt when UHCI_SEND_A_REG_Q_INT_ENA is set to 1."]
    #[inline(always)]
    pub fn send_a_reg_q(&self) -> SEND_A_REG_Q_R {
        SEND_A_REG_Q_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - This is the masked interrupt bit for UHCI_DMA_INFIFO_FULL_WM_INT INTERRUPT when UHCI_DMA_INFIFO_FULL_WM_INT_ENA is set to 1."]
    #[inline(always)]
    pub fn dma_infifo_full_wm(&self) -> DMA_INFIFO_FULL_WM_R {
        DMA_INFIFO_FULL_WM_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ST")
            .field("rx_start", &self.rx_start().bit())
            .field("tx_start", &self.tx_start().bit())
            .field("rx_hung", &self.rx_hung().bit())
            .field("tx_hung", &self.tx_hung().bit())
            .field("in_done", &self.in_done().bit())
            .field("in_suc_eof", &self.in_suc_eof().bit())
            .field("in_err_eof", &self.in_err_eof().bit())
            .field("out_done", &self.out_done().bit())
            .field("out_eof", &self.out_eof().bit())
            .field("in_dscr_err", &self.in_dscr_err().bit())
            .field("out_dscr_err", &self.out_dscr_err().bit())
            .field("in_dscr_empty", &self.in_dscr_empty().bit())
            .field("outlink_eof_err", &self.outlink_eof_err().bit())
            .field("out_total_eof", &self.out_total_eof().bit())
            .field("send_s_reg_q", &self.send_s_reg_q().bit())
            .field("send_a_reg_q", &self.send_a_reg_q().bit())
            .field("dma_infifo_full_wm", &self.dma_infifo_full_wm().bit())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_ST_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Masked interrupt status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_st::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ST_SPEC;
impl crate::RegisterSpec for INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_st::R`](R) reader structure"]
impl crate::Readable for INT_ST_SPEC {}
#[doc = "`reset()` method sets INT_ST to value 0"]
impl crate::Resettable for INT_ST_SPEC {
    const RESET_VALUE: u32 = 0;
}
