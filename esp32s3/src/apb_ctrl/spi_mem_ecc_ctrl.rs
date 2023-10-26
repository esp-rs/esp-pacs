#[doc = "Register `SPI_MEM_ECC_CTRL` reader"]
pub type R = crate::R<SPI_MEM_ECC_CTRL_SPEC>;
#[doc = "Register `SPI_MEM_ECC_CTRL` writer"]
pub type W = crate::W<SPI_MEM_ECC_CTRL_SPEC>;
#[doc = "Field `FLASH_PAGE_SIZE` reader - Set the page size of the used MSPI flash. 0: 256 bytes. 1: 512 bytes. 2: 1024 bytes. 3: 2048 bytes."]
pub type FLASH_PAGE_SIZE_R = crate::FieldReader;
#[doc = "Field `FLASH_PAGE_SIZE` writer - Set the page size of the used MSPI flash. 0: 256 bytes. 1: 512 bytes. 2: 1024 bytes. 3: 2048 bytes."]
pub type FLASH_PAGE_SIZE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `SRAM_PAGE_SIZE` reader - Set the page size of the used MSPI external RAM. 0: 256 bytes. 1: 512 bytes. 2: 1024 bytes. 3: 2048 bytes."]
pub type SRAM_PAGE_SIZE_R = crate::FieldReader;
#[doc = "Field `SRAM_PAGE_SIZE` writer - Set the page size of the used MSPI external RAM. 0: 256 bytes. 1: 512 bytes. 2: 1024 bytes. 3: 2048 bytes."]
pub type SRAM_PAGE_SIZE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
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
            .field(
                "flash_page_size",
                &format_args!("{}", self.flash_page_size().bits()),
            )
            .field(
                "sram_page_size",
                &format_args!("{}", self.sram_page_size().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SPI_MEM_ECC_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 18:19 - Set the page size of the used MSPI flash. 0: 256 bytes. 1: 512 bytes. 2: 1024 bytes. 3: 2048 bytes."]
    #[inline(always)]
    #[must_use]
    pub fn flash_page_size(&mut self) -> FLASH_PAGE_SIZE_W<SPI_MEM_ECC_CTRL_SPEC, 18> {
        FLASH_PAGE_SIZE_W::new(self)
    }
    #[doc = "Bits 20:21 - Set the page size of the used MSPI external RAM. 0: 256 bytes. 1: 512 bytes. 2: 1024 bytes. 3: 2048 bytes."]
    #[inline(always)]
    #[must_use]
    pub fn sram_page_size(&mut self) -> SRAM_PAGE_SIZE_W<SPI_MEM_ECC_CTRL_SPEC, 20> {
        SRAM_PAGE_SIZE_W::new(self)
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
#[doc = "******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_ecc_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_ecc_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_MEM_ECC_CTRL_SPEC;
impl crate::RegisterSpec for SPI_MEM_ECC_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_mem_ecc_ctrl::R`](R) reader structure"]
impl crate::Readable for SPI_MEM_ECC_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_mem_ecc_ctrl::W`](W) writer structure"]
impl crate::Writable for SPI_MEM_ECC_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPI_MEM_ECC_CTRL to value 0x0020_0000"]
impl crate::Resettable for SPI_MEM_ECC_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0020_0000;
}
