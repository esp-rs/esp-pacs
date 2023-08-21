#[doc = "Register `EXTMEM_REJECT_INT_CLR` writer"]
pub type W = crate::W<EXTMEM_REJECT_INT_CLR_SPEC>;
#[doc = "Field `EXTMEM_REJECT_INT_CLR` writer - Set this bit to clear the EXTMEM_REJECT_INT interrupt."]
pub type EXTMEM_REJECT_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<EXTMEM_REJECT_INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to clear the EXTMEM_REJECT_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn extmem_reject_int_clr(
        &mut self,
    ) -> EXTMEM_REJECT_INT_CLR_W<EXTMEM_REJECT_INT_CLR_SPEC, 0> {
        EXTMEM_REJECT_INT_CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Interrupt clear bits of external RAM permission\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`extmem_reject_int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXTMEM_REJECT_INT_CLR_SPEC;
impl crate::RegisterSpec for EXTMEM_REJECT_INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`extmem_reject_int_clr::W`](W) writer structure"]
impl crate::Writable for EXTMEM_REJECT_INT_CLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EXTMEM_REJECT_INT_CLR to value 0"]
impl crate::Resettable for EXTMEM_REJECT_INT_CLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
