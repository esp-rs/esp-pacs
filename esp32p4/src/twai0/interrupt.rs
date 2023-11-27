#[doc = "Register `INTERRUPT` reader"]
pub type R = crate::R<INTERRUPT_SPEC>;
#[doc = "Field `RECEIVE_INT_ST` reader - 1: this bit is set while the receive FIFO is not empty and the RIE bit is set within the interrupt enable register. 0: reset"]
pub type RECEIVE_INT_ST_R = crate::BitReader;
#[doc = "Field `TRANSMIT_INT_ST` reader - 1: this bit is set whenever the transmit buffer status changes from '0-to-1' (released) and the TIE bit is set within the interrupt enable register. 0: reset"]
pub type TRANSMIT_INT_ST_R = crate::BitReader;
#[doc = "Field `ERR_WARNING_INT_ST` reader - 1: this bit is set on every change (set and clear) of either the error status or bus status bits and the EIE bit is set within the interrupt enable register. 0: reset"]
pub type ERR_WARNING_INT_ST_R = crate::BitReader;
#[doc = "Field `DATA_OVERRUN_INT_ST` reader - 1: this bit is set on a '0-to-1' transition of the data overrun status bit and the DOIE bit is set within the interrupt enable register. 0: reset"]
pub type DATA_OVERRUN_INT_ST_R = crate::BitReader;
#[doc = "Field `TS_COUNTER_OVFL_INT_ST` reader - 1: this bit is set then the timestamp counter reaches the maximum value and overflow."]
pub type TS_COUNTER_OVFL_INT_ST_R = crate::BitReader;
#[doc = "Field `ERR_PASSIVE_INT_ST` reader - 1: this bit is set whenever the TWAI controller has reached the error passive status (at least one error counter exceeds the protocol-defined level of 127) or if the TWAI controller is in the error passive status and enters the error active status again and the EPIE bit is set within the interrupt enable register. 0: reset"]
pub type ERR_PASSIVE_INT_ST_R = crate::BitReader;
#[doc = "Field `ARBITRATION_LOST_INT_ST` reader - 1: this bit is set when the TWAI controller lost the arbitration and becomes a receiver and the ALIE bit is set within the interrupt enable register. 0: reset"]
pub type ARBITRATION_LOST_INT_ST_R = crate::BitReader;
#[doc = "Field `BUS_ERR_INT_ST` reader - 1: this bit is set when the TWAI controller detects an error on the TWAI-bus and the BEIE bit is set within the interrupt enable register. 0: reset"]
pub type BUS_ERR_INT_ST_R = crate::BitReader;
#[doc = "Field `IDLE_INT_ST` reader - 1: this bit is set when the TWAI controller detects state of TWAI become IDLE and this interrupt enable bit is set within the interrupt enable register. 0: reset"]
pub type IDLE_INT_ST_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - 1: this bit is set while the receive FIFO is not empty and the RIE bit is set within the interrupt enable register. 0: reset"]
    #[inline(always)]
    pub fn receive_int_st(&self) -> RECEIVE_INT_ST_R {
        RECEIVE_INT_ST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1: this bit is set whenever the transmit buffer status changes from '0-to-1' (released) and the TIE bit is set within the interrupt enable register. 0: reset"]
    #[inline(always)]
    pub fn transmit_int_st(&self) -> TRANSMIT_INT_ST_R {
        TRANSMIT_INT_ST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 1: this bit is set on every change (set and clear) of either the error status or bus status bits and the EIE bit is set within the interrupt enable register. 0: reset"]
    #[inline(always)]
    pub fn err_warning_int_st(&self) -> ERR_WARNING_INT_ST_R {
        ERR_WARNING_INT_ST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 1: this bit is set on a '0-to-1' transition of the data overrun status bit and the DOIE bit is set within the interrupt enable register. 0: reset"]
    #[inline(always)]
    pub fn data_overrun_int_st(&self) -> DATA_OVERRUN_INT_ST_R {
        DATA_OVERRUN_INT_ST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 1: this bit is set then the timestamp counter reaches the maximum value and overflow."]
    #[inline(always)]
    pub fn ts_counter_ovfl_int_st(&self) -> TS_COUNTER_OVFL_INT_ST_R {
        TS_COUNTER_OVFL_INT_ST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 1: this bit is set whenever the TWAI controller has reached the error passive status (at least one error counter exceeds the protocol-defined level of 127) or if the TWAI controller is in the error passive status and enters the error active status again and the EPIE bit is set within the interrupt enable register. 0: reset"]
    #[inline(always)]
    pub fn err_passive_int_st(&self) -> ERR_PASSIVE_INT_ST_R {
        ERR_PASSIVE_INT_ST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 1: this bit is set when the TWAI controller lost the arbitration and becomes a receiver and the ALIE bit is set within the interrupt enable register. 0: reset"]
    #[inline(always)]
    pub fn arbitration_lost_int_st(&self) -> ARBITRATION_LOST_INT_ST_R {
        ARBITRATION_LOST_INT_ST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 1: this bit is set when the TWAI controller detects an error on the TWAI-bus and the BEIE bit is set within the interrupt enable register. 0: reset"]
    #[inline(always)]
    pub fn bus_err_int_st(&self) -> BUS_ERR_INT_ST_R {
        BUS_ERR_INT_ST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 1: this bit is set when the TWAI controller detects state of TWAI become IDLE and this interrupt enable bit is set within the interrupt enable register. 0: reset"]
    #[inline(always)]
    pub fn idle_int_st(&self) -> IDLE_INT_ST_R {
        IDLE_INT_ST_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTERRUPT")
            .field(
                "receive_int_st",
                &format_args!("{}", self.receive_int_st().bit()),
            )
            .field(
                "transmit_int_st",
                &format_args!("{}", self.transmit_int_st().bit()),
            )
            .field(
                "err_warning_int_st",
                &format_args!("{}", self.err_warning_int_st().bit()),
            )
            .field(
                "data_overrun_int_st",
                &format_args!("{}", self.data_overrun_int_st().bit()),
            )
            .field(
                "ts_counter_ovfl_int_st",
                &format_args!("{}", self.ts_counter_ovfl_int_st().bit()),
            )
            .field(
                "err_passive_int_st",
                &format_args!("{}", self.err_passive_int_st().bit()),
            )
            .field(
                "arbitration_lost_int_st",
                &format_args!("{}", self.arbitration_lost_int_st().bit()),
            )
            .field(
                "bus_err_int_st",
                &format_args!("{}", self.bus_err_int_st().bit()),
            )
            .field("idle_int_st", &format_args!("{}", self.idle_int_st().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INTERRUPT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Interrupt signals' register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`interrupt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTERRUPT_SPEC;
impl crate::RegisterSpec for INTERRUPT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`interrupt::R`](R) reader structure"]
impl crate::Readable for INTERRUPT_SPEC {}
#[doc = "`reset()` method sets INTERRUPT to value 0"]
impl crate::Resettable for INTERRUPT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
