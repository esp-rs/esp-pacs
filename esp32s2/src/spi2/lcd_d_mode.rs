#[doc = "Register `LCD_D_MODE` reader"]
pub type R = crate::R<LCD_D_MODE_SPEC>;
#[doc = "Register `LCD_D_MODE` writer"]
pub type W = crate::W<LCD_D_MODE_SPEC>;
#[doc = "Field `D_DQS_MODE` reader - the output spi_dqs is delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk. Can be configured in CONF state."]
pub type D_DQS_MODE_R = crate::FieldReader;
#[doc = "Field `D_DQS_MODE` writer - the output spi_dqs is delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk. Can be configured in CONF state."]
pub type D_DQS_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `D_CD_MODE` reader - the output spi_cd is delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk. Can be configured in CONF state."]
pub type D_CD_MODE_R = crate::FieldReader;
#[doc = "Field `D_CD_MODE` writer - the output spi_cd is delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk. Can be configured in CONF state."]
pub type D_CD_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `D_DE_MODE` reader - the output spi_de is delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk. Can be configured in CONF state."]
pub type D_DE_MODE_R = crate::FieldReader;
#[doc = "Field `D_DE_MODE` writer - the output spi_de is delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk. Can be configured in CONF state."]
pub type D_DE_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `D_HSYNC_MODE` reader - the output spi_hsync is delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk. Can be configured in CONF state."]
pub type D_HSYNC_MODE_R = crate::FieldReader;
#[doc = "Field `D_HSYNC_MODE` writer - the output spi_hsync is delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk. Can be configured in CONF state."]
pub type D_HSYNC_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `D_VSYNC_MODE` reader - the output spi_vsync is delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk. Can be configured in CONF state."]
pub type D_VSYNC_MODE_R = crate::FieldReader;
#[doc = "Field `D_VSYNC_MODE` writer - the output spi_vsync is delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk. Can be configured in CONF state."]
pub type D_VSYNC_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DE_IDLE_POL` reader - It is the idle value of spi_de."]
pub type DE_IDLE_POL_R = crate::BitReader;
#[doc = "Field `DE_IDLE_POL` writer - It is the idle value of spi_de."]
pub type DE_IDLE_POL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HS_BLANK_EN` reader - 1: The pulse of spi_hsync is out in vertical blanking lines in seg-trans or one trans. 0: spi_hsync pulse is valid only in active region lines in seg-trans."]
pub type HS_BLANK_EN_R = crate::BitReader;
#[doc = "Field `HS_BLANK_EN` writer - 1: The pulse of spi_hsync is out in vertical blanking lines in seg-trans or one trans. 0: spi_hsync pulse is valid only in active region lines in seg-trans."]
pub type HS_BLANK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - the output spi_dqs is delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk. Can be configured in CONF state."]
    #[inline(always)]
    pub fn d_dqs_mode(&self) -> D_DQS_MODE_R {
        D_DQS_MODE_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - the output spi_cd is delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk. Can be configured in CONF state."]
    #[inline(always)]
    pub fn d_cd_mode(&self) -> D_CD_MODE_R {
        D_CD_MODE_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - the output spi_de is delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk. Can be configured in CONF state."]
    #[inline(always)]
    pub fn d_de_mode(&self) -> D_DE_MODE_R {
        D_DE_MODE_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - the output spi_hsync is delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk. Can be configured in CONF state."]
    #[inline(always)]
    pub fn d_hsync_mode(&self) -> D_HSYNC_MODE_R {
        D_HSYNC_MODE_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14 - the output spi_vsync is delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk. Can be configured in CONF state."]
    #[inline(always)]
    pub fn d_vsync_mode(&self) -> D_VSYNC_MODE_R {
        D_VSYNC_MODE_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - It is the idle value of spi_de."]
    #[inline(always)]
    pub fn de_idle_pol(&self) -> DE_IDLE_POL_R {
        DE_IDLE_POL_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 1: The pulse of spi_hsync is out in vertical blanking lines in seg-trans or one trans. 0: spi_hsync pulse is valid only in active region lines in seg-trans."]
    #[inline(always)]
    pub fn hs_blank_en(&self) -> HS_BLANK_EN_R {
        HS_BLANK_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LCD_D_MODE")
            .field("d_dqs_mode", &self.d_dqs_mode())
            .field("d_cd_mode", &self.d_cd_mode())
            .field("d_de_mode", &self.d_de_mode())
            .field("d_hsync_mode", &self.d_hsync_mode())
            .field("d_vsync_mode", &self.d_vsync_mode())
            .field("de_idle_pol", &self.de_idle_pol())
            .field("hs_blank_en", &self.hs_blank_en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - the output spi_dqs is delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk. Can be configured in CONF state."]
    #[inline(always)]
    pub fn d_dqs_mode(&mut self) -> D_DQS_MODE_W<'_, LCD_D_MODE_SPEC> {
        D_DQS_MODE_W::new(self, 0)
    }
    #[doc = "Bits 3:5 - the output spi_cd is delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk. Can be configured in CONF state."]
    #[inline(always)]
    pub fn d_cd_mode(&mut self) -> D_CD_MODE_W<'_, LCD_D_MODE_SPEC> {
        D_CD_MODE_W::new(self, 3)
    }
    #[doc = "Bits 6:8 - the output spi_de is delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk. Can be configured in CONF state."]
    #[inline(always)]
    pub fn d_de_mode(&mut self) -> D_DE_MODE_W<'_, LCD_D_MODE_SPEC> {
        D_DE_MODE_W::new(self, 6)
    }
    #[doc = "Bits 9:11 - the output spi_hsync is delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk. Can be configured in CONF state."]
    #[inline(always)]
    pub fn d_hsync_mode(&mut self) -> D_HSYNC_MODE_W<'_, LCD_D_MODE_SPEC> {
        D_HSYNC_MODE_W::new(self, 9)
    }
    #[doc = "Bits 12:14 - the output spi_vsync is delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk. Can be configured in CONF state."]
    #[inline(always)]
    pub fn d_vsync_mode(&mut self) -> D_VSYNC_MODE_W<'_, LCD_D_MODE_SPEC> {
        D_VSYNC_MODE_W::new(self, 12)
    }
    #[doc = "Bit 15 - It is the idle value of spi_de."]
    #[inline(always)]
    pub fn de_idle_pol(&mut self) -> DE_IDLE_POL_W<'_, LCD_D_MODE_SPEC> {
        DE_IDLE_POL_W::new(self, 15)
    }
    #[doc = "Bit 16 - 1: The pulse of spi_hsync is out in vertical blanking lines in seg-trans or one trans. 0: spi_hsync pulse is valid only in active region lines in seg-trans."]
    #[inline(always)]
    pub fn hs_blank_en(&mut self) -> HS_BLANK_EN_W<'_, LCD_D_MODE_SPEC> {
        HS_BLANK_EN_W::new(self, 16)
    }
}
#[doc = "LCD delay number\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_d_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_d_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LCD_D_MODE_SPEC;
impl crate::RegisterSpec for LCD_D_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcd_d_mode::R`](R) reader structure"]
impl crate::Readable for LCD_D_MODE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lcd_d_mode::W`](W) writer structure"]
impl crate::Writable for LCD_D_MODE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCD_D_MODE to value 0"]
impl crate::Resettable for LCD_D_MODE_SPEC {}
