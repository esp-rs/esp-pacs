#[doc = "Register `LPPERI` reader"]
pub type R = crate::R<LPPERI_SPEC>;
#[doc = "Register `LPPERI` writer"]
pub type W = crate::W<LPPERI_SPEC>;
#[doc = "Field `LP_I2C_CLK_SEL` reader - need_des"]
pub type LP_I2C_CLK_SEL_R = crate::BitReader;
#[doc = "Field `LP_I2C_CLK_SEL` writer - need_des"]
pub type LP_I2C_CLK_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_UART_CLK_SEL` reader - need_des"]
pub type LP_UART_CLK_SEL_R = crate::BitReader;
#[doc = "Field `LP_UART_CLK_SEL` writer - need_des"]
pub type LP_UART_CLK_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
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
            .field("lp_i2c_clk_sel", &self.lp_i2c_clk_sel())
            .field("lp_uart_clk_sel", &self.lp_uart_clk_sel())
            .finish()
    }
}
impl W {
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_i2c_clk_sel(&mut self) -> LP_I2C_CLK_SEL_W<LPPERI_SPEC> {
        LP_I2C_CLK_SEL_W::new(self, 30)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_uart_clk_sel(&mut self) -> LP_UART_CLK_SEL_W<LPPERI_SPEC> {
        LP_UART_CLK_SEL_W::new(self, 31)
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
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LPPERI to value 0"]
impl crate::Resettable for LPPERI_SPEC {
    const RESET_VALUE: u32 = 0;
}
