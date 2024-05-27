///Register `INT_RAW` reader
pub type R = crate::R<INT_RAW_SPEC>;
///Field `RX_DONE` reader - The raw interrupt status bit for the i2s_rx_done_int interrupt
pub type RX_DONE_R = crate::BitReader;
///Field `TX_DONE` reader - The raw interrupt status bit for the i2s_tx_done_int interrupt
pub type TX_DONE_R = crate::BitReader;
///Field `RX_HUNG` reader - The raw interrupt status bit for the i2s_rx_hung_int interrupt
pub type RX_HUNG_R = crate::BitReader;
///Field `TX_HUNG` reader - The raw interrupt status bit for the i2s_tx_hung_int interrupt
pub type TX_HUNG_R = crate::BitReader;
impl R {
    ///Bit 0 - The raw interrupt status bit for the i2s_rx_done_int interrupt
    #[inline(always)]
    pub fn rx_done(&self) -> RX_DONE_R {
        RX_DONE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - The raw interrupt status bit for the i2s_tx_done_int interrupt
    #[inline(always)]
    pub fn tx_done(&self) -> TX_DONE_R {
        TX_DONE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - The raw interrupt status bit for the i2s_rx_hung_int interrupt
    #[inline(always)]
    pub fn rx_hung(&self) -> RX_HUNG_R {
        RX_HUNG_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - The raw interrupt status bit for the i2s_tx_hung_int interrupt
    #[inline(always)]
    pub fn tx_hung(&self) -> TX_HUNG_R {
        TX_HUNG_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_RAW")
            .field("rx_done", &self.rx_done())
            .field("tx_done", &self.tx_done())
            .field("rx_hung", &self.rx_hung())
            .field("tx_hung", &self.tx_hung())
            .finish()
    }
}
/**I2S interrupt raw register, valid in level.

You can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct INT_RAW_SPEC;
impl crate::RegisterSpec for INT_RAW_SPEC {
    type Ux = u32;
}
///`read()` method returns [`int_raw::R`](R) reader structure
impl crate::Readable for INT_RAW_SPEC {}
///`reset()` method sets INT_RAW to value 0
impl crate::Resettable for INT_RAW_SPEC {
    const RESET_VALUE: u32 = 0;
}
