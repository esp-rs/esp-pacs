#[doc = "Register `SPI_MEM_NAND_FLASH_SR_ADDR0` reader"]
pub type R = crate::R<SPI_MEM_NAND_FLASH_SR_ADDR0_SPEC>;
#[doc = "Field `SPI_MEM_NAND_FLASH_SR_ADDR0` reader - configure state register address for SPI SEQ need. If OIP is in address C0H , user could configure C0H into this register"]
pub type SPI_MEM_NAND_FLASH_SR_ADDR0_R = crate::FieldReader;
#[doc = "Field `SPI_MEM_NAND_FLASH_SR_ADDR1` reader - configure state register address for SPI SEQ need. If OIP is in address C0H , user could configure C0H into this register"]
pub type SPI_MEM_NAND_FLASH_SR_ADDR1_R = crate::FieldReader;
#[doc = "Field `SPI_MEM_NAND_FLASH_SR_ADDR2` reader - configure state register address for SPI SEQ need. If OIP is in address C0H , user could configure C0H into this register"]
pub type SPI_MEM_NAND_FLASH_SR_ADDR2_R = crate::FieldReader;
#[doc = "Field `SPI_MEM_NAND_FLASH_SR_ADDR3` reader - configure state register address for SPI SEQ need. If OIP is in address C0H , user could configure C0H into this register"]
pub type SPI_MEM_NAND_FLASH_SR_ADDR3_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - configure state register address for SPI SEQ need. If OIP is in address C0H , user could configure C0H into this register"]
    #[inline(always)]
    pub fn spi_mem_nand_flash_sr_addr0(&self) -> SPI_MEM_NAND_FLASH_SR_ADDR0_R {
        SPI_MEM_NAND_FLASH_SR_ADDR0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - configure state register address for SPI SEQ need. If OIP is in address C0H , user could configure C0H into this register"]
    #[inline(always)]
    pub fn spi_mem_nand_flash_sr_addr1(&self) -> SPI_MEM_NAND_FLASH_SR_ADDR1_R {
        SPI_MEM_NAND_FLASH_SR_ADDR1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - configure state register address for SPI SEQ need. If OIP is in address C0H , user could configure C0H into this register"]
    #[inline(always)]
    pub fn spi_mem_nand_flash_sr_addr2(&self) -> SPI_MEM_NAND_FLASH_SR_ADDR2_R {
        SPI_MEM_NAND_FLASH_SR_ADDR2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - configure state register address for SPI SEQ need. If OIP is in address C0H , user could configure C0H into this register"]
    #[inline(always)]
    pub fn spi_mem_nand_flash_sr_addr3(&self) -> SPI_MEM_NAND_FLASH_SR_ADDR3_R {
        SPI_MEM_NAND_FLASH_SR_ADDR3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_MEM_NAND_FLASH_SR_ADDR0")
            .field(
                "spi_mem_nand_flash_sr_addr0",
                &self.spi_mem_nand_flash_sr_addr0(),
            )
            .field(
                "spi_mem_nand_flash_sr_addr1",
                &self.spi_mem_nand_flash_sr_addr1(),
            )
            .field(
                "spi_mem_nand_flash_sr_addr2",
                &self.spi_mem_nand_flash_sr_addr2(),
            )
            .field(
                "spi_mem_nand_flash_sr_addr3",
                &self.spi_mem_nand_flash_sr_addr3(),
            )
            .finish()
    }
}
#[doc = "NAND FLASH SPI SEQ control register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_mem_nand_flash_sr_addr0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_MEM_NAND_FLASH_SR_ADDR0_SPEC;
impl crate::RegisterSpec for SPI_MEM_NAND_FLASH_SR_ADDR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_mem_nand_flash_sr_addr0::R`](R) reader structure"]
impl crate::Readable for SPI_MEM_NAND_FLASH_SR_ADDR0_SPEC {}
#[doc = "`reset()` method sets SPI_MEM_NAND_FLASH_SR_ADDR0 to value 0"]
impl crate::Resettable for SPI_MEM_NAND_FLASH_SR_ADDR0_SPEC {}
