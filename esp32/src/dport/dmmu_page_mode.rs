#[doc = "Register `DMMU_PAGE_MODE` reader"]
pub type R = crate::R<DMMU_PAGE_MODE_SPEC>;
#[doc = "Register `DMMU_PAGE_MODE` writer"]
pub type W = crate::W<DMMU_PAGE_MODE_SPEC>;
#[doc = "Field `INTERNAL_SRAM_DMMU_ENA` reader - "]
pub type INTERNAL_SRAM_DMMU_ENA_R = crate::BitReader;
#[doc = "Field `INTERNAL_SRAM_DMMU_ENA` writer - "]
pub type INTERNAL_SRAM_DMMU_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMMU_PAGE_MODE` reader - "]
pub type DMMU_PAGE_MODE_R = crate::FieldReader;
#[doc = "Field `DMMU_PAGE_MODE` writer - "]
pub type DMMU_PAGE_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
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
            .field("internal_sram_dmmu_ena", &self.internal_sram_dmmu_ena())
            .field("dmmu_page_mode", &self.dmmu_page_mode())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn internal_sram_dmmu_ena(&mut self) -> INTERNAL_SRAM_DMMU_ENA_W<'_, DMMU_PAGE_MODE_SPEC> {
        INTERNAL_SRAM_DMMU_ENA_W::new(self, 0)
    }
    #[doc = "Bits 1:2"]
    #[inline(always)]
    pub fn dmmu_page_mode(&mut self) -> DMMU_PAGE_MODE_W<'_, DMMU_PAGE_MODE_SPEC> {
        DMMU_PAGE_MODE_W::new(self, 1)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`dmmu_page_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmmu_page_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMMU_PAGE_MODE_SPEC;
impl crate::RegisterSpec for DMMU_PAGE_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmmu_page_mode::R`](R) reader structure"]
impl crate::Readable for DMMU_PAGE_MODE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dmmu_page_mode::W`](W) writer structure"]
impl crate::Writable for DMMU_PAGE_MODE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMMU_PAGE_MODE to value 0"]
impl crate::Resettable for DMMU_PAGE_MODE_SPEC {}
