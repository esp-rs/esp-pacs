#[doc = "Register `IV_%s` reader"]
pub struct R(crate::R<IV__SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IV__SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IV__SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IV__SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IV_%s` writer"]
pub struct W(crate::W<IV__SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IV__SPEC>;
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
impl From<crate::W<IV__SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IV__SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IV_0` reader - Stores IV block data"]
pub type IV_0_R = crate::FieldReader<u32, u32>;
#[doc = "Field `IV_0` writer - Stores IV block data"]
pub type IV_0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IV__SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Stores IV block data"]
    #[inline(always)]
    pub fn iv_0(&self) -> IV_0_R {
        IV_0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Stores IV block data"]
    #[inline(always)]
    pub fn iv_0(&mut self) -> IV_0_W<0> {
        IV_0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IV block data\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iv_](index.html) module"]
pub struct IV__SPEC;
impl crate::RegisterSpec for IV__SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iv_::R](R) reader structure"]
impl crate::Readable for IV__SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iv_::W](W) writer structure"]
impl crate::Writable for IV__SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IV_%s to value 0"]
impl crate::Resettable for IV__SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
