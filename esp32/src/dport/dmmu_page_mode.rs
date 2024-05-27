///Register `DMMU_PAGE_MODE` reader
pub type R = crate::R<DMMU_PAGE_MODE_SPEC>;
///Register `DMMU_PAGE_MODE` writer
pub type W = crate::W<DMMU_PAGE_MODE_SPEC>;
///Field `INTERNAL_SRAM_DMMU_ENA` reader -
pub type INTERNAL_SRAM_DMMU_ENA_R = crate::BitReader;
///Field `INTERNAL_SRAM_DMMU_ENA` writer -
pub type INTERNAL_SRAM_DMMU_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMMU_PAGE_MODE` reader -
pub type DMMU_PAGE_MODE_R = crate::FieldReader;
///Field `DMMU_PAGE_MODE` writer -
pub type DMMU_PAGE_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bit 0
    #[inline(always)]
    pub fn internal_sram_dmmu_ena(&self) -> INTERNAL_SRAM_DMMU_ENA_R {
        INTERNAL_SRAM_DMMU_ENA_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:2
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
    ///Bit 0
    #[inline(always)]
    #[must_use]
    pub fn internal_sram_dmmu_ena(&mut self) -> INTERNAL_SRAM_DMMU_ENA_W<DMMU_PAGE_MODE_SPEC> {
        INTERNAL_SRAM_DMMU_ENA_W::new(self, 0)
    }
    ///Bits 1:2
    #[inline(always)]
    #[must_use]
    pub fn dmmu_page_mode(&mut self) -> DMMU_PAGE_MODE_W<DMMU_PAGE_MODE_SPEC> {
        DMMU_PAGE_MODE_W::new(self, 1)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`dmmu_page_mode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmmu_page_mode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DMMU_PAGE_MODE_SPEC;
impl crate::RegisterSpec for DMMU_PAGE_MODE_SPEC {
    type Ux = u32;
}
///`read()` method returns [`dmmu_page_mode::R`](R) reader structure
impl crate::Readable for DMMU_PAGE_MODE_SPEC {}
///`write(|w| ..)` method takes [`dmmu_page_mode::W`](W) writer structure
impl crate::Writable for DMMU_PAGE_MODE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DMMU_PAGE_MODE to value 0
impl crate::Resettable for DMMU_PAGE_MODE_SPEC {
    const RESET_VALUE: u32 = 0;
}
