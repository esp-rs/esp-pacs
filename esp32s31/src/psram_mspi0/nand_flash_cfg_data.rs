#[doc = "Register `NAND_FLASH_CFG_DATA%s` reader"]
pub type R = crate::R<NAND_FLASH_CFG_DATA_SPEC>;
#[doc = "Register `NAND_FLASH_CFG_DATA%s` writer"]
pub type W = crate::W<NAND_FLASH_CFG_DATA_SPEC>;
#[doc = "Field `NAND_FLASH_CFG_DATA0` reader - "]
pub type NAND_FLASH_CFG_DATA0_R = crate::FieldReader<u16>;
#[doc = "Field `NAND_FLASH_CFG_DATA0` writer - "]
pub type NAND_FLASH_CFG_DATA0_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `NAND_FLASH_CFG_DATA1` reader - "]
pub type NAND_FLASH_CFG_DATA1_R = crate::FieldReader<u16>;
#[doc = "Field `NAND_FLASH_CFG_DATA1` writer - "]
pub type NAND_FLASH_CFG_DATA1_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn nand_flash_cfg_data0(&self) -> NAND_FLASH_CFG_DATA0_R {
        NAND_FLASH_CFG_DATA0_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn nand_flash_cfg_data1(&self) -> NAND_FLASH_CFG_DATA1_R {
        NAND_FLASH_CFG_DATA1_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NAND_FLASH_CFG_DATA")
            .field("nand_flash_cfg_data0", &self.nand_flash_cfg_data0())
            .field("nand_flash_cfg_data1", &self.nand_flash_cfg_data1())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn nand_flash_cfg_data0(&mut self) -> NAND_FLASH_CFG_DATA0_W<'_, NAND_FLASH_CFG_DATA_SPEC> {
        NAND_FLASH_CFG_DATA0_W::new(self, 0)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn nand_flash_cfg_data1(&mut self) -> NAND_FLASH_CFG_DATA1_W<'_, NAND_FLASH_CFG_DATA_SPEC> {
        NAND_FLASH_CFG_DATA1_W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`nand_flash_cfg_data::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nand_flash_cfg_data::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NAND_FLASH_CFG_DATA_SPEC;
impl crate::RegisterSpec for NAND_FLASH_CFG_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nand_flash_cfg_data::R`](R) reader structure"]
impl crate::Readable for NAND_FLASH_CFG_DATA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`nand_flash_cfg_data::W`](W) writer structure"]
impl crate::Writable for NAND_FLASH_CFG_DATA_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets NAND_FLASH_CFG_DATA%s to value 0"]
impl crate::Resettable for NAND_FLASH_CFG_DATA_SPEC {}
