#[doc = "Register `NAND_FLASH_SR_DIN0` reader"]
pub type R = crate::R<NAND_FLASH_SR_DIN0_SPEC>;
#[doc = "Field `NAND_FLASH_SR_DIN0` reader - spi read state register data to this register for SPI SEQ need. SPI_MEM_NAND_FLASH_SR_DIN0_REG corresponds to SPI_MEM_NAND_FLASH_SR_ADDR0_REG."]
pub type NAND_FLASH_SR_DIN0_R = crate::FieldReader;
#[doc = "Field `NAND_FLASH_SR_DIN1` reader - spi read state register data to this register for SPI SEQ need. SPI_MEM_NAND_FLASH_SR_DIN0_REG corresponds to SPI_MEM_NAND_FLASH_SR_ADDR0_REG."]
pub type NAND_FLASH_SR_DIN1_R = crate::FieldReader;
#[doc = "Field `NAND_FLASH_SR_DIN2` reader - spi read state register data to this register for SPI SEQ need. SPI_MEM_NAND_FLASH_SR_DIN0_REG corresponds to SPI_MEM_NAND_FLASH_SR_ADDR0_REG."]
pub type NAND_FLASH_SR_DIN2_R = crate::FieldReader;
#[doc = "Field `NAND_FLASH_SR_DIN3` reader - spi read state register data to this register for SPI SEQ need. SPI_MEM_NAND_FLASH_SR_DIN0_REG corresponds to SPI_MEM_NAND_FLASH_SR_ADDR0_REG."]
pub type NAND_FLASH_SR_DIN3_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - spi read state register data to this register for SPI SEQ need. SPI_MEM_NAND_FLASH_SR_DIN0_REG corresponds to SPI_MEM_NAND_FLASH_SR_ADDR0_REG."]
    #[inline(always)]
    pub fn nand_flash_sr_din0(&self) -> NAND_FLASH_SR_DIN0_R {
        NAND_FLASH_SR_DIN0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - spi read state register data to this register for SPI SEQ need. SPI_MEM_NAND_FLASH_SR_DIN0_REG corresponds to SPI_MEM_NAND_FLASH_SR_ADDR0_REG."]
    #[inline(always)]
    pub fn nand_flash_sr_din1(&self) -> NAND_FLASH_SR_DIN1_R {
        NAND_FLASH_SR_DIN1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - spi read state register data to this register for SPI SEQ need. SPI_MEM_NAND_FLASH_SR_DIN0_REG corresponds to SPI_MEM_NAND_FLASH_SR_ADDR0_REG."]
    #[inline(always)]
    pub fn nand_flash_sr_din2(&self) -> NAND_FLASH_SR_DIN2_R {
        NAND_FLASH_SR_DIN2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - spi read state register data to this register for SPI SEQ need. SPI_MEM_NAND_FLASH_SR_DIN0_REG corresponds to SPI_MEM_NAND_FLASH_SR_ADDR0_REG."]
    #[inline(always)]
    pub fn nand_flash_sr_din3(&self) -> NAND_FLASH_SR_DIN3_R {
        NAND_FLASH_SR_DIN3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NAND_FLASH_SR_DIN0")
            .field("nand_flash_sr_din0", &self.nand_flash_sr_din0())
            .field("nand_flash_sr_din1", &self.nand_flash_sr_din1())
            .field("nand_flash_sr_din2", &self.nand_flash_sr_din2())
            .field("nand_flash_sr_din3", &self.nand_flash_sr_din3())
            .finish()
    }
}
#[doc = "NAND FLASH SPI SEQ control register\n\nYou can [`read`](crate::Reg::read) this register and get [`nand_flash_sr_din0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NAND_FLASH_SR_DIN0_SPEC;
impl crate::RegisterSpec for NAND_FLASH_SR_DIN0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nand_flash_sr_din0::R`](R) reader structure"]
impl crate::Readable for NAND_FLASH_SR_DIN0_SPEC {}
#[doc = "`reset()` method sets NAND_FLASH_SR_DIN0 to value 0"]
impl crate::Resettable for NAND_FLASH_SR_DIN0_SPEC {}
