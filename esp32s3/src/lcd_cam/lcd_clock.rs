#[doc = "Register `LCD_CLOCK` reader"]
pub struct R(crate::R<LCD_CLOCK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCD_CLOCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCD_CLOCK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCD_CLOCK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LCD_CLOCK` writer"]
pub struct W(crate::W<LCD_CLOCK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCD_CLOCK_SPEC>;
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
impl From<crate::W<LCD_CLOCK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LCD_CLOCK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LCD_CLKCNT_N` reader - f_LCD_PCLK = f_LCD_CLK / (reg_clkcnt_N + 1) when reg_clk_equ_sysclk is 0."]
pub type LCD_CLKCNT_N_R = crate::FieldReader;
#[doc = "Field `LCD_CLKCNT_N` writer - f_LCD_PCLK = f_LCD_CLK / (reg_clkcnt_N + 1) when reg_clk_equ_sysclk is 0."]
pub type LCD_CLKCNT_N_W<'a, const O: u8> = crate::FieldWriter<'a, LCD_CLOCK_SPEC, 6, O>;
#[doc = "Field `LCD_CLK_EQU_SYSCLK` reader - 1: f_LCD_PCLK = f_LCD_CLK. 0: f_LCD_PCLK = f_LCD_CLK / (reg_clkcnt_N + 1)."]
pub type LCD_CLK_EQU_SYSCLK_R = crate::BitReader;
#[doc = "Field `LCD_CLK_EQU_SYSCLK` writer - 1: f_LCD_PCLK = f_LCD_CLK. 0: f_LCD_PCLK = f_LCD_CLK / (reg_clkcnt_N + 1)."]
pub type LCD_CLK_EQU_SYSCLK_W<'a, const O: u8> = crate::BitWriter<'a, LCD_CLOCK_SPEC, O>;
#[doc = "Field `LCD_CK_IDLE_EDGE` reader - 1: LCD_PCLK line is high when idle 0: LCD_PCLK line is low when idle."]
pub type LCD_CK_IDLE_EDGE_R = crate::BitReader;
#[doc = "Field `LCD_CK_IDLE_EDGE` writer - 1: LCD_PCLK line is high when idle 0: LCD_PCLK line is low when idle."]
pub type LCD_CK_IDLE_EDGE_W<'a, const O: u8> = crate::BitWriter<'a, LCD_CLOCK_SPEC, O>;
#[doc = "Field `LCD_CK_OUT_EDGE` reader - 1: LCD_PCLK high in first half clock cycle. 0: LCD_PCLK low in first half clock cycle."]
pub type LCD_CK_OUT_EDGE_R = crate::BitReader;
#[doc = "Field `LCD_CK_OUT_EDGE` writer - 1: LCD_PCLK high in first half clock cycle. 0: LCD_PCLK low in first half clock cycle."]
pub type LCD_CK_OUT_EDGE_W<'a, const O: u8> = crate::BitWriter<'a, LCD_CLOCK_SPEC, O>;
#[doc = "Field `LCD_CLKM_DIV_NUM` reader - Integral LCD clock divider value"]
pub type LCD_CLKM_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `LCD_CLKM_DIV_NUM` writer - Integral LCD clock divider value"]
pub type LCD_CLKM_DIV_NUM_W<'a, const O: u8> = crate::FieldWriter<'a, LCD_CLOCK_SPEC, 8, O>;
#[doc = "Field `LCD_CLKM_DIV_B` reader - Fractional clock divider numerator value"]
pub type LCD_CLKM_DIV_B_R = crate::FieldReader;
#[doc = "Field `LCD_CLKM_DIV_B` writer - Fractional clock divider numerator value"]
pub type LCD_CLKM_DIV_B_W<'a, const O: u8> = crate::FieldWriter<'a, LCD_CLOCK_SPEC, 6, O>;
#[doc = "Field `LCD_CLKM_DIV_A` reader - Fractional clock divider denominator value"]
pub type LCD_CLKM_DIV_A_R = crate::FieldReader;
#[doc = "Field `LCD_CLKM_DIV_A` writer - Fractional clock divider denominator value"]
pub type LCD_CLKM_DIV_A_W<'a, const O: u8> = crate::FieldWriter<'a, LCD_CLOCK_SPEC, 6, O>;
#[doc = "Field `LCD_CLK_SEL` reader - Select LCD module source clock. 0: no clock. 1: APLL. 2: CLK160. 3: no clock."]
pub type LCD_CLK_SEL_R = crate::FieldReader;
#[doc = "Field `LCD_CLK_SEL` writer - Select LCD module source clock. 0: no clock. 1: APLL. 2: CLK160. 3: no clock."]
pub type LCD_CLK_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, LCD_CLOCK_SPEC, 2, O>;
#[doc = "Field `CLK_EN` reader - Set this bit to enable clk gate"]
pub type CLK_EN_R = crate::BitReader;
#[doc = "Field `CLK_EN` writer - Set this bit to enable clk gate"]
pub type CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, LCD_CLOCK_SPEC, O>;
impl R {
    #[doc = "Bits 0:5 - f_LCD_PCLK = f_LCD_CLK / (reg_clkcnt_N + 1) when reg_clk_equ_sysclk is 0."]
    #[inline(always)]
    pub fn lcd_clkcnt_n(&self) -> LCD_CLKCNT_N_R {
        LCD_CLKCNT_N_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - 1: f_LCD_PCLK = f_LCD_CLK. 0: f_LCD_PCLK = f_LCD_CLK / (reg_clkcnt_N + 1)."]
    #[inline(always)]
    pub fn lcd_clk_equ_sysclk(&self) -> LCD_CLK_EQU_SYSCLK_R {
        LCD_CLK_EQU_SYSCLK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 1: LCD_PCLK line is high when idle 0: LCD_PCLK line is low when idle."]
    #[inline(always)]
    pub fn lcd_ck_idle_edge(&self) -> LCD_CK_IDLE_EDGE_R {
        LCD_CK_IDLE_EDGE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 1: LCD_PCLK high in first half clock cycle. 0: LCD_PCLK low in first half clock cycle."]
    #[inline(always)]
    pub fn lcd_ck_out_edge(&self) -> LCD_CK_OUT_EDGE_R {
        LCD_CK_OUT_EDGE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:16 - Integral LCD clock divider value"]
    #[inline(always)]
    pub fn lcd_clkm_div_num(&self) -> LCD_CLKM_DIV_NUM_R {
        LCD_CLKM_DIV_NUM_R::new(((self.bits >> 9) & 0xff) as u8)
    }
    #[doc = "Bits 17:22 - Fractional clock divider numerator value"]
    #[inline(always)]
    pub fn lcd_clkm_div_b(&self) -> LCD_CLKM_DIV_B_R {
        LCD_CLKM_DIV_B_R::new(((self.bits >> 17) & 0x3f) as u8)
    }
    #[doc = "Bits 23:28 - Fractional clock divider denominator value"]
    #[inline(always)]
    pub fn lcd_clkm_div_a(&self) -> LCD_CLKM_DIV_A_R {
        LCD_CLKM_DIV_A_R::new(((self.bits >> 23) & 0x3f) as u8)
    }
    #[doc = "Bits 29:30 - Select LCD module source clock. 0: no clock. 1: APLL. 2: CLK160. 3: no clock."]
    #[inline(always)]
    pub fn lcd_clk_sel(&self) -> LCD_CLK_SEL_R {
        LCD_CLK_SEL_R::new(((self.bits >> 29) & 3) as u8)
    }
    #[doc = "Bit 31 - Set this bit to enable clk gate"]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LCD_CLOCK")
            .field(
                "lcd_clkcnt_n",
                &format_args!("{}", self.lcd_clkcnt_n().bits()),
            )
            .field(
                "lcd_clk_equ_sysclk",
                &format_args!("{}", self.lcd_clk_equ_sysclk().bit()),
            )
            .field(
                "lcd_ck_idle_edge",
                &format_args!("{}", self.lcd_ck_idle_edge().bit()),
            )
            .field(
                "lcd_ck_out_edge",
                &format_args!("{}", self.lcd_ck_out_edge().bit()),
            )
            .field(
                "lcd_clkm_div_num",
                &format_args!("{}", self.lcd_clkm_div_num().bits()),
            )
            .field(
                "lcd_clkm_div_b",
                &format_args!("{}", self.lcd_clkm_div_b().bits()),
            )
            .field(
                "lcd_clkm_div_a",
                &format_args!("{}", self.lcd_clkm_div_a().bits()),
            )
            .field(
                "lcd_clk_sel",
                &format_args!("{}", self.lcd_clk_sel().bits()),
            )
            .field("clk_en", &format_args!("{}", self.clk_en().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LCD_CLOCK_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:5 - f_LCD_PCLK = f_LCD_CLK / (reg_clkcnt_N + 1) when reg_clk_equ_sysclk is 0."]
    #[inline(always)]
    #[must_use]
    pub fn lcd_clkcnt_n(&mut self) -> LCD_CLKCNT_N_W<0> {
        LCD_CLKCNT_N_W::new(self)
    }
    #[doc = "Bit 6 - 1: f_LCD_PCLK = f_LCD_CLK. 0: f_LCD_PCLK = f_LCD_CLK / (reg_clkcnt_N + 1)."]
    #[inline(always)]
    #[must_use]
    pub fn lcd_clk_equ_sysclk(&mut self) -> LCD_CLK_EQU_SYSCLK_W<6> {
        LCD_CLK_EQU_SYSCLK_W::new(self)
    }
    #[doc = "Bit 7 - 1: LCD_PCLK line is high when idle 0: LCD_PCLK line is low when idle."]
    #[inline(always)]
    #[must_use]
    pub fn lcd_ck_idle_edge(&mut self) -> LCD_CK_IDLE_EDGE_W<7> {
        LCD_CK_IDLE_EDGE_W::new(self)
    }
    #[doc = "Bit 8 - 1: LCD_PCLK high in first half clock cycle. 0: LCD_PCLK low in first half clock cycle."]
    #[inline(always)]
    #[must_use]
    pub fn lcd_ck_out_edge(&mut self) -> LCD_CK_OUT_EDGE_W<8> {
        LCD_CK_OUT_EDGE_W::new(self)
    }
    #[doc = "Bits 9:16 - Integral LCD clock divider value"]
    #[inline(always)]
    #[must_use]
    pub fn lcd_clkm_div_num(&mut self) -> LCD_CLKM_DIV_NUM_W<9> {
        LCD_CLKM_DIV_NUM_W::new(self)
    }
    #[doc = "Bits 17:22 - Fractional clock divider numerator value"]
    #[inline(always)]
    #[must_use]
    pub fn lcd_clkm_div_b(&mut self) -> LCD_CLKM_DIV_B_W<17> {
        LCD_CLKM_DIV_B_W::new(self)
    }
    #[doc = "Bits 23:28 - Fractional clock divider denominator value"]
    #[inline(always)]
    #[must_use]
    pub fn lcd_clkm_div_a(&mut self) -> LCD_CLKM_DIV_A_W<23> {
        LCD_CLKM_DIV_A_W::new(self)
    }
    #[doc = "Bits 29:30 - Select LCD module source clock. 0: no clock. 1: APLL. 2: CLK160. 3: no clock."]
    #[inline(always)]
    #[must_use]
    pub fn lcd_clk_sel(&mut self) -> LCD_CLK_SEL_W<29> {
        LCD_CLK_SEL_W::new(self)
    }
    #[doc = "Bit 31 - Set this bit to enable clk gate"]
    #[inline(always)]
    #[must_use]
    pub fn clk_en(&mut self) -> CLK_EN_W<31> {
        CLK_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LCD clock register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcd_clock](index.html) module"]
pub struct LCD_CLOCK_SPEC;
impl crate::RegisterSpec for LCD_CLOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lcd_clock::R](R) reader structure"]
impl crate::Readable for LCD_CLOCK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lcd_clock::W](W) writer structure"]
impl crate::Writable for LCD_CLOCK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LCD_CLOCK to value 0x0843"]
impl crate::Resettable for LCD_CLOCK_SPEC {
    const RESET_VALUE: Self::Ux = 0x0843;
}
