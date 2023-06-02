#[doc = "Register `SPI_MEM_CACHE_SCTRL` reader"]
pub struct R(crate::R<SPI_MEM_CACHE_SCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_MEM_CACHE_SCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_MEM_CACHE_SCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_MEM_CACHE_SCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SPI_MEM_CACHE_USR_SADDR_4BYTE` reader - For SPI0, In the external RAM mode, cache read flash with 4 bytes command, 1: enable, 0:disable."]
pub type SPI_MEM_CACHE_USR_SADDR_4BYTE_R = crate::BitReader;
#[doc = "Field `SPI_MEM_USR_SRAM_DIO` reader - For SPI0, In the external RAM mode, spi dual I/O mode enable, 1: enable, 0:disable"]
pub type SPI_MEM_USR_SRAM_DIO_R = crate::BitReader;
#[doc = "Field `SPI_MEM_USR_SRAM_QIO` reader - For SPI0, In the external RAM mode, spi quad I/O mode enable, 1: enable, 0:disable"]
pub type SPI_MEM_USR_SRAM_QIO_R = crate::BitReader;
#[doc = "Field `SPI_MEM_USR_WR_SRAM_DUMMY` reader - For SPI0, In the external RAM mode, it is the enable bit of dummy phase for write operations."]
pub type SPI_MEM_USR_WR_SRAM_DUMMY_R = crate::BitReader;
#[doc = "Field `SPI_MEM_USR_RD_SRAM_DUMMY` reader - For SPI0, In the external RAM mode, it is the enable bit of dummy phase for read operations."]
pub type SPI_MEM_USR_RD_SRAM_DUMMY_R = crate::BitReader;
#[doc = "Field `SPI_MEM_CACHE_SRAM_USR_RCMD` reader - For SPI0, In the external RAM mode cache read external RAM for user define command."]
pub type SPI_MEM_CACHE_SRAM_USR_RCMD_R = crate::BitReader;
#[doc = "Field `SPI_MEM_SRAM_RDUMMY_CYCLELEN` reader - For SPI0, In the external RAM mode, it is the length in bits of read dummy phase. The register value shall be (bit_num-1)."]
pub type SPI_MEM_SRAM_RDUMMY_CYCLELEN_R = crate::FieldReader;
#[doc = "Field `SPI_MEM_SRAM_ADDR_BITLEN` reader - For SPI0, In the external RAM mode, it is the length in bits of address phase. The register value shall be (bit_num-1)."]
pub type SPI_MEM_SRAM_ADDR_BITLEN_R = crate::FieldReader;
#[doc = "Field `SPI_MEM_CACHE_SRAM_USR_WCMD` reader - For SPI0, In the external RAM mode cache write sram for user define command"]
pub type SPI_MEM_CACHE_SRAM_USR_WCMD_R = crate::BitReader;
#[doc = "Field `SPI_MEM_SRAM_OCT` reader - reserved"]
pub type SPI_MEM_SRAM_OCT_R = crate::BitReader;
#[doc = "Field `SPI_MEM_SRAM_WDUMMY_CYCLELEN` reader - For SPI0, In the external RAM mode, it is the length in bits of write dummy phase. The register value shall be (bit_num-1)."]
pub type SPI_MEM_SRAM_WDUMMY_CYCLELEN_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - For SPI0, In the external RAM mode, cache read flash with 4 bytes command, 1: enable, 0:disable."]
    #[inline(always)]
    pub fn spi_mem_cache_usr_saddr_4byte(&self) -> SPI_MEM_CACHE_USR_SADDR_4BYTE_R {
        SPI_MEM_CACHE_USR_SADDR_4BYTE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - For SPI0, In the external RAM mode, spi dual I/O mode enable, 1: enable, 0:disable"]
    #[inline(always)]
    pub fn spi_mem_usr_sram_dio(&self) -> SPI_MEM_USR_SRAM_DIO_R {
        SPI_MEM_USR_SRAM_DIO_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - For SPI0, In the external RAM mode, spi quad I/O mode enable, 1: enable, 0:disable"]
    #[inline(always)]
    pub fn spi_mem_usr_sram_qio(&self) -> SPI_MEM_USR_SRAM_QIO_R {
        SPI_MEM_USR_SRAM_QIO_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - For SPI0, In the external RAM mode, it is the enable bit of dummy phase for write operations."]
    #[inline(always)]
    pub fn spi_mem_usr_wr_sram_dummy(&self) -> SPI_MEM_USR_WR_SRAM_DUMMY_R {
        SPI_MEM_USR_WR_SRAM_DUMMY_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - For SPI0, In the external RAM mode, it is the enable bit of dummy phase for read operations."]
    #[inline(always)]
    pub fn spi_mem_usr_rd_sram_dummy(&self) -> SPI_MEM_USR_RD_SRAM_DUMMY_R {
        SPI_MEM_USR_RD_SRAM_DUMMY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - For SPI0, In the external RAM mode cache read external RAM for user define command."]
    #[inline(always)]
    pub fn spi_mem_cache_sram_usr_rcmd(&self) -> SPI_MEM_CACHE_SRAM_USR_RCMD_R {
        SPI_MEM_CACHE_SRAM_USR_RCMD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:11 - For SPI0, In the external RAM mode, it is the length in bits of read dummy phase. The register value shall be (bit_num-1)."]
    #[inline(always)]
    pub fn spi_mem_sram_rdummy_cyclelen(&self) -> SPI_MEM_SRAM_RDUMMY_CYCLELEN_R {
        SPI_MEM_SRAM_RDUMMY_CYCLELEN_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 14:19 - For SPI0, In the external RAM mode, it is the length in bits of address phase. The register value shall be (bit_num-1)."]
    #[inline(always)]
    pub fn spi_mem_sram_addr_bitlen(&self) -> SPI_MEM_SRAM_ADDR_BITLEN_R {
        SPI_MEM_SRAM_ADDR_BITLEN_R::new(((self.bits >> 14) & 0x3f) as u8)
    }
    #[doc = "Bit 20 - For SPI0, In the external RAM mode cache write sram for user define command"]
    #[inline(always)]
    pub fn spi_mem_cache_sram_usr_wcmd(&self) -> SPI_MEM_CACHE_SRAM_USR_WCMD_R {
        SPI_MEM_CACHE_SRAM_USR_WCMD_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - reserved"]
    #[inline(always)]
    pub fn spi_mem_sram_oct(&self) -> SPI_MEM_SRAM_OCT_R {
        SPI_MEM_SRAM_OCT_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:27 - For SPI0, In the external RAM mode, it is the length in bits of write dummy phase. The register value shall be (bit_num-1)."]
    #[inline(always)]
    pub fn spi_mem_sram_wdummy_cyclelen(&self) -> SPI_MEM_SRAM_WDUMMY_CYCLELEN_R {
        SPI_MEM_SRAM_WDUMMY_CYCLELEN_R::new(((self.bits >> 22) & 0x3f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_MEM_CACHE_SCTRL")
            .field(
                "spi_mem_cache_usr_saddr_4byte",
                &format_args!("{}", self.spi_mem_cache_usr_saddr_4byte().bit()),
            )
            .field(
                "spi_mem_usr_sram_dio",
                &format_args!("{}", self.spi_mem_usr_sram_dio().bit()),
            )
            .field(
                "spi_mem_usr_sram_qio",
                &format_args!("{}", self.spi_mem_usr_sram_qio().bit()),
            )
            .field(
                "spi_mem_usr_wr_sram_dummy",
                &format_args!("{}", self.spi_mem_usr_wr_sram_dummy().bit()),
            )
            .field(
                "spi_mem_usr_rd_sram_dummy",
                &format_args!("{}", self.spi_mem_usr_rd_sram_dummy().bit()),
            )
            .field(
                "spi_mem_cache_sram_usr_rcmd",
                &format_args!("{}", self.spi_mem_cache_sram_usr_rcmd().bit()),
            )
            .field(
                "spi_mem_sram_rdummy_cyclelen",
                &format_args!("{}", self.spi_mem_sram_rdummy_cyclelen().bits()),
            )
            .field(
                "spi_mem_sram_addr_bitlen",
                &format_args!("{}", self.spi_mem_sram_addr_bitlen().bits()),
            )
            .field(
                "spi_mem_cache_sram_usr_wcmd",
                &format_args!("{}", self.spi_mem_cache_sram_usr_wcmd().bit()),
            )
            .field(
                "spi_mem_sram_oct",
                &format_args!("{}", self.spi_mem_sram_oct().bit()),
            )
            .field(
                "spi_mem_sram_wdummy_cyclelen",
                &format_args!("{}", self.spi_mem_sram_wdummy_cyclelen().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SPI_MEM_CACHE_SCTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "SPI0 external RAM control register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_mem_cache_sctrl](index.html) module"]
pub struct SPI_MEM_CACHE_SCTRL_SPEC;
impl crate::RegisterSpec for SPI_MEM_CACHE_SCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_mem_cache_sctrl::R](R) reader structure"]
impl crate::Readable for SPI_MEM_CACHE_SCTRL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SPI_MEM_CACHE_SCTRL to value 0x0055_c070"]
impl crate::Resettable for SPI_MEM_CACHE_SCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0055_c070;
}
