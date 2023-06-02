#[doc = "Register `INTERRUPT_ENABLE` reader"]
pub struct R(crate::R<INTERRUPT_ENABLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTERRUPT_ENABLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTERRUPT_ENABLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTERRUPT_ENABLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTERRUPT_ENABLE` writer"]
pub struct W(crate::W<INTERRUPT_ENABLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTERRUPT_ENABLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<INTERRUPT_ENABLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTERRUPT_ENABLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EXT_RECEIVE_INT_ENA` reader - 1: enabled, when the receive buffer status is 'full' the TWAI controller requests the respective interrupt. 0: disable"]
pub type EXT_RECEIVE_INT_ENA_R = crate::BitReader;
#[doc = "Field `EXT_RECEIVE_INT_ENA` writer - 1: enabled, when the receive buffer status is 'full' the TWAI controller requests the respective interrupt. 0: disable"]
pub type EXT_RECEIVE_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, INTERRUPT_ENABLE_SPEC, O>;
#[doc = "Field `EXT_TRANSMIT_INT_ENA` reader - 1: enabled, when a message has been successfully transmitted or the transmit buffer is accessible again (e.g. after an abort transmission command), the TWAI controller requests the respective interrupt. 0: disable"]
pub type EXT_TRANSMIT_INT_ENA_R = crate::BitReader;
#[doc = "Field `EXT_TRANSMIT_INT_ENA` writer - 1: enabled, when a message has been successfully transmitted or the transmit buffer is accessible again (e.g. after an abort transmission command), the TWAI controller requests the respective interrupt. 0: disable"]
pub type EXT_TRANSMIT_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, INTERRUPT_ENABLE_SPEC, O>;
#[doc = "Field `EXT_ERR_WARNING_INT_ENA` reader - 1: enabled, if the error or bus status change (see status register. Table 14), the TWAI controllerrequests the respective interrupt. 0: disable"]
pub type EXT_ERR_WARNING_INT_ENA_R = crate::BitReader;
#[doc = "Field `EXT_ERR_WARNING_INT_ENA` writer - 1: enabled, if the error or bus status change (see status register. Table 14), the TWAI controllerrequests the respective interrupt. 0: disable"]
pub type EXT_ERR_WARNING_INT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, INTERRUPT_ENABLE_SPEC, O>;
#[doc = "Field `EXT_DATA_OVERRUN_INT_ENA` reader - 1: enabled, if the data overrun status bit is set (see status register. Table 14), the TWAI controllerrequests the respective interrupt. 0: disable"]
pub type EXT_DATA_OVERRUN_INT_ENA_R = crate::BitReader;
#[doc = "Field `EXT_DATA_OVERRUN_INT_ENA` writer - 1: enabled, if the data overrun status bit is set (see status register. Table 14), the TWAI controllerrequests the respective interrupt. 0: disable"]
pub type EXT_DATA_OVERRUN_INT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, INTERRUPT_ENABLE_SPEC, O>;
#[doc = "Field `ERR_PASSIVE_INT_ENA` reader - 1: enabled, if the error status of the TWAI controller changes from error active to error passive or vice versa, the respective interrupt is requested. 0: disable"]
pub type ERR_PASSIVE_INT_ENA_R = crate::BitReader;
#[doc = "Field `ERR_PASSIVE_INT_ENA` writer - 1: enabled, if the error status of the TWAI controller changes from error active to error passive or vice versa, the respective interrupt is requested. 0: disable"]
pub type ERR_PASSIVE_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, INTERRUPT_ENABLE_SPEC, O>;
#[doc = "Field `ARBITRATION_LOST_INT_ENA` reader - 1: enabled, if the TWAI controller has lost arbitration, the respective interrupt is requested. 0: disable"]
pub type ARBITRATION_LOST_INT_ENA_R = crate::BitReader;
#[doc = "Field `ARBITRATION_LOST_INT_ENA` writer - 1: enabled, if the TWAI controller has lost arbitration, the respective interrupt is requested. 0: disable"]
pub type ARBITRATION_LOST_INT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, INTERRUPT_ENABLE_SPEC, O>;
#[doc = "Field `BUS_ERR_INT_ENA` reader - 1: enabled, if an bus error has been detected, the TWAI controller requests the respective interrupt. 0: disable"]
pub type BUS_ERR_INT_ENA_R = crate::BitReader;
#[doc = "Field `BUS_ERR_INT_ENA` writer - 1: enabled, if an bus error has been detected, the TWAI controller requests the respective interrupt. 0: disable"]
pub type BUS_ERR_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, INTERRUPT_ENABLE_SPEC, O>;
#[doc = "Field `IDLE_INT_ENA` reader - 1: enabled, if state of TWAI become IDLE, the TWAI controller requests the respective interrupt. 0: disable"]
pub type IDLE_INT_ENA_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - 1: enabled, when the receive buffer status is 'full' the TWAI controller requests the respective interrupt. 0: disable"]
    #[inline(always)]
    pub fn ext_receive_int_ena(&self) -> EXT_RECEIVE_INT_ENA_R {
        EXT_RECEIVE_INT_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1: enabled, when a message has been successfully transmitted or the transmit buffer is accessible again (e.g. after an abort transmission command), the TWAI controller requests the respective interrupt. 0: disable"]
    #[inline(always)]
    pub fn ext_transmit_int_ena(&self) -> EXT_TRANSMIT_INT_ENA_R {
        EXT_TRANSMIT_INT_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 1: enabled, if the error or bus status change (see status register. Table 14), the TWAI controllerrequests the respective interrupt. 0: disable"]
    #[inline(always)]
    pub fn ext_err_warning_int_ena(&self) -> EXT_ERR_WARNING_INT_ENA_R {
        EXT_ERR_WARNING_INT_ENA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 1: enabled, if the data overrun status bit is set (see status register. Table 14), the TWAI controllerrequests the respective interrupt. 0: disable"]
    #[inline(always)]
    pub fn ext_data_overrun_int_ena(&self) -> EXT_DATA_OVERRUN_INT_ENA_R {
        EXT_DATA_OVERRUN_INT_ENA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - 1: enabled, if the error status of the TWAI controller changes from error active to error passive or vice versa, the respective interrupt is requested. 0: disable"]
    #[inline(always)]
    pub fn err_passive_int_ena(&self) -> ERR_PASSIVE_INT_ENA_R {
        ERR_PASSIVE_INT_ENA_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 1: enabled, if the TWAI controller has lost arbitration, the respective interrupt is requested. 0: disable"]
    #[inline(always)]
    pub fn arbitration_lost_int_ena(&self) -> ARBITRATION_LOST_INT_ENA_R {
        ARBITRATION_LOST_INT_ENA_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 1: enabled, if an bus error has been detected, the TWAI controller requests the respective interrupt. 0: disable"]
    #[inline(always)]
    pub fn bus_err_int_ena(&self) -> BUS_ERR_INT_ENA_R {
        BUS_ERR_INT_ENA_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 1: enabled, if state of TWAI become IDLE, the TWAI controller requests the respective interrupt. 0: disable"]
    #[inline(always)]
    pub fn idle_int_ena(&self) -> IDLE_INT_ENA_R {
        IDLE_INT_ENA_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTERRUPT_ENABLE")
            .field(
                "ext_receive_int_ena",
                &format_args!("{}", self.ext_receive_int_ena().bit()),
            )
            .field(
                "ext_transmit_int_ena",
                &format_args!("{}", self.ext_transmit_int_ena().bit()),
            )
            .field(
                "ext_err_warning_int_ena",
                &format_args!("{}", self.ext_err_warning_int_ena().bit()),
            )
            .field(
                "ext_data_overrun_int_ena",
                &format_args!("{}", self.ext_data_overrun_int_ena().bit()),
            )
            .field(
                "err_passive_int_ena",
                &format_args!("{}", self.err_passive_int_ena().bit()),
            )
            .field(
                "arbitration_lost_int_ena",
                &format_args!("{}", self.arbitration_lost_int_ena().bit()),
            )
            .field(
                "bus_err_int_ena",
                &format_args!("{}", self.bus_err_int_ena().bit()),
            )
            .field(
                "idle_int_ena",
                &format_args!("{}", self.idle_int_ena().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INTERRUPT_ENABLE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - 1: enabled, when the receive buffer status is 'full' the TWAI controller requests the respective interrupt. 0: disable"]
    #[inline(always)]
    #[must_use]
    pub fn ext_receive_int_ena(&mut self) -> EXT_RECEIVE_INT_ENA_W<0> {
        EXT_RECEIVE_INT_ENA_W::new(self)
    }
    #[doc = "Bit 1 - 1: enabled, when a message has been successfully transmitted or the transmit buffer is accessible again (e.g. after an abort transmission command), the TWAI controller requests the respective interrupt. 0: disable"]
    #[inline(always)]
    #[must_use]
    pub fn ext_transmit_int_ena(&mut self) -> EXT_TRANSMIT_INT_ENA_W<1> {
        EXT_TRANSMIT_INT_ENA_W::new(self)
    }
    #[doc = "Bit 2 - 1: enabled, if the error or bus status change (see status register. Table 14), the TWAI controllerrequests the respective interrupt. 0: disable"]
    #[inline(always)]
    #[must_use]
    pub fn ext_err_warning_int_ena(&mut self) -> EXT_ERR_WARNING_INT_ENA_W<2> {
        EXT_ERR_WARNING_INT_ENA_W::new(self)
    }
    #[doc = "Bit 3 - 1: enabled, if the data overrun status bit is set (see status register. Table 14), the TWAI controllerrequests the respective interrupt. 0: disable"]
    #[inline(always)]
    #[must_use]
    pub fn ext_data_overrun_int_ena(&mut self) -> EXT_DATA_OVERRUN_INT_ENA_W<3> {
        EXT_DATA_OVERRUN_INT_ENA_W::new(self)
    }
    #[doc = "Bit 5 - 1: enabled, if the error status of the TWAI controller changes from error active to error passive or vice versa, the respective interrupt is requested. 0: disable"]
    #[inline(always)]
    #[must_use]
    pub fn err_passive_int_ena(&mut self) -> ERR_PASSIVE_INT_ENA_W<5> {
        ERR_PASSIVE_INT_ENA_W::new(self)
    }
    #[doc = "Bit 6 - 1: enabled, if the TWAI controller has lost arbitration, the respective interrupt is requested. 0: disable"]
    #[inline(always)]
    #[must_use]
    pub fn arbitration_lost_int_ena(&mut self) -> ARBITRATION_LOST_INT_ENA_W<6> {
        ARBITRATION_LOST_INT_ENA_W::new(self)
    }
    #[doc = "Bit 7 - 1: enabled, if an bus error has been detected, the TWAI controller requests the respective interrupt. 0: disable"]
    #[inline(always)]
    #[must_use]
    pub fn bus_err_int_ena(&mut self) -> BUS_ERR_INT_ENA_W<7> {
        BUS_ERR_INT_ENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt enable register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_enable](index.html) module"]
pub struct INTERRUPT_ENABLE_SPEC;
impl crate::RegisterSpec for INTERRUPT_ENABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [interrupt_enable::R](R) reader structure"]
impl crate::Readable for INTERRUPT_ENABLE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [interrupt_enable::W](W) writer structure"]
impl crate::Writable for INTERRUPT_ENABLE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTERRUPT_ENABLE to value 0"]
impl crate::Resettable for INTERRUPT_ENABLE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
