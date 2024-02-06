#[doc = "Register `RX_MESSAGE_CNT` reader"]
pub type R = crate::R<RX_MESSAGE_CNT_SPEC>;
#[doc = "Field `RX_MESSAGE_COUNTER` reader - This register reflects the number of messages available within the RX FIFO."]
pub type RX_MESSAGE_COUNTER_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:6 - This register reflects the number of messages available within the RX FIFO."]
    #[inline(always)]
    pub fn rx_message_counter(&self) -> RX_MESSAGE_COUNTER_R {
        RX_MESSAGE_COUNTER_R::new((self.bits & 0x7f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RX_MESSAGE_CNT")
            .field(
                "rx_message_counter",
                &format_args!("{}", self.rx_message_counter().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RX_MESSAGE_CNT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Receive Message Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_message_cnt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RX_MESSAGE_CNT_SPEC;
impl crate::RegisterSpec for RX_MESSAGE_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_message_cnt::R`](R) reader structure"]
impl crate::Readable for RX_MESSAGE_CNT_SPEC {}
#[doc = "`reset()` method sets RX_MESSAGE_CNT to value 0"]
impl crate::Resettable for RX_MESSAGE_CNT_SPEC {
    const RESET_VALUE: u32 = 0;
}
