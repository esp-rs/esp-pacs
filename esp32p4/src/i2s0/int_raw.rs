#[doc = "Register `INT_RAW` reader"]
pub type R = crate::R<INT_RAW_SPEC>;
#[doc = "Field `RX_DONE_INT_RAW` reader - The raw interrupt status bit for the i2s_rx_done_int interrupt"]
pub type RX_DONE_INT_RAW_R = crate::BitReader;
#[doc = "Field `TX_DONE_INT_RAW` reader - The raw interrupt status bit for the i2s_tx_done_int interrupt"]
pub type TX_DONE_INT_RAW_R = crate::BitReader;
#[doc = "Field `RX_HUNG_INT_RAW` reader - The raw interrupt status bit for the i2s_rx_hung_int interrupt"]
pub type RX_HUNG_INT_RAW_R = crate::BitReader;
#[doc = "Field `TX_HUNG_INT_RAW` reader - The raw interrupt status bit for the i2s_tx_hung_int interrupt"]
pub type TX_HUNG_INT_RAW_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The raw interrupt status bit for the i2s_rx_done_int interrupt"]
    #[inline(always)]
    pub fn rx_done_int_raw(&self) -> RX_DONE_INT_RAW_R {
        RX_DONE_INT_RAW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The raw interrupt status bit for the i2s_tx_done_int interrupt"]
    #[inline(always)]
    pub fn tx_done_int_raw(&self) -> TX_DONE_INT_RAW_R {
        TX_DONE_INT_RAW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The raw interrupt status bit for the i2s_rx_hung_int interrupt"]
    #[inline(always)]
    pub fn rx_hung_int_raw(&self) -> RX_HUNG_INT_RAW_R {
        RX_HUNG_INT_RAW_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The raw interrupt status bit for the i2s_tx_hung_int interrupt"]
    #[inline(always)]
    pub fn tx_hung_int_raw(&self) -> TX_HUNG_INT_RAW_R {
        TX_HUNG_INT_RAW_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_RAW")
            .field(
                "rx_done_int_raw",
                &format_args!("{}", self.rx_done_int_raw().bit()),
            )
            .field(
                "tx_done_int_raw",
                &format_args!("{}", self.tx_done_int_raw().bit()),
            )
            .field(
                "rx_hung_int_raw",
                &format_args!("{}", self.rx_hung_int_raw().bit()),
            )
            .field(
                "tx_hung_int_raw",
                &format_args!("{}", self.tx_hung_int_raw().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_RAW_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "I2S interrupt raw register, valid in level.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
