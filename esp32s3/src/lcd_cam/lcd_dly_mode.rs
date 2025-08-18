#[doc = "Register `LCD_DLY_MODE` reader"]
pub type R = crate::R<LCD_DLY_MODE_SPEC>;
#[doc = "Register `LCD_DLY_MODE` writer"]
pub type W = crate::W<LCD_DLY_MODE_SPEC>;
#[doc = "Field `LCD_CD_MODE` reader - The output LCD_CD is delayed by module clock LCD_CLK. 0: output without delay. 1: delayed by the rising edge of LCD_CLK. 2: delayed by the falling edge of LCD_CLK."]
pub type LCD_CD_MODE_R = crate::FieldReader;
#[doc = "Field `LCD_CD_MODE` writer - The output LCD_CD is delayed by module clock LCD_CLK. 0: output without delay. 1: delayed by the rising edge of LCD_CLK. 2: delayed by the falling edge of LCD_CLK."]
pub type LCD_CD_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LCD_DE_MODE` reader - The output LCD_DE is delayed by module clock LCD_CLK. 0: output without delay. 1: delayed by the rising edge of LCD_CLK. 2: delayed by the falling edge of LCD_CLK."]
pub type LCD_DE_MODE_R = crate::FieldReader;
#[doc = "Field `LCD_DE_MODE` writer - The output LCD_DE is delayed by module clock LCD_CLK. 0: output without delay. 1: delayed by the rising edge of LCD_CLK. 2: delayed by the falling edge of LCD_CLK."]
pub type LCD_DE_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LCD_HSYNC_MODE` reader - The output LCD_HSYNC is delayed by module clock LCD_CLK. 0: output without delay. 1: delayed by the rising edge of LCD_CLK. 2: delayed by the falling edge of LCD_CLK."]
pub type LCD_HSYNC_MODE_R = crate::FieldReader;
#[doc = "Field `LCD_HSYNC_MODE` writer - The output LCD_HSYNC is delayed by module clock LCD_CLK. 0: output without delay. 1: delayed by the rising edge of LCD_CLK. 2: delayed by the falling edge of LCD_CLK."]
pub type LCD_HSYNC_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LCD_VSYNC_MODE` reader - The output LCD_VSYNC is delayed by module clock LCD_CLK. 0: output without delay. 1: delayed by the rising edge of LCD_CLK. 2: delay by the falling edge of LCD_CLK."]
pub type LCD_VSYNC_MODE_R = crate::FieldReader;
#[doc = "Field `LCD_VSYNC_MODE` writer - The output LCD_VSYNC is delayed by module clock LCD_CLK. 0: output without delay. 1: delayed by the rising edge of LCD_CLK. 2: delay by the falling edge of LCD_CLK."]
pub type LCD_VSYNC_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - The output LCD_CD is delayed by module clock LCD_CLK. 0: output without delay. 1: delayed by the rising edge of LCD_CLK. 2: delayed by the falling edge of LCD_CLK."]
    #[inline(always)]
    pub fn lcd_cd_mode(&self) -> LCD_CD_MODE_R {
        LCD_CD_MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - The output LCD_DE is delayed by module clock LCD_CLK. 0: output without delay. 1: delayed by the rising edge of LCD_CLK. 2: delayed by the falling edge of LCD_CLK."]
    #[inline(always)]
    pub fn lcd_de_mode(&self) -> LCD_DE_MODE_R {
        LCD_DE_MODE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - The output LCD_HSYNC is delayed by module clock LCD_CLK. 0: output without delay. 1: delayed by the rising edge of LCD_CLK. 2: delayed by the falling edge of LCD_CLK."]
    #[inline(always)]
    pub fn lcd_hsync_mode(&self) -> LCD_HSYNC_MODE_R {
        LCD_HSYNC_MODE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - The output LCD_VSYNC is delayed by module clock LCD_CLK. 0: output without delay. 1: delayed by the rising edge of LCD_CLK. 2: delay by the falling edge of LCD_CLK."]
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
    #[doc = "Bits 0:1 - The output LCD_CD is delayed by module clock LCD_CLK. 0: output without delay. 1: delayed by the rising edge of LCD_CLK. 2: delayed by the falling edge of LCD_CLK."]
    #[inline(always)]
    pub fn lcd_cd_mode(&mut self) -> LCD_CD_MODE_W<'_, LCD_DLY_MODE_SPEC> {
        LCD_CD_MODE_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - The output LCD_DE is delayed by module clock LCD_CLK. 0: output without delay. 1: delayed by the rising edge of LCD_CLK. 2: delayed by the falling edge of LCD_CLK."]
    #[inline(always)]
    pub fn lcd_de_mode(&mut self) -> LCD_DE_MODE_W<'_, LCD_DLY_MODE_SPEC> {
        LCD_DE_MODE_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - The output LCD_HSYNC is delayed by module clock LCD_CLK. 0: output without delay. 1: delayed by the rising edge of LCD_CLK. 2: delayed by the falling edge of LCD_CLK."]
    #[inline(always)]
    pub fn lcd_hsync_mode(&mut self) -> LCD_HSYNC_MODE_W<'_, LCD_DLY_MODE_SPEC> {
        LCD_HSYNC_MODE_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - The output LCD_VSYNC is delayed by module clock LCD_CLK. 0: output without delay. 1: delayed by the rising edge of LCD_CLK. 2: delay by the falling edge of LCD_CLK."]
    #[inline(always)]
    pub fn lcd_vsync_mode(&mut self) -> LCD_VSYNC_MODE_W<'_, LCD_DLY_MODE_SPEC> {
        LCD_VSYNC_MODE_W::new(self, 6)
    }
}
#[doc = "LCD signal delay configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_dly_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_dly_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LCD_DLY_MODE_SPEC;
impl crate::RegisterSpec for LCD_DLY_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcd_dly_mode::R`](R) reader structure"]
impl crate::Readable for LCD_DLY_MODE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lcd_dly_mode::W`](W) writer structure"]
impl crate::Writable for LCD_DLY_MODE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCD_DLY_MODE to value 0"]
impl crate::Resettable for LCD_DLY_MODE_SPEC {}
