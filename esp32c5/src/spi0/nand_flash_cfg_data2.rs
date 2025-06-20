#[doc = "Register `NAND_FLASH_CFG_DATA2` reader"]
pub type R = crate::R<NAND_FLASH_CFG_DATA2_SPEC>;
#[doc = "Field `NAND_FLASH_CFG_DATA4` reader - configure data for SPI SEQ din/dout need. The data could be use to configure NAND FLASH or compare read data"]
pub type NAND_FLASH_CFG_DATA4_R = crate::FieldReader<u16>;
#[doc = "Field `NAND_FLASH_CFG_DATA5` reader - configure data for SPI SEQ din/dout need. The data could be use to configure NAND FLASH or compare read data"]
pub type NAND_FLASH_CFG_DATA5_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - configure data for SPI SEQ din/dout need. The data could be use to configure NAND FLASH or compare read data"]
    #[inline(always)]
    pub fn nand_flash_cfg_data4(&self) -> NAND_FLASH_CFG_DATA4_R {
        NAND_FLASH_CFG_DATA4_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - configure data for SPI SEQ din/dout need. The data could be use to configure NAND FLASH or compare read data"]
    #[inline(always)]
    pub fn nand_flash_cfg_data5(&self) -> NAND_FLASH_CFG_DATA5_R {
        NAND_FLASH_CFG_DATA5_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NAND_FLASH_CFG_DATA2")
            .field("nand_flash_cfg_data4", &self.nand_flash_cfg_data4())
            .field("nand_flash_cfg_data5", &self.nand_flash_cfg_data5())
            .finish()
    }
}
#[doc = "NAND FLASH SPI SEQ control register\n\nYou can [`read`](crate::Reg::read) this register and get [`nand_flash_cfg_data2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NAND_FLASH_CFG_DATA2_SPEC;
impl crate::RegisterSpec for NAND_FLASH_CFG_DATA2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nand_flash_cfg_data2::R`](R) reader structure"]
impl crate::Readable for NAND_FLASH_CFG_DATA2_SPEC {}
#[doc = "`reset()` method sets NAND_FLASH_CFG_DATA2 to value 0"]
impl crate::Resettable for NAND_FLASH_CFG_DATA2_SPEC {
    const RESET_VALUE: u32 = 0;
}
