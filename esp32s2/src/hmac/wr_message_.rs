#[doc = "Register `WR_MESSAGE_%s` writer"]
pub struct W(crate::W<WR_MESSAGE__SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WR_MESSAGE__SPEC>;
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
impl From<crate::W<WR_MESSAGE__SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WR_MESSAGE__SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDATA` writer - Store the %sth 32-bit of message."]
pub type WDATA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WR_MESSAGE__SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31 - Store the %sth 32-bit of message."]
    #[inline(always)]
    pub fn wdata(&mut self) -> WDATA_W<0> {
        WDATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Message register %s\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wr_message_](index.html) module"]
pub struct WR_MESSAGE__SPEC;
impl crate::RegisterSpec for WR_MESSAGE__SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [wr_message_::W](W) writer structure"]
impl crate::Writable for WR_MESSAGE__SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WR_MESSAGE_%s to value 0"]
impl crate::Resettable for WR_MESSAGE__SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
