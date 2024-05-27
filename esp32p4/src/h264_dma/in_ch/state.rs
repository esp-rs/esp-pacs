///Register `STATE` reader
pub type R = crate::R<STATE_SPEC>;
///Field `INLINK_DSCR_ADDR` reader - This register stores the current inlink descriptor's address.
pub type INLINK_DSCR_ADDR_R = crate::FieldReader<u32>;
///Field `IN_DSCR_STATE` reader - This register stores the current descriptor state machine state.
pub type IN_DSCR_STATE_R = crate::FieldReader;
///Field `IN_STATE` reader - This register stores the current control module state machine state.
pub type IN_STATE_R = crate::FieldReader;
///Field `IN_RESET_AVAIL` reader - This register indicate that if the channel reset is safety.
pub type IN_RESET_AVAIL_R = crate::BitReader;
impl R {
    ///Bits 0:17 - This register stores the current inlink descriptor's address.
    #[inline(always)]
    pub fn inlink_dscr_addr(&self) -> INLINK_DSCR_ADDR_R {
        INLINK_DSCR_ADDR_R::new(self.bits & 0x0003_ffff)
    }
    ///Bits 18:19 - This register stores the current descriptor state machine state.
    #[inline(always)]
    pub fn in_dscr_state(&self) -> IN_DSCR_STATE_R {
        IN_DSCR_STATE_R::new(((self.bits >> 18) & 3) as u8)
    }
    ///Bits 20:22 - This register stores the current control module state machine state.
    #[inline(always)]
    pub fn in_state(&self) -> IN_STATE_R {
        IN_STATE_R::new(((self.bits >> 20) & 7) as u8)
    }
    ///Bit 23 - This register indicate that if the channel reset is safety.
    #[inline(always)]
    pub fn in_reset_avail(&self) -> IN_RESET_AVAIL_R {
        IN_RESET_AVAIL_R::new(((self.bits >> 23) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATE")
            .field("inlink_dscr_addr", &self.inlink_dscr_addr())
            .field("in_dscr_state", &self.in_dscr_state())
            .field("in_state", &self.in_state())
            .field("in_reset_avail", &self.in_reset_avail())
            .finish()
    }
}
/**RX CHx state register

You can [`read`](crate::generic::Reg::read) this register and get [`state::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct STATE_SPEC;
impl crate::RegisterSpec for STATE_SPEC {
    type Ux = u32;
}
///`read()` method returns [`state::R`](R) reader structure
impl crate::Readable for STATE_SPEC {}
///`reset()` method sets STATE to value 0x0080_0000
impl crate::Resettable for STATE_SPEC {
    const RESET_VALUE: u32 = 0x0080_0000;
}
