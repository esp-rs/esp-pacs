#[doc = "Register `LCD_DLY_MODE_CFG1` reader"]
pub type R = crate::R<LCD_DLY_MODE_CFG1_SPEC>;
#[doc = "Register `LCD_DLY_MODE_CFG1` writer"]
pub type W = crate::W<LCD_DLY_MODE_CFG1_SPEC>;
#[doc = "Field `DOUT16_MODE` reader - The output data bit 0 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
pub type DOUT16_MODE_R = crate::FieldReader;
#[doc = "Field `DOUT16_MODE` writer - The output data bit 0 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
pub type DOUT16_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DOUT17_MODE` reader - The output data bit 2 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
pub type DOUT17_MODE_R = crate::FieldReader;
#[doc = "Field `DOUT17_MODE` writer - The output data bit 2 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
pub type DOUT17_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DOUT18_MODE` reader - The output data bit 4 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
pub type DOUT18_MODE_R = crate::FieldReader;
#[doc = "Field `DOUT18_MODE` writer - The output data bit 4 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
pub type DOUT18_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DOUT19_MODE` reader - The output data bit 6 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
pub type DOUT19_MODE_R = crate::FieldReader;
#[doc = "Field `DOUT19_MODE` writer - The output data bit 6 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
pub type DOUT19_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DOUT20_MODE` reader - The output data bit 8 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
pub type DOUT20_MODE_R = crate::FieldReader;
#[doc = "Field `DOUT20_MODE` writer - The output data bit 8 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
pub type DOUT20_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DOUT21_MODE` reader - The output data bit 10 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
pub type DOUT21_MODE_R = crate::FieldReader;
#[doc = "Field `DOUT21_MODE` writer - The output data bit 10 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
pub type DOUT21_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DOUT22_MODE` reader - The output data bit 12 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
pub type DOUT22_MODE_R = crate::FieldReader;
#[doc = "Field `DOUT22_MODE` writer - The output data bit 12 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
pub type DOUT22_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DOUT23_MODE` reader - The output data bit 14 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
pub type DOUT23_MODE_R = crate::FieldReader;
#[doc = "Field `DOUT23_MODE` writer - The output data bit 14 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
pub type DOUT23_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LCD_CD_MODE` reader - The output LCD_CD is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
pub type LCD_CD_MODE_R = crate::FieldReader;
#[doc = "Field `LCD_CD_MODE` writer - The output LCD_CD is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
pub type LCD_CD_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LCD_DE_MODE` reader - The output LCD_DE is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
pub type LCD_DE_MODE_R = crate::FieldReader;
#[doc = "Field `LCD_DE_MODE` writer - The output LCD_DE is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
pub type LCD_DE_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LCD_HSYNC_MODE` reader - The output LCD_HSYNC is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
pub type LCD_HSYNC_MODE_R = crate::FieldReader;
#[doc = "Field `LCD_HSYNC_MODE` writer - The output LCD_HSYNC is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
pub type LCD_HSYNC_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LCD_VSYNC_MODE` reader - The output LCD_VSYNC is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
pub type LCD_VSYNC_MODE_R = crate::FieldReader;
#[doc = "Field `LCD_VSYNC_MODE` writer - The output LCD_VSYNC is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
pub type LCD_VSYNC_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - The output data bit 0 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
    #[inline(always)]
    pub fn dout16_mode(&self) -> DOUT16_MODE_R {
        DOUT16_MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - The output data bit 2 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
    #[inline(always)]
    pub fn dout17_mode(&self) -> DOUT17_MODE_R {
        DOUT17_MODE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - The output data bit 4 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
    #[inline(always)]
    pub fn dout18_mode(&self) -> DOUT18_MODE_R {
        DOUT18_MODE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - The output data bit 6 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
    #[inline(always)]
    pub fn dout19_mode(&self) -> DOUT19_MODE_R {
        DOUT19_MODE_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - The output data bit 8 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
    #[inline(always)]
    pub fn dout20_mode(&self) -> DOUT20_MODE_R {
        DOUT20_MODE_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - The output data bit 10 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
    #[inline(always)]
    pub fn dout21_mode(&self) -> DOUT21_MODE_R {
        DOUT21_MODE_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - The output data bit 12 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
    #[inline(always)]
    pub fn dout22_mode(&self) -> DOUT22_MODE_R {
        DOUT22_MODE_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - The output data bit 14 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
    #[inline(always)]
    pub fn dout23_mode(&self) -> DOUT23_MODE_R {
        DOUT23_MODE_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - The output LCD_CD is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
    #[inline(always)]
    pub fn lcd_cd_mode(&self) -> LCD_CD_MODE_R {
        LCD_CD_MODE_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - The output LCD_DE is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
    #[inline(always)]
    pub fn lcd_de_mode(&self) -> LCD_DE_MODE_R {
        LCD_DE_MODE_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - The output LCD_HSYNC is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
    #[inline(always)]
    pub fn lcd_hsync_mode(&self) -> LCD_HSYNC_MODE_R {
        LCD_HSYNC_MODE_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - The output LCD_VSYNC is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
    #[inline(always)]
    pub fn lcd_vsync_mode(&self) -> LCD_VSYNC_MODE_R {
        LCD_VSYNC_MODE_R::new(((self.bits >> 22) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LCD_DLY_MODE_CFG1")
            .field(
                "dout16_mode",
                &format_args!("{}", self.dout16_mode().bits()),
            )
            .field(
                "dout17_mode",
                &format_args!("{}", self.dout17_mode().bits()),
            )
            .field(
                "dout18_mode",
                &format_args!("{}", self.dout18_mode().bits()),
            )
            .field(
                "dout19_mode",
                &format_args!("{}", self.dout19_mode().bits()),
            )
            .field(
                "dout20_mode",
                &format_args!("{}", self.dout20_mode().bits()),
            )
            .field(
                "dout21_mode",
                &format_args!("{}", self.dout21_mode().bits()),
            )
            .field(
                "dout22_mode",
                &format_args!("{}", self.dout22_mode().bits()),
            )
            .field(
                "dout23_mode",
                &format_args!("{}", self.dout23_mode().bits()),
            )
            .field(
                "lcd_cd_mode",
                &format_args!("{}", self.lcd_cd_mode().bits()),
            )
            .field(
                "lcd_de_mode",
                &format_args!("{}", self.lcd_de_mode().bits()),
            )
            .field(
                "lcd_hsync_mode",
                &format_args!("{}", self.lcd_hsync_mode().bits()),
            )
            .field(
                "lcd_vsync_mode",
                &format_args!("{}", self.lcd_vsync_mode().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LCD_DLY_MODE_CFG1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:1 - The output data bit 0 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
    #[inline(always)]
    #[must_use]
    pub fn dout16_mode(&mut self) -> DOUT16_MODE_W<LCD_DLY_MODE_CFG1_SPEC> {
        DOUT16_MODE_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - The output data bit 2 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
    #[inline(always)]
    #[must_use]
    pub fn dout17_mode(&mut self) -> DOUT17_MODE_W<LCD_DLY_MODE_CFG1_SPEC> {
        DOUT17_MODE_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - The output data bit 4 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
    #[inline(always)]
    #[must_use]
    pub fn dout18_mode(&mut self) -> DOUT18_MODE_W<LCD_DLY_MODE_CFG1_SPEC> {
        DOUT18_MODE_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - The output data bit 6 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
    #[inline(always)]
    #[must_use]
    pub fn dout19_mode(&mut self) -> DOUT19_MODE_W<LCD_DLY_MODE_CFG1_SPEC> {
        DOUT19_MODE_W::new(self, 6)
    }
    #[doc = "Bits 8:9 - The output data bit 8 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
    #[inline(always)]
    #[must_use]
    pub fn dout20_mode(&mut self) -> DOUT20_MODE_W<LCD_DLY_MODE_CFG1_SPEC> {
        DOUT20_MODE_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - The output data bit 10 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
    #[inline(always)]
    #[must_use]
    pub fn dout21_mode(&mut self) -> DOUT21_MODE_W<LCD_DLY_MODE_CFG1_SPEC> {
        DOUT21_MODE_W::new(self, 10)
    }
    #[doc = "Bits 12:13 - The output data bit 12 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
    #[inline(always)]
    #[must_use]
    pub fn dout22_mode(&mut self) -> DOUT22_MODE_W<LCD_DLY_MODE_CFG1_SPEC> {
        DOUT22_MODE_W::new(self, 12)
    }
    #[doc = "Bits 14:15 - The output data bit 14 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
    #[inline(always)]
    #[must_use]
    pub fn dout23_mode(&mut self) -> DOUT23_MODE_W<LCD_DLY_MODE_CFG1_SPEC> {
        DOUT23_MODE_W::new(self, 14)
    }
    #[doc = "Bits 16:17 - The output LCD_CD is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
    #[inline(always)]
    #[must_use]
    pub fn lcd_cd_mode(&mut self) -> LCD_CD_MODE_W<LCD_DLY_MODE_CFG1_SPEC> {
        LCD_CD_MODE_W::new(self, 16)
    }
    #[doc = "Bits 18:19 - The output LCD_DE is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
    #[inline(always)]
    #[must_use]
    pub fn lcd_de_mode(&mut self) -> LCD_DE_MODE_W<LCD_DLY_MODE_CFG1_SPEC> {
        LCD_DE_MODE_W::new(self, 18)
    }
    #[doc = "Bits 20:21 - The output LCD_HSYNC is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
    #[inline(always)]
    #[must_use]
    pub fn lcd_hsync_mode(&mut self) -> LCD_HSYNC_MODE_W<LCD_DLY_MODE_CFG1_SPEC> {
        LCD_HSYNC_MODE_W::new(self, 20)
    }
    #[doc = "Bits 22:23 - The output LCD_VSYNC is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
    #[inline(always)]
    #[must_use]
    pub fn lcd_vsync_mode(&mut self) -> LCD_VSYNC_MODE_W<LCD_DLY_MODE_CFG1_SPEC> {
        LCD_VSYNC_MODE_W::new(self, 22)
    }
}
#[doc = "LCD config register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_dly_mode_cfg1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_dly_mode_cfg1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LCD_DLY_MODE_CFG1_SPEC;
impl crate::RegisterSpec for LCD_DLY_MODE_CFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcd_dly_mode_cfg1::R`](R) reader structure"]
impl crate::Readable for LCD_DLY_MODE_CFG1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lcd_dly_mode_cfg1::W`](W) writer structure"]
impl crate::Writable for LCD_DLY_MODE_CFG1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LCD_DLY_MODE_CFG1 to value 0"]
impl crate::Resettable for LCD_DLY_MODE_CFG1_SPEC {
    const RESET_VALUE: u32 = 0;
}
