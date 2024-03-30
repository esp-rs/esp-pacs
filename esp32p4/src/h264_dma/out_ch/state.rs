#[doc = "Register `STATE` reader"]
pub type R = crate::R<STATE_SPEC>;
#[doc = "Field `OUTLINK_DSCR_ADDR` reader - This register stores the current outlink descriptor's address."]
pub type OUTLINK_DSCR_ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `OUT_DSCR_STATE` reader - This register stores the current descriptor state machine state."]
pub type OUT_DSCR_STATE_R = crate::FieldReader;
#[doc = "Field `OUT_STATE` reader - This register stores the current control module state machine state."]
pub type OUT_STATE_R = crate::FieldReader;
#[doc = "Field `OUT_RESET_AVAIL` reader - This register indicate that if the channel reset is safety."]
pub type OUT_RESET_AVAIL_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:17 - This register stores the current outlink descriptor's address."]
    #[inline(always)]
    pub fn outlink_dscr_addr(&self) -> OUTLINK_DSCR_ADDR_R {
        OUTLINK_DSCR_ADDR_R::new(self.bits & 0x0003_ffff)
    }
    #[doc = "Bits 18:19 - This register stores the current descriptor state machine state."]
    #[inline(always)]
    pub fn out_dscr_state(&self) -> OUT_DSCR_STATE_R {
        OUT_DSCR_STATE_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:23 - This register stores the current control module state machine state."]
    #[inline(always)]
    pub fn out_state(&self) -> OUT_STATE_R {
        OUT_STATE_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - This register indicate that if the channel reset is safety."]
    #[inline(always)]
    pub fn out_reset_avail(&self) -> OUT_RESET_AVAIL_R {
        OUT_RESET_AVAIL_R::new(((self.bits >> 24) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATE")
            .field(
                "outlink_dscr_addr",
                &format_args!("{}", self.outlink_dscr_addr().bits()),
            )
            .field(
                "out_dscr_state",
                &format_args!("{}", self.out_dscr_state().bits()),
            )
            .field("out_state", &format_args!("{}", self.out_state().bits()))
            .field(
                "out_reset_avail",
                &format_args!("{}", self.out_reset_avail().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<STATE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "TX CHx state register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`state::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATE_SPEC;
impl crate::RegisterSpec for STATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`state::R`](R) reader structure"]
impl crate::Readable for STATE_SPEC {}
#[doc = "`reset()` method sets STATE to value 0x0100_0000"]
impl crate::Resettable for STATE_SPEC {
    const RESET_VALUE: u32 = 0x0100_0000;
}
