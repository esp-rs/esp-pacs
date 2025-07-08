#[doc = "Register `MEM_NAND_FLASH_CFG_DATA1` reader"]
pub type R = crate::R<MEM_NAND_FLASH_CFG_DATA1_SPEC>;
#[doc = "Field `MEM_NAND_FLASH_CFG_DATA2` reader - configure data for SPI SEQ din/dout need. The data could be use to configure NAND FLASH or compare read data"]
pub type MEM_NAND_FLASH_CFG_DATA2_R = crate::FieldReader<u16>;
#[doc = "Field `MEM_NAND_FLASH_CFG_DATA3` reader - configure data for SPI SEQ din/dout need. The data could be use to configure NAND FLASH or compare read data"]
pub type MEM_NAND_FLASH_CFG_DATA3_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - configure data for SPI SEQ din/dout need. The data could be use to configure NAND FLASH or compare read data"]
    #[inline(always)]
    pub fn mem_nand_flash_cfg_data2(&self) -> MEM_NAND_FLASH_CFG_DATA2_R {
        MEM_NAND_FLASH_CFG_DATA2_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - configure data for SPI SEQ din/dout need. The data could be use to configure NAND FLASH or compare read data"]
    #[inline(always)]
    pub fn mem_nand_flash_cfg_data3(&self) -> MEM_NAND_FLASH_CFG_DATA3_R {
        MEM_NAND_FLASH_CFG_DATA3_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MEM_NAND_FLASH_CFG_DATA1")
            .field("mem_nand_flash_cfg_data2", &self.mem_nand_flash_cfg_data2())
            .field("mem_nand_flash_cfg_data3", &self.mem_nand_flash_cfg_data3())
            .finish()
    }
}
#[doc = "NAND FLASH SPI SEQ control register\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_nand_flash_cfg_data1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MEM_NAND_FLASH_CFG_DATA1_SPEC;
impl crate::RegisterSpec for MEM_NAND_FLASH_CFG_DATA1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_nand_flash_cfg_data1::R`](R) reader structure"]
impl crate::Readable for MEM_NAND_FLASH_CFG_DATA1_SPEC {}
#[doc = "`reset()` method sets MEM_NAND_FLASH_CFG_DATA1 to value 0"]
impl crate::Resettable for MEM_NAND_FLASH_CFG_DATA1_SPEC {}
