#[doc = "Register `STATUS` reader"]
pub struct R(crate::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RECEIVE_BUFFER` reader - 1: full, one or more complete messages are available in the RXFIFO. 0: empty, no message is available"]
pub type RECEIVE_BUFFER_R = crate::BitReader;
#[doc = "Field `OVERRUN` reader - 1: overrun, a message was lost because there was not enough space for that message in the RXFIFO. 0: absent, no data overrun has occurred since the last clear data overrun command was given"]
pub type OVERRUN_R = crate::BitReader;
#[doc = "Field `TRANSMIT_BUFFER` reader - 1: released, the CPU may write a message into the transmit buffer. 0: locked, the CPU cannot access the transmit buffer, a message is either waiting for transmission or is in the process of being transmitted"]
pub type TRANSMIT_BUFFER_R = crate::BitReader;
#[doc = "Field `TRANSMISSION_COMPLETE` reader - 1: complete, last requested transmission has been successfully completed. 0: incomplete, previously requested transmission is not yet completed"]
pub type TRANSMISSION_COMPLETE_R = crate::BitReader;
#[doc = "Field `RECEIVE` reader - 1: receive, the TWAI controller is receiving a message. 0: idle"]
pub type RECEIVE_R = crate::BitReader;
#[doc = "Field `TRANSMIT` reader - 1: transmit, the TWAI controller is transmitting a message. 0: idle"]
pub type TRANSMIT_R = crate::BitReader;
#[doc = "Field `ERR` reader - 1: error, at least one of the error counters has reached or exceeded the CPU warning limit defined by the Error Warning Limit Register (EWLR). 0: ok, both error counters are below the warning limit"]
pub type ERR_R = crate::BitReader;
#[doc = "Field `NODE_BUS_OFF` reader - 1: bus-off, the TWAI controller is not involved in bus activities. 0: bus-on, the TWAI controller is involved in bus activities"]
pub type NODE_BUS_OFF_R = crate::BitReader;
#[doc = "Field `MISS` reader - 1: current message is destroyed because of FIFO overflow."]
pub type MISS_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - 1: full, one or more complete messages are available in the RXFIFO. 0: empty, no message is available"]
    #[inline(always)]
    pub fn receive_buffer(&self) -> RECEIVE_BUFFER_R {
        RECEIVE_BUFFER_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1: overrun, a message was lost because there was not enough space for that message in the RXFIFO. 0: absent, no data overrun has occurred since the last clear data overrun command was given"]
    #[inline(always)]
    pub fn overrun(&self) -> OVERRUN_R {
        OVERRUN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 1: released, the CPU may write a message into the transmit buffer. 0: locked, the CPU cannot access the transmit buffer, a message is either waiting for transmission or is in the process of being transmitted"]
    #[inline(always)]
    pub fn transmit_buffer(&self) -> TRANSMIT_BUFFER_R {
        TRANSMIT_BUFFER_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 1: complete, last requested transmission has been successfully completed. 0: incomplete, previously requested transmission is not yet completed"]
    #[inline(always)]
    pub fn transmission_complete(&self) -> TRANSMISSION_COMPLETE_R {
        TRANSMISSION_COMPLETE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 1: receive, the TWAI controller is receiving a message. 0: idle"]
    #[inline(always)]
    pub fn receive(&self) -> RECEIVE_R {
        RECEIVE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 1: transmit, the TWAI controller is transmitting a message. 0: idle"]
    #[inline(always)]
    pub fn transmit(&self) -> TRANSMIT_R {
        TRANSMIT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 1: error, at least one of the error counters has reached or exceeded the CPU warning limit defined by the Error Warning Limit Register (EWLR). 0: ok, both error counters are below the warning limit"]
    #[inline(always)]
    pub fn err(&self) -> ERR_R {
        ERR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 1: bus-off, the TWAI controller is not involved in bus activities. 0: bus-on, the TWAI controller is involved in bus activities"]
    #[inline(always)]
    pub fn node_bus_off(&self) -> NODE_BUS_OFF_R {
        NODE_BUS_OFF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 1: current message is destroyed because of FIFO overflow."]
    #[inline(always)]
    pub fn miss(&self) -> MISS_R {
        MISS_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATUS")
            .field(
                "receive_buffer",
                &format_args!("{}", self.receive_buffer().bit()),
            )
            .field("overrun", &format_args!("{}", self.overrun().bit()))
            .field(
                "transmit_buffer",
                &format_args!("{}", self.transmit_buffer().bit()),
            )
            .field(
                "transmission_complete",
                &format_args!("{}", self.transmission_complete().bit()),
            )
            .field("receive", &format_args!("{}", self.receive().bit()))
            .field("transmit", &format_args!("{}", self.transmit().bit()))
            .field("err", &format_args!("{}", self.err().bit()))
            .field(
                "node_bus_off",
                &format_args!("{}", self.node_bus_off().bit()),
            )
            .field("miss", &format_args!("{}", self.miss().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<STATUS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "TWAI status register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
