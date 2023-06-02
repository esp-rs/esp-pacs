#[doc = "Register `INTR_CLR` writer"]
pub struct W(crate::W<INTR_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTR_CLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<INTR_CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTR_CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FIFO_OVERFLOW_INTR_CLR` writer - Set 1 clr fifo overflow interrupt"]
pub type FIFO_OVERFLOW_INTR_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INTR_CLR_SPEC, O>;
#[doc = "Field `MEM_FULL_INTR_CLR` writer - Set 1 clr mem full interrupt"]
pub type MEM_FULL_INTR_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INTR_CLR_SPEC, O>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INTR_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Set 1 clr fifo overflow interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn fifo_overflow_intr_clr(&mut self) -> FIFO_OVERFLOW_INTR_CLR_W<0> {
        FIFO_OVERFLOW_INTR_CLR_W::new(self)
    }
    #[doc = "Bit 1 - Set 1 clr mem full interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn mem_full_intr_clr(&mut self) -> MEM_FULL_INTR_CLR_W<1> {
        MEM_FULL_INTR_CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "interrupt clear register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_clr](index.html) module"]
pub struct INTR_CLR_SPEC;
impl crate::RegisterSpec for INTR_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [intr_clr::W](W) writer structure"]
impl crate::Writable for INTR_CLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTR_CLR to value 0"]
impl crate::Resettable for INTR_CLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
