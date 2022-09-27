#[doc = "Register `SHA_INT_MAP` reader"]
pub struct R(crate::R<SHA_INT_MAP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SHA_INT_MAP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SHA_INT_MAP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SHA_INT_MAP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SHA_INT_MAP` writer"]
pub struct W(crate::W<SHA_INT_MAP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SHA_INT_MAP_SPEC>;
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
impl From<crate::W<SHA_INT_MAP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SHA_INT_MAP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SHA_INT_MAP` reader - this register used to map sha interrupt to one of core1's external interrupt"]
pub type SHA_INT_MAP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SHA_INT_MAP` writer - this register used to map sha interrupt to one of core1's external interrupt"]
pub type SHA_INT_MAP_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SHA_INT_MAP_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - this register used to map sha interrupt to one of core1's external interrupt"]
    #[inline(always)]
    pub fn sha_int_map(&self) -> SHA_INT_MAP_R {
        SHA_INT_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - this register used to map sha interrupt to one of core1's external interrupt"]
    #[inline(always)]
    pub fn sha_int_map(&mut self) -> SHA_INT_MAP_W<0> {
        SHA_INT_MAP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "sha interrupt configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sha_int_map](index.html) module"]
pub struct SHA_INT_MAP_SPEC;
impl crate::RegisterSpec for SHA_INT_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sha_int_map::R](R) reader structure"]
impl crate::Readable for SHA_INT_MAP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sha_int_map::W](W) writer structure"]
impl crate::Writable for SHA_INT_MAP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SHA_INT_MAP to value 0x10"]
impl crate::Resettable for SHA_INT_MAP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x10
    }
}
