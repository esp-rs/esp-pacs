#[doc = "Register `NAND_FLASH_CMD_LUT0` reader"]
pub type R = crate::R<NAND_FLASH_CMD_LUT0_SPEC>;
#[doc = "Field `LUT_CMD_VALUE` reader - "]
pub type LUT_CMD_VALUE_R = crate::FieldReader<u16>;
#[doc = "Field `LUT_SFSM_ST_EN` reader - "]
pub type LUT_SFSM_ST_EN_R = crate::FieldReader;
#[doc = "Field `LUT_CMD_LEN` reader - "]
pub type LUT_CMD_LEN_R = crate::FieldReader;
#[doc = "Field `LUT_ADDR_LEN` reader - "]
pub type LUT_ADDR_LEN_R = crate::FieldReader;
#[doc = "Field `LUT_DATA_LEN` reader - "]
pub type LUT_DATA_LEN_R = crate::FieldReader;
#[doc = "Field `LUT_BUS_EN` reader - "]
pub type LUT_BUS_EN_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn lut_cmd_value(&self) -> LUT_CMD_VALUE_R {
        LUT_CMD_VALUE_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn lut_sfsm_st_en(&self) -> LUT_SFSM_ST_EN_R {
        LUT_SFSM_ST_EN_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn lut_cmd_len(&self) -> LUT_CMD_LEN_R {
        LUT_CMD_LEN_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn lut_addr_len(&self) -> LUT_ADDR_LEN_R {
        LUT_ADDR_LEN_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn lut_data_len(&self) -> LUT_DATA_LEN_R {
        LUT_DATA_LEN_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn lut_bus_en(&self) -> LUT_BUS_EN_R {
        LUT_BUS_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NAND_FLASH_CMD_LUT0")
            .field("lut_cmd_value", &self.lut_cmd_value())
            .field("lut_sfsm_st_en", &self.lut_sfsm_st_en())
            .field("lut_cmd_len", &self.lut_cmd_len())
            .field("lut_addr_len", &self.lut_addr_len())
            .field("lut_data_len", &self.lut_data_len())
            .field("lut_bus_en", &self.lut_bus_en())
            .finish()
    }
}
#[doc = "MSPI NAND FLASH CMD LUT control register\n\nYou can [`read`](crate::Reg::read) this register and get [`nand_flash_cmd_lut0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NAND_FLASH_CMD_LUT0_SPEC;
impl crate::RegisterSpec for NAND_FLASH_CMD_LUT0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nand_flash_cmd_lut0::R`](R) reader structure"]
impl crate::Readable for NAND_FLASH_CMD_LUT0_SPEC {}
#[doc = "`reset()` method sets NAND_FLASH_CMD_LUT0 to value 0"]
impl crate::Resettable for NAND_FLASH_CMD_LUT0_SPEC {}
