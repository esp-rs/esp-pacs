#[doc = "Register `SPI_MEM_SRAM_DWR_CMD` reader"]
pub struct R(crate::R<SPI_MEM_SRAM_DWR_CMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_MEM_SRAM_DWR_CMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_MEM_SRAM_DWR_CMD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_MEM_SRAM_DWR_CMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SPI_MEM_CACHE_SRAM_USR_WR_CMD_VALUE` reader - For SPI0,When cache mode is enable it is the write command value of command phase for sram."]
pub type SPI_MEM_CACHE_SRAM_USR_WR_CMD_VALUE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SPI_MEM_CACHE_SRAM_USR_WR_CMD_BITLEN` reader - For SPI0,When cache mode is enable it is the in bits of command phase for sram. The register value shall be (bit_num-1)."]
pub type SPI_MEM_CACHE_SRAM_USR_WR_CMD_BITLEN_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:15 - For SPI0,When cache mode is enable it is the write command value of command phase for sram."]
    #[inline(always)]
    pub fn spi_mem_cache_sram_usr_wr_cmd_value(&self) -> SPI_MEM_CACHE_SRAM_USR_WR_CMD_VALUE_R {
        SPI_MEM_CACHE_SRAM_USR_WR_CMD_VALUE_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 28:31 - For SPI0,When cache mode is enable it is the in bits of command phase for sram. The register value shall be (bit_num-1)."]
    #[inline(always)]
    pub fn spi_mem_cache_sram_usr_wr_cmd_bitlen(&self) -> SPI_MEM_CACHE_SRAM_USR_WR_CMD_BITLEN_R {
        SPI_MEM_CACHE_SRAM_USR_WR_CMD_BITLEN_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_MEM_SRAM_DWR_CMD")
            .field(
                "spi_mem_cache_sram_usr_wr_cmd_value",
                &format_args!("{}", self.spi_mem_cache_sram_usr_wr_cmd_value().bits()),
            )
            .field(
                "spi_mem_cache_sram_usr_wr_cmd_bitlen",
                &format_args!("{}", self.spi_mem_cache_sram_usr_wr_cmd_bitlen().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SPI_MEM_SRAM_DWR_CMD_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "SPI0 external RAM DDR write command control register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_mem_sram_dwr_cmd](index.html) module"]
pub struct SPI_MEM_SRAM_DWR_CMD_SPEC;
impl crate::RegisterSpec for SPI_MEM_SRAM_DWR_CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_mem_sram_dwr_cmd::R](R) reader structure"]
impl crate::Readable for SPI_MEM_SRAM_DWR_CMD_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SPI_MEM_SRAM_DWR_CMD to value 0"]
impl crate::Resettable for SPI_MEM_SRAM_DWR_CMD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
