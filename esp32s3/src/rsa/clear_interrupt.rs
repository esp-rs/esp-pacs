#[doc = "Register `CLEAR_INTERRUPT` writer"]
pub struct W(crate::W<CLEAR_INTERRUPT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLEAR_INTERRUPT_SPEC>;
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
impl From<crate::W<CLEAR_INTERRUPT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLEAR_INTERRUPT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLEAR_INTERRUPT` writer - set this bit to 1 to clear the RSA interrupt."]
pub type CLEAR_INTERRUPT_W<'a, const O: u8> = crate::BitWriter<'a, CLEAR_INTERRUPT_SPEC, O>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CLEAR_INTERRUPT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - set this bit to 1 to clear the RSA interrupt."]
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
#[doc = "RSA interrupt clear register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clear_interrupt](index.html) module"]
pub struct CLEAR_INTERRUPT_SPEC;
impl crate::RegisterSpec for CLEAR_INTERRUPT_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [clear_interrupt::W](W) writer structure"]
impl crate::Writable for CLEAR_INTERRUPT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLEAR_INTERRUPT to value 0"]
impl crate::Resettable for CLEAR_INTERRUPT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
