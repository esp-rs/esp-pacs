///Register `LPPERI` reader
pub type R = crate::R<LPPERI_SPEC>;
///Register `LPPERI` writer
pub type W = crate::W<LPPERI_SPEC>;
///Field `LP_BLETIMER_DIV_NUM` reader - need_des
pub type LP_BLETIMER_DIV_NUM_R = crate::FieldReader<u16>;
///Field `LP_BLETIMER_DIV_NUM` writer - need_des
pub type LP_BLETIMER_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
///Field `LP_BLETIMER_32K_SEL` reader - need_des
pub type LP_BLETIMER_32K_SEL_R = crate::FieldReader;
///Field `LP_BLETIMER_32K_SEL` writer - need_des
pub type LP_BLETIMER_32K_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `LP_SEL_OSC_SLOW` reader - need_des
pub type LP_SEL_OSC_SLOW_R = crate::BitReader;
///Field `LP_SEL_OSC_SLOW` writer - need_des
pub type LP_SEL_OSC_SLOW_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LP_SEL_OSC_FAST` reader - need_des
pub type LP_SEL_OSC_FAST_R = crate::BitReader;
///Field `LP_SEL_OSC_FAST` writer - need_des
pub type LP_SEL_OSC_FAST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LP_SEL_XTAL` reader - need_des
pub type LP_SEL_XTAL_R = crate::BitReader;
///Field `LP_SEL_XTAL` writer - need_des
pub type LP_SEL_XTAL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LP_SEL_XTAL32K` reader - need_des
pub type LP_SEL_XTAL32K_R = crate::BitReader;
///Field `LP_SEL_XTAL32K` writer - need_des
pub type LP_SEL_XTAL32K_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LP_I2C_CLK_SEL` reader - need_des
pub type LP_I2C_CLK_SEL_R = crate::BitReader;
///Field `LP_I2C_CLK_SEL` writer - need_des
pub type LP_I2C_CLK_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LP_UART_CLK_SEL` reader - need_des
pub type LP_UART_CLK_SEL_R = crate::BitReader;
///Field `LP_UART_CLK_SEL` writer - need_des
pub type LP_UART_CLK_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 12:23 - need_des
    #[inline(always)]
    pub fn lp_bletimer_div_num(&self) -> LP_BLETIMER_DIV_NUM_R {
        LP_BLETIMER_DIV_NUM_R::new(((self.bits >> 12) & 0x0fff) as u16)
    }
    ///Bits 24:25 - need_des
    #[inline(always)]
    pub fn lp_bletimer_32k_sel(&self) -> LP_BLETIMER_32K_SEL_R {
        LP_BLETIMER_32K_SEL_R::new(((self.bits >> 24) & 3) as u8)
    }
    ///Bit 26 - need_des
    #[inline(always)]
    pub fn lp_sel_osc_slow(&self) -> LP_SEL_OSC_SLOW_R {
        LP_SEL_OSC_SLOW_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - need_des
    #[inline(always)]
    pub fn lp_sel_osc_fast(&self) -> LP_SEL_OSC_FAST_R {
        LP_SEL_OSC_FAST_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - need_des
    #[inline(always)]
    pub fn lp_sel_xtal(&self) -> LP_SEL_XTAL_R {
        LP_SEL_XTAL_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - need_des
    #[inline(always)]
    pub fn lp_sel_xtal32k(&self) -> LP_SEL_XTAL32K_R {
        LP_SEL_XTAL32K_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - need_des
    #[inline(always)]
    pub fn lp_i2c_clk_sel(&self) -> LP_I2C_CLK_SEL_R {
        LP_I2C_CLK_SEL_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - need_des
    #[inline(always)]
    pub fn lp_uart_clk_sel(&self) -> LP_UART_CLK_SEL_R {
        LP_UART_CLK_SEL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPPERI")
            .field("lp_bletimer_div_num", &self.lp_bletimer_div_num())
            .field("lp_bletimer_32k_sel", &self.lp_bletimer_32k_sel())
            .field("lp_sel_osc_slow", &self.lp_sel_osc_slow())
            .field("lp_sel_osc_fast", &self.lp_sel_osc_fast())
            .field("lp_sel_xtal", &self.lp_sel_xtal())
            .field("lp_sel_xtal32k", &self.lp_sel_xtal32k())
            .field("lp_i2c_clk_sel", &self.lp_i2c_clk_sel())
            .field("lp_uart_clk_sel", &self.lp_uart_clk_sel())
            .finish()
    }
}
impl W {
    ///Bits 12:23 - need_des
    #[inline(always)]
    #[must_use]
    pub fn lp_bletimer_div_num(&mut self) -> LP_BLETIMER_DIV_NUM_W<LPPERI_SPEC> {
        LP_BLETIMER_DIV_NUM_W::new(self, 12)
    }
    ///Bits 24:25 - need_des
    #[inline(always)]
    #[must_use]
    pub fn lp_bletimer_32k_sel(&mut self) -> LP_BLETIMER_32K_SEL_W<LPPERI_SPEC> {
        LP_BLETIMER_32K_SEL_W::new(self, 24)
    }
    ///Bit 26 - need_des
    #[inline(always)]
    #[must_use]
    pub fn lp_sel_osc_slow(&mut self) -> LP_SEL_OSC_SLOW_W<LPPERI_SPEC> {
        LP_SEL_OSC_SLOW_W::new(self, 26)
    }
    ///Bit 27 - need_des
    #[inline(always)]
    #[must_use]
    pub fn lp_sel_osc_fast(&mut self) -> LP_SEL_OSC_FAST_W<LPPERI_SPEC> {
        LP_SEL_OSC_FAST_W::new(self, 27)
    }
    ///Bit 28 - need_des
    #[inline(always)]
    #[must_use]
    pub fn lp_sel_xtal(&mut self) -> LP_SEL_XTAL_W<LPPERI_SPEC> {
        LP_SEL_XTAL_W::new(self, 28)
    }
    ///Bit 29 - need_des
    #[inline(always)]
    #[must_use]
    pub fn lp_sel_xtal32k(&mut self) -> LP_SEL_XTAL32K_W<LPPERI_SPEC> {
        LP_SEL_XTAL32K_W::new(self, 29)
    }
    ///Bit 30 - need_des
    #[inline(always)]
    #[must_use]
    pub fn lp_i2c_clk_sel(&mut self) -> LP_I2C_CLK_SEL_W<LPPERI_SPEC> {
        LP_I2C_CLK_SEL_W::new(self, 30)
    }
    ///Bit 31 - need_des
    #[inline(always)]
    #[must_use]
    pub fn lp_uart_clk_sel(&mut self) -> LP_UART_CLK_SEL_W<LPPERI_SPEC> {
        LP_UART_CLK_SEL_W::new(self, 31)
    }
}
/**need_des

You can [`read`](crate::generic::Reg::read) this register and get [`lpperi::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpperi::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct LPPERI_SPEC;
impl crate::RegisterSpec for LPPERI_SPEC {
    type Ux = u32;
}
///`read()` method returns [`lpperi::R`](R) reader structure
impl crate::Readable for LPPERI_SPEC {}
///`write(|w| ..)` method takes [`lpperi::W`](W) writer structure
impl crate::Writable for LPPERI_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets LPPERI to value 0x2000_0000
impl crate::Resettable for LPPERI_SPEC {
    const RESET_VALUE: u32 = 0x2000_0000;
}
