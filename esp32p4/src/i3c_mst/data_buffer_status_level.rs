///Register `DATA_BUFFER_STATUS_LEVEL` reader
pub type R = crate::R<DATA_BUFFER_STATUS_LEVEL_SPEC>;
///Field `TX_DATA_BUF_EMPTY_CNT` reader - Transmit Buffer Empty Level Value contains the number of empty locations in the transmit Buffer.
pub type TX_DATA_BUF_EMPTY_CNT_R = crate::FieldReader;
///Field `RX_DATA_BUF_CNT` reader - Receive Buffer Level value contains the number of valid data entries in the receive buffer.
pub type RX_DATA_BUF_CNT_R = crate::FieldReader;
impl R {
    ///Bits 0:5 - Transmit Buffer Empty Level Value contains the number of empty locations in the transmit Buffer.
    #[inline(always)]
    pub fn tx_data_buf_empty_cnt(&self) -> TX_DATA_BUF_EMPTY_CNT_R {
        TX_DATA_BUF_EMPTY_CNT_R::new((self.bits & 0x3f) as u8)
    }
    ///Bits 16:21 - Receive Buffer Level value contains the number of valid data entries in the receive buffer.
    #[inline(always)]
    pub fn rx_data_buf_cnt(&self) -> RX_DATA_BUF_CNT_R {
        RX_DATA_BUF_CNT_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DATA_BUFFER_STATUS_LEVEL")
            .field("tx_data_buf_empty_cnt", &self.tx_data_buf_empty_cnt())
            .field("rx_data_buf_cnt", &self.rx_data_buf_cnt())
            .finish()
    }
}
/**DATA_BUFFER_STATUS_LEVEL reflects the status level of the Buffers in the controller.

You can [`read`](crate::generic::Reg::read) this register and get [`data_buffer_status_level::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DATA_BUFFER_STATUS_LEVEL_SPEC;
impl crate::RegisterSpec for DATA_BUFFER_STATUS_LEVEL_SPEC {
    type Ux = u32;
}
///`read()` method returns [`data_buffer_status_level::R`](R) reader structure
impl crate::Readable for DATA_BUFFER_STATUS_LEVEL_SPEC {}
///`reset()` method sets DATA_BUFFER_STATUS_LEVEL to value 0x20
impl crate::Resettable for DATA_BUFFER_STATUS_LEVEL_SPEC {
    const RESET_VALUE: u32 = 0x20;
}
