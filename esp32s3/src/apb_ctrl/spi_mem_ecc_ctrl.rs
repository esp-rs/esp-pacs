#[doc = "Register `SPI_MEM_ECC_CTRL` reader"]
pub type R = crate::R<SPI_MEM_ECC_CTRL_SPEC>;
#[doc = "Register `SPI_MEM_ECC_CTRL` writer"]
pub type W = crate::W<SPI_MEM_ECC_CTRL_SPEC>;
#[doc = "Field `FLASH_PAGE_SIZE` reader - Set the page size of the used MSPI flash. 0: 256 bytes. 1: 512 bytes. 2: 1024 bytes. 3: 2048 bytes."]
pub type FLASH_PAGE_SIZE_R = crate::FieldReader;
#[doc = "Field `FLASH_PAGE_SIZE` writer - Set the page size of the used MSPI flash. 0: 256 bytes. 1: 512 bytes. 2: 1024 bytes. 3: 2048 bytes."]
pub type FLASH_PAGE_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SRAM_PAGE_SIZE` reader - Set the page size of the used MSPI external RAM. 0: 256 bytes. 1: 512 bytes. 2: 1024 bytes. 3: 2048 bytes."]
pub type SRAM_PAGE_SIZE_R = crate::FieldReader;
#[doc = "Field `SRAM_PAGE_SIZE` writer - Set the page size of the used MSPI external RAM. 0: 256 bytes. 1: 512 bytes. 2: 1024 bytes. 3: 2048 bytes."]
pub type SRAM_PAGE_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 18:19 - Set the page size of the used MSPI flash. 0: 256 bytes. 1: 512 bytes. 2: 1024 bytes. 3: 2048 bytes."]
    #[inline(always)]
    pub fn flash_page_size(&self) -> FLASH_PAGE_SIZE_R {
        FLASH_PAGE_SIZE_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Set the page size of the used MSPI external RAM. 0: 256 bytes. 1: 512 bytes. 2: 1024 bytes. 3: 2048 bytes."]
    #[inline(always)]
    pub fn sram_page_size(&self) -> SRAM_PAGE_SIZE_R {
        SRAM_PAGE_SIZE_R::new(((self.bits >> 20) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_MEM_ECC_CTRL")
            .field("flash_page_size", &self.flash_page_size())
            .field("sram_page_size", &self.sram_page_size())
            .finish()
    }
}
impl W {
    #[doc = "Bits 18:19 - Set the page size of the used MSPI flash. 0: 256 bytes. 1: 512 bytes. 2: 1024 bytes. 3: 2048 bytes."]
    #[inline(always)]
    pub fn flash_page_size(&mut self) -> FLASH_PAGE_SIZE_W<'_, SPI_MEM_ECC_CTRL_SPEC> {
        FLASH_PAGE_SIZE_W::new(self, 18)
    }
    #[doc = "Bits 20:21 - Set the page size of the used MSPI external RAM. 0: 256 bytes. 1: 512 bytes. 2: 1024 bytes. 3: 2048 bytes."]
    #[inline(always)]
    pub fn sram_page_size(&mut self) -> SRAM_PAGE_SIZE_W<'_, SPI_MEM_ECC_CTRL_SPEC> {
        SRAM_PAGE_SIZE_W::new(self, 20)
    }
}
#[doc = "******* Description ***********\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_mem_ecc_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_mem_ecc_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_MEM_ECC_CTRL_SPEC;
impl crate::RegisterSpec for SPI_MEM_ECC_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_mem_ecc_ctrl::R`](R) reader structure"]
impl crate::Readable for SPI_MEM_ECC_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_mem_ecc_ctrl::W`](W) writer structure"]
impl crate::Writable for SPI_MEM_ECC_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI_MEM_ECC_CTRL to value 0x0020_0000"]
impl crate::Resettable for SPI_MEM_ECC_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x0020_0000;
}
