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
#[doc = "Field `RX_BUF_ST` reader - 1: The data in the RX buffer is not empty, with at least one received data packet."]
pub type RX_BUF_ST_R = crate::BitReader;
#[doc = "Field `OVERRUN_ST` reader - 1: The RX FIFO is full and data overrun has occurred."]
pub type OVERRUN_ST_R = crate::BitReader;
#[doc = "Field `TX_BUF_ST` reader - 1: The TX buffer is empty, the CPU may write a message into it."]
pub type TX_BUF_ST_R = crate::BitReader;
#[doc = "Field `TX_COMPLETE` reader - 1: The TWAI controller has successfully received a packet from the bus."]
pub type TX_COMPLETE_R = crate::BitReader;
#[doc = "Field `RX_ST` reader - 1: The TWAI Controller is receiving a message from the bus."]
pub type RX_ST_R = crate::BitReader;
#[doc = "Field `TX_ST` reader - 1: The TWAI Controller is transmitting a message to the bus."]
pub type TX_ST_R = crate::BitReader;
#[doc = "Field `ERR_ST` reader - 1: At least one of the RX/TX error counter has reached or exceeded the value set in register TWAI_ERR_WARNING_LIMIT_REG."]
pub type ERR_ST_R = crate::BitReader;
#[doc = "Field `BUS_OFF_ST` reader - 1: In bus-off status, the TWAI Controller is no longer involved in bus activities."]
pub type BUS_OFF_ST_R = crate::BitReader;
#[doc = "Field `MISS_ST` reader - This bit reflects whether the data packet in the RX FIFO is complete. 1: The current packet is missing; 0: The current packet is complete"]
pub type MISS_ST_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - 1: The data in the RX buffer is not empty, with at least one received data packet."]
    #[inline(always)]
    pub fn rx_buf_st(&self) -> RX_BUF_ST_R {
        RX_BUF_ST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1: The RX FIFO is full and data overrun has occurred."]
    #[inline(always)]
    pub fn overrun_st(&self) -> OVERRUN_ST_R {
        OVERRUN_ST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 1: The TX buffer is empty, the CPU may write a message into it."]
    #[inline(always)]
    pub fn tx_buf_st(&self) -> TX_BUF_ST_R {
        TX_BUF_ST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 1: The TWAI controller has successfully received a packet from the bus."]
    #[inline(always)]
    pub fn tx_complete(&self) -> TX_COMPLETE_R {
        TX_COMPLETE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 1: The TWAI Controller is receiving a message from the bus."]
    #[inline(always)]
    pub fn rx_st(&self) -> RX_ST_R {
        RX_ST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 1: The TWAI Controller is transmitting a message to the bus."]
    #[inline(always)]
    pub fn tx_st(&self) -> TX_ST_R {
        TX_ST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 1: At least one of the RX/TX error counter has reached or exceeded the value set in register TWAI_ERR_WARNING_LIMIT_REG."]
    #[inline(always)]
    pub fn err_st(&self) -> ERR_ST_R {
        ERR_ST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 1: In bus-off status, the TWAI Controller is no longer involved in bus activities."]
    #[inline(always)]
    pub fn bus_off_st(&self) -> BUS_OFF_ST_R {
        BUS_OFF_ST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - This bit reflects whether the data packet in the RX FIFO is complete. 1: The current packet is missing; 0: The current packet is complete"]
    #[inline(always)]
    pub fn miss_st(&self) -> MISS_ST_R {
        MISS_ST_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATUS")
            .field("rx_buf_st", &format_args!("{}", self.rx_buf_st().bit()))
            .field("overrun_st", &format_args!("{}", self.overrun_st().bit()))
            .field("tx_buf_st", &format_args!("{}", self.tx_buf_st().bit()))
            .field("tx_complete", &format_args!("{}", self.tx_complete().bit()))
            .field("rx_st", &format_args!("{}", self.rx_st().bit()))
            .field("tx_st", &format_args!("{}", self.tx_st().bit()))
            .field("err_st", &format_args!("{}", self.err_st().bit()))
            .field("bus_off_st", &format_args!("{}", self.bus_off_st().bit()))
            .field("miss_st", &format_args!("{}", self.miss_st().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<STATUS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
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
