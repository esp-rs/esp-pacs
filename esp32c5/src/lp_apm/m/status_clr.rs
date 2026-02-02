#[doc = "Register `STATUS_CLR` writer"]
pub type W = crate::W<STATUS_CLR_SPEC>;
#[doc = "Field `EXCEPTION_STATUS_CLR` writer - Configures to clear exception status."]
pub type EXCEPTION_STATUS_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<STATUS_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Configures to clear exception status."]
    #[inline(always)]
    pub fn exception_status_clr(&mut self) -> EXCEPTION_STATUS_CLR_W<'_, STATUS_CLR_SPEC> {
        EXCEPTION_STATUS_CLR_W::new(self, 0)
    }
}
#[doc = "LP_APM_CTRL M0 status clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUS_CLR_SPEC;
impl crate::RegisterSpec for STATUS_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`status_clr::W`](W) writer structure"]
impl crate::Writable for STATUS_CLR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STATUS_CLR to value 0"]
impl crate::Resettable for STATUS_CLR_SPEC {}
