#[doc = "Register `SPI_MEM_ECC_ERR_ADDR` reader"]
pub struct R(crate::R<SPI_MEM_ECC_ERR_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_MEM_ECC_ERR_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_MEM_ECC_ERR_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_MEM_ECC_ERR_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SPI_MEM_ECC_ERR_ADDR` reader - This bits show the first MSPI ECC error address. It is cleared by when SPI_MEM_ECC_ERR_INT_CLR bit is set."]
pub type SPI_MEM_ECC_ERR_ADDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SPI_MEM_ECC_ERR_CNT` reader - This bits show the error times of MSPI ECC read. It is cleared by when SPI_MEM_ECC_ERR_INT_CLR bit is set."]
pub type SPI_MEM_ECC_ERR_CNT_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:25 - This bits show the first MSPI ECC error address. It is cleared by when SPI_MEM_ECC_ERR_INT_CLR bit is set."]
    #[inline(always)]
    pub fn spi_mem_ecc_err_addr(&self) -> SPI_MEM_ECC_ERR_ADDR_R {
        SPI_MEM_ECC_ERR_ADDR_R::new(self.bits & 0x03ff_ffff)
    }
    #[doc = "Bits 26:31 - This bits show the error times of MSPI ECC read. It is cleared by when SPI_MEM_ECC_ERR_INT_CLR bit is set."]
    #[inline(always)]
    pub fn spi_mem_ecc_err_cnt(&self) -> SPI_MEM_ECC_ERR_CNT_R {
        SPI_MEM_ECC_ERR_CNT_R::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_MEM_ECC_ERR_ADDR")
            .field(
                "spi_mem_ecc_err_addr",
                &format_args!("{}", self.spi_mem_ecc_err_addr().bits()),
            )
            .field(
                "spi_mem_ecc_err_cnt",
                &format_args!("{}", self.spi_mem_ecc_err_cnt().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SPI_MEM_ECC_ERR_ADDR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "MSPI ECC error address register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_mem_ecc_err_addr](index.html) module"]
pub struct SPI_MEM_ECC_ERR_ADDR_SPEC;
impl crate::RegisterSpec for SPI_MEM_ECC_ERR_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_mem_ecc_err_addr::R](R) reader structure"]
impl crate::Readable for SPI_MEM_ECC_ERR_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SPI_MEM_ECC_ERR_ADDR to value 0"]
impl crate::Resettable for SPI_MEM_ECC_ERR_ADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
