#[doc = "Register `ENDIAN` reader"]
pub struct R(crate::R<ENDIAN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ENDIAN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ENDIAN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ENDIAN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ENDIAN` writer"]
pub struct W(crate::W<ENDIAN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ENDIAN_SPEC>;
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
impl From<crate::W<ENDIAN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ENDIAN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENDIAN` reader - Endianness selection register. See Table 22-2 for details."]
pub type ENDIAN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ENDIAN` writer - Endianness selection register. See Table 22-2 for details."]
pub type ENDIAN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ENDIAN_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1 - Endianness selection register. See Table 22-2 for details."]
    #[inline(always)]
    pub fn endian(&self) -> ENDIAN_R {
        ENDIAN_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Endianness selection register. See Table 22-2 for details."]
    #[inline(always)]
    pub fn endian(&mut self) -> ENDIAN_W<0> {
        ENDIAN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [endian](index.html) module"]
pub struct ENDIAN_SPEC;
impl crate::RegisterSpec for ENDIAN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [endian::R](R) reader structure"]
impl crate::Readable for ENDIAN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [endian::W](W) writer structure"]
impl crate::Writable for ENDIAN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ENDIAN to value 0"]
impl crate::Resettable for ENDIAN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
