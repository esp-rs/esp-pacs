#[doc = "Register `CMD11` reader"]
pub struct R(crate::R<CMD11_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMD11_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMD11_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMD11_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMD11` writer"]
pub struct W(crate::W<CMD11_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMD11_SPEC>;
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
impl From<crate::W<CMD11_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMD11_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMMAND11` reader - command11"]
pub type COMMAND11_R = crate::FieldReader<u16, u16>;
#[doc = "Field `COMMAND11` writer - command11"]
pub type COMMAND11_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CMD11_SPEC, u16, u16, 14, O>;
#[doc = "Field `COMMAND11_DONE` reader - command11_done"]
pub type COMMAND11_DONE_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:13 - command11"]
    #[inline(always)]
    pub fn command11(&self) -> COMMAND11_R {
        COMMAND11_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 31 - command11_done"]
    #[inline(always)]
    pub fn command11_done(&self) -> COMMAND11_DONE_R {
        COMMAND11_DONE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:13 - command11"]
    #[inline(always)]
    #[must_use]
    pub fn command11(&mut self) -> COMMAND11_W<0> {
        COMMAND11_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "i2c commond11 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd11](index.html) module"]
pub struct CMD11_SPEC;
impl crate::RegisterSpec for CMD11_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmd11::R](R) reader structure"]
impl crate::Readable for CMD11_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmd11::W](W) writer structure"]
impl crate::Writable for CMD11_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMD11 to value 0x0901"]
impl crate::Resettable for CMD11_SPEC {
    const RESET_VALUE: Self::Ux = 0x0901;
}
