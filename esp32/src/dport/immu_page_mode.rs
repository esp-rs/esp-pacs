#[doc = "Register `IMMU_PAGE_MODE` reader"]
pub type R = crate::R<IMMU_PAGE_MODE_SPEC>;
#[doc = "Register `IMMU_PAGE_MODE` writer"]
pub type W = crate::W<IMMU_PAGE_MODE_SPEC>;
#[doc = "Field `INTERNAL_SRAM_IMMU_ENA` reader - "]
pub type INTERNAL_SRAM_IMMU_ENA_R = crate::BitReader;
#[doc = "Field `INTERNAL_SRAM_IMMU_ENA` writer - "]
pub type INTERNAL_SRAM_IMMU_ENA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IMMU_PAGE_MODE` reader - "]
pub type IMMU_PAGE_MODE_R = crate::FieldReader;
#[doc = "Field `IMMU_PAGE_MODE` writer - "]
pub type IMMU_PAGE_MODE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
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
    pub fn internal_sram_immu_ena(&mut self) -> INTERNAL_SRAM_IMMU_ENA_W<IMMU_PAGE_MODE_SPEC, 0> {
        INTERNAL_SRAM_IMMU_ENA_W::new(self)
    }
    #[doc = "Bits 1:2"]
    #[inline(always)]
    #[must_use]
    pub fn immu_page_mode(&mut self) -> IMMU_PAGE_MODE_W<IMMU_PAGE_MODE_SPEC, 1> {
        IMMU_PAGE_MODE_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`immu_page_mode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`immu_page_mode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IMMU_PAGE_MODE_SPEC;
impl crate::RegisterSpec for IMMU_PAGE_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`immu_page_mode::R`](R) reader structure"]
impl crate::Readable for IMMU_PAGE_MODE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`immu_page_mode::W`](W) writer structure"]
impl crate::Writable for IMMU_PAGE_MODE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IMMU_PAGE_MODE to value 0"]
impl crate::Resettable for IMMU_PAGE_MODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
