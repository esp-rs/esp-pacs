#[doc = "Register `EDMA_BOUNDARY_2` reader"]
pub struct R(crate::R<EDMA_BOUNDARY_2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EDMA_BOUNDARY_2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EDMA_BOUNDARY_2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EDMA_BOUNDARY_2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EDMA_BOUNDARY_2` writer"]
pub struct W(crate::W<EDMA_BOUNDARY_2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EDMA_BOUNDARY_2_SPEC>;
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
impl From<crate::W<EDMA_BOUNDARY_2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EDMA_BOUNDARY_2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EDMA_BOUNDARY_2` reader - This field is used to configure the boundary 2 of external RAM. The unit is 4K. For example, set this field to 0x80, then the address boundary 0 would be 0x3C080000 (0x3C000000 + 0x80 * 4K)."]
pub type EDMA_BOUNDARY_2_R = crate::FieldReader<u16>;
#[doc = "Field `EDMA_BOUNDARY_2` writer - This field is used to configure the boundary 2 of external RAM. The unit is 4K. For example, set this field to 0x80, then the address boundary 0 would be 0x3C080000 (0x3C000000 + 0x80 * 4K)."]
pub type EDMA_BOUNDARY_2_W<'a, const O: u8> =
    crate::FieldWriter<'a, EDMA_BOUNDARY_2_SPEC, 14, O, u16>;
impl R {
    #[doc = "Bits 0:13 - This field is used to configure the boundary 2 of external RAM. The unit is 4K. For example, set this field to 0x80, then the address boundary 0 would be 0x3C080000 (0x3C000000 + 0x80 * 4K)."]
    #[inline(always)]
    pub fn edma_boundary_2(&self) -> EDMA_BOUNDARY_2_R {
        EDMA_BOUNDARY_2_R::new((self.bits & 0x3fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EDMA_BOUNDARY_2")
            .field(
                "edma_boundary_2",
                &format_args!("{}", self.edma_boundary_2().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<EDMA_BOUNDARY_2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:13 - This field is used to configure the boundary 2 of external RAM. The unit is 4K. For example, set this field to 0x80, then the address boundary 0 would be 0x3C080000 (0x3C000000 + 0x80 * 4K)."]
    #[inline(always)]
    #[must_use]
    pub fn edma_boundary_2(&mut self) -> EDMA_BOUNDARY_2_W<0> {
        EDMA_BOUNDARY_2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EDMA boundary 2 configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [edma_boundary_2](index.html) module"]
pub struct EDMA_BOUNDARY_2_SPEC;
impl crate::RegisterSpec for EDMA_BOUNDARY_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [edma_boundary_2::R](R) reader structure"]
impl crate::Readable for EDMA_BOUNDARY_2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [edma_boundary_2::W](W) writer structure"]
impl crate::Writable for EDMA_BOUNDARY_2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EDMA_BOUNDARY_2 to value 0x2000"]
impl crate::Resettable for EDMA_BOUNDARY_2_SPEC {
    const RESET_VALUE: Self::Ux = 0x2000;
}
