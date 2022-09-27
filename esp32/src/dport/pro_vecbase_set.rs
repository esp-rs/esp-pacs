#[doc = "Register `PRO_VECBASE_SET` reader"]
pub struct R(crate::R<PRO_VECBASE_SET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRO_VECBASE_SET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRO_VECBASE_SET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRO_VECBASE_SET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRO_VECBASE_SET` writer"]
pub struct W(crate::W<PRO_VECBASE_SET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRO_VECBASE_SET_SPEC>;
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
impl From<crate::W<PRO_VECBASE_SET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRO_VECBASE_SET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRO_OUT_VECBASE` reader - "]
pub type PRO_OUT_VECBASE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `PRO_OUT_VECBASE` writer - "]
pub type PRO_OUT_VECBASE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PRO_VECBASE_SET_SPEC, u32, u32, 22, O>;
impl R {
    #[doc = "Bits 0:21"]
    #[inline(always)]
    pub fn pro_out_vecbase(&self) -> PRO_OUT_VECBASE_R {
        PRO_OUT_VECBASE_R::new((self.bits & 0x003f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:21"]
    #[inline(always)]
    pub fn pro_out_vecbase(&mut self) -> PRO_OUT_VECBASE_W<0> {
        PRO_OUT_VECBASE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pro_vecbase_set](index.html) module"]
pub struct PRO_VECBASE_SET_SPEC;
impl crate::RegisterSpec for PRO_VECBASE_SET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pro_vecbase_set::R](R) reader structure"]
impl crate::Readable for PRO_VECBASE_SET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pro_vecbase_set::W](W) writer structure"]
impl crate::Writable for PRO_VECBASE_SET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PRO_VECBASE_SET to value 0"]
impl crate::Resettable for PRO_VECBASE_SET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
