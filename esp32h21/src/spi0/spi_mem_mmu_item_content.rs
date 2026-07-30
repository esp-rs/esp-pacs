#[doc = "Register `SPI_MEM_MMU_ITEM_CONTENT` reader"]
pub type R = crate::R<SPI_MEM_MMU_ITEM_CONTENT_SPEC>;
#[doc = "Register `SPI_MEM_MMU_ITEM_CONTENT` writer"]
pub type W = crate::W<SPI_MEM_MMU_ITEM_CONTENT_SPEC>;
#[doc = "Field `SPI_MMU_ITEM_CONTENT` reader - MSPI-MMU item content"]
pub type SPI_MMU_ITEM_CONTENT_R = crate::FieldReader<u32>;
#[doc = "Field `SPI_MMU_ITEM_CONTENT` writer - MSPI-MMU item content"]
pub type SPI_MMU_ITEM_CONTENT_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - MSPI-MMU item content"]
    #[inline(always)]
    pub fn spi_mmu_item_content(&self) -> SPI_MMU_ITEM_CONTENT_R {
        SPI_MMU_ITEM_CONTENT_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_MEM_MMU_ITEM_CONTENT")
            .field("spi_mmu_item_content", &self.spi_mmu_item_content())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - MSPI-MMU item content"]
    #[inline(always)]
    pub fn spi_mmu_item_content(
        &mut self,
    ) -> SPI_MMU_ITEM_CONTENT_W<'_, SPI_MEM_MMU_ITEM_CONTENT_SPEC> {
        SPI_MMU_ITEM_CONTENT_W::new(self, 0)
    }
}
#[doc = "MSPI-MMU item content register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_mem_mmu_item_content::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_mem_mmu_item_content::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_MEM_MMU_ITEM_CONTENT_SPEC;
impl crate::RegisterSpec for SPI_MEM_MMU_ITEM_CONTENT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_mem_mmu_item_content::R`](R) reader structure"]
impl crate::Readable for SPI_MEM_MMU_ITEM_CONTENT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_mem_mmu_item_content::W`](W) writer structure"]
impl crate::Writable for SPI_MEM_MMU_ITEM_CONTENT_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI_MEM_MMU_ITEM_CONTENT to value 0x037c"]
impl crate::Resettable for SPI_MEM_MMU_ITEM_CONTENT_SPEC {
    const RESET_VALUE: u32 = 0x037c;
}
