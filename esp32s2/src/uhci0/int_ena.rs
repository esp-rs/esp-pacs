#[doc = "Register `INT_ENA` reader"]
pub type R = crate::R<INT_ENA_SPEC>;
#[doc = "Register `INT_ENA` writer"]
pub type W = crate::W<INT_ENA_SPEC>;
#[doc = "Field `RX_START_INT_ENA` reader - This is the interrupt enable bit for UHCI_RX_START_INT interrupt."]
pub type RX_START_INT_ENA_R = crate::BitReader;
#[doc = "Field `RX_START_INT_ENA` writer - This is the interrupt enable bit for UHCI_RX_START_INT interrupt."]
pub type RX_START_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_START_INT_ENA` reader - This is the interrupt enable bit for UHCI_TX_START_INT interrupt."]
pub type TX_START_INT_ENA_R = crate::BitReader;
#[doc = "Field `TX_START_INT_ENA` writer - This is the interrupt enable bit for UHCI_TX_START_INT interrupt."]
pub type TX_START_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_HUNG_INT_ENA` reader - This is the interrupt enable bit for UHCI_RX_HUNG_INT interrupt."]
pub type RX_HUNG_INT_ENA_R = crate::BitReader;
#[doc = "Field `RX_HUNG_INT_ENA` writer - This is the interrupt enable bit for UHCI_RX_HUNG_INT interrupt."]
pub type RX_HUNG_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_HUNG_INT_ENA` reader - This is the interrupt enable bit for UHCI_TX_HUNG_INT interrupt."]
pub type TX_HUNG_INT_ENA_R = crate::BitReader;
#[doc = "Field `TX_HUNG_INT_ENA` writer - This is the interrupt enable bit for UHCI_TX_HUNG_INT interrupt."]
pub type TX_HUNG_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_DONE_INT_ENA` reader - This is the interrupt enable bit for UHCI_IN_DONE_INT interrupt."]
pub type IN_DONE_INT_ENA_R = crate::BitReader;
#[doc = "Field `IN_DONE_INT_ENA` writer - This is the interrupt enable bit for UHCI_IN_DONE_INT interrupt."]
pub type IN_DONE_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_SUC_EOF_INT_ENA` reader - This is the interrupt enable bit for UHCI_IN_SUC_EOF_INT interrupt."]
pub type IN_SUC_EOF_INT_ENA_R = crate::BitReader;
#[doc = "Field `IN_SUC_EOF_INT_ENA` writer - This is the interrupt enable bit for UHCI_IN_SUC_EOF_INT interrupt."]
pub type IN_SUC_EOF_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_ERR_EOF_INT_ENA` reader - This is the interrupt enable bit for UHCI_IN_ERR_EOF_INT interrupt."]
pub type IN_ERR_EOF_INT_ENA_R = crate::BitReader;
#[doc = "Field `IN_ERR_EOF_INT_ENA` writer - This is the interrupt enable bit for UHCI_IN_ERR_EOF_INT interrupt."]
pub type IN_ERR_EOF_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_DONE_INT_ENA` reader - This is the interrupt enable bit for UHCI_OUT_DONE_INT interrupt."]
pub type OUT_DONE_INT_ENA_R = crate::BitReader;
#[doc = "Field `OUT_DONE_INT_ENA` writer - This is the interrupt enable bit for UHCI_OUT_DONE_INT interrupt."]
pub type OUT_DONE_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_EOF_INT_ENA` reader - This is the interrupt enable bit for UHCI_OUT_EOF_INT interrupt."]
pub type OUT_EOF_INT_ENA_R = crate::BitReader;
#[doc = "Field `OUT_EOF_INT_ENA` writer - This is the interrupt enable bit for UHCI_OUT_EOF_INT interrupt."]
pub type OUT_EOF_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_DSCR_ERR_INT_ENA` reader - This is the interrupt enable bit for UHCI_IN_DSCR_ERR_INT interrupt."]
pub type IN_DSCR_ERR_INT_ENA_R = crate::BitReader;
#[doc = "Field `IN_DSCR_ERR_INT_ENA` writer - This is the interrupt enable bit for UHCI_IN_DSCR_ERR_INT interrupt."]
pub type IN_DSCR_ERR_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_DSCR_ERR_INT_ENA` reader - This is the interrupt enable bit for UHCI_OUT_DSCR_ERR_INT interrupt."]
pub type OUT_DSCR_ERR_INT_ENA_R = crate::BitReader;
#[doc = "Field `OUT_DSCR_ERR_INT_ENA` writer - This is the interrupt enable bit for UHCI_OUT_DSCR_ERR_INT interrupt."]
pub type OUT_DSCR_ERR_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_DSCR_EMPTY_INT_ENA` reader - This is the interrupt enable bit for UHCI_IN_DSCR_EMPTY_INT interrupt."]
pub type IN_DSCR_EMPTY_INT_ENA_R = crate::BitReader;
#[doc = "Field `IN_DSCR_EMPTY_INT_ENA` writer - This is the interrupt enable bit for UHCI_IN_DSCR_EMPTY_INT interrupt."]
pub type IN_DSCR_EMPTY_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTLINK_EOF_ERR_INT_ENA` reader - This is the interrupt enable bit for UHCI_OUTLINK_EOF_ERR_INT interrupt."]
pub type OUTLINK_EOF_ERR_INT_ENA_R = crate::BitReader;
#[doc = "Field `OUTLINK_EOF_ERR_INT_ENA` writer - This is the interrupt enable bit for UHCI_OUTLINK_EOF_ERR_INT interrupt."]
pub type OUTLINK_EOF_ERR_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_TOTAL_EOF_INT_ENA` reader - This is the interrupt enable bit for UHCI_OUT_TOTAL_EOF_INT interrupt."]
pub type OUT_TOTAL_EOF_INT_ENA_R = crate::BitReader;
#[doc = "Field `OUT_TOTAL_EOF_INT_ENA` writer - This is the interrupt enable bit for UHCI_OUT_TOTAL_EOF_INT interrupt."]
pub type OUT_TOTAL_EOF_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEND_S_REG_Q_INT_ENA` reader - This is the interrupt enable bit for UHCI_SEND_S_REG_Q_INT interrupt."]
pub type SEND_S_REG_Q_INT_ENA_R = crate::BitReader;
#[doc = "Field `SEND_S_REG_Q_INT_ENA` writer - This is the interrupt enable bit for UHCI_SEND_S_REG_Q_INT interrupt."]
pub type SEND_S_REG_Q_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEND_A_REG_Q_INT_ENA` reader - This is the interrupt enable bit for UHCI_SEND_A_REG_Q_INT interrupt."]
pub type SEND_A_REG_Q_INT_ENA_R = crate::BitReader;
#[doc = "Field `SEND_A_REG_Q_INT_ENA` writer - This is the interrupt enable bit for UHCI_SEND_A_REG_Q_INT interrupt."]
pub type SEND_A_REG_Q_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_INFIFO_FULL_WM_INT_ENA` reader - This is the interrupt enable bit for UHCI_DMA_INFIFO_FULL_WM_INT interrupt."]
pub type DMA_INFIFO_FULL_WM_INT_ENA_R = crate::BitReader;
#[doc = "Field `DMA_INFIFO_FULL_WM_INT_ENA` writer - This is the interrupt enable bit for UHCI_DMA_INFIFO_FULL_WM_INT interrupt."]
pub type DMA_INFIFO_FULL_WM_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - This is the interrupt enable bit for UHCI_RX_START_INT interrupt."]
    #[inline(always)]
    pub fn rx_start_int_ena(&self) -> RX_START_INT_ENA_R {
        RX_START_INT_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This is the interrupt enable bit for UHCI_TX_START_INT interrupt."]
    #[inline(always)]
    pub fn tx_start_int_ena(&self) -> TX_START_INT_ENA_R {
        TX_START_INT_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This is the interrupt enable bit for UHCI_RX_HUNG_INT interrupt."]
    #[inline(always)]
    pub fn rx_hung_int_ena(&self) -> RX_HUNG_INT_ENA_R {
        RX_HUNG_INT_ENA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This is the interrupt enable bit for UHCI_TX_HUNG_INT interrupt."]
    #[inline(always)]
    pub fn tx_hung_int_ena(&self) -> TX_HUNG_INT_ENA_R {
        TX_HUNG_INT_ENA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This is the interrupt enable bit for UHCI_IN_DONE_INT interrupt."]
    #[inline(always)]
    pub fn in_done_int_ena(&self) -> IN_DONE_INT_ENA_R {
        IN_DONE_INT_ENA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This is the interrupt enable bit for UHCI_IN_SUC_EOF_INT interrupt."]
    #[inline(always)]
    pub fn in_suc_eof_int_ena(&self) -> IN_SUC_EOF_INT_ENA_R {
        IN_SUC_EOF_INT_ENA_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - This is the interrupt enable bit for UHCI_IN_ERR_EOF_INT interrupt."]
    #[inline(always)]
    pub fn in_err_eof_int_ena(&self) -> IN_ERR_EOF_INT_ENA_R {
        IN_ERR_EOF_INT_ENA_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - This is the interrupt enable bit for UHCI_OUT_DONE_INT interrupt."]
    #[inline(always)]
    pub fn out_done_int_ena(&self) -> OUT_DONE_INT_ENA_R {
        OUT_DONE_INT_ENA_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - This is the interrupt enable bit for UHCI_OUT_EOF_INT interrupt."]
    #[inline(always)]
    pub fn out_eof_int_ena(&self) -> OUT_EOF_INT_ENA_R {
        OUT_EOF_INT_ENA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - This is the interrupt enable bit for UHCI_IN_DSCR_ERR_INT interrupt."]
    #[inline(always)]
    pub fn in_dscr_err_int_ena(&self) -> IN_DSCR_ERR_INT_ENA_R {
        IN_DSCR_ERR_INT_ENA_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - This is the interrupt enable bit for UHCI_OUT_DSCR_ERR_INT interrupt."]
    #[inline(always)]
    pub fn out_dscr_err_int_ena(&self) -> OUT_DSCR_ERR_INT_ENA_R {
        OUT_DSCR_ERR_INT_ENA_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - This is the interrupt enable bit for UHCI_IN_DSCR_EMPTY_INT interrupt."]
    #[inline(always)]
    pub fn in_dscr_empty_int_ena(&self) -> IN_DSCR_EMPTY_INT_ENA_R {
        IN_DSCR_EMPTY_INT_ENA_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - This is the interrupt enable bit for UHCI_OUTLINK_EOF_ERR_INT interrupt."]
    #[inline(always)]
    pub fn outlink_eof_err_int_ena(&self) -> OUTLINK_EOF_ERR_INT_ENA_R {
        OUTLINK_EOF_ERR_INT_ENA_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - This is the interrupt enable bit for UHCI_OUT_TOTAL_EOF_INT interrupt."]
    #[inline(always)]
    pub fn out_total_eof_int_ena(&self) -> OUT_TOTAL_EOF_INT_ENA_R {
        OUT_TOTAL_EOF_INT_ENA_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - This is the interrupt enable bit for UHCI_SEND_S_REG_Q_INT interrupt."]
    #[inline(always)]
    pub fn send_s_reg_q_int_ena(&self) -> SEND_S_REG_Q_INT_ENA_R {
        SEND_S_REG_Q_INT_ENA_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - This is the interrupt enable bit for UHCI_SEND_A_REG_Q_INT interrupt."]
    #[inline(always)]
    pub fn send_a_reg_q_int_ena(&self) -> SEND_A_REG_Q_INT_ENA_R {
        SEND_A_REG_Q_INT_ENA_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - This is the interrupt enable bit for UHCI_DMA_INFIFO_FULL_WM_INT interrupt."]
    #[inline(always)]
    pub fn dma_infifo_full_wm_int_ena(&self) -> DMA_INFIFO_FULL_WM_INT_ENA_R {
        DMA_INFIFO_FULL_WM_INT_ENA_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ENA")
            .field(
                "rx_start_int_ena",
                &format_args!("{}", self.rx_start_int_ena().bit()),
            )
            .field(
                "tx_start_int_ena",
                &format_args!("{}", self.tx_start_int_ena().bit()),
            )
            .field(
                "rx_hung_int_ena",
                &format_args!("{}", self.rx_hung_int_ena().bit()),
            )
            .field(
                "tx_hung_int_ena",
                &format_args!("{}", self.tx_hung_int_ena().bit()),
            )
            .field(
                "in_done_int_ena",
                &format_args!("{}", self.in_done_int_ena().bit()),
            )
            .field(
                "in_suc_eof_int_ena",
                &format_args!("{}", self.in_suc_eof_int_ena().bit()),
            )
            .field(
                "in_err_eof_int_ena",
                &format_args!("{}", self.in_err_eof_int_ena().bit()),
            )
            .field(
                "out_done_int_ena",
                &format_args!("{}", self.out_done_int_ena().bit()),
            )
            .field(
                "out_eof_int_ena",
                &format_args!("{}", self.out_eof_int_ena().bit()),
            )
            .field(
                "in_dscr_err_int_ena",
                &format_args!("{}", self.in_dscr_err_int_ena().bit()),
            )
            .field(
                "out_dscr_err_int_ena",
                &format_args!("{}", self.out_dscr_err_int_ena().bit()),
            )
            .field(
                "in_dscr_empty_int_ena",
                &format_args!("{}", self.in_dscr_empty_int_ena().bit()),
            )
            .field(
                "outlink_eof_err_int_ena",
                &format_args!("{}", self.outlink_eof_err_int_ena().bit()),
            )
            .field(
                "out_total_eof_int_ena",
                &format_args!("{}", self.out_total_eof_int_ena().bit()),
            )
            .field(
                "send_s_reg_q_int_ena",
                &format_args!("{}", self.send_s_reg_q_int_ena().bit()),
            )
            .field(
                "send_a_reg_q_int_ena",
                &format_args!("{}", self.send_a_reg_q_int_ena().bit()),
            )
            .field(
                "dma_infifo_full_wm_int_ena",
                &format_args!("{}", self.dma_infifo_full_wm_int_ena().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_ENA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - This is the interrupt enable bit for UHCI_RX_START_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn rx_start_int_ena(&mut self) -> RX_START_INT_ENA_W<INT_ENA_SPEC> {
        RX_START_INT_ENA_W::new(self, 0)
    }
    #[doc = "Bit 1 - This is the interrupt enable bit for UHCI_TX_START_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn tx_start_int_ena(&mut self) -> TX_START_INT_ENA_W<INT_ENA_SPEC> {
        TX_START_INT_ENA_W::new(self, 1)
    }
    #[doc = "Bit 2 - This is the interrupt enable bit for UHCI_RX_HUNG_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn rx_hung_int_ena(&mut self) -> RX_HUNG_INT_ENA_W<INT_ENA_SPEC> {
        RX_HUNG_INT_ENA_W::new(self, 2)
    }
    #[doc = "Bit 3 - This is the interrupt enable bit for UHCI_TX_HUNG_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn tx_hung_int_ena(&mut self) -> TX_HUNG_INT_ENA_W<INT_ENA_SPEC> {
        TX_HUNG_INT_ENA_W::new(self, 3)
    }
    #[doc = "Bit 4 - This is the interrupt enable bit for UHCI_IN_DONE_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn in_done_int_ena(&mut self) -> IN_DONE_INT_ENA_W<INT_ENA_SPEC> {
        IN_DONE_INT_ENA_W::new(self, 4)
    }
    #[doc = "Bit 5 - This is the interrupt enable bit for UHCI_IN_SUC_EOF_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn in_suc_eof_int_ena(&mut self) -> IN_SUC_EOF_INT_ENA_W<INT_ENA_SPEC> {
        IN_SUC_EOF_INT_ENA_W::new(self, 5)
    }
    #[doc = "Bit 6 - This is the interrupt enable bit for UHCI_IN_ERR_EOF_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn in_err_eof_int_ena(&mut self) -> IN_ERR_EOF_INT_ENA_W<INT_ENA_SPEC> {
        IN_ERR_EOF_INT_ENA_W::new(self, 6)
    }
    #[doc = "Bit 7 - This is the interrupt enable bit for UHCI_OUT_DONE_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn out_done_int_ena(&mut self) -> OUT_DONE_INT_ENA_W<INT_ENA_SPEC> {
        OUT_DONE_INT_ENA_W::new(self, 7)
    }
    #[doc = "Bit 8 - This is the interrupt enable bit for UHCI_OUT_EOF_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn out_eof_int_ena(&mut self) -> OUT_EOF_INT_ENA_W<INT_ENA_SPEC> {
        OUT_EOF_INT_ENA_W::new(self, 8)
    }
    #[doc = "Bit 9 - This is the interrupt enable bit for UHCI_IN_DSCR_ERR_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn in_dscr_err_int_ena(&mut self) -> IN_DSCR_ERR_INT_ENA_W<INT_ENA_SPEC> {
        IN_DSCR_ERR_INT_ENA_W::new(self, 9)
    }
    #[doc = "Bit 10 - This is the interrupt enable bit for UHCI_OUT_DSCR_ERR_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn out_dscr_err_int_ena(&mut self) -> OUT_DSCR_ERR_INT_ENA_W<INT_ENA_SPEC> {
        OUT_DSCR_ERR_INT_ENA_W::new(self, 10)
    }
    #[doc = "Bit 11 - This is the interrupt enable bit for UHCI_IN_DSCR_EMPTY_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn in_dscr_empty_int_ena(&mut self) -> IN_DSCR_EMPTY_INT_ENA_W<INT_ENA_SPEC> {
        IN_DSCR_EMPTY_INT_ENA_W::new(self, 11)
    }
    #[doc = "Bit 12 - This is the interrupt enable bit for UHCI_OUTLINK_EOF_ERR_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn outlink_eof_err_int_ena(&mut self) -> OUTLINK_EOF_ERR_INT_ENA_W<INT_ENA_SPEC> {
        OUTLINK_EOF_ERR_INT_ENA_W::new(self, 12)
    }
    #[doc = "Bit 13 - This is the interrupt enable bit for UHCI_OUT_TOTAL_EOF_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn out_total_eof_int_ena(&mut self) -> OUT_TOTAL_EOF_INT_ENA_W<INT_ENA_SPEC> {
        OUT_TOTAL_EOF_INT_ENA_W::new(self, 13)
    }
    #[doc = "Bit 14 - This is the interrupt enable bit for UHCI_SEND_S_REG_Q_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn send_s_reg_q_int_ena(&mut self) -> SEND_S_REG_Q_INT_ENA_W<INT_ENA_SPEC> {
        SEND_S_REG_Q_INT_ENA_W::new(self, 14)
    }
    #[doc = "Bit 15 - This is the interrupt enable bit for UHCI_SEND_A_REG_Q_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn send_a_reg_q_int_ena(&mut self) -> SEND_A_REG_Q_INT_ENA_W<INT_ENA_SPEC> {
        SEND_A_REG_Q_INT_ENA_W::new(self, 15)
    }
    #[doc = "Bit 16 - This is the interrupt enable bit for UHCI_DMA_INFIFO_FULL_WM_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn dma_infifo_full_wm_int_ena(&mut self) -> DMA_INFIFO_FULL_WM_INT_ENA_W<INT_ENA_SPEC> {
        DMA_INFIFO_FULL_WM_INT_ENA_W::new(self, 16)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Interrupt enable bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ENA_SPEC;
impl crate::RegisterSpec for INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_ena::R`](R) reader structure"]
impl crate::Readable for INT_ENA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_ena::W`](W) writer structure"]
impl crate::Writable for INT_ENA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_ENA to value 0"]
impl crate::Resettable for INT_ENA_SPEC {
    const RESET_VALUE: u32 = 0;
}
