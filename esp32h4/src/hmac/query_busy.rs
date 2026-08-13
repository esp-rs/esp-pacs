#[doc = "Register `QUERY_BUSY` reader"]
pub type R = crate::R<QUERY_BUSY_SPEC>;
#[doc = "Field `BUSY_STATE` reader - Represents whether or not HMAC is in a busy state. Before configuring HMAC, please make sure HMAC is in an IDLE state. \\\\0: Idle \\\\1: HMAC is still working on the calculation"]
pub type BUSY_STATE_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Represents whether or not HMAC is in a busy state. Before configuring HMAC, please make sure HMAC is in an IDLE state. \\\\0: Idle \\\\1: HMAC is still working on the calculation"]
    #[inline(always)]
    pub fn busy_state(&self) -> BUSY_STATE_R {
        BUSY_STATE_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("QUERY_BUSY")
            .field("busy_state", &self.busy_state())
            .finish()
    }
}
#[doc = "Busy state of HMAC module\n\nYou can [`read`](crate::Reg::read) this register and get [`query_busy::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QUERY_BUSY_SPEC;
impl crate::RegisterSpec for QUERY_BUSY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`query_busy::R`](R) reader structure"]
impl crate::Readable for QUERY_BUSY_SPEC {}
#[doc = "`reset()` method sets QUERY_BUSY to value 0"]
impl crate::Resettable for QUERY_BUSY_SPEC {}
