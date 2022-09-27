#[doc = "Register `SET_INVALIDATE_JTAG` writer"]
pub struct W(crate::W<SET_INVALIDATE_JTAG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SET_INVALIDATE_JTAG_SPEC>;
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
impl From<crate::W<SET_INVALIDATE_JTAG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SET_INVALIDATE_JTAG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SET_INVALIDATE_JTAG` writer - Set this bit to clear calculation results in JTAG re-enable function under downstream mode."]
pub type SET_INVALIDATE_JTAG_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SET_INVALIDATE_JTAG_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Set this bit to clear calculation results in JTAG re-enable function under downstream mode."]
    #[inline(always)]
    pub fn set_invalidate_jtag(&mut self) -> SET_INVALIDATE_JTAG_W<0> {
        SET_INVALIDATE_JTAG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Invalidate JTAG result register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [set_invalidate_jtag](index.html) module"]
pub struct SET_INVALIDATE_JTAG_SPEC;
impl crate::RegisterSpec for SET_INVALIDATE_JTAG_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [set_invalidate_jtag::W](W) writer structure"]
impl crate::Writable for SET_INVALIDATE_JTAG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SET_INVALIDATE_JTAG to value 0"]
impl crate::Resettable for SET_INVALIDATE_JTAG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
