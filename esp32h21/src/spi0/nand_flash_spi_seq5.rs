#[doc = "Register `NAND_FLASH_SPI_SEQ5` reader"]
pub type R = crate::R<NAND_FLASH_SPI_SEQ5_SPEC>;
#[doc = "Field `NAND_FLASH_SEQ_TAIL_FLG5` reader - MSPI NAND FLASH config seq_tail_flg at spi seq index 5.1: The last index for sequence. 0: Not the last index."]
pub type NAND_FLASH_SEQ_TAIL_FLG5_R = crate::BitReader;
#[doc = "Field `NAND_FLASH_SR_CHK_EN5` reader - MSPI NAND FLASH config sr_chk_en at spi seq index 5. 1: enable 0: disable."]
pub type NAND_FLASH_SR_CHK_EN5_R = crate::BitReader;
#[doc = "Field `NAND_FLASH_DIN_INDEX5` reader - MSPI NAND FLASH config din_index at spi seq index 5. Use with SPI_MEM_NAND_FLASH_CFG_DATA"]
pub type NAND_FLASH_DIN_INDEX5_R = crate::FieldReader;
#[doc = "Field `NAND_FLASH_ADDR_INDEX5` reader - MSPI NAND FLASH config addr_index at spi seq index 5. Use with SPI_MEM_NAND_FLASH_SR_ADDR"]
pub type NAND_FLASH_ADDR_INDEX5_R = crate::FieldReader;
#[doc = "Field `NAND_FLASH_REQ_OR_CFG5` reader - MSPI NAND FLASH config reg_or_cfg at spi seq index 5. 1: AXI/APB request 0: SPI SEQ configuration."]
pub type NAND_FLASH_REQ_OR_CFG5_R = crate::BitReader;
#[doc = "Field `NAND_FLASH_CMD_INDEX5` reader - MSPI NAND FLASH config spi_cmd_index at spi seq index 5. Use to find SPI command in CMD LUT.(Note these registers are described to indicate the SPI_MEM_NAND_FLASH_SPI_SEQ_REG' fieldd The number of CMD LUT entries can be defined by the user, but cannot exceed 16 )"]
pub type NAND_FLASH_CMD_INDEX5_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - MSPI NAND FLASH config seq_tail_flg at spi seq index 5.1: The last index for sequence. 0: Not the last index."]
    #[inline(always)]
    pub fn nand_flash_seq_tail_flg5(&self) -> NAND_FLASH_SEQ_TAIL_FLG5_R {
        NAND_FLASH_SEQ_TAIL_FLG5_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MSPI NAND FLASH config sr_chk_en at spi seq index 5. 1: enable 0: disable."]
    #[inline(always)]
    pub fn nand_flash_sr_chk_en5(&self) -> NAND_FLASH_SR_CHK_EN5_R {
        NAND_FLASH_SR_CHK_EN5_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:5 - MSPI NAND FLASH config din_index at spi seq index 5. Use with SPI_MEM_NAND_FLASH_CFG_DATA"]
    #[inline(always)]
    pub fn nand_flash_din_index5(&self) -> NAND_FLASH_DIN_INDEX5_R {
        NAND_FLASH_DIN_INDEX5_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bits 6:9 - MSPI NAND FLASH config addr_index at spi seq index 5. Use with SPI_MEM_NAND_FLASH_SR_ADDR"]
    #[inline(always)]
    pub fn nand_flash_addr_index5(&self) -> NAND_FLASH_ADDR_INDEX5_R {
        NAND_FLASH_ADDR_INDEX5_R::new(((self.bits >> 6) & 0x0f) as u8)
    }
    #[doc = "Bit 10 - MSPI NAND FLASH config reg_or_cfg at spi seq index 5. 1: AXI/APB request 0: SPI SEQ configuration."]
    #[inline(always)]
    pub fn nand_flash_req_or_cfg5(&self) -> NAND_FLASH_REQ_OR_CFG5_R {
        NAND_FLASH_REQ_OR_CFG5_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:14 - MSPI NAND FLASH config spi_cmd_index at spi seq index 5. Use to find SPI command in CMD LUT.(Note these registers are described to indicate the SPI_MEM_NAND_FLASH_SPI_SEQ_REG' fieldd The number of CMD LUT entries can be defined by the user, but cannot exceed 16 )"]
    #[inline(always)]
    pub fn nand_flash_cmd_index5(&self) -> NAND_FLASH_CMD_INDEX5_R {
        NAND_FLASH_CMD_INDEX5_R::new(((self.bits >> 11) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NAND_FLASH_SPI_SEQ5")
            .field("nand_flash_seq_tail_flg5", &self.nand_flash_seq_tail_flg5())
            .field("nand_flash_sr_chk_en5", &self.nand_flash_sr_chk_en5())
            .field("nand_flash_din_index5", &self.nand_flash_din_index5())
            .field("nand_flash_addr_index5", &self.nand_flash_addr_index5())
            .field("nand_flash_req_or_cfg5", &self.nand_flash_req_or_cfg5())
            .field("nand_flash_cmd_index5", &self.nand_flash_cmd_index5())
            .finish()
    }
}
#[doc = "NAND FLASH SPI SEQ control register\n\nYou can [`read`](crate::Reg::read) this register and get [`nand_flash_spi_seq5::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NAND_FLASH_SPI_SEQ5_SPEC;
impl crate::RegisterSpec for NAND_FLASH_SPI_SEQ5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nand_flash_spi_seq5::R`](R) reader structure"]
impl crate::Readable for NAND_FLASH_SPI_SEQ5_SPEC {}
#[doc = "`reset()` method sets NAND_FLASH_SPI_SEQ5 to value 0"]
impl crate::Resettable for NAND_FLASH_SPI_SEQ5_SPEC {}
