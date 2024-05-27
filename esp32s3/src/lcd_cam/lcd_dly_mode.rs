///Register `LCD_DLY_MODE` reader
pub type R = crate::R<LCD_DLY_MODE_SPEC>;
///Register `LCD_DLY_MODE` writer
pub type W = crate::W<LCD_DLY_MODE_SPEC>;
///Field `LCD_CD_MODE` reader - The output LCD_CD is delayed by module clock LCD_CLK. 0: output without delay. 1: delayed by the rising edge of LCD_CLK. 2: delayed by the falling edge of LCD_CLK.
pub type LCD_CD_MODE_R = crate::FieldReader;
///Field `LCD_CD_MODE` writer - The output LCD_CD is delayed by module clock LCD_CLK. 0: output without delay. 1: delayed by the rising edge of LCD_CLK. 2: delayed by the falling edge of LCD_CLK.
pub type LCD_CD_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `LCD_DE_MODE` reader - The output LCD_DE is delayed by module clock LCD_CLK. 0: output without delay. 1: delayed by the rising edge of LCD_CLK. 2: delayed by the falling edge of LCD_CLK.
pub type LCD_DE_MODE_R = crate::FieldReader;
///Field `LCD_DE_MODE` writer - The output LCD_DE is delayed by module clock LCD_CLK. 0: output without delay. 1: delayed by the rising edge of LCD_CLK. 2: delayed by the falling edge of LCD_CLK.
pub type LCD_DE_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `LCD_HSYNC_MODE` reader - The output LCD_HSYNC is delayed by module clock LCD_CLK. 0: output without delay. 1: delayed by the rising edge of LCD_CLK. 2: delayed by the falling edge of LCD_CLK.
pub type LCD_HSYNC_MODE_R = crate::FieldReader;
///Field `LCD_HSYNC_MODE` writer - The output LCD_HSYNC is delayed by module clock LCD_CLK. 0: output without delay. 1: delayed by the rising edge of LCD_CLK. 2: delayed by the falling edge of LCD_CLK.
pub type LCD_HSYNC_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `LCD_VSYNC_MODE` reader - The output LCD_VSYNC is delayed by module clock LCD_CLK. 0: output without delay. 1: delayed by the rising edge of LCD_CLK. 2: delay by the falling edge of LCD_CLK.
pub type LCD_VSYNC_MODE_R = crate::FieldReader;
///Field `LCD_VSYNC_MODE` writer - The output LCD_VSYNC is delayed by module clock LCD_CLK. 0: output without delay. 1: delayed by the rising edge of LCD_CLK. 2: delay by the falling edge of LCD_CLK.
pub type LCD_VSYNC_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:1 - The output LCD_CD is delayed by module clock LCD_CLK. 0: output without delay. 1: delayed by the rising edge of LCD_CLK. 2: delayed by the falling edge of LCD_CLK.
    #[inline(always)]
    pub fn lcd_cd_mode(&self) -> LCD_CD_MODE_R {
        LCD_CD_MODE_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - The output LCD_DE is delayed by module clock LCD_CLK. 0: output without delay. 1: delayed by the rising edge of LCD_CLK. 2: delayed by the falling edge of LCD_CLK.
    #[inline(always)]
    pub fn lcd_de_mode(&self) -> LCD_DE_MODE_R {
        LCD_DE_MODE_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:5 - The output LCD_HSYNC is delayed by module clock LCD_CLK. 0: output without delay. 1: delayed by the rising edge of LCD_CLK. 2: delayed by the falling edge of LCD_CLK.
    #[inline(always)]
    pub fn lcd_hsync_mode(&self) -> LCD_HSYNC_MODE_R {
        LCD_HSYNC_MODE_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 6:7 - The output LCD_VSYNC is delayed by module clock LCD_CLK. 0: output without delay. 1: delayed by the rising edge of LCD_CLK. 2: delay by the falling edge of LCD_CLK.
    #[inline(always)]
    pub fn lcd_vsync_mode(&self) -> LCD_VSYNC_MODE_R {
        LCD_VSYNC_MODE_R::new(((self.bits >> 6) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LCD_DLY_MODE")
            .field("lcd_cd_mode", &self.lcd_cd_mode())
            .field("lcd_de_mode", &self.lcd_de_mode())
            .field("lcd_hsync_mode", &self.lcd_hsync_mode())
            .field("lcd_vsync_mode", &self.lcd_vsync_mode())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - The output LCD_CD is delayed by module clock LCD_CLK. 0: output without delay. 1: delayed by the rising edge of LCD_CLK. 2: delayed by the falling edge of LCD_CLK.
    #[inline(always)]
    #[must_use]
    pub fn lcd_cd_mode(&mut self) -> LCD_CD_MODE_W<LCD_DLY_MODE_SPEC> {
        LCD_CD_MODE_W::new(self, 0)
    }
    ///Bits 2:3 - The output LCD_DE is delayed by module clock LCD_CLK. 0: output without delay. 1: delayed by the rising edge of LCD_CLK. 2: delayed by the falling edge of LCD_CLK.
    #[inline(always)]
    #[must_use]
    pub fn lcd_de_mode(&mut self) -> LCD_DE_MODE_W<LCD_DLY_MODE_SPEC> {
        LCD_DE_MODE_W::new(self, 2)
    }
    ///Bits 4:5 - The output LCD_HSYNC is delayed by module clock LCD_CLK. 0: output without delay. 1: delayed by the rising edge of LCD_CLK. 2: delayed by the falling edge of LCD_CLK.
    #[inline(always)]
    #[must_use]
    pub fn lcd_hsync_mode(&mut self) -> LCD_HSYNC_MODE_W<LCD_DLY_MODE_SPEC> {
        LCD_HSYNC_MODE_W::new(self, 4)
    }
    ///Bits 6:7 - The output LCD_VSYNC is delayed by module clock LCD_CLK. 0: output without delay. 1: delayed by the rising edge of LCD_CLK. 2: delay by the falling edge of LCD_CLK.
    #[inline(always)]
    #[must_use]
    pub fn lcd_vsync_mode(&mut self) -> LCD_VSYNC_MODE_W<LCD_DLY_MODE_SPEC> {
        LCD_VSYNC_MODE_W::new(self, 6)
    }
}
/**LCD signal delay configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`lcd_dly_mode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_dly_mode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct LCD_DLY_MODE_SPEC;
impl crate::RegisterSpec for LCD_DLY_MODE_SPEC {
    type Ux = u32;
}
///`read()` method returns [`lcd_dly_mode::R`](R) reader structure
impl crate::Readable for LCD_DLY_MODE_SPEC {}
///`write(|w| ..)` method takes [`lcd_dly_mode::W`](W) writer structure
impl crate::Writable for LCD_DLY_MODE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets LCD_DLY_MODE to value 0
impl crate::Resettable for LCD_DLY_MODE_SPEC {
    const RESET_VALUE: u32 = 0;
}
