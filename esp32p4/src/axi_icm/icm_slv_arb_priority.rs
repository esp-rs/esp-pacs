#[doc = "Register `ICM_SLV_ARB_PRIORITY` reader"]
pub type R = crate::R<ICM_SLV_ARB_PRIORITY_SPEC>;
#[doc = "Register `ICM_SLV_ARB_PRIORITY` writer"]
pub type W = crate::W<ICM_SLV_ARB_PRIORITY_SPEC>;
#[doc = "Field `L2MEM_PRIORITY` reader - "]
pub type L2MEM_PRIORITY_R = crate::FieldReader;
#[doc = "Field `L2MEM_PRIORITY` writer - "]
pub type L2MEM_PRIORITY_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `FLASH_MSPI_PRIORITY` reader - "]
pub type FLASH_MSPI_PRIORITY_R = crate::FieldReader;
#[doc = "Field `FLASH_MSPI_PRIORITY` writer - "]
pub type FLASH_MSPI_PRIORITY_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PSRAM_MSPI_PRIORITY` reader - "]
pub type PSRAM_MSPI_PRIORITY_R = crate::FieldReader;
#[doc = "Field `PSRAM_MSPI_PRIORITY` writer - "]
pub type PSRAM_MSPI_PRIORITY_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `LCD_PRIORITY` reader - "]
pub type LCD_PRIORITY_R = crate::FieldReader;
#[doc = "Field `LCD_PRIORITY` writer - "]
pub type LCD_PRIORITY_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CAM_PRIORITY` reader - "]
pub type CAM_PRIORITY_R = crate::FieldReader;
#[doc = "Field `CAM_PRIORITY` writer - "]
pub type CAM_PRIORITY_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 3:5"]
    #[inline(always)]
    pub fn l2mem_priority(&self) -> L2MEM_PRIORITY_R {
        L2MEM_PRIORITY_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn flash_mspi_priority(&self) -> FLASH_MSPI_PRIORITY_R {
        FLASH_MSPI_PRIORITY_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17"]
    #[inline(always)]
    pub fn psram_mspi_priority(&self) -> PSRAM_MSPI_PRIORITY_R {
        PSRAM_MSPI_PRIORITY_R::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:20"]
    #[inline(always)]
    pub fn lcd_priority(&self) -> LCD_PRIORITY_R {
        LCD_PRIORITY_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:23"]
    #[inline(always)]
    pub fn cam_priority(&self) -> CAM_PRIORITY_R {
        CAM_PRIORITY_R::new(((self.bits >> 21) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICM_SLV_ARB_PRIORITY")
            .field("l2mem_priority", &self.l2mem_priority())
            .field("flash_mspi_priority", &self.flash_mspi_priority())
            .field("psram_mspi_priority", &self.psram_mspi_priority())
            .field("lcd_priority", &self.lcd_priority())
            .field("cam_priority", &self.cam_priority())
            .finish()
    }
}
impl W {
    #[doc = "Bits 3:5"]
    #[inline(always)]
    pub fn l2mem_priority(&mut self) -> L2MEM_PRIORITY_W<'_, ICM_SLV_ARB_PRIORITY_SPEC> {
        L2MEM_PRIORITY_W::new(self, 3)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn flash_mspi_priority(&mut self) -> FLASH_MSPI_PRIORITY_W<'_, ICM_SLV_ARB_PRIORITY_SPEC> {
        FLASH_MSPI_PRIORITY_W::new(self, 12)
    }
    #[doc = "Bits 15:17"]
    #[inline(always)]
    pub fn psram_mspi_priority(&mut self) -> PSRAM_MSPI_PRIORITY_W<'_, ICM_SLV_ARB_PRIORITY_SPEC> {
        PSRAM_MSPI_PRIORITY_W::new(self, 15)
    }
    #[doc = "Bits 18:20"]
    #[inline(always)]
    pub fn lcd_priority(&mut self) -> LCD_PRIORITY_W<'_, ICM_SLV_ARB_PRIORITY_SPEC> {
        LCD_PRIORITY_W::new(self, 18)
    }
    #[doc = "Bits 21:23"]
    #[inline(always)]
    pub fn cam_priority(&mut self) -> CAM_PRIORITY_W<'_, ICM_SLV_ARB_PRIORITY_SPEC> {
        CAM_PRIORITY_W::new(self, 21)
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
