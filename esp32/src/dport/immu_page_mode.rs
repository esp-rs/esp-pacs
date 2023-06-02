#[doc = "Register `IMMU_PAGE_MODE` reader"]
pub struct R(crate::R<IMMU_PAGE_MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IMMU_PAGE_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IMMU_PAGE_MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IMMU_PAGE_MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IMMU_PAGE_MODE` writer"]
pub struct W(crate::W<IMMU_PAGE_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IMMU_PAGE_MODE_SPEC>;
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
impl From<crate::W<IMMU_PAGE_MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IMMU_PAGE_MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INTERNAL_SRAM_IMMU_ENA` reader - "]
pub type INTERNAL_SRAM_IMMU_ENA_R = crate::BitReader;
#[doc = "Field `INTERNAL_SRAM_IMMU_ENA` writer - "]
pub type INTERNAL_SRAM_IMMU_ENA_W<'a, const O: u8> = crate::BitWriter<'a, IMMU_PAGE_MODE_SPEC, O>;
#[doc = "Field `IMMU_PAGE_MODE` reader - "]
pub type IMMU_PAGE_MODE_R = crate::FieldReader;
#[doc = "Field `IMMU_PAGE_MODE` writer - "]
pub type IMMU_PAGE_MODE_W<'a, const O: u8> = crate::FieldWriter<'a, IMMU_PAGE_MODE_SPEC, 2, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn internal_sram_immu_ena(&self) -> INTERNAL_SRAM_IMMU_ENA_R {
        INTERNAL_SRAM_IMMU_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2"]
    #[inline(always)]
    pub fn immu_page_mode(&self) -> IMMU_PAGE_MODE_R {
        IMMU_PAGE_MODE_R::new(((self.bits >> 1) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IMMU_PAGE_MODE")
            .field(
                "internal_sram_immu_ena",
                &format_args!("{}", self.internal_sram_immu_ena().bit()),
            )
            .field(
                "immu_page_mode",
                &format_args!("{}", self.immu_page_mode().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<IMMU_PAGE_MODE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn internal_sram_immu_ena(&mut self) -> INTERNAL_SRAM_IMMU_ENA_W<0> {
        INTERNAL_SRAM_IMMU_ENA_W::new(self)
    }
    #[doc = "Bits 1:2"]
    #[inline(always)]
    #[must_use]
    pub fn immu_page_mode(&mut self) -> IMMU_PAGE_MODE_W<1> {
        IMMU_PAGE_MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [immu_page_mode](index.html) module"]
pub struct IMMU_PAGE_MODE_SPEC;
impl crate::RegisterSpec for IMMU_PAGE_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [immu_page_mode::R](R) reader structure"]
impl crate::Readable for IMMU_PAGE_MODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [immu_page_mode::W](W) writer structure"]
impl crate::Writable for IMMU_PAGE_MODE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IMMU_PAGE_MODE to value 0"]
impl crate::Resettable for IMMU_PAGE_MODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
