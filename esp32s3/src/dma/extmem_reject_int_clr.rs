#[doc = "Register `EXTMEM_REJECT_INT_CLR` writer"]
pub type W = crate::W<EXTMEM_REJECT_INT_CLR_SPEC>;
#[doc = "Field `EXTMEM_REJECT_INT_CLR` writer - Set this bit to clear the EXTMEM_REJECT_INT interrupt."]
pub type EXTMEM_REJECT_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<EXTMEM_REJECT_INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to clear the EXTMEM_REJECT_INT interrupt."]
    #[inline(always)]
    pub fn extmem_reject_int_clr(
        &mut self,
    ) -> EXTMEM_REJECT_INT_CLR_W<'_, EXTMEM_REJECT_INT_CLR_SPEC> {
        EXTMEM_REJECT_INT_CLR_W::new(self, 0)
    }
}
#[doc = "Interrupt clear bits of external RAM permission\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`extmem_reject_int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXTMEM_REJECT_INT_CLR_SPEC;
impl crate::RegisterSpec for EXTMEM_REJECT_INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`extmem_reject_int_clr::W`](W) writer structure"]
impl crate::Writable for EXTMEM_REJECT_INT_CLR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EXTMEM_REJECT_INT_CLR to value 0"]
impl crate::Resettable for EXTMEM_REJECT_INT_CLR_SPEC {}
