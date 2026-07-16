#[doc = "Register `LCD_RGB_VERTICAL` reader"]
pub type R = crate::R<LCD_RGB_VERTICAL_SPEC>;
#[doc = "Register `LCD_RGB_VERTICAL` writer"]
pub type W = crate::W<LCD_RGB_VERTICAL_SPEC>;
#[doc = "Field `LCD_VA_HEIGHT` reader - It is the vertical active height of a frame."]
pub type LCD_VA_HEIGHT_R = crate::FieldReader<u16>;
#[doc = "Field `LCD_VA_HEIGHT` writer - It is the vertical active height of a frame."]
pub type LCD_VA_HEIGHT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `LCD_VT_HEIGHT` reader - It is the vertical total height of a frame."]
pub type LCD_VT_HEIGHT_R = crate::FieldReader<u16>;
#[doc = "Field `LCD_VT_HEIGHT` writer - It is the vertical total height of a frame."]
pub type LCD_VT_HEIGHT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - It is the vertical active height of a frame."]
    #[inline(always)]
    pub fn lcd_va_height(&self) -> LCD_VA_HEIGHT_R {
        LCD_VA_HEIGHT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - It is the vertical total height of a frame."]
    #[inline(always)]
    pub fn lcd_vt_height(&self) -> LCD_VT_HEIGHT_R {
        LCD_VT_HEIGHT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LCD_RGB_VERTICAL")
            .field("lcd_va_height", &self.lcd_va_height())
            .field("lcd_vt_height", &self.lcd_vt_height())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - It is the vertical active height of a frame."]
    #[inline(always)]
    pub fn lcd_va_height(&mut self) -> LCD_VA_HEIGHT_W<'_, LCD_RGB_VERTICAL_SPEC> {
        LCD_VA_HEIGHT_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - It is the vertical total height of a frame."]
    #[inline(always)]
    pub fn lcd_vt_height(&mut self) -> LCD_VT_HEIGHT_W<'_, LCD_RGB_VERTICAL_SPEC> {
        LCD_VT_HEIGHT_W::new(self, 16)
    }
}
#[doc = "LCD config register.\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_rgb_vertical::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_rgb_vertical::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LCD_RGB_VERTICAL_SPEC;
impl crate::RegisterSpec for LCD_RGB_VERTICAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcd_rgb_vertical::R`](R) reader structure"]
impl crate::Readable for LCD_RGB_VERTICAL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lcd_rgb_vertical::W`](W) writer structure"]
impl crate::Writable for LCD_RGB_VERTICAL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCD_RGB_VERTICAL to value 0"]
impl crate::Resettable for LCD_RGB_VERTICAL_SPEC {}
