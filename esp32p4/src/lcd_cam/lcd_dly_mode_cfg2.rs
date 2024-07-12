#[doc = "Register `LCD_DLY_MODE_CFG2` reader"]
pub type R = crate::R<LCD_DLY_MODE_CFG2_SPEC>;
#[doc = "Register `LCD_DLY_MODE_CFG2` writer"]
pub type W = crate::W<LCD_DLY_MODE_CFG2_SPEC>;
#[doc = "The output data bit %s is delayed by module clock LCD_CLK"]
pub use super::lcd_dly_mode_cfg1::DELAY_MODE;
#[doc = "Field `DOUT_MODE(0-15)` reader - The output data bit %s is delayed by module clock LCD_CLK"]
pub use super::lcd_dly_mode_cfg1::DOUT_MODE_R;
#[doc = "Field `DOUT_MODE(0-15)` writer - The output data bit %s is delayed by module clock LCD_CLK"]
pub use super::lcd_dly_mode_cfg1::DOUT_MODE_W;
impl R {
    #[doc = "The output data bit (0-15) is delayed by module clock LCD_CLK"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `DOUT0_MODE` field.</div>"]
    #[inline(always)]
    pub fn dout_mode(&self, n: u8) -> DOUT_MODE_R {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        DOUT_MODE_R::new(((self.bits >> (n * 2)) & 3) as u8)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "The output data bit (0-15) is delayed by module clock LCD_CLK"]
    #[inline(always)]
    pub fn dout_mode_iter(&self) -> impl Iterator<Item = DOUT_MODE_R> + '_ {
        (0..16).map(move |n| DOUT_MODE_R::new(((self.bits >> (n * 2)) & 3) as u8))
    }
    #[doc = "Bits 0:1 - The output data bit 0 is delayed by module clock LCD_CLK"]
    #[inline(always)]
    pub fn dout0_mode(&self) -> DOUT_MODE_R {
        DOUT_MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - The output data bit 1 is delayed by module clock LCD_CLK"]
    #[inline(always)]
    pub fn dout1_mode(&self) -> DOUT_MODE_R {
        DOUT_MODE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - The output data bit 2 is delayed by module clock LCD_CLK"]
    #[inline(always)]
    pub fn dout2_mode(&self) -> DOUT_MODE_R {
        DOUT_MODE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - The output data bit 3 is delayed by module clock LCD_CLK"]
    #[inline(always)]
    pub fn dout3_mode(&self) -> DOUT_MODE_R {
        DOUT_MODE_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - The output data bit 4 is delayed by module clock LCD_CLK"]
    #[inline(always)]
    pub fn dout4_mode(&self) -> DOUT_MODE_R {
        DOUT_MODE_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - The output data bit 5 is delayed by module clock LCD_CLK"]
    #[inline(always)]
    pub fn dout5_mode(&self) -> DOUT_MODE_R {
        DOUT_MODE_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - The output data bit 6 is delayed by module clock LCD_CLK"]
    #[inline(always)]
    pub fn dout6_mode(&self) -> DOUT_MODE_R {
        DOUT_MODE_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - The output data bit 7 is delayed by module clock LCD_CLK"]
    #[inline(always)]
    pub fn dout7_mode(&self) -> DOUT_MODE_R {
        DOUT_MODE_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - The output data bit 8 is delayed by module clock LCD_CLK"]
    #[inline(always)]
    pub fn dout8_mode(&self) -> DOUT_MODE_R {
        DOUT_MODE_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - The output data bit 9 is delayed by module clock LCD_CLK"]
    #[inline(always)]
    pub fn dout9_mode(&self) -> DOUT_MODE_R {
        DOUT_MODE_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - The output data bit 10 is delayed by module clock LCD_CLK"]
    #[inline(always)]
    pub fn dout10_mode(&self) -> DOUT_MODE_R {
        DOUT_MODE_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - The output data bit 11 is delayed by module clock LCD_CLK"]
    #[inline(always)]
    pub fn dout11_mode(&self) -> DOUT_MODE_R {
        DOUT_MODE_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - The output data bit 12 is delayed by module clock LCD_CLK"]
    #[inline(always)]
    pub fn dout12_mode(&self) -> DOUT_MODE_R {
        DOUT_MODE_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - The output data bit 13 is delayed by module clock LCD_CLK"]
    #[inline(always)]
    pub fn dout13_mode(&self) -> DOUT_MODE_R {
        DOUT_MODE_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - The output data bit 14 is delayed by module clock LCD_CLK"]
    #[inline(always)]
    pub fn dout14_mode(&self) -> DOUT_MODE_R {
        DOUT_MODE_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - The output data bit 15 is delayed by module clock LCD_CLK"]
    #[inline(always)]
    pub fn dout15_mode(&self) -> DOUT_MODE_R {
        DOUT_MODE_R::new(((self.bits >> 30) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LCD_DLY_MODE_CFG2")
            .field("dout0_mode", &self.dout0_mode())
            .field("dout1_mode", &self.dout1_mode())
            .field("dout2_mode", &self.dout2_mode())
            .field("dout3_mode", &self.dout3_mode())
            .field("dout4_mode", &self.dout4_mode())
            .field("dout5_mode", &self.dout5_mode())
            .field("dout6_mode", &self.dout6_mode())
            .field("dout7_mode", &self.dout7_mode())
            .field("dout8_mode", &self.dout8_mode())
            .field("dout9_mode", &self.dout9_mode())
            .field("dout10_mode", &self.dout10_mode())
            .field("dout11_mode", &self.dout11_mode())
            .field("dout12_mode", &self.dout12_mode())
            .field("dout13_mode", &self.dout13_mode())
            .field("dout14_mode", &self.dout14_mode())
            .field("dout15_mode", &self.dout15_mode())
            .finish()
    }
}
impl W {
    #[doc = "The output data bit (0-15) is delayed by module clock LCD_CLK"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `DOUT0_MODE` field.</div>"]
    #[inline(always)]
    #[must_use]
    pub fn dout_mode(&mut self, n: u8) -> DOUT_MODE_W<LCD_DLY_MODE_CFG2_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        DOUT_MODE_W::new(self, n * 2)
    }
    #[doc = "Bits 0:1 - The output data bit 0 is delayed by module clock LCD_CLK"]
    #[inline(always)]
    #[must_use]
    pub fn dout0_mode(&mut self) -> DOUT_MODE_W<LCD_DLY_MODE_CFG2_SPEC> {
        DOUT_MODE_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - The output data bit 1 is delayed by module clock LCD_CLK"]
    #[inline(always)]
    #[must_use]
    pub fn dout1_mode(&mut self) -> DOUT_MODE_W<LCD_DLY_MODE_CFG2_SPEC> {
        DOUT_MODE_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - The output data bit 2 is delayed by module clock LCD_CLK"]
    #[inline(always)]
    #[must_use]
    pub fn dout2_mode(&mut self) -> DOUT_MODE_W<LCD_DLY_MODE_CFG2_SPEC> {
        DOUT_MODE_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - The output data bit 3 is delayed by module clock LCD_CLK"]
    #[inline(always)]
    #[must_use]
    pub fn dout3_mode(&mut self) -> DOUT_MODE_W<LCD_DLY_MODE_CFG2_SPEC> {
        DOUT_MODE_W::new(self, 6)
    }
    #[doc = "Bits 8:9 - The output data bit 4 is delayed by module clock LCD_CLK"]
    #[inline(always)]
    #[must_use]
    pub fn dout4_mode(&mut self) -> DOUT_MODE_W<LCD_DLY_MODE_CFG2_SPEC> {
        DOUT_MODE_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - The output data bit 5 is delayed by module clock LCD_CLK"]
    #[inline(always)]
    #[must_use]
    pub fn dout5_mode(&mut self) -> DOUT_MODE_W<LCD_DLY_MODE_CFG2_SPEC> {
        DOUT_MODE_W::new(self, 10)
    }
    #[doc = "Bits 12:13 - The output data bit 6 is delayed by module clock LCD_CLK"]
    #[inline(always)]
    #[must_use]
    pub fn dout6_mode(&mut self) -> DOUT_MODE_W<LCD_DLY_MODE_CFG2_SPEC> {
        DOUT_MODE_W::new(self, 12)
    }
    #[doc = "Bits 14:15 - The output data bit 7 is delayed by module clock LCD_CLK"]
    #[inline(always)]
    #[must_use]
    pub fn dout7_mode(&mut self) -> DOUT_MODE_W<LCD_DLY_MODE_CFG2_SPEC> {
        DOUT_MODE_W::new(self, 14)
    }
    #[doc = "Bits 16:17 - The output data bit 8 is delayed by module clock LCD_CLK"]
    #[inline(always)]
    #[must_use]
    pub fn dout8_mode(&mut self) -> DOUT_MODE_W<LCD_DLY_MODE_CFG2_SPEC> {
        DOUT_MODE_W::new(self, 16)
    }
    #[doc = "Bits 18:19 - The output data bit 9 is delayed by module clock LCD_CLK"]
    #[inline(always)]
    #[must_use]
    pub fn dout9_mode(&mut self) -> DOUT_MODE_W<LCD_DLY_MODE_CFG2_SPEC> {
        DOUT_MODE_W::new(self, 18)
    }
    #[doc = "Bits 20:21 - The output data bit 10 is delayed by module clock LCD_CLK"]
    #[inline(always)]
    #[must_use]
    pub fn dout10_mode(&mut self) -> DOUT_MODE_W<LCD_DLY_MODE_CFG2_SPEC> {
        DOUT_MODE_W::new(self, 20)
    }
    #[doc = "Bits 22:23 - The output data bit 11 is delayed by module clock LCD_CLK"]
    #[inline(always)]
    #[must_use]
    pub fn dout11_mode(&mut self) -> DOUT_MODE_W<LCD_DLY_MODE_CFG2_SPEC> {
        DOUT_MODE_W::new(self, 22)
    }
    #[doc = "Bits 24:25 - The output data bit 12 is delayed by module clock LCD_CLK"]
    #[inline(always)]
    #[must_use]
    pub fn dout12_mode(&mut self) -> DOUT_MODE_W<LCD_DLY_MODE_CFG2_SPEC> {
        DOUT_MODE_W::new(self, 24)
    }
    #[doc = "Bits 26:27 - The output data bit 13 is delayed by module clock LCD_CLK"]
    #[inline(always)]
    #[must_use]
    pub fn dout13_mode(&mut self) -> DOUT_MODE_W<LCD_DLY_MODE_CFG2_SPEC> {
        DOUT_MODE_W::new(self, 26)
    }
    #[doc = "Bits 28:29 - The output data bit 14 is delayed by module clock LCD_CLK"]
    #[inline(always)]
    #[must_use]
    pub fn dout14_mode(&mut self) -> DOUT_MODE_W<LCD_DLY_MODE_CFG2_SPEC> {
        DOUT_MODE_W::new(self, 28)
    }
    #[doc = "Bits 30:31 - The output data bit 15 is delayed by module clock LCD_CLK"]
    #[inline(always)]
    #[must_use]
    pub fn dout15_mode(&mut self) -> DOUT_MODE_W<LCD_DLY_MODE_CFG2_SPEC> {
        DOUT_MODE_W::new(self, 30)
    }
}
#[doc = "LCD config register.\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_dly_mode_cfg2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_dly_mode_cfg2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LCD_DLY_MODE_CFG2_SPEC;
impl crate::RegisterSpec for LCD_DLY_MODE_CFG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcd_dly_mode_cfg2::R`](R) reader structure"]
impl crate::Readable for LCD_DLY_MODE_CFG2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lcd_dly_mode_cfg2::W`](W) writer structure"]
impl crate::Writable for LCD_DLY_MODE_CFG2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LCD_DLY_MODE_CFG2 to value 0"]
impl crate::Resettable for LCD_DLY_MODE_CFG2_SPEC {
    const RESET_VALUE: u32 = 0;
}
