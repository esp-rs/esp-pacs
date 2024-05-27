///Register `LCD_CTRL2` reader
pub type R = crate::R<LCD_CTRL2_SPEC>;
///Register `LCD_CTRL2` writer
pub type W = crate::W<LCD_CTRL2_SPEC>;
///Field `LCD_VSYNC_WIDTH` reader - It is the position of spi_vsync active pulse in a line. Can be configured in CONF state.
pub type LCD_VSYNC_WIDTH_R = crate::FieldReader;
///Field `LCD_VSYNC_WIDTH` writer - It is the position of spi_vsync active pulse in a line. Can be configured in CONF state.
pub type LCD_VSYNC_WIDTH_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `VSYNC_IDLE_POL` reader - It is the idle value of spi_vsync. Can be configured in CONF state.
pub type VSYNC_IDLE_POL_R = crate::BitReader;
///Field `VSYNC_IDLE_POL` writer - It is the idle value of spi_vsync. Can be configured in CONF state.
pub type VSYNC_IDLE_POL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LCD_HSYNC_WIDTH` reader - It is the position of spi_hsync active pulse in a line. Can be configured in CONF state.
pub type LCD_HSYNC_WIDTH_R = crate::FieldReader;
///Field `LCD_HSYNC_WIDTH` writer - It is the position of spi_hsync active pulse in a line. Can be configured in CONF state.
pub type LCD_HSYNC_WIDTH_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `HSYNC_IDLE_POL` reader - It is the idle value of spi_hsync. Can be configured in CONF state.
pub type HSYNC_IDLE_POL_R = crate::BitReader;
///Field `HSYNC_IDLE_POL` writer - It is the idle value of spi_hsync. Can be configured in CONF state.
pub type HSYNC_IDLE_POL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LCD_HSYNC_POSITION` reader - It is the position of spi_hsync active pulse in a line. Can be configured in CONF state.
pub type LCD_HSYNC_POSITION_R = crate::FieldReader;
///Field `LCD_HSYNC_POSITION` writer - It is the position of spi_hsync active pulse in a line. Can be configured in CONF state.
pub type LCD_HSYNC_POSITION_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:6 - It is the position of spi_vsync active pulse in a line. Can be configured in CONF state.
    #[inline(always)]
    pub fn lcd_vsync_width(&self) -> LCD_VSYNC_WIDTH_R {
        LCD_VSYNC_WIDTH_R::new((self.bits & 0x7f) as u8)
    }
    ///Bit 7 - It is the idle value of spi_vsync. Can be configured in CONF state.
    #[inline(always)]
    pub fn vsync_idle_pol(&self) -> VSYNC_IDLE_POL_R {
        VSYNC_IDLE_POL_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 16:22 - It is the position of spi_hsync active pulse in a line. Can be configured in CONF state.
    #[inline(always)]
    pub fn lcd_hsync_width(&self) -> LCD_HSYNC_WIDTH_R {
        LCD_HSYNC_WIDTH_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    ///Bit 23 - It is the idle value of spi_hsync. Can be configured in CONF state.
    #[inline(always)]
    pub fn hsync_idle_pol(&self) -> HSYNC_IDLE_POL_R {
        HSYNC_IDLE_POL_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bits 24:31 - It is the position of spi_hsync active pulse in a line. Can be configured in CONF state.
    #[inline(always)]
    pub fn lcd_hsync_position(&self) -> LCD_HSYNC_POSITION_R {
        LCD_HSYNC_POSITION_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LCD_CTRL2")
            .field("lcd_vsync_width", &self.lcd_vsync_width())
            .field("vsync_idle_pol", &self.vsync_idle_pol())
            .field("lcd_hsync_width", &self.lcd_hsync_width())
            .field("hsync_idle_pol", &self.hsync_idle_pol())
            .field("lcd_hsync_position", &self.lcd_hsync_position())
            .finish()
    }
}
impl W {
    ///Bits 0:6 - It is the position of spi_vsync active pulse in a line. Can be configured in CONF state.
    #[inline(always)]
    #[must_use]
    pub fn lcd_vsync_width(&mut self) -> LCD_VSYNC_WIDTH_W<LCD_CTRL2_SPEC> {
        LCD_VSYNC_WIDTH_W::new(self, 0)
    }
    ///Bit 7 - It is the idle value of spi_vsync. Can be configured in CONF state.
    #[inline(always)]
    #[must_use]
    pub fn vsync_idle_pol(&mut self) -> VSYNC_IDLE_POL_W<LCD_CTRL2_SPEC> {
        VSYNC_IDLE_POL_W::new(self, 7)
    }
    ///Bits 16:22 - It is the position of spi_hsync active pulse in a line. Can be configured in CONF state.
    #[inline(always)]
    #[must_use]
    pub fn lcd_hsync_width(&mut self) -> LCD_HSYNC_WIDTH_W<LCD_CTRL2_SPEC> {
        LCD_HSYNC_WIDTH_W::new(self, 16)
    }
    ///Bit 23 - It is the idle value of spi_hsync. Can be configured in CONF state.
    #[inline(always)]
    #[must_use]
    pub fn hsync_idle_pol(&mut self) -> HSYNC_IDLE_POL_W<LCD_CTRL2_SPEC> {
        HSYNC_IDLE_POL_W::new(self, 23)
    }
    ///Bits 24:31 - It is the position of spi_hsync active pulse in a line. Can be configured in CONF state.
    #[inline(always)]
    #[must_use]
    pub fn lcd_hsync_position(&mut self) -> LCD_HSYNC_POSITION_W<LCD_CTRL2_SPEC> {
        LCD_HSYNC_POSITION_W::new(self, 24)
    }
}
/**LCD frame control2 register

You can [`read`](crate::generic::Reg::read) this register and get [`lcd_ctrl2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_ctrl2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct LCD_CTRL2_SPEC;
impl crate::RegisterSpec for LCD_CTRL2_SPEC {
    type Ux = u32;
}
///`read()` method returns [`lcd_ctrl2::R`](R) reader structure
impl crate::Readable for LCD_CTRL2_SPEC {}
///`write(|w| ..)` method takes [`lcd_ctrl2::W`](W) writer structure
impl crate::Writable for LCD_CTRL2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets LCD_CTRL2 to value 0x0001_0001
impl crate::Resettable for LCD_CTRL2_SPEC {
    const RESET_VALUE: u32 = 0x0001_0001;
}
