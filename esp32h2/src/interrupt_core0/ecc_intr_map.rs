#[doc = "Register `ECC_INTR_MAP` reader"]
pub struct R(crate::R<ECC_INTR_MAP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ECC_INTR_MAP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ECC_INTR_MAP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ECC_INTR_MAP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ECC_INTR_MAP` writer"]
pub struct W(crate::W<ECC_INTR_MAP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ECC_INTR_MAP_SPEC>;
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
impl From<crate::W<ECC_INTR_MAP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ECC_INTR_MAP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ECC_INTR_MAP` reader - CORE0_ECC_INTR mapping register"]
pub type ECC_INTR_MAP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ECC_INTR_MAP` writer - CORE0_ECC_INTR mapping register"]
pub type ECC_INTR_MAP_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ECC_INTR_MAP_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - CORE0_ECC_INTR mapping register"]
    #[inline(always)]
    pub fn ecc_intr_map(&self) -> ECC_INTR_MAP_R {
        ECC_INTR_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - CORE0_ECC_INTR mapping register"]
    #[inline(always)]
    #[must_use]
    pub fn ecc_intr_map(&mut self) -> ECC_INTR_MAP_W<0> {
        ECC_INTR_MAP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ecc_intr_map](index.html) module"]
pub struct ECC_INTR_MAP_SPEC;
impl crate::RegisterSpec for ECC_INTR_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ecc_intr_map::R](R) reader structure"]
impl crate::Readable for ECC_INTR_MAP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ecc_intr_map::W](W) writer structure"]
impl crate::Writable for ECC_INTR_MAP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ECC_INTR_MAP to value 0"]
impl crate::Resettable for ECC_INTR_MAP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
