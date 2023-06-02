#[doc = "Register `LCD_CTRL2` reader"]
pub struct R(crate::R<LCD_CTRL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCD_CTRL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCD_CTRL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCD_CTRL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LCD_CTRL2` writer"]
pub struct W(crate::W<LCD_CTRL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCD_CTRL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<LCD_CTRL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LCD_CTRL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LCD_VSYNC_WIDTH` reader - It is the position of LCD_VSYNC active pulse in a line."]
pub type LCD_VSYNC_WIDTH_R = crate::FieldReader;
#[doc = "Field `LCD_VSYNC_WIDTH` writer - It is the position of LCD_VSYNC active pulse in a line."]
pub type LCD_VSYNC_WIDTH_W<'a, const O: u8> = crate::FieldWriter<'a, LCD_CTRL2_SPEC, 7, O>;
#[doc = "Field `LCD_VSYNC_IDLE_POL` reader - It is the idle value of LCD_VSYNC."]
pub type LCD_VSYNC_IDLE_POL_R = crate::BitReader;
#[doc = "Field `LCD_VSYNC_IDLE_POL` writer - It is the idle value of LCD_VSYNC."]
pub type LCD_VSYNC_IDLE_POL_W<'a, const O: u8> = crate::BitWriter<'a, LCD_CTRL2_SPEC, O>;
#[doc = "Field `LCD_DE_IDLE_POL` reader - It is the idle value of LCD_DE."]
pub type LCD_DE_IDLE_POL_R = crate::BitReader;
#[doc = "Field `LCD_DE_IDLE_POL` writer - It is the idle value of LCD_DE."]
pub type LCD_DE_IDLE_POL_W<'a, const O: u8> = crate::BitWriter<'a, LCD_CTRL2_SPEC, O>;
#[doc = "Field `LCD_HS_BLANK_EN` reader - 1: The pulse of LCD_HSYNC is out in vertical blanking lines RGB mode. 0: LCD_HSYNC pulse is valid only in active region lines in RGB mode."]
pub type LCD_HS_BLANK_EN_R = crate::BitReader;
#[doc = "Field `LCD_HS_BLANK_EN` writer - 1: The pulse of LCD_HSYNC is out in vertical blanking lines RGB mode. 0: LCD_HSYNC pulse is valid only in active region lines in RGB mode."]
pub type LCD_HS_BLANK_EN_W<'a, const O: u8> = crate::BitWriter<'a, LCD_CTRL2_SPEC, O>;
#[doc = "Field `LCD_HSYNC_WIDTH` reader - It is the position of LCD_HSYNC active pulse in a line."]
pub type LCD_HSYNC_WIDTH_R = crate::FieldReader;
#[doc = "Field `LCD_HSYNC_WIDTH` writer - It is the position of LCD_HSYNC active pulse in a line."]
pub type LCD_HSYNC_WIDTH_W<'a, const O: u8> = crate::FieldWriter<'a, LCD_CTRL2_SPEC, 7, O>;
#[doc = "Field `LCD_HSYNC_IDLE_POL` reader - It is the idle value of LCD_HSYNC."]
pub type LCD_HSYNC_IDLE_POL_R = crate::BitReader;
#[doc = "Field `LCD_HSYNC_IDLE_POL` writer - It is the idle value of LCD_HSYNC."]
pub type LCD_HSYNC_IDLE_POL_W<'a, const O: u8> = crate::BitWriter<'a, LCD_CTRL2_SPEC, O>;
#[doc = "Field `LCD_HSYNC_POSITION` reader - It is the position of LCD_HSYNC active pulse in a line."]
pub type LCD_HSYNC_POSITION_R = crate::FieldReader;
#[doc = "Field `LCD_HSYNC_POSITION` writer - It is the position of LCD_HSYNC active pulse in a line."]
pub type LCD_HSYNC_POSITION_W<'a, const O: u8> = crate::FieldWriter<'a, LCD_CTRL2_SPEC, 8, O>;
impl R {
    #[doc = "Bits 0:6 - It is the position of LCD_VSYNC active pulse in a line."]
    #[inline(always)]
    pub fn lcd_vsync_width(&self) -> LCD_VSYNC_WIDTH_R {
        LCD_VSYNC_WIDTH_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - It is the idle value of LCD_VSYNC."]
    #[inline(always)]
    pub fn lcd_vsync_idle_pol(&self) -> LCD_VSYNC_IDLE_POL_R {
        LCD_VSYNC_IDLE_POL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - It is the idle value of LCD_DE."]
    #[inline(always)]
    pub fn lcd_de_idle_pol(&self) -> LCD_DE_IDLE_POL_R {
        LCD_DE_IDLE_POL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 1: The pulse of LCD_HSYNC is out in vertical blanking lines RGB mode. 0: LCD_HSYNC pulse is valid only in active region lines in RGB mode."]
    #[inline(always)]
    pub fn lcd_hs_blank_en(&self) -> LCD_HS_BLANK_EN_R {
        LCD_HS_BLANK_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 16:22 - It is the position of LCD_HSYNC active pulse in a line."]
    #[inline(always)]
    pub fn lcd_hsync_width(&self) -> LCD_HSYNC_WIDTH_R {
        LCD_HSYNC_WIDTH_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 23 - It is the idle value of LCD_HSYNC."]
    #[inline(always)]
    pub fn lcd_hsync_idle_pol(&self) -> LCD_HSYNC_IDLE_POL_R {
        LCD_HSYNC_IDLE_POL_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:31 - It is the position of LCD_HSYNC active pulse in a line."]
    #[inline(always)]
    pub fn lcd_hsync_position(&self) -> LCD_HSYNC_POSITION_R {
        LCD_HSYNC_POSITION_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LCD_CTRL2")
            .field(
                "lcd_vsync_width",
                &format_args!("{}", self.lcd_vsync_width().bits()),
            )
            .field(
                "lcd_vsync_idle_pol",
                &format_args!("{}", self.lcd_vsync_idle_pol().bit()),
            )
            .field(
                "lcd_de_idle_pol",
                &format_args!("{}", self.lcd_de_idle_pol().bit()),
            )
            .field(
                "lcd_hs_blank_en",
                &format_args!("{}", self.lcd_hs_blank_en().bit()),
            )
            .field(
                "lcd_hsync_width",
                &format_args!("{}", self.lcd_hsync_width().bits()),
            )
            .field(
                "lcd_hsync_idle_pol",
                &format_args!("{}", self.lcd_hsync_idle_pol().bit()),
            )
            .field(
                "lcd_hsync_position",
                &format_args!("{}", self.lcd_hsync_position().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LCD_CTRL2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:6 - It is the position of LCD_VSYNC active pulse in a line."]
    #[inline(always)]
    #[must_use]
    pub fn lcd_vsync_width(&mut self) -> LCD_VSYNC_WIDTH_W<0> {
        LCD_VSYNC_WIDTH_W::new(self)
    }
    #[doc = "Bit 7 - It is the idle value of LCD_VSYNC."]
    #[inline(always)]
    #[must_use]
    pub fn lcd_vsync_idle_pol(&mut self) -> LCD_VSYNC_IDLE_POL_W<7> {
        LCD_VSYNC_IDLE_POL_W::new(self)
    }
    #[doc = "Bit 8 - It is the idle value of LCD_DE."]
    #[inline(always)]
    #[must_use]
    pub fn lcd_de_idle_pol(&mut self) -> LCD_DE_IDLE_POL_W<8> {
        LCD_DE_IDLE_POL_W::new(self)
    }
    #[doc = "Bit 9 - 1: The pulse of LCD_HSYNC is out in vertical blanking lines RGB mode. 0: LCD_HSYNC pulse is valid only in active region lines in RGB mode."]
    #[inline(always)]
    #[must_use]
    pub fn lcd_hs_blank_en(&mut self) -> LCD_HS_BLANK_EN_W<9> {
        LCD_HS_BLANK_EN_W::new(self)
    }
    #[doc = "Bits 16:22 - It is the position of LCD_HSYNC active pulse in a line."]
    #[inline(always)]
    #[must_use]
    pub fn lcd_hsync_width(&mut self) -> LCD_HSYNC_WIDTH_W<16> {
        LCD_HSYNC_WIDTH_W::new(self)
    }
    #[doc = "Bit 23 - It is the idle value of LCD_HSYNC."]
    #[inline(always)]
    #[must_use]
    pub fn lcd_hsync_idle_pol(&mut self) -> LCD_HSYNC_IDLE_POL_W<23> {
        LCD_HSYNC_IDLE_POL_W::new(self)
    }
    #[doc = "Bits 24:31 - It is the position of LCD_HSYNC active pulse in a line."]
    #[inline(always)]
    #[must_use]
    pub fn lcd_hsync_position(&mut self) -> LCD_HSYNC_POSITION_W<24> {
        LCD_HSYNC_POSITION_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LCD configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcd_ctrl2](index.html) module"]
pub struct LCD_CTRL2_SPEC;
impl crate::RegisterSpec for LCD_CTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lcd_ctrl2::R](R) reader structure"]
impl crate::Readable for LCD_CTRL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lcd_ctrl2::W](W) writer structure"]
impl crate::Writable for LCD_CTRL2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LCD_CTRL2 to value 0x0001_0001"]
impl crate::Resettable for LCD_CTRL2_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_0001;
}
