#[doc = "Register `NAND_FLASH_CMD_LUT9` reader"]
pub type R = crate::R<NAND_FLASH_CMD_LUT9_SPEC>;
#[doc = "Field `NAND_FLASH_LUT_CMD_VALUE9` reader - MSPI NAND FLASH config cmd value at cmd lut address 9."]
pub type NAND_FLASH_LUT_CMD_VALUE9_R = crate::FieldReader<u16>;
#[doc = "Field `NAND_FLASH_LUT_SFSM_ST_EN9` reader - MSPI NAND FLASH config sfsm_st_en at cmd lut address 9.\\[3\\]-ADDR period enable; \\[2\\]-DUMMY period enable; \\[1\\]-DIN period; \\[0\\]-DOUT period."]
pub type NAND_FLASH_LUT_SFSM_ST_EN9_R = crate::FieldReader;
#[doc = "Field `NAND_FLASH_LUT_CMD_LEN9` reader - MSPI NAND FLASH config cmd length at cmd lut address 9."]
pub type NAND_FLASH_LUT_CMD_LEN9_R = crate::FieldReader;
#[doc = "Field `NAND_FLASH_LUT_ADDR_LEN9` reader - MSPI NAND FLASH config address length at cmd lut address 9."]
pub type NAND_FLASH_LUT_ADDR_LEN9_R = crate::FieldReader;
#[doc = "Field `NAND_FLASH_LUT_DATA_LEN9` reader - MSPI NAND FLASH config data length at cmd lut address 9."]
pub type NAND_FLASH_LUT_DATA_LEN9_R = crate::FieldReader;
#[doc = "Field `NAND_FLASH_LUT_BUS_EN9` reader - MSPI NAND FLASH config spi_bus_en at cmd lut address 9,SPI could use DUAL/QUAD mode while enable, SPI could use SINGLE mode while disable.1:Enable. 0:Disable.(Note these registers are described to indicate the SPI_MEM_NAND_FLASH_CMD_LUT9_REG's field. The number of CMD LUT entries can be defined by the user, but cannot exceed 16 )"]
pub type NAND_FLASH_LUT_BUS_EN9_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:15 - MSPI NAND FLASH config cmd value at cmd lut address 9."]
    #[inline(always)]
    pub fn nand_flash_lut_cmd_value9(&self) -> NAND_FLASH_LUT_CMD_VALUE9_R {
        NAND_FLASH_LUT_CMD_VALUE9_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - MSPI NAND FLASH config sfsm_st_en at cmd lut address 9.\\[3\\]-ADDR period enable; \\[2\\]-DUMMY period enable; \\[1\\]-DIN period; \\[0\\]-DOUT period."]
    #[inline(always)]
    pub fn nand_flash_lut_sfsm_st_en9(&self) -> NAND_FLASH_LUT_SFSM_ST_EN9_R {
        NAND_FLASH_LUT_SFSM_ST_EN9_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - MSPI NAND FLASH config cmd length at cmd lut address 9."]
    #[inline(always)]
    pub fn nand_flash_lut_cmd_len9(&self) -> NAND_FLASH_LUT_CMD_LEN9_R {
        NAND_FLASH_LUT_CMD_LEN9_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - MSPI NAND FLASH config address length at cmd lut address 9."]
    #[inline(always)]
    pub fn nand_flash_lut_addr_len9(&self) -> NAND_FLASH_LUT_ADDR_LEN9_R {
        NAND_FLASH_LUT_ADDR_LEN9_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:29 - MSPI NAND FLASH config data length at cmd lut address 9."]
    #[inline(always)]
    pub fn nand_flash_lut_data_len9(&self) -> NAND_FLASH_LUT_DATA_LEN9_R {
        NAND_FLASH_LUT_DATA_LEN9_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 30 - MSPI NAND FLASH config spi_bus_en at cmd lut address 9,SPI could use DUAL/QUAD mode while enable, SPI could use SINGLE mode while disable.1:Enable. 0:Disable.(Note these registers are described to indicate the SPI_MEM_NAND_FLASH_CMD_LUT9_REG's field. The number of CMD LUT entries can be defined by the user, but cannot exceed 16 )"]
    #[inline(always)]
    pub fn nand_flash_lut_bus_en9(&self) -> NAND_FLASH_LUT_BUS_EN9_R {
        NAND_FLASH_LUT_BUS_EN9_R::new(((self.bits >> 30) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NAND_FLASH_CMD_LUT9")
            .field(
                "nand_flash_lut_cmd_value9",
                &self.nand_flash_lut_cmd_value9(),
            )
            .field(
                "nand_flash_lut_sfsm_st_en9",
                &self.nand_flash_lut_sfsm_st_en9(),
            )
            .field("nand_flash_lut_cmd_len9", &self.nand_flash_lut_cmd_len9())
            .field("nand_flash_lut_addr_len9", &self.nand_flash_lut_addr_len9())
            .field("nand_flash_lut_data_len9", &self.nand_flash_lut_data_len9())
            .field("nand_flash_lut_bus_en9", &self.nand_flash_lut_bus_en9())
            .finish()
    }
}
#[doc = "MSPI NAND FLASH CMD LUT control register\n\nYou can [`read`](crate::Reg::read) this register and get [`nand_flash_cmd_lut9::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NAND_FLASH_CMD_LUT9_SPEC;
impl crate::RegisterSpec for NAND_FLASH_CMD_LUT9_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nand_flash_cmd_lut9::R`](R) reader structure"]
impl crate::Readable for NAND_FLASH_CMD_LUT9_SPEC {}
#[doc = "`reset()` method sets NAND_FLASH_CMD_LUT9 to value 0"]
impl crate::Resettable for NAND_FLASH_CMD_LUT9_SPEC {}
