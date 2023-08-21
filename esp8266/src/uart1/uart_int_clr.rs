#[doc = "Register `UART_INT_CLR` writer"]
pub type W = crate::W<UART_INT_CLR_SPEC>;
#[doc = "Field `rxfifo_full_int_clr` writer - Set this bit to clear the rx fifo full interrupt"]
pub type RXFIFO_FULL_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `txfifo_empty_int_clr` writer - Set this bit to clear the tx fifo empty interrupt"]
pub type TXFIFO_EMPTY_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `parity_err_int_clr` writer - Set this bit to clear the parity error interrupt"]
pub type PARITY_ERR_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `frm_err_int_clr` writer - Set this bit to clear other rx error interrupt"]
pub type FRM_ERR_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `rxfifo_ovf_int_clr` writer - Set this bit to clear the rx fifo over-flow interrupt"]
pub type RXFIFO_OVF_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `dsr_chg_int_clr` writer - Set this bit to clear the DSR changing interrupt"]
pub type DSR_CHG_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `cts_chg_int_clr` writer - Set this bit to clear the CTS changing interrupt"]
pub type CTS_CHG_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `brk_det_int_clr` writer - Set this bit to clear the rx byte start interrupt"]
pub type BRK_DET_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `rxfifo_tout_int_clr` writer - Set this bit to clear the rx time-out interrupt"]
pub type RXFIFO_TOUT_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<UART_INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to clear the rx fifo full interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn rxfifo_full_int_clr(&mut self) -> RXFIFO_FULL_INT_CLR_W<UART_INT_CLR_SPEC, 0> {
        RXFIFO_FULL_INT_CLR_W::new(self)
    }
    #[doc = "Bit 1 - Set this bit to clear the tx fifo empty interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn txfifo_empty_int_clr(&mut self) -> TXFIFO_EMPTY_INT_CLR_W<UART_INT_CLR_SPEC, 1> {
        TXFIFO_EMPTY_INT_CLR_W::new(self)
    }
    #[doc = "Bit 2 - Set this bit to clear the parity error interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn parity_err_int_clr(&mut self) -> PARITY_ERR_INT_CLR_W<UART_INT_CLR_SPEC, 2> {
        PARITY_ERR_INT_CLR_W::new(self)
    }
    #[doc = "Bit 3 - Set this bit to clear other rx error interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn frm_err_int_clr(&mut self) -> FRM_ERR_INT_CLR_W<UART_INT_CLR_SPEC, 3> {
        FRM_ERR_INT_CLR_W::new(self)
    }
    #[doc = "Bit 4 - Set this bit to clear the rx fifo over-flow interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn rxfifo_ovf_int_clr(&mut self) -> RXFIFO_OVF_INT_CLR_W<UART_INT_CLR_SPEC, 4> {
        RXFIFO_OVF_INT_CLR_W::new(self)
    }
    #[doc = "Bit 5 - Set this bit to clear the DSR changing interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn dsr_chg_int_clr(&mut self) -> DSR_CHG_INT_CLR_W<UART_INT_CLR_SPEC, 5> {
        DSR_CHG_INT_CLR_W::new(self)
    }
    #[doc = "Bit 6 - Set this bit to clear the CTS changing interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn cts_chg_int_clr(&mut self) -> CTS_CHG_INT_CLR_W<UART_INT_CLR_SPEC, 6> {
        CTS_CHG_INT_CLR_W::new(self)
    }
    #[doc = "Bit 7 - Set this bit to clear the rx byte start interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn brk_det_int_clr(&mut self) -> BRK_DET_INT_CLR_W<UART_INT_CLR_SPEC, 7> {
        BRK_DET_INT_CLR_W::new(self)
    }
    #[doc = "Bit 8 - Set this bit to clear the rx time-out interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn rxfifo_tout_int_clr(&mut self) -> RXFIFO_TOUT_INT_CLR_W<UART_INT_CLR_SPEC, 8> {
        RXFIFO_TOUT_INT_CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "UART INTERRUPT CLEAR REGISTER\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UART_INT_CLR_SPEC;
impl crate::RegisterSpec for UART_INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`uart_int_clr::W`](W) writer structure"]
impl crate::Writable for UART_INT_CLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UART_INT_CLR to value 0"]
impl crate::Resettable for UART_INT_CLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
