#[doc = "Register `LCD_CTRL` reader"]
pub type R = crate::R<LCD_CTRL_SPEC>;
#[doc = "Register `LCD_CTRL` writer"]
pub type W = crate::W<LCD_CTRL_SPEC>;
#[doc = "Field `LCD_HB_FRONT` reader - It is the horizontal blank front porch of a frame."]
pub type LCD_HB_FRONT_R = crate::FieldReader<u16>;
#[doc = "Field `LCD_HB_FRONT` writer - It is the horizontal blank front porch of a frame."]
pub type LCD_HB_FRONT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 11, O, u16>;
#[doc = "Field `LCD_VA_HEIGHT` reader - It is the vertical active height of a frame."]
pub type LCD_VA_HEIGHT_R = crate::FieldReader<u16>;
#[doc = "Field `LCD_VA_HEIGHT` writer - It is the vertical active height of a frame."]
pub type LCD_VA_HEIGHT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 10, O, u16>;
#[doc = "Field `LCD_VT_HEIGHT` reader - It is the vertical total height of a frame."]
pub type LCD_VT_HEIGHT_R = crate::FieldReader<u16>;
#[doc = "Field `LCD_VT_HEIGHT` writer - It is the vertical total height of a frame."]
pub type LCD_VT_HEIGHT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 10, O, u16>;
#[doc = "Field `LCD_RGB_MODE_EN` reader - 1: Enable reg mode input vsync, hsync, de. 0: Disable."]
pub type LCD_RGB_MODE_EN_R = crate::BitReader;
#[doc = "Field `LCD_RGB_MODE_EN` writer - 1: Enable reg mode input vsync, hsync, de. 0: Disable."]
pub type LCD_RGB_MODE_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:10 - It is the horizontal blank front porch of a frame."]
    #[inline(always)]
    pub fn lcd_hb_front(&self) -> LCD_HB_FRONT_R {
        LCD_HB_FRONT_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:20 - It is the vertical active height of a frame."]
    #[inline(always)]
    pub fn lcd_va_height(&self) -> LCD_VA_HEIGHT_R {
        LCD_VA_HEIGHT_R::new(((self.bits >> 11) & 0x03ff) as u16)
    }
    #[doc = "Bits 21:30 - It is the vertical total height of a frame."]
    #[inline(always)]
    pub fn lcd_vt_height(&self) -> LCD_VT_HEIGHT_R {
        LCD_VT_HEIGHT_R::new(((self.bits >> 21) & 0x03ff) as u16)
    }
    #[doc = "Bit 31 - 1: Enable reg mode input vsync, hsync, de. 0: Disable."]
    #[inline(always)]
    pub fn lcd_rgb_mode_en(&self) -> LCD_RGB_MODE_EN_R {
        LCD_RGB_MODE_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LCD_CTRL")
            .field(
                "lcd_hb_front",
                &format_args!("{}", self.lcd_hb_front().bits()),
            )
            .field(
                "lcd_va_height",
                &format_args!("{}", self.lcd_va_height().bits()),
            )
            .field(
                "lcd_vt_height",
                &format_args!("{}", self.lcd_vt_height().bits()),
            )
            .field(
                "lcd_rgb_mode_en",
                &format_args!("{}", self.lcd_rgb_mode_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LCD_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:10 - It is the horizontal blank front porch of a frame."]
    #[inline(always)]
    #[must_use]
    pub fn lcd_hb_front(&mut self) -> LCD_HB_FRONT_W<LCD_CTRL_SPEC, 0> {
        LCD_HB_FRONT_W::new(self)
    }
    #[doc = "Bits 11:20 - It is the vertical active height of a frame."]
    #[inline(always)]
    #[must_use]
    pub fn lcd_va_height(&mut self) -> LCD_VA_HEIGHT_W<LCD_CTRL_SPEC, 11> {
        LCD_VA_HEIGHT_W::new(self)
    }
    #[doc = "Bits 21:30 - It is the vertical total height of a frame."]
    #[inline(always)]
    #[must_use]
    pub fn lcd_vt_height(&mut self) -> LCD_VT_HEIGHT_W<LCD_CTRL_SPEC, 21> {
        LCD_VT_HEIGHT_W::new(self)
    }
    #[doc = "Bit 31 - 1: Enable reg mode input vsync, hsync, de. 0: Disable."]
    #[inline(always)]
    #[must_use]
    pub fn lcd_rgb_mode_en(&mut self) -> LCD_RGB_MODE_EN_W<LCD_CTRL_SPEC, 31> {
        LCD_RGB_MODE_EN_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "LCD configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LCD_CTRL_SPEC;
impl crate::RegisterSpec for LCD_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcd_ctrl::R`](R) reader structure"]
impl crate::Readable for LCD_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lcd_ctrl::W`](W) writer structure"]
impl crate::Writable for LCD_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LCD_CTRL to value 0"]
impl crate::Resettable for LCD_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
