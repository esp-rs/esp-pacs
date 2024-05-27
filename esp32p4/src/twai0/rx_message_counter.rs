///Register `RX_MESSAGE_COUNTER` reader
pub type R = crate::R<RX_MESSAGE_COUNTER_SPEC>;
///Field `RX_MESSAGE_COUNTER` reader - Reflects the number of messages available within the RXFIFO. The value is incremented with each receive event and decremented by the release receive buffer command.
pub type RX_MESSAGE_COUNTER_R = crate::FieldReader;
impl R {
    ///Bits 0:6 - Reflects the number of messages available within the RXFIFO. The value is incremented with each receive event and decremented by the release receive buffer command.
    #[inline(always)]
    pub fn rx_message_counter(&self) -> RX_MESSAGE_COUNTER_R {
        RX_MESSAGE_COUNTER_R::new((self.bits & 0x7f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RX_MESSAGE_COUNTER")
            .field("rx_message_counter", &self.rx_message_counter())
            .finish()
    }
}
/**Received message counter register.

You can [`read`](crate::generic::Reg::read) this register and get [`rx_message_counter::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RX_MESSAGE_COUNTER_SPEC;
impl crate::RegisterSpec for RX_MESSAGE_COUNTER_SPEC {
    type Ux = u32;
}
///`read()` method returns [`rx_message_counter::R`](R) reader structure
impl crate::Readable for RX_MESSAGE_COUNTER_SPEC {}
///`reset()` method sets RX_MESSAGE_COUNTER to value 0
impl crate::Resettable for RX_MESSAGE_COUNTER_SPEC {
    const RESET_VALUE: u32 = 0;
}
