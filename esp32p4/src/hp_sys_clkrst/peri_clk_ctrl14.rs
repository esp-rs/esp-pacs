#[doc = "Register `PERI_CLK_CTRL14` reader"]
pub type R = crate::R<PERI_CLK_CTRL14_SPEC>;
#[doc = "Register `PERI_CLK_CTRL14` writer"]
pub type W = crate::W<PERI_CLK_CTRL14_SPEC>;
#[doc = "Field `I2S0_TX_DIV_Y` reader - Reserved"]
pub type I2S0_TX_DIV_Y_R = crate::FieldReader<u16>;
#[doc = "Field `I2S0_TX_DIV_Y` writer - Reserved"]
pub type I2S0_TX_DIV_Y_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `I2S0_TX_DIV_Z` reader - Reserved"]
pub type I2S0_TX_DIV_Z_R = crate::FieldReader<u16>;
#[doc = "Field `I2S0_TX_DIV_Z` writer - Reserved"]
pub type I2S0_TX_DIV_Z_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `I2S0_TX_DIV_YN1` reader - Reserved"]
pub type I2S0_TX_DIV_YN1_R = crate::BitReader;
#[doc = "Field `I2S0_TX_DIV_YN1` writer - Reserved"]
pub type I2S0_TX_DIV_YN1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2S0_MST_CLK_SEL` reader - Reserved"]
pub type I2S0_MST_CLK_SEL_R = crate::BitReader;
#[doc = "Field `I2S0_MST_CLK_SEL` writer - Reserved"]
pub type I2S0_MST_CLK_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2S1_RX_CLK_EN` reader - Reserved"]
pub type I2S1_RX_CLK_EN_R = crate::BitReader;
#[doc = "Field `I2S1_RX_CLK_EN` writer - Reserved"]
pub type I2S1_RX_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2S1_RX_CLK_SRC_SEL` reader - Reserved"]
pub type I2S1_RX_CLK_SRC_SEL_R = crate::FieldReader;
#[doc = "Field `I2S1_RX_CLK_SRC_SEL` writer - Reserved"]
pub type I2S1_RX_CLK_SRC_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `I2S1_RX_DIV_N` reader - Reserved"]
pub type I2S1_RX_DIV_N_R = crate::FieldReader;
#[doc = "Field `I2S1_RX_DIV_N` writer - Reserved"]
pub type I2S1_RX_DIV_N_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:8 - Reserved"]
    #[inline(always)]
    pub fn i2s0_tx_div_y(&self) -> I2S0_TX_DIV_Y_R {
        I2S0_TX_DIV_Y_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:17 - Reserved"]
    #[inline(always)]
    pub fn i2s0_tx_div_z(&self) -> I2S0_TX_DIV_Z_R {
        I2S0_TX_DIV_Z_R::new(((self.bits >> 9) & 0x01ff) as u16)
    }
    #[doc = "Bit 18 - Reserved"]
    #[inline(always)]
    pub fn i2s0_tx_div_yn1(&self) -> I2S0_TX_DIV_YN1_R {
        I2S0_TX_DIV_YN1_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Reserved"]
    #[inline(always)]
    pub fn i2s0_mst_clk_sel(&self) -> I2S0_MST_CLK_SEL_R {
        I2S0_MST_CLK_SEL_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Reserved"]
    #[inline(always)]
    pub fn i2s1_rx_clk_en(&self) -> I2S1_RX_CLK_EN_R {
        I2S1_RX_CLK_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:22 - Reserved"]
    #[inline(always)]
    pub fn i2s1_rx_clk_src_sel(&self) -> I2S1_RX_CLK_SRC_SEL_R {
        I2S1_RX_CLK_SRC_SEL_R::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bits 23:30 - Reserved"]
    #[inline(always)]
    pub fn i2s1_rx_div_n(&self) -> I2S1_RX_DIV_N_R {
        I2S1_RX_DIV_N_R::new(((self.bits >> 23) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PERI_CLK_CTRL14")
            .field("i2s0_tx_div_y", &self.i2s0_tx_div_y())
            .field("i2s0_tx_div_z", &self.i2s0_tx_div_z())
            .field("i2s0_tx_div_yn1", &self.i2s0_tx_div_yn1())
            .field("i2s0_mst_clk_sel", &self.i2s0_mst_clk_sel())
            .field("i2s1_rx_clk_en", &self.i2s1_rx_clk_en())
            .field("i2s1_rx_clk_src_sel", &self.i2s1_rx_clk_src_sel())
            .field("i2s1_rx_div_n", &self.i2s1_rx_div_n())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:8 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn i2s0_tx_div_y(&mut self) -> I2S0_TX_DIV_Y_W<PERI_CLK_CTRL14_SPEC> {
        I2S0_TX_DIV_Y_W::new(self, 0)
    }
    #[doc = "Bits 9:17 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn i2s0_tx_div_z(&mut self) -> I2S0_TX_DIV_Z_W<PERI_CLK_CTRL14_SPEC> {
        I2S0_TX_DIV_Z_W::new(self, 9)
    }
    #[doc = "Bit 18 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn i2s0_tx_div_yn1(&mut self) -> I2S0_TX_DIV_YN1_W<PERI_CLK_CTRL14_SPEC> {
        I2S0_TX_DIV_YN1_W::new(self, 18)
    }
    #[doc = "Bit 19 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn i2s0_mst_clk_sel(&mut self) -> I2S0_MST_CLK_SEL_W<PERI_CLK_CTRL14_SPEC> {
        I2S0_MST_CLK_SEL_W::new(self, 19)
    }
    #[doc = "Bit 20 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn i2s1_rx_clk_en(&mut self) -> I2S1_RX_CLK_EN_W<PERI_CLK_CTRL14_SPEC> {
        I2S1_RX_CLK_EN_W::new(self, 20)
    }
    #[doc = "Bits 21:22 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn i2s1_rx_clk_src_sel(&mut self) -> I2S1_RX_CLK_SRC_SEL_W<PERI_CLK_CTRL14_SPEC> {
        I2S1_RX_CLK_SRC_SEL_W::new(self, 21)
    }
    #[doc = "Bits 23:30 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn i2s1_rx_div_n(&mut self) -> I2S1_RX_DIV_N_W<PERI_CLK_CTRL14_SPEC> {
        I2S1_RX_DIV_N_W::new(self, 23)
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`peri_clk_ctrl14::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peri_clk_ctrl14::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PERI_CLK_CTRL14_SPEC;
impl crate::RegisterSpec for PERI_CLK_CTRL14_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`peri_clk_ctrl14::R`](R) reader structure"]
impl crate::Readable for PERI_CLK_CTRL14_SPEC {}
#[doc = "`write(|w| ..)` method takes [`peri_clk_ctrl14::W`](W) writer structure"]
impl crate::Writable for PERI_CLK_CTRL14_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PERI_CLK_CTRL14 to value 0"]
impl crate::Resettable for PERI_CLK_CTRL14_SPEC {
    const RESET_VALUE: u32 = 0;
}
