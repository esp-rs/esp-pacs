#[doc = "Register `SPI_MEM_NAND_FLASH_SPI_SEQ7` reader"]
pub type R = crate::R<SPI_MEM_NAND_FLASH_SPI_SEQ7_SPEC>;
#[doc = "Field `SPI_MEM_NAND_FLASH_SEQ_TAIL_FLG7` reader - MSPI NAND FLASH config seq_tail_flg at spi seq index 7.1: The last index for sequence. 0: Not the last index."]
pub type SPI_MEM_NAND_FLASH_SEQ_TAIL_FLG7_R = crate::BitReader;
#[doc = "Field `SPI_MEM_NAND_FLASH_SR_CHK_EN7` reader - MSPI NAND FLASH config sr_chk_en at spi seq index 7. 1: enable 0: disable."]
pub type SPI_MEM_NAND_FLASH_SR_CHK_EN7_R = crate::BitReader;
#[doc = "Field `SPI_MEM_NAND_FLASH_DIN_INDEX7` reader - MSPI NAND FLASH config din_index at spi seq index 7. Use with SPI_MEM_NAND_FLASH_CFG_DATA"]
pub type SPI_MEM_NAND_FLASH_DIN_INDEX7_R = crate::FieldReader;
#[doc = "Field `SPI_MEM_NAND_FLASH_ADDR_INDEX7` reader - MSPI NAND FLASH config addr_index at spi seq index 7. Use with SPI_MEM_NAND_FLASH_SR_ADDR"]
pub type SPI_MEM_NAND_FLASH_ADDR_INDEX7_R = crate::FieldReader;
#[doc = "Field `SPI_MEM_NAND_FLASH_REQ_OR_CFG7` reader - MSPI NAND FLASH config reg_or_cfg at spi seq index 7. 1: AXI/APB request 0: SPI SEQ configuration."]
pub type SPI_MEM_NAND_FLASH_REQ_OR_CFG7_R = crate::BitReader;
#[doc = "Field `SPI_MEM_NAND_FLASH_CMD_INDEX7` reader - MSPI NAND FLASH config spi_cmd_index at spi seq index 7. Use to find SPI command in CMD LUT.(Note these registers are described to indicate the SPI_MEM_NAND_FLASH_SPI_SEQ_REG' fieldd The number of CMD LUT entries can be defined by the user, but cannot exceed 16 )"]
pub type SPI_MEM_NAND_FLASH_CMD_INDEX7_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - MSPI NAND FLASH config seq_tail_flg at spi seq index 7.1: The last index for sequence. 0: Not the last index."]
    #[inline(always)]
    pub fn spi_mem_nand_flash_seq_tail_flg7(&self) -> SPI_MEM_NAND_FLASH_SEQ_TAIL_FLG7_R {
        SPI_MEM_NAND_FLASH_SEQ_TAIL_FLG7_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MSPI NAND FLASH config sr_chk_en at spi seq index 7. 1: enable 0: disable."]
    #[inline(always)]
    pub fn spi_mem_nand_flash_sr_chk_en7(&self) -> SPI_MEM_NAND_FLASH_SR_CHK_EN7_R {
        SPI_MEM_NAND_FLASH_SR_CHK_EN7_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:5 - MSPI NAND FLASH config din_index at spi seq index 7. Use with SPI_MEM_NAND_FLASH_CFG_DATA"]
    #[inline(always)]
    pub fn spi_mem_nand_flash_din_index7(&self) -> SPI_MEM_NAND_FLASH_DIN_INDEX7_R {
        SPI_MEM_NAND_FLASH_DIN_INDEX7_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bits 6:9 - MSPI NAND FLASH config addr_index at spi seq index 7. Use with SPI_MEM_NAND_FLASH_SR_ADDR"]
    #[inline(always)]
    pub fn spi_mem_nand_flash_addr_index7(&self) -> SPI_MEM_NAND_FLASH_ADDR_INDEX7_R {
        SPI_MEM_NAND_FLASH_ADDR_INDEX7_R::new(((self.bits >> 6) & 0x0f) as u8)
    }
    #[doc = "Bit 10 - MSPI NAND FLASH config reg_or_cfg at spi seq index 7. 1: AXI/APB request 0: SPI SEQ configuration."]
    #[inline(always)]
    pub fn spi_mem_nand_flash_req_or_cfg7(&self) -> SPI_MEM_NAND_FLASH_REQ_OR_CFG7_R {
        SPI_MEM_NAND_FLASH_REQ_OR_CFG7_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:14 - MSPI NAND FLASH config spi_cmd_index at spi seq index 7. Use to find SPI command in CMD LUT.(Note these registers are described to indicate the SPI_MEM_NAND_FLASH_SPI_SEQ_REG' fieldd The number of CMD LUT entries can be defined by the user, but cannot exceed 16 )"]
    #[inline(always)]
    pub fn spi_mem_nand_flash_cmd_index7(&self) -> SPI_MEM_NAND_FLASH_CMD_INDEX7_R {
        SPI_MEM_NAND_FLASH_CMD_INDEX7_R::new(((self.bits >> 11) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_MEM_NAND_FLASH_SPI_SEQ7")
            .field(
                "spi_mem_nand_flash_seq_tail_flg7",
                &self.spi_mem_nand_flash_seq_tail_flg7(),
            )
            .field(
                "spi_mem_nand_flash_sr_chk_en7",
                &self.spi_mem_nand_flash_sr_chk_en7(),
            )
            .field(
                "spi_mem_nand_flash_din_index7",
                &self.spi_mem_nand_flash_din_index7(),
            )
            .field(
                "spi_mem_nand_flash_addr_index7",
                &self.spi_mem_nand_flash_addr_index7(),
            )
            .field(
                "spi_mem_nand_flash_req_or_cfg7",
                &self.spi_mem_nand_flash_req_or_cfg7(),
            )
            .field(
                "spi_mem_nand_flash_cmd_index7",
                &self.spi_mem_nand_flash_cmd_index7(),
            )
            .finish()
    }
}
#[doc = "NAND FLASH SPI SEQ control register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_mem_nand_flash_spi_seq7::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_MEM_NAND_FLASH_SPI_SEQ7_SPEC;
impl crate::RegisterSpec for SPI_MEM_NAND_FLASH_SPI_SEQ7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_mem_nand_flash_spi_seq7::R`](R) reader structure"]
impl crate::Readable for SPI_MEM_NAND_FLASH_SPI_SEQ7_SPEC {}
#[doc = "`reset()` method sets SPI_MEM_NAND_FLASH_SPI_SEQ7 to value 0"]
impl crate::Resettable for SPI_MEM_NAND_FLASH_SPI_SEQ7_SPEC {}
