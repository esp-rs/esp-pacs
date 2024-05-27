///Register `RX_ST0` reader
pub type R = crate::R<RX_ST0_SPEC>;
///Field `RX_CNT` reader - Indicates the cycle number of reading Rx FIFO.
pub type RX_CNT_R = crate::FieldReader;
///Field `RX_FIFO_WR_BIT_CNT` reader - Indicates the current written bit number into Rx FIFO.
pub type RX_FIFO_WR_BIT_CNT_R = crate::FieldReader<u32>;
impl R {
    ///Bits 8:12 - Indicates the cycle number of reading Rx FIFO.
    #[inline(always)]
    pub fn rx_cnt(&self) -> RX_CNT_R {
        RX_CNT_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    ///Bits 13:31 - Indicates the current written bit number into Rx FIFO.
    #[inline(always)]
    pub fn rx_fifo_wr_bit_cnt(&self) -> RX_FIFO_WR_BIT_CNT_R {
        RX_FIFO_WR_BIT_CNT_R::new((self.bits >> 13) & 0x0007_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RX_ST0")
            .field("rx_cnt", &self.rx_cnt())
            .field("rx_fifo_wr_bit_cnt", &self.rx_fifo_wr_bit_cnt())
            .finish()
    }
}
/**Parallel IO RX status register0

You can [`read`](crate::generic::Reg::read) this register and get [`rx_st0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RX_ST0_SPEC;
impl crate::RegisterSpec for RX_ST0_SPEC {
    type Ux = u32;
}
///`read()` method returns [`rx_st0::R`](R) reader structure
impl crate::Readable for RX_ST0_SPEC {}
///`reset()` method sets RX_ST0 to value 0
impl crate::Resettable for RX_ST0_SPEC {
    const RESET_VALUE: u32 = 0;
}
