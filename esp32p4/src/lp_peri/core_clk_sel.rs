#[doc = "Register `CORE_CLK_SEL` reader"]
pub type R = crate::R<CORE_CLK_SEL_SPEC>;
#[doc = "Register `CORE_CLK_SEL` writer"]
pub type W = crate::W<CORE_CLK_SEL_SPEC>;
#[doc = "Field `LP_I2S_TX_CLK_SEL` reader - need_des"]
pub type LP_I2S_TX_CLK_SEL_R = crate::FieldReader;
#[doc = "Field `LP_I2S_TX_CLK_SEL` writer - need_des"]
pub type LP_I2S_TX_CLK_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LP_I2S_RX_CLK_SEL` reader - need_des"]
pub type LP_I2S_RX_CLK_SEL_R = crate::FieldReader;
#[doc = "Field `LP_I2S_RX_CLK_SEL` writer - need_des"]
pub type LP_I2S_RX_CLK_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LP_I2C_CLK_SEL` reader - need_des"]
pub type LP_I2C_CLK_SEL_R = crate::FieldReader;
#[doc = "Field `LP_I2C_CLK_SEL` writer - need_des"]
pub type LP_I2C_CLK_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LP_UART_CLK_SEL` reader - need_des"]
pub type LP_UART_CLK_SEL_R = crate::FieldReader;
#[doc = "Field `LP_UART_CLK_SEL` writer - need_des"]
pub type LP_UART_CLK_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 24:25 - need_des"]
    #[inline(always)]
    pub fn lp_i2s_tx_clk_sel(&self) -> LP_I2S_TX_CLK_SEL_R {
        LP_I2S_TX_CLK_SEL_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - need_des"]
    #[inline(always)]
    pub fn lp_i2s_rx_clk_sel(&self) -> LP_I2S_RX_CLK_SEL_R {
        LP_I2S_RX_CLK_SEL_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - need_des"]
    #[inline(always)]
    pub fn lp_i2c_clk_sel(&self) -> LP_I2C_CLK_SEL_R {
        LP_I2C_CLK_SEL_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - need_des"]
    #[inline(always)]
    pub fn lp_uart_clk_sel(&self) -> LP_UART_CLK_SEL_R {
        LP_UART_CLK_SEL_R::new(((self.bits >> 30) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_CLK_SEL")
            .field("lp_i2s_tx_clk_sel", &self.lp_i2s_tx_clk_sel())
            .field("lp_i2s_rx_clk_sel", &self.lp_i2s_rx_clk_sel())
            .field("lp_i2c_clk_sel", &self.lp_i2c_clk_sel())
            .field("lp_uart_clk_sel", &self.lp_uart_clk_sel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 24:25 - need_des"]
    #[inline(always)]
    pub fn lp_i2s_tx_clk_sel(&mut self) -> LP_I2S_TX_CLK_SEL_W<CORE_CLK_SEL_SPEC> {
        LP_I2S_TX_CLK_SEL_W::new(self, 24)
    }
    #[doc = "Bits 26:27 - need_des"]
    #[inline(always)]
    pub fn lp_i2s_rx_clk_sel(&mut self) -> LP_I2S_RX_CLK_SEL_W<CORE_CLK_SEL_SPEC> {
        LP_I2S_RX_CLK_SEL_W::new(self, 26)
    }
    #[doc = "Bits 28:29 - need_des"]
    #[inline(always)]
    pub fn lp_i2c_clk_sel(&mut self) -> LP_I2C_CLK_SEL_W<CORE_CLK_SEL_SPEC> {
        LP_I2C_CLK_SEL_W::new(self, 28)
    }
    #[doc = "Bits 30:31 - need_des"]
    #[inline(always)]
    pub fn lp_uart_clk_sel(&mut self) -> LP_UART_CLK_SEL_W<CORE_CLK_SEL_SPEC> {
        LP_UART_CLK_SEL_W::new(self, 30)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`core_clk_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core_clk_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_CLK_SEL_SPEC;
impl crate::RegisterSpec for CORE_CLK_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_clk_sel::R`](R) reader structure"]
impl crate::Readable for CORE_CLK_SEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`core_clk_sel::W`](W) writer structure"]
impl crate::Writable for CORE_CLK_SEL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CORE_CLK_SEL to value 0"]
impl crate::Resettable for CORE_CLK_SEL_SPEC {}
