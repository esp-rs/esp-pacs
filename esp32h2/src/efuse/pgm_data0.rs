#[doc = "Register `PGM_DATA0` reader"]
pub struct R(crate::R<PGM_DATA0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PGM_DATA0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PGM_DATA0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PGM_DATA0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PGM_DATA0` writer"]
pub struct W(crate::W<PGM_DATA0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PGM_DATA0_SPEC>;
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
impl From<crate::W<PGM_DATA0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PGM_DATA0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PGM_DATA_0` reader - Configures the 0th 32-bit data to be programmed."]
pub type PGM_DATA_0_R = crate::FieldReader<u32, u32>;
#[doc = "Field `PGM_DATA_0` writer - Configures the 0th 32-bit data to be programmed."]
pub type PGM_DATA_0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PGM_DATA0_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Configures the 0th 32-bit data to be programmed."]
    #[inline(always)]
    pub fn pgm_data_0(&self) -> PGM_DATA_0_R {
        PGM_DATA_0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Configures the 0th 32-bit data to be programmed."]
    #[inline(always)]
    #[must_use]
    pub fn pgm_data_0(&mut self) -> PGM_DATA_0_W<0> {
        PGM_DATA_0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Register 0 that stores data to be programmed.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pgm_data0](index.html) module"]
pub struct PGM_DATA0_SPEC;
impl crate::RegisterSpec for PGM_DATA0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pgm_data0::R](R) reader structure"]
impl crate::Readable for PGM_DATA0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pgm_data0::W](W) writer structure"]
impl crate::Writable for PGM_DATA0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PGM_DATA0 to value 0"]
impl crate::Resettable for PGM_DATA0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
