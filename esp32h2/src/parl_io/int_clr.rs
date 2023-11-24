#[doc = "Register `INT_CLR` writer"]
pub type W = crate::W<INT_CLR_SPEC>;
#[doc = "Field `TX_FIFO_REMPTY_INT_CLR` writer - Set this bit to clear TX_FIFO_REMPTY_INT."]
pub type TX_FIFO_REMPTY_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_FIFO_WOVF_INT_CLR` writer - Set this bit to clear RX_FIFO_WOVF_INT."]
pub type RX_FIFO_WOVF_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_EOF_INT_CLR` writer - Set this bit to clear TX_EOF_INT."]
pub type TX_EOF_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to clear TX_FIFO_REMPTY_INT."]
    #[inline(always)]
    #[must_use]
    pub fn tx_fifo_rempty_int_clr(&mut self) -> TX_FIFO_REMPTY_INT_CLR_W<INT_CLR_SPEC> {
        TX_FIFO_REMPTY_INT_CLR_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set this bit to clear RX_FIFO_WOVF_INT."]
    #[inline(always)]
    #[must_use]
    pub fn rx_fifo_wovf_int_clr(&mut self) -> RX_FIFO_WOVF_INT_CLR_W<INT_CLR_SPEC> {
        RX_FIFO_WOVF_INT_CLR_W::new(self, 1)
    }
    #[doc = "Bit 2 - Set this bit to clear TX_EOF_INT."]
    #[inline(always)]
    #[must_use]
    pub fn tx_eof_int_clr(&mut self) -> TX_EOF_INT_CLR_W<INT_CLR_SPEC> {
        TX_EOF_INT_CLR_W::new(self, 2)
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
#[doc = "Parallel IO interrupt clear singal configuration register.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
