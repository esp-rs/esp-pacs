#[doc = "Register `M2_STATUS_CLR` writer"]
pub type W = crate::W<M2_STATUS_CLR_SPEC>;
#[doc = "Field `M2_EXCEPTION_STATUS_CLR` writer - Configures to clear exception status."]
pub type M2_EXCEPTION_STATUS_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<M2_STATUS_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Configures to clear exception status."]
    #[inline(always)]
    pub fn m2_exception_status_clr(&mut self) -> M2_EXCEPTION_STATUS_CLR_W<M2_STATUS_CLR_SPEC> {
        M2_EXCEPTION_STATUS_CLR_W::new(self, 0)
    }
}
#[doc = "M2 status clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m2_status_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct M2_STATUS_CLR_SPEC;
impl crate::RegisterSpec for M2_STATUS_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`m2_status_clr::W`](W) writer structure"]
impl crate::Writable for M2_STATUS_CLR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets M2_STATUS_CLR to value 0"]
impl crate::Resettable for M2_STATUS_CLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
