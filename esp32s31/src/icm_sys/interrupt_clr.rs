#[doc = "Register `INTERRUPT_CLR` writer"]
pub type W = crate::W<INTERRUPT_CLR_SPEC>;
#[doc = "Field `REG_DEC_FAILURE_INT_CLR` writer - "]
pub type REG_DEC_FAILURE_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_TIMEOUT_INT_CLR` writer - "]
pub type REG_TIMEOUT_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INTERRUPT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_dec_failure_int_clr(&mut self) -> REG_DEC_FAILURE_INT_CLR_W<'_, INTERRUPT_CLR_SPEC> {
        REG_DEC_FAILURE_INT_CLR_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_timeout_int_clr(&mut self) -> REG_TIMEOUT_INT_CLR_W<'_, INTERRUPT_CLR_SPEC> {
        REG_TIMEOUT_INT_CLR_W::new(self, 1)
    }
}
#[doc = "\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interrupt_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTERRUPT_CLR_SPEC;
impl crate::RegisterSpec for INTERRUPT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`interrupt_clr::W`](W) writer structure"]
impl crate::Writable for INTERRUPT_CLR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTERRUPT_CLR to value 0"]
impl crate::Resettable for INTERRUPT_CLR_SPEC {}
