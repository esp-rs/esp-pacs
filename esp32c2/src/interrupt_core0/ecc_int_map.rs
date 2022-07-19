#[doc = "Register `ECC_INT_MAP` reader"]
pub struct R(crate::R<ECC_INT_MAP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ECC_INT_MAP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ECC_INT_MAP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ECC_INT_MAP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ECC_INT_MAP` writer"]
pub struct W(crate::W<ECC_INT_MAP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ECC_INT_MAP_SPEC>;
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
impl From<crate::W<ECC_INT_MAP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ECC_INT_MAP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ECC_INT_MAP` reader - Need add description"]
pub type ECC_INT_MAP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ECC_INT_MAP` writer - Need add description"]
pub type ECC_INT_MAP_W<'a> = crate::FieldWriter<'a, u32, ECC_INT_MAP_SPEC, u8, u8, 5, 0>;
impl R {
    #[doc = "Bits 0:4 - Need add description"]
    #[inline(always)]
    pub fn ecc_int_map(&self) -> ECC_INT_MAP_R {
        ECC_INT_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Need add description"]
    #[inline(always)]
    pub fn ecc_int_map(&mut self) -> ECC_INT_MAP_W {
        ECC_INT_MAP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ecc_int_map](index.html) module"]
pub struct ECC_INT_MAP_SPEC;
impl crate::RegisterSpec for ECC_INT_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ecc_int_map::R](R) reader structure"]
impl crate::Readable for ECC_INT_MAP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ecc_int_map::W](W) writer structure"]
impl crate::Writable for ECC_INT_MAP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ECC_INT_MAP to value 0"]
impl crate::Resettable for ECC_INT_MAP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
