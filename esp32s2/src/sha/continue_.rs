#[doc = "Register `CONTINUE` writer"]
pub type W = crate::W<CONTINUE_SPEC>;
#[doc = "Field `CONTINUE_OP` writer - Write 1 to continue Typical SHA calculation."]
pub type CONTINUE_OP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CONTINUE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Write 1 to continue Typical SHA calculation."]
    #[inline(always)]
    #[must_use]
    pub fn continue_op(&mut self) -> CONTINUE_OP_W<CONTINUE_SPEC, 0> {
        CONTINUE_OP_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Continues SHA operation (only effective in Typical SHA mode)\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`continue_::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONTINUE_SPEC;
impl crate::RegisterSpec for CONTINUE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`continue_::W`](W) writer structure"]
impl crate::Writable for CONTINUE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONTINUE to value 0"]
impl crate::Resettable for CONTINUE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
