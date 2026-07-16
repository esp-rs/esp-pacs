#[doc = "Register `NAND_FLASH_CMD_LUT0` reader"]
pub type R = crate::R<NAND_FLASH_CMD_LUT0_SPEC>;
#[doc = "Register `NAND_FLASH_CMD_LUT0` writer"]
pub type W = crate::W<NAND_FLASH_CMD_LUT0_SPEC>;
#[doc = "Field `NAND_FLASH_LUT_CMD_VALUE0` reader - "]
pub type NAND_FLASH_LUT_CMD_VALUE0_R = crate::FieldReader<u16>;
#[doc = "Field `NAND_FLASH_LUT_CMD_VALUE0` writer - "]
pub type NAND_FLASH_LUT_CMD_VALUE0_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `NAND_FLASH_LUT_SFSM_ST_EN0` reader - "]
pub type NAND_FLASH_LUT_SFSM_ST_EN0_R = crate::FieldReader;
#[doc = "Field `NAND_FLASH_LUT_SFSM_ST_EN0` writer - "]
pub type NAND_FLASH_LUT_SFSM_ST_EN0_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `NAND_FLASH_LUT_CMD_LEN0` reader - "]
pub type NAND_FLASH_LUT_CMD_LEN0_R = crate::FieldReader;
#[doc = "Field `NAND_FLASH_LUT_CMD_LEN0` writer - "]
pub type NAND_FLASH_LUT_CMD_LEN0_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `NAND_FLASH_LUT_ADDR_LEN0` reader - "]
pub type NAND_FLASH_LUT_ADDR_LEN0_R = crate::FieldReader;
#[doc = "Field `NAND_FLASH_LUT_ADDR_LEN0` writer - "]
pub type NAND_FLASH_LUT_ADDR_LEN0_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `NAND_FLASH_LUT_DATA_LEN0` reader - "]
pub type NAND_FLASH_LUT_DATA_LEN0_R = crate::FieldReader;
#[doc = "Field `NAND_FLASH_LUT_DATA_LEN0` writer - "]
pub type NAND_FLASH_LUT_DATA_LEN0_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `NAND_FLASH_LUT_BUS_EN0` reader - "]
pub type NAND_FLASH_LUT_BUS_EN0_R = crate::BitReader;
#[doc = "Field `NAND_FLASH_LUT_BUS_EN0` writer - "]
pub type NAND_FLASH_LUT_BUS_EN0_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn nand_flash_lut_cmd_value0(&self) -> NAND_FLASH_LUT_CMD_VALUE0_R {
        NAND_FLASH_LUT_CMD_VALUE0_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn nand_flash_lut_sfsm_st_en0(&self) -> NAND_FLASH_LUT_SFSM_ST_EN0_R {
        NAND_FLASH_LUT_SFSM_ST_EN0_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn nand_flash_lut_cmd_len0(&self) -> NAND_FLASH_LUT_CMD_LEN0_R {
        NAND_FLASH_LUT_CMD_LEN0_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn nand_flash_lut_addr_len0(&self) -> NAND_FLASH_LUT_ADDR_LEN0_R {
        NAND_FLASH_LUT_ADDR_LEN0_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn nand_flash_lut_data_len0(&self) -> NAND_FLASH_LUT_DATA_LEN0_R {
        NAND_FLASH_LUT_DATA_LEN0_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn nand_flash_lut_bus_en0(&self) -> NAND_FLASH_LUT_BUS_EN0_R {
        NAND_FLASH_LUT_BUS_EN0_R::new(((self.bits >> 30) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NAND_FLASH_CMD_LUT0")
            .field(
                "nand_flash_lut_cmd_value0",
                &self.nand_flash_lut_cmd_value0(),
            )
            .field(
                "nand_flash_lut_sfsm_st_en0",
                &self.nand_flash_lut_sfsm_st_en0(),
            )
            .field("nand_flash_lut_cmd_len0", &self.nand_flash_lut_cmd_len0())
            .field("nand_flash_lut_addr_len0", &self.nand_flash_lut_addr_len0())
            .field("nand_flash_lut_data_len0", &self.nand_flash_lut_data_len0())
            .field("nand_flash_lut_bus_en0", &self.nand_flash_lut_bus_en0())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn nand_flash_lut_cmd_value0(
        &mut self,
    ) -> NAND_FLASH_LUT_CMD_VALUE0_W<'_, NAND_FLASH_CMD_LUT0_SPEC> {
        NAND_FLASH_LUT_CMD_VALUE0_W::new(self, 0)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn nand_flash_lut_sfsm_st_en0(
        &mut self,
    ) -> NAND_FLASH_LUT_SFSM_ST_EN0_W<'_, NAND_FLASH_CMD_LUT0_SPEC> {
        NAND_FLASH_LUT_SFSM_ST_EN0_W::new(self, 16)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn nand_flash_lut_cmd_len0(
        &mut self,
    ) -> NAND_FLASH_LUT_CMD_LEN0_W<'_, NAND_FLASH_CMD_LUT0_SPEC> {
        NAND_FLASH_LUT_CMD_LEN0_W::new(self, 20)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn nand_flash_lut_addr_len0(
        &mut self,
    ) -> NAND_FLASH_LUT_ADDR_LEN0_W<'_, NAND_FLASH_CMD_LUT0_SPEC> {
        NAND_FLASH_LUT_ADDR_LEN0_W::new(self, 24)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn nand_flash_lut_data_len0(
        &mut self,
    ) -> NAND_FLASH_LUT_DATA_LEN0_W<'_, NAND_FLASH_CMD_LUT0_SPEC> {
        NAND_FLASH_LUT_DATA_LEN0_W::new(self, 28)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn nand_flash_lut_bus_en0(
        &mut self,
    ) -> NAND_FLASH_LUT_BUS_EN0_W<'_, NAND_FLASH_CMD_LUT0_SPEC> {
        NAND_FLASH_LUT_BUS_EN0_W::new(self, 30)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`nand_flash_cmd_lut0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nand_flash_cmd_lut0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NAND_FLASH_CMD_LUT0_SPEC;
impl crate::RegisterSpec for NAND_FLASH_CMD_LUT0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nand_flash_cmd_lut0::R`](R) reader structure"]
impl crate::Readable for NAND_FLASH_CMD_LUT0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`nand_flash_cmd_lut0::W`](W) writer structure"]
impl crate::Writable for NAND_FLASH_CMD_LUT0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets NAND_FLASH_CMD_LUT0 to value 0"]
impl crate::Resettable for NAND_FLASH_CMD_LUT0_SPEC {}
