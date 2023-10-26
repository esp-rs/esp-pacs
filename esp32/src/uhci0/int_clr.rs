#[doc = "Register `INT_CLR` writer"]
pub type W = crate::W<INT_CLR_SPEC>;
#[doc = "Field `RX_START_INT_CLR` writer - "]
pub type RX_START_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TX_START_INT_CLR` writer - "]
pub type TX_START_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RX_HUNG_INT_CLR` writer - "]
pub type RX_HUNG_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TX_HUNG_INT_CLR` writer - "]
pub type TX_HUNG_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IN_DONE_INT_CLR` writer - "]
pub type IN_DONE_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IN_SUC_EOF_INT_CLR` writer - "]
pub type IN_SUC_EOF_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IN_ERR_EOF_INT_CLR` writer - "]
pub type IN_ERR_EOF_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OUT_DONE_INT_CLR` writer - "]
pub type OUT_DONE_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OUT_EOF_INT_CLR` writer - "]
pub type OUT_EOF_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IN_DSCR_ERR_INT_CLR` writer - "]
pub type IN_DSCR_ERR_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OUT_DSCR_ERR_INT_CLR` writer - "]
pub type OUT_DSCR_ERR_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IN_DSCR_EMPTY_INT_CLR` writer - "]
pub type IN_DSCR_EMPTY_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OUTLINK_EOF_ERR_INT_CLR` writer - "]
pub type OUTLINK_EOF_ERR_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OUT_TOTAL_EOF_INT_CLR` writer - "]
pub type OUT_TOTAL_EOF_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SEND_S_Q_INT_CLR` writer - "]
pub type SEND_S_Q_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SEND_A_Q_INT_CLR` writer - "]
pub type SEND_A_Q_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DMA_INFIFO_FULL_WM_INT_CLR` writer - "]
pub type DMA_INFIFO_FULL_WM_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn rx_start_int_clr(&mut self) -> RX_START_INT_CLR_W<INT_CLR_SPEC, 0> {
        RX_START_INT_CLR_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn tx_start_int_clr(&mut self) -> TX_START_INT_CLR_W<INT_CLR_SPEC, 1> {
        TX_START_INT_CLR_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn rx_hung_int_clr(&mut self) -> RX_HUNG_INT_CLR_W<INT_CLR_SPEC, 2> {
        RX_HUNG_INT_CLR_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn tx_hung_int_clr(&mut self) -> TX_HUNG_INT_CLR_W<INT_CLR_SPEC, 3> {
        TX_HUNG_INT_CLR_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn in_done_int_clr(&mut self) -> IN_DONE_INT_CLR_W<INT_CLR_SPEC, 4> {
        IN_DONE_INT_CLR_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn in_suc_eof_int_clr(&mut self) -> IN_SUC_EOF_INT_CLR_W<INT_CLR_SPEC, 5> {
        IN_SUC_EOF_INT_CLR_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn in_err_eof_int_clr(&mut self) -> IN_ERR_EOF_INT_CLR_W<INT_CLR_SPEC, 6> {
        IN_ERR_EOF_INT_CLR_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn out_done_int_clr(&mut self) -> OUT_DONE_INT_CLR_W<INT_CLR_SPEC, 7> {
        OUT_DONE_INT_CLR_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn out_eof_int_clr(&mut self) -> OUT_EOF_INT_CLR_W<INT_CLR_SPEC, 8> {
        OUT_EOF_INT_CLR_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn in_dscr_err_int_clr(&mut self) -> IN_DSCR_ERR_INT_CLR_W<INT_CLR_SPEC, 9> {
        IN_DSCR_ERR_INT_CLR_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn out_dscr_err_int_clr(&mut self) -> OUT_DSCR_ERR_INT_CLR_W<INT_CLR_SPEC, 10> {
        OUT_DSCR_ERR_INT_CLR_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn in_dscr_empty_int_clr(&mut self) -> IN_DSCR_EMPTY_INT_CLR_W<INT_CLR_SPEC, 11> {
        IN_DSCR_EMPTY_INT_CLR_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn outlink_eof_err_int_clr(&mut self) -> OUTLINK_EOF_ERR_INT_CLR_W<INT_CLR_SPEC, 12> {
        OUTLINK_EOF_ERR_INT_CLR_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn out_total_eof_int_clr(&mut self) -> OUT_TOTAL_EOF_INT_CLR_W<INT_CLR_SPEC, 13> {
        OUT_TOTAL_EOF_INT_CLR_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn send_s_q_int_clr(&mut self) -> SEND_S_Q_INT_CLR_W<INT_CLR_SPEC, 14> {
        SEND_S_Q_INT_CLR_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn send_a_q_int_clr(&mut self) -> SEND_A_Q_INT_CLR_W<INT_CLR_SPEC, 15> {
        SEND_A_Q_INT_CLR_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn dma_infifo_full_wm_int_clr(&mut self) -> DMA_INFIFO_FULL_WM_INT_CLR_W<INT_CLR_SPEC, 16> {
        DMA_INFIFO_FULL_WM_INT_CLR_W::new(self)
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
#[doc = "\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_CLR_SPEC;
impl crate::RegisterSpec for INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_clr::W`](W) writer structure"]
impl crate::Writable for INT_CLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INT_CLR to value 0"]
impl crate::Resettable for INT_CLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
