#[doc = "Register `CONTINUE_OP` writer"]
pub struct W(crate::W<CONTINUE_OP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONTINUE_OP_SPEC>;
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
impl From<crate::W<CONTINUE_OP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONTINUE_OP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CONTINUE_OP` writer - Write 1 to continue Typical SHA calculation."]
pub type CONTINUE_OP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONTINUE_OP_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Write 1 to continue Typical SHA calculation."]
    #[inline(always)]
    pub fn continue_op(&mut self) -> CONTINUE_OP_W<0> {
        CONTINUE_OP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Continues SHA operation (only effective in Typical SHA mode)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [continue_op](index.html) module"]
pub struct CONTINUE_OP_SPEC;
impl crate::RegisterSpec for CONTINUE_OP_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [continue_op::W](W) writer structure"]
impl crate::Writable for CONTINUE_OP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CONTINUE_OP to value 0"]
impl crate::Resettable for CONTINUE_OP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
