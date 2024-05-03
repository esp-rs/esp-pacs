#[doc = "Register `STATE` reader"]
pub type R = crate::R<STATE_SPEC>;
#[doc = "Field `IN_STATE` reader - This register stores the current control module state machine state."]
pub type IN_STATE_R = crate::FieldReader;
#[doc = "Field `IN_RESET_AVAIL` reader - This register indicate that if the channel reset is safety."]
pub type IN_RESET_AVAIL_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:2 - This register stores the current control module state machine state."]
    #[inline(always)]
    pub fn in_state(&self) -> IN_STATE_R {
        IN_STATE_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - This register indicate that if the channel reset is safety."]
    #[inline(always)]
    pub fn in_reset_avail(&self) -> IN_RESET_AVAIL_R {
        IN_RESET_AVAIL_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATE")
            .field("in_state", &self.in_state().bits())
            .field("in_reset_avail", &self.in_reset_avail().bit())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<STATE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "RX CH5 state register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`state::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATE_SPEC;
impl crate::RegisterSpec for STATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`state::R`](R) reader structure"]
impl crate::Readable for STATE_SPEC {}
#[doc = "`reset()` method sets STATE to value 0x08"]
impl crate::Resettable for STATE_SPEC {
    const RESET_VALUE: u32 = 0x08;
}
