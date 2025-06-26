#[doc = "Register `IN_STATE_CH%s` reader"]
pub type R = crate::R<IN_STATE_CH_SPEC>;
#[doc = "Field `INLINK_DSCR_ADDR` reader - Represents the address of the lower 18 bits of the next receive descriptor to be processed."]
pub type INLINK_DSCR_ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `IN_DSCR_STATE` reader - reserved"]
pub type IN_DSCR_STATE_R = crate::FieldReader;
#[doc = "Field `IN_STATE` reader - reserved"]
pub type IN_STATE_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:17 - Represents the address of the lower 18 bits of the next receive descriptor to be processed."]
    #[inline(always)]
    pub fn inlink_dscr_addr(&self) -> INLINK_DSCR_ADDR_R {
        INLINK_DSCR_ADDR_R::new(self.bits & 0x0003_ffff)
    }
    #[doc = "Bits 18:19 - reserved"]
    #[inline(always)]
    pub fn in_dscr_state(&self) -> IN_DSCR_STATE_R {
        IN_DSCR_STATE_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:22 - reserved"]
    #[inline(always)]
    pub fn in_state(&self) -> IN_STATE_R {
        IN_STATE_R::new(((self.bits >> 20) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IN_STATE_CH")
            .field("inlink_dscr_addr", &self.inlink_dscr_addr())
            .field("in_dscr_state", &self.in_dscr_state())
            .field("in_state", &self.in_state())
            .finish()
    }
}
#[doc = "Receive status of RX channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`in_state_ch::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IN_STATE_CH_SPEC;
impl crate::RegisterSpec for IN_STATE_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in_state_ch::R`](R) reader structure"]
impl crate::Readable for IN_STATE_CH_SPEC {}
#[doc = "`reset()` method sets IN_STATE_CH%s to value 0"]
impl crate::Resettable for IN_STATE_CH_SPEC {
    const RESET_VALUE: u32 = 0;
}
