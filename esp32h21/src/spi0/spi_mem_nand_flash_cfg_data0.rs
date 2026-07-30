#[doc = "Register `SPI_MEM_NAND_FLASH_CFG_DATA0` reader"]
pub type R = crate::R<SPI_MEM_NAND_FLASH_CFG_DATA0_SPEC>;
#[doc = "Field `SPI_MEM_NAND_FLASH_CFG_DATA0` reader - configure data for SPI SEQ din/dout need. The data could be use to configure NAND FLASH or compare read data"]
pub type SPI_MEM_NAND_FLASH_CFG_DATA0_R = crate::FieldReader<u16>;
#[doc = "Field `SPI_MEM_NAND_FLASH_CFG_DATA1` reader - configure data for SPI SEQ din/dout need. The data could be use to configure NAND FLASH or compare read data"]
pub type SPI_MEM_NAND_FLASH_CFG_DATA1_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - configure data for SPI SEQ din/dout need. The data could be use to configure NAND FLASH or compare read data"]
    #[inline(always)]
    pub fn spi_mem_nand_flash_cfg_data0(&self) -> SPI_MEM_NAND_FLASH_CFG_DATA0_R {
        SPI_MEM_NAND_FLASH_CFG_DATA0_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - configure data for SPI SEQ din/dout need. The data could be use to configure NAND FLASH or compare read data"]
    #[inline(always)]
    pub fn spi_mem_nand_flash_cfg_data1(&self) -> SPI_MEM_NAND_FLASH_CFG_DATA1_R {
        SPI_MEM_NAND_FLASH_CFG_DATA1_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_MEM_NAND_FLASH_CFG_DATA0")
            .field(
                "spi_mem_nand_flash_cfg_data0",
                &self.spi_mem_nand_flash_cfg_data0(),
            )
            .field(
                "spi_mem_nand_flash_cfg_data1",
                &self.spi_mem_nand_flash_cfg_data1(),
            )
            .finish()
    }
}
#[doc = "NAND FLASH SPI SEQ control register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_mem_nand_flash_cfg_data0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_MEM_NAND_FLASH_CFG_DATA0_SPEC;
impl crate::RegisterSpec for SPI_MEM_NAND_FLASH_CFG_DATA0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_mem_nand_flash_cfg_data0::R`](R) reader structure"]
impl crate::Readable for SPI_MEM_NAND_FLASH_CFG_DATA0_SPEC {}
#[doc = "`reset()` method sets SPI_MEM_NAND_FLASH_CFG_DATA0 to value 0"]
impl crate::Resettable for SPI_MEM_NAND_FLASH_CFG_DATA0_SPEC {}
