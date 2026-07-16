#[doc = "Register `APPWR_CAUSE_CLR` writer"]
pub type W = crate::W<APPWR_CAUSE_CLR_SPEC>;
#[doc = "Field `APPWR_REJECT_CAUSE_CLR` writer - reject cause clear signal for appwr"]
pub type APPWR_REJECT_CAUSE_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APPWR_WAKEUP_CAUSE_CLR` writer - wakeup cause clear signal for appwr"]
pub type APPWR_WAKEUP_CAUSE_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<APPWR_CAUSE_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - reject cause clear signal for appwr"]
    #[inline(always)]
    pub fn appwr_reject_cause_clr(&mut self) -> APPWR_REJECT_CAUSE_CLR_W<'_, APPWR_CAUSE_CLR_SPEC> {
        APPWR_REJECT_CAUSE_CLR_W::new(self, 0)
    }
    #[doc = "Bit 1 - wakeup cause clear signal for appwr"]
    #[inline(always)]
    pub fn appwr_wakeup_cause_clr(&mut self) -> APPWR_WAKEUP_CAUSE_CLR_W<'_, APPWR_CAUSE_CLR_SPEC> {
        APPWR_WAKEUP_CAUSE_CLR_W::new(self, 1)
    }
}
#[doc = "cause clear register for appwr\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appwr_cause_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APPWR_CAUSE_CLR_SPEC;
impl crate::RegisterSpec for APPWR_CAUSE_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`appwr_cause_clr::W`](W) writer structure"]
impl crate::Writable for APPWR_CAUSE_CLR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets APPWR_CAUSE_CLR to value 0"]
impl crate::Resettable for APPWR_CAUSE_CLR_SPEC {}
