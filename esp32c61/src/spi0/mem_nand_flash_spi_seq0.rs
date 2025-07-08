#[doc = "Register `MEM_NAND_FLASH_SPI_SEQ0` reader"]
pub type R = crate::R<MEM_NAND_FLASH_SPI_SEQ0_SPEC>;
#[doc = "Field `MEM_NAND_FLASH_SEQ_TAIL_FLG0` reader - MSPI NAND FLASH config seq_tail_flg at spi seq index 0.1: The last index for sequence. 0: Not the last index."]
pub type MEM_NAND_FLASH_SEQ_TAIL_FLG0_R = crate::BitReader;
#[doc = "Field `MEM_NAND_FLASH_SR_CHK_EN0` reader - MSPI NAND FLASH config sr_chk_en at spi seq index 0. 1: enable 0: disable."]
pub type MEM_NAND_FLASH_SR_CHK_EN0_R = crate::BitReader;
#[doc = "Field `MEM_NAND_FLASH_DIN_INDEX0` reader - MSPI NAND FLASH config din_index at spi seq index 0. Use with SPI_MEM_NAND_FLASH_CFG_DATA"]
pub type MEM_NAND_FLASH_DIN_INDEX0_R = crate::FieldReader;
#[doc = "Field `MEM_NAND_FLASH_ADDR_INDEX0` reader - MSPI NAND FLASH config addr_index at spi seq index 0. Use with SPI_MEM_NAND_FLASH_SR_ADDR"]
pub type MEM_NAND_FLASH_ADDR_INDEX0_R = crate::FieldReader;
#[doc = "Field `MEM_NAND_FLASH_REQ_OR_CFG0` reader - MSPI NAND FLASH config reg_or_cfg at spi seq index 0. 1: AXI/APB request 0: SPI SEQ configuration."]
pub type MEM_NAND_FLASH_REQ_OR_CFG0_R = crate::BitReader;
#[doc = "Field `MEM_NAND_FLASH_CMD_INDEX0` reader - MSPI NAND FLASH config spi_cmd_index at spi seq index 0. Use to find SPI command in CMD LUT.(Note these registers are described to indicate the SPI_MEM_NAND_FLASH_SPI_SEQ_REG' fieldd The number of CMD LUT entries can be defined by the user, but cannot exceed 16 )"]
pub type MEM_NAND_FLASH_CMD_INDEX0_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - MSPI NAND FLASH config seq_tail_flg at spi seq index 0.1: The last index for sequence. 0: Not the last index."]
    #[inline(always)]
    pub fn mem_nand_flash_seq_tail_flg0(&self) -> MEM_NAND_FLASH_SEQ_TAIL_FLG0_R {
        MEM_NAND_FLASH_SEQ_TAIL_FLG0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MSPI NAND FLASH config sr_chk_en at spi seq index 0. 1: enable 0: disable."]
    #[inline(always)]
    pub fn mem_nand_flash_sr_chk_en0(&self) -> MEM_NAND_FLASH_SR_CHK_EN0_R {
        MEM_NAND_FLASH_SR_CHK_EN0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:5 - MSPI NAND FLASH config din_index at spi seq index 0. Use with SPI_MEM_NAND_FLASH_CFG_DATA"]
    #[inline(always)]
    pub fn mem_nand_flash_din_index0(&self) -> MEM_NAND_FLASH_DIN_INDEX0_R {
        MEM_NAND_FLASH_DIN_INDEX0_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bits 6:9 - MSPI NAND FLASH config addr_index at spi seq index 0. Use with SPI_MEM_NAND_FLASH_SR_ADDR"]
    #[inline(always)]
    pub fn mem_nand_flash_addr_index0(&self) -> MEM_NAND_FLASH_ADDR_INDEX0_R {
        MEM_NAND_FLASH_ADDR_INDEX0_R::new(((self.bits >> 6) & 0x0f) as u8)
    }
    #[doc = "Bit 10 - MSPI NAND FLASH config reg_or_cfg at spi seq index 0. 1: AXI/APB request 0: SPI SEQ configuration."]
    #[inline(always)]
    pub fn mem_nand_flash_req_or_cfg0(&self) -> MEM_NAND_FLASH_REQ_OR_CFG0_R {
        MEM_NAND_FLASH_REQ_OR_CFG0_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:14 - MSPI NAND FLASH config spi_cmd_index at spi seq index 0. Use to find SPI command in CMD LUT.(Note these registers are described to indicate the SPI_MEM_NAND_FLASH_SPI_SEQ_REG' fieldd The number of CMD LUT entries can be defined by the user, but cannot exceed 16 )"]
    #[inline(always)]
    pub fn mem_nand_flash_cmd_index0(&self) -> MEM_NAND_FLASH_CMD_INDEX0_R {
        MEM_NAND_FLASH_CMD_INDEX0_R::new(((self.bits >> 11) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MEM_NAND_FLASH_SPI_SEQ0")
            .field(
                "mem_nand_flash_seq_tail_flg0",
                &self.mem_nand_flash_seq_tail_flg0(),
            )
            .field(
                "mem_nand_flash_sr_chk_en0",
                &self.mem_nand_flash_sr_chk_en0(),
            )
            .field(
                "mem_nand_flash_din_index0",
                &self.mem_nand_flash_din_index0(),
            )
            .field(
                "mem_nand_flash_addr_index0",
                &self.mem_nand_flash_addr_index0(),
            )
            .field(
                "mem_nand_flash_req_or_cfg0",
                &self.mem_nand_flash_req_or_cfg0(),
            )
            .field(
                "mem_nand_flash_cmd_index0",
                &self.mem_nand_flash_cmd_index0(),
            )
            .finish()
    }
}
#[doc = "NAND FLASH SPI SEQ control register\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_nand_flash_spi_seq0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MEM_NAND_FLASH_SPI_SEQ0_SPEC;
impl crate::RegisterSpec for MEM_NAND_FLASH_SPI_SEQ0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_nand_flash_spi_seq0::R`](R) reader structure"]
impl crate::Readable for MEM_NAND_FLASH_SPI_SEQ0_SPEC {}
#[doc = "`reset()` method sets MEM_NAND_FLASH_SPI_SEQ0 to value 0"]
impl crate::Resettable for MEM_NAND_FLASH_SPI_SEQ0_SPEC {}
