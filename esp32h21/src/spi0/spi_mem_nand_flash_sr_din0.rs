#[doc = "Register `SPI_MEM_NAND_FLASH_SR_DIN0` reader"]
pub type R = crate::R<SPI_MEM_NAND_FLASH_SR_DIN0_SPEC>;
#[doc = "Field `SPI_MEM_NAND_FLASH_SR_DIN0` reader - spi read state register data to this register for SPI SEQ need. SPI_MEM_NAND_FLASH_SR_DIN0_REG corresponds to SPI_MEM_NAND_FLASH_SR_ADDR0_REG."]
pub type SPI_MEM_NAND_FLASH_SR_DIN0_R = crate::FieldReader;
#[doc = "Field `SPI_MEM_NAND_FLASH_SR_DIN1` reader - spi read state register data to this register for SPI SEQ need. SPI_MEM_NAND_FLASH_SR_DIN0_REG corresponds to SPI_MEM_NAND_FLASH_SR_ADDR0_REG."]
pub type SPI_MEM_NAND_FLASH_SR_DIN1_R = crate::FieldReader;
#[doc = "Field `SPI_MEM_NAND_FLASH_SR_DIN2` reader - spi read state register data to this register for SPI SEQ need. SPI_MEM_NAND_FLASH_SR_DIN0_REG corresponds to SPI_MEM_NAND_FLASH_SR_ADDR0_REG."]
pub type SPI_MEM_NAND_FLASH_SR_DIN2_R = crate::FieldReader;
#[doc = "Field `SPI_MEM_NAND_FLASH_SR_DIN3` reader - spi read state register data to this register for SPI SEQ need. SPI_MEM_NAND_FLASH_SR_DIN0_REG corresponds to SPI_MEM_NAND_FLASH_SR_ADDR0_REG."]
pub type SPI_MEM_NAND_FLASH_SR_DIN3_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - spi read state register data to this register for SPI SEQ need. SPI_MEM_NAND_FLASH_SR_DIN0_REG corresponds to SPI_MEM_NAND_FLASH_SR_ADDR0_REG."]
    #[inline(always)]
    pub fn spi_mem_nand_flash_sr_din0(&self) -> SPI_MEM_NAND_FLASH_SR_DIN0_R {
        SPI_MEM_NAND_FLASH_SR_DIN0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - spi read state register data to this register for SPI SEQ need. SPI_MEM_NAND_FLASH_SR_DIN0_REG corresponds to SPI_MEM_NAND_FLASH_SR_ADDR0_REG."]
    #[inline(always)]
    pub fn spi_mem_nand_flash_sr_din1(&self) -> SPI_MEM_NAND_FLASH_SR_DIN1_R {
        SPI_MEM_NAND_FLASH_SR_DIN1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - spi read state register data to this register for SPI SEQ need. SPI_MEM_NAND_FLASH_SR_DIN0_REG corresponds to SPI_MEM_NAND_FLASH_SR_ADDR0_REG."]
    #[inline(always)]
    pub fn spi_mem_nand_flash_sr_din2(&self) -> SPI_MEM_NAND_FLASH_SR_DIN2_R {
        SPI_MEM_NAND_FLASH_SR_DIN2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - spi read state register data to this register for SPI SEQ need. SPI_MEM_NAND_FLASH_SR_DIN0_REG corresponds to SPI_MEM_NAND_FLASH_SR_ADDR0_REG."]
    #[inline(always)]
    pub fn spi_mem_nand_flash_sr_din3(&self) -> SPI_MEM_NAND_FLASH_SR_DIN3_R {
        SPI_MEM_NAND_FLASH_SR_DIN3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_MEM_NAND_FLASH_SR_DIN0")
            .field(
                "spi_mem_nand_flash_sr_din0",
                &self.spi_mem_nand_flash_sr_din0(),
            )
            .field(
                "spi_mem_nand_flash_sr_din1",
                &self.spi_mem_nand_flash_sr_din1(),
            )
            .field(
                "spi_mem_nand_flash_sr_din2",
                &self.spi_mem_nand_flash_sr_din2(),
            )
            .field(
                "spi_mem_nand_flash_sr_din3",
                &self.spi_mem_nand_flash_sr_din3(),
            )
            .finish()
    }
}
#[doc = "NAND FLASH SPI SEQ control register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_mem_nand_flash_sr_din0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_MEM_NAND_FLASH_SR_DIN0_SPEC;
impl crate::RegisterSpec for SPI_MEM_NAND_FLASH_SR_DIN0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_mem_nand_flash_sr_din0::R`](R) reader structure"]
impl crate::Readable for SPI_MEM_NAND_FLASH_SR_DIN0_SPEC {}
#[doc = "`reset()` method sets SPI_MEM_NAND_FLASH_SR_DIN0 to value 0"]
impl crate::Resettable for SPI_MEM_NAND_FLASH_SR_DIN0_SPEC {}
