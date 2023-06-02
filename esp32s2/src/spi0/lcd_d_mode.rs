#[doc = "Register `LCD_D_MODE` reader"]
pub struct R(crate::R<LCD_D_MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCD_D_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCD_D_MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCD_D_MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LCD_D_MODE` writer"]
pub struct W(crate::W<LCD_D_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCD_D_MODE_SPEC>;
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
impl From<crate::W<LCD_D_MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LCD_D_MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `D_DQS_MODE` reader - the output spi_dqs is delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk. Can be configured in CONF state."]
pub type D_DQS_MODE_R = crate::FieldReader;
#[doc = "Field `D_DQS_MODE` writer - the output spi_dqs is delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk. Can be configured in CONF state."]
pub type D_DQS_MODE_W<'a, const O: u8> = crate::FieldWriter<'a, LCD_D_MODE_SPEC, 3, O>;
#[doc = "Field `D_CD_MODE` reader - the output spi_cd is delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk. Can be configured in CONF state."]
pub type D_CD_MODE_R = crate::FieldReader;
#[doc = "Field `D_CD_MODE` writer - the output spi_cd is delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk. Can be configured in CONF state."]
pub type D_CD_MODE_W<'a, const O: u8> = crate::FieldWriter<'a, LCD_D_MODE_SPEC, 3, O>;
#[doc = "Field `D_DE_MODE` reader - the output spi_de is delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk. Can be configured in CONF state."]
pub type D_DE_MODE_R = crate::FieldReader;
#[doc = "Field `D_DE_MODE` writer - the output spi_de is delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk. Can be configured in CONF state."]
pub type D_DE_MODE_W<'a, const O: u8> = crate::FieldWriter<'a, LCD_D_MODE_SPEC, 3, O>;
#[doc = "Field `D_HSYNC_MODE` reader - the output spi_hsync is delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk. Can be configured in CONF state."]
pub type D_HSYNC_MODE_R = crate::FieldReader;
#[doc = "Field `D_HSYNC_MODE` writer - the output spi_hsync is delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk. Can be configured in CONF state."]
pub type D_HSYNC_MODE_W<'a, const O: u8> = crate::FieldWriter<'a, LCD_D_MODE_SPEC, 3, O>;
#[doc = "Field `D_VSYNC_MODE` reader - the output spi_vsync is delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk. Can be configured in CONF state."]
pub type D_VSYNC_MODE_R = crate::FieldReader;
#[doc = "Field `D_VSYNC_MODE` writer - the output spi_vsync is delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk. Can be configured in CONF state."]
pub type D_VSYNC_MODE_W<'a, const O: u8> = crate::FieldWriter<'a, LCD_D_MODE_SPEC, 3, O>;
#[doc = "Field `DE_IDLE_POL` reader - It is the idle value of spi_de."]
pub type DE_IDLE_POL_R = crate::BitReader;
#[doc = "Field `DE_IDLE_POL` writer - It is the idle value of spi_de."]
pub type DE_IDLE_POL_W<'a, const O: u8> = crate::BitWriter<'a, LCD_D_MODE_SPEC, O>;
#[doc = "Field `HS_BLANK_EN` reader - 1: The pulse of spi_hsync is out in vertical blanking lines in seg-trans or one trans. 0: spi_hsync pulse is valid only in active region lines in seg-trans."]
pub type HS_BLANK_EN_R = crate::BitReader;
#[doc = "Field `HS_BLANK_EN` writer - 1: The pulse of spi_hsync is out in vertical blanking lines in seg-trans or one trans. 0: spi_hsync pulse is valid only in active region lines in seg-trans."]
pub type HS_BLANK_EN_W<'a, const O: u8> = crate::BitWriter<'a, LCD_D_MODE_SPEC, O>;
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
            .field("d_dqs_mode", &format_args!("{}", self.d_dqs_mode().bits()))
            .field("d_cd_mode", &format_args!("{}", self.d_cd_mode().bits()))
            .field("d_de_mode", &format_args!("{}", self.d_de_mode().bits()))
            .field(
                "d_hsync_mode",
                &format_args!("{}", self.d_hsync_mode().bits()),
            )
            .field(
                "d_vsync_mode",
                &format_args!("{}", self.d_vsync_mode().bits()),
            )
            .field("de_idle_pol", &format_args!("{}", self.de_idle_pol().bit()))
            .field("hs_blank_en", &format_args!("{}", self.hs_blank_en().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LCD_D_MODE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:2 - the output spi_dqs is delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn d_dqs_mode(&mut self) -> D_DQS_MODE_W<0> {
        D_DQS_MODE_W::new(self)
    }
    #[doc = "Bits 3:5 - the output spi_cd is delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn d_cd_mode(&mut self) -> D_CD_MODE_W<3> {
        D_CD_MODE_W::new(self)
    }
    #[doc = "Bits 6:8 - the output spi_de is delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn d_de_mode(&mut self) -> D_DE_MODE_W<6> {
        D_DE_MODE_W::new(self)
    }
    #[doc = "Bits 9:11 - the output spi_hsync is delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn d_hsync_mode(&mut self) -> D_HSYNC_MODE_W<9> {
        D_HSYNC_MODE_W::new(self)
    }
    #[doc = "Bits 12:14 - the output spi_vsync is delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn d_vsync_mode(&mut self) -> D_VSYNC_MODE_W<12> {
        D_VSYNC_MODE_W::new(self)
    }
    #[doc = "Bit 15 - It is the idle value of spi_de."]
    #[inline(always)]
    #[must_use]
    pub fn de_idle_pol(&mut self) -> DE_IDLE_POL_W<15> {
        DE_IDLE_POL_W::new(self)
    }
    #[doc = "Bit 16 - 1: The pulse of spi_hsync is out in vertical blanking lines in seg-trans or one trans. 0: spi_hsync pulse is valid only in active region lines in seg-trans."]
    #[inline(always)]
    #[must_use]
    pub fn hs_blank_en(&mut self) -> HS_BLANK_EN_W<16> {
        HS_BLANK_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LCD delay number\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcd_d_mode](index.html) module"]
pub struct LCD_D_MODE_SPEC;
impl crate::RegisterSpec for LCD_D_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lcd_d_mode::R](R) reader structure"]
impl crate::Readable for LCD_D_MODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lcd_d_mode::W](W) writer structure"]
impl crate::Writable for LCD_D_MODE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LCD_D_MODE to value 0"]
impl crate::Resettable for LCD_D_MODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
