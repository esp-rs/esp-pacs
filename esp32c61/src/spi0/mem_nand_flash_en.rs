#[doc = "Register `MEM_NAND_FLASH_EN` reader"]
pub type R = crate::R<MEM_NAND_FLASH_EN_SPEC>;
#[doc = "Field `MEM_NAND_FLASH_EN` reader - NAND FLASH function enable signal. 1: Enable NAND FLASH, Disable NOR FLASH. 0: Disable NAND FLASH, Enable NOR FLASH."]
pub type MEM_NAND_FLASH_EN_R = crate::BitReader;
#[doc = "Field `MEM_NAND_FLASH_SEQ_HD_INDEX` reader - NAND FLASH spi seq head index configure register. Every 5 bits represent the 1st index of a SPI CMD sequence.\\[14:10\\]:usr; \\[9:5\\]:axi_rd; \\[4:0\\]:axi_wr."]
pub type MEM_NAND_FLASH_SEQ_HD_INDEX_R = crate::FieldReader<u16>;
#[doc = "Field `MEM_NAND_FLASH_SEQ_USR_TRIG` reader - NAND FLASH spi seq user trigger configure register. SPI_MEM_NAND_FLASH_SEQ_USR_TRIG is corresponds to SPI_MEM_NAND_FLASH_SEQ_HD_INDEX\\[14:10\\].1: enable 0: disable."]
pub type MEM_NAND_FLASH_SEQ_USR_TRIG_R = crate::BitReader;
#[doc = "Field `MEM_NAND_FLASH_LUT_EN` reader - NAND FLASH spi seq & cmd lut cfg en. 1: enable 0: disable."]
pub type MEM_NAND_FLASH_LUT_EN_R = crate::BitReader;
#[doc = "Field `MEM_NAND_FLASH_SEQ_USR_WEND` reader - Used with SPI_MEM_NAND_FLASH_SEQ_USR_TRIG to indecate the last page program ,and to excute page excute. 1: write end 0: write in a page size."]
pub type MEM_NAND_FLASH_SEQ_USR_WEND_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - NAND FLASH function enable signal. 1: Enable NAND FLASH, Disable NOR FLASH. 0: Disable NAND FLASH, Enable NOR FLASH."]
    #[inline(always)]
    pub fn mem_nand_flash_en(&self) -> MEM_NAND_FLASH_EN_R {
        MEM_NAND_FLASH_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:15 - NAND FLASH spi seq head index configure register. Every 5 bits represent the 1st index of a SPI CMD sequence.\\[14:10\\]:usr; \\[9:5\\]:axi_rd; \\[4:0\\]:axi_wr."]
    #[inline(always)]
    pub fn mem_nand_flash_seq_hd_index(&self) -> MEM_NAND_FLASH_SEQ_HD_INDEX_R {
        MEM_NAND_FLASH_SEQ_HD_INDEX_R::new(((self.bits >> 1) & 0x7fff) as u16)
    }
    #[doc = "Bit 16 - NAND FLASH spi seq user trigger configure register. SPI_MEM_NAND_FLASH_SEQ_USR_TRIG is corresponds to SPI_MEM_NAND_FLASH_SEQ_HD_INDEX\\[14:10\\].1: enable 0: disable."]
    #[inline(always)]
    pub fn mem_nand_flash_seq_usr_trig(&self) -> MEM_NAND_FLASH_SEQ_USR_TRIG_R {
        MEM_NAND_FLASH_SEQ_USR_TRIG_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - NAND FLASH spi seq & cmd lut cfg en. 1: enable 0: disable."]
    #[inline(always)]
    pub fn mem_nand_flash_lut_en(&self) -> MEM_NAND_FLASH_LUT_EN_R {
        MEM_NAND_FLASH_LUT_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Used with SPI_MEM_NAND_FLASH_SEQ_USR_TRIG to indecate the last page program ,and to excute page excute. 1: write end 0: write in a page size."]
    #[inline(always)]
    pub fn mem_nand_flash_seq_usr_wend(&self) -> MEM_NAND_FLASH_SEQ_USR_WEND_R {
        MEM_NAND_FLASH_SEQ_USR_WEND_R::new(((self.bits >> 18) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MEM_NAND_FLASH_EN")
            .field("mem_nand_flash_en", &self.mem_nand_flash_en())
            .field(
                "mem_nand_flash_seq_hd_index",
                &self.mem_nand_flash_seq_hd_index(),
            )
            .field(
                "mem_nand_flash_seq_usr_trig",
                &self.mem_nand_flash_seq_usr_trig(),
            )
            .field("mem_nand_flash_lut_en", &self.mem_nand_flash_lut_en())
            .field(
                "mem_nand_flash_seq_usr_wend",
                &self.mem_nand_flash_seq_usr_wend(),
            )
            .finish()
    }
}
#[doc = "NAND FLASH control register\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_nand_flash_en::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MEM_NAND_FLASH_EN_SPEC;
impl crate::RegisterSpec for MEM_NAND_FLASH_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_nand_flash_en::R`](R) reader structure"]
impl crate::Readable for MEM_NAND_FLASH_EN_SPEC {}
#[doc = "`reset()` method sets MEM_NAND_FLASH_EN to value 0xfffe"]
impl crate::Resettable for MEM_NAND_FLASH_EN_SPEC {
    const RESET_VALUE: u32 = 0xfffe;
}
