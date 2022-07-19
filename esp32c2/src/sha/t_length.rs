#[doc = "Register `T_LENGTH` reader"]
pub struct R(crate::R<T_LENGTH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<T_LENGTH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<T_LENGTH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<T_LENGTH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `T_LENGTH` writer"]
pub struct W(crate::W<T_LENGTH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<T_LENGTH_SPEC>;
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
impl From<crate::W<T_LENGTH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<T_LENGTH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `T_LENGTH` reader - Sha t_length (used if and only if mode == SHA_512/t)."]
pub type T_LENGTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `T_LENGTH` writer - Sha t_length (used if and only if mode == SHA_512/t)."]
pub type T_LENGTH_W<'a> = crate::FieldWriter<'a, u32, T_LENGTH_SPEC, u8, u8, 6, 0>;
impl R {
    #[doc = "Bits 0:5 - Sha t_length (used if and only if mode == SHA_512/t)."]
    #[inline(always)]
    pub fn t_length(&self) -> T_LENGTH_R {
        T_LENGTH_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Sha t_length (used if and only if mode == SHA_512/t)."]
    #[inline(always)]
    pub fn t_length(&mut self) -> T_LENGTH_W {
        T_LENGTH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SHA 512/t configuration register 1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t_length](index.html) module"]
pub struct T_LENGTH_SPEC;
impl crate::RegisterSpec for T_LENGTH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [t_length::R](R) reader structure"]
impl crate::Readable for T_LENGTH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [t_length::W](W) writer structure"]
impl crate::Writable for T_LENGTH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets T_LENGTH to value 0"]
impl crate::Resettable for T_LENGTH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
