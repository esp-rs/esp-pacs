///Register `SLP_WAKEUP_CNTL0` writer
pub type W = crate::W<SLP_WAKEUP_CNTL0_SPEC>;
///Field `SLEEP_REQ` writer - need_des
pub type SLEEP_REQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SLP_WAKEUP_CNTL0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 31 - need_des
    #[inline(always)]
    #[must_use]
    pub fn sleep_req(&mut self) -> SLEEP_REQ_W<SLP_WAKEUP_CNTL0_SPEC> {
        SLEEP_REQ_W::new(self, 31)
    }
}
/**need_des

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slp_wakeup_cntl0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SLP_WAKEUP_CNTL0_SPEC;
impl crate::RegisterSpec for SLP_WAKEUP_CNTL0_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`slp_wakeup_cntl0::W`](W) writer structure
impl crate::Writable for SLP_WAKEUP_CNTL0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SLP_WAKEUP_CNTL0 to value 0
impl crate::Resettable for SLP_WAKEUP_CNTL0_SPEC {
    const RESET_VALUE: u32 = 0;
}
