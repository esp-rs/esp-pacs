#[doc = "Register `EDMA_BOUNDARY_1` reader"]
pub struct R(crate::R<EDMA_BOUNDARY_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EDMA_BOUNDARY_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EDMA_BOUNDARY_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EDMA_BOUNDARY_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EDMA_BOUNDARY_1` writer"]
pub struct W(crate::W<EDMA_BOUNDARY_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EDMA_BOUNDARY_1_SPEC>;
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
impl From<crate::W<EDMA_BOUNDARY_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EDMA_BOUNDARY_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EDMA_BOUNDARY_1` reader - This field is used to configure the boundary 1 of external RAM. The unit is 4K. For example, set this field to 0x80, then the address boundary 0 would be 0x3C080000 (0x3C000000 + 0x80 * 4K)."]
pub type EDMA_BOUNDARY_1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `EDMA_BOUNDARY_1` writer - This field is used to configure the boundary 1 of external RAM. The unit is 4K. For example, set this field to 0x80, then the address boundary 0 would be 0x3C080000 (0x3C000000 + 0x80 * 4K)."]
pub type EDMA_BOUNDARY_1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EDMA_BOUNDARY_1_SPEC, u16, u16, 14, O>;
impl R {
    #[doc = "Bits 0:13 - This field is used to configure the boundary 1 of external RAM. The unit is 4K. For example, set this field to 0x80, then the address boundary 0 would be 0x3C080000 (0x3C000000 + 0x80 * 4K)."]
    #[inline(always)]
    pub fn edma_boundary_1(&self) -> EDMA_BOUNDARY_1_R {
        EDMA_BOUNDARY_1_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - This field is used to configure the boundary 1 of external RAM. The unit is 4K. For example, set this field to 0x80, then the address boundary 0 would be 0x3C080000 (0x3C000000 + 0x80 * 4K)."]
    #[inline(always)]
    #[must_use]
    pub fn edma_boundary_1(&mut self) -> EDMA_BOUNDARY_1_W<0> {
        EDMA_BOUNDARY_1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EDMA boundary 1 configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [edma_boundary_1](index.html) module"]
pub struct EDMA_BOUNDARY_1_SPEC;
impl crate::RegisterSpec for EDMA_BOUNDARY_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [edma_boundary_1::R](R) reader structure"]
impl crate::Readable for EDMA_BOUNDARY_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [edma_boundary_1::W](W) writer structure"]
impl crate::Writable for EDMA_BOUNDARY_1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EDMA_BOUNDARY_1 to value 0x2000"]
impl crate::Resettable for EDMA_BOUNDARY_1_SPEC {
    const RESET_VALUE: Self::Ux = 0x2000;
}
