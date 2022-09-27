#[doc = "Register `CLKSRC` reader"]
pub struct R(crate::R<CLKSRC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLKSRC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLKSRC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLKSRC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLKSRC` writer"]
pub struct W(crate::W<CLKSRC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLKSRC_SPEC>;
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
impl From<crate::W<CLKSRC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLKSRC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLKSRC` reader - Clock divider source for two SD cards is supported. Each card has two bits assigned to it. For example, bit\\[1:0\\] are assigned for card 0, bit\\[3:2\\] are assigned for card 1. Card 0 maps and internally routes clock divider\\[0:3\\] outputs to cclk_out\\[1:0\\] pins, depending on bit value. 00 : Clock divider 0; 01 : Clock divider 1; 10 : Clock divider 2; 11 : Clock divider 3."]
pub type CLKSRC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CLKSRC` writer - Clock divider source for two SD cards is supported. Each card has two bits assigned to it. For example, bit\\[1:0\\] are assigned for card 0, bit\\[3:2\\] are assigned for card 1. Card 0 maps and internally routes clock divider\\[0:3\\] outputs to cclk_out\\[1:0\\] pins, depending on bit value. 00 : Clock divider 0; 01 : Clock divider 1; 10 : Clock divider 2; 11 : Clock divider 3."]
pub type CLKSRC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLKSRC_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Clock divider source for two SD cards is supported. Each card has two bits assigned to it. For example, bit\\[1:0\\] are assigned for card 0, bit\\[3:2\\] are assigned for card 1. Card 0 maps and internally routes clock divider\\[0:3\\] outputs to cclk_out\\[1:0\\] pins, depending on bit value. 00 : Clock divider 0; 01 : Clock divider 1; 10 : Clock divider 2; 11 : Clock divider 3."]
    #[inline(always)]
    pub fn clksrc(&self) -> CLKSRC_R {
        CLKSRC_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Clock divider source for two SD cards is supported. Each card has two bits assigned to it. For example, bit\\[1:0\\] are assigned for card 0, bit\\[3:2\\] are assigned for card 1. Card 0 maps and internally routes clock divider\\[0:3\\] outputs to cclk_out\\[1:0\\] pins, depending on bit value. 00 : Clock divider 0; 01 : Clock divider 1; 10 : Clock divider 2; 11 : Clock divider 3."]
    #[inline(always)]
    pub fn clksrc(&mut self) -> CLKSRC_W<0> {
        CLKSRC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock source selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clksrc](index.html) module"]
pub struct CLKSRC_SPEC;
impl crate::RegisterSpec for CLKSRC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clksrc::R](R) reader structure"]
impl crate::Readable for CLKSRC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clksrc::W](W) writer structure"]
impl crate::Writable for CLKSRC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLKSRC to value 0"]
impl crate::Resettable for CLKSRC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
