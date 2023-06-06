#[doc = "Register `WR_JTAG` writer"]
pub struct W(crate::W<WR_JTAG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WR_JTAG_SPEC>;
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
impl From<crate::W<WR_JTAG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WR_JTAG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WR_JTAG` writer - 32-bit of key to be compared."]
pub type WR_JTAG_W<'a, const O: u8> = crate::FieldWriter<'a, WR_JTAG_SPEC, 32, O, u32>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<WR_JTAG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:31 - 32-bit of key to be compared."]
    #[inline(always)]
    #[must_use]
    pub fn wr_jtag(&mut self) -> WR_JTAG_W<0> {
        WR_JTAG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Jtag register 1.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wr_jtag](index.html) module"]
pub struct WR_JTAG_SPEC;
impl crate::RegisterSpec for WR_JTAG_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [wr_jtag::W](W) writer structure"]
impl crate::Writable for WR_JTAG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WR_JTAG to value 0"]
impl crate::Resettable for WR_JTAG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
