#[doc = "Register `NAND_FLASH_SPI_SEQ4` reader"]
pub type R = crate::R<NAND_FLASH_SPI_SEQ4_SPEC>;
#[doc = "Field `NAND_FLASH_SEQ_TAIL_FLG4` reader - MSPI NAND FLASH config seq_tail_flg at spi seq index 4.1: The last index for sequence. 0: Not the last index."]
pub type NAND_FLASH_SEQ_TAIL_FLG4_R = crate::BitReader;
#[doc = "Field `NAND_FLASH_SR_CHK_EN4` reader - MSPI NAND FLASH config sr_chk_en at spi seq index 4. 1: enable 0: disable."]
pub type NAND_FLASH_SR_CHK_EN4_R = crate::BitReader;
#[doc = "Field `NAND_FLASH_DIN_INDEX4` reader - MSPI NAND FLASH config din_index at spi seq index 4. Use with SPI_MEM_NAND_FLASH_CFG_DATA"]
pub type NAND_FLASH_DIN_INDEX4_R = crate::FieldReader;
#[doc = "Field `NAND_FLASH_ADDR_INDEX4` reader - MSPI NAND FLASH config addr_index at spi seq index 4. Use with SPI_MEM_NAND_FLASH_SR_ADDR"]
pub type NAND_FLASH_ADDR_INDEX4_R = crate::FieldReader;
#[doc = "Field `NAND_FLASH_REQ_OR_CFG4` reader - MSPI NAND FLASH config reg_or_cfg at spi seq index 4. 1: AXI/APB request 0: SPI SEQ configuration."]
pub type NAND_FLASH_REQ_OR_CFG4_R = crate::BitReader;
#[doc = "Field `NAND_FLASH_CMD_INDEX4` reader - MSPI NAND FLASH config spi_cmd_index at spi seq index 4. Use to find SPI command in CMD LUT.(Note these registers are described to indicate the SPI_MEM_NAND_FLASH_SPI_SEQ_REG' fieldd The number of CMD LUT entries can be defined by the user, but cannot exceed 16 )"]
pub type NAND_FLASH_CMD_INDEX4_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - MSPI NAND FLASH config seq_tail_flg at spi seq index 4.1: The last index for sequence. 0: Not the last index."]
    #[inline(always)]
    pub fn nand_flash_seq_tail_flg4(&self) -> NAND_FLASH_SEQ_TAIL_FLG4_R {
        NAND_FLASH_SEQ_TAIL_FLG4_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MSPI NAND FLASH config sr_chk_en at spi seq index 4. 1: enable 0: disable."]
    #[inline(always)]
    pub fn nand_flash_sr_chk_en4(&self) -> NAND_FLASH_SR_CHK_EN4_R {
        NAND_FLASH_SR_CHK_EN4_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:5 - MSPI NAND FLASH config din_index at spi seq index 4. Use with SPI_MEM_NAND_FLASH_CFG_DATA"]
    #[inline(always)]
    pub fn nand_flash_din_index4(&self) -> NAND_FLASH_DIN_INDEX4_R {
        NAND_FLASH_DIN_INDEX4_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bits 6:9 - MSPI NAND FLASH config addr_index at spi seq index 4. Use with SPI_MEM_NAND_FLASH_SR_ADDR"]
    #[inline(always)]
    pub fn nand_flash_addr_index4(&self) -> NAND_FLASH_ADDR_INDEX4_R {
        NAND_FLASH_ADDR_INDEX4_R::new(((self.bits >> 6) & 0x0f) as u8)
    }
    #[doc = "Bit 10 - MSPI NAND FLASH config reg_or_cfg at spi seq index 4. 1: AXI/APB request 0: SPI SEQ configuration."]
    #[inline(always)]
    pub fn nand_flash_req_or_cfg4(&self) -> NAND_FLASH_REQ_OR_CFG4_R {
        NAND_FLASH_REQ_OR_CFG4_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:14 - MSPI NAND FLASH config spi_cmd_index at spi seq index 4. Use to find SPI command in CMD LUT.(Note these registers are described to indicate the SPI_MEM_NAND_FLASH_SPI_SEQ_REG' fieldd The number of CMD LUT entries can be defined by the user, but cannot exceed 16 )"]
    #[inline(always)]
    pub fn nand_flash_cmd_index4(&self) -> NAND_FLASH_CMD_INDEX4_R {
        NAND_FLASH_CMD_INDEX4_R::new(((self.bits >> 11) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NAND_FLASH_SPI_SEQ4")
            .field("nand_flash_seq_tail_flg4", &self.nand_flash_seq_tail_flg4())
            .field("nand_flash_sr_chk_en4", &self.nand_flash_sr_chk_en4())
            .field("nand_flash_din_index4", &self.nand_flash_din_index4())
            .field("nand_flash_addr_index4", &self.nand_flash_addr_index4())
            .field("nand_flash_req_or_cfg4", &self.nand_flash_req_or_cfg4())
            .field("nand_flash_cmd_index4", &self.nand_flash_cmd_index4())
            .finish()
    }
}
#[doc = "NAND FLASH SPI SEQ control register\n\nYou can [`read`](crate::Reg::read) this register and get [`nand_flash_spi_seq4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NAND_FLASH_SPI_SEQ4_SPEC;
impl crate::RegisterSpec for NAND_FLASH_SPI_SEQ4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nand_flash_spi_seq4::R`](R) reader structure"]
impl crate::Readable for NAND_FLASH_SPI_SEQ4_SPEC {}
#[doc = "`reset()` method sets NAND_FLASH_SPI_SEQ4 to value 0"]
impl crate::Resettable for NAND_FLASH_SPI_SEQ4_SPEC {}
