#[doc = "Register `INT_ST` reader"]
pub type R = crate::R<INT_ST_SPEC>;
#[doc = "Field `TX_FIFO_REMPTY_INT_ST` reader - The masked interrupt status of TX_FIFO_REMPTY_INTR."]
pub type TX_FIFO_REMPTY_INT_ST_R = crate::BitReader;
#[doc = "Field `RX_FIFO_WFULL_INT_ST` reader - The masked interrupt status of RX_FIFO_WFULL_INTR."]
pub type RX_FIFO_WFULL_INT_ST_R = crate::BitReader;
#[doc = "Field `TX_EOF_INT_ST` reader - The masked interrupt status of TX_EOF_INTR."]
pub type TX_EOF_INT_ST_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The masked interrupt status of TX_FIFO_REMPTY_INTR."]
    #[inline(always)]
    pub fn tx_fifo_rempty_int_st(&self) -> TX_FIFO_REMPTY_INT_ST_R {
        TX_FIFO_REMPTY_INT_ST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The masked interrupt status of RX_FIFO_WFULL_INTR."]
    #[inline(always)]
    pub fn rx_fifo_wfull_int_st(&self) -> RX_FIFO_WFULL_INT_ST_R {
        RX_FIFO_WFULL_INT_ST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The masked interrupt status of TX_EOF_INTR."]
    #[inline(always)]
    pub fn tx_eof_int_st(&self) -> TX_EOF_INT_ST_R {
        TX_EOF_INT_ST_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ST")
            .field(
                "tx_fifo_rempty_int_st",
                &format_args!("{}", self.tx_fifo_rempty_int_st().bit()),
            )
            .field(
                "rx_fifo_wfull_int_st",
                &format_args!("{}", self.rx_fifo_wfull_int_st().bit()),
            )
            .field(
                "tx_eof_int_st",
                &format_args!("{}", self.tx_eof_int_st().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_ST_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Parallel IO interrupt singal status register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_st::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ST_SPEC;
impl crate::RegisterSpec for INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_st::R`](R) reader structure"]
impl crate::Readable for INT_ST_SPEC {}
#[doc = "`reset()` method sets INT_ST to value 0"]
impl crate::Resettable for INT_ST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
