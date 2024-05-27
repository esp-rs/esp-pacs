///Register `INTERRUPT_ENABLE` reader
pub type R = crate::R<INTERRUPT_ENABLE_SPEC>;
///Register `INTERRUPT_ENABLE` writer
pub type W = crate::W<INTERRUPT_ENABLE_SPEC>;
///Field `EXT_RECEIVE_INT_ENA` reader - 1: enabled, when the receive buffer status is 'full' the TWAI controller requests the respective interrupt. 0: disable
pub type EXT_RECEIVE_INT_ENA_R = crate::BitReader;
///Field `EXT_RECEIVE_INT_ENA` writer - 1: enabled, when the receive buffer status is 'full' the TWAI controller requests the respective interrupt. 0: disable
pub type EXT_RECEIVE_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EXT_TRANSMIT_INT_ENA` reader - 1: enabled, when a message has been successfully transmitted or the transmit buffer is accessible again (e.g. after an abort transmission command), the TWAI controller requests the respective interrupt. 0: disable
pub type EXT_TRANSMIT_INT_ENA_R = crate::BitReader;
///Field `EXT_TRANSMIT_INT_ENA` writer - 1: enabled, when a message has been successfully transmitted or the transmit buffer is accessible again (e.g. after an abort transmission command), the TWAI controller requests the respective interrupt. 0: disable
pub type EXT_TRANSMIT_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EXT_ERR_WARNING_INT_ENA` reader - 1: enabled, if the error or bus status change (see status register. Table 14), the TWAI controllerrequests the respective interrupt. 0: disable
pub type EXT_ERR_WARNING_INT_ENA_R = crate::BitReader;
///Field `EXT_ERR_WARNING_INT_ENA` writer - 1: enabled, if the error or bus status change (see status register. Table 14), the TWAI controllerrequests the respective interrupt. 0: disable
pub type EXT_ERR_WARNING_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EXT_DATA_OVERRUN_INT_ENA` reader - 1: enabled, if the data overrun status bit is set (see status register. Table 14), the TWAI controllerrequests the respective interrupt. 0: disable
pub type EXT_DATA_OVERRUN_INT_ENA_R = crate::BitReader;
///Field `EXT_DATA_OVERRUN_INT_ENA` writer - 1: enabled, if the data overrun status bit is set (see status register. Table 14), the TWAI controllerrequests the respective interrupt. 0: disable
pub type EXT_DATA_OVERRUN_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TS_COUNTER_OVFL_INT_ENA` reader - enable the timestamp counter overflow interrupt request.
pub type TS_COUNTER_OVFL_INT_ENA_R = crate::BitReader;
///Field `TS_COUNTER_OVFL_INT_ENA` writer - enable the timestamp counter overflow interrupt request.
pub type TS_COUNTER_OVFL_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ERR_PASSIVE_INT_ENA` reader - 1: enabled, if the error status of the TWAI controller changes from error active to error passive or vice versa, the respective interrupt is requested. 0: disable
pub type ERR_PASSIVE_INT_ENA_R = crate::BitReader;
///Field `ERR_PASSIVE_INT_ENA` writer - 1: enabled, if the error status of the TWAI controller changes from error active to error passive or vice versa, the respective interrupt is requested. 0: disable
pub type ERR_PASSIVE_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ARBITRATION_LOST_INT_ENA` reader - 1: enabled, if the TWAI controller has lost arbitration, the respective interrupt is requested. 0: disable
pub type ARBITRATION_LOST_INT_ENA_R = crate::BitReader;
///Field `ARBITRATION_LOST_INT_ENA` writer - 1: enabled, if the TWAI controller has lost arbitration, the respective interrupt is requested. 0: disable
pub type ARBITRATION_LOST_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BUS_ERR_INT_ENA` reader - 1: enabled, if an bus error has been detected, the TWAI controller requests the respective interrupt. 0: disable
pub type BUS_ERR_INT_ENA_R = crate::BitReader;
///Field `BUS_ERR_INT_ENA` writer - 1: enabled, if an bus error has been detected, the TWAI controller requests the respective interrupt. 0: disable
pub type BUS_ERR_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IDLE_INT_ENA` reader - 1: enabled, if state of TWAI become IDLE, the TWAI controller requests the respective interrupt. 0: disable
pub type IDLE_INT_ENA_R = crate::BitReader;
impl R {
    ///Bit 0 - 1: enabled, when the receive buffer status is 'full' the TWAI controller requests the respective interrupt. 0: disable
    #[inline(always)]
    pub fn ext_receive_int_ena(&self) -> EXT_RECEIVE_INT_ENA_R {
        EXT_RECEIVE_INT_ENA_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - 1: enabled, when a message has been successfully transmitted or the transmit buffer is accessible again (e.g. after an abort transmission command), the TWAI controller requests the respective interrupt. 0: disable
    #[inline(always)]
    pub fn ext_transmit_int_ena(&self) -> EXT_TRANSMIT_INT_ENA_R {
        EXT_TRANSMIT_INT_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - 1: enabled, if the error or bus status change (see status register. Table 14), the TWAI controllerrequests the respective interrupt. 0: disable
    #[inline(always)]
    pub fn ext_err_warning_int_ena(&self) -> EXT_ERR_WARNING_INT_ENA_R {
        EXT_ERR_WARNING_INT_ENA_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - 1: enabled, if the data overrun status bit is set (see status register. Table 14), the TWAI controllerrequests the respective interrupt. 0: disable
    #[inline(always)]
    pub fn ext_data_overrun_int_ena(&self) -> EXT_DATA_OVERRUN_INT_ENA_R {
        EXT_DATA_OVERRUN_INT_ENA_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - enable the timestamp counter overflow interrupt request.
    #[inline(always)]
    pub fn ts_counter_ovfl_int_ena(&self) -> TS_COUNTER_OVFL_INT_ENA_R {
        TS_COUNTER_OVFL_INT_ENA_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - 1: enabled, if the error status of the TWAI controller changes from error active to error passive or vice versa, the respective interrupt is requested. 0: disable
    #[inline(always)]
    pub fn err_passive_int_ena(&self) -> ERR_PASSIVE_INT_ENA_R {
        ERR_PASSIVE_INT_ENA_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - 1: enabled, if the TWAI controller has lost arbitration, the respective interrupt is requested. 0: disable
    #[inline(always)]
    pub fn arbitration_lost_int_ena(&self) -> ARBITRATION_LOST_INT_ENA_R {
        ARBITRATION_LOST_INT_ENA_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - 1: enabled, if an bus error has been detected, the TWAI controller requests the respective interrupt. 0: disable
    #[inline(always)]
    pub fn bus_err_int_ena(&self) -> BUS_ERR_INT_ENA_R {
        BUS_ERR_INT_ENA_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - 1: enabled, if state of TWAI become IDLE, the TWAI controller requests the respective interrupt. 0: disable
    #[inline(always)]
    pub fn idle_int_ena(&self) -> IDLE_INT_ENA_R {
        IDLE_INT_ENA_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTERRUPT_ENABLE")
            .field("ext_receive_int_ena", &self.ext_receive_int_ena())
            .field("ext_transmit_int_ena", &self.ext_transmit_int_ena())
            .field("ext_err_warning_int_ena", &self.ext_err_warning_int_ena())
            .field("ext_data_overrun_int_ena", &self.ext_data_overrun_int_ena())
            .field("ts_counter_ovfl_int_ena", &self.ts_counter_ovfl_int_ena())
            .field("err_passive_int_ena", &self.err_passive_int_ena())
            .field("arbitration_lost_int_ena", &self.arbitration_lost_int_ena())
            .field("bus_err_int_ena", &self.bus_err_int_ena())
            .field("idle_int_ena", &self.idle_int_ena())
            .finish()
    }
}
impl W {
    ///Bit 0 - 1: enabled, when the receive buffer status is 'full' the TWAI controller requests the respective interrupt. 0: disable
    #[inline(always)]
    #[must_use]
    pub fn ext_receive_int_ena(&mut self) -> EXT_RECEIVE_INT_ENA_W<INTERRUPT_ENABLE_SPEC> {
        EXT_RECEIVE_INT_ENA_W::new(self, 0)
    }
    ///Bit 1 - 1: enabled, when a message has been successfully transmitted or the transmit buffer is accessible again (e.g. after an abort transmission command), the TWAI controller requests the respective interrupt. 0: disable
    #[inline(always)]
    #[must_use]
    pub fn ext_transmit_int_ena(&mut self) -> EXT_TRANSMIT_INT_ENA_W<INTERRUPT_ENABLE_SPEC> {
        EXT_TRANSMIT_INT_ENA_W::new(self, 1)
    }
    ///Bit 2 - 1: enabled, if the error or bus status change (see status register. Table 14), the TWAI controllerrequests the respective interrupt. 0: disable
    #[inline(always)]
    #[must_use]
    pub fn ext_err_warning_int_ena(&mut self) -> EXT_ERR_WARNING_INT_ENA_W<INTERRUPT_ENABLE_SPEC> {
        EXT_ERR_WARNING_INT_ENA_W::new(self, 2)
    }
    ///Bit 3 - 1: enabled, if the data overrun status bit is set (see status register. Table 14), the TWAI controllerrequests the respective interrupt. 0: disable
    #[inline(always)]
    #[must_use]
    pub fn ext_data_overrun_int_ena(
        &mut self,
    ) -> EXT_DATA_OVERRUN_INT_ENA_W<INTERRUPT_ENABLE_SPEC> {
        EXT_DATA_OVERRUN_INT_ENA_W::new(self, 3)
    }
    ///Bit 4 - enable the timestamp counter overflow interrupt request.
    #[inline(always)]
    #[must_use]
    pub fn ts_counter_ovfl_int_ena(&mut self) -> TS_COUNTER_OVFL_INT_ENA_W<INTERRUPT_ENABLE_SPEC> {
        TS_COUNTER_OVFL_INT_ENA_W::new(self, 4)
    }
    ///Bit 5 - 1: enabled, if the error status of the TWAI controller changes from error active to error passive or vice versa, the respective interrupt is requested. 0: disable
    #[inline(always)]
    #[must_use]
    pub fn err_passive_int_ena(&mut self) -> ERR_PASSIVE_INT_ENA_W<INTERRUPT_ENABLE_SPEC> {
        ERR_PASSIVE_INT_ENA_W::new(self, 5)
    }
    ///Bit 6 - 1: enabled, if the TWAI controller has lost arbitration, the respective interrupt is requested. 0: disable
    #[inline(always)]
    #[must_use]
    pub fn arbitration_lost_int_ena(
        &mut self,
    ) -> ARBITRATION_LOST_INT_ENA_W<INTERRUPT_ENABLE_SPEC> {
        ARBITRATION_LOST_INT_ENA_W::new(self, 6)
    }
    ///Bit 7 - 1: enabled, if an bus error has been detected, the TWAI controller requests the respective interrupt. 0: disable
    #[inline(always)]
    #[must_use]
    pub fn bus_err_int_ena(&mut self) -> BUS_ERR_INT_ENA_W<INTERRUPT_ENABLE_SPEC> {
        BUS_ERR_INT_ENA_W::new(self, 7)
    }
}
/**Interrupt enable register.

You can [`read`](crate::generic::Reg::read) this register and get [`interrupt_enable::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`interrupt_enable::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct INTERRUPT_ENABLE_SPEC;
impl crate::RegisterSpec for INTERRUPT_ENABLE_SPEC {
    type Ux = u32;
}
///`read()` method returns [`interrupt_enable::R`](R) reader structure
impl crate::Readable for INTERRUPT_ENABLE_SPEC {}
///`write(|w| ..)` method takes [`interrupt_enable::W`](W) writer structure
impl crate::Writable for INTERRUPT_ENABLE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets INTERRUPT_ENABLE to value 0
impl crate::Resettable for INTERRUPT_ENABLE_SPEC {
    const RESET_VALUE: u32 = 0;
}
