#[doc = "Register `LCD_RGB_HORIZONTAL` reader"]
pub type R = crate::R<LCD_RGB_HORIZONTAL_SPEC>;
#[doc = "Register `LCD_RGB_HORIZONTAL` writer"]
pub type W = crate::W<LCD_RGB_HORIZONTAL_SPEC>;
#[doc = "Field `LCD_HA_WIDTH` reader - It is the horizontal active width of a frame."]
pub type LCD_HA_WIDTH_R = crate::FieldReader<u16>;
#[doc = "Field `LCD_HA_WIDTH` writer - It is the horizontal active width of a frame."]
pub type LCD_HA_WIDTH_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `LCD_HT_WIDTH` reader - It is the horizontal total width of a frame."]
pub type LCD_HT_WIDTH_R = crate::FieldReader<u16>;
#[doc = "Field `LCD_HT_WIDTH` writer - It is the horizontal total width of a frame."]
pub type LCD_HT_WIDTH_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - It is the horizontal active width of a frame."]
    #[inline(always)]
    pub fn lcd_ha_width(&self) -> LCD_HA_WIDTH_R {
        LCD_HA_WIDTH_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - It is the horizontal total width of a frame."]
    #[inline(always)]
    pub fn lcd_ht_width(&self) -> LCD_HT_WIDTH_R {
        LCD_HT_WIDTH_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LCD_RGB_HORIZONTAL")
            .field("lcd_ha_width", &self.lcd_ha_width())
            .field("lcd_ht_width", &self.lcd_ht_width())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - It is the horizontal active width of a frame."]
    #[inline(always)]
    pub fn lcd_ha_width(&mut self) -> LCD_HA_WIDTH_W<'_, LCD_RGB_HORIZONTAL_SPEC> {
        LCD_HA_WIDTH_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - It is the horizontal total width of a frame."]
    #[inline(always)]
    pub fn lcd_ht_width(&mut self) -> LCD_HT_WIDTH_W<'_, LCD_RGB_HORIZONTAL_SPEC> {
        LCD_HT_WIDTH_W::new(self, 16)
    }
}
#[doc = "LCD config register.\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_rgb_horizontal::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_rgb_horizontal::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LCD_RGB_HORIZONTAL_SPEC;
impl crate::RegisterSpec for LCD_RGB_HORIZONTAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcd_rgb_horizontal::R`](R) reader structure"]
impl crate::Readable for LCD_RGB_HORIZONTAL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lcd_rgb_horizontal::W`](W) writer structure"]
impl crate::Writable for LCD_RGB_HORIZONTAL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCD_RGB_HORIZONTAL to value 0"]
impl crate::Resettable for LCD_RGB_HORIZONTAL_SPEC {}
