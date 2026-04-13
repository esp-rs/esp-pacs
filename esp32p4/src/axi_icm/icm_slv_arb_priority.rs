#[doc = "Register `ICM_SLV_ARB_PRIORITY` reader"]
pub type R = crate::R<ICM_SLV_ARB_PRIORITY_SPEC>;
#[doc = "Register `ICM_SLV_ARB_PRIORITY` writer"]
pub type W = crate::W<ICM_SLV_ARB_PRIORITY_SPEC>;
#[doc = "Field `ICM_REG_L2MEM_PRIORITY` reader - "]
pub type ICM_REG_L2MEM_PRIORITY_R = crate::FieldReader;
#[doc = "Field `ICM_REG_L2MEM_PRIORITY` writer - "]
pub type ICM_REG_L2MEM_PRIORITY_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ICM_REG_FLASH_MSPI_PRIORITY` reader - "]
pub type ICM_REG_FLASH_MSPI_PRIORITY_R = crate::FieldReader;
#[doc = "Field `ICM_REG_FLASH_MSPI_PRIORITY` writer - "]
pub type ICM_REG_FLASH_MSPI_PRIORITY_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ICM_REG_PSRAM_MSPI_PRIORITY` reader - "]
pub type ICM_REG_PSRAM_MSPI_PRIORITY_R = crate::FieldReader;
#[doc = "Field `ICM_REG_PSRAM_MSPI_PRIORITY` writer - "]
pub type ICM_REG_PSRAM_MSPI_PRIORITY_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ICM_REG_LCD_PRIORITY` reader - "]
pub type ICM_REG_LCD_PRIORITY_R = crate::FieldReader;
#[doc = "Field `ICM_REG_LCD_PRIORITY` writer - "]
pub type ICM_REG_LCD_PRIORITY_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ICM_REG_CAM_PRIORITY` reader - "]
pub type ICM_REG_CAM_PRIORITY_R = crate::FieldReader;
#[doc = "Field `ICM_REG_CAM_PRIORITY` writer - "]
pub type ICM_REG_CAM_PRIORITY_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 3:5"]
    #[inline(always)]
    pub fn icm_reg_l2mem_priority(&self) -> ICM_REG_L2MEM_PRIORITY_R {
        ICM_REG_L2MEM_PRIORITY_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn icm_reg_flash_mspi_priority(&self) -> ICM_REG_FLASH_MSPI_PRIORITY_R {
        ICM_REG_FLASH_MSPI_PRIORITY_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17"]
    #[inline(always)]
    pub fn icm_reg_psram_mspi_priority(&self) -> ICM_REG_PSRAM_MSPI_PRIORITY_R {
        ICM_REG_PSRAM_MSPI_PRIORITY_R::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:20"]
    #[inline(always)]
    pub fn icm_reg_lcd_priority(&self) -> ICM_REG_LCD_PRIORITY_R {
        ICM_REG_LCD_PRIORITY_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:23"]
    #[inline(always)]
    pub fn icm_reg_cam_priority(&self) -> ICM_REG_CAM_PRIORITY_R {
        ICM_REG_CAM_PRIORITY_R::new(((self.bits >> 21) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICM_SLV_ARB_PRIORITY")
            .field("icm_reg_l2mem_priority", &self.icm_reg_l2mem_priority())
            .field(
                "icm_reg_flash_mspi_priority",
                &self.icm_reg_flash_mspi_priority(),
            )
            .field(
                "icm_reg_psram_mspi_priority",
                &self.icm_reg_psram_mspi_priority(),
            )
            .field("icm_reg_lcd_priority", &self.icm_reg_lcd_priority())
            .field("icm_reg_cam_priority", &self.icm_reg_cam_priority())
            .finish()
    }
}
impl W {
    #[doc = "Bits 3:5"]
    #[inline(always)]
    pub fn icm_reg_l2mem_priority(
        &mut self,
    ) -> ICM_REG_L2MEM_PRIORITY_W<'_, ICM_SLV_ARB_PRIORITY_SPEC> {
        ICM_REG_L2MEM_PRIORITY_W::new(self, 3)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn icm_reg_flash_mspi_priority(
        &mut self,
    ) -> ICM_REG_FLASH_MSPI_PRIORITY_W<'_, ICM_SLV_ARB_PRIORITY_SPEC> {
        ICM_REG_FLASH_MSPI_PRIORITY_W::new(self, 12)
    }
    #[doc = "Bits 15:17"]
    #[inline(always)]
    pub fn icm_reg_psram_mspi_priority(
        &mut self,
    ) -> ICM_REG_PSRAM_MSPI_PRIORITY_W<'_, ICM_SLV_ARB_PRIORITY_SPEC> {
        ICM_REG_PSRAM_MSPI_PRIORITY_W::new(self, 15)
    }
    #[doc = "Bits 18:20"]
    #[inline(always)]
    pub fn icm_reg_lcd_priority(
        &mut self,
    ) -> ICM_REG_LCD_PRIORITY_W<'_, ICM_SLV_ARB_PRIORITY_SPEC> {
        ICM_REG_LCD_PRIORITY_W::new(self, 18)
    }
    #[doc = "Bits 21:23"]
    #[inline(always)]
    pub fn icm_reg_cam_priority(
        &mut self,
    ) -> ICM_REG_CAM_PRIORITY_W<'_, ICM_SLV_ARB_PRIORITY_SPEC> {
        ICM_REG_CAM_PRIORITY_W::new(self, 21)
    }
}
#[doc = "Slave arbitration priority\n\nYou can [`read`](crate::Reg::read) this register and get [`icm_slv_arb_priority::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icm_slv_arb_priority::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICM_SLV_ARB_PRIORITY_SPEC;
impl crate::RegisterSpec for ICM_SLV_ARB_PRIORITY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icm_slv_arb_priority::R`](R) reader structure"]
impl crate::Readable for ICM_SLV_ARB_PRIORITY_SPEC {}
#[doc = "`write(|w| ..)` method takes [`icm_slv_arb_priority::W`](W) writer structure"]
impl crate::Writable for ICM_SLV_ARB_PRIORITY_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ICM_SLV_ARB_PRIORITY to value 0"]
impl crate::Resettable for ICM_SLV_ARB_PRIORITY_SPEC {}
