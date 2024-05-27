///Register `LP_I2S_RXCLK_DIV_NUM` reader
pub type R = crate::R<LP_I2S_RXCLK_DIV_NUM_SPEC>;
///Register `LP_I2S_RXCLK_DIV_NUM` writer
pub type W = crate::W<LP_I2S_RXCLK_DIV_NUM_SPEC>;
///Field `LP_I2S_RX_CLKM_DIV_NUM` reader - need_des
pub type LP_I2S_RX_CLKM_DIV_NUM_R = crate::FieldReader;
///Field `LP_I2S_RX_CLKM_DIV_NUM` writer - need_des
pub type LP_I2S_RX_CLKM_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 24:31 - need_des
    #[inline(always)]
    pub fn lp_i2s_rx_clkm_div_num(&self) -> LP_I2S_RX_CLKM_DIV_NUM_R {
        LP_I2S_RX_CLKM_DIV_NUM_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_I2S_RXCLK_DIV_NUM")
            .field("lp_i2s_rx_clkm_div_num", &self.lp_i2s_rx_clkm_div_num())
            .finish()
    }
}
impl W {
    ///Bits 24:31 - need_des
    #[inline(always)]
    #[must_use]
    pub fn lp_i2s_rx_clkm_div_num(
        &mut self,
    ) -> LP_I2S_RX_CLKM_DIV_NUM_W<LP_I2S_RXCLK_DIV_NUM_SPEC> {
        LP_I2S_RX_CLKM_DIV_NUM_W::new(self, 24)
    }
}
/**need_des

You can [`read`](crate::generic::Reg::read) this register and get [`lp_i2s_rxclk_div_num::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_i2s_rxclk_div_num::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct LP_I2S_RXCLK_DIV_NUM_SPEC;
impl crate::RegisterSpec for LP_I2S_RXCLK_DIV_NUM_SPEC {
    type Ux = u32;
}
///`read()` method returns [`lp_i2s_rxclk_div_num::R`](R) reader structure
impl crate::Readable for LP_I2S_RXCLK_DIV_NUM_SPEC {}
///`write(|w| ..)` method takes [`lp_i2s_rxclk_div_num::W`](W) writer structure
impl crate::Writable for LP_I2S_RXCLK_DIV_NUM_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets LP_I2S_RXCLK_DIV_NUM to value 0x0200_0000
impl crate::Resettable for LP_I2S_RXCLK_DIV_NUM_SPEC {
    const RESET_VALUE: u32 = 0x0200_0000;
}
