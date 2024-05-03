#[doc = "Register `LP_I2S_TXCLK_DIV_XYZ` reader"]
pub type R = crate::R<LP_I2S_TXCLK_DIV_XYZ_SPEC>;
#[doc = "Register `LP_I2S_TXCLK_DIV_XYZ` writer"]
pub type W = crate::W<LP_I2S_TXCLK_DIV_XYZ_SPEC>;
#[doc = "Field `LP_I2S_TX_CLKM_DIV_YN1` reader - need_des"]
pub type LP_I2S_TX_CLKM_DIV_YN1_R = crate::BitReader;
#[doc = "Field `LP_I2S_TX_CLKM_DIV_YN1` writer - need_des"]
pub type LP_I2S_TX_CLKM_DIV_YN1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_I2S_TX_CLKM_DIV_Z` reader - need_des"]
pub type LP_I2S_TX_CLKM_DIV_Z_R = crate::FieldReader<u16>;
#[doc = "Field `LP_I2S_TX_CLKM_DIV_Z` writer - need_des"]
pub type LP_I2S_TX_CLKM_DIV_Z_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `LP_I2S_TX_CLKM_DIV_Y` reader - need_des"]
pub type LP_I2S_TX_CLKM_DIV_Y_R = crate::FieldReader<u16>;
#[doc = "Field `LP_I2S_TX_CLKM_DIV_Y` writer - need_des"]
pub type LP_I2S_TX_CLKM_DIV_Y_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `LP_I2S_TX_CLKM_DIV_X` reader - need_des"]
pub type LP_I2S_TX_CLKM_DIV_X_R = crate::FieldReader<u16>;
#[doc = "Field `LP_I2S_TX_CLKM_DIV_X` writer - need_des"]
pub type LP_I2S_TX_CLKM_DIV_X_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bit 4 - need_des"]
    #[inline(always)]
    pub fn lp_i2s_tx_clkm_div_yn1(&self) -> LP_I2S_TX_CLKM_DIV_YN1_R {
        LP_I2S_TX_CLKM_DIV_YN1_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:13 - need_des"]
    #[inline(always)]
    pub fn lp_i2s_tx_clkm_div_z(&self) -> LP_I2S_TX_CLKM_DIV_Z_R {
        LP_I2S_TX_CLKM_DIV_Z_R::new(((self.bits >> 5) & 0x01ff) as u16)
    }
    #[doc = "Bits 14:22 - need_des"]
    #[inline(always)]
    pub fn lp_i2s_tx_clkm_div_y(&self) -> LP_I2S_TX_CLKM_DIV_Y_R {
        LP_I2S_TX_CLKM_DIV_Y_R::new(((self.bits >> 14) & 0x01ff) as u16)
    }
    #[doc = "Bits 23:31 - need_des"]
    #[inline(always)]
    pub fn lp_i2s_tx_clkm_div_x(&self) -> LP_I2S_TX_CLKM_DIV_X_R {
        LP_I2S_TX_CLKM_DIV_X_R::new(((self.bits >> 23) & 0x01ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_I2S_TXCLK_DIV_XYZ")
            .field(
                "lp_i2s_tx_clkm_div_yn1",
                &self.lp_i2s_tx_clkm_div_yn1().bit(),
            )
            .field("lp_i2s_tx_clkm_div_z", &self.lp_i2s_tx_clkm_div_z().bits())
            .field("lp_i2s_tx_clkm_div_y", &self.lp_i2s_tx_clkm_div_y().bits())
            .field("lp_i2s_tx_clkm_div_x", &self.lp_i2s_tx_clkm_div_x().bits())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LP_I2S_TXCLK_DIV_XYZ_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 4 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_i2s_tx_clkm_div_yn1(
        &mut self,
    ) -> LP_I2S_TX_CLKM_DIV_YN1_W<LP_I2S_TXCLK_DIV_XYZ_SPEC> {
        LP_I2S_TX_CLKM_DIV_YN1_W::new(self, 4)
    }
    #[doc = "Bits 5:13 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_i2s_tx_clkm_div_z(&mut self) -> LP_I2S_TX_CLKM_DIV_Z_W<LP_I2S_TXCLK_DIV_XYZ_SPEC> {
        LP_I2S_TX_CLKM_DIV_Z_W::new(self, 5)
    }
    #[doc = "Bits 14:22 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_i2s_tx_clkm_div_y(&mut self) -> LP_I2S_TX_CLKM_DIV_Y_W<LP_I2S_TXCLK_DIV_XYZ_SPEC> {
        LP_I2S_TX_CLKM_DIV_Y_W::new(self, 14)
    }
    #[doc = "Bits 23:31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_i2s_tx_clkm_div_x(&mut self) -> LP_I2S_TX_CLKM_DIV_X_W<LP_I2S_TXCLK_DIV_XYZ_SPEC> {
        LP_I2S_TX_CLKM_DIV_X_W::new(self, 23)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_i2s_txclk_div_xyz::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_i2s_txclk_div_xyz::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LP_I2S_TXCLK_DIV_XYZ_SPEC;
impl crate::RegisterSpec for LP_I2S_TXCLK_DIV_XYZ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_i2s_txclk_div_xyz::R`](R) reader structure"]
impl crate::Readable for LP_I2S_TXCLK_DIV_XYZ_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lp_i2s_txclk_div_xyz::W`](W) writer structure"]
impl crate::Writable for LP_I2S_TXCLK_DIV_XYZ_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LP_I2S_TXCLK_DIV_XYZ to value 0x4000"]
impl crate::Resettable for LP_I2S_TXCLK_DIV_XYZ_SPEC {
    const RESET_VALUE: u32 = 0x4000;
}
