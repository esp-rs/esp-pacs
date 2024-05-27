///Register `PERI_CLK_CTRL17` reader
pub type R = crate::R<PERI_CLK_CTRL17_SPEC>;
///Register `PERI_CLK_CTRL17` writer
pub type W = crate::W<PERI_CLK_CTRL17_SPEC>;
///Field `I2S1_TX_DIV_Z` reader - Reserved
pub type I2S1_TX_DIV_Z_R = crate::FieldReader<u16>;
///Field `I2S1_TX_DIV_Z` writer - Reserved
pub type I2S1_TX_DIV_Z_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
///Field `I2S1_TX_DIV_YN1` reader - Reserved
pub type I2S1_TX_DIV_YN1_R = crate::BitReader;
///Field `I2S1_TX_DIV_YN1` writer - Reserved
pub type I2S1_TX_DIV_YN1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2S1_MST_CLK_SEL` reader - Reserved
pub type I2S1_MST_CLK_SEL_R = crate::BitReader;
///Field `I2S1_MST_CLK_SEL` writer - Reserved
pub type I2S1_MST_CLK_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2S2_RX_CLK_EN` reader - Reserved
pub type I2S2_RX_CLK_EN_R = crate::BitReader;
///Field `I2S2_RX_CLK_EN` writer - Reserved
pub type I2S2_RX_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2S2_RX_CLK_SRC_SEL` reader - Reserved
pub type I2S2_RX_CLK_SRC_SEL_R = crate::FieldReader;
///Field `I2S2_RX_CLK_SRC_SEL` writer - Reserved
pub type I2S2_RX_CLK_SRC_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `I2S2_RX_DIV_N` reader - Reserved
pub type I2S2_RX_DIV_N_R = crate::FieldReader;
///Field `I2S2_RX_DIV_N` writer - Reserved
pub type I2S2_RX_DIV_N_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `I2S2_RX_DIV_X` reader - Reserved
pub type I2S2_RX_DIV_X_R = crate::FieldReader<u16>;
///Field `I2S2_RX_DIV_X` writer - Reserved
pub type I2S2_RX_DIV_X_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    ///Bits 0:8 - Reserved
    #[inline(always)]
    pub fn i2s1_tx_div_z(&self) -> I2S1_TX_DIV_Z_R {
        I2S1_TX_DIV_Z_R::new((self.bits & 0x01ff) as u16)
    }
    ///Bit 9 - Reserved
    #[inline(always)]
    pub fn i2s1_tx_div_yn1(&self) -> I2S1_TX_DIV_YN1_R {
        I2S1_TX_DIV_YN1_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Reserved
    #[inline(always)]
    pub fn i2s1_mst_clk_sel(&self) -> I2S1_MST_CLK_SEL_R {
        I2S1_MST_CLK_SEL_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Reserved
    #[inline(always)]
    pub fn i2s2_rx_clk_en(&self) -> I2S2_RX_CLK_EN_R {
        I2S2_RX_CLK_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bits 12:13 - Reserved
    #[inline(always)]
    pub fn i2s2_rx_clk_src_sel(&self) -> I2S2_RX_CLK_SRC_SEL_R {
        I2S2_RX_CLK_SRC_SEL_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bits 14:21 - Reserved
    #[inline(always)]
    pub fn i2s2_rx_div_n(&self) -> I2S2_RX_DIV_N_R {
        I2S2_RX_DIV_N_R::new(((self.bits >> 14) & 0xff) as u8)
    }
    ///Bits 22:30 - Reserved
    #[inline(always)]
    pub fn i2s2_rx_div_x(&self) -> I2S2_RX_DIV_X_R {
        I2S2_RX_DIV_X_R::new(((self.bits >> 22) & 0x01ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PERI_CLK_CTRL17")
            .field("i2s1_tx_div_z", &self.i2s1_tx_div_z())
            .field("i2s1_tx_div_yn1", &self.i2s1_tx_div_yn1())
            .field("i2s1_mst_clk_sel", &self.i2s1_mst_clk_sel())
            .field("i2s2_rx_clk_en", &self.i2s2_rx_clk_en())
            .field("i2s2_rx_clk_src_sel", &self.i2s2_rx_clk_src_sel())
            .field("i2s2_rx_div_n", &self.i2s2_rx_div_n())
            .field("i2s2_rx_div_x", &self.i2s2_rx_div_x())
            .finish()
    }
}
impl W {
    ///Bits 0:8 - Reserved
    #[inline(always)]
    #[must_use]
    pub fn i2s1_tx_div_z(&mut self) -> I2S1_TX_DIV_Z_W<PERI_CLK_CTRL17_SPEC> {
        I2S1_TX_DIV_Z_W::new(self, 0)
    }
    ///Bit 9 - Reserved
    #[inline(always)]
    #[must_use]
    pub fn i2s1_tx_div_yn1(&mut self) -> I2S1_TX_DIV_YN1_W<PERI_CLK_CTRL17_SPEC> {
        I2S1_TX_DIV_YN1_W::new(self, 9)
    }
    ///Bit 10 - Reserved
    #[inline(always)]
    #[must_use]
    pub fn i2s1_mst_clk_sel(&mut self) -> I2S1_MST_CLK_SEL_W<PERI_CLK_CTRL17_SPEC> {
        I2S1_MST_CLK_SEL_W::new(self, 10)
    }
    ///Bit 11 - Reserved
    #[inline(always)]
    #[must_use]
    pub fn i2s2_rx_clk_en(&mut self) -> I2S2_RX_CLK_EN_W<PERI_CLK_CTRL17_SPEC> {
        I2S2_RX_CLK_EN_W::new(self, 11)
    }
    ///Bits 12:13 - Reserved
    #[inline(always)]
    #[must_use]
    pub fn i2s2_rx_clk_src_sel(&mut self) -> I2S2_RX_CLK_SRC_SEL_W<PERI_CLK_CTRL17_SPEC> {
        I2S2_RX_CLK_SRC_SEL_W::new(self, 12)
    }
    ///Bits 14:21 - Reserved
    #[inline(always)]
    #[must_use]
    pub fn i2s2_rx_div_n(&mut self) -> I2S2_RX_DIV_N_W<PERI_CLK_CTRL17_SPEC> {
        I2S2_RX_DIV_N_W::new(self, 14)
    }
    ///Bits 22:30 - Reserved
    #[inline(always)]
    #[must_use]
    pub fn i2s2_rx_div_x(&mut self) -> I2S2_RX_DIV_X_W<PERI_CLK_CTRL17_SPEC> {
        I2S2_RX_DIV_X_W::new(self, 22)
    }
}
/**Reserved

You can [`read`](crate::generic::Reg::read) this register and get [`peri_clk_ctrl17::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`peri_clk_ctrl17::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PERI_CLK_CTRL17_SPEC;
impl crate::RegisterSpec for PERI_CLK_CTRL17_SPEC {
    type Ux = u32;
}
///`read()` method returns [`peri_clk_ctrl17::R`](R) reader structure
impl crate::Readable for PERI_CLK_CTRL17_SPEC {}
///`write(|w| ..)` method takes [`peri_clk_ctrl17::W`](W) writer structure
impl crate::Writable for PERI_CLK_CTRL17_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PERI_CLK_CTRL17 to value 0
impl crate::Resettable for PERI_CLK_CTRL17_SPEC {
    const RESET_VALUE: u32 = 0;
}
