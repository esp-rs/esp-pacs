#[doc = "Register `SPI_MEM_INT_ST` reader"]
pub type R = crate::R<SPI_MEM_INT_ST_SPEC>;
#[doc = "Field `SPI_MEM_SLV_ST_END_INT_ST` reader - The status bit for SPI_MEM_SLV_ST_END_INT interrupt."]
pub type SPI_MEM_SLV_ST_END_INT_ST_R = crate::BitReader;
#[doc = "Field `SPI_MEM_MST_ST_END_INT_ST` reader - The status bit for SPI_MEM_MST_ST_END_INT interrupt."]
pub type SPI_MEM_MST_ST_END_INT_ST_R = crate::BitReader;
#[doc = "Field `SPI_MEM_ECC_ERR_INT_ST` reader - The status bit for SPI_MEM_ECC_ERR_INT interrupt."]
pub type SPI_MEM_ECC_ERR_INT_ST_R = crate::BitReader;
#[doc = "Field `SPI_MEM_PMS_REJECT_INT_ST` reader - The status bit for SPI_MEM_PMS_REJECT_INT interrupt."]
pub type SPI_MEM_PMS_REJECT_INT_ST_R = crate::BitReader;
#[doc = "Field `SPI_MEM_AXI_RADDR_ERR_INT_ST` reader - The status bit for SPI_MEM_AXI_RADDR_ERR_INT interrupt."]
pub type SPI_MEM_AXI_RADDR_ERR_INT_ST_R = crate::BitReader;
#[doc = "Field `SPI_MEM_AXI_WR_FLASH_ERR_INT_ST` reader - The status bit for SPI_MEM_AXI_WR_FALSH_ERR_INT interrupt."]
pub type SPI_MEM_AXI_WR_FLASH_ERR_INT_ST_R = crate::BitReader;
#[doc = "Field `SPI_MEM_AXI_WADDR_ERR_INT_ST` reader - The status bit for SPI_MEM_AXI_WADDR_ERR_INT interrupt."]
pub type SPI_MEM_AXI_WADDR_ERR_INT_ST_R = crate::BitReader;
#[doc = "Field `SPI_MEM_NANDFLASH_PE_FAIL_INT_ST` reader - The status bit for SPI_MEM_NANDFLASH_PE_FAIL_INT interrupt."]
pub type SPI_MEM_NANDFLASH_PE_FAIL_INT_ST_R = crate::BitReader;
#[doc = "Field `SPI_MEM_NANDFLASH_ECC_ERR_INT_ST` reader - The status bit for SPI_MEM_NANDFLASH_ECC_ERR_INT interrupt."]
pub type SPI_MEM_NANDFLASH_ECC_ERR_INT_ST_R = crate::BitReader;
impl R {
    #[doc = "Bit 3 - The status bit for SPI_MEM_SLV_ST_END_INT interrupt."]
    #[inline(always)]
    pub fn spi_mem_slv_st_end_int_st(&self) -> SPI_MEM_SLV_ST_END_INT_ST_R {
        SPI_MEM_SLV_ST_END_INT_ST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The status bit for SPI_MEM_MST_ST_END_INT interrupt."]
    #[inline(always)]
    pub fn spi_mem_mst_st_end_int_st(&self) -> SPI_MEM_MST_ST_END_INT_ST_R {
        SPI_MEM_MST_ST_END_INT_ST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The status bit for SPI_MEM_ECC_ERR_INT interrupt."]
    #[inline(always)]
    pub fn spi_mem_ecc_err_int_st(&self) -> SPI_MEM_ECC_ERR_INT_ST_R {
        SPI_MEM_ECC_ERR_INT_ST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The status bit for SPI_MEM_PMS_REJECT_INT interrupt."]
    #[inline(always)]
    pub fn spi_mem_pms_reject_int_st(&self) -> SPI_MEM_PMS_REJECT_INT_ST_R {
        SPI_MEM_PMS_REJECT_INT_ST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The status bit for SPI_MEM_AXI_RADDR_ERR_INT interrupt."]
    #[inline(always)]
    pub fn spi_mem_axi_raddr_err_int_st(&self) -> SPI_MEM_AXI_RADDR_ERR_INT_ST_R {
        SPI_MEM_AXI_RADDR_ERR_INT_ST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The status bit for SPI_MEM_AXI_WR_FALSH_ERR_INT interrupt."]
    #[inline(always)]
    pub fn spi_mem_axi_wr_flash_err_int_st(&self) -> SPI_MEM_AXI_WR_FLASH_ERR_INT_ST_R {
        SPI_MEM_AXI_WR_FLASH_ERR_INT_ST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The status bit for SPI_MEM_AXI_WADDR_ERR_INT interrupt."]
    #[inline(always)]
    pub fn spi_mem_axi_waddr_err_int_st(&self) -> SPI_MEM_AXI_WADDR_ERR_INT_ST_R {
        SPI_MEM_AXI_WADDR_ERR_INT_ST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - The status bit for SPI_MEM_NANDFLASH_PE_FAIL_INT interrupt."]
    #[inline(always)]
    pub fn spi_mem_nandflash_pe_fail_int_st(&self) -> SPI_MEM_NANDFLASH_PE_FAIL_INT_ST_R {
        SPI_MEM_NANDFLASH_PE_FAIL_INT_ST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - The status bit for SPI_MEM_NANDFLASH_ECC_ERR_INT interrupt."]
    #[inline(always)]
    pub fn spi_mem_nandflash_ecc_err_int_st(&self) -> SPI_MEM_NANDFLASH_ECC_ERR_INT_ST_R {
        SPI_MEM_NANDFLASH_ECC_ERR_INT_ST_R::new(((self.bits >> 12) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_MEM_INT_ST")
            .field(
                "spi_mem_slv_st_end_int_st",
                &self.spi_mem_slv_st_end_int_st(),
            )
            .field(
                "spi_mem_mst_st_end_int_st",
                &self.spi_mem_mst_st_end_int_st(),
            )
            .field("spi_mem_ecc_err_int_st", &self.spi_mem_ecc_err_int_st())
            .field(
                "spi_mem_pms_reject_int_st",
                &self.spi_mem_pms_reject_int_st(),
            )
            .field(
                "spi_mem_axi_raddr_err_int_st",
                &self.spi_mem_axi_raddr_err_int_st(),
            )
            .field(
                "spi_mem_axi_wr_flash_err_int_st",
                &self.spi_mem_axi_wr_flash_err_int_st(),
            )
            .field(
                "spi_mem_axi_waddr_err_int_st",
                &self.spi_mem_axi_waddr_err_int_st(),
            )
            .field(
                "spi_mem_nandflash_pe_fail_int_st",
                &self.spi_mem_nandflash_pe_fail_int_st(),
            )
            .field(
                "spi_mem_nandflash_ecc_err_int_st",
                &self.spi_mem_nandflash_ecc_err_int_st(),
            )
            .finish()
    }
}
#[doc = "SPI0 interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_mem_int_st::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_MEM_INT_ST_SPEC;
impl crate::RegisterSpec for SPI_MEM_INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_mem_int_st::R`](R) reader structure"]
impl crate::Readable for SPI_MEM_INT_ST_SPEC {}
#[doc = "`reset()` method sets SPI_MEM_INT_ST to value 0"]
impl crate::Resettable for SPI_MEM_INT_ST_SPEC {}
