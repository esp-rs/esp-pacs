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
#[doc = "Field `SPI_FMEM_ECC_ERR_INT_NUM` reader - Set the error times of MSPI ECC read to generate MSPI SPI_MEM_ECC_ERR_INT interrupt."]
pub type SPI_FMEM_ECC_ERR_INT_NUM_R = crate::FieldReader;
#[doc = "Field `SPI_FMEM_ECC_ERR_INT_EN` reader - Set this bit to calculate the error times of MSPI ECC read when accesses to flash."]
pub type SPI_FMEM_ECC_ERR_INT_EN_R = crate::BitReader;
#[doc = "Field `SPI_FMEM_PAGE_SIZE` reader - Set the page size of the flash accessed by MSPI. 0: 256 bytes. 1: 512 bytes. 2: 1024 bytes. 3: 2048 bytes."]
pub type SPI_FMEM_PAGE_SIZE_R = crate::FieldReader;
#[doc = "Field `SPI_FMEM_PAGE_SIZE` writer - Set the page size of the flash accessed by MSPI. 0: 256 bytes. 1: 512 bytes. 2: 1024 bytes. 3: 2048 bytes."]
pub type SPI_FMEM_PAGE_SIZE_W<'a, const O: u8> =
    crate::FieldWriter<'a, SPI_MEM_ECC_CTRL_SPEC, 2, O>;
#[doc = "Field `SPI_FMEM_ECC_ADDR_EN` reader - Set this bit to enable MSPI ECC address conversion, no matter MSPI accesses to the ECC region or non-ECC region of flash. If there is no ECC region in flash, this bit should be 0. Otherwise, this bit should be 1."]
pub type SPI_FMEM_ECC_ADDR_EN_R = crate::BitReader;
#[doc = "Field `SPI_MEM_USR_ECC_ADDR_EN` reader - Set this bit to enable ECC address convert in SPI0/1 USR_CMD transfer."]
pub type SPI_MEM_USR_ECC_ADDR_EN_R = crate::BitReader;
#[doc = "Field `SPI_MEM_ECC_CONTINUE_RECORD_ERR_EN` reader - 1: The error information in SPI_MEM_ECC_ERR_BITS and SPI_MEM_ECC_ERR_ADDR is updated when there is an ECC error. 0: SPI_MEM_ECC_ERR_BITS and SPI_MEM_ECC_ERR_ADDR record the first ECC error information."]
pub type SPI_MEM_ECC_CONTINUE_RECORD_ERR_EN_R = crate::BitReader;
#[doc = "Field `SPI_MEM_ECC_ERR_BITS` reader - Records the first ECC error bit number in the 16 bytes(From 0~127, corresponding to byte 0 bit 0 to byte 15 bit 7)"]
pub type SPI_MEM_ECC_ERR_BITS_R = crate::FieldReader;
impl R {
    #[doc = "Bits 11:16 - Set the error times of MSPI ECC read to generate MSPI SPI_MEM_ECC_ERR_INT interrupt."]
    #[inline(always)]
    pub fn spi_fmem_ecc_err_int_num(&self) -> SPI_FMEM_ECC_ERR_INT_NUM_R {
        SPI_FMEM_ECC_ERR_INT_NUM_R::new(((self.bits >> 11) & 0x3f) as u8)
    }
    #[doc = "Bit 17 - Set this bit to calculate the error times of MSPI ECC read when accesses to flash."]
    #[inline(always)]
    pub fn spi_fmem_ecc_err_int_en(&self) -> SPI_FMEM_ECC_ERR_INT_EN_R {
        SPI_FMEM_ECC_ERR_INT_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - Set the page size of the flash accessed by MSPI. 0: 256 bytes. 1: 512 bytes. 2: 1024 bytes. 3: 2048 bytes."]
    #[inline(always)]
    pub fn spi_fmem_page_size(&self) -> SPI_FMEM_PAGE_SIZE_R {
        SPI_FMEM_PAGE_SIZE_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - Set this bit to enable MSPI ECC address conversion, no matter MSPI accesses to the ECC region or non-ECC region of flash. If there is no ECC region in flash, this bit should be 0. Otherwise, this bit should be 1."]
    #[inline(always)]
    pub fn spi_fmem_ecc_addr_en(&self) -> SPI_FMEM_ECC_ADDR_EN_R {
        SPI_FMEM_ECC_ADDR_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Set this bit to enable ECC address convert in SPI0/1 USR_CMD transfer."]
    #[inline(always)]
    pub fn spi_mem_usr_ecc_addr_en(&self) -> SPI_MEM_USR_ECC_ADDR_EN_R {
        SPI_MEM_USR_ECC_ADDR_EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 24 - 1: The error information in SPI_MEM_ECC_ERR_BITS and SPI_MEM_ECC_ERR_ADDR is updated when there is an ECC error. 0: SPI_MEM_ECC_ERR_BITS and SPI_MEM_ECC_ERR_ADDR record the first ECC error information."]
    #[inline(always)]
    pub fn spi_mem_ecc_continue_record_err_en(&self) -> SPI_MEM_ECC_CONTINUE_RECORD_ERR_EN_R {
        SPI_MEM_ECC_CONTINUE_RECORD_ERR_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:31 - Records the first ECC error bit number in the 16 bytes(From 0~127, corresponding to byte 0 bit 0 to byte 15 bit 7)"]
    #[inline(always)]
    pub fn spi_mem_ecc_err_bits(&self) -> SPI_MEM_ECC_ERR_BITS_R {
        SPI_MEM_ECC_ERR_BITS_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_MEM_ECC_CTRL")
            .field(
                "spi_fmem_ecc_err_int_num",
                &format_args!("{}", self.spi_fmem_ecc_err_int_num().bits()),
            )
            .field(
                "spi_fmem_ecc_err_int_en",
                &format_args!("{}", self.spi_fmem_ecc_err_int_en().bit()),
            )
            .field(
                "spi_fmem_page_size",
                &format_args!("{}", self.spi_fmem_page_size().bits()),
            )
            .field(
                "spi_fmem_ecc_addr_en",
                &format_args!("{}", self.spi_fmem_ecc_addr_en().bit()),
            )
            .field(
                "spi_mem_usr_ecc_addr_en",
                &format_args!("{}", self.spi_mem_usr_ecc_addr_en().bit()),
            )
            .field(
                "spi_mem_ecc_continue_record_err_en",
                &format_args!("{}", self.spi_mem_ecc_continue_record_err_en().bit()),
            )
            .field(
                "spi_mem_ecc_err_bits",
                &format_args!("{}", self.spi_mem_ecc_err_bits().bits()),
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
    #[doc = "Bits 18:19 - Set the page size of the flash accessed by MSPI. 0: 256 bytes. 1: 512 bytes. 2: 1024 bytes. 3: 2048 bytes."]
    #[inline(always)]
    #[must_use]
    pub fn spi_fmem_page_size(&mut self) -> SPI_FMEM_PAGE_SIZE_W<18> {
        SPI_FMEM_PAGE_SIZE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MSPI ECC control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_mem_ecc_ctrl](index.html) module"]
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
#[doc = "`reset()` method sets SPI_MEM_ECC_CTRL to value 0x0100_5000"]
impl crate::Resettable for SPI_MEM_ECC_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0100_5000;
}
