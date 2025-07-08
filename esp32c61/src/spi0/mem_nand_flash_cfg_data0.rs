#[doc = "Register `MEM_NAND_FLASH_CFG_DATA0` reader"]
pub type R = crate::R<MEM_NAND_FLASH_CFG_DATA0_SPEC>;
#[doc = "Field `MEM_NAND_FLASH_CFG_DATA0` reader - configure data for SPI SEQ din/dout need. The data could be use to configure NAND FLASH or compare read data"]
pub type MEM_NAND_FLASH_CFG_DATA0_R = crate::FieldReader<u16>;
#[doc = "Field `MEM_NAND_FLASH_CFG_DATA1` reader - configure data for SPI SEQ din/dout need. The data could be use to configure NAND FLASH or compare read data"]
pub type MEM_NAND_FLASH_CFG_DATA1_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - configure data for SPI SEQ din/dout need. The data could be use to configure NAND FLASH or compare read data"]
    #[inline(always)]
    pub fn mem_nand_flash_cfg_data0(&self) -> MEM_NAND_FLASH_CFG_DATA0_R {
        MEM_NAND_FLASH_CFG_DATA0_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - configure data for SPI SEQ din/dout need. The data could be use to configure NAND FLASH or compare read data"]
    #[inline(always)]
    pub fn mem_nand_flash_cfg_data1(&self) -> MEM_NAND_FLASH_CFG_DATA1_R {
        MEM_NAND_FLASH_CFG_DATA1_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MEM_NAND_FLASH_CFG_DATA0")
            .field("mem_nand_flash_cfg_data0", &self.mem_nand_flash_cfg_data0())
            .field("mem_nand_flash_cfg_data1", &self.mem_nand_flash_cfg_data1())
            .finish()
    }
}
#[doc = "NAND FLASH SPI SEQ control register\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_nand_flash_cfg_data0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MEM_NAND_FLASH_CFG_DATA0_SPEC;
impl crate::RegisterSpec for MEM_NAND_FLASH_CFG_DATA0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_nand_flash_cfg_data0::R`](R) reader structure"]
impl crate::Readable for MEM_NAND_FLASH_CFG_DATA0_SPEC {}
#[doc = "`reset()` method sets MEM_NAND_FLASH_CFG_DATA0 to value 0"]
impl crate::Resettable for MEM_NAND_FLASH_CFG_DATA0_SPEC {}
