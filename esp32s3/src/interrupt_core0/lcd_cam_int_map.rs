#[doc = "Register `LCD_CAM_INT_MAP` reader"]
pub type R = crate::R<LCD_CAM_INT_MAP_SPEC>;
#[doc = "Register `LCD_CAM_INT_MAP` writer"]
pub type W = crate::W<LCD_CAM_INT_MAP_SPEC>;
#[doc = "Field `LCD_CAM_INT_MAP` reader - this register used to map lcd_cam interrupt to one of core0's external interrupt"]
pub type LCD_CAM_INT_MAP_R = crate::FieldReader;
#[doc = "Field `LCD_CAM_INT_MAP` writer - this register used to map lcd_cam interrupt to one of core0's external interrupt"]
pub type LCD_CAM_INT_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - this register used to map lcd_cam interrupt to one of core0's external interrupt"]
    #[inline(always)]
    pub fn lcd_cam_int_map(&self) -> LCD_CAM_INT_MAP_R {
        LCD_CAM_INT_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LCD_CAM_INT_MAP")
            .field("lcd_cam_int_map", &self.lcd_cam_int_map())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - this register used to map lcd_cam interrupt to one of core0's external interrupt"]
    #[inline(always)]
    pub fn lcd_cam_int_map(&mut self) -> LCD_CAM_INT_MAP_W<LCD_CAM_INT_MAP_SPEC> {
        LCD_CAM_INT_MAP_W::new(self, 0)
    }
}
#[doc = "lcd_cam interrupt configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_cam_int_map::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_cam_int_map::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LCD_CAM_INT_MAP_SPEC;
impl crate::RegisterSpec for LCD_CAM_INT_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcd_cam_int_map::R`](R) reader structure"]
impl crate::Readable for LCD_CAM_INT_MAP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lcd_cam_int_map::W`](W) writer structure"]
impl crate::Writable for LCD_CAM_INT_MAP_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCD_CAM_INT_MAP to value 0x10"]
impl crate::Resettable for LCD_CAM_INT_MAP_SPEC {
    const RESET_VALUE: u32 = 0x10;
}
