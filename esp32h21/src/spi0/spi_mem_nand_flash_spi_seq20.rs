#[doc = "Register `SPI_MEM_NAND_FLASH_SPI_SEQ20` reader"]
pub type R = crate::R<SPI_MEM_NAND_FLASH_SPI_SEQ20_SPEC>;
#[doc = "Field `SPI_MEM_NAND_FLASH_SEQ_TAIL_FLG20` reader - MSPI NAND FLASH config seq_tail_flg at spi seq index 20.1: The last index for sequence. 0: Not the last index."]
pub type SPI_MEM_NAND_FLASH_SEQ_TAIL_FLG20_R = crate::BitReader;
#[doc = "Field `SPI_MEM_NAND_FLASH_SR_CHK_EN20` reader - MSPI NAND FLASH config sr_chk_en at spi seq index 20. 1: enable 0: disable."]
pub type SPI_MEM_NAND_FLASH_SR_CHK_EN20_R = crate::BitReader;
#[doc = "Field `SPI_MEM_NAND_FLASH_DIN_INDEX20` reader - MSPI NAND FLASH config din_index at spi seq index 20. Use with SPI_MEM_NAND_FLASH_CFG_DATA"]
pub type SPI_MEM_NAND_FLASH_DIN_INDEX20_R = crate::FieldReader;
#[doc = "Field `SPI_MEM_NAND_FLASH_ADDR_INDEX20` reader - MSPI NAND FLASH config addr_index at spi seq index 20. Use with SPI_MEM_NAND_FLASH_SR_ADDR"]
pub type SPI_MEM_NAND_FLASH_ADDR_INDEX20_R = crate::FieldReader;
#[doc = "Field `SPI_MEM_NAND_FLASH_REQ_OR_CFG20` reader - MSPI NAND FLASH config reg_or_cfg at spi seq index 20. 1: AXI/APB request 0: SPI SEQ configuration."]
pub type SPI_MEM_NAND_FLASH_REQ_OR_CFG20_R = crate::BitReader;
#[doc = "Field `SPI_MEM_NAND_FLASH_CMD_INDEX20` reader - MSPI NAND FLASH config spi_cmd_index at spi seq index 20. Use to find SPI command in CMD LUT.(Note these registers are described to indicate the SPI_MEM_NAND_FLASH_SPI_SEQ_REG' fieldd The number of CMD LUT entries can be defined by the user, but cannot exceed 16 )"]
pub type SPI_MEM_NAND_FLASH_CMD_INDEX20_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - MSPI NAND FLASH config seq_tail_flg at spi seq index 20.1: The last index for sequence. 0: Not the last index."]
    #[inline(always)]
    pub fn spi_mem_nand_flash_seq_tail_flg20(&self) -> SPI_MEM_NAND_FLASH_SEQ_TAIL_FLG20_R {
        SPI_MEM_NAND_FLASH_SEQ_TAIL_FLG20_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MSPI NAND FLASH config sr_chk_en at spi seq index 20. 1: enable 0: disable."]
    #[inline(always)]
    pub fn spi_mem_nand_flash_sr_chk_en20(&self) -> SPI_MEM_NAND_FLASH_SR_CHK_EN20_R {
        SPI_MEM_NAND_FLASH_SR_CHK_EN20_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:5 - MSPI NAND FLASH config din_index at spi seq index 20. Use with SPI_MEM_NAND_FLASH_CFG_DATA"]
    #[inline(always)]
    pub fn spi_mem_nand_flash_din_index20(&self) -> SPI_MEM_NAND_FLASH_DIN_INDEX20_R {
        SPI_MEM_NAND_FLASH_DIN_INDEX20_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bits 6:9 - MSPI NAND FLASH config addr_index at spi seq index 20. Use with SPI_MEM_NAND_FLASH_SR_ADDR"]
    #[inline(always)]
    pub fn spi_mem_nand_flash_addr_index20(&self) -> SPI_MEM_NAND_FLASH_ADDR_INDEX20_R {
        SPI_MEM_NAND_FLASH_ADDR_INDEX20_R::new(((self.bits >> 6) & 0x0f) as u8)
    }
    #[doc = "Bit 10 - MSPI NAND FLASH config reg_or_cfg at spi seq index 20. 1: AXI/APB request 0: SPI SEQ configuration."]
    #[inline(always)]
    pub fn spi_mem_nand_flash_req_or_cfg20(&self) -> SPI_MEM_NAND_FLASH_REQ_OR_CFG20_R {
        SPI_MEM_NAND_FLASH_REQ_OR_CFG20_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:14 - MSPI NAND FLASH config spi_cmd_index at spi seq index 20. Use to find SPI command in CMD LUT.(Note these registers are described to indicate the SPI_MEM_NAND_FLASH_SPI_SEQ_REG' fieldd The number of CMD LUT entries can be defined by the user, but cannot exceed 16 )"]
    #[inline(always)]
    pub fn spi_mem_nand_flash_cmd_index20(&self) -> SPI_MEM_NAND_FLASH_CMD_INDEX20_R {
        SPI_MEM_NAND_FLASH_CMD_INDEX20_R::new(((self.bits >> 11) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_MEM_NAND_FLASH_SPI_SEQ20")
            .field(
                "spi_mem_nand_flash_seq_tail_flg20",
                &self.spi_mem_nand_flash_seq_tail_flg20(),
            )
            .field(
                "spi_mem_nand_flash_sr_chk_en20",
                &self.spi_mem_nand_flash_sr_chk_en20(),
            )
            .field(
                "spi_mem_nand_flash_din_index20",
                &self.spi_mem_nand_flash_din_index20(),
            )
            .field(
                "spi_mem_nand_flash_addr_index20",
                &self.spi_mem_nand_flash_addr_index20(),
            )
            .field(
                "spi_mem_nand_flash_req_or_cfg20",
                &self.spi_mem_nand_flash_req_or_cfg20(),
            )
            .field(
                "spi_mem_nand_flash_cmd_index20",
                &self.spi_mem_nand_flash_cmd_index20(),
            )
            .finish()
    }
}
#[doc = "NAND FLASH SPI SEQ control register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_mem_nand_flash_spi_seq20::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_MEM_NAND_FLASH_SPI_SEQ20_SPEC;
impl crate::RegisterSpec for SPI_MEM_NAND_FLASH_SPI_SEQ20_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_mem_nand_flash_spi_seq20::R`](R) reader structure"]
impl crate::Readable for SPI_MEM_NAND_FLASH_SPI_SEQ20_SPEC {}
#[doc = "`reset()` method sets SPI_MEM_NAND_FLASH_SPI_SEQ20 to value 0"]
impl crate::Resettable for SPI_MEM_NAND_FLASH_SPI_SEQ20_SPEC {}
