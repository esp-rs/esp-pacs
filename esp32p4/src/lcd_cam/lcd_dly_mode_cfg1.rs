#[doc = "Register `LCD_DLY_MODE_CFG1` reader"]
pub type R = crate::R<LCD_DLY_MODE_CFG1_SPEC>;
#[doc = "Register `LCD_DLY_MODE_CFG1` writer"]
pub type W = crate::W<LCD_DLY_MODE_CFG1_SPEC>;
#[doc = "The output data bit %s is delayed by module clock LCD_CLK\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DELAY_MODE {
    #[doc = "0: Output without delay"]
    None = 0,
    #[doc = "1: Delayed by the rising edge of LCD_CLK"]
    RaisingEdge = 1,
    #[doc = "2: Delayed by the falling edge of LCD_CLK"]
    FallingEdge = 2,
}
impl From<DELAY_MODE> for u8 {
    #[inline(always)]
    fn from(variant: DELAY_MODE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DELAY_MODE {
    type Ux = u8;
}
impl crate::IsEnum for DELAY_MODE {}
#[doc = "Field `DOUT_MODE(16-23)` reader - The output data bit %s is delayed by module clock LCD_CLK"]
pub type DOUT_MODE_R = crate::FieldReader<DELAY_MODE>;
impl DOUT_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DELAY_MODE> {
        match self.bits {
            0 => Some(DELAY_MODE::None),
            1 => Some(DELAY_MODE::RaisingEdge),
            2 => Some(DELAY_MODE::FallingEdge),
            _ => None,
        }
    }
    #[doc = "Output without delay"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == DELAY_MODE::None
    }
    #[doc = "Delayed by the rising edge of LCD_CLK"]
    #[inline(always)]
    pub fn is_raising_edge(&self) -> bool {
        *self == DELAY_MODE::RaisingEdge
    }
    #[doc = "Delayed by the falling edge of LCD_CLK"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == DELAY_MODE::FallingEdge
    }
}
#[doc = "Field `DOUT_MODE(16-23)` writer - The output data bit %s is delayed by module clock LCD_CLK"]
pub type DOUT_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, DELAY_MODE>;
impl<'a, REG> DOUT_MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Output without delay"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(DELAY_MODE::None)
    }
    #[doc = "Delayed by the rising edge of LCD_CLK"]
    #[inline(always)]
    pub fn raising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(DELAY_MODE::RaisingEdge)
    }
    #[doc = "Delayed by the falling edge of LCD_CLK"]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(DELAY_MODE::FallingEdge)
    }
}
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
    #[doc = "The output data bit (16-23) is delayed by module clock LCD_CLK"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `DOUT16_MODE` field"]
    #[inline(always)]
    pub fn dout_mode(&self, n: u8) -> DOUT_MODE_R {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        DOUT_MODE_R::new(((self.bits >> (n * 2)) & 3) as u8)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "The output data bit (16-23) is delayed by module clock LCD_CLK"]
    #[inline(always)]
    pub fn dout_mode_iter(&self) -> impl Iterator<Item = DOUT_MODE_R> + '_ {
        (0..8).map(move |n| DOUT_MODE_R::new(((self.bits >> (n * 2)) & 3) as u8))
    }
    #[doc = "Bits 0:1 - The output data bit 16 is delayed by module clock LCD_CLK"]
    #[inline(always)]
    pub fn dout16_mode(&self) -> DOUT_MODE_R {
        DOUT_MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - The output data bit 17 is delayed by module clock LCD_CLK"]
    #[inline(always)]
    pub fn dout17_mode(&self) -> DOUT_MODE_R {
        DOUT_MODE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - The output data bit 18 is delayed by module clock LCD_CLK"]
    #[inline(always)]
    pub fn dout18_mode(&self) -> DOUT_MODE_R {
        DOUT_MODE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - The output data bit 19 is delayed by module clock LCD_CLK"]
    #[inline(always)]
    pub fn dout19_mode(&self) -> DOUT_MODE_R {
        DOUT_MODE_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - The output data bit 20 is delayed by module clock LCD_CLK"]
    #[inline(always)]
    pub fn dout20_mode(&self) -> DOUT_MODE_R {
        DOUT_MODE_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - The output data bit 21 is delayed by module clock LCD_CLK"]
    #[inline(always)]
    pub fn dout21_mode(&self) -> DOUT_MODE_R {
        DOUT_MODE_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - The output data bit 22 is delayed by module clock LCD_CLK"]
    #[inline(always)]
    pub fn dout22_mode(&self) -> DOUT_MODE_R {
        DOUT_MODE_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - The output data bit 23 is delayed by module clock LCD_CLK"]
    #[inline(always)]
    pub fn dout23_mode(&self) -> DOUT_MODE_R {
        DOUT_MODE_R::new(((self.bits >> 14) & 3) as u8)
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
            .field("dout16_mode", &self.dout16_mode())
            .field("dout17_mode", &self.dout17_mode())
            .field("dout18_mode", &self.dout18_mode())
            .field("dout19_mode", &self.dout19_mode())
            .field("dout20_mode", &self.dout20_mode())
            .field("dout21_mode", &self.dout21_mode())
            .field("dout22_mode", &self.dout22_mode())
            .field("dout23_mode", &self.dout23_mode())
            .field("lcd_cd_mode", &self.lcd_cd_mode())
            .field("lcd_de_mode", &self.lcd_de_mode())
            .field("lcd_hsync_mode", &self.lcd_hsync_mode())
            .field("lcd_vsync_mode", &self.lcd_vsync_mode())
            .finish()
    }
}
impl W {
    #[doc = "The output data bit (16-23) is delayed by module clock LCD_CLK"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `DOUT16_MODE` field"]
    #[inline(always)]
    #[must_use]
    pub fn dout_mode(&mut self, n: u8) -> DOUT_MODE_W<LCD_DLY_MODE_CFG1_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        DOUT_MODE_W::new(self, n * 2)
    }
    #[doc = "Bits 0:1 - The output data bit 16 is delayed by module clock LCD_CLK"]
    #[inline(always)]
    #[must_use]
    pub fn dout16_mode(&mut self) -> DOUT_MODE_W<LCD_DLY_MODE_CFG1_SPEC> {
        DOUT_MODE_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - The output data bit 17 is delayed by module clock LCD_CLK"]
    #[inline(always)]
    #[must_use]
    pub fn dout17_mode(&mut self) -> DOUT_MODE_W<LCD_DLY_MODE_CFG1_SPEC> {
        DOUT_MODE_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - The output data bit 18 is delayed by module clock LCD_CLK"]
    #[inline(always)]
    #[must_use]
    pub fn dout18_mode(&mut self) -> DOUT_MODE_W<LCD_DLY_MODE_CFG1_SPEC> {
        DOUT_MODE_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - The output data bit 19 is delayed by module clock LCD_CLK"]
    #[inline(always)]
    #[must_use]
    pub fn dout19_mode(&mut self) -> DOUT_MODE_W<LCD_DLY_MODE_CFG1_SPEC> {
        DOUT_MODE_W::new(self, 6)
    }
    #[doc = "Bits 8:9 - The output data bit 20 is delayed by module clock LCD_CLK"]
    #[inline(always)]
    #[must_use]
    pub fn dout20_mode(&mut self) -> DOUT_MODE_W<LCD_DLY_MODE_CFG1_SPEC> {
        DOUT_MODE_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - The output data bit 21 is delayed by module clock LCD_CLK"]
    #[inline(always)]
    #[must_use]
    pub fn dout21_mode(&mut self) -> DOUT_MODE_W<LCD_DLY_MODE_CFG1_SPEC> {
        DOUT_MODE_W::new(self, 10)
    }
    #[doc = "Bits 12:13 - The output data bit 22 is delayed by module clock LCD_CLK"]
    #[inline(always)]
    #[must_use]
    pub fn dout22_mode(&mut self) -> DOUT_MODE_W<LCD_DLY_MODE_CFG1_SPEC> {
        DOUT_MODE_W::new(self, 12)
    }
    #[doc = "Bits 14:15 - The output data bit 23 is delayed by module clock LCD_CLK"]
    #[inline(always)]
    #[must_use]
    pub fn dout23_mode(&mut self) -> DOUT_MODE_W<LCD_DLY_MODE_CFG1_SPEC> {
        DOUT_MODE_W::new(self, 14)
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
