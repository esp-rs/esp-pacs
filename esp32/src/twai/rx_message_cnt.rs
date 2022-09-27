#[doc = "Register `RX_MESSAGE_CNT` reader"]
pub struct R(crate::R<RX_MESSAGE_CNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_MESSAGE_CNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_MESSAGE_CNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_MESSAGE_CNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RX_MESSAGE_COUNTER` reader - This register reflects the number of messages available within the RX FIFO."]
pub type RX_MESSAGE_COUNTER_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:6 - This register reflects the number of messages available within the RX FIFO."]
    #[inline(always)]
    pub fn rx_message_counter(&self) -> RX_MESSAGE_COUNTER_R {
        RX_MESSAGE_COUNTER_R::new((self.bits & 0x7f) as u8)
    }
}
#[doc = "Receive Message Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_message_cnt](index.html) module"]
pub struct RX_MESSAGE_CNT_SPEC;
impl crate::RegisterSpec for RX_MESSAGE_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx_message_cnt::R](R) reader structure"]
impl crate::Readable for RX_MESSAGE_CNT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RX_MESSAGE_CNT to value 0"]
impl crate::Resettable for RX_MESSAGE_CNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
