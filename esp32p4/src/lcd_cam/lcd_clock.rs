#[doc = "Register `LCD_CLOCK` reader"]
pub type R = crate::R<LCD_CLOCK_SPEC>;
#[doc = "Register `LCD_CLOCK` writer"]
pub type W = crate::W<LCD_CLOCK_SPEC>;
#[doc = "Field `LCD_CLKCNT_N` reader - f_LCD_PCLK = f_LCD_CLK / (reg_clkcnt_N + 1) when reg_clk_equ_sysclk is 0."]
pub type LCD_CLKCNT_N_R = crate::FieldReader;
#[doc = "Field `LCD_CLKCNT_N` writer - f_LCD_PCLK = f_LCD_CLK / (reg_clkcnt_N + 1) when reg_clk_equ_sysclk is 0."]
pub type LCD_CLKCNT_N_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `LCD_CLK_EQU_SYSCLK` reader - 1: f_LCD_PCLK = f_LCD_CLK. 0: f_LCD_PCLK = f_LCD_CLK / (reg_clkcnt_N + 1)."]
pub type LCD_CLK_EQU_SYSCLK_R = crate::BitReader;
#[doc = "Field `LCD_CLK_EQU_SYSCLK` writer - 1: f_LCD_PCLK = f_LCD_CLK. 0: f_LCD_PCLK = f_LCD_CLK / (reg_clkcnt_N + 1)."]
pub type LCD_CLK_EQU_SYSCLK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCD_CK_IDLE_EDGE` reader - 1: LCD_PCLK line is high when idle 0: LCD_PCLK line is low when idle."]
pub type LCD_CK_IDLE_EDGE_R = crate::BitReader;
#[doc = "Field `LCD_CK_IDLE_EDGE` writer - 1: LCD_PCLK line is high when idle 0: LCD_PCLK line is low when idle."]
pub type LCD_CK_IDLE_EDGE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCD_CK_OUT_EDGE` reader - 1: LCD_PCLK line is high in the first half data cycle. 0: LCD_PCLK line is low in the second half data cycle."]
pub type LCD_CK_OUT_EDGE_R = crate::BitReader;
#[doc = "Field `LCD_CK_OUT_EDGE` writer - 1: LCD_PCLK line is high in the first half data cycle. 0: LCD_PCLK line is low in the second half data cycle."]
pub type LCD_CK_OUT_EDGE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCD_CLKM_DIV_NUM` reader - Integral LCD clock divider value"]
pub type LCD_CLKM_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `LCD_CLKM_DIV_NUM` writer - Integral LCD clock divider value"]
pub type LCD_CLKM_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `LCD_CLKM_DIV_B` reader - Fractional clock divider numerator value"]
pub type LCD_CLKM_DIV_B_R = crate::FieldReader;
#[doc = "Field `LCD_CLKM_DIV_B` writer - Fractional clock divider numerator value"]
pub type LCD_CLKM_DIV_B_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `LCD_CLKM_DIV_A` reader - Fractional clock divider denominator value"]
pub type LCD_CLKM_DIV_A_R = crate::FieldReader;
#[doc = "Field `LCD_CLKM_DIV_A` writer - Fractional clock divider denominator value"]
pub type LCD_CLKM_DIV_A_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `LCD_CLK_SEL` reader - Select LCD module source clock. 0: no clock. 1: APLL. 2: CLK160. 3: no clock."]
pub type LCD_CLK_SEL_R = crate::FieldReader;
#[doc = "Field `LCD_CLK_SEL` writer - Select LCD module source clock. 0: no clock. 1: APLL. 2: CLK160. 3: no clock."]
pub type LCD_CLK_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CLK_EN` reader - Set this bit to enable clk gate"]
pub type CLK_EN_R = crate::BitReader;
#[doc = "Field `CLK_EN` writer - Set this bit to enable clk gate"]
pub type CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 8 - 1: LCD_PCLK line is high in the first half data cycle. 0: LCD_PCLK line is low in the second half data cycle."]
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
            .field("lcd_clkcnt_n", &self.lcd_clkcnt_n())
            .field("lcd_clk_equ_sysclk", &self.lcd_clk_equ_sysclk())
            .field("lcd_ck_idle_edge", &self.lcd_ck_idle_edge())
            .field("lcd_ck_out_edge", &self.lcd_ck_out_edge())
            .field("lcd_clkm_div_num", &self.lcd_clkm_div_num())
            .field("lcd_clkm_div_b", &self.lcd_clkm_div_b())
            .field("lcd_clkm_div_a", &self.lcd_clkm_div_a())
            .field("lcd_clk_sel", &self.lcd_clk_sel())
            .field("clk_en", &self.clk_en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:5 - f_LCD_PCLK = f_LCD_CLK / (reg_clkcnt_N + 1) when reg_clk_equ_sysclk is 0."]
    #[inline(always)]
    pub fn lcd_clkcnt_n(&mut self) -> LCD_CLKCNT_N_W<LCD_CLOCK_SPEC> {
        LCD_CLKCNT_N_W::new(self, 0)
    }
    #[doc = "Bit 6 - 1: f_LCD_PCLK = f_LCD_CLK. 0: f_LCD_PCLK = f_LCD_CLK / (reg_clkcnt_N + 1)."]
    #[inline(always)]
    pub fn lcd_clk_equ_sysclk(&mut self) -> LCD_CLK_EQU_SYSCLK_W<LCD_CLOCK_SPEC> {
        LCD_CLK_EQU_SYSCLK_W::new(self, 6)
    }
    #[doc = "Bit 7 - 1: LCD_PCLK line is high when idle 0: LCD_PCLK line is low when idle."]
    #[inline(always)]
    pub fn lcd_ck_idle_edge(&mut self) -> LCD_CK_IDLE_EDGE_W<LCD_CLOCK_SPEC> {
        LCD_CK_IDLE_EDGE_W::new(self, 7)
    }
    #[doc = "Bit 8 - 1: LCD_PCLK line is high in the first half data cycle. 0: LCD_PCLK line is low in the second half data cycle."]
    #[inline(always)]
    pub fn lcd_ck_out_edge(&mut self) -> LCD_CK_OUT_EDGE_W<LCD_CLOCK_SPEC> {
        LCD_CK_OUT_EDGE_W::new(self, 8)
    }
    #[doc = "Bits 9:16 - Integral LCD clock divider value"]
    #[inline(always)]
    pub fn lcd_clkm_div_num(&mut self) -> LCD_CLKM_DIV_NUM_W<LCD_CLOCK_SPEC> {
        LCD_CLKM_DIV_NUM_W::new(self, 9)
    }
    #[doc = "Bits 17:22 - Fractional clock divider numerator value"]
    #[inline(always)]
    pub fn lcd_clkm_div_b(&mut self) -> LCD_CLKM_DIV_B_W<LCD_CLOCK_SPEC> {
        LCD_CLKM_DIV_B_W::new(self, 17)
    }
    #[doc = "Bits 23:28 - Fractional clock divider denominator value"]
    #[inline(always)]
    pub fn lcd_clkm_div_a(&mut self) -> LCD_CLKM_DIV_A_W<LCD_CLOCK_SPEC> {
        LCD_CLKM_DIV_A_W::new(self, 23)
    }
    #[doc = "Bits 29:30 - Select LCD module source clock. 0: no clock. 1: APLL. 2: CLK160. 3: no clock."]
    #[inline(always)]
    pub fn lcd_clk_sel(&mut self) -> LCD_CLK_SEL_W<LCD_CLOCK_SPEC> {
        LCD_CLK_SEL_W::new(self, 29)
    }
    #[doc = "Bit 31 - Set this bit to enable clk gate"]
    #[inline(always)]
    pub fn clk_en(&mut self) -> CLK_EN_W<LCD_CLOCK_SPEC> {
        CLK_EN_W::new(self, 31)
    }
}
#[doc = "LCD clock config register.\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_clock::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_clock::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LCD_CLOCK_SPEC;
impl crate::RegisterSpec for LCD_CLOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcd_clock::R`](R) reader structure"]
impl crate::Readable for LCD_CLOCK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lcd_clock::W`](W) writer structure"]
impl crate::Writable for LCD_CLOCK_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LCD_CLOCK to value 0x0843"]
impl crate::Resettable for LCD_CLOCK_SPEC {
    const RESET_VALUE: u32 = 0x0843;
}
