#[doc = "Register `LCD_CTRL` reader"]
pub type R = crate::R<LCD_CTRL_SPEC>;
#[doc = "Register `LCD_CTRL` writer"]
pub type W = crate::W<LCD_CTRL_SPEC>;
#[doc = "Field `LCD_HB_FRONT` reader - It is the horizontal blank front porch of a frame. Can be configured in CONF state."]
pub type LCD_HB_FRONT_R = crate::FieldReader<u16>;
#[doc = "Field `LCD_HB_FRONT` writer - It is the horizontal blank front porch of a frame. Can be configured in CONF state."]
pub type LCD_HB_FRONT_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `LCD_VA_HEIGHT` reader - It is the vertical active height of a frame. Can be configured in CONF state."]
pub type LCD_VA_HEIGHT_R = crate::FieldReader<u16>;
#[doc = "Field `LCD_VA_HEIGHT` writer - It is the vertical active height of a frame. Can be configured in CONF state."]
pub type LCD_VA_HEIGHT_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `LCD_VT_HEIGHT` reader - It is the vertical total height of a frame. Can be configured in CONF state."]
pub type LCD_VT_HEIGHT_R = crate::FieldReader<u16>;
#[doc = "Field `LCD_VT_HEIGHT` writer - It is the vertical total height of a frame. Can be configured in CONF state."]
pub type LCD_VT_HEIGHT_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `LCD_MODE_EN` reader - 1: Enable LCD mode output vsync, hsync, de. 0: Disable. Can be configured in CONF state."]
pub type LCD_MODE_EN_R = crate::BitReader;
#[doc = "Field `LCD_MODE_EN` writer - 1: Enable LCD mode output vsync, hsync, de. 0: Disable. Can be configured in CONF state."]
pub type LCD_MODE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:10 - It is the horizontal blank front porch of a frame. Can be configured in CONF state."]
    #[inline(always)]
    pub fn lcd_hb_front(&self) -> LCD_HB_FRONT_R {
        LCD_HB_FRONT_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:20 - It is the vertical active height of a frame. Can be configured in CONF state."]
    #[inline(always)]
    pub fn lcd_va_height(&self) -> LCD_VA_HEIGHT_R {
        LCD_VA_HEIGHT_R::new(((self.bits >> 11) & 0x03ff) as u16)
    }
    #[doc = "Bits 21:30 - It is the vertical total height of a frame. Can be configured in CONF state."]
    #[inline(always)]
    pub fn lcd_vt_height(&self) -> LCD_VT_HEIGHT_R {
        LCD_VT_HEIGHT_R::new(((self.bits >> 21) & 0x03ff) as u16)
    }
    #[doc = "Bit 31 - 1: Enable LCD mode output vsync, hsync, de. 0: Disable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn lcd_mode_en(&self) -> LCD_MODE_EN_R {
        LCD_MODE_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LCD_CTRL")
            .field("lcd_hb_front", &self.lcd_hb_front().bits())
            .field("lcd_va_height", &self.lcd_va_height().bits())
            .field("lcd_vt_height", &self.lcd_vt_height().bits())
            .field("lcd_mode_en", &self.lcd_mode_en().bit())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LCD_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:10 - It is the horizontal blank front porch of a frame. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn lcd_hb_front(&mut self) -> LCD_HB_FRONT_W<LCD_CTRL_SPEC> {
        LCD_HB_FRONT_W::new(self, 0)
    }
    #[doc = "Bits 11:20 - It is the vertical active height of a frame. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn lcd_va_height(&mut self) -> LCD_VA_HEIGHT_W<LCD_CTRL_SPEC> {
        LCD_VA_HEIGHT_W::new(self, 11)
    }
    #[doc = "Bits 21:30 - It is the vertical total height of a frame. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn lcd_vt_height(&mut self) -> LCD_VT_HEIGHT_W<LCD_CTRL_SPEC> {
        LCD_VT_HEIGHT_W::new(self, 21)
    }
    #[doc = "Bit 31 - 1: Enable LCD mode output vsync, hsync, de. 0: Disable. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn lcd_mode_en(&mut self) -> LCD_MODE_EN_W<LCD_CTRL_SPEC> {
        LCD_MODE_EN_W::new(self, 31)
    }
}
#[doc = "LCD frame control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LCD_CTRL_SPEC;
impl crate::RegisterSpec for LCD_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcd_ctrl::R`](R) reader structure"]
impl crate::Readable for LCD_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lcd_ctrl::W`](W) writer structure"]
impl crate::Writable for LCD_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LCD_CTRL to value 0"]
impl crate::Resettable for LCD_CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
