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
#[doc = "Field `LCD_VSYNC_WIDTH` reader - It is the position of spi_vsync active pulse in a line. Can be configured in CONF state."]
pub type LCD_VSYNC_WIDTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LCD_VSYNC_WIDTH` writer - It is the position of spi_vsync active pulse in a line. Can be configured in CONF state."]
pub type LCD_VSYNC_WIDTH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LCD_CTRL2_SPEC, u8, u8, 7, O>;
#[doc = "Field `VSYNC_IDLE_POL` reader - It is the idle value of spi_vsync. Can be configured in CONF state."]
pub type VSYNC_IDLE_POL_R = crate::BitReader<bool>;
#[doc = "Field `VSYNC_IDLE_POL` writer - It is the idle value of spi_vsync. Can be configured in CONF state."]
pub type VSYNC_IDLE_POL_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCD_CTRL2_SPEC, bool, O>;
#[doc = "Field `LCD_HSYNC_WIDTH` reader - It is the position of spi_hsync active pulse in a line. Can be configured in CONF state."]
pub type LCD_HSYNC_WIDTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LCD_HSYNC_WIDTH` writer - It is the position of spi_hsync active pulse in a line. Can be configured in CONF state."]
pub type LCD_HSYNC_WIDTH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LCD_CTRL2_SPEC, u8, u8, 7, O>;
#[doc = "Field `HSYNC_IDLE_POL` reader - It is the idle value of spi_hsync. Can be configured in CONF state."]
pub type HSYNC_IDLE_POL_R = crate::BitReader<bool>;
#[doc = "Field `HSYNC_IDLE_POL` writer - It is the idle value of spi_hsync. Can be configured in CONF state."]
pub type HSYNC_IDLE_POL_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCD_CTRL2_SPEC, bool, O>;
#[doc = "Field `LCD_HSYNC_POSITION` reader - It is the position of spi_hsync active pulse in a line. Can be configured in CONF state."]
pub type LCD_HSYNC_POSITION_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LCD_HSYNC_POSITION` writer - It is the position of spi_hsync active pulse in a line. Can be configured in CONF state."]
pub type LCD_HSYNC_POSITION_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LCD_CTRL2_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:6 - It is the position of spi_vsync active pulse in a line. Can be configured in CONF state."]
    #[inline(always)]
    pub fn lcd_vsync_width(&self) -> LCD_VSYNC_WIDTH_R {
        LCD_VSYNC_WIDTH_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - It is the idle value of spi_vsync. Can be configured in CONF state."]
    #[inline(always)]
    pub fn vsync_idle_pol(&self) -> VSYNC_IDLE_POL_R {
        VSYNC_IDLE_POL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 16:22 - It is the position of spi_hsync active pulse in a line. Can be configured in CONF state."]
    #[inline(always)]
    pub fn lcd_hsync_width(&self) -> LCD_HSYNC_WIDTH_R {
        LCD_HSYNC_WIDTH_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 23 - It is the idle value of spi_hsync. Can be configured in CONF state."]
    #[inline(always)]
    pub fn hsync_idle_pol(&self) -> HSYNC_IDLE_POL_R {
        HSYNC_IDLE_POL_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:31 - It is the position of spi_hsync active pulse in a line. Can be configured in CONF state."]
    #[inline(always)]
    pub fn lcd_hsync_position(&self) -> LCD_HSYNC_POSITION_R {
        LCD_HSYNC_POSITION_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - It is the position of spi_vsync active pulse in a line. Can be configured in CONF state."]
    #[inline(always)]
    pub fn lcd_vsync_width(&mut self) -> LCD_VSYNC_WIDTH_W<0> {
        LCD_VSYNC_WIDTH_W::new(self)
    }
    #[doc = "Bit 7 - It is the idle value of spi_vsync. Can be configured in CONF state."]
    #[inline(always)]
    pub fn vsync_idle_pol(&mut self) -> VSYNC_IDLE_POL_W<7> {
        VSYNC_IDLE_POL_W::new(self)
    }
    #[doc = "Bits 16:22 - It is the position of spi_hsync active pulse in a line. Can be configured in CONF state."]
    #[inline(always)]
    pub fn lcd_hsync_width(&mut self) -> LCD_HSYNC_WIDTH_W<16> {
        LCD_HSYNC_WIDTH_W::new(self)
    }
    #[doc = "Bit 23 - It is the idle value of spi_hsync. Can be configured in CONF state."]
    #[inline(always)]
    pub fn hsync_idle_pol(&mut self) -> HSYNC_IDLE_POL_W<23> {
        HSYNC_IDLE_POL_W::new(self)
    }
    #[doc = "Bits 24:31 - It is the position of spi_hsync active pulse in a line. Can be configured in CONF state."]
    #[inline(always)]
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
#[doc = "LCD frame control2 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcd_ctrl2](index.html) module"]
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
}
#[doc = "`reset()` method sets LCD_CTRL2 to value 0x0001_0001"]
impl crate::Resettable for LCD_CTRL2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0001_0001
    }
}
