#[doc = "Register `REG_Q5_WORD1` reader"]
pub struct R(crate::R<REG_Q5_WORD1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REG_Q5_WORD1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REG_Q5_WORD1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REG_Q5_WORD1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REG_Q5_WORD1` writer"]
pub struct W(crate::W<REG_Q5_WORD1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REG_Q5_WORD1_SPEC>;
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
impl From<crate::W<REG_Q5_WORD1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REG_Q5_WORD1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEND_Q5_WORD1` reader - a"]
pub type SEND_Q5_WORD1_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SEND_Q5_WORD1` writer - a"]
pub type SEND_Q5_WORD1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, REG_Q5_WORD1_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - a"]
    #[inline(always)]
    pub fn send_q5_word1(&self) -> SEND_Q5_WORD1_R {
        SEND_Q5_WORD1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - a"]
    #[inline(always)]
    #[must_use]
    pub fn send_q5_word1(&mut self) -> SEND_Q5_WORD1_W<0> {
        SEND_Q5_WORD1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "a\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg_q5_word1](index.html) module"]
pub struct REG_Q5_WORD1_SPEC;
impl crate::RegisterSpec for REG_Q5_WORD1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reg_q5_word1::R](R) reader structure"]
impl crate::Readable for REG_Q5_WORD1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [reg_q5_word1::W](W) writer structure"]
impl crate::Writable for REG_Q5_WORD1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REG_Q5_WORD1 to value 0"]
impl crate::Resettable for REG_Q5_WORD1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}