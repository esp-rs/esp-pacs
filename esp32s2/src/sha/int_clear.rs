#[doc = "Register `INT_CLEAR` writer"]
pub struct W(crate::W<INT_CLEAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_CLEAR_SPEC>;
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
impl From<crate::W<INT_CLEAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_CLEAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLEAR_INTERRUPT` writer - Clears DMA-SHA interrupt."]
pub type CLEAR_INTERRUPT_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLEAR_SPEC, O>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_CLEAR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Clears DMA-SHA interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn clear_interrupt(&mut self) -> CLEAR_INTERRUPT_W<0> {
        CLEAR_INTERRUPT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA-SHA interrupt clear register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_clear](index.html) module"]
pub struct INT_CLEAR_SPEC;
impl crate::RegisterSpec for INT_CLEAR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [int_clear::W](W) writer structure"]
impl crate::Writable for INT_CLEAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INT_CLEAR to value 0"]
impl crate::Resettable for INT_CLEAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
