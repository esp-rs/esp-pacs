#[doc = "Register `DMMU_PAGE_MODE` reader"]
pub struct R(crate::R<DMMU_PAGE_MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMMU_PAGE_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMMU_PAGE_MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMMU_PAGE_MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMMU_PAGE_MODE` writer"]
pub struct W(crate::W<DMMU_PAGE_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMMU_PAGE_MODE_SPEC>;
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
impl From<crate::W<DMMU_PAGE_MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMMU_PAGE_MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INTERNAL_SRAM_DMMU_ENA` reader - "]
pub type INTERNAL_SRAM_DMMU_ENA_R = crate::BitReader;
#[doc = "Field `INTERNAL_SRAM_DMMU_ENA` writer - "]
pub type INTERNAL_SRAM_DMMU_ENA_W<'a, const O: u8> = crate::BitWriter<'a, DMMU_PAGE_MODE_SPEC, O>;
#[doc = "Field `DMMU_PAGE_MODE` reader - "]
pub type DMMU_PAGE_MODE_R = crate::FieldReader;
#[doc = "Field `DMMU_PAGE_MODE` writer - "]
pub type DMMU_PAGE_MODE_W<'a, const O: u8> = crate::FieldWriter<'a, DMMU_PAGE_MODE_SPEC, 2, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn internal_sram_dmmu_ena(&self) -> INTERNAL_SRAM_DMMU_ENA_R {
        INTERNAL_SRAM_DMMU_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2"]
    #[inline(always)]
    pub fn dmmu_page_mode(&self) -> DMMU_PAGE_MODE_R {
        DMMU_PAGE_MODE_R::new(((self.bits >> 1) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMMU_PAGE_MODE")
            .field(
                "internal_sram_dmmu_ena",
                &format_args!("{}", self.internal_sram_dmmu_ena().bit()),
            )
            .field(
                "dmmu_page_mode",
                &format_args!("{}", self.dmmu_page_mode().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DMMU_PAGE_MODE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn internal_sram_dmmu_ena(&mut self) -> INTERNAL_SRAM_DMMU_ENA_W<0> {
        INTERNAL_SRAM_DMMU_ENA_W::new(self)
    }
    #[doc = "Bits 1:2"]
    #[inline(always)]
    #[must_use]
    pub fn dmmu_page_mode(&mut self) -> DMMU_PAGE_MODE_W<1> {
        DMMU_PAGE_MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmmu_page_mode](index.html) module"]
pub struct DMMU_PAGE_MODE_SPEC;
impl crate::RegisterSpec for DMMU_PAGE_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmmu_page_mode::R](R) reader structure"]
impl crate::Readable for DMMU_PAGE_MODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmmu_page_mode::W](W) writer structure"]
impl crate::Writable for DMMU_PAGE_MODE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMMU_PAGE_MODE to value 0"]
impl crate::Resettable for DMMU_PAGE_MODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
