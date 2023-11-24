#[doc = "Register `SPI_MEM_MMU_ITEM_INDEX` reader"]
pub type R = crate::R<SPI_MEM_MMU_ITEM_INDEX_SPEC>;
#[doc = "Register `SPI_MEM_MMU_ITEM_INDEX` writer"]
pub type W = crate::W<SPI_MEM_MMU_ITEM_INDEX_SPEC>;
#[doc = "Field `SPI_MMU_ITEM_INDEX` reader - MSPI-MMU item index"]
pub type SPI_MMU_ITEM_INDEX_R = crate::FieldReader<u32>;
#[doc = "Field `SPI_MMU_ITEM_INDEX` writer - MSPI-MMU item index"]
pub type SPI_MMU_ITEM_INDEX_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - MSPI-MMU item index"]
    #[inline(always)]
    pub fn spi_mmu_item_index(&self) -> SPI_MMU_ITEM_INDEX_R {
        SPI_MMU_ITEM_INDEX_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_MEM_MMU_ITEM_INDEX")
            .field(
                "spi_mmu_item_index",
                &format_args!("{}", self.spi_mmu_item_index().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SPI_MEM_MMU_ITEM_INDEX_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - MSPI-MMU item index"]
    #[inline(always)]
    #[must_use]
    pub fn spi_mmu_item_index(&mut self) -> SPI_MMU_ITEM_INDEX_W<SPI_MEM_MMU_ITEM_INDEX_SPEC> {
        SPI_MMU_ITEM_INDEX_W::new(self, 0)
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
#[doc = "MSPI-MMU item index register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_mmu_item_index::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_mmu_item_index::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_MEM_MMU_ITEM_INDEX_SPEC;
impl crate::RegisterSpec for SPI_MEM_MMU_ITEM_INDEX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_mem_mmu_item_index::R`](R) reader structure"]
impl crate::Readable for SPI_MEM_MMU_ITEM_INDEX_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_mem_mmu_item_index::W`](W) writer structure"]
impl crate::Writable for SPI_MEM_MMU_ITEM_INDEX_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPI_MEM_MMU_ITEM_INDEX to value 0"]
impl crate::Resettable for SPI_MEM_MMU_ITEM_INDEX_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
