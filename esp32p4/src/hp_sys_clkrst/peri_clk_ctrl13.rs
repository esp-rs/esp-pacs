#[doc = "Register `PERI_CLK_CTRL13` reader"]
pub type R = crate::R<PERI_CLK_CTRL13_SPEC>;
#[doc = "Register `PERI_CLK_CTRL13` writer"]
pub type W = crate::W<PERI_CLK_CTRL13_SPEC>;
#[doc = "Field `I2S0_RX_DIV_Z` reader - Reserved"]
pub type I2S0_RX_DIV_Z_R = crate::FieldReader<u16>;
#[doc = "Field `I2S0_RX_DIV_Z` writer - Reserved"]
pub type I2S0_RX_DIV_Z_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `I2S0_RX_DIV_YN1` reader - Reserved"]
pub type I2S0_RX_DIV_YN1_R = crate::BitReader;
#[doc = "Field `I2S0_RX_DIV_YN1` writer - Reserved"]
pub type I2S0_RX_DIV_YN1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2S0_TX_CLK_EN` reader - Reserved"]
pub type I2S0_TX_CLK_EN_R = crate::BitReader;
#[doc = "Field `I2S0_TX_CLK_EN` writer - Reserved"]
pub type I2S0_TX_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2S0_TX_CLK_SRC_SEL` reader - Reserved"]
pub type I2S0_TX_CLK_SRC_SEL_R = crate::FieldReader;
#[doc = "Field `I2S0_TX_CLK_SRC_SEL` writer - Reserved"]
pub type I2S0_TX_CLK_SRC_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `I2S0_TX_DIV_N` reader - Reserved"]
pub type I2S0_TX_DIV_N_R = crate::FieldReader;
#[doc = "Field `I2S0_TX_DIV_N` writer - Reserved"]
pub type I2S0_TX_DIV_N_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `I2S0_TX_DIV_X` reader - Reserved"]
pub type I2S0_TX_DIV_X_R = crate::FieldReader<u16>;
#[doc = "Field `I2S0_TX_DIV_X` writer - Reserved"]
pub type I2S0_TX_DIV_X_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8 - Reserved"]
    #[inline(always)]
    pub fn i2s0_rx_div_z(&self) -> I2S0_RX_DIV_Z_R {
        I2S0_RX_DIV_Z_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 9 - Reserved"]
    #[inline(always)]
    pub fn i2s0_rx_div_yn1(&self) -> I2S0_RX_DIV_YN1_R {
        I2S0_RX_DIV_YN1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Reserved"]
    #[inline(always)]
    pub fn i2s0_tx_clk_en(&self) -> I2S0_TX_CLK_EN_R {
        I2S0_TX_CLK_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:12 - Reserved"]
    #[inline(always)]
    pub fn i2s0_tx_clk_src_sel(&self) -> I2S0_TX_CLK_SRC_SEL_R {
        I2S0_TX_CLK_SRC_SEL_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bits 13:20 - Reserved"]
    #[inline(always)]
    pub fn i2s0_tx_div_n(&self) -> I2S0_TX_DIV_N_R {
        I2S0_TX_DIV_N_R::new(((self.bits >> 13) & 0xff) as u8)
    }
    #[doc = "Bits 21:29 - Reserved"]
    #[inline(always)]
    pub fn i2s0_tx_div_x(&self) -> I2S0_TX_DIV_X_R {
        I2S0_TX_DIV_X_R::new(((self.bits >> 21) & 0x01ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PERI_CLK_CTRL13")
            .field(
                "i2s0_rx_div_z",
                &format_args!("{}", self.i2s0_rx_div_z().bits()),
            )
            .field(
                "i2s0_rx_div_yn1",
                &format_args!("{}", self.i2s0_rx_div_yn1().bit()),
            )
            .field(
                "i2s0_tx_clk_en",
                &format_args!("{}", self.i2s0_tx_clk_en().bit()),
            )
            .field(
                "i2s0_tx_clk_src_sel",
                &format_args!("{}", self.i2s0_tx_clk_src_sel().bits()),
            )
            .field(
                "i2s0_tx_div_n",
                &format_args!("{}", self.i2s0_tx_div_n().bits()),
            )
            .field(
                "i2s0_tx_div_x",
                &format_args!("{}", self.i2s0_tx_div_x().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PERI_CLK_CTRL13_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:8 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn i2s0_rx_div_z(&mut self) -> I2S0_RX_DIV_Z_W<PERI_CLK_CTRL13_SPEC> {
        I2S0_RX_DIV_Z_W::new(self, 0)
    }
    #[doc = "Bit 9 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn i2s0_rx_div_yn1(&mut self) -> I2S0_RX_DIV_YN1_W<PERI_CLK_CTRL13_SPEC> {
        I2S0_RX_DIV_YN1_W::new(self, 9)
    }
    #[doc = "Bit 10 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn i2s0_tx_clk_en(&mut self) -> I2S0_TX_CLK_EN_W<PERI_CLK_CTRL13_SPEC> {
        I2S0_TX_CLK_EN_W::new(self, 10)
    }
    #[doc = "Bits 11:12 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn i2s0_tx_clk_src_sel(&mut self) -> I2S0_TX_CLK_SRC_SEL_W<PERI_CLK_CTRL13_SPEC> {
        I2S0_TX_CLK_SRC_SEL_W::new(self, 11)
    }
    #[doc = "Bits 13:20 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn i2s0_tx_div_n(&mut self) -> I2S0_TX_DIV_N_W<PERI_CLK_CTRL13_SPEC> {
        I2S0_TX_DIV_N_W::new(self, 13)
    }
    #[doc = "Bits 21:29 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn i2s0_tx_div_x(&mut self) -> I2S0_TX_DIV_X_W<PERI_CLK_CTRL13_SPEC> {
        I2S0_TX_DIV_X_W::new(self, 21)
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`peri_clk_ctrl13::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`peri_clk_ctrl13::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PERI_CLK_CTRL13_SPEC;
impl crate::RegisterSpec for PERI_CLK_CTRL13_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`peri_clk_ctrl13::R`](R) reader structure"]
impl crate::Readable for PERI_CLK_CTRL13_SPEC {}
#[doc = "`write(|w| ..)` method takes [`peri_clk_ctrl13::W`](W) writer structure"]
impl crate::Writable for PERI_CLK_CTRL13_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PERI_CLK_CTRL13 to value 0"]
impl crate::Resettable for PERI_CLK_CTRL13_SPEC {
    const RESET_VALUE: u32 = 0;
}
