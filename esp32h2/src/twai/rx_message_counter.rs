#[doc = "Register `RX_MESSAGE_COUNTER` reader"]
pub struct R(crate::R<RX_MESSAGE_COUNTER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_MESSAGE_COUNTER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_MESSAGE_COUNTER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_MESSAGE_COUNTER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RX_MESSAGE_COUNTER` reader - Reflects the number of messages available within the RXFIFO. The value is incremented with each receive event and decremented by the release receive buffer command."]
pub type RX_MESSAGE_COUNTER_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:6 - Reflects the number of messages available within the RXFIFO. The value is incremented with each receive event and decremented by the release receive buffer command."]
    #[inline(always)]
    pub fn rx_message_counter(&self) -> RX_MESSAGE_COUNTER_R {
        RX_MESSAGE_COUNTER_R::new((self.bits & 0x7f) as u8)
    }
}
#[doc = "Received message counter register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_message_counter](index.html) module"]
pub struct RX_MESSAGE_COUNTER_SPEC;
impl crate::RegisterSpec for RX_MESSAGE_COUNTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx_message_counter::R](R) reader structure"]
impl crate::Readable for RX_MESSAGE_COUNTER_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RX_MESSAGE_COUNTER to value 0"]
impl crate::Resettable for RX_MESSAGE_COUNTER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
