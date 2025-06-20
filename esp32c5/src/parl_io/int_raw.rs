#[doc = "Register `INT_RAW` reader"]
pub type R = crate::R<INT_RAW_SPEC>;
#[doc = "Field `TX_FIFO_REMPTY_INT_RAW` reader - The raw interrupt status of TX_FIFO_REMPTY_INT."]
pub type TX_FIFO_REMPTY_INT_RAW_R = crate::BitReader;
#[doc = "Field `RX_FIFO_WOVF_INT_RAW` reader - The raw interrupt status of RX_FIFO_WOVF_INT."]
pub type RX_FIFO_WOVF_INT_RAW_R = crate::BitReader;
#[doc = "Field `TX_EOF_INT_RAW` reader - The raw interrupt status of TX_EOF_INT."]
pub type TX_EOF_INT_RAW_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The raw interrupt status of TX_FIFO_REMPTY_INT."]
    #[inline(always)]
    pub fn tx_fifo_rempty_int_raw(&self) -> TX_FIFO_REMPTY_INT_RAW_R {
        TX_FIFO_REMPTY_INT_RAW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The raw interrupt status of RX_FIFO_WOVF_INT."]
    #[inline(always)]
    pub fn rx_fifo_wovf_int_raw(&self) -> RX_FIFO_WOVF_INT_RAW_R {
        RX_FIFO_WOVF_INT_RAW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The raw interrupt status of TX_EOF_INT."]
    #[inline(always)]
    pub fn tx_eof_int_raw(&self) -> TX_EOF_INT_RAW_R {
        TX_EOF_INT_RAW_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_RAW")
            .field("tx_fifo_rempty_int_raw", &self.tx_fifo_rempty_int_raw())
            .field("rx_fifo_wovf_int_raw", &self.rx_fifo_wovf_int_raw())
            .field("tx_eof_int_raw", &self.tx_eof_int_raw())
            .finish()
    }
}
#[doc = "Parallel IO interrupt raw singal status register.\n\nYou can [`read`](crate::Reg::read) this register and get [`int_raw::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_RAW_SPEC;
impl crate::RegisterSpec for INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_raw::R`](R) reader structure"]
impl crate::Readable for INT_RAW_SPEC {}
#[doc = "`reset()` method sets INT_RAW to value 0"]
impl crate::Resettable for INT_RAW_SPEC {
    const RESET_VALUE: u32 = 0;
}
