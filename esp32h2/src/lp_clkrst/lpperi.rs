#[doc = "Register `LPPERI` reader"]
pub type R = crate::R<LPPERI_SPEC>;
#[doc = "Register `LPPERI` writer"]
pub type W = crate::W<LPPERI_SPEC>;
#[doc = "Field `LP_BLETIMER_DIV_NUM` reader - need_des"]
pub type LP_BLETIMER_DIV_NUM_R = crate::FieldReader<u16>;
#[doc = "Field `LP_BLETIMER_DIV_NUM` writer - need_des"]
pub type LP_BLETIMER_DIV_NUM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, u16>;
#[doc = "Field `LP_BLETIMER_32K_SEL` reader - need_des"]
pub type LP_BLETIMER_32K_SEL_R = crate::FieldReader;
#[doc = "Field `LP_BLETIMER_32K_SEL` writer - need_des"]
pub type LP_BLETIMER_32K_SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `LP_SEL_OSC_SLOW` reader - need_des"]
pub type LP_SEL_OSC_SLOW_R = crate::BitReader;
#[doc = "Field `LP_SEL_OSC_SLOW` writer - need_des"]
pub type LP_SEL_OSC_SLOW_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LP_SEL_OSC_FAST` reader - need_des"]
pub type LP_SEL_OSC_FAST_R = crate::BitReader;
#[doc = "Field `LP_SEL_OSC_FAST` writer - need_des"]
pub type LP_SEL_OSC_FAST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LP_SEL_XTAL` reader - need_des"]
pub type LP_SEL_XTAL_R = crate::BitReader;
#[doc = "Field `LP_SEL_XTAL` writer - need_des"]
pub type LP_SEL_XTAL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LP_SEL_XTAL32K` reader - need_des"]
pub type LP_SEL_XTAL32K_R = crate::BitReader;
#[doc = "Field `LP_SEL_XTAL32K` writer - need_des"]
pub type LP_SEL_XTAL32K_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LP_I2C_CLK_SEL` reader - need_des"]
pub type LP_I2C_CLK_SEL_R = crate::BitReader;
#[doc = "Field `LP_I2C_CLK_SEL` writer - need_des"]
pub type LP_I2C_CLK_SEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LP_UART_CLK_SEL` reader - need_des"]
pub type LP_UART_CLK_SEL_R = crate::BitReader;
#[doc = "Field `LP_UART_CLK_SEL` writer - need_des"]
pub type LP_UART_CLK_SEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 12:23 - need_des"]
    #[inline(always)]
    pub fn lp_bletimer_div_num(&self) -> LP_BLETIMER_DIV_NUM_R {
        LP_BLETIMER_DIV_NUM_R::new(((self.bits >> 12) & 0x0fff) as u16)
    }
    #[doc = "Bits 24:25 - need_des"]
    #[inline(always)]
    pub fn lp_bletimer_32k_sel(&self) -> LP_BLETIMER_32K_SEL_R {
        LP_BLETIMER_32K_SEL_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26 - need_des"]
    #[inline(always)]
    pub fn lp_sel_osc_slow(&self) -> LP_SEL_OSC_SLOW_R {
        LP_SEL_OSC_SLOW_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    pub fn lp_sel_osc_fast(&self) -> LP_SEL_OSC_FAST_R {
        LP_SEL_OSC_FAST_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    pub fn lp_sel_xtal(&self) -> LP_SEL_XTAL_R {
        LP_SEL_XTAL_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    pub fn lp_sel_xtal32k(&self) -> LP_SEL_XTAL32K_R {
        LP_SEL_XTAL32K_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn lp_i2c_clk_sel(&self) -> LP_I2C_CLK_SEL_R {
        LP_I2C_CLK_SEL_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn lp_uart_clk_sel(&self) -> LP_UART_CLK_SEL_R {
        LP_UART_CLK_SEL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPPERI")
            .field(
                "lp_bletimer_div_num",
                &format_args!("{}", self.lp_bletimer_div_num().bits()),
            )
            .field(
                "lp_bletimer_32k_sel",
                &format_args!("{}", self.lp_bletimer_32k_sel().bits()),
            )
            .field(
                "lp_sel_osc_slow",
                &format_args!("{}", self.lp_sel_osc_slow().bit()),
            )
            .field(
                "lp_sel_osc_fast",
                &format_args!("{}", self.lp_sel_osc_fast().bit()),
            )
            .field("lp_sel_xtal", &format_args!("{}", self.lp_sel_xtal().bit()))
            .field(
                "lp_sel_xtal32k",
                &format_args!("{}", self.lp_sel_xtal32k().bit()),
            )
            .field(
                "lp_i2c_clk_sel",
                &format_args!("{}", self.lp_i2c_clk_sel().bit()),
            )
            .field(
                "lp_uart_clk_sel",
                &format_args!("{}", self.lp_uart_clk_sel().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LPPERI_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 12:23 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_bletimer_div_num(&mut self) -> LP_BLETIMER_DIV_NUM_W<LPPERI_SPEC, 12> {
        LP_BLETIMER_DIV_NUM_W::new(self)
    }
    #[doc = "Bits 24:25 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_bletimer_32k_sel(&mut self) -> LP_BLETIMER_32K_SEL_W<LPPERI_SPEC, 24> {
        LP_BLETIMER_32K_SEL_W::new(self)
    }
    #[doc = "Bit 26 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_sel_osc_slow(&mut self) -> LP_SEL_OSC_SLOW_W<LPPERI_SPEC, 26> {
        LP_SEL_OSC_SLOW_W::new(self)
    }
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_sel_osc_fast(&mut self) -> LP_SEL_OSC_FAST_W<LPPERI_SPEC, 27> {
        LP_SEL_OSC_FAST_W::new(self)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_sel_xtal(&mut self) -> LP_SEL_XTAL_W<LPPERI_SPEC, 28> {
        LP_SEL_XTAL_W::new(self)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_sel_xtal32k(&mut self) -> LP_SEL_XTAL32K_W<LPPERI_SPEC, 29> {
        LP_SEL_XTAL32K_W::new(self)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_i2c_clk_sel(&mut self) -> LP_I2C_CLK_SEL_W<LPPERI_SPEC, 30> {
        LP_I2C_CLK_SEL_W::new(self)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_uart_clk_sel(&mut self) -> LP_UART_CLK_SEL_W<LPPERI_SPEC, 31> {
        LP_UART_CLK_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpperi::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpperi::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LPPERI_SPEC;
impl crate::RegisterSpec for LPPERI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lpperi::R`](R) reader structure"]
impl crate::Readable for LPPERI_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lpperi::W`](W) writer structure"]
impl crate::Writable for LPPERI_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LPPERI to value 0x2000_0000"]
impl crate::Resettable for LPPERI_SPEC {
    const RESET_VALUE: Self::Ux = 0x2000_0000;
}
