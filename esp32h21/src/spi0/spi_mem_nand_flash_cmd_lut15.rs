#[doc = "Register `SPI_MEM_NAND_FLASH_CMD_LUT15` reader"]
pub type R = crate::R<SPI_MEM_NAND_FLASH_CMD_LUT15_SPEC>;
#[doc = "Field `SPI_MEM_NAND_FLASH_LUT_CMD_VALUE15` reader - MSPI NAND FLASH config cmd value at cmd lut address 15."]
pub type SPI_MEM_NAND_FLASH_LUT_CMD_VALUE15_R = crate::FieldReader<u16>;
#[doc = "Field `SPI_MEM_NAND_FLASH_LUT_SFSM_ST_EN15` reader - MSPI NAND FLASH config sfsm_st_en at cmd lut address 15.\\[3\\]-ADDR period enable; \\[2\\]-DUMMY period enable; \\[1\\]-DIN period; \\[0\\]-DOUT period."]
pub type SPI_MEM_NAND_FLASH_LUT_SFSM_ST_EN15_R = crate::FieldReader;
#[doc = "Field `SPI_MEM_NAND_FLASH_LUT_CMD_LEN15` reader - MSPI NAND FLASH config cmd length at cmd lut address 15."]
pub type SPI_MEM_NAND_FLASH_LUT_CMD_LEN15_R = crate::FieldReader;
#[doc = "Field `SPI_MEM_NAND_FLASH_LUT_ADDR_LEN15` reader - MSPI NAND FLASH config address length at cmd lut address 15."]
pub type SPI_MEM_NAND_FLASH_LUT_ADDR_LEN15_R = crate::FieldReader;
#[doc = "Field `SPI_MEM_NAND_FLASH_LUT_DATA_LEN15` reader - MSPI NAND FLASH config data length at cmd lut address 15."]
pub type SPI_MEM_NAND_FLASH_LUT_DATA_LEN15_R = crate::FieldReader;
#[doc = "Field `SPI_MEM_NAND_FLASH_LUT_BUS_EN15` reader - MSPI NAND FLASH config spi_bus_en at cmd lut address 15,SPI could use DUAL/QUAD mode while enable, SPI could use SINGLE mode while disable.1:Enable. 0:Disable.(Note these registers are described to indicate the SPI_MEM_NAND_FLASH_CMD_LUT15_REG's field. The number of CMD LUT entries can be defined by the user, but cannot exceed 16 )"]
pub type SPI_MEM_NAND_FLASH_LUT_BUS_EN15_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:15 - MSPI NAND FLASH config cmd value at cmd lut address 15."]
    #[inline(always)]
    pub fn spi_mem_nand_flash_lut_cmd_value15(&self) -> SPI_MEM_NAND_FLASH_LUT_CMD_VALUE15_R {
        SPI_MEM_NAND_FLASH_LUT_CMD_VALUE15_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - MSPI NAND FLASH config sfsm_st_en at cmd lut address 15.\\[3\\]-ADDR period enable; \\[2\\]-DUMMY period enable; \\[1\\]-DIN period; \\[0\\]-DOUT period."]
    #[inline(always)]
    pub fn spi_mem_nand_flash_lut_sfsm_st_en15(&self) -> SPI_MEM_NAND_FLASH_LUT_SFSM_ST_EN15_R {
        SPI_MEM_NAND_FLASH_LUT_SFSM_ST_EN15_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - MSPI NAND FLASH config cmd length at cmd lut address 15."]
    #[inline(always)]
    pub fn spi_mem_nand_flash_lut_cmd_len15(&self) -> SPI_MEM_NAND_FLASH_LUT_CMD_LEN15_R {
        SPI_MEM_NAND_FLASH_LUT_CMD_LEN15_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - MSPI NAND FLASH config address length at cmd lut address 15."]
    #[inline(always)]
    pub fn spi_mem_nand_flash_lut_addr_len15(&self) -> SPI_MEM_NAND_FLASH_LUT_ADDR_LEN15_R {
        SPI_MEM_NAND_FLASH_LUT_ADDR_LEN15_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:29 - MSPI NAND FLASH config data length at cmd lut address 15."]
    #[inline(always)]
    pub fn spi_mem_nand_flash_lut_data_len15(&self) -> SPI_MEM_NAND_FLASH_LUT_DATA_LEN15_R {
        SPI_MEM_NAND_FLASH_LUT_DATA_LEN15_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 30 - MSPI NAND FLASH config spi_bus_en at cmd lut address 15,SPI could use DUAL/QUAD mode while enable, SPI could use SINGLE mode while disable.1:Enable. 0:Disable.(Note these registers are described to indicate the SPI_MEM_NAND_FLASH_CMD_LUT15_REG's field. The number of CMD LUT entries can be defined by the user, but cannot exceed 16 )"]
    #[inline(always)]
    pub fn spi_mem_nand_flash_lut_bus_en15(&self) -> SPI_MEM_NAND_FLASH_LUT_BUS_EN15_R {
        SPI_MEM_NAND_FLASH_LUT_BUS_EN15_R::new(((self.bits >> 30) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_MEM_NAND_FLASH_CMD_LUT15")
            .field(
                "spi_mem_nand_flash_lut_cmd_value15",
                &self.spi_mem_nand_flash_lut_cmd_value15(),
            )
            .field(
                "spi_mem_nand_flash_lut_sfsm_st_en15",
                &self.spi_mem_nand_flash_lut_sfsm_st_en15(),
            )
            .field(
                "spi_mem_nand_flash_lut_cmd_len15",
                &self.spi_mem_nand_flash_lut_cmd_len15(),
            )
            .field(
                "spi_mem_nand_flash_lut_addr_len15",
                &self.spi_mem_nand_flash_lut_addr_len15(),
            )
            .field(
                "spi_mem_nand_flash_lut_data_len15",
                &self.spi_mem_nand_flash_lut_data_len15(),
            )
            .field(
                "spi_mem_nand_flash_lut_bus_en15",
                &self.spi_mem_nand_flash_lut_bus_en15(),
            )
            .finish()
    }
}
#[doc = "MSPI NAND FLASH CMD LUT control register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_mem_nand_flash_cmd_lut15::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_MEM_NAND_FLASH_CMD_LUT15_SPEC;
impl crate::RegisterSpec for SPI_MEM_NAND_FLASH_CMD_LUT15_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_mem_nand_flash_cmd_lut15::R`](R) reader structure"]
impl crate::Readable for SPI_MEM_NAND_FLASH_CMD_LUT15_SPEC {}
#[doc = "`reset()` method sets SPI_MEM_NAND_FLASH_CMD_LUT15 to value 0"]
impl crate::Resettable for SPI_MEM_NAND_FLASH_CMD_LUT15_SPEC {}
