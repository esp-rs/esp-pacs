#[doc = "Register `SPI_MEM_ECC_CTRL` reader"]
pub struct R(crate::R<SPI_MEM_ECC_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_MEM_ECC_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_MEM_ECC_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_MEM_ECC_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_MEM_ECC_CTRL` writer"]
pub struct W(crate::W<SPI_MEM_ECC_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_MEM_ECC_CTRL_SPEC>;
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
impl From<crate::W<SPI_MEM_ECC_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_MEM_ECC_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLASH_PAGE_SIZE` reader - Set the page size of the used MSPI flash. 0: 256 bytes. 1: 512 bytes. 2: 1024 bytes. 3: 2048 bytes."]
pub type FLASH_PAGE_SIZE_R = crate::FieldReader;
#[doc = "Field `FLASH_PAGE_SIZE` writer - Set the page size of the used MSPI flash. 0: 256 bytes. 1: 512 bytes. 2: 1024 bytes. 3: 2048 bytes."]
pub type FLASH_PAGE_SIZE_W<'a, const O: u8> = crate::FieldWriter<'a, SPI_MEM_ECC_CTRL_SPEC, 2, O>;
#[doc = "Field `SRAM_PAGE_SIZE` reader - Set the page size of the used MSPI external RAM. 0: 256 bytes. 1: 512 bytes. 2: 1024 bytes. 3: 2048 bytes."]
pub type SRAM_PAGE_SIZE_R = crate::FieldReader;
#[doc = "Field `SRAM_PAGE_SIZE` writer - Set the page size of the used MSPI external RAM. 0: 256 bytes. 1: 512 bytes. 2: 1024 bytes. 3: 2048 bytes."]
pub type SRAM_PAGE_SIZE_W<'a, const O: u8> = crate::FieldWriter<'a, SPI_MEM_ECC_CTRL_SPEC, 2, O>;
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
    pub fn flash_page_size(&mut self) -> FLASH_PAGE_SIZE_W<18> {
        FLASH_PAGE_SIZE_W::new(self)
    }
    #[doc = "Bits 20:21 - Set the page size of the used MSPI external RAM. 0: 256 bytes. 1: 512 bytes. 2: 1024 bytes. 3: 2048 bytes."]
    #[inline(always)]
    #[must_use]
    pub fn sram_page_size(&mut self) -> SRAM_PAGE_SIZE_W<20> {
        SRAM_PAGE_SIZE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "******* Description ***********\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_mem_ecc_ctrl](index.html) module"]
pub struct SPI_MEM_ECC_CTRL_SPEC;
impl crate::RegisterSpec for SPI_MEM_ECC_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_mem_ecc_ctrl::R](R) reader structure"]
impl crate::Readable for SPI_MEM_ECC_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_mem_ecc_ctrl::W](W) writer structure"]
impl crate::Writable for SPI_MEM_ECC_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPI_MEM_ECC_CTRL to value 0x0020_0000"]
impl crate::Resettable for SPI_MEM_ECC_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0020_0000;
}
