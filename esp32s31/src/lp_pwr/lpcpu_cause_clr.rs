#[doc = "Register `LPCPU_CAUSE_CLR` writer"]
pub type W = crate::W<LPCPU_CAUSE_CLR_SPEC>;
#[doc = "Field `LPCPU_REJECT_CAUSE_CLR` writer - reject cause clear signal for lpcpu"]
pub type LPCPU_REJECT_CAUSE_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPCPU_WAKEUP_CAUSE_CLR` writer - wakeup cause clear signal for lpcpu"]
pub type LPCPU_WAKEUP_CAUSE_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LPCPU_CAUSE_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - reject cause clear signal for lpcpu"]
    #[inline(always)]
    pub fn lpcpu_reject_cause_clr(&mut self) -> LPCPU_REJECT_CAUSE_CLR_W<'_, LPCPU_CAUSE_CLR_SPEC> {
        LPCPU_REJECT_CAUSE_CLR_W::new(self, 0)
    }
    #[doc = "Bit 1 - wakeup cause clear signal for lpcpu"]
    #[inline(always)]
    pub fn lpcpu_wakeup_cause_clr(&mut self) -> LPCPU_WAKEUP_CAUSE_CLR_W<'_, LPCPU_CAUSE_CLR_SPEC> {
        LPCPU_WAKEUP_CAUSE_CLR_W::new(self, 1)
    }
}
#[doc = "cause clear register for lpcpu\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpcpu_cause_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LPCPU_CAUSE_CLR_SPEC;
impl crate::RegisterSpec for LPCPU_CAUSE_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`lpcpu_cause_clr::W`](W) writer structure"]
impl crate::Writable for LPCPU_CAUSE_CLR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LPCPU_CAUSE_CLR to value 0"]
impl crate::Resettable for LPCPU_CAUSE_CLR_SPEC {}
