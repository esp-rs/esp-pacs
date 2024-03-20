#[doc = "Register `INT_CLR` writer"]
pub type W = crate::W<INT_CLR_SPEC>;
#[doc = "Field `TX_FIFO_REMPTY_INT_CLR` writer - Write 1 to clear TX_FIFO_REMPTY_INTR."]
pub type TX_FIFO_REMPTY_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_FIFO_WOVF_INT_CLR` writer - Write 1 to clear RX_FIFO_WOVF_INTR."]
pub type RX_FIFO_WOVF_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_EOF_INT_CLR` writer - Write 1 to clear TX_EOF_INTR."]
pub type TX_EOF_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Write 1 to clear TX_FIFO_REMPTY_INTR."]
    #[inline(always)]
    #[must_use]
    pub fn tx_fifo_rempty_int_clr(&mut self) -> TX_FIFO_REMPTY_INT_CLR_W<INT_CLR_SPEC> {
        TX_FIFO_REMPTY_INT_CLR_W::new(self, 0)
    }
    #[doc = "Bit 1 - Write 1 to clear RX_FIFO_WOVF_INTR."]
    #[inline(always)]
    #[must_use]
    pub fn rx_fifo_wovf_int_clr(&mut self) -> RX_FIFO_WOVF_INT_CLR_W<INT_CLR_SPEC> {
        RX_FIFO_WOVF_INT_CLR_W::new(self, 1)
    }
    #[doc = "Bit 2 - Write 1 to clear TX_EOF_INTR."]
    #[inline(always)]
    #[must_use]
    pub fn tx_eof_int_clr(&mut self) -> TX_EOF_INT_CLR_W<INT_CLR_SPEC> {
        TX_EOF_INT_CLR_W::new(self, 2)
    }
}
#[doc = "Parallel IO interrupt clear singal configuration register.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_CLR_SPEC;
impl crate::RegisterSpec for INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_clr::W`](W) writer structure"]
impl crate::Writable for INT_CLR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_CLR to value 0"]
impl crate::Resettable for INT_CLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
