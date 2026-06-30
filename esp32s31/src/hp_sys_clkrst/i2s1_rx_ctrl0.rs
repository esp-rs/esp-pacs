#[doc = "Register `I2S1_RX_CTRL0` reader"]
pub type R = crate::R<I2S1_RX_CTRL0_SPEC>;
#[doc = "Register `I2S1_RX_CTRL0` writer"]
pub type W = crate::W<I2S1_RX_CTRL0_SPEC>;
#[doc = "Field `REG_I2S1_RX_CLK_EN` reader - need_des"]
pub type REG_I2S1_RX_CLK_EN_R = crate::BitReader;
#[doc = "Field `REG_I2S1_RX_CLK_EN` writer - need_des"]
pub type REG_I2S1_RX_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_I2S1_RX_CLK_SRC_SEL` reader - need_des"]
pub type REG_I2S1_RX_CLK_SRC_SEL_R = crate::FieldReader;
#[doc = "Field `REG_I2S1_RX_CLK_SRC_SEL` writer - need_des"]
pub type REG_I2S1_RX_CLK_SRC_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `REG_I2S1_RX_DIV_N` reader - need_des"]
pub type REG_I2S1_RX_DIV_N_R = crate::FieldReader;
#[doc = "Field `REG_I2S1_RX_DIV_N` writer - need_des"]
pub type REG_I2S1_RX_DIV_N_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REG_I2S1_MST_CLK_SEL` reader - need_des"]
pub type REG_I2S1_MST_CLK_SEL_R = crate::BitReader;
#[doc = "Field `REG_I2S1_MST_CLK_SEL` writer - need_des"]
pub type REG_I2S1_MST_CLK_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn reg_i2s1_rx_clk_en(&self) -> REG_I2S1_RX_CLK_EN_R {
        REG_I2S1_RX_CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - need_des"]
    #[inline(always)]
    pub fn reg_i2s1_rx_clk_src_sel(&self) -> REG_I2S1_RX_CLK_SRC_SEL_R {
        REG_I2S1_RX_CLK_SRC_SEL_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:10 - need_des"]
    #[inline(always)]
    pub fn reg_i2s1_rx_div_n(&self) -> REG_I2S1_RX_DIV_N_R {
        REG_I2S1_RX_DIV_N_R::new(((self.bits >> 3) & 0xff) as u8)
    }
    #[doc = "Bit 11 - need_des"]
    #[inline(always)]
    pub fn reg_i2s1_mst_clk_sel(&self) -> REG_I2S1_MST_CLK_SEL_R {
        REG_I2S1_MST_CLK_SEL_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2S1_RX_CTRL0")
            .field("reg_i2s1_rx_clk_en", &self.reg_i2s1_rx_clk_en())
            .field("reg_i2s1_rx_clk_src_sel", &self.reg_i2s1_rx_clk_src_sel())
            .field("reg_i2s1_rx_div_n", &self.reg_i2s1_rx_div_n())
            .field("reg_i2s1_mst_clk_sel", &self.reg_i2s1_mst_clk_sel())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn reg_i2s1_rx_clk_en(&mut self) -> REG_I2S1_RX_CLK_EN_W<'_, I2S1_RX_CTRL0_SPEC> {
        REG_I2S1_RX_CLK_EN_W::new(self, 0)
    }
    #[doc = "Bits 1:2 - need_des"]
    #[inline(always)]
    pub fn reg_i2s1_rx_clk_src_sel(&mut self) -> REG_I2S1_RX_CLK_SRC_SEL_W<'_, I2S1_RX_CTRL0_SPEC> {
        REG_I2S1_RX_CLK_SRC_SEL_W::new(self, 1)
    }
    #[doc = "Bits 3:10 - need_des"]
    #[inline(always)]
    pub fn reg_i2s1_rx_div_n(&mut self) -> REG_I2S1_RX_DIV_N_W<'_, I2S1_RX_CTRL0_SPEC> {
        REG_I2S1_RX_DIV_N_W::new(self, 3)
    }
    #[doc = "Bit 11 - need_des"]
    #[inline(always)]
    pub fn reg_i2s1_mst_clk_sel(&mut self) -> REG_I2S1_MST_CLK_SEL_W<'_, I2S1_RX_CTRL0_SPEC> {
        REG_I2S1_MST_CLK_SEL_W::new(self, 11)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`i2s1_rx_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2s1_rx_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2S1_RX_CTRL0_SPEC;
impl crate::RegisterSpec for I2S1_RX_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2s1_rx_ctrl0::R`](R) reader structure"]
impl crate::Readable for I2S1_RX_CTRL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`i2s1_rx_ctrl0::W`](W) writer structure"]
impl crate::Writable for I2S1_RX_CTRL0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2S1_RX_CTRL0 to value 0"]
impl crate::Resettable for I2S1_RX_CTRL0_SPEC {}
