#[doc = "Register `PGM_DATA6` reader"]
pub struct R(crate::R<PGM_DATA6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PGM_DATA6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PGM_DATA6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PGM_DATA6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PGM_DATA6` writer"]
pub struct W(crate::W<PGM_DATA6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PGM_DATA6_SPEC>;
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
impl From<crate::W<PGM_DATA6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PGM_DATA6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PGM_DATA_6` reader - The content of the 6th 32-bit data to be programmed."]
pub type PGM_DATA_6_R = crate::FieldReader<u32>;
#[doc = "Field `PGM_DATA_6` writer - The content of the 6th 32-bit data to be programmed."]
pub type PGM_DATA_6_W<'a, const O: u8> = crate::FieldWriter<'a, PGM_DATA6_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - The content of the 6th 32-bit data to be programmed."]
    #[inline(always)]
    pub fn pgm_data_6(&self) -> PGM_DATA_6_R {
        PGM_DATA_6_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PGM_DATA6")
            .field("pgm_data_6", &format_args!("{}", self.pgm_data_6().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PGM_DATA6_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - The content of the 6th 32-bit data to be programmed."]
    #[inline(always)]
    #[must_use]
    pub fn pgm_data_6(&mut self) -> PGM_DATA_6_W<0> {
        PGM_DATA_6_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Register 6 that stores data to be programmed.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pgm_data6](index.html) module"]
pub struct PGM_DATA6_SPEC;
impl crate::RegisterSpec for PGM_DATA6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pgm_data6::R](R) reader structure"]
impl crate::Readable for PGM_DATA6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pgm_data6::W](W) writer structure"]
impl crate::Writable for PGM_DATA6_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PGM_DATA6 to value 0"]
impl crate::Resettable for PGM_DATA6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
