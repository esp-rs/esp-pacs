#[doc = "Register `IN_STATE_CH5` reader"]
pub type R = crate::R<IN_STATE_CH5_SPEC>;
#[doc = "Field `IN_STATE_CH5` reader - This register stores the current control module state machine state."]
pub type IN_STATE_CH5_R = crate::FieldReader;
#[doc = "Field `IN_RESET_AVAIL_CH5` reader - This register indicate that if the channel reset is safety."]
pub type IN_RESET_AVAIL_CH5_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:2 - This register stores the current control module state machine state."]
    #[inline(always)]
    pub fn in_state_ch5(&self) -> IN_STATE_CH5_R {
        IN_STATE_CH5_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - This register indicate that if the channel reset is safety."]
    #[inline(always)]
    pub fn in_reset_avail_ch5(&self) -> IN_RESET_AVAIL_CH5_R {
        IN_RESET_AVAIL_CH5_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IN_STATE_CH5")
            .field(
                "in_state_ch5",
                &format_args!("{}", self.in_state_ch5().bits()),
            )
            .field(
                "in_reset_avail_ch5",
                &format_args!("{}", self.in_reset_avail_ch5().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<IN_STATE_CH5_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "RX CH5 state register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_state_ch5::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IN_STATE_CH5_SPEC;
impl crate::RegisterSpec for IN_STATE_CH5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in_state_ch5::R`](R) reader structure"]
impl crate::Readable for IN_STATE_CH5_SPEC {}
#[doc = "`reset()` method sets IN_STATE_CH5 to value 0x08"]
impl crate::Resettable for IN_STATE_CH5_SPEC {
    const RESET_VALUE: Self::Ux = 0x08;
}
