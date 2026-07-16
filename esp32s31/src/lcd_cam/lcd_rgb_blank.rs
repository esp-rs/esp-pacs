#[doc = "Register `LCD_RGB_BLANK` reader"]
pub type R = crate::R<LCD_RGB_BLANK_SPEC>;
#[doc = "Register `LCD_RGB_BLANK` writer"]
pub type W = crate::W<LCD_RGB_BLANK_SPEC>;
#[doc = "Field `LCD_HB_FRONT` reader - It is the horizontal blank front porch of a frame."]
pub type LCD_HB_FRONT_R = crate::FieldReader<u16>;
#[doc = "Field `LCD_HB_FRONT` writer - It is the horizontal blank front porch of a frame."]
pub type LCD_HB_FRONT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `LCD_VB_FRONT` reader - It is the vertical blank front porch of a frame."]
pub type LCD_VB_FRONT_R = crate::FieldReader<u16>;
#[doc = "Field `LCD_VB_FRONT` writer - It is the vertical blank front porch of a frame."]
pub type LCD_VB_FRONT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - It is the horizontal blank front porch of a frame."]
    #[inline(always)]
    pub fn lcd_hb_front(&self) -> LCD_HB_FRONT_R {
        LCD_HB_FRONT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - It is the vertical blank front porch of a frame."]
    #[inline(always)]
    pub fn lcd_vb_front(&self) -> LCD_VB_FRONT_R {
        LCD_VB_FRONT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LCD_RGB_BLANK")
            .field("lcd_hb_front", &self.lcd_hb_front())
            .field("lcd_vb_front", &self.lcd_vb_front())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - It is the horizontal blank front porch of a frame."]
    #[inline(always)]
    pub fn lcd_hb_front(&mut self) -> LCD_HB_FRONT_W<'_, LCD_RGB_BLANK_SPEC> {
        LCD_HB_FRONT_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - It is the vertical blank front porch of a frame."]
    #[inline(always)]
    pub fn lcd_vb_front(&mut self) -> LCD_VB_FRONT_W<'_, LCD_RGB_BLANK_SPEC> {
        LCD_VB_FRONT_W::new(self, 16)
    }
}
#[doc = "LCD config register.\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_rgb_blank::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_rgb_blank::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LCD_RGB_BLANK_SPEC;
impl crate::RegisterSpec for LCD_RGB_BLANK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcd_rgb_blank::R`](R) reader structure"]
impl crate::Readable for LCD_RGB_BLANK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lcd_rgb_blank::W`](W) writer structure"]
impl crate::Writable for LCD_RGB_BLANK_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCD_RGB_BLANK to value 0"]
impl crate::Resettable for LCD_RGB_BLANK_SPEC {}
