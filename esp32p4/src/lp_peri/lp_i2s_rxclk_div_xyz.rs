///Register `LP_I2S_RXCLK_DIV_XYZ` reader
pub type R = crate::R<LP_I2S_RXCLK_DIV_XYZ_SPEC>;
///Register `LP_I2S_RXCLK_DIV_XYZ` writer
pub type W = crate::W<LP_I2S_RXCLK_DIV_XYZ_SPEC>;
///Field `LP_I2S_RX_CLKM_DIV_YN1` reader - need_des
pub type LP_I2S_RX_CLKM_DIV_YN1_R = crate::BitReader;
///Field `LP_I2S_RX_CLKM_DIV_YN1` writer - need_des
pub type LP_I2S_RX_CLKM_DIV_YN1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LP_I2S_RX_CLKM_DIV_Z` reader - need_des
pub type LP_I2S_RX_CLKM_DIV_Z_R = crate::FieldReader<u16>;
///Field `LP_I2S_RX_CLKM_DIV_Z` writer - need_des
pub type LP_I2S_RX_CLKM_DIV_Z_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
///Field `LP_I2S_RX_CLKM_DIV_Y` reader - need_des
pub type LP_I2S_RX_CLKM_DIV_Y_R = crate::FieldReader<u16>;
///Field `LP_I2S_RX_CLKM_DIV_Y` writer - need_des
pub type LP_I2S_RX_CLKM_DIV_Y_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
///Field `LP_I2S_RX_CLKM_DIV_X` reader - need_des
pub type LP_I2S_RX_CLKM_DIV_X_R = crate::FieldReader<u16>;
///Field `LP_I2S_RX_CLKM_DIV_X` writer - need_des
pub type LP_I2S_RX_CLKM_DIV_X_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    ///Bit 4 - need_des
    #[inline(always)]
    pub fn lp_i2s_rx_clkm_div_yn1(&self) -> LP_I2S_RX_CLKM_DIV_YN1_R {
        LP_I2S_RX_CLKM_DIV_YN1_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bits 5:13 - need_des
    #[inline(always)]
    pub fn lp_i2s_rx_clkm_div_z(&self) -> LP_I2S_RX_CLKM_DIV_Z_R {
        LP_I2S_RX_CLKM_DIV_Z_R::new(((self.bits >> 5) & 0x01ff) as u16)
    }
    ///Bits 14:22 - need_des
    #[inline(always)]
    pub fn lp_i2s_rx_clkm_div_y(&self) -> LP_I2S_RX_CLKM_DIV_Y_R {
        LP_I2S_RX_CLKM_DIV_Y_R::new(((self.bits >> 14) & 0x01ff) as u16)
    }
    ///Bits 23:31 - need_des
    #[inline(always)]
    pub fn lp_i2s_rx_clkm_div_x(&self) -> LP_I2S_RX_CLKM_DIV_X_R {
        LP_I2S_RX_CLKM_DIV_X_R::new(((self.bits >> 23) & 0x01ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_I2S_RXCLK_DIV_XYZ")
            .field("lp_i2s_rx_clkm_div_yn1", &self.lp_i2s_rx_clkm_div_yn1())
            .field("lp_i2s_rx_clkm_div_z", &self.lp_i2s_rx_clkm_div_z())
            .field("lp_i2s_rx_clkm_div_y", &self.lp_i2s_rx_clkm_div_y())
            .field("lp_i2s_rx_clkm_div_x", &self.lp_i2s_rx_clkm_div_x())
            .finish()
    }
}
impl W {
    ///Bit 4 - need_des
    #[inline(always)]
    #[must_use]
    pub fn lp_i2s_rx_clkm_div_yn1(
        &mut self,
    ) -> LP_I2S_RX_CLKM_DIV_YN1_W<LP_I2S_RXCLK_DIV_XYZ_SPEC> {
        LP_I2S_RX_CLKM_DIV_YN1_W::new(self, 4)
    }
    ///Bits 5:13 - need_des
    #[inline(always)]
    #[must_use]
    pub fn lp_i2s_rx_clkm_div_z(&mut self) -> LP_I2S_RX_CLKM_DIV_Z_W<LP_I2S_RXCLK_DIV_XYZ_SPEC> {
        LP_I2S_RX_CLKM_DIV_Z_W::new(self, 5)
    }
    ///Bits 14:22 - need_des
    #[inline(always)]
    #[must_use]
    pub fn lp_i2s_rx_clkm_div_y(&mut self) -> LP_I2S_RX_CLKM_DIV_Y_W<LP_I2S_RXCLK_DIV_XYZ_SPEC> {
        LP_I2S_RX_CLKM_DIV_Y_W::new(self, 14)
    }
    ///Bits 23:31 - need_des
    #[inline(always)]
    #[must_use]
    pub fn lp_i2s_rx_clkm_div_x(&mut self) -> LP_I2S_RX_CLKM_DIV_X_W<LP_I2S_RXCLK_DIV_XYZ_SPEC> {
        LP_I2S_RX_CLKM_DIV_X_W::new(self, 23)
    }
}
/**need_des

You can [`read`](crate::generic::Reg::read) this register and get [`lp_i2s_rxclk_div_xyz::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_i2s_rxclk_div_xyz::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct LP_I2S_RXCLK_DIV_XYZ_SPEC;
impl crate::RegisterSpec for LP_I2S_RXCLK_DIV_XYZ_SPEC {
    type Ux = u32;
}
///`read()` method returns [`lp_i2s_rxclk_div_xyz::R`](R) reader structure
impl crate::Readable for LP_I2S_RXCLK_DIV_XYZ_SPEC {}
///`write(|w| ..)` method takes [`lp_i2s_rxclk_div_xyz::W`](W) writer structure
impl crate::Writable for LP_I2S_RXCLK_DIV_XYZ_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets LP_I2S_RXCLK_DIV_XYZ to value 0x4000
impl crate::Resettable for LP_I2S_RXCLK_DIV_XYZ_SPEC {
    const RESET_VALUE: u32 = 0x4000;
}
