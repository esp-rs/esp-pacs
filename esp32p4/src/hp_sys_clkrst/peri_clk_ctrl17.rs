#[doc = "Register `PERI_CLK_CTRL17` reader"]
pub type R = crate::R<PERI_CLK_CTRL17_SPEC>;
#[doc = "Register `PERI_CLK_CTRL17` writer"]
pub type W = crate::W<PERI_CLK_CTRL17_SPEC>;
#[doc = "Field `REG_I2S1_TX_DIV_Z` reader - Reserved"]
pub type REG_I2S1_TX_DIV_Z_R = crate::FieldReader<u16>;
#[doc = "Field `REG_I2S1_TX_DIV_Z` writer - Reserved"]
pub type REG_I2S1_TX_DIV_Z_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `REG_I2S1_TX_DIV_YN1` reader - Reserved"]
pub type REG_I2S1_TX_DIV_YN1_R = crate::BitReader;
#[doc = "Field `REG_I2S1_TX_DIV_YN1` writer - Reserved"]
pub type REG_I2S1_TX_DIV_YN1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_I2S1_MST_CLK_SEL` reader - Reserved"]
pub type REG_I2S1_MST_CLK_SEL_R = crate::BitReader;
#[doc = "Field `REG_I2S1_MST_CLK_SEL` writer - Reserved"]
pub type REG_I2S1_MST_CLK_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_I2S2_RX_CLK_EN` reader - Reserved"]
pub type REG_I2S2_RX_CLK_EN_R = crate::BitReader;
#[doc = "Field `REG_I2S2_RX_CLK_EN` writer - Reserved"]
pub type REG_I2S2_RX_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_I2S2_RX_CLK_SRC_SEL` reader - Reserved"]
pub type REG_I2S2_RX_CLK_SRC_SEL_R = crate::FieldReader;
#[doc = "Field `REG_I2S2_RX_CLK_SRC_SEL` writer - Reserved"]
pub type REG_I2S2_RX_CLK_SRC_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `REG_I2S2_RX_DIV_N` reader - Reserved"]
pub type REG_I2S2_RX_DIV_N_R = crate::FieldReader;
#[doc = "Field `REG_I2S2_RX_DIV_N` writer - Reserved"]
pub type REG_I2S2_RX_DIV_N_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REG_I2S2_RX_DIV_X` reader - Reserved"]
pub type REG_I2S2_RX_DIV_X_R = crate::FieldReader<u16>;
#[doc = "Field `REG_I2S2_RX_DIV_X` writer - Reserved"]
pub type REG_I2S2_RX_DIV_X_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8 - Reserved"]
    #[inline(always)]
    pub fn reg_i2s1_tx_div_z(&self) -> REG_I2S1_TX_DIV_Z_R {
        REG_I2S1_TX_DIV_Z_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 9 - Reserved"]
    #[inline(always)]
    pub fn reg_i2s1_tx_div_yn1(&self) -> REG_I2S1_TX_DIV_YN1_R {
        REG_I2S1_TX_DIV_YN1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Reserved"]
    #[inline(always)]
    pub fn reg_i2s1_mst_clk_sel(&self) -> REG_I2S1_MST_CLK_SEL_R {
        REG_I2S1_MST_CLK_SEL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Reserved"]
    #[inline(always)]
    pub fn reg_i2s2_rx_clk_en(&self) -> REG_I2S2_RX_CLK_EN_R {
        REG_I2S2_RX_CLK_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Reserved"]
    #[inline(always)]
    pub fn reg_i2s2_rx_clk_src_sel(&self) -> REG_I2S2_RX_CLK_SRC_SEL_R {
        REG_I2S2_RX_CLK_SRC_SEL_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:21 - Reserved"]
    #[inline(always)]
    pub fn reg_i2s2_rx_div_n(&self) -> REG_I2S2_RX_DIV_N_R {
        REG_I2S2_RX_DIV_N_R::new(((self.bits >> 14) & 0xff) as u8)
    }
    #[doc = "Bits 22:30 - Reserved"]
    #[inline(always)]
    pub fn reg_i2s2_rx_div_x(&self) -> REG_I2S2_RX_DIV_X_R {
        REG_I2S2_RX_DIV_X_R::new(((self.bits >> 22) & 0x01ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PERI_CLK_CTRL17")
            .field(
                "reg_i2s1_tx_div_z",
                &format_args!("{}", self.reg_i2s1_tx_div_z().bits()),
            )
            .field(
                "reg_i2s1_tx_div_yn1",
                &format_args!("{}", self.reg_i2s1_tx_div_yn1().bit()),
            )
            .field(
                "reg_i2s1_mst_clk_sel",
                &format_args!("{}", self.reg_i2s1_mst_clk_sel().bit()),
            )
            .field(
                "reg_i2s2_rx_clk_en",
                &format_args!("{}", self.reg_i2s2_rx_clk_en().bit()),
            )
            .field(
                "reg_i2s2_rx_clk_src_sel",
                &format_args!("{}", self.reg_i2s2_rx_clk_src_sel().bits()),
            )
            .field(
                "reg_i2s2_rx_div_n",
                &format_args!("{}", self.reg_i2s2_rx_div_n().bits()),
            )
            .field(
                "reg_i2s2_rx_div_x",
                &format_args!("{}", self.reg_i2s2_rx_div_x().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PERI_CLK_CTRL17_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:8 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_i2s1_tx_div_z(&mut self) -> REG_I2S1_TX_DIV_Z_W<PERI_CLK_CTRL17_SPEC> {
        REG_I2S1_TX_DIV_Z_W::new(self, 0)
    }
    #[doc = "Bit 9 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_i2s1_tx_div_yn1(&mut self) -> REG_I2S1_TX_DIV_YN1_W<PERI_CLK_CTRL17_SPEC> {
        REG_I2S1_TX_DIV_YN1_W::new(self, 9)
    }
    #[doc = "Bit 10 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_i2s1_mst_clk_sel(&mut self) -> REG_I2S1_MST_CLK_SEL_W<PERI_CLK_CTRL17_SPEC> {
        REG_I2S1_MST_CLK_SEL_W::new(self, 10)
    }
    #[doc = "Bit 11 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_i2s2_rx_clk_en(&mut self) -> REG_I2S2_RX_CLK_EN_W<PERI_CLK_CTRL17_SPEC> {
        REG_I2S2_RX_CLK_EN_W::new(self, 11)
    }
    #[doc = "Bits 12:13 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_i2s2_rx_clk_src_sel(&mut self) -> REG_I2S2_RX_CLK_SRC_SEL_W<PERI_CLK_CTRL17_SPEC> {
        REG_I2S2_RX_CLK_SRC_SEL_W::new(self, 12)
    }
    #[doc = "Bits 14:21 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_i2s2_rx_div_n(&mut self) -> REG_I2S2_RX_DIV_N_W<PERI_CLK_CTRL17_SPEC> {
        REG_I2S2_RX_DIV_N_W::new(self, 14)
    }
    #[doc = "Bits 22:30 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_i2s2_rx_div_x(&mut self) -> REG_I2S2_RX_DIV_X_W<PERI_CLK_CTRL17_SPEC> {
        REG_I2S2_RX_DIV_X_W::new(self, 22)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`peri_clk_ctrl17::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`peri_clk_ctrl17::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PERI_CLK_CTRL17_SPEC;
impl crate::RegisterSpec for PERI_CLK_CTRL17_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`peri_clk_ctrl17::R`](R) reader structure"]
impl crate::Readable for PERI_CLK_CTRL17_SPEC {}
#[doc = "`write(|w| ..)` method takes [`peri_clk_ctrl17::W`](W) writer structure"]
impl crate::Writable for PERI_CLK_CTRL17_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PERI_CLK_CTRL17 to value 0"]
impl crate::Resettable for PERI_CLK_CTRL17_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
